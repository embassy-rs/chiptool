use log::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteFieldsets {
    pub from: RegexSet,
    #[serde(default)]
    pub useless: bool,
    #[serde(default)]
    pub soft: bool,
}

impl DeleteFieldsets {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        let mut ids: BTreeSet<String> = BTreeSet::new();
        for (id, fs) in ir.fieldsets.iter() {
            if self.from.is_match(id) && (!self.useless | is_useless(fs)) {
                info!("deleting fieldset {}", id);
                ids.insert(id.clone());
            }
        }

        remove_fieldset_ids(ir, &ids);

        if !self.soft {
            for id in ids {
                ir.fieldsets.remove(&id);
            }
        }

        Ok(())
    }
}

// Fieldset is useless when
// 1. it has no Fields, or
// 2. it has one Fields, which occupied entire Fieldset, and without a enum
fn is_useless(fs: &FieldSet) -> bool {
    match &fs.fields[..] {
        [] => true,
        [f] => fs.bit_size == f.bit_size && f.bit_offset.min_offset() == 0 && f.enumm.is_none(),
        _ => false,
    }
}

pub(crate) fn remove_fieldset_ids(ir: &mut IR, from: &BTreeSet<String>) {
    for (_, b) in ir.blocks.iter_mut() {
        for i in b.items.iter_mut() {
            if let BlockItemInner::Register(reg) = &mut i.inner {
                if let Some(id) = &reg.fieldset {
                    if from.contains(id) {
                        reg.fieldset = None
                    }
                }
            }
        }
    }
}
