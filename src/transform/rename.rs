use serde::{Deserialize, Serialize};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Rename {
    pub from: String,
    pub to: String,
}

impl Rename {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        let re = make_regex(&self.from)?;
        for (_, x) in ir.devices.iter_mut() {
            if let Some(res) = path_match_expand(&x.path, &re, &self.to) {
                x.path = res
            }
        }
        for (_, x) in ir.blocks.iter_mut() {
            if let Some(res) = path_match_expand(&x.path, &re, &self.to) {
                x.path = res
            }
        }
        for (_, x) in ir.fieldsets.iter_mut() {
            if let Some(res) = path_match_expand(&x.path, &re, &self.to) {
                x.path = res
            }
        }
        for (_, x) in ir.enums.iter_mut() {
            if let Some(res) = path_match_expand(&x.path, &re, &self.to) {
                x.path = res
            }
        }

        Ok(())
    }
}
