use std::collections::HashMap;

use log::*;
use svd_parser as svd;

use crate::ir::*;
use crate::util;

struct ProtoBlock {
    name: Vec<String>,
    description: Option<String>,
    registers: Vec<svd::RegisterCluster>,
}

struct ProtoFieldset {
    name: Vec<String>,
    description: Option<String>,
    bit_size: u32,
    fields: Vec<svd::Field>,
}

struct ProtoEnum {
    name: Vec<String>,
    usage: Option<svd::Usage>,
    bit_size: u32,
    variants: Vec<svd::EnumeratedValue>,
}

pub fn convert_peripheral(ir: &mut IR, p: &svd::Peripheral) -> anyhow::Result<()> {
    let mut blocks = Vec::new();
    collect_blocks(
        &mut blocks,
        vec![p.name.clone()],
        p.description.clone(),
        p.registers.as_ref().unwrap(),
    );

    let mut fieldsets: Vec<ProtoFieldset> = Vec::new();
    let mut enums: Vec<ProtoEnum> = Vec::new();

    for block in &blocks {
        for r in &block.registers {
            if let svd::RegisterCluster::Register(r) = r {
                if r.derived_from.is_some() {
                    continue;
                }

                if let Some(fields) = &r.fields {
                    let mut fieldset_name = block.name.clone();
                    fieldset_name.push(util::replace_suffix(&r.name, ""));
                    fieldsets.push(ProtoFieldset {
                        name: fieldset_name.clone(),
                        description: r.description.clone(),
                        bit_size: 32, // todo
                        fields: fields.clone(),
                    });

                    for f in fields {
                        if f.derived_from.is_some() {
                            continue;
                        }

                        let field_name = f.name.clone();

                        for e in &f.enumerated_values {
                            if e.derived_from.is_some() {
                                continue;
                            }

                            let mut enum_name = fieldset_name.clone();
                            enum_name.push(e.name.clone().unwrap_or_else(|| field_name.clone()));
                            info!("adding enum {:?}", enum_name);

                            enums.push(ProtoEnum {
                                name: enum_name,
                                usage: e.usage,
                                bit_size: f.bit_range.width,
                                variants: e.values.clone(),
                            });
                        }
                    }
                };
            }
        }
    }

    // Make all collected names unique by prefixing with parents' names if needed.
    let block_names = unique_names(blocks.iter().map(|x| x.name.clone()).collect());
    let fieldset_names = unique_names(fieldsets.iter().map(|x| x.name.clone()).collect());
    let enum_names = unique_names(enums.iter().map(|x| x.name.clone()).collect());

    // Convert blocks
    for proto in &blocks {
        let mut block = Block {
            description: proto.description.clone(),
            items: Vec::new(),
        };

        for r in &proto.registers {
            match r {
                svd::RegisterCluster::Register(r) => {
                    if r.derived_from.is_some() {
                        warn!("unsupported derived_from in registers");
                        continue;
                    }

                    let fieldset_name = if r.fields.is_some() {
                        let mut fieldset_name = proto.name.clone();
                        fieldset_name.push(util::replace_suffix(&r.name, ""));
                        Some(fieldset_names.get(&fieldset_name).unwrap().clone())
                    } else {
                        None
                    };

                    let array = if let svd::Register::Array(_, dim) = r {
                        Some(Array {
                            len: dim.dim,
                            stride: dim.dim_increment,
                        })
                    } else {
                        None
                    };

                    let access = match r.access {
                        None => Access::ReadWrite,
                        Some(svd::Access::ReadOnly) => Access::Read,
                        Some(svd::Access::WriteOnly) => Access::Write,
                        Some(svd::Access::WriteOnce) => Access::Write,
                        Some(svd::Access::ReadWrite) => Access::ReadWrite,
                        Some(svd::Access::ReadWriteOnce) => Access::ReadWrite,
                    };

                    let block_item = BlockItem {
                        name: util::replace_suffix(&r.name, ""),
                        description: r.description.clone(),
                        array,
                        byte_offset: r.address_offset,
                        inner: BlockItemInner::Register(Register {
                            access, // todo
                            bit_size: r.size.unwrap_or(32),
                            fieldset: fieldset_name.clone(),
                            reset_value: r.reset_value.map(|v| v as _),
                        }),
                    };

                    block.items.push(block_item)
                }
                svd::RegisterCluster::Cluster(c) => {
                    if c.derived_from.is_some() {
                        warn!("unsupported derived_from in clusters");
                        continue;
                    }

                    let cname = util::replace_suffix(&c.name, "");

                    let array = if let svd::Cluster::Array(_, dim) = c {
                        Some(Array {
                            len: dim.dim,
                            stride: dim.dim_increment,
                        })
                    } else {
                        None
                    };

                    let mut block_name = proto.name.clone();
                    block_name.push(util::replace_suffix(&c.name, ""));
                    let block_name = block_names.get(&block_name).unwrap().clone();

                    block.items.push(BlockItem {
                        name: cname.clone(),
                        description: c.description.clone(),
                        array,
                        byte_offset: c.address_offset,
                        inner: BlockItemInner::Block(block_name),
                    });
                }
            }
        }

        let block_name = block_names.get(&proto.name).unwrap().clone();
        assert!(ir.blocks.insert(block_name, block).is_none())
    }

    // Convert fieldsets
    for proto in &fieldsets {
        let mut fieldset = FieldSet {
            description: proto.description.clone(),
            bit_size: proto.bit_size,
            fields: Vec::new(),
        };

        for f in &proto.fields {
            if f.derived_from.is_some() {
                warn!("unsupported derived_from in fieldset");
            }

            let mut field = Field {
                name: f.name.clone(),
                description: f.description.clone(),
                bit_offset: f.bit_range.offset,
                bit_size: f.bit_range.width,
                array: None,
                enum_read: None,
                enum_write: None,
                enum_readwrite: None,
            };

            for e in &f.enumerated_values {
                let mut enum_name = proto.name.clone();
                enum_name.push(
                    e.derived_from
                        .clone()
                        .or_else(|| e.name.clone())
                        .unwrap_or_else(|| f.name.clone()),
                );
                info!("finding enum {:?}", enum_name);
                let enumm = enums.iter().find(|e| e.name == enum_name).unwrap();
                let enum_name = enum_names.get(&enum_name).unwrap().clone();
                info!("found {:?}", enum_name);

                let usage = enumm.usage.unwrap_or(svd::Usage::ReadWrite);

                match usage {
                    svd::Usage::Read => field.enum_read = Some(enum_name.clone()),
                    svd::Usage::Write => field.enum_write = Some(enum_name.clone()),
                    svd::Usage::ReadWrite => field.enum_readwrite = Some(enum_name.clone()),
                }
            }

            fieldset.fields.push(field)
        }

        let fieldset_name = fieldset_names.get(&proto.name).unwrap().clone();
        assert!(ir.fieldsets.insert(fieldset_name, fieldset).is_none())
    }

    for proto in &enums {
        let variants = proto
            .variants
            .iter()
            .map(|v| EnumVariant {
                description: v.description.clone(),
                name: v.name.clone(),
                value: v.value.unwrap() as _, // TODO what are variants without values used for??
            })
            .collect();

        let enumm = Enum {
            description: None,
            bit_size: proto.bit_size,
            variants,
        };

        let enum_name = enum_names.get(&proto.name).unwrap().clone();
        assert!(ir.enums.insert(enum_name.clone(), enumm).is_none());
    }

    /*
    let mut device = Device {
        path: Path::new(path, "Device".to_owned()),
        cpu: svd.cpu.clone(),
        interrupts: vec![],
        peripherals: vec![],
    };

    for p in &svd.peripherals {
        let peripheral_name = p.derived_from.as_ref().unwrap_or(&p.name);
        let block = *peripheral_ids.get(peripheral_name).unwrap();

        device.peripherals.push(Peripheral {
            name: p.name.clone(),
            description: p.description.clone(),
            base_address: p.base_address,
            block,
        });

        for i in &p.interrupt {
            if !device.interrupts.iter().any(|i2| i2.name == i.name) {
                device.interrupts.push(Interrupt {
                    name: i.name.clone(),
                    description: i.description.clone(),
                    value: i.value,
                })
            }
        }
    }

    ir.devices.put(device);
     */

    Ok(())
}

fn collect_blocks(
    out: &mut Vec<ProtoBlock>,
    block_name: Vec<String>,
    description: Option<String>,
    registers: &Vec<svd::RegisterCluster>,
) {
    out.push(ProtoBlock {
        name: block_name.clone(),
        description,
        registers: registers.clone(),
    });

    for r in registers {
        if let svd::RegisterCluster::Cluster(c) = r {
            if c.derived_from.is_some() {
                continue;
            }

            let mut block_name = block_name.clone();
            block_name.push(util::replace_suffix(&c.name, ""));
            collect_blocks(out, block_name, c.description.clone(), &c.children);
        }
    }
}

fn unique_names(names: Vec<Vec<String>>) -> HashMap<Vec<String>, String> {
    let mut res = HashMap::new();

    let suffix_exists = |n: &[String], i: usize| {
        names
            .iter()
            .enumerate()
            .filter(|(j, _)| *j != i)
            .any(|(_, n2)| n2.ends_with(n))
    };
    for (i, n) in names.iter().enumerate() {
        let j = (0..n.len())
            .rev()
            .filter(|&j| !suffix_exists(&n[j..], i))
            .next()
            .unwrap();
        assert!(res.insert(n.clone(), n[j..].join("_")).is_none());
    }
    res
}
