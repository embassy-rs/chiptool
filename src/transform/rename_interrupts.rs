use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct RenameInterrupts {
    pub from: RegexSet,
    pub to: String,
    #[serde(default = "get_true")]
    pub error_on_duplicate: bool,
}

impl RenameInterrupts {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        let mut had_duplicate = false;

        for (dev_name, d) in ir.devices.iter_mut() {
            let mut renames = HashMap::new();
            let fmt = |itr| format!("interrupt {itr} for device {dev_name}");

            for i in &mut d.interrupts {
                if let Some(name) = match_expand(&i.name, &self.from, &self.to) {
                    had_duplicate |=
                        !can_rename(self.error_on_duplicate, &mut renames, &name, &i.name, fmt);
                    i.name = name;
                }
            }
        }

        if had_duplicate && self.error_on_duplicate {
            anyhow::bail!("Failed to rename interrupts");
        }

        Ok(())
    }
}
