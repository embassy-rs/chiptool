use crate::commands::load_svd;
use anyhow::Result;
use clap::Parser;
use std::io::stdout;

/// Extract peripheral from SVD to YAML
#[derive(Parser)]
pub struct ExtractPeripheral {
    /// SVD file path
    #[clap(long)]
    pub svd: String,
    /// Peripheral from the SVD
    #[clap(long)]
    pub peripheral: String,
    /// Transforms file path
    #[clap(long)]
    pub transform: Vec<String>,
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
