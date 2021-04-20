use log::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::ir::*;
use crate::util::{ToSanitizedPascalCase, ToSanitizedSnakeCase, ToSanitizedUpperCase};

#[derive(Debug, Serialize, Deserialize)]
pub struct Sanitize {}

impl Sanitize {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        map_names(ir, |p, k| match k {
            NameKind::Device => sanitize_path(p),
            NameKind::DevicePeripheral => p.to_sanitized_upper_case().to_string(),
            NameKind::DeviceInterrupt => p.to_sanitized_upper_case().to_string(),
            NameKind::Block => sanitize_path(p),
            NameKind::Fieldset => sanitize_path(p),
            NameKind::Enum => sanitize_path(p),
            NameKind::BlockItem => p.to_sanitized_snake_case().to_string(),
            NameKind::Field => p.to_sanitized_snake_case().to_string(),
            NameKind::EnumVariant => p.to_sanitized_upper_case().to_string(),
        })
    }
}

pub enum NameKind {
    Device,
    DevicePeripheral,
    DeviceInterrupt,
    Block,
    BlockItem,
    Fieldset,
    Field,
    Enum,
    EnumVariant,
}

pub fn map_names(ir: &mut IR, mut ff: impl FnMut(&str, NameKind) -> String) -> anyhow::Result<()> {
    remap_names(&mut ir.devices, |p| ff(p, NameKind::Device));
    remap_names(&mut ir.blocks, |p| ff(p, NameKind::Block));
    remap_names(&mut ir.fieldsets, |p| ff(p, NameKind::Fieldset));
    remap_names(&mut ir.enums, |p| ff(p, NameKind::Enum));

    for (_, d) in ir.devices.iter_mut() {
        for p in &mut d.peripherals {
            p.name = ff(&p.name, NameKind::DevicePeripheral);
            p.block = p.block.as_ref().map(|p| ff(p, NameKind::Block));
        }
        for i in &mut d.interrupts {
            i.name = ff(&i.name, NameKind::DeviceInterrupt);
        }
    }

    for (_, b) in ir.blocks.iter_mut() {
        for i in b.items.iter_mut() {
            i.name = ff(&i.name, NameKind::BlockItem);
            match &mut i.inner {
                BlockItemInner::Block(p) => {
                    i.inner = BlockItemInner::Block(BlockItemBlock {
                        block: ff(&p.block, NameKind::Block),
                    })
                }
                BlockItemInner::Register(r) => {
                    r.fieldset = r.fieldset.as_ref().map(|p| ff(p, NameKind::Fieldset));
                }
            }
        }
    }

    for (_, fs) in ir.fieldsets.iter_mut() {
        for f in fs.fields.iter_mut() {
            f.name = ff(&f.name, NameKind::Field);
            f.enum_read = f.enum_read.as_ref().map(|p| ff(p, NameKind::Enum));
            f.enum_write = f.enum_write.as_ref().map(|p| ff(p, NameKind::Enum));
            f.enum_readwrite = f.enum_readwrite.as_ref().map(|p| ff(p, NameKind::Enum));
        }
    }

    for (_, e) in ir.enums.iter_mut() {
        for v in e.variants.iter_mut() {
            v.name = ff(&v.name, NameKind::EnumVariant);
        }
    }

    Ok(())
}

pub fn map_descriptions(ir: &mut IR, mut ff: impl FnMut(&str) -> String) -> anyhow::Result<()> {
    let mut mapit = |d: &mut Option<String>| {
        *d = d.as_ref().map(|p| ff(p));
    };

    for (_, b) in ir.blocks.iter_mut() {
        mapit(&mut b.description);
        for i in b.items.iter_mut() {
            mapit(&mut i.description);
        }
    }

    for (_, fs) in ir.fieldsets.iter_mut() {
        mapit(&mut fs.description);
        for f in fs.fields.iter_mut() {
            mapit(&mut f.description);
        }
    }

    for (_, e) in ir.enums.iter_mut() {
        mapit(&mut e.description);
        for v in e.variants.iter_mut() {
            mapit(&mut v.description);
        }
    }

    Ok(())
}

fn remap_names<T>(x: &mut HashMap<String, T>, mut f: impl FnMut(&str) -> String) {
    let mut res = HashMap::new();
    for (path, val) in x.drain() {
        assert!(res.insert(f(&path), val).is_none())
    }
    *x = res
}

fn sanitize_path(p: &str) -> String {
    let v = p.split("::").collect::<Vec<_>>();
    let len = v.len();
    v.into_iter()
        .enumerate()
        .map(|(i, s)| {
            if i == len - 1 {
                s.to_sanitized_pascal_case()
            } else {
                s.to_sanitized_snake_case()
            }
        })
        .collect::<Vec<_>>()
        .join("::")
}

mod common;

mod delete;
mod delete_enums;
mod delete_fieldsets;
//mod find_duplicate_enums;
//mod find_duplicate_fieldsets;
//mod load_svd;
//mod make_block;
mod make_field_array;
mod make_register_array;
//mod merge_blocks;
mod merge_enums;
mod merge_fieldsets;
//mod rename;
//mod rename_fields;
pub(crate) mod expand_extends;
pub(crate) mod sort;

#[derive(Debug, Serialize, Deserialize)]
pub enum Transform {
    //LoadSvd(load_svd::LoadSvd),
    Sanitize(Sanitize),
    Sort(sort::Sort),
    Delete(delete::Delete),
    DeleteEnums(delete_enums::DeleteEnums),
    DeleteFieldsets(delete_fieldsets::DeleteFieldsets),
    //MergeBlocks(merge_blocks::MergeBlocks),
    MergeEnums(merge_enums::MergeEnums),
    MergeFieldsets(merge_fieldsets::MergeFieldsets),
    //Rename(rename::Rename),
    //RenameFields(rename_fields::RenameFields),
    MakeRegisterArray(make_register_array::MakeRegisterArray),
    MakeFieldArray(make_field_array::MakeFieldArray),
    //MakeBlock(make_block::MakeBlock),
    //FindDuplicateEnums(find_duplicate_enums::FindDuplicateEnums),
    //FindDuplicateFieldsets(find_duplicate_fieldsets::FindDuplicateFieldsets),
}

impl Transform {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        match self {
            Self::Sanitize(t) => t.run(ir),
            Self::Sort(t) => t.run(ir),
            //Self::LoadSvd(t) => t.run(ir),
            Self::Delete(t) => t.run(ir),
            Self::DeleteEnums(t) => t.run(ir),
            Self::DeleteFieldsets(t) => t.run(ir),
            //Self::MergeBlocks(t) => t.run(ir),
            Self::MergeEnums(t) => t.run(ir),
            Self::MergeFieldsets(t) => t.run(ir),
            //Self::Rename(t) => t.run(ir),
            //Self::RenameFields(t) => t.run(ir),
            Self::MakeRegisterArray(t) => t.run(ir),
            Self::MakeFieldArray(t) => t.run(ir),
            //Self::MakeBlock(t) => t.run(ir),
            //Self::FindDuplicateEnums(t) => t.run(ir),
            //Self::FindDuplicateFieldsets(t) => t.run(ir),
        }
    }
}
