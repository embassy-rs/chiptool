use serde::{Deserialize, Serialize};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct ModifyByteOffset {
    pub blocks: String,
    pub exclude_items: Option<String>,
    pub add_offset: i32,
}

impl ModifyByteOffset {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        let path_re = make_regex(&self.blocks)?;
        let ex_re = if let Some(exclude_items) = &self.exclude_items {
            make_regex(exclude_items)?
        } else {
            make_regex("")?
        };

        let mut err_names = Vec::new();

        for id in match_all(ir.blocks.keys().cloned(), &path_re) {
            let b = ir.blocks.get_mut(&id).unwrap();
            for i in &mut b.items {
                if ex_re.is_match(&i.name) {
                    continue;
                }

                match i.byte_offset.checked_add_signed(self.add_offset) {
                    Some(new_offset) => i.byte_offset = new_offset,
                    None => err_names.push((id.clone(), i.name.clone())),
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
