use log::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct MakeBlock {
    pub block: String,
    pub from: String,
    pub to_outer: String,
    pub to_block: String,
    pub to_inner: String,
}

impl MakeBlock {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        let path_re = make_regex(&self.block)?;
        let re = make_regex(&self.from)?;
        for id in match_paths(&ir.blocks, &path_re) {
            let b = ir.blocks.get_mut(id);
            let groups = string_groups(b.items.iter().map(|f| f.name.clone()), &re, &self.to_outer);
            for (to, group) in groups {
                let b = ir.blocks.get_mut(id);
                info!("blockifizing to {}", to);

                // Grab all items into a vec
                let mut items = Vec::new();
                for i in b.items.iter().filter(|i| group.contains(&i.name)) {
                    items.push(i);
                }

                // Sort by offs
                items.sort_by_key(|i| i.byte_offset);
                for i in &items {
                    info!("    {}", i.name);
                }

                // todo check they're mergeable
                // todo check they're not arrays (arrays of arrays not supported)

                let byte_offset = items[0].byte_offset;
                let len = items.len() as u32;
                let byte_stride = if len == 1 {
                    // If there's only 1 item, we can't know the stride, but it
                    // doesn't really matter!
                    0
                } else {
                    items[1].byte_offset - items[0].byte_offset
                };

                let b2 = Block {
                    path: Path::new_from_string(&self.to_block), // todo regex
                    description: None,
                    items: items
                        .iter()
                        .map(|&i| {
                            let mut i = i.clone();
                            i.name = string_match_expand(&i.name, &re, &self.to_inner).unwrap();
                            i.byte_offset -= byte_offset;
                            i
                        })
                        .collect(),
                };
                let b2_id = if let Some((id, b3)) = ir.blocks.find(|b| b.path == b2.path) {
                    // todo check blocks are mergeable
                    id
                } else {
                    ir.blocks.put(b2)
                };

                // Remove all items
                let b = ir.blocks.get_mut(id);
                b.items.retain(|i| !group.contains(&i.name));

                // Create the new block item
                b.items.push(BlockItem {
                    name: to,
                    description: None,
                    array: None,
                    byte_offset,
                    inner: BlockItemInner::Block(b2_id),
                });
            }
        }
        Ok(())
    }
}
