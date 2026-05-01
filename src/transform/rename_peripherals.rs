use serde::{Deserialize, Serialize};

use super::common::*;
use crate::ir::*;
use crate::transform::rename::{Rename, RenameType};

#[derive(Debug, Serialize, Deserialize)]
pub struct RenamePeripherals {
    pub from: RegexSet,
    pub to: String,
    #[serde(default = "get_true")]
    pub error_on_duplicate: bool,
}

impl RenamePeripherals {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        Rename {
            from: self.from.clone(),
            to: self.to.clone(),
            r#type: RenameType::Peripheral,
            error_on_duplicate: self.error_on_duplicate,
        }
        .run(ir)
    }
}
