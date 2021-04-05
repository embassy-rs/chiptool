use log::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct MergeEnums {
    pub from: String,
    pub to: String,
}

impl MergeEnums {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        let re = make_regex(&self.from)?;
        let groups = path_groups(&ir.enums, &re, &self.to);

        for (to, group) in groups {
            info!("Merging enums, dest: {}", to);
            for id in &group {
                info!("   {}", ir.enums.get(*id).path);
            }
            self.merge_enums(ir, group, to);
        }

        Ok(())
    }

    fn merge_enums(&self, ir: &mut IR, ids: HashSet<String>, to: Path) {
        let mut e = ir.enums.get(*ids.iter().next().unwrap()).clone();

        for id in &ids {
            let e2 = ir.enums.get(*id);
            if !mergeable_enums(&e, e2) {
                panic!("mergeing nonmergeable enums");
            }
        }

        e.path = to;
        let final_id = ir.enums.put(e);
        replace_enum_ids(ir, &ids, final_id);
        for id in ids {
            ir.enums.remove(id)
        }
    }
}
