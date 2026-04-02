use crate::{
    commands::{get_generate_opts, GenShared},
    generate,
    ir::IR,
};

use anyhow::Result;
use clap::Parser;
use std::fs;
use std::path::PathBuf;

/// Generate Rust code from a YAML register block
#[derive(Parser)]
pub struct GenBlock {
    /// Input YAML path
    #[clap(short, long)]
    pub input: PathBuf,
    /// Output Rust code path
    #[clap(short, long)]
    pub output: PathBuf,
    #[clap(flatten)]
    pub gen_shared: GenShared,
}

pub fn gen_block(args: GenBlock) -> Result<()> {
    let data = fs::read(&args.input)?;
    let mut ir: IR = serde_yaml::from_slice(&data)?;

    crate::transform::sanitize::Sanitize {}
        .run(&mut ir)
        .unwrap();

    // Ensure consistent sort order in the YAML.
    crate::transform::sort::Sort {}.run(&mut ir).unwrap();

    let generate_opts = get_generate_opts(args.gen_shared)?;
    let items = generate::render(&ir, &generate_opts).unwrap();
    fs::write(&args.output, items.to_string())?;

    Ok(())
}
