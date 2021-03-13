use log::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::ir::*;
use crate::util::{ToSanitizedPascalCase, ToSanitizedSnakeCase, ToSanitizedUpperCase};

#[derive(Debug, Serialize, Deserialize)]
pub struct Sanitize {}

impl Sanitize {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        for (_, d) in ir.devices.iter_mut() {
            sanitize_path(&mut d.path);
        }

        for (_, b) in ir.blocks.iter_mut() {
            sanitize_path(&mut b.path);
            for i in b.items.iter_mut() {
                i.name = i.name.to_sanitized_snake_case().to_string();
            }
        }

        for (_, fs) in ir.fieldsets.iter_mut() {
            sanitize_path(&mut fs.path);
            for f in fs.fields.iter_mut() {
                f.name = f.name.to_sanitized_snake_case().to_string();
            }
        }

        for (_, e) in ir.enums.iter_mut() {
            sanitize_path(&mut e.path);
            for v in e.variants.iter_mut() {
                v.name = v.name.to_sanitized_upper_case().to_string();
            }
        }

        Ok(())
    }
}

fn sanitize_path(p: &mut Path) {
    for s in &mut p.modules {
        *s = s.to_sanitized_snake_case().to_string();
    }
    p.name = p.name.to_sanitized_pascal_case().to_string();
}

mod common;

mod delete_enums;
mod delete_fieldsets;
mod find_duplicate_enums;
mod find_duplicate_fieldsets;
mod load_svd;
mod make_block;
mod make_field_array;
mod make_register_array;
mod merge_blocks;
mod merge_enums;
mod merge_fieldsets;
mod rename;
mod rename_fields;

#[derive(Debug, Serialize, Deserialize)]
pub enum Transform {
    LoadSvd(load_svd::LoadSvd),
    Sanitize(Sanitize),
    DeleteEnums(delete_enums::DeleteEnums),
    DeleteFieldsets(delete_fieldsets::DeleteFieldsets),
    MergeBlocks(merge_blocks::MergeBlocks),
    MergeEnums(merge_enums::MergeEnums),
    MergeFieldsets(merge_fieldsets::MergeFieldsets),
    Rename(rename::Rename),
    RenameFields(rename_fields::RenameFields),
    MakeRegisterArray(make_register_array::MakeRegisterArray),
    MakeFieldArray(make_field_array::MakeFieldArray),
    MakeBlock(make_block::MakeBlock),
    FindDuplicateEnums(find_duplicate_enums::FindDuplicateEnums),
    FindDuplicateFieldsets(find_duplicate_fieldsets::FindDuplicateFieldsets),
}

impl Transform {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        match self {
            Self::Sanitize(t) => t.run(ir),
            Self::LoadSvd(t) => t.run(ir),
            Self::DeleteEnums(t) => t.run(ir),
            Self::DeleteFieldsets(t) => t.run(ir),
            Self::MergeBlocks(t) => t.run(ir),
            Self::MergeEnums(t) => t.run(ir),
            Self::MergeFieldsets(t) => t.run(ir),
            Self::Rename(t) => t.run(ir),
            Self::RenameFields(t) => t.run(ir),
            Self::MakeRegisterArray(t) => t.run(ir),
            Self::MakeFieldArray(t) => t.run(ir),
            Self::MakeBlock(t) => t.run(ir),
            Self::FindDuplicateEnums(t) => t.run(ir),
            Self::FindDuplicateFieldsets(t) => t.run(ir),
        }
    }
}
