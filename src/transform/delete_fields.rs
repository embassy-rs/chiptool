use serde::{Deserialize, Serialize};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteFields {
    pub fieldset: RegexSet,
    pub from: RegexSet,
}

impl DeleteFields {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        for id in match_all(ir.fieldsets.keys().cloned(), &self.fieldset) {
            let fs = ir.fieldsets.get_mut(&id).unwrap();
            fs.fields.retain(|f| !self.from.is_match(&f.name));
        }
        Ok(())
    }
}
