use serde::{Deserialize, Serialize};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct RenameRegisters {
    pub block: String,
    pub from: String,
    pub to: String,
}

impl RenameRegisters {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        let path_re = make_regex(&self.block)?;
        let re = make_regex(&self.from)?;
        for id in match_all(ir.blocks.keys().cloned(), &path_re) {
            let b = ir.blocks.get_mut(&id).unwrap();
            for i in &mut b.items {
                if let Some(name) = match_expand(&i.name, &re, &self.to) {
                    i.name = name;
                }
            }
        }
        Ok(())
    }
}
