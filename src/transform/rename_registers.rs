use serde::{Deserialize, Serialize};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct RenameRegisters {
    pub block: RegexSet,
    pub from: RegexSet,
    pub to: String,
}

impl RenameRegisters {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        for id in match_all(ir.blocks.keys().cloned(), &self.block) {
            let b = ir.blocks.get_mut(&id).unwrap();
            for i in &mut b.items {
                if let Some(name) = match_expand(&i.name, &self.from, &self.to) {
                    i.name = name;
                }
            }
        }
        Ok(())
    }
}
