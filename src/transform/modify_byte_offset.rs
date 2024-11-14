use serde::{Deserialize, Serialize};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct ModifyByteOffset {
    pub blocks: RegexSet,
    pub exclude_items: Option<RegexSet>,
    pub add_offset: i32,
    pub strict: Option<bool>, // if this value is false, bypass overflowed/underflowed modification
}

impl ModifyByteOffset {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        let strict = self.strict.unwrap_or_default();

        let mut err_names = Vec::new();

        for id in match_all(ir.blocks.keys().cloned(), &self.blocks) {
            let b = ir.blocks.get_mut(&id).unwrap();
            for i in &mut b.items {
                if let Some(exclude) = &self.exclude_items {
                    if exclude.is_match(&i.name) {
                        continue;
                    }
                }

                match i.byte_offset.checked_add_signed(self.add_offset) {
                    Some(new_offset) => i.byte_offset = new_offset,
                    None if strict => err_names.push((id.clone(), i.name.clone())),
                    None => (),
                };
            }
        }

        if !err_names.is_empty() {
            let mut err_msg = String::new();

            for e_name in err_names {
                err_msg.push_str(&format!(
                    "Block: {} Item: {}: byte_offset out of range after modify\n",
                    e_name.0, e_name.1
                ));
            }

            panic!("{err_msg}")
        }

        Ok(())
    }
}
