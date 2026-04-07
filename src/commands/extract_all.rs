use crate::commands::{extract_peripheral, load_svd, ExtractShared};
use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::path::PathBuf;

/// Extract all peripherals from SVD to YAML
#[derive(Parser)]
pub struct ExtractAll {
    #[clap(flatten)]
    pub extract_shared: ExtractShared,

    /// Output directory. Each peripheral will be created as a YAML file here.
    #[clap(short, long)]
    pub output: PathBuf,
}

pub fn extract_all(args: ExtractAll) -> Result<()> {
    std::fs::create_dir_all(&args.output)?;

    let svd = load_svd(&args.extract_shared.svd)?;

    for p in &svd.peripherals {
        if p.derived_from.is_some() {
            continue;
        }

        let ir = extract_peripheral(
            p,
            &args.extract_shared.transform,
            args.extract_shared.namespaces,
        )?;

        if ir.blocks.is_empty() {
            continue;
        }

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
