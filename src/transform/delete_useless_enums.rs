use log::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

use super::delete_enums::remove_enum_ids;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteUselessEnums {
    #[serde(default)]
    pub soft: bool,
}

impl DeleteUselessEnums {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        let mut ids: BTreeSet<String> = BTreeSet::new();

        for (id, e) in &ir.enums {
            if is_useless(e) {
                info!("deleting enum {}", id);
                ids.insert(id.clone());
            }
        }

        remove_enum_ids(ir, &ids);

        if !self.soft {
            for id in ids {
                ir.enums.remove(&id);
            }
        }

        Ok(())
    }
}

const USELESS_ZERO_NAMES: &[&str] = &[
    "dis",
    "disable",
    "disabled",
    "off",
    "false",
    "no",
    "busy",
    "pending",
    "discon",
    "disconnect",
    "disconnected",
    "not_detected",
    "invalid",
    "no_effect",
];
const USELESS_ONE_NAMES: &[&str] = &[
    "en",
    "enable",
    "enabled",
    "on",
    "true",
    "yes",
    "ready",
    "available",
    "connect",
    "connected",
    "detected",
    "valid",
    "set",
    "clr",
];

const NOT_NAMES: &[&str] = &["not", "no", "un", "de", "in"];

fn is_useless(e: &Enum) -> bool {
    match e.bit_size {
        0 => true,
        1 => match e.variants.len() {
            0 => true,
            1 => true,
            2 => {
                let zero = e.variants.iter().find(|v| v.value == 0).unwrap();
                let zero_name = zero.name.to_ascii_lowercase();
                let one = e.variants.iter().find(|v| v.value == 1).unwrap();
                let one_name = one.name.to_ascii_lowercase();

                let obvious = USELESS_ZERO_NAMES.iter().any(|s| s == &zero_name)
                    && USELESS_ONE_NAMES.iter().any(|s| s == &one_name);
                let not = NOT_NAMES.iter().any(|not| {
                    zero_name == format!("{not}{one_name}")
                        || zero_name == format!("{not}_{one_name}")
                });

                obvious || not
            }
            _ => unreachable!(),
        },
        _ => false,
    }
}
