use serde::{Deserialize, Serialize};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct ModifyByteOffset {
    pub block: String,
    pub add_offset: u32,
}

impl ModifyByteOffset {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        let path_re = make_regex(&self.block)?;
        for id in match_all(ir.blocks.keys().cloned(), &path_re) {
            let b = ir.blocks.get_mut(&id).unwrap();
            for i in &mut b.items {
                i.byte_offset += self.add_offset;
            }
        }
        Ok(())
    }
}
