use log::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct MergeFieldsets {
    pub from: RegexSet,
    pub to: String,
    pub main: Option<RegexSet>,
    #[serde(default)]
    pub check: CheckLevel,
}

impl MergeFieldsets {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        let groups = match_groups(ir.fieldsets.keys().cloned(), &self.from, &self.to);

        let mut errors = Vec::new();

        for (to, group) in groups {
            info!("Merging fieldsets, dest: {}", to);
            for id in &group {
                info!("   {}", id);
            }

            let mut current = merge_fieldsets(ir, group, to, self.main.as_ref());
            errors.append(&mut current);
        }

        let mut had_breaking_error = false;

        for (main, other, error) in errors {
            let min_check_level = error.min_check_level();

            if self.check >= min_check_level {
                error!("merging {main} and {other}: {error}");
                had_breaking_error = true;
            } else if min_check_level == CheckLevel::Descriptions {
                debug!("merging {main} and {other}: {error}");
            } else {
                warn!("merging {main} and {other}: {error}");
            }
        }

        if had_breaking_error {
            anyhow::bail!("failed to merge field sets");
        }

        Ok(())
    }
}

fn merge_fieldsets(
    ir: &mut IR,
    ids: BTreeSet<String>,
    to: String,
    main: Option<&RegexSet>,
) -> Vec<(String, String, FieldSetError)> {
    let mut main_id = ids.iter().next().unwrap().clone();
    if let Some(main) = main {
        for id in ids.iter() {
            if main.is_match(id) {
                main_id = id.clone();
                break;
            }
        }
    }

    let fs = ir.fieldsets.get(&main_id).unwrap().clone();

    let mut errors = Vec::new();
    for id in &ids {
        let fs2 = ir.fieldsets.get(id).unwrap();
        let compat_check = fieldset_compat(&fs, fs2)
            .into_iter()
            .map(|v| (main_id.clone(), id.clone(), v));
        errors.extend(compat_check);

        ir.fieldsets.remove(id);
    }

    assert!(ir.fieldsets.insert(to.clone(), fs).is_none());
    replace_fieldset_ids(ir, &ids, to);

    errors
}

#[derive(Debug)]
enum MissingFieldReason {
    NoFieldAtOffset(BitOffset),
    BitsizeMismatch(u32, u32),
}

impl std::fmt::Display for MissingFieldReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use MissingFieldReason::*;

        match self {
            NoFieldAtOffset(offset) => {
                write!(f, "no field starting at offset {}", offset.min_offset())
            }
            BitsizeMismatch(l, r) => write!(f, "size mismatch: {l} != {r}"),
        }
    }
}

#[derive(Debug)]
enum FieldSetError {
    Bitsize(u32, u32),
    Extends(Option<String>, Option<String>),
    Field {
        lhs_name: String,
        bit_size: u32,
        bit_offset: BitOffset,
        error: FieldError,
    },
    LhsMissingfield(String, MissingFieldReason),
    RhsMissingField(String, MissingFieldReason),
    Description(Option<String>, Option<String>),
}

impl FieldSetError {
    pub fn min_check_level(&self) -> CheckLevel {
        match self {
            FieldSetError::Description(_, _) => CheckLevel::Descriptions,
            FieldSetError::Field { error, .. } => error.min_check_level(),
            _ => CheckLevel::Layout,
        }
    }
}

impl std::fmt::Display for FieldSetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldSetError::Bitsize(s1, s2) => {
                write!(f, "size mismatch: {} != {}", s1, s2)
            }
            FieldSetError::Extends(e1, e2) => write!(f, "extends mismatch: {:?} != {:?}", e1, e2),
            FieldSetError::Field {
                lhs_name,
                bit_size,
                bit_offset,
                error,
            } => write!(
                f,
                "{lhs_name} at offset {} with size {bit_size}: {error}",
                bit_offset.min_offset()
            ),
            FieldSetError::LhsMissingfield(field, reason) => {
                write!(f, "lhs is missing field '{field}': {reason}")
            }
            FieldSetError::RhsMissingField(field, reason) => {
                write!(f, "rhs is missing field '{field}': {reason}")
            }
            FieldSetError::Description(l, r) => {
                write!(f, "description mismatch: '{l:?}' != '{r:?}'")
            }
        }
    }
}

