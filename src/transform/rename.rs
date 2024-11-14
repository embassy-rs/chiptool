use serde::{Deserialize, Serialize};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Rename {
    pub from: RegexSet,
    pub to: String,
}

impl Rename {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        let renamer = |name: &mut String| {
            if let Some(res) = match_expand(name, &self.from, &self.to) {
                *name = res
            }
        };

        super::map_device_names(ir, renamer);
        super::map_block_names(ir, renamer);
        super::map_fieldset_names(ir, renamer);
        super::map_enum_names(ir, renamer);

        Ok(())
    }
}
