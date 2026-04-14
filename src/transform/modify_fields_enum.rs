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
        let mut enum_id = None;

        for id in match_all(ir.fieldsets.keys().cloned(), &self.fieldset) {
            let enum_id = match &enum_id {
                Some(enum_id) => enum_id,
                None => {
                    let matched_enums = match_all(ir.enums.keys().cloned(), &self.enumm);
                    if matched_enums.len() != 1 {
                        anyhow::bail!(
                            "Expected exactly one enum to match, found {}",
                            matched_enums.len()
                        );
                    }
                    enum_id.insert(matched_enums.first().unwrap().clone())
                }
            };

            let fs = get_mut!(ir, fieldsets, &id)?;
            fs.fields
                .iter_mut()
                .filter(|f| self.field.is_match(&f.name))
                .for_each(|f| f.enumm = Some(enum_id.clone()));
        }

        Ok(())
    }
}
