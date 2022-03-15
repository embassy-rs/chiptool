use serde::{Deserialize, Serialize};

use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Sort {}

impl Sort {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        for z in ir.blocks.values_mut() {
            z.items.sort_by_key(|i| (i.byte_offset, i.name.clone()))
        }
        for z in ir.fieldsets.values_mut() {
            z.fields.sort_by_key(|i| (i.bit_offset, i.name.clone()))
        }
        for z in ir.enums.values_mut() {
            z.variants.sort_by_key(|i| (i.value, i.name.clone()))
        }

        Ok(())
    }
}
