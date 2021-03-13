use log::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct MergeBlocks {
    pub from: String,
    pub to: String,
    pub main: Option<String>,
    #[serde(default)]
    pub check: CheckLevel,
}

impl MergeBlocks {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        let re = make_regex(&self.from)?;
        let groups = path_groups(&ir.blocks, &re, &self.to);

        for (to, group) in groups {
            info!("Merging fieldsets, dest: {}", to);
            for id in &group {
                info!("   {}", ir.blocks.get(*id).path);
            }
            self.merge_blocks(ir, group, to, self.main.as_ref())?;
        }

        Ok(())
    }

    fn merge_blocks(
        &self,
        ir: &mut IR,
        ids: HashSet<Id<Block>>,
        to: Path,
        main: Option<&String>,
    ) -> anyhow::Result<()> {
        let mut main_id = *ids.iter().next().unwrap();
        if let Some(main) = main {
            let re = make_regex(main)?;
            for &id in ids.iter() {
                let fs = ir.blocks.get(id);
                if re.is_match(&fs.path.to_string()) {
                    main_id = id;
                    break;
                }
            }
        }
        let mut fs = ir.blocks.get(main_id).clone();

        for id in &ids {
            let fs2 = ir.blocks.get(*id);
            // todo
            //check_mergeable_blocks(&fs, fs2, self.check)?;
        }

        fs.path = to;
        let final_id = ir.blocks.put(fs);
        replace_block_ids(ir, &ids, final_id);
        for id in ids {
            ir.blocks.remove(id)
        }

        Ok(())
    }
}
