use anyhow::Context;
use log::*;
use serde::{Deserialize, Serialize};

use super::common::*;
use crate::ir::*;
use crate::transform::merge_blocks::block_item_compat;

#[derive(Debug, Serialize, Deserialize)]
pub struct MakeRegisterArray {
    pub blocks: RegexSet,
    pub from: RegexSet,
    pub to: String,
    #[serde(default)]
    pub mode: ArrayMode,
    // Arrays are often numbered, so we generally care less about the naming
    #[serde(default = "layout")]
    pub check: CheckLevel,
}

impl MakeRegisterArray {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        let mut errors = Vec::new();

        for id in match_all(ir.blocks.keys().cloned(), &self.blocks) {
            let mut b = ir.blocks.get(&id).unwrap().clone();
            let groups = match_groups(b.items.iter().map(|f| f.name.clone()), &self.from, &self.to);
            for (to, group) in groups {
                info!("arrayizing to {}", to);

                // Grab all items into a vec
                let mut items = Vec::new();
                for i in b.items.iter().filter(|i| group.contains(&i.name)) {
                    items.push(i);
                }

                let mut iter = items.iter();
                let main = iter.next().unwrap();

                for other in iter {
                    errors.extend(
                        block_item_compat(ir, main, other)
                            .into_iter()
                            .map(|v| (main.name.clone(), other.name.clone(), v)),
                    );
                }

                // todo check they're not arrays (arrays of arrays not supported)

                // Sort by offs
                items.sort_by_key(|i| i.byte_offset);
                for i in &items {
                    info!("    {}", i.name);
                }

                let (offset, array) =
                    calc_array(items.iter().map(|x| x.byte_offset).collect(), self.mode)?;

                let mut item = items[0].clone();

                // Remove all
                b.items.retain(|i| !group.contains(&i.name));

                // Create the new array item
                item.name = to;
                item.array = Some(array);
                item.byte_offset = offset;
                b.items.push(item);
            }

            ir.blocks.insert(id.clone(), b);
        }

        self.check
            .check(module_path!(), "making/merging register arrays", &errors)
            .context("failed to make register array")?;

        Ok(())
    }
}
