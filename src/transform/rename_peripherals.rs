use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct RenamePeripherals {
    pub from: RegexSet,
    pub to: String,
    #[serde(default = "get_true")]
    pub error_on_duplicate: bool,
}

impl RenamePeripherals {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        let mut had_duplicate = false;

        for (dev_name, d) in ir.devices.iter_mut() {
            let mut renames = HashMap::new();
            let fmt = |peri| format!("peripheral {peri} for device {dev_name}");

            for p in &mut d.peripherals {
                if let Some(name) = match_expand(&p.name, &self.from, &self.to) {
                    had_duplicate |=
                        !can_rename(self.error_on_duplicate, &mut renames, &name, &p.name, fmt);
                    p.name = name;
                }
            }
        }

        if had_duplicate && self.error_on_duplicate {
            anyhow::bail!("Failed to rename peripherals");
        }

        Ok(())
    }
}
