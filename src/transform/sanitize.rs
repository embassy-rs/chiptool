use serde::{Deserialize, Serialize};

use crate::util::StringExt;

use super::{map_names, NameKind, IR};

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
            NameKind::EnumVariant => *p = p.to_sanitized_constant_case().to_string(),
        });

        // After sanitizing names, merge duplicate enum variants with the same name and value
        for (_, enumm) in ir.enums.iter_mut() {
            merge_duplicate_variants(enumm);
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

fn merge_duplicate_variants(enumm: &mut crate::ir::Enum) {
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
