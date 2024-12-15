use serde::{Deserialize, Serialize};

use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Add {
    ir: IR,
}

impl Add {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        ir.merge(self.ir.clone());
        Ok(())
    }
}
