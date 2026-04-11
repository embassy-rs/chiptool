use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct RenameRegisters {
    pub block: RegexSet,
    pub from: RegexSet,
    pub to: String,
    #[serde(default = "get_true")]
    pub error_on_duplicate: bool,
}

impl RenameRegisters {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        let mut renames = HashMap::new();
        let mut had_duplicate = false;

        for id in match_all(ir.blocks.keys().cloned(), &self.block) {
            let b = ir.blocks.get_mut(&id).unwrap();
            let renames = renames.entry(id.clone()).or_default();

            let fmt = |field| format!("register {field} in block {id}");

            for i in b.items.iter_mut() {
                if let Some(name) = match_expand(&i.name, &self.from, &self.to) {
                    had_duplicate |=
                        !can_rename(self.error_on_duplicate, renames, &name, &i.name, fmt);
                    i.name = name;
                }
            }
        }

        if had_duplicate && self.error_on_duplicate {
            anyhow::bail!("Duplicate use of new names");
        }

        Ok(())
    }
}
