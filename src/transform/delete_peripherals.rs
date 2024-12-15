use log::*;
use serde::{Deserialize, Serialize};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeletePeripherals {
    pub devices: RegexSet,
    pub from: RegexSet,
}

impl DeletePeripherals {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        for id in match_all(ir.devices.keys().cloned(), &self.devices) {
            let d = ir.devices.get_mut(&id).unwrap();
            d.peripherals.retain(|i| {
                info!("deleting peripheral {}", &i.name);
                !self.from.is_match(&i.name)
            });
        }
        Ok(())
    }
}
