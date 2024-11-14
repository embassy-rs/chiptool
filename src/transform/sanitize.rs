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
