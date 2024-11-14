use serde::{Deserialize, Serialize};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct RenameEnumVariants {
    #[serde(rename = "enum")]
    pub enumm: RegexSet,
    pub from: RegexSet,
    pub to: String,
}

impl RenameEnumVariants {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        for id in match_all(ir.enums.keys().cloned(), &self.enumm) {
            let e = ir.enums.get_mut(&id).unwrap();
            for i in &mut e.variants {
                if let Some(name) = match_expand(&i.name, &self.from, &self.to) {
                    i.name = name;
                }
            }
        }
        Ok(())
    }
}
