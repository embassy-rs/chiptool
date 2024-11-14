use log::*;
use serde::{Deserialize, Serialize};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct MakeRegisterArray {
    pub blocks: RegexSet,
    pub from: RegexSet,
    pub to: String,
    #[serde(default)]
    pub mode: ArrayMode,
}

impl MakeRegisterArray {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        for id in match_all(ir.blocks.keys().cloned(), &self.blocks) {
            let b = ir.blocks.get_mut(&id).unwrap();
            let groups = match_groups(b.items.iter().map(|f| f.name.clone()), &self.from, &self.to);
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
        }
        Ok(())
    }
}
