use log::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

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

        let mut ids: HashSet<String> = HashSet::new();
        for (id, e) in ir.enums.iter() {
            let bit_size_matches = self.bit_size.map_or(true, |s| s == e.bit_size);
            if re.is_match(id) && bit_size_matches {
                info!("deleting enum {}", id);
                ids.insert(id.clone());
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

pub(crate) fn remove_enum_ids(ir: &mut IR, from: &HashSet<String>) {
    for (_, fs) in ir.fieldsets.iter_mut() {
        for f in fs.fields.iter_mut() {
            for e in [&mut f.enum_read, &mut f.enum_write, &mut f.enum_readwrite].into_iter() {
                if let Some(id) = e {
                    if from.contains(id) {
                        *e = None
                    }
                }
            }
        }
    }
}
