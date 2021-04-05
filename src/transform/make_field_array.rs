use log::*;
use serde::{Deserialize, Serialize};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct MakeFieldArray {
    pub fieldsets: String,
    pub from: String,
    pub to: String,
}

impl MakeFieldArray {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        let path_re = make_regex(&self.fieldsets)?;
        let re = make_regex(&self.from)?;
        for id in match_all(ir.fieldsets.keys().cloned(), &path_re) {
            let b = ir.fieldsets.get_mut(&id).unwrap();
            let groups = match_groups(b.fields.iter().map(|f| f.name.clone()), &re, &self.to);
            for (to, group) in groups {
                info!("arrayizing to {}", to);

                // Grab all items into a vec
                let mut items = Vec::new();
                for i in b.fields.iter().filter(|i| group.contains(&i.name)) {
                    items.push(i);
                }

                // Sort by offs
                items.sort_by_key(|i| i.bit_offset);
                for i in &items {
                    info!("    {}", i.name);
                }

                // todo check they're mergeable
                // todo check they're not arrays (arrays of arrays not supported)

                // Guess stride.
                let bit_offset = items[0].bit_offset;
                let len = items.len() as u32;
                let bit_stride = if len == 1 {
                    // If there's only 1 item, we can't know the stride, but it
                    // doesn't really matter!
                    0
                } else {
                    items[1].bit_offset - items[0].bit_offset
                };

                // Check the stride guess is OK

                if items
                    .iter()
                    .enumerate()
                    .any(|(n, i)| i.bit_offset != bit_offset + (n as u32) * bit_stride)
                {
                    panic!("arrayize: items are not evenly spaced")
                }

                info!("offs {} stride {}", bit_offset, bit_stride);

                let mut item = items[0].clone();

                // Remove all
                b.fields.retain(|i| !group.contains(&i.name));

                // Create the new array item
                item.name = to;
                item.array = Some(Array {
                    stride: bit_stride,
                    len,
                });
                item.bit_offset = bit_offset;
                b.fields.push(item);
            }
        }
        Ok(())
    }
}
