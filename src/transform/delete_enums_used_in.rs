use log::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

use super::common::*;
use super::delete_enums::remove_enum_ids;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteEnumsUsedIn {
    pub fieldsets: String,
    #[serde(default)]
    pub soft: bool,
}

impl DeleteEnumsUsedIn {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        let re = make_regex(&self.fieldsets)?;
        let mut ids: BTreeSet<String> = BTreeSet::new();

        for (id, fs) in ir.fieldsets.iter() {
            if re.is_match(id) {
                info!("matched fieldset {}", id);
                for f in &fs.fields {
                    if let Some(id) = &f.enumm {
                        info!("deleting enum {}", id);
                        ids.insert(id.clone());
                    }
                }
            }
        }

        remove_enum_ids(ir, &ids);

        if !self.soft {
            for id in ids {
                ir.enums.remove(&id);
            }
        }

        Ok(())
    }
}
