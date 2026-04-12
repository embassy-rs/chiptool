use anyhow::Context;
use log::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fmt::{Display, Formatter};

use super::common::*;
use crate::ir::*;
use crate::transform::merge_fieldsets::{array_compat, fieldset_compat, ArrayError, FieldSetError};

#[derive(Debug, Serialize, Deserialize)]
pub struct MergeBlocks {
    pub from: RegexSet,
    pub to: String,
    pub main: Option<RegexSet>,
    #[serde(default)]
    pub check: CheckLevel,
}

impl MergeBlocks {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        let groups = match_groups(ir.blocks.keys().cloned(), &self.from, &self.to);

        let mut errors = Vec::new();

        for (to, group) in groups {
            info!("Merging blocks, dest: {}", to);

            for id in &group {
                info!("   {}", id);
            }

            errors.extend(merge_blocks(ir, group, to, self.main.as_ref()));
        }

        self.check
            .check("merging blocks", &errors)
            .context("failed to merge blocks")
    }
}

fn merge_blocks(
    ir: &mut IR,
    ids: BTreeSet<String>,
    to: String,
    main: Option<&RegexSet>,
) -> Vec<(String, String, BlockError)> {
    let mut main_id = ids.iter().next().unwrap().clone();
    if let Some(main) = main {
        for id in ids.iter() {
            if main.is_match(id) {
                main_id = id.clone();
                break;
            }
        }
    }
    let b = ir.blocks.get(&main_id).unwrap().clone();

    let mut errors = Vec::new();
    for id in &ids {
        let b2 = ir.blocks.get(id).unwrap();
        errors.extend(
            block_compat(ir, &b, b2)
                .into_iter()
                .map(|v| (main_id.clone(), id.clone(), v)),
        );
    }

    replace_block_ids(ir, &ids, to.clone());
    for id in &ids {
        ir.blocks.remove(id);
    }
    ir.blocks.insert(to, b);

    errors
}

#[derive(Debug)]
pub(crate) enum BlockError {
    Description(Option<String>, Option<String>),
    Extends(Option<String>, Option<String>),
    LhsMissingItem(&'static str, u32, String),
    RhsMissingItem(&'static str, u32, String),
    Item(String, u32, BlockItemError),
}

impl MinCheckLevel for BlockError {
    fn min_check_level(&self) -> CheckLevel {
        match self {
            BlockError::Description(_, _) => CheckLevel::Descriptions,
            BlockError::Item(_, _, e) => e.min_check_level(),
            _ => CheckLevel::Layout,
        }
    }
}

impl core::fmt::Display for BlockError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockError::Description(l, r) => write!(f, "description mismatch: '{l:?}' != '{r:?}'"),
            BlockError::Extends(l, r) => write!(f, "extends mismatch: '{l:?}' != '{r:?}'"),
            BlockError::Item(name, offset, error) => {
                write!(f, "inner item {name} at offset {offset}: {error}")
            }
            BlockError::LhsMissingItem(ty, offset, i) => {
                write!(f, "lhs is missing {ty} '{i}' at offset {offset}")
            }
            BlockError::RhsMissingItem(ty, offset, i) => {
                write!(f, "rhs is missing {ty} '{i}' at offset {offset}")
            }
        }
    }
}

pub(crate) fn block_compat(ir: &IR, main: &Block, other: &Block) -> Vec<BlockError> {
    let mut errors = Vec::new();

    let Block {
        extends,
        description,
        items,
    } = main;

    if description.as_ref() != other.description.as_ref() {
        errors.push(BlockError::Description(
            description.clone(),
            other.description.clone(),
        ));
    }

    if extends.as_ref() != other.extends.as_ref() {
        errors.push(BlockError::Extends(extends.clone(), other.extends.clone()));
    }

    for main in items.iter() {
        let Some(other) = other
            .items
            .iter()
            .find(|v| v.byte_offset == main.byte_offset)
        else {
            let ty = match main.inner {
                BlockItemInner::Block(_) => "block",
                BlockItemInner::Register(_) => "register",
            };

            errors.push(BlockError::RhsMissingItem(
                ty,
                main.byte_offset,
                main.name.clone(),
            ));
            continue;
        };

        errors.extend(
            block_item_compat(ir, main, other)
                .into_iter()
                .map(|v| BlockError::Item(main.name.clone(), main.byte_offset, v)),
        );
    }

    for other in other.items.iter() {
        if main
            .items
            .iter()
            .find(|v| v.byte_offset == other.byte_offset)
            .is_none()
        {
            let ty = match other.inner {
                BlockItemInner::Block(_) => "block",
                BlockItemInner::Register(_) => "register",
            };

            errors.push(BlockError::LhsMissingItem(
                ty,
                other.byte_offset,
                other.name.clone(),
            ));
            continue;
        };
    }

    errors
}

fn fmt_access(access: &Access) -> &str {
    match access {
        Access::ReadWrite => "RW",
        Access::Read => "RO",
        Access::Write => "WO",
    }
}

#[derive(Debug)]
pub(crate) enum BlockItemError {
    Description(Option<String>, Option<String>),
    ArrayXor(bool, bool),
    Array(ArrayError),
    InnerXor(bool),
    RegisterAccessMismatch(Access, Access),
    RegisterSizeMismatch(u32, u32),
    FieldSetXor(bool, bool),
    FieldSet(String, String, FieldSetError),
    InnerBlock(String, String, Box<BlockError>),
    Name(String, String),
}

