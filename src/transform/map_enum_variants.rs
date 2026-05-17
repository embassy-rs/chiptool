//! Simple transform for enum variant names
//!
//! This is useful when an SVD contains enum variants with purely numeric identifiers.
//! It is not meant to be used to apply inflection, that's what the `Sanitize` transform is for.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct MapEnumVariants {
    #[serde(rename = "enum")]
    pub enumm: RegexSet,
    #[serde(default)]
    pub variants: BTreeMap<String, String>,
    #[serde(default)]
    pub descriptions: BTreeMap<String, String>,
}

impl MapEnumVariants {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        for id in match_all(ir.enums.keys().cloned(), &self.enumm) {
            let e = ir.enums.get_mut(&id).unwrap();
            for variant in e.variants.iter_mut() {
                if let Some(new_description) = self.descriptions.get(&variant.name) {
                    variant.description = Some(new_description.clone());
                }
                if let Some(new_name) = self.variants.get(&variant.name) {
                    variant.name = new_name.clone();
                }
            }
        }
        Ok(())
    }
}
