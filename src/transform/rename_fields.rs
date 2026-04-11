use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct RenameFields {
    pub fieldset: RegexSet,
    pub from: RegexSet,
    pub to: String,
    #[serde(default = "get_true")]
    pub error_on_duplicate: bool,
}

impl RenameFields {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        let mut renames = HashMap::new();
        let mut had_duplicate = false;

        for id in match_all(ir.fieldsets.keys().cloned(), &self.fieldset) {
            let fs = ir.fieldsets.get_mut(&id).unwrap();
            let renames = renames.entry(id.clone()).or_default();

            let fmt = |field| format!("field {field} in fieldset {id}");

            for f in &mut fs.fields {
                if let Some(name) = match_expand(&f.name, &self.from, &self.to) {
                    had_duplicate |=
                        !can_rename(self.error_on_duplicate, renames, &name, &f.name, fmt);
                    f.name = name;
                }
            }
        }

        if had_duplicate && self.error_on_duplicate {
            anyhow::bail!("Duplicate use of new names");
        }

        Ok(())
    }
}
