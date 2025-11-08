use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashSet};
use std::mem::take;

use crate::ir::*;

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
    x: &mut BTreeMap<String, T>,
    f: impl Fn(&mut String),
) -> Result<(), NameCollisionErrors> {
    let mut res = BTreeMap::new();
    let mut errs = HashSet::new();

    for (mut name, val) in take(x) {
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

mod common;

macro_rules! transforms {
    ($($mod:ident::$struct:ident,)*) => {
        $( pub mod $mod; )*

        #[derive(Debug, Serialize, Deserialize)]
        pub enum Transform {
            $( $struct($mod::$struct), )*
        }

        impl Transform {
            pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
                match self {
                    $( Self::$struct(t) => t.run(ir), )*
                }
            }
        }
    };
}

transforms!(
    sanitize::Sanitize,
    sort::Sort,
    add::Add,
    add_enum_variants::AddEnumVariants,
    add_fields::AddFields,
    add_registers::AddRegisters,
    add_interrupts::AddInterrupts,
    add_peripherals::AddPeripherals,
    delete::Delete,
    delete_enum_variants::DeleteEnumVariants,
    delete_enums::DeleteEnums,
    delete_enums_with_variants::DeleteEnumsWithVariants,
    delete_enums_used_in::DeleteEnumsUsedIn,
    delete_useless_enums::DeleteUselessEnums,
    delete_fields::DeleteFields,
    delete_fieldsets::DeleteFieldsets,
    delete_peripherals::DeletePeripherals,
    delete_registers::DeleteRegisters,
    expand_extends::ExpandExtends,
    merge_blocks::MergeBlocks,
    merge_enums::MergeEnums,
    merge_fieldsets::MergeFieldsets,
    rename::Rename,
    rename_fields::RenameFields,
    rename_registers::RenameRegisters,
    rename_enum_variants::RenameEnumVariants,
    resize_enums::ResizeEnums,
    make_register_array::MakeRegisterArray,
    make_field_array::MakeFieldArray,
    make_block::MakeBlock,
    modify_byte_offset::ModifyByteOffset,
    modify_fields_enum::ModifyFieldsEnum,
    modify_registers::ModifyRegisters,
    fix_register_bit_sizes::FixRegisterBitSizes,
    rename_interrupts::RenameInterrupts,
    rename_peripherals::RenamePeripherals,
);
