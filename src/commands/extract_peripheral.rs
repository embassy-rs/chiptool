use crate::commands::{load_svd, ExtractShared};
use anyhow::Result;
use clap::Parser;
use std::io::stdout;
use std::path::PathBuf;

/// Extract peripheral from SVD to YAML and print to stdout.
#[derive(Parser)]
pub struct ExtractPeripheral {
    #[clap(flatten)]
    pub extract_shared: ExtractShared,

    /// Peripheral from the SVD
    #[clap(long)]
    pub peripheral: PathBuf,
}

pub fn extract_peripheral(args: ExtractPeripheral) -> Result<()> {
    let svd = load_svd(&args.extract_shared.svd)?;

    let peri = args.peripheral;
    let mut p = svd
        .peripherals
        .iter()
        .find(|p| p.name == peri)
        .expect("peripheral not found");

    if let Some(f) = &p.derived_from {
        p = svd
            .peripherals
            .iter()
            .find(|p| p.name == *f)
            .expect("derivedFrom peripheral not found");
    }

    let ir = crate::commands::extract_peripheral(
        p,
        &args.extract_shared.transform,
        args.extract_shared.namespaces,
    )?;

    serde_yaml::to_writer(stdout(), &ir).unwrap();
    Ok(())
}
