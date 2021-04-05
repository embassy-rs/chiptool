use log::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Sort {}

impl Sort {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        for (_, z) in &mut ir.blocks {
            z.items.sort_by_key(|i| (i.byte_offset, i.name.clone()))
        }
        for (_, z) in &mut ir.fieldsets {
            z.fields.sort_by_key(|i| (i.bit_offset, i.name.clone()))
        }
        for (_, z) in &mut ir.enums {
            z.variants.sort_by_key(|i| (i.value, i.name.clone()))
        }

        Ok(())
    }
}
