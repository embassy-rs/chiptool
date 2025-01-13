use serde::{Deserialize, Serialize};

use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct FixRegisterBitSizes {
    pub create_fieldsets: bool,
}

impl FixRegisterBitSizes {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        for b in ir.blocks.values_mut() {
            for i in &mut b.items {
                if let BlockItemInner::Register(r) = &mut i.inner {
                    let orig_bit_size = r.bit_size;
                    let good_bit_size = match r.bit_size {
                        0..=8 => 8,
                        9..=16 => 16,
                        17..=32 => 32,
                        33..=64 => 64,
                        65.. => panic!("Invalid register bit size {}", r.bit_size),
                    };
                    if r.bit_size != good_bit_size {
                        r.bit_size = good_bit_size;
                        match &r.fieldset {
                            None => {
                                if self.create_fieldsets {
                                    // create a new fieldset, with a single field with the original bit size.
                                    r.fieldset = Some(i.name.clone());
                                    let fs = FieldSet {
                                        bit_size: good_bit_size,
                                        fields: vec![Field {
                                            name: "val".to_string(),
                                            bit_offset: BitOffset::Regular(0),
                                            bit_size: orig_bit_size,
                                            description: None,
                                            enumm: None,
                                            array: None,
                                        }],
                                        reset_value: None,
                                        description: None,
                                        extends: None,
                                    };
                                    if ir.fieldsets.insert(i.name.clone(), fs).is_some() {
                                        panic!("dup fieldset {}", i.name);
                                    }
                                }
                            }
                            Some(fs) => {
                                // expand the size of the existing fieldset.
                                let fs = ir.fieldsets.get_mut(fs).unwrap();
                                fs.bit_size = good_bit_size;
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
