use std::collections::HashMap;

use log::*;
use svd::RegisterCluster;
use svd_parser as svd;

use crate::ir::*;
use crate::util;

pub fn convert_block(
    ir: &mut IR,
    path: Vec<String>,
    name: String,
    description: Option<String>,
    regs: &Vec<RegisterCluster>,
) -> Id<Block> {
    let mut block = Block {
        path: Path::new(path.clone(), name.clone()),
        description,
        items: Vec::new(),
    };

    let mut regs_path = pushit(&path, "regs".to_owned());
    let mut vals_path = pushit(&path, "vals".to_owned());

    for r in regs {
        match r {
            svd::RegisterCluster::Register(r) => {
                if r.derived_from.is_some() {
                    warn!("unsupported derived_from in registers");
                }

                let rname = util::replace_suffix(&r.name, "");

                let array = if let svd::Register::Array(_, dim) = r {
                    Some(Array {
                        len: dim.dim,
                        stride: dim.dim_increment,
                    })
                } else {
                    None
                };

                // If the register has fields, create a FieldSet for it.
                let fieldset_id = if let Some(fields) = &r.fields {
                    let mut fieldset = FieldSet {
                        path: Path::new(regs_path.clone(), rname.clone()),
                        description: r.description.clone(),
                        bit_size: 32, // todo
                        fields: Vec::new(),
                    };
                    for f in fields {
                        if f.derived_from.is_some() {
                            warn!("unsupported derived_from in fields");
                        }

                        let enumm = if f.enumerated_values.len() == 0 {
                            None
                        } else {
                            let mut variants = Vec::new();

                            for e in &f.enumerated_values {
                                if e.derived_from.is_some() {
                                    warn!("unsupported derived_from in enums");
                                }

                                let prefix = if f.enumerated_values.len() == 1 {
                                    ""
                                } else {
                                    match e.usage {
                                        None => "",
                                        Some(svd::Usage::Read) => "R_",
                                        Some(svd::Usage::Write) => "W_",
                                        Some(svd::Usage::ReadWrite) => "RW_",
                                    }
                                };
                                variants.extend(e.values.iter().map(|v| EnumVariant {
                                    description: v.description.clone(),
                                    name: format!("{}{}", prefix, v.name),
                                    value: v.value.unwrap() as _, // TODO what are variants without values used for??
                                }));
                            }

                            let enum_name = if rname == f.name {
                                rname.clone()
                            } else {
                                format!("{}_{}", rname, f.name)
                            };

                            let enumm = Enum {
                                path: Path::new(vals_path.clone(), enum_name),
                                description: r.description.clone(),
                                bit_size: f.bit_range.width,
                                variants,
                            };
                            Some(ir.enums.put(enumm))
                        };

                        fieldset.fields.push(Field {
                            name: f.name.clone(),
                            description: f.description.clone(),
                            bit_offset: f.bit_range.offset,
                            bit_size: f.bit_range.width,
                            array: None,
                            enumm,
                        })
                    }
                    Some(ir.fieldsets.put(fieldset))
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

                block.items.push(BlockItem {
                    name: rname.clone(),
                    description: r.description.clone(),
                    array,
                    byte_offset: r.address_offset,
                    inner: BlockItemInner::Register(Register {
                        access, // todo
                        bit_size: r.size.unwrap_or(32),
                        fieldset: fieldset_id,
                        reset_value: r.reset_value.map(|v| v as _),
                    }),
                })
            }
            svd::RegisterCluster::Cluster(c) => {
                let cname = util::replace_suffix(&c.name, "");

                let array = if let svd::Cluster::Array(_, dim) = c {
                    Some(Array {
                        len: dim.dim,
                        stride: dim.dim_increment,
                    })
                } else {
                    None
                };

                let block_path = pushit(&path, cname.clone());
                let id = convert_block(
                    ir,
                    block_path,
                    cname.clone(),
                    c.description.clone(),
                    &c.children,
                );

                block.items.push(BlockItem {
                    name: cname.clone(),
                    description: c.description.clone(),
                    array,
                    byte_offset: c.address_offset,
                    inner: BlockItemInner::Block(id),
                });
            }
        }
    }

    ir.blocks.put(block)
}

fn pushit(v: &Vec<String>, s: String) -> Vec<String> {
    let mut r = v.clone();
    r.push(s);
    r
}

pub fn convert(ir: &mut IR, svd: &svd::Device, path: Vec<String>) -> anyhow::Result<()> {
    let mut peripheral_ids = HashMap::new();

    for p in &svd.peripherals {
        if let Some(regs) = &p.registers {
            let id = convert_block(
                ir,
                pushit(&path, p.name.clone()),
                p.name.clone(),
                p.description.clone(),
                regs,
            );
            peripheral_ids.insert(p.name.clone(), id);
        }
    }

    // Device

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

    Ok(())
}
