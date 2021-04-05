use log::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteFieldsets {
    pub from: String,
    #[serde(default)]
    pub useless: bool,
    #[serde(default)]
    pub soft: bool,
}

impl DeleteFieldsets {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        let re = make_regex(&self.from)?;

        let mut ids: HashSet<String> = HashSet::new();
        for (id, fs) in ir.fieldsets.iter() {
            if re.is_match(id) && (!self.useless | is_useless(fs)) {
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

fn is_useless(fs: &FieldSet) -> bool {
    match &fs.fields[..] {
        [] => true,
        [f] => {
            fs.bit_size == f.bit_size
                && f.bit_offset == 0
                && f.enum_read.is_none()
                && f.enum_write.is_none()
                && f.enum_readwrite.is_none()
        }
        _ => false,
    }
}

pub(crate) fn remove_fieldset_ids(ir: &mut IR, from: &HashSet<String>) {
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
