use log::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteEnums {
    pub from: RegexSet,
    pub bit_size: Option<u32>,
    #[serde(default)]
    pub soft: bool,
    pub keep_desc: Option<bool>,
}

impl DeleteEnums {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        if self.keep_desc.unwrap_or(false) {
            let variant_desc = extract_variant_desc(ir, &self.from, self.bit_size)?;
            append_variant_desc_to_field(ir, &variant_desc, self.bit_size);
        }

        let mut ids: BTreeSet<String> = BTreeSet::new();
        for (id, e) in ir.enums.iter() {
            let bit_size_matches = self.bit_size.map_or(true, |s| s == e.bit_size);
            if self.from.is_match(id) && bit_size_matches {
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

pub(crate) fn remove_enum_ids(ir: &mut IR, from: &BTreeSet<String>) {
    for (_, fs) in ir.fieldsets.iter_mut() {
        for f in fs.fields.iter_mut() {
            if let Some(id) = &mut f.enumm {
                if from.contains(id) {
                    f.enumm = None
                }
            }
        }
    }
}
