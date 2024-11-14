use serde::{Deserialize, Serialize};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct RenameInterrupts {
    pub from: String,
    pub to: String,
}

impl RenameInterrupts {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        let re = make_regex(&self.from)?;

        for d in ir.devices.values_mut() {
            for i in &mut d.interrupts {
                if let Some(name) = match_expand(&i.name, &re, &self.to) {
                    i.name = name;
                }
            }
        }
        Ok(())
    }
}
