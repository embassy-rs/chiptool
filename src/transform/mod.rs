use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::ir::*;
use crate::util::StringExt;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(PartialEq, Eq, Hash)]
struct NameCollisionError {
    kind: NameKind,
    old: String,
    new: String,
}

impl NameCollisionError {
    fn new(kind: NameKind, old: String, new: String) -> Self {
        Self { kind, old, new }
    }
}

impl std::fmt::Debug for NameCollisionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Err: on rename {:?} \"{}\", new name \"{}\" already exist",
            self.kind, self.old, self.new
        )
    }
}

struct NameCollisionErrors(HashSet<NameCollisionError>);

impl std::fmt::Debug for NameCollisionErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.0.is_empty() {
            writeln!(f)?
        }

        for err in self.0.iter() {
            writeln!(f, "{:?}", err)?
        }
        Ok(())
    }
}

fn rename_opt(s: &mut Option<String>, f: impl Fn(&mut String)) {
    if let Some(s) = s {
        f(s)
    }
}

pub fn map_block_names(ir: &mut IR, f: impl Fn(&mut String)) {
    remap_names(NameKind::Block, &mut ir.blocks, &f).unwrap();

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
    remap_names(NameKind::Fieldset, &mut ir.fieldsets, &f).unwrap();

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
    remap_names(NameKind::Enum, &mut ir.enums, &f).unwrap();

    for (_, fs) in ir.fieldsets.iter_mut() {
        for ff in fs.fields.iter_mut() {
            rename_opt(&mut ff.enumm, &f);
        }
    }
}

pub fn map_device_names(ir: &mut IR, f: impl Fn(&mut String)) {
    remap_names(NameKind::Device, &mut ir.devices, &f).unwrap();
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

fn remap_names<T>(
    kind: NameKind,
    x: &mut HashMap<String, T>,
    f: impl Fn(&mut String),
) -> Result<(), NameCollisionErrors> {
    let mut res = HashMap::new();
    let mut errs = HashSet::new();

    for (mut name, val) in x.drain() {
        let orginal_name = name.clone();
        f(&mut name);
        if res.insert(name.clone(), val).is_some() {
            errs.insert(NameCollisionError::new(kind, orginal_name, name));
        }
    }

    if !errs.is_empty() {
        return Err(NameCollisionErrors(errs));
    }

    *x = res;
    Ok(())
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
pub mod delete_enums_used_in;
pub mod delete_fieldsets;
pub mod delete_useless_enums;
//pub mod find_duplicate_enums;
//pub mod find_duplicate_fieldsets;
pub mod delete_registers;
pub mod expand_extends;
pub mod fix_register_bit_sizes;
pub mod make_block;
pub mod make_field_array;
pub mod make_register_array;
pub mod merge_blocks;
pub mod merge_enums;
pub mod merge_fieldsets;
pub mod modify_byte_offset;
pub mod rename;
pub mod rename_enum_variants;
pub mod rename_fields;
pub mod rename_registers;
pub mod sort;

#[derive(Debug, Serialize, Deserialize)]
pub enum Transform {
    Sanitize(Sanitize),
    Sort(sort::Sort),
    Delete(delete::Delete),
    DeleteEnums(delete_enums::DeleteEnums),
    DeleteEnumsUsedIn(delete_enums_used_in::DeleteEnumsUsedIn),
    DeleteUselessEnums(delete_useless_enums::DeleteUselessEnums),
    DeleteFieldsets(delete_fieldsets::DeleteFieldsets),
    DeleteRegisters(delete_registers::DeleteRegisters),
    MergeBlocks(merge_blocks::MergeBlocks),
    MergeEnums(merge_enums::MergeEnums),
    MergeFieldsets(merge_fieldsets::MergeFieldsets),
    Rename(rename::Rename),
    RenameFields(rename_fields::RenameFields),
    RenameRegisters(rename_registers::RenameRegisters),
    RenameEnumVariants(rename_enum_variants::RenameEnumVariants),
    MakeRegisterArray(make_register_array::MakeRegisterArray),
    MakeFieldArray(make_field_array::MakeFieldArray),
    MakeBlock(make_block::MakeBlock),
    ModifyByteOffset(modify_byte_offset::ModifyByteOffset),
    FixRegisterBitSizes(fix_register_bit_sizes::FixRegisterBitSizes),
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
            Self::DeleteEnumsUsedIn(t) => t.run(ir),
            Self::DeleteUselessEnums(t) => t.run(ir),
            Self::DeleteFieldsets(t) => t.run(ir),
            Self::DeleteRegisters(t) => t.run(ir),
            Self::MergeBlocks(t) => t.run(ir),
            Self::MergeEnums(t) => t.run(ir),
            Self::MergeFieldsets(t) => t.run(ir),
            Self::Rename(t) => t.run(ir),
            Self::RenameFields(t) => t.run(ir),
            Self::RenameRegisters(t) => t.run(ir),
            Self::RenameEnumVariants(t) => t.run(ir),
            Self::MakeRegisterArray(t) => t.run(ir),
            Self::MakeFieldArray(t) => t.run(ir),
            Self::MakeBlock(t) => t.run(ir),
            Self::ModifyByteOffset(t) => t.run(ir),
            Self::FixRegisterBitSizes(t) => t.run(ir),
            //Self::FindDuplicateEnums(t) => t.run(ir),
            //Self::FindDuplicateFieldsets(t) => t.run(ir),
        }
    }
}
