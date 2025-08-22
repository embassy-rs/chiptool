use serde::{Deserialize, Serialize};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct ModifyRegisters {
    pub blocks: RegexSet,
    pub registers: RegexSet,
    pub fieldset: Option<String>,
}

impl ModifyRegisters {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        for id in match_all(ir.blocks.keys().cloned(), &self.blocks) {
            let block = ir.blocks.get_mut(&id).unwrap();

            for item in block
                .items
                .iter_mut()
                .filter(|i| self.registers.is_match(&i.name))
            {
                let BlockItemInner::Register(reg) = &mut item.inner else {
                    continue;
                };

                if let Some(fieldset) = &self.fieldset {
                    reg.fieldset = Some(fieldset.clone());
                }
            }
        }
        Ok(())
    }
}
