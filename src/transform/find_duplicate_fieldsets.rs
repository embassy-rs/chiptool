use log::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct FindDuplicateFieldsets {}
impl FindDuplicateFieldsets {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        let mut suggested = HashSet::new();

        for (id1, fs1) in ir.fieldsets.iter() {
            if suggested.contains(&id1) {
                continue;
            }

            let mut ids = Vec::new();
            for (id2, fs2) in ir.fieldsets.iter() {
                if id1 != id2 && check_mergeable_fieldsets(fs1, fs2, CheckLevel::Names).is_ok() {
                    ids.push(id2)
                }
            }

            if !ids.is_empty() {
                ids.push(id1);
                info!("Duplicated fieldsets:");
                for id in ids {
                    suggested.insert(id);
                    info!("   {}", ir.fieldsets.get(id).path);
                }
            }
        }

        Ok(())
    }
}
