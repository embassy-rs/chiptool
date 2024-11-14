use log::*;
use serde::{Deserialize, Serialize};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct MakeBlock {
    pub blocks: RegexSet,
    pub from: RegexSet,
    pub to_outer: String,
    pub to_block: String,
    pub to_inner: String,
}

impl MakeBlock {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        for id in match_all(ir.blocks.keys().cloned(), &self.blocks) {
            let b = ir.blocks.get_mut(&id).unwrap();
            let groups = match_groups(
                b.items.iter().map(|f| f.name.clone()),
                &self.from,
                &self.to_outer,
            );
            for (to, group) in groups {
                let b = ir.blocks.get_mut(&id).unwrap();
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

                let b2 = Block {
                    extends: None,
                    description: None,
                    items: items
                        .iter()
                        .map(|&i| {
                            let mut i = i.clone();
                            i.name = match_expand(&i.name, &self.from, &self.to_inner).unwrap();
                            i.byte_offset -= byte_offset;
                            i
                        })
                        .collect(),
                };

                // TODO if destination block exists, check mergeable
                let dest = self.to_block.clone(); // todo regex
                ir.blocks.insert(dest.clone(), b2);

                // Remove all items
                let b = ir.blocks.get_mut(&id).unwrap();
                b.items.retain(|i| !group.contains(&i.name));

                // Create the new block item
                b.items.push(BlockItem {
                    name: to,
                    description: None,
                    array: None,
                    byte_offset,
                    inner: BlockItemInner::Block(BlockItemBlock { block: dest }),
                });
            }
        }
        Ok(())
    }
}
