use log::*;
use svd_parser as svd;

use crate::ir::*;
use crate::util;

pub fn convert(svd: &svd::Device) -> Device {
    let mut device = Device::new();
    device.name = svd.name.clone();
    device.cpu = svd.cpu.clone();

    // Interrupts
    for p in &svd.peripherals {
        for i in &p.interrupt {
            device.interrupts.push(Interrupt {
                name: i.name.clone(),
                description: i.description.clone(),
                value: i.value,
            })
        }
    }

    // Peripherals
    for p in &svd.peripherals {
        if let Some(regs) = &p.registers {
            let mut block = Block {
                path: Path::new(vec![p.name.clone()], p.name.clone()),
                items: Vec::new(),
            };

            for r in regs {
                match r {
                    svd::RegisterCluster::Register(r) => {
                        let rname = util::replace_suffix(&r.name, "");

                        let array = if let svd::Register::Array(_, dim) = r {
                            Some(Array {
                                len: dim.dim,
                                byte_stride: dim.dim_increment,
                            })
                        } else {
                            None
                        };
                        // If the register has fields, create a FieldSet for it.
                        let fieldset_id = if let Some(fields) = &r.fields {
                            let mut fieldset = FieldSet {
                                path: Path::new(
                                    vec![p.name.clone(), "fields".to_owned()],
                                    rname.clone(),
                                ),
                                description: r.description.clone(),
                                bit_size: 32, // todo
                                fields: Vec::new(),
                            };
                            for f in fields {
                                if f.enumerated_values.len() > 1 {
                                    warn!(
                                        "{}.{}.{}: multiple enumerated_values",
                                        p.name, r.name, f.name
                                    )
                                }
                                let enum_id = if f.enumerated_values.len() == 1 {
                                    let e = &f.enumerated_values[0];
                                    let enumm = Enum {
                                        path: Path::new(
                                            vec![p.name.clone(), "values".to_owned()],
                                            format!("{}_{}", rname, f.name),
                                        ),
                                        description: r.description.clone(),
                                        bit_size: f.bit_range.width,
                                        variants: e
                                            .values
                                            .iter()
                                            .map(|v| EnumVariant {
                                                description: v.description.clone(),
                                                name: v.name.clone(),
                                                value: v.value.unwrap() as _, // TODO what are variants without values used for??
                                            })
                                            .collect(),
                                    };
                                    Some(device.enums.put(enumm))
                                } else {
                                    None
                                };

                                fieldset.fields.push(Field {
                                    name: f.name.clone(),
                                    description: f.description.clone(),
                                    bit_offset: f.bit_range.offset,
                                    bit_size: f.bit_range.width,
                                    enumm: enum_id,
                                })
                            }
                            Some(device.fieldsets.put(fieldset))
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
                    svd::RegisterCluster::Cluster(r) => {
                        if let svd::Cluster::Array(_, dim) = r {
                            warn!("{}.{}: cluster array TODO", p.name, r.name)
                        }

                        warn!("{}.{}: cluster TODO", p.name, r.name)
                    }
                }
            }

            let block_id = device.blocks.put(block);
            device.peripherals.put(Peripheral {
                path: Path::new(vec![], p.name.clone()),
                block: block_id,
            });
        }
    }

    // Peripheral instances
    for p in &svd.peripherals {
        let peripheral_name = p.derived_from.as_ref().unwrap_or(&p.name);
        let (peripheral, _) = device
            .peripherals
            .find(|p| &p.path.name == peripheral_name)
            .unwrap();

        let item = PeripheralInstance {
            path: Path::new(vec![], p.name.clone()),
            description: p.description.clone(),
            base_address: p.base_address,

            peripheral,
        };
        device.instances.put(item);
    }

    device
}
