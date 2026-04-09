use convert_case::{Boundary, Casing};
use serde::{Deserialize, Serialize};

use crate::util::sanitize_ident;

use super::{map_names, NameKind, IR};

/// Sanitize names and paths of all objects, using proper casing and stripping keywords.
///
/// # Changes relative to Sanitize(V1):
/// * Uses PascalCase for enum variants instead of CONSTANT_CASE
/// * Uses PascalCase for interrupts (which are enum variants under the hood) instead of CONSTANT_CASE and later UPPER CASE in the generator.
/// * Uses convert_case crate instead of Inflections to have a fine control over word boundaries
///   such that the operations are more consistent.
#[derive(Debug, Serialize, Deserialize)]
pub struct SanitizeV2 {}

impl SanitizeV2 {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        map_names(ir, |k, p| match k {
            NameKind::Device => *p = sanitize_path(p),
            NameKind::DevicePeripheral => *p = p.to_sanitized_constant_case().to_string(),
            NameKind::DeviceInterrupt => *p = p.to_sanitized_pascal_case().to_string(),
            NameKind::Block => *p = sanitize_path(p),
            NameKind::Fieldset => *p = sanitize_path(p),
            NameKind::Enum => *p = sanitize_path(p),
            NameKind::BlockItem => *p = p.to_sanitized_snake_case().to_string(),
            NameKind::Field => *p = p.to_sanitized_snake_case().to_string(),
            NameKind::EnumVariant => *p = p.to_sanitized_pascal_case().to_string(),
        });

        // After sanitizing names, merge duplicate enum variants with the same name and value
        for (_, enumm) in ir.enums.iter_mut() {
            super::sanitize::merge_duplicate_variants(enumm);
            // rename duplicate enum variants with the same name but different values
            super::sanitize::rename_duplicate_variants(enumm);
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
