use log::*;
use serde::{Deserialize, Serialize};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct MakeRegisterArray {
    pub blocks: String,
    pub from: String,
    pub to: String,
    #[serde(default)]
    pub allow_cursed: bool,
}

impl MakeRegisterArray {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        let path_re = make_regex(&self.blocks)?;
        let re = make_regex(&self.from)?;
        for id in match_all(ir.blocks.keys().cloned(), &path_re) {
            let b = ir.blocks.get_mut(&id).unwrap();
            let groups = match_groups(b.items.iter().map(|f| f.name.clone()), &re, &self.to);
            for (to, group) in groups {
                info!("arrayizing to {}", to);

                // Grab all items into a vec
                let mut items = Vec::new();
                for i in b.items.iter().filter(|i| group.contains(&i.name)) {
                    items.push(i);
                }

                // todo check they're mergeable
                // todo check they're not arrays (arrays of arrays not supported)

                // Sort by offs
                items.sort_by_key(|i| i.byte_offset);
                for i in &items {
                    info!("    {}", i.name);
                }

                let (offset, array) = calc_array(items.iter().map(|x| x.byte_offset).collect());
                if let Array::Cursed(_) = &array {
                    if !self.allow_cursed {
                        panic!("arrayize: items are not evenly spaced. Set `allow_cursed: true` to allow this.")
                    }
                }

                let mut item = items[0].clone();

                // Remove all
                b.items.retain(|i| !group.contains(&i.name));

                // Create the new array item
                item.name = to;
                item.array = Some(array);
                item.byte_offset = offset;
                b.items.push(item);
            }
        }
        Ok(())
    }
}
