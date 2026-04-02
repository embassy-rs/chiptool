use crate::commands::{load_svd, process_peripheral};
use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::path::PathBuf;

/// Extract all peripherals from SVD to YAML
#[derive(Parser)]
pub struct ExtractAll {
    /// SVD file path
    #[clap(long)]
    pub svd: PathBuf,
    /// Output directory. Each peripheral will be created as a YAML file here.
    #[clap(short, long)]
    pub output: PathBuf,
    /// Transforms file path
    #[clap(long)]
    pub transform: Option<Vec<PathBuf>>,
}

pub fn extract_all(args: ExtractAll) -> Result<()> {
    std::fs::create_dir_all(&args.output)?;

    let svd = load_svd(&args.svd)?;

    for p in &svd.peripherals {
        if p.derived_from.is_some() {
            continue;
        }

        let ir = process_peripheral(p, args.transform.as_deref().unwrap_or(&[]))?;

        let f = File::create(PathBuf::from(&args.output).join(format!(
                "{}.yaml",
                // Take the shortest block name as the file name
                ir.blocks
                    .keys()
                    .reduce(|acc, val| if val.len() < acc.len() { val } else { acc })
                    .unwrap()
            )))?;
        serde_yaml::to_writer(f, &ir).unwrap();
    }

    Ok(())
}
