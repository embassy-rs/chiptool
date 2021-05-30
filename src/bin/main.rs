#![recursion_limit = "128"]

use anyhow::{Context, Result};
use clap::Clap;
use log::error;
use log::*;
use quote::__private::ext;
use regex::{Captures, Regex};
use serde::Deserialize;
use std::collections::HashMap;
use std::io::Read;
use std::io::Write;
use std::{fs::File, io::stdout};
use std::{process, u32};

use chiptool::ir::IR;

#[derive(Clap)]
#[clap(version = "1.0", author = "Dirbaio <dirbaio@dirbaio.net>")]
struct Opts {
    #[clap(subcommand)]
    subcommand: Subcommand,
}

#[derive(Clap)]
enum Subcommand {
    ExtractPeripheral(ExtractPeripheral),
    ExtractDevice(ExtractDevice),
}

/// Extract peripheral from SVD to YAML
#[derive(Clap)]
struct ExtractPeripheral {
    /// SVD file path
    #[clap(long)]
    svd: String,
    /// Peripheral from the SVD
    #[clap(long)]
    peripheral: String,
    /// Transforms file path
    #[clap(long)]
    transform: Option<String>,
}

/// Extract peripheral from SVD to YAML
#[derive(Clap)]
struct ExtractDevice {
    /// SVD file path
    #[clap(long)]
    svd: String,
}

fn main() -> Result<()> {
    env_logger::init();

    let opts: Opts = Opts::parse();

    match opts.subcommand {
        Subcommand::ExtractPeripheral(x) => extract_peripheral(x),
        Subcommand::ExtractDevice(x) => extract_device(x),
    }
}

fn load_svd(path: &str) -> Result<svd_parser::Device> {
    let xml = &mut String::new();
    File::open(path)
        .context("Cannot open the SVD file")?
        .read_to_string(xml)
        .context("Cannot read the SVD file")?;

    let device = svd_parser::parse(xml)?;
    Ok(device)
}

fn extract_device(args: ExtractDevice) -> Result<()> {
    let svd = load_svd(&args.svd)?;

    let mut device = chiptool::svd2ir::convert_device(&svd)?;

    device.peripherals.sort_by_key(|p| p.name.clone());
    device.interrupts.sort_by_key(|p| p.value);

    let y = serde_yaml::to_string(&device).unwrap();

    // Convert all addresses to hex...
    let re = Regex::new("base_address: (\\d+)").unwrap();
    let y = re.replace_all(&y, |caps: &Captures| {
        format!("base_address: 0x{:08x}", &caps[1].parse::<u32>().unwrap())
    });

    stdout().write_all(y.as_bytes()).unwrap();

    Ok(())
}

fn extract_peripheral(args: ExtractPeripheral) -> Result<()> {
    let config = match args.transform {
        Some(file) => {
            let config = &mut String::new();
            File::open(file)
                .context("Cannot open the config file")?
                .read_to_string(config)
                .context("Cannot read the config file")?;
            serde_yaml::from_str(config).context("cannot deserialize config")?
        }
        None => Config::default(),
    };

    let svd = load_svd(&args.svd)?;
    let mut ir = IR::new();

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

    chiptool::svd2ir::convert_peripheral(&mut ir, &p)?;

    // Fix weird newline spam in descriptions.
    let re = Regex::new("[ \n]+").unwrap();
    chiptool::transform::map_descriptions(&mut ir, |d| re.replace_all(d, " ").into_owned())?;

    for t in &config.transforms {
        info!("running: {:?}", t);
        t.run(&mut ir)?;
    }

    // Ensure consistent sort order in the YAML.
    chiptool::transform::sort::Sort {}.run(&mut ir).unwrap();

    serde_yaml::to_writer(stdout(), &ir).unwrap();
    Ok(())
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Config {
    transforms: Vec<chiptool::transform::Transform>,
}

impl Default for Config {
    fn default() -> Self {
        Self { transforms: vec![] }
    }
}