impl MinCheckLevel for BlockItemError {
    fn min_check_level(&self) -> CheckLevel {
        match self {
            BlockItemError::Description(_, _) => CheckLevel::Descriptions,
            BlockItemError::Name(..) => CheckLevel::Names,
            BlockItemError::Array(_) => CheckLevel::Layout,
            BlockItemError::RegisterAccessMismatch(..) => CheckLevel::Layout,
            BlockItemError::FieldSet(_, _, e) => e.min_check_level(),
            BlockItemError::InnerBlock(_, _, e) => e.min_check_level(),
            _ => CheckLevel::Layout,
        }
    }
}

impl Display for BlockItemError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use BlockItemError::*;

        match self {
            Description(l, r) => write!(f, "description mismatch: '{l:?}' != '{r:?}'"),
            ArrayXor(true, _b) => write!(f, "array mismatch: lhs is array, rhs is not"),
            ArrayXor(false, _b) => write!(f, "array mismatch: lhs is not array, rhs is"),
            Array(e) => write!(f, "array mismatch: {e}"),
            InnerXor(true) => {
                write!(
                    f,
                    "inner type mismatch: lhs is nested block, rhs is register"
                )
            }
            InnerXor(false) => {
                write!(
                    f,
                    "inner type mismatch: lhs is register, rhs is nested block"
                )
            }
            RegisterAccessMismatch(left, right) => {
                let (left, right) = (fmt_access(left), fmt_access(right));
                write!(f, "register access mismatch: {left} != {right}",)
            }
            RegisterSizeMismatch(left, right) => {
                write!(f, "register size mismatch: {left} != {right}")
            }
            FieldSetXor(true, _b) => write!(f, "fieldset mismatch: lhs has fieldset, rhs does not"),
            FieldSetXor(false, _b) => {
                write!(f, "fieldset mismatch: lhs doesn't have fieldset, rhs does")
            }
            FieldSet(l, r, e) => write!(f, "field set {l} and {r} mismatch: {e}"),
            InnerBlock(l, r, e) => write!(f, "inner blocks {l} and {r} mismatch: {e}"),
            Name(l, r) => write!(f, "name mismatch: {l} != {r}"),
        }
    }
}

/// Check if two block items are compatible.
///
/// byte offset is _not_ validated, as this technically does
/// not matter for compatibility. Callers of this function
/// are expected to validate byte offset correctness themselves
/// when necessary.
pub(crate) fn block_item_compat(
    ir: &IR,
    main: &BlockItem,
    other: &BlockItem,
) -> Vec<BlockItemError> {
    let mut errors = Vec::new();

    let BlockItem {
        name,
        description,
        array,
        byte_offset: _,
        inner,
    } = main;

    if name != &other.name {
        errors.push(BlockItemError::Name(name.clone(), other.name.clone()))
    }

    if description.as_ref() != other.description.as_ref() {
        errors.push(BlockItemError::Description(
            description.clone(),
            other.description.clone(),
        ));
    }

    match (array.as_ref(), other.array.as_ref()) {
        (None, None) => {}
        (Some(a1), Some(a2)) => {
            errors.extend(array_compat(a1, a2).into_iter().map(BlockItemError::Array))
        }
        (l, r) => errors.push(BlockItemError::ArrayXor(l.is_some(), r.is_some())),
    }

    use crate::ir::BlockItemInner::*;
    match (&inner, &other.inner) {
        (Block(main_name), Block(other_name)) => {
            let main = ir
                .blocks
                .get(&main_name.block)
                .expect("main inner block exists");
            let other = ir.blocks.get(&other_name.block).expect("other name exists");

            errors.extend(block_compat(ir, main, other).into_iter().map(|v| {
                BlockItemError::InnerBlock(
                    main_name.block.clone(),
                    other_name.block.clone(),
                    Box::new(v),
                )
            }));
        }
        (Register(main), Register(other)) => {
            let crate::ir::Register {
                access,
                bit_size,
                fieldset,
            } = main;

            if access != &other.access {
                errors.push(BlockItemError::RegisterAccessMismatch(
                    access.clone(),
                    other.access.clone(),
                ));
            }

            if *bit_size != other.bit_size {
                errors.push(BlockItemError::RegisterSizeMismatch(
                    *bit_size,
                    other.bit_size,
                ));
            }

            match (fieldset.as_ref(), other.fieldset.as_ref()) {
                (Some(main_name), Some(other_name)) => {
                    let main = ir.fieldsets.get(main_name).expect("main fieldset exists");
                    let other = ir.fieldsets.get(other_name).expect("other fieldset exists");
                    errors.extend(fieldset_compat(main, other).into_iter().map(|v| {
                        BlockItemError::FieldSet(main_name.clone(), other_name.clone(), v)
                    }));
                }
                (None, None) => {}
                (main, other) => {
                    errors.push(BlockItemError::FieldSetXor(main.is_some(), other.is_some()));
                }
            }
        }
        (left, _) => errors.push(BlockItemError::InnerXor(matches!(left, Block(_)))),
    }

    errors
}
