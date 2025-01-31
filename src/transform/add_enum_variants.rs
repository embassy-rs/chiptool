use serde::{Deserialize, Serialize};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct AddEnumVariants {
    #[serde(rename = "enum")]
    pub enumm: RegexSet,
    pub variants: Vec<EnumVariant>,
}

impl AddEnumVariants {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        for id in match_all(ir.enums.keys().cloned(), &self.enumm) {
            let d = ir.enums.get_mut(&id).unwrap();
            d.variants.extend(self.variants.clone());
        }

        Ok(())
    }
}
