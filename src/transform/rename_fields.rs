use serde::{Deserialize, Serialize};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct RenameFields {
    pub fieldset: String,
    pub from: String,
    pub to: String,
}

impl RenameFields {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        let path_re = make_regex(&self.fieldset)?;
        let re = make_regex(&self.from)?;
        for id in match_paths(&ir.fieldsets, &path_re) {
            let fs = ir.fieldsets.get_mut(id);
            for f in &mut fs.fields {
                if let Some(name) = string_match_expand(&f.name, &re, &self.to) {
                    f.name = name;
                }
            }
        }
        Ok(())
    }
}
