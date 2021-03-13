use log::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteEnums {
    pub from: String,
    pub bit_size: Option<u32>,
    #[serde(default)]
    pub soft: bool,
}

impl DeleteEnums {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        let re = make_regex(&self.from)?;

        let mut ids: HashSet<Id<Enum>> = HashSet::new();
        for (id, e) in ir.enums.iter() {
            let bit_size_matches = self.bit_size.map_or(true, |s| s == e.bit_size);
            if path_matches(&e.path, &re) && bit_size_matches {
                info!("deleting enum {}", e.path);
                ids.insert(id);
            }
        }

        remove_enum_ids(ir, &ids);

        if !self.soft {
            for id in ids {
                ir.enums.remove(id)
            }
        }

        Ok(())
    }
}

fn remove_enum_ids(ir: &mut IR, from: &HashSet<Id<Enum>>) {
    for (_, fs) in ir.fieldsets.iter_mut() {
        for f in fs.fields.iter_mut() {
            if let Some(id) = f.enumm {
                if from.contains(&id) {
                    f.enumm = None
                }
            }
        }
    }
}
