use log::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

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
        let groups = path_groups(&ir.fieldsets, &re, &self.to);

        for (to, group) in groups {
            info!("Merging fieldsets, dest: {}", to);
            for id in &group {
                info!("   {}", ir.fieldsets.get(*id).path);
            }
            self.merge_fieldsets(ir, group, to, self.main.as_ref())?;
        }

        Ok(())
    }

    fn merge_fieldsets(
        &self,
        ir: &mut IR,
        ids: HashSet<String>,
        to: Path,
        main: Option<&String>,
    ) -> anyhow::Result<()> {
        let mut main_id = *ids.iter().next().unwrap();
        if let Some(main) = main {
            let re = make_regex(main)?;
            for &id in ids.iter() {
                let fs = ir.fieldsets.get(id);
                if re.is_match(&fs.path.to_string()) {
                    main_id = id;
                    break;
                }
            }
        }
        let mut fs = ir.fieldsets.get(main_id).clone();

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
