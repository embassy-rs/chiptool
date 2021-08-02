#![recursion_limit = "128"]

use anyhow::{Context, Result};
use chiptool::{generate, svd2ir};
use clap::Clap;
use log::*;
use regex::Regex;
use std::fs;
use std::io::Read;
use std::{fs::File, io::stdout};

use chiptool::ir::IR;

#[derive(Clap)]
#[clap(version = "1.0", author = "Dirbaio <dirbaio@dirbaio.net>")]
struct Opts {
    #[clap(subcommand)]
    subcommand: Subcommand,
}

#[derive(Clap)]
enum Subcommand {
    Generate(Generate),
    ExtractPeripheral(ExtractPeripheral),
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

/// Generate a PAC directly from a SVD
#[derive(Clap)]
struct Generate {
    /// SVD file path
    #[clap(long)]
    svd: String,
    /// Transforms file path
    #[clap(long)]
    transform: Option<String>,
}

fn main() -> Result<()> {
    env_logger::init();

    let opts: Opts = Opts::parse();

    match opts.subcommand {
        Subcommand::ExtractPeripheral(x) => extract_peripheral(x),
        Subcommand::Generate(x) => gen(x),
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

fn load_config(path: &str) -> Result<Config> {
    let config = fs::read(path).context("Cannot read the config file")?;
    serde_yaml::from_slice(&config).context("cannot deserialize config")
}

fn extract_peripheral(args: ExtractPeripheral) -> Result<()> {
    let config = match args.transform {
        Some(s) => load_config(&s)?,
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

    chiptool::svd2ir::convert_peripheral(&mut ir, p)?;

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

fn gen(args: Generate) -> Result<()> {
    let config = match args.transform {
        Some(s) => load_config(&s)?,
        None => Config::default(),
    };

    let svd = load_svd(&args.svd)?;
    let mut ir = svd2ir::convert_svd(&svd)?;

    // Fix weird newline spam in descriptions.
    let re = Regex::new("[ \n]+").unwrap();
    chiptool::transform::map_descriptions(&mut ir, |d| re.replace_all(d, " ").into_owned())?;

    for t in &config.transforms {
        info!("running: {:?}", t);
        t.run(&mut ir)?;
    }

    let generate_opts = generate::Options {
        common_module: generate::CommonModule::Builtin,
    };
    let items = generate::render(&ir, &generate_opts).unwrap();
    fs::write("lib.rs", items.to_string())?;

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
