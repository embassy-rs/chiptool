use anyhow::bail;
use log::*;
use serde::{Deserialize, Serialize};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct MakeFieldArray {
    pub fieldsets: RegexSet,
    pub from: RegexSet,
    pub to: String,
    #[serde(default)]
    pub mode: ArrayMode,
}

impl MakeFieldArray {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        for id in match_all(ir.fieldsets.keys().cloned(), &self.fieldsets) {
            let b = ir.fieldsets.get_mut(&id).unwrap();
            let groups = match_groups(
                b.fields.iter().map(|f| f.name.clone()),
                &self.from,
                &self.to,
            );
            for (to, group) in groups {
                info!("arrayizing to {}", to);

                // Grab all items into a vec
                let mut items = Vec::new();
                for i in b.fields.iter().filter(|i| group.contains(&i.name)) {
                    items.push(i);
                }

                // todo check they're mergeable

                // one array shouldn't contain both regular and cursed bit_offset type
                {
                    let has_regular_bit_offset = items
                        .iter()
                        .any(|i| matches!(i.bit_offset, BitOffset::Regular(_)));

                    let has_cursed_bit_offset = items
                        .iter()
                        .any(|i| matches!(i.bit_offset, BitOffset::Cursed(_)));

                    if has_regular_bit_offset && has_cursed_bit_offset {
                        bail!("arrayize: items {} cannot mix bit_offset type", to)
                    }
                }

                // todo check they're not arrays (arrays of arrays not supported)

                // Sort by offs
                items.sort_by_key(|i| &i.bit_offset);
                for i in &items {
                    info!("    {}", i.name);
                }

                let (offset, array) = calc_array(
                    items.iter().map(|x| x.bit_offset.min_offset()).collect(),
                    self.mode,
                )?;

                let mut item = items[0].clone();

                // Remove all
                b.fields.retain(|i| !group.contains(&i.name));

                // Create the new array item
                item.name = to;
                item.array = Some(array);
                item.bit_offset = BitOffset::Regular(offset);
                b.fields.push(item);
            }
        }
        Ok(())
    }
}
