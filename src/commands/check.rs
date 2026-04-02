use crate::ir::IR;

use anyhow::{bail, Result};
use clap::Parser;
use std::fs;
use std::path::PathBuf;

/// Check a YAML for errors.
#[derive(Parser)]
pub struct Check {
    /// Peripheral file path
    pub files: Vec<PathBuf>,

    #[clap(long)]
    pub allow_register_overlap: bool,
    #[clap(long)]
    pub allow_field_overlap: bool,
    #[clap(long)]
    pub allow_enum_dup_value: bool,
    #[clap(long)]
    pub allow_unused_enums: bool,
    #[clap(long)]
    pub allow_unused_fieldsets: bool,
}

pub fn check(args: Check) -> Result<()> {
    let opts = crate::validate::Options {
        allow_enum_dup_value: args.allow_enum_dup_value,
        allow_field_overlap: args.allow_field_overlap,
        allow_register_overlap: args.allow_register_overlap,
        allow_unused_enums: args.allow_unused_enums,
        allow_unused_fieldsets: args.allow_unused_fieldsets,
    };

    let mut fails = 0;

    for file in args.files {
        let got_data = fs::read(&file)?;
        let ir: IR = serde_yaml::from_slice(&got_data)?;
        let errs = crate::validate::validate(&ir, opts.clone());
        fails += errs.len();
        for e in errs {
            println!("{}: {}", file.display(), e);
        }
    }

    if fails != 0 {
        bail!("{} failures", fails)
    }

    Ok(())
}
