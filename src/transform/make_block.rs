use log::*;
use serde::{Deserialize, Serialize};

use super::common::*;
use crate::ir::*;
use crate::transform::merge_blocks::block_compat;

#[derive(Debug, Serialize, Deserialize)]
pub struct MakeBlock {
    pub blocks: RegexSet,
    pub from: RegexSet,
    pub to_outer: String,
    pub to_block: String,
    pub to_inner: String,
    #[serde(default)]
    pub array_on_outer: bool,
    #[serde(default = "layout")]
    pub check: CheckLevel,
}

impl MakeBlock {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        let mut had_breaking_error = false;

        for id in match_all(ir.blocks.keys().cloned(), &self.blocks) {
            let b = ir.blocks.get_mut(&id).unwrap();

            // Mapping of new item function to existing item functions
            let groups = match_groups(
                b.items.iter().map(|f| f.name.clone()),
                &self.from,
                &self.to_outer,
            );

            for (to, group) in groups {
                let b = ir.blocks.get_mut(&id).unwrap();
                info!("blockifizing to {}", to);

                // Grab all items into a vec
                let mut items: Vec<_> =
                    b.items.iter().filter(|i| group.contains(&i.name)).collect();

                // Sort by offs
                items.sort_by_key(|i| i.byte_offset);
                for i in &items {
                    info!("    {}", i.name);
                }

                // todo check they're not arrays (arrays of arrays not supported)

                let byte_offset = items[0].byte_offset;
                let array = items[0].array.clone();

                let b2 = Block {
                    extends: None,
                    description: None,
                    items: items
                        .iter()
                        .map(|&i| {
                            let mut i = i.clone();
                            i.name = match_expand(&i.name, &self.from, &self.to_inner).unwrap();
                            i.byte_offset -= byte_offset;
                            if self.array_on_outer {
                                i.array = None; // Move array to outer block
                            }
                            i
                        })
                        .collect(),
                };

                let dest = self.to_block.clone(); // todo regex
                if let Some(prev) = ir.blocks.insert(dest.clone(), b2.clone()) {
                    let errors: Vec<_> = block_compat(ir, &prev, &b2)
                        .into_iter()
                        .map(|v| (dest.clone(), dest.clone(), v))
                        .collect();

                    had_breaking_error |= self
                        .check
                        .check(&format!("making block {dest} from {group:?}"), &errors)
                        .is_err();
                }

                // Remove all items
                let b = ir.blocks.get_mut(&id).unwrap();
                b.items.retain(|i| !group.contains(&i.name));

                // Create the new block item
                b.items.push(BlockItem {
                    name: to,
                    description: None,
                    array: if self.array_on_outer {
                        array // Take array from original items
                    } else {
                        None
                    },
                    byte_offset,
                    inner: BlockItemInner::Block(BlockItemBlock { block: dest }),
                });
            }
        }

        if had_breaking_error {
            anyhow::bail!("Failed to make block")
        }

        Ok(())
    }
}
