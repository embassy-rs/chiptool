use log::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

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
        for (id, _fs) in ir.fieldsets.iter() {
            if re.is_match(id) {
                info!("deleting fieldset {}", id);
                ids.insert(id.clone());
            }
        }

        super::delete_fieldsets::remove_fieldset_ids(ir, &ids);

        for id in ids {
            ir.fieldsets.remove(&id);
        }

        let mut ids: HashSet<String> = HashSet::new();
        for (id, _e) in ir.enums.iter() {
            if re.is_match(id) {
                info!("deleting enum {}", id);
                ids.insert(id.clone());
            }
        }

        super::delete_enums::remove_enum_ids(ir, &ids);

        for id in ids {
            ir.enums.remove(&id);
        }

        let mut ids: HashSet<String> = HashSet::new();
        for (id, _b) in ir.blocks.iter() {
            if re.is_match(id) {
                info!("deleting block {}", id);
                ids.insert(id.clone());
            }
        }

        remove_block_ids(ir, &ids);

        for id in ids {
            ir.blocks.remove(&id);
        }

        Ok(())
    }
}

pub(crate) fn remove_block_ids(ir: &mut IR, from: &HashSet<String>) {
    for (_, b) in ir.blocks.iter_mut() {
        b.items.retain(|i| {
            if let BlockItemInner::Block(bi) = &i.inner {
                !from.contains(&bi.block)
            } else {
                true
            }
        });
    }

    for (_, d) in ir.devices.iter_mut() {
        d.peripherals.retain(|p| match &p.block {
            Some(block) => !from.contains(block),
            None => true,
        });
    }
}
