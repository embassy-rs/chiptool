use log::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct MergeFieldsets {
    pub from: String,
    pub to: String,
    pub main: Option<String>,
    #[serde(default)]
    pub check: CheckLevel,
}

impl MergeFieldsets {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        let re = make_regex(&self.from)?;
        let groups = match_groups(ir.fieldsets.keys().cloned(), &re, &self.to);

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
        ids: HashSet<String>,
        to: String,
        main: Option<&String>,
    ) -> anyhow::Result<()> {
        let mut main_id = ids.iter().next().unwrap().clone();
        if let Some(main) = main {
            let re = make_regex(main)?;
            for id in ids.iter() {
                if re.is_match(id) {
                    main_id = id.clone();
                    break;
                }
            }
        }
        let fs = ir.fieldsets.get(&main_id).unwrap().clone();

        for id in &ids {
            let fs2 = ir.fieldsets.get(id).unwrap();
            check_mergeable_fieldsets(&fs, fs2, self.check)?;
        }

        for id in &ids {
            ir.fieldsets.remove(id);
        }

        assert!(ir.fieldsets.insert(to.clone(), fs).is_none());
        replace_fieldset_ids(ir, &ids, to);

        Ok(())
    }
}
