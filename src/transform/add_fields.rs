use serde::{Deserialize, Serialize};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct AddFields {
    pub fieldset: RegexSet,
    pub fields: Vec<Field>,
}

impl AddFields {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        for id in match_all(ir.fieldsets.keys().cloned(), &self.fieldset) {
            let d = ir.fieldsets.get_mut(&id).unwrap();
            d.fields.extend(self.fields.clone());
        }

        Ok(())
    }
}
