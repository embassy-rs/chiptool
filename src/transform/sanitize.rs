use convert_case::{Boundary, Casing};
use serde::{Deserialize, Serialize};

use super::{map_names, NameKind, IR};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SanitizeCase {
    None,
    Snake,
    Constant,
    Pascal,
    Camel,
    Flat,
    UpperFlat,
    /// Path: `snake::Pascal`
    PathSnakePascal,
}

impl SanitizeCase {
    fn apply(self, s: &str) -> String {
        match self {
            SanitizeCase::None => s.to_string(),
            SanitizeCase::Snake => sanitize_with_case(s, convert_case::Case::Snake),
            SanitizeCase::Constant => sanitize_with_case(s, convert_case::Case::Constant),
            SanitizeCase::Pascal => sanitize_with_case(s, convert_case::Case::Pascal),
            SanitizeCase::Camel => sanitize_with_case(s, convert_case::Case::Camel),
            SanitizeCase::Flat => sanitize_with_case(s, convert_case::Case::Flat),
            SanitizeCase::UpperFlat => sanitize_with_case(s, convert_case::Case::UpperFlat),
            SanitizeCase::PathSnakePascal => {
                let v = s.split("::").collect::<Vec<_>>();
                let len = v.len();
                v.into_iter()
                    .enumerate()
                    .map(|(i, seg)| {
                        if i == len - 1 {
                            sanitize_with_case(seg, convert_case::Case::Pascal)
                        } else {
                            sanitize_with_case(seg, convert_case::Case::Snake)
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("::")
            }
        }
    }
}

/// Sanitize names and paths of all objects, using proper casing and stripping keywords.
#[derive(Debug, Serialize, Deserialize)]
pub struct Sanitize {
    #[serde(default = "default_device")]
    pub device: SanitizeCase,
    #[serde(default = "default_device_peripheral")]
    pub device_peripheral: SanitizeCase,
    #[serde(default = "default_device_interrupt")]
    pub device_interrupt: SanitizeCase,
    #[serde(default = "default_block")]
    pub block: SanitizeCase,
    #[serde(default = "default_fieldset")]
    pub fieldset: SanitizeCase,
    #[serde(rename = "enum", default = "default_enum")]
    pub enum_: SanitizeCase,
    #[serde(default = "default_block_item")]
    pub block_item: SanitizeCase,
    #[serde(default = "default_field")]
    pub field: SanitizeCase,
    #[serde(default = "default_enum_variant")]
    pub enum_variant: SanitizeCase,
}

fn default_device() -> SanitizeCase {
    SanitizeCase::PathSnakePascal
}
fn default_device_peripheral() -> SanitizeCase {
    SanitizeCase::Constant
}
fn default_device_interrupt() -> SanitizeCase {
    SanitizeCase::Constant
}
fn default_block() -> SanitizeCase {
    SanitizeCase::PathSnakePascal
}
fn default_fieldset() -> SanitizeCase {
    SanitizeCase::PathSnakePascal
}
fn default_enum() -> SanitizeCase {
    SanitizeCase::PathSnakePascal
}
fn default_block_item() -> SanitizeCase {
    SanitizeCase::Snake
}
fn default_field() -> SanitizeCase {
    SanitizeCase::Snake
}
fn default_enum_variant() -> SanitizeCase {
    SanitizeCase::Pascal
}

impl Sanitize {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        map_names(ir, |k, p| {
            let case = match k {
                NameKind::Device => self.device,
                NameKind::DevicePeripheral => self.device_peripheral,
                NameKind::DeviceInterrupt => self.device_interrupt,
                NameKind::Block => self.block,
                NameKind::Fieldset => self.fieldset,
                NameKind::Enum => self.enum_,
                NameKind::BlockItem => self.block_item,
                NameKind::Field => self.field,
                NameKind::EnumVariant => self.enum_variant,
            };
            *p = case.apply(p);
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
