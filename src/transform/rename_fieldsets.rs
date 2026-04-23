use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct RenameFieldsets {
    pub from: RegexSet,
    pub to: String,
}

impl RenameFieldsets {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        let mut renames = HashMap::new();
        let mut mapping = HashMap::new();
        let mut had_duplicate = false;

        for id in match_all(ir.fieldsets.keys().cloned(), &self.from) {
            let renames = renames.entry(id.clone()).or_default();

            let fmt = |id| format!("fieldset {id}");

            if let Some(name) = match_expand(&id, &self.from, &self.to) {
                let can_rename = can_rename(true, renames, &name, &id, fmt);
                had_duplicate |= !can_rename;

                if can_rename {
                    log::info!("Renaming fieldset '{}' to '{}'", id, name);
                }

                let removed = ir.fieldsets.remove(&id).expect("Match should be in set");
                ir.fieldsets.insert(name.clone(), removed);
                mapping.insert(id.clone(), name.clone());
            }
        }

        ir.blocks
            .iter_mut()
            .flat_map(|(_, b)| b.items.iter_mut())
            .filter_map(|i| match &mut i.inner {
                BlockItemInner::Block(_) => None,
                BlockItemInner::Register(register) => Some(register),
            })
            .for_each(|r| {
                let Some(fieldset) = r.fieldset.as_mut() else {
                    return;
                };

                let Some(new_name) = mapping.get(fieldset) else {
                    return;
                };

                *fieldset = new_name.clone();
            });

        if had_duplicate {
            anyhow::bail!("Duplicate use of new names");
        }

        Ok(())
    }
}
