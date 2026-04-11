use convert_case::{Boundary, Casing};
use serde::{Deserialize, Serialize};

use super::{map_names, NameKind, IR};

/// Sanitize names and paths of all objects, using proper casing and stripping keywords.
#[derive(Debug, Serialize, Deserialize)]
pub struct Sanitize {}

impl Sanitize {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        map_names(ir, |k, p| match k {
            NameKind::Device => *p = sanitize_path(p),
            NameKind::DevicePeripheral => *p = p.to_sanitized_constant_case().to_string(),
            NameKind::DeviceInterrupt => *p = p.to_sanitized_constant_case().to_string(),
            NameKind::Block => *p = sanitize_path(p),
            NameKind::Fieldset => *p = sanitize_path(p),
            NameKind::Enum => *p = sanitize_path(p),
            NameKind::BlockItem => *p = p.to_sanitized_snake_case().to_string(),
            NameKind::Field => *p = p.to_sanitized_snake_case().to_string(),
            NameKind::EnumVariant => *p = p.to_sanitized_pascal_case().to_string(),
        });

        // After sanitizing names, merge duplicate enum variants with the same name and value
        for (_, enumm) in ir.enums.iter_mut() {
            merge_duplicate_variants(enumm);
            // rename duplicate enum variants with the same name but different values
            rename_duplicate_variants(enumm);
        }

        Ok(())
    }
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

fn sanitize_with_case(str: &str, case: convert_case::Case) -> String {
    sanitize_ident(
        str.remove_boundaries(&[Boundary::LowerDigit, Boundary::UpperDigit])
            .to_case(case),
    )
}

pub(crate) fn merge_duplicate_variants(enumm: &mut crate::ir::Enum) {
    use std::collections::BTreeMap;

    let mut seen: BTreeMap<(String, u64), crate::ir::EnumVariant> = BTreeMap::new();
    let mut new_variants = Vec::new();

    for v in enumm.variants.drain(..) {
        let key = (v.name.clone(), v.value);

        if let Some(existing) = seen.get_mut(&key) {
            // Merge description if the existing one doesn't have one
            if existing.description.is_none() && v.description.is_some() {
                existing.description = v.description;
            }
        } else {
            seen.insert(key, v);
        }
    }

    // Collect all unique variants
    for (_, variant) in seen {
        new_variants.push(variant);
    }

    enumm.variants = new_variants;
}

pub(crate) fn rename_duplicate_variants(enumm: &mut crate::ir::Enum) {
    use std::collections::BTreeMap;

    let mut name_counts: BTreeMap<String, usize> = BTreeMap::new();

    for v in &enumm.variants {
        *name_counts.entry(v.name.clone()).or_insert(0) += 1;
    }

    for v in &mut enumm.variants {
        if name_counts.get(&v.name).is_some_and(|&c| c > 1) {
            v.name = format!("{}_{:x}", v.name, v.value);
            // increment new name to catch cascading name collisons
            *name_counts.entry(v.name.clone()).or_insert(0) += 1;
        }
    }
}

trait StringExt {
    fn to_sanitized_pascal_case(&self) -> String;
    fn to_sanitized_constant_case(&self) -> String;
    fn to_sanitized_snake_case(&self) -> String;
}

impl StringExt for str {
    fn to_sanitized_snake_case(&self) -> String {
        sanitize_with_case(self, convert_case::Case::Snake)
    }

    fn to_sanitized_constant_case(&self) -> String {
        sanitize_with_case(self, convert_case::Case::Constant)
    }

    fn to_sanitized_pascal_case(&self) -> String {
        sanitize_with_case(self, convert_case::Case::Pascal)
    }
}

/// List of chars that some vendors use in their peripheral/field names but
/// that are not valid in Rust ident
const INVALID_CHARS: &[char] = &['(', ')', '[', ']', '/', ' ', '-'];

static KEYWORDS: &[&str] = &[
    "abstract", "as", "async", "await", "become", "box", "break", "const", "continue", "crate",
    "do", "dyn", "else", "enum", "extern", "false", "final", "fn", "for", "if", "impl", "in",
    "let", "loop", "macro", "match", "mod", "move", "mut", "override", "priv", "pub", "ref",
    "return", "self", "Self", "static", "struct", "super", "trait", "true", "try", "type",
    "typeof", "unsafe", "unsized", "use", "virtual", "where", "while", "yield",
];

/// Make `s` a valid identifier, making the minimal changes (no case changes)
pub(crate) fn sanitize_ident(s: String) -> String {
    let mut s = s.replace(INVALID_CHARS, "");
    if KEYWORDS.contains(&&*s) {
        s.push('_');
        s
    } else if s.starts_with(char::is_numeric) {
        format!("_{}", s)
    } else {
        s
    }
}
