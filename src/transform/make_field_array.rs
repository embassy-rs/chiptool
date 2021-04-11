use log::*;
use serde::{Deserialize, Serialize};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct MakeFieldArray {
    pub fieldsets: String,
    pub from: String,
    pub to: String,
    #[serde(default)]
    pub allow_cursed: bool,
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

                // todo check they're mergeable
                // todo check they're not arrays (arrays of arrays not supported)

                // Sort by offs
                items.sort_by_key(|i| i.bit_offset);
                for i in &items {
                    info!("    {}", i.name);
                }

                let (offset, array) = calc_array(items.iter().map(|x| x.bit_offset).collect());
                if let Array::Cursed(_) = &array {
                    if !self.allow_cursed {
                        panic!("arrayize: items are not evenly spaced. Set `allow_cursed: true` to allow this.")
                    }
                }

                let mut item = items[0].clone();

                // Remove all
                b.fields.retain(|i| !group.contains(&i.name));

                // Create the new array item
                item.name = to;
                item.array = Some(array);
                item.bit_offset = offset;
                b.fields.push(item);
            }
        }
        Ok(())
    }
}
