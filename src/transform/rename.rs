use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use serde::{Deserialize, Serialize};

use super::common::*;
use crate::ir::*;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub enum RenameType {
    #[default]
    All,
    Device,
    Block,
    Fieldset,
    Enum,
}

impl RenameType {
    pub fn fmt<'a>(&self) -> impl Fn(String) -> String + 'a {
        let me = *self;
        move |name: String| match me {
            RenameType::All => unimplemented!(),
            RenameType::Device => format!("device {name}"),
            RenameType::Block => format!("block {name}"),
            RenameType::Fieldset => format!("fieldset {name}"),
            RenameType::Enum => format!("enum {name}"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rename {
    pub from: RegexSet,
    pub to: String,
    #[serde(default)]
    pub r#type: RenameType,
    #[serde(default = "get_true")]
    pub error_on_duplicate: bool,
}

impl Rename {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        let had_duplicates = Rc::new(RefCell::new(false));

        let renamer = |ty: RenameType| {
            let mut state = HashMap::new();
            let had_duplicates = had_duplicates.clone();
            let eod = self.error_on_duplicate;

            move |name: &mut String| {
                if let Some(res) = match_expand(name, &self.from, &self.to) {
                    *had_duplicates.borrow_mut() |=
                        !can_rename(eod, &mut state, &res, name, ty.fmt());
                    *name = res
                }
            }
        };

        match self.r#type {
            RenameType::All => {
                super::map_device_names(ir, renamer(RenameType::Device));
                super::map_block_names(ir, renamer(RenameType::Block));
                super::map_fieldset_names(ir, renamer(RenameType::Fieldset));
                super::map_enum_names(ir, renamer(RenameType::Enum));
            }
            RenameType::Device => super::map_device_names(ir, renamer(RenameType::Device)),
            RenameType::Block => super::map_block_names(ir, renamer(RenameType::Block)),
            RenameType::Fieldset => super::map_fieldset_names(ir, renamer(RenameType::Fieldset)),
            RenameType::Enum => super::map_enum_names(ir, renamer(RenameType::Enum)),
        }

        if *had_duplicates.borrow() && self.error_on_duplicate {
            anyhow::bail!("Failed to rename interrupts");
        }

        Ok(())
    }
}
