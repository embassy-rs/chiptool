use log::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct MergeFieldsets {
    pub from: RegexSet,
    pub to: String,
    pub main: Option<RegexSet>,
    #[serde(default)]
    pub check: CheckLevel,
}

impl MergeFieldsets {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        let groups = match_groups(ir.fieldsets.keys().cloned(), &self.from, &self.to);

        for (to, group) in groups {
            info!("Merging fieldsets, dest: {}", to);
            for id in &group {
                info!("   {}", id);
            }
            self.merge_fieldsets(ir, group, to, self.main.as_ref())?;
        }

        Ok(())
    }

    fn merge_fieldsets(
        &self,
        ir: &mut IR,
        ids: BTreeSet<String>,
        to: String,
        main: Option<&RegexSet>,
    ) -> anyhow::Result<()> {
        let mut main_id = ids.iter().next().unwrap().clone();
        if let Some(main) = main {
            for id in ids.iter() {
                if main.is_match(id) {
                    main_id = id.clone();
                    break;
                }
            }
        }
        let fs = ir.fieldsets.get(&main_id).unwrap().clone();

        for id in &ids {
            let fs2 = ir.fieldsets.get(id).unwrap();
            check_mergeable_fieldsets(&main_id, &fs, id, fs2, self.check)?;
        }

        for id in &ids {
            ir.fieldsets.remove(id);
        }

        assert!(ir.fieldsets.insert(to.clone(), fs).is_none());
        replace_fieldset_ids(ir, &ids, to);

        Ok(())
    }
}
