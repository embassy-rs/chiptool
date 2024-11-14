use serde::{Deserialize, Serialize};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteRegisters {
    pub block: RegexSet,
    pub from: RegexSet,
}

impl DeleteRegisters {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        for id in match_all(ir.blocks.keys().cloned(), &self.block) {
            let b = ir.blocks.get_mut(&id).unwrap();
            b.items.retain(|i| !self.from.is_match(&i.name));
        }
        Ok(())
    }
}
