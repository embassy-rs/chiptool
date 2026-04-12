use serde::{Deserialize, Serialize};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct AddPeripherals {
    pub devices: RegexSet,
    pub peripherals: Vec<Peripheral>,
}

impl AddPeripherals {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        for id in match_all(ir.devices.keys().cloned(), &self.devices) {
            let d = ir.devices.get_mut(&id).unwrap();
            d.peripherals.extend(self.peripherals.clone());
        }
        Ok(())
    }
}
