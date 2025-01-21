use serde::{Deserialize, Serialize};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct AddInterrupts {
    pub devices: RegexSet,
    pub interrupts: Vec<Interrupt>,
}

impl AddInterrupts {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        for id in match_all(ir.devices.keys().cloned(), &self.devices) {
            let d = ir.devices.get_mut(&id).unwrap();
            d.interrupts.extend(self.interrupts.clone());
        }
        Ok(())
    }
}
