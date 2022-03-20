use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::ir::*;
use crate::util::{ToSanitizedPascalCase, ToSanitizedSnakeCase, ToSanitizedUpperCase};

#[derive(Debug, Serialize, Deserialize)]
pub struct Sanitize {}

impl Sanitize {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        map_names(ir, |k, p| match k {
            NameKind::Device => *p = sanitize_path(p),
            NameKind::DevicePeripheral => *p = p.to_sanitized_upper_case().to_string(),
            NameKind::DeviceInterrupt => *p = p.to_sanitized_upper_case().to_string(),
            NameKind::Block => *p = sanitize_path(p),
            NameKind::Fieldset => *p = sanitize_path(p),
            NameKind::Enum => *p = sanitize_path(p),
            NameKind::BlockItem => *p = p.to_sanitized_snake_case().to_string(),
            NameKind::Field => *p = p.to_sanitized_snake_case().to_string(),
            NameKind::EnumVariant => *p = p.to_sanitized_upper_case().to_string(),
        });
        Ok(())
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

fn rename_opt(s: &mut Option<String>, f: impl Fn(&mut String)) {
    if let Some(s) = s {
        f(s)
    }
}

pub fn map_block_names(ir: &mut IR, f: impl Fn(&mut String)) {
    remap_names(&mut ir.blocks, &f);

    for (_, d) in ir.devices.iter_mut() {
        for p in &mut d.peripherals {
            rename_opt(&mut p.block, &f);
        }
    }

    for (_, b) in ir.blocks.iter_mut() {
        for i in b.items.iter_mut() {
            match &mut i.inner {
                BlockItemInner::Block(p) => f(&mut p.block),
                BlockItemInner::Register(_r) => {}
            }
        }
    }
}

pub fn map_fieldset_names(ir: &mut IR, f: impl Fn(&mut String)) {
    remap_names(&mut ir.fieldsets, &f);

    for (_, b) in ir.blocks.iter_mut() {
        for i in b.items.iter_mut() {
            match &mut i.inner {
                BlockItemInner::Block(_p) => {}
                BlockItemInner::Register(r) => rename_opt(&mut r.fieldset, &f),
            }
        }
    }
}

pub fn map_enum_names(ir: &mut IR, f: impl Fn(&mut String)) {
    remap_names(&mut ir.enums, &f);

    for (_, fs) in ir.fieldsets.iter_mut() {
        for ff in fs.fields.iter_mut() {
            rename_opt(&mut ff.enum_read, &f);
            rename_opt(&mut ff.enum_write, &f);
            rename_opt(&mut ff.enum_readwrite, &f);
        }
    }
}

pub fn map_device_names(ir: &mut IR, f: impl Fn(&mut String)) {
    remap_names(&mut ir.devices, &f);
}

pub fn map_device_interrupt_names(ir: &mut IR, f: impl Fn(&mut String)) {
    for (_, d) in ir.devices.iter_mut() {
        for i in &mut d.interrupts {
            f(&mut i.name);
        }
    }
}

pub fn map_device_peripheral_names(ir: &mut IR, f: impl Fn(&mut String)) {
    for (_, d) in ir.devices.iter_mut() {
        for p in &mut d.peripherals {
            f(&mut p.name);
        }
    }
}

pub fn map_block_item_names(ir: &mut IR, f: impl Fn(&mut String)) {
    for (_, b) in ir.blocks.iter_mut() {
        for i in b.items.iter_mut() {
            f(&mut i.name)
        }
    }
}

pub fn map_field_names(ir: &mut IR, f: impl Fn(&mut String)) {
    for (_, fs) in ir.fieldsets.iter_mut() {
        for ff in fs.fields.iter_mut() {
            f(&mut ff.name)
        }
    }
}

pub fn map_enum_variant_names(ir: &mut IR, f: impl Fn(&mut String)) {
    for (_, e) in ir.enums.iter_mut() {
        for v in e.variants.iter_mut() {
            f(&mut v.name)
        }
    }
}

pub fn map_names(ir: &mut IR, f: impl Fn(NameKind, &mut String)) {
    map_device_names(ir, |s| f(NameKind::Device, s));
    map_device_peripheral_names(ir, |s| f(NameKind::DevicePeripheral, s));
    map_device_interrupt_names(ir, |s| f(NameKind::DeviceInterrupt, s));
    map_block_names(ir, |s| f(NameKind::Block, s));
    map_block_item_names(ir, |s| f(NameKind::BlockItem, s));
    map_fieldset_names(ir, |s| f(NameKind::Fieldset, s));
    map_field_names(ir, |s| f(NameKind::Field, s));
    map_enum_names(ir, |s| f(NameKind::Enum, s));
    map_enum_variant_names(ir, |s| f(NameKind::EnumVariant, s));
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

fn remap_names<T>(x: &mut HashMap<String, T>, f: impl Fn(&mut String)) {
    let mut res = HashMap::new();
    for (mut name, val) in x.drain() {
        f(&mut name);
        assert!(res.insert(name, val).is_none())
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

pub mod delete;
pub mod delete_enums;
pub mod delete_fieldsets;
//pub mod find_duplicate_enums;
//pub mod find_duplicate_fieldsets;
pub mod expand_extends;
pub mod make_block;
pub mod make_field_array;
pub mod make_register_array;
pub mod merge_blocks;
pub mod merge_enums;
pub mod merge_fieldsets;
pub mod rename;
pub mod rename_fields;
pub mod rename_registers;
pub mod sort;
pub mod modify_byte_offset;

#[derive(Debug, Serialize, Deserialize)]
pub enum Transform {
    Sanitize(Sanitize),
    Sort(sort::Sort),
    Delete(delete::Delete),
    DeleteEnums(delete_enums::DeleteEnums),
    DeleteFieldsets(delete_fieldsets::DeleteFieldsets),
    MergeBlocks(merge_blocks::MergeBlocks),
    MergeEnums(merge_enums::MergeEnums),
    MergeFieldsets(merge_fieldsets::MergeFieldsets),
    Rename(rename::Rename),
    RenameFields(rename_fields::RenameFields),
    RenameRegisters(rename_registers::RenameRegisters),
    MakeRegisterArray(make_register_array::MakeRegisterArray),
    MakeFieldArray(make_field_array::MakeFieldArray),
    MakeBlock(make_block::MakeBlock),
    ModifyByteOffset(modify_byte_offset::ModifyByteOffset),
    //FindDuplicateEnums(find_duplicate_enums::FindDuplicateEnums),
    //FindDuplicateFieldsets(find_duplicate_fieldsets::FindDuplicateFieldsets),
}

impl Transform {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        match self {
            Self::Sanitize(t) => t.run(ir),
            Self::Sort(t) => t.run(ir),
            Self::Delete(t) => t.run(ir),
            Self::DeleteEnums(t) => t.run(ir),
            Self::DeleteFieldsets(t) => t.run(ir),
            Self::MergeBlocks(t) => t.run(ir),
            Self::MergeEnums(t) => t.run(ir),
            Self::MergeFieldsets(t) => t.run(ir),
            Self::Rename(t) => t.run(ir),
            Self::RenameFields(t) => t.run(ir),
            Self::RenameRegisters(t) => t.run(ir),
            Self::MakeRegisterArray(t) => t.run(ir),
            Self::MakeFieldArray(t) => t.run(ir),
            Self::MakeBlock(t) => t.run(ir),
            Self::ModifyByteOffset(t) => t.run(ir),
            //Self::FindDuplicateEnums(t) => t.run(ir),
            //Self::FindDuplicateFieldsets(t) => t.run(ir),
        }
    }
}
