use serde::{Deserialize, Serialize};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct RenameEnumVariants {
    #[serde(rename = "enum")]
    pub enumm: String,
    pub from: String,
    pub to: String,
}

impl RenameEnumVariants {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        let path_re = make_regex(&self.enumm)?;
        let re = make_regex(&self.from)?;
        for id in match_all(ir.enums.keys().cloned(), &path_re) {
            let e = ir.enums.get_mut(&id).unwrap();
            for i in &mut e.variants {
                if let Some(name) = match_expand(&i.name, &re, &self.to) {
                    i.name = name;
                }
            }
        }
        Ok(())
    }
}
