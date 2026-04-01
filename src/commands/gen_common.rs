use crate::generate;
use anyhow::Result;
use clap::Parser;
use std::fs;

/// Output the common.rs file
#[derive(Parser)]
pub struct GenCommon {
    /// Output Rust code *file* path
    #[clap(short, long)]
    pub output: String,
}

pub fn gen_common(args: GenCommon) -> Result<()> {
    fs::write(&args.output, generate::COMMON_MODULE)?;

    Ok(())
}
