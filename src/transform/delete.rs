use log::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Delete {
    pub from: String,
}

impl Delete {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        let re = make_regex(&self.from)?;

        let mut ids: HashSet<String> = HashSet::new();
        for (id, fs) in ir.fieldsets.iter() {
            if path_matches(&fs.path, &re) {
                info!("deleting fieldset {}", fs.path);
                ids.insert(id);
            }
        }

        super::delete_fieldsets::remove_fieldset_ids(ir, &ids);

        for id in ids {
            ir.fieldsets.remove(id)
        }

        let mut ids: HashSet<String> = HashSet::new();
        for (id, e) in ir.enums.iter() {
            if path_matches(&e.path, &re) {
                info!("deleting enum {}", e.path);
                ids.insert(id);
            }
        }

        super::delete_enums::remove_enum_ids(ir, &ids);

        for id in ids {
            ir.enums.remove(id)
        }

        let mut ids: HashSet<String> = HashSet::new();
        for (id, b) in ir.blocks.iter() {
            if path_matches(&b.path, &re) {
                info!("deleting block {}", b.path);
                ids.insert(id);
            }
        }

        remove_block_ids(ir, &ids);

        for id in ids {
            ir.blocks.remove(id)
        }

        Ok(())
    }
}

pub(crate) fn remove_block_ids(ir: &mut IR, from: &HashSet<String>) {
    for (_, b) in ir.blocks.iter_mut() {
        b.items.retain(|i| {
            if let BlockItemInner::Block(bid) = i.inner {
                !from.contains(&bid)
            } else {
                true
            }
        });
    }

    for (_, d) in ir.devices.iter_mut() {
        d.peripherals.retain(|p| !from.contains(&p.block));
    }
}
