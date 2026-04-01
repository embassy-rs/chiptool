use anyhow::{bail, Result};
use clap::Parser;
use std::collections::BTreeSet;
use std::fs;

use crate::ir::IR;

/// Reformat a YAML
#[derive(Parser)]
pub struct Fmt {
    /// Peripheral file path
    pub files: Vec<String>,
    /// Error if incorrectly formatted, instead of fixing.
    #[clap(long)]
    pub check: bool,
    /// Remove unused enums
    #[clap(long)]
    pub remove_unused: bool,
}

pub fn fmt(args: Fmt) -> Result<()> {
    for file in args.files {
        let got_data = fs::read(&file)?;
        let mut ir: IR = serde_yaml::from_slice(&got_data)?;

        if args.remove_unused {
            let mut used_enums = BTreeSet::new();
            for fs in ir.fieldsets.values_mut() {
                for f in fs.fields.iter_mut().filter(|f| f.enumm.is_some()) {
                    used_enums.insert(f.enumm.as_ref().unwrap().clone());
                }
            }

            ir.enums.retain(|name, _| used_enums.contains(name));
        }

        // Ensure consistent sort order in the YAML.
        crate::transform::sort::Sort {}.run(&mut ir).unwrap();

        // Trim all descriptions

        let cleanup = |s: &mut Option<String>| {
            if let Some(s) = s.as_mut() {
                *s = s.trim().to_string()
            }
        };

        for b in ir.blocks.values_mut() {
            cleanup(&mut b.description);
            for i in b.items.iter_mut() {
                cleanup(&mut i.description);
            }
        }

        for b in ir.fieldsets.values_mut() {
            cleanup(&mut b.description);
            for i in b.fields.iter_mut() {
                cleanup(&mut i.description);
            }
        }

        for b in ir.enums.values_mut() {
            cleanup(&mut b.description);
            for i in b.variants.iter_mut() {
                cleanup(&mut i.description);
            }
        }

        let want_data = serde_yaml::to_string(&ir)?;

        if got_data != want_data.as_bytes() {
            if args.check {
                bail!("File {} is not correctly formatted", &file);
            } else {
                fs::write(&file, want_data)?;
            }
        }
    }
    Ok(())
}
