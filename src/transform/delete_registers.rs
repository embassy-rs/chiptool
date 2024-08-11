use serde::{Deserialize, Serialize};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteRegisters {
    pub block: String,
    pub from: String,
}

impl DeleteRegisters {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        let path_re = make_regex(&self.block)?;
        let re = make_regex(&self.from)?;
        for id in match_all(ir.blocks.keys().cloned(), &path_re) {
            let b = ir.blocks.get_mut(&id).unwrap();
            b.items.retain(|i| !re.is_match(&i.name));
        }
        Ok(())
    }
}
