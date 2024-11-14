use serde::{Deserialize, Serialize};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct RenameFields {
    pub fieldset: RegexSet,
    pub from: RegexSet,
    pub to: String,
}

impl RenameFields {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        for id in match_all(ir.fieldsets.keys().cloned(), &self.fieldset) {
            let fs = ir.fieldsets.get_mut(&id).unwrap();
            for f in &mut fs.fields {
                if let Some(name) = match_expand(&f.name, &self.from, &self.to) {
                    f.name = name;
                }
            }
        }
        Ok(())
    }
}
