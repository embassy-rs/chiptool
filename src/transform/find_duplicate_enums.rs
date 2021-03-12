use log::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct FindDuplicateEnums {}
impl FindDuplicateEnums {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        let mut suggested = HashSet::new();

        for (id1, e1) in ir.enums.iter() {
            if suggested.contains(&id1) {
                continue;
            }

            let mut ids = Vec::new();
            for (id2, e2) in ir.enums.iter() {
                if id1 != id2 && mergeable_enums(e1, e2) {
                    ids.push(id2)
                }
            }

            if !ids.is_empty() {
                ids.push(id1);
                info!("Duplicated enums:");
                for id in ids {
                    suggested.insert(id);
                    info!("   {}", ir.enums.get(id).path);
                }
            }
        }

        Ok(())
    }
}
