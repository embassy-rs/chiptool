use log::*;
use serde::{Deserialize, Serialize};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteEnumVariants {
    #[serde(rename = "enum")]
    pub enumm: RegexSet,
    pub from: RegexSet,
}

impl DeleteEnumVariants {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        for id in match_all(ir.enums.keys().cloned(), &self.enumm) {
            let e = ir.enums.get_mut(&id).unwrap();

            e.variants.retain(|variant| {
                if self.from.is_match(&variant.name) {
                    info!("deleting enum variant {}::{}", id, &variant.name);
                    return false;
                }

                true
            });
        }

        Ok(())
    }
}
