use crate::commands::apply_transform;
use anyhow::Result;
use clap::Parser;
use std::fs;

use crate::ir::IR;

/// Apply transform to YAML
#[derive(Parser)]
pub struct Transform {
    /// Input YAML path
    #[clap(short, long)]
    pub input: String,
    /// Output YAML path
    #[clap(short, long)]
    pub output: String,
    /// Transforms file path
    #[clap(short, long)]
    pub transform: String,
}

pub fn transform(args: Transform) -> Result<()> {
    let data = fs::read(&args.input)?;
    let mut ir: IR = serde_yaml::from_slice(&data)?;
    apply_transform(&mut ir, args.transform)?;

    let data = serde_yaml::to_string(&ir)?;
    fs::write(&args.output, data.as_bytes())?;

    Ok(())
}
