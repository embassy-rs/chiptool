use log::*;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

use super::delete_enums::remove_enum_ids;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteEnumsWithVariants {
    variants: BTreeMap<u64, String>,
    #[serde(default)]
    pub soft: bool,
}

impl DeleteEnumsWithVariants {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        let mut ids: BTreeSet<String> = BTreeSet::new();

        'e: for (id, e) in &ir.enums {
            if e.variants.len() != self.variants.len() {
                continue;
            }
            for v in &e.variants {
                let Some(name) = self.variants.get(&v.value) else {
                    continue 'e;
                };
                if name != &v.name {
                    continue 'e;
                }
            }
            info!("deleting enum {}", id);
            ids.insert(id.clone());
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
