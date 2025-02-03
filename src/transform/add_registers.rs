use serde::{Deserialize, Serialize};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct AddRegisters {
    pub block: RegexSet,
    pub registers: Vec<BlockItem>,
}

impl AddRegisters {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        for id in match_all(ir.blocks.keys().cloned(), &self.block) {
            let d = ir.blocks.get_mut(&id).unwrap();
            d.items.extend(self.registers.clone());
        }

        Ok(())
    }
}
