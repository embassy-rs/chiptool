use serde::{Deserialize, Serialize};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct ModifyFieldsEnum {
    pub fieldset: RegexSet,
    pub field: RegexSet,
    #[serde(rename = "enum")]
    pub enumm: RegexSet,
}

impl ModifyFieldsEnum {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        let matched_enums = match_all(ir.enums.keys().cloned(), &self.enumm);
        if matched_enums.len() != 1 {
            anyhow::bail!(
                "Expected exactly one enum to match, found {}",
                matched_enums.len()
            );
        }
        let enum_id = matched_enums.first().unwrap().clone();

        for id in match_all(ir.fieldsets.keys().cloned(), &self.fieldset) {
            let fs = ir.fieldsets.get_mut(&id).unwrap();
            fs.fields
                .iter_mut()
                .filter(|f| self.field.is_match(&f.name))
                .for_each(|f| f.enumm = Some(enum_id.clone()));
        }

        Ok(())
    }
}
