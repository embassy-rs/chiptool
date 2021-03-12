use log::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct MakeRegisterArray {
    pub block: String,
    pub from: String,
    pub to: String,
}

impl MakeRegisterArray {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        let path_re = make_regex(&self.block)?;
        let re = make_regex(&self.from)?;
        for id in match_paths(&ir.blocks, &path_re) {
            let b = ir.blocks.get_mut(id);
            let groups = string_groups(b.items.iter().map(|f| f.name.clone()), &re, &self.to);
            for (to, group) in groups {
                info!("arrayizing to {}", to);

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

                // Guess stride.
                let byte_offset = items[0].byte_offset;
                let len = items.len() as u32;
                let byte_stride = if len == 1 {
                    // If there's only 1 item, we can't know the stride, but it
                    // doesn't really matter!
                    0
                } else {
                    items[1].byte_offset - items[0].byte_offset
                };

                // Check the stride guess is OK

                if items
                    .iter()
                    .enumerate()
                    .any(|(n, i)| i.byte_offset != byte_offset + (n as u32) * byte_stride)
                {
                    panic!("arrayize: items are not evenly spaced")
                }

                info!("offs {} stride {}", byte_offset, byte_stride);

                let mut item = items[0].clone();

                // Remove all
                b.items.retain(|i| !group.contains(&i.name));

                // Create the new array item
                item.name = to;
                item.array = Some(Array { byte_stride, len });
                item.byte_offset = byte_offset;
                b.items.push(item);
            }
        }
        Ok(())
    }
}
