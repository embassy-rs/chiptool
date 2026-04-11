use std::collections::hash_map::Entry;
use std::collections::HashMap;

use log::Level;
use serde::{Deserialize, Serialize};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct RenameRegisters {
    pub block: RegexSet,
    pub from: RegexSet,
    pub to: String,
    #[serde(default = "get_true")]
    pub error_on_duplicate: bool,
}

fn get_true() -> bool {
    true
}

impl RenameRegisters {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        // Map blocks to new names that have been used, and the first
        // field that was renamed to that new name.
        let mut renames: HashMap<_, HashMap<String, Option<String>>> = HashMap::new();
        let mut had_duplicate = false;

        for id in match_all(ir.blocks.keys().cloned(), &self.block) {
            let b = ir.blocks.get_mut(&id).unwrap();
            let renames = renames.entry(id.clone()).or_default();

            for i in b.items.iter_mut() {
                if let Some(name) = match_expand(&i.name, &self.from, &self.to) {
                    match renames.entry(name.clone()) {
                        // Not duplicate
                        Entry::Vacant(e) => {
                            e.insert(Some(i.name.clone()));
                            i.name = name.clone();
                        }
                        Entry::Occupied(mut e) => {
                            let level = self
                                .error_on_duplicate
                                .then_some(Level::Error)
                                .unwrap_or(Level::Warn);

                            let log = |reg: &str| {
                                log::log!(level, "Renaming register {reg} in block {id} failed: reused new name {name}");
                            };

                            // Log the name of the first field for good measure, and get rid of it
                            if let Some(prev) = e.get_mut().take() {
                                log(&prev)
                            }

                            log(&i.name);
                            had_duplicate = true;
                        }
                    }
                }
            }
        }

        if had_duplicate && self.error_on_duplicate {
            anyhow::bail!("Duplicate use of new names");
        }

        Ok(())
    }
}
