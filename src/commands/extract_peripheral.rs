use crate::commands::load_svd;
use anyhow::Result;
use clap::Parser;
use std::io::stdout;
use std::path::PathBuf;

/// Extract peripheral from SVD to YAML
#[derive(Parser)]
pub struct ExtractPeripheral {
    /// SVD file path
    #[clap(long)]
    pub svd: PathBuf,
    /// Peripheral from the SVD
    #[clap(long)]
    pub peripheral: PathBuf,
    /// Transforms file path
    #[clap(long)]
    pub transform: Vec<PathBuf>,
}

pub fn extract_peripheral(args: ExtractPeripheral) -> Result<()> {
    let svd = load_svd(&args.svd)?;

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

    let ir = crate::commands::process_peripheral(p, &args.transform)?;

    serde_yaml::to_writer(stdout(), &ir).unwrap();
    Ok(())
}
