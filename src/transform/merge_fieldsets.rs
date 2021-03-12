use log::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct MergeFieldsets {
    pub from: String,
    pub to: String,
    #[serde(default)]
    pub check: FieldsetMergeCheck,
}

impl MergeFieldsets {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        let re = make_regex(&self.from)?;
        let groups = path_groups(&ir.fieldsets, &re, &self.to);

        for (to, group) in groups {
            info!("Merging fieldsets, dest: {}", to);
            for id in &group {
                info!("   {}", ir.fieldsets.get(*id).path);
            }
            self.merge_fieldsets(ir, group, to)?;
        }

        Ok(())
    }

    fn merge_fieldsets(
        &self,
        ir: &mut IR,
        ids: HashSet<Id<FieldSet>>,
        to: Path,
    ) -> anyhow::Result<()> {
        let mut fs = ir.fieldsets.get(*ids.iter().next().unwrap()).clone();

        for id in &ids {
            let fs2 = ir.fieldsets.get(*id);
            check_mergeable_fieldsets(&fs, fs2, self.check)?;
        }

        fs.path = to;
        let final_id = ir.fieldsets.put(fs);
        replace_fieldset_ids(ir, &ids, final_id);
        for id in ids {
            ir.fieldsets.remove(id)
        }

        Ok(())
    }
}