/// Check if fieldset `other` is compatible with `main` to the level of `level`.
fn fieldset_compat(main: &FieldSet, other: &FieldSet) -> Vec<FieldSetError> {
    let mut errors = Vec::new();

    let FieldSet {
        extends,
        description,
        bit_size,
        fields,
    } = main;

    if extends.as_ref() != other.extends.as_ref() {
        errors.push(FieldSetError::Extends(
            extends.clone(),
            other.extends.clone(),
        ));
    }

    if *bit_size != other.bit_size {
        errors.push(FieldSetError::Bitsize(*bit_size, other.bit_size));
    }

    if description.as_ref() != other.description.as_ref() {
        errors.push(FieldSetError::Description(
            description.clone(),
            other.description.clone(),
        ));
    }

    for main in fields {
        let Some(other) = other
            .fields
            .iter()
            .find(|f| f.bit_offset == main.bit_offset)
        else {
            errors.push(FieldSetError::RhsMissingField(
                main.name.clone(),
                MissingFieldReason::NoFieldAtOffset(main.bit_offset.clone()),
            ));
            continue;
        };

        if main.bit_size != other.bit_size {
            errors.push(FieldSetError::RhsMissingField(
                main.name.clone(),
                MissingFieldReason::BitsizeMismatch(main.bit_size, other.bit_size),
            ));
            continue;
        };

        errors.extend(
            field_compat(&main, &other)
                .into_iter()
                .map(|error| FieldSetError::Field {
                    lhs_name: main.name.clone(),
                    bit_offset: main.bit_offset.clone(),
                    bit_size: main.bit_size,
                    error,
                }),
        );
    }

    for other in other.fields.iter() {
        let Some(main) = main
            .fields
            .iter()
            .find(|f| f.bit_offset == other.bit_offset)
        else {
            errors.push(FieldSetError::LhsMissingfield(
                other.name.clone(),
                MissingFieldReason::NoFieldAtOffset(other.bit_offset.clone()),
            ));
            continue;
        };

        if main.bit_size != other.bit_size {
            errors.push(FieldSetError::LhsMissingfield(
                other.name.clone(),
                MissingFieldReason::BitsizeMismatch(main.bit_size, other.bit_size),
            ));
        }
    }

    errors
}

#[derive(Debug)]
enum FieldError {
    Array(ArrayError),
    ArrayXor(bool, bool),
    Description(Option<String>, Option<String>),
    Enum(Option<String>, Option<String>),
    Name(String, String),
}

impl FieldError {
    pub fn min_check_level(&self) -> CheckLevel {
        match self {
            FieldError::Array(_) => CheckLevel::Layout,
            FieldError::Name(_, _) => CheckLevel::Names,
            FieldError::Description(_, _) => CheckLevel::Descriptions,
            _ => CheckLevel::Layout,
        }
    }
}

impl core::fmt::Display for FieldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldError::ArrayXor(a1, _b) => {
                if *a1 {
                    write!(f, "array types mismatch: lhs is an array, rhs is not")
                } else {
                    write!(f, "array types mismatch: lhs is not an array, rhs is")
                }
            }
            FieldError::Array(e) => write!(f, "{}", e),
            FieldError::Description(l, r) => write!(f, "description mismatch: '{l:?}' != '{r:?}'"),
            FieldError::Enum(l, r) => write!(f, "enum mismatch: '{l:?}' != '{r:?}'"),
            FieldError::Name(l, r) => write!(f, "name mismatch: '{r}' != '{l}'"),
        }
    }
}

fn field_compat(main: &Field, other: &Field) -> Vec<FieldError> {
    let mut errors = Vec::new();

    let Field {
        name,
        description,
        bit_offset,
        bit_size,
        array,
        enumm,
    } = main;

    assert_eq!(bit_offset, &other.bit_offset);
    assert_eq!(bit_size, &other.bit_size);

    match (array.as_ref(), other.array.as_ref()) {
        (Some(a1), Some(a2)) => {
            errors.extend(array_compat(a1, a2).into_iter().map(FieldError::Array))
        }
        (None, None) => {}
        (a1, a2) => errors.push(FieldError::ArrayXor(a1.is_some(), a2.is_some())),
    }

    if description.as_ref() != other.description.as_ref() {
        errors.push(FieldError::Description(
            description.clone(),
            other.description.clone(),
        ));
    }

    if name != &other.name {
        errors.push(FieldError::Name(name.clone(), other.name.clone()))
    }

    if enumm.as_ref() != other.enumm.as_ref() {
        errors.push(FieldError::Enum(enumm.clone(), other.enumm.clone()));
    }

    errors
}

#[derive(Debug)]
enum ArrayError {
    Xor(Array, Array),
    Len(usize, usize),
    Stride(u32, u32),
}

impl core::fmt::Display for ArrayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArrayError::Xor(Array::Regular(_), Array::Cursed(_)) => {
                write!(f, "array type mismatch: lhs is regular, rhs is cursed")
            }
            ArrayError::Xor(Array::Cursed(_), Array::Regular(_)) => {
                write!(f, "array type mismatch: lhs is cursed, rhs is regular")
            }
            ArrayError::Xor(..) => unreachable!(),
            ArrayError::Len(l1, l2) => write!(f, "array len mismatch: {} != {}", l1, l2),
            ArrayError::Stride(s1, s2) => write!(f, "stride mismatch: {} != {}", s1, s2),
        }
    }
}

fn array_compat(main: &Array, other: &Array) -> Vec<ArrayError> {
    let mut errors = Vec::new();

    if main.len() != other.len() {
        errors.push(ArrayError::Len(main.len(), other.len()));
    }

    match (main, other) {
        (Array::Regular(a1), Array::Regular(a2)) => {
            if a1.stride != a2.stride {
                errors.push(ArrayError::Stride(a1.stride, a2.stride));
            }
        }
        (Array::Cursed(_), Array::Cursed(_)) => {}
        (a1, a2) => errors.push(ArrayError::Xor(a1.clone(), a2.clone())),
    }

    errors
}
