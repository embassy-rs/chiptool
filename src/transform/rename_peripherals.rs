use serde::{Deserialize, Serialize};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct RenamePeripherals {
    pub from: RegexSet,
    pub to: String,
}

impl RenamePeripherals {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        for d in ir.devices.values_mut() {
            for p in &mut d.peripherals {
                if let Some(name) = match_expand(&p.name, &self.from, &self.to) {
                    p.name = name;
                }
            }
        }
        Ok(())
    }
}
