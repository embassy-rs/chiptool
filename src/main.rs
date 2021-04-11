#![recursion_limit = "128"]

use ir::IR;
use log::error;
use quote::__private::ext;
use regex::{Captures, Regex};
use svd_parser as svd;
use transform::map_names;

mod generate;
mod ir;
mod svd2ir;
mod transform;
mod util;

use std::io::Read;
use std::io::Write;
use std::{fs::File, io::stdout};
use std::{process, u32};

use anyhow::{Context, Result};
use clap::Clap;
use log::*;

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
    Generate(Generate),
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

/// Generate a PAC from a set of peripheral YAMLs
#[derive(Clap)]
struct Generate {
    /// Input directory containing the peripheral YAMLs
    #[clap(long)]
    dir: String,
}

fn main() -> Result<()> {
    env_logger::init();

    let opts: Opts = Opts::parse();

    match opts.subcommand {
        Subcommand::ExtractPeripheral(x) => extract_peripheral(x),
        Subcommand::ExtractDevice(x) => extract_device(x),
        Subcommand::Generate(x) => generate(x),
    }
}

fn load_svd(path: &str) -> Result<svd::Device> {
    let xml = &mut String::new();
    File::open(path)
        .context("Cannot open the SVD file")?
        .read_to_string(xml)
        .context("Cannot read the SVD file")?;

    let device = svd::parse(xml)?;
    Ok(device)
}

fn extract_device(args: ExtractDevice) -> Result<()> {
    let svd = load_svd(&args.svd)?;

    let mut device = svd2ir::convert_device(&svd)?;

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
    let p = svd
        .peripherals
        .iter()
        .find(|p| p.name == peri)
        .expect("peripheral not found");

    svd2ir::convert_peripheral(&mut ir, &p)?;

    // Fix weird newline spam in descriptions.
    let re = Regex::new("[ \n]+").unwrap();
    transform::map_descriptions(&mut ir, |d| re.replace_all(d, " ").into_owned())?;

    for t in &config.transforms {
        info!("running: {:?}", t);
        t.run(&mut ir)?;
    }

    // Ensure consistent sort order in the YAML.
    transform::sort::Sort {}.run(&mut ir).unwrap();

    serde_yaml::to_writer(stdout(), &ir).unwrap();
    Ok(())
}

fn generate(args: Generate) -> Result<()> {
    let mut ir = IR::new();
    for f in std::fs::read_dir(&args.dir)
        .unwrap()
        .filter_map(|res| res.unwrap().file_name().to_str().map(|s| s.to_string()))
        .filter(|s| s.ends_with(".yaml"))
    {
        let name = f.strip_suffix(".yaml").unwrap();
        info!("loading {}", name);
        let mut peri: IR =
            serde_yaml::from_reader(File::open(format!("{}/{}", args.dir, f)).unwrap()).unwrap();

        let prefix = name;

        transform::expand_extends::ExpandExtends {}
            .run(&mut peri)
            .unwrap();

        transform::map_names(&mut peri, |s, k| match k {
            transform::NameKind::Block => format!("{}::{}", prefix, s),
            transform::NameKind::Fieldset => format!("{}::regs::{}", prefix, s),
            transform::NameKind::Enum => format!("{}::vals::{}", prefix, s),
            _ => s.to_string(),
        })
        .unwrap();

        ir.merge(peri);
    }

    // Cleanups!

    transform::sort::Sort {}.run(&mut ir).unwrap();
    transform::Sanitize {}.run(&mut ir).unwrap();

    let items = generate::render(&ir)?;
    let mut file = File::create("lib.rs").expect("Couldn't create lib.rs file");
    let data = items.to_string().replace("] ", "]\n");
    file.write_all(data.as_ref())
        .expect("Could not write code to lib.rs");

    Ok(())
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Config {
    transforms: Vec<transform::Transform>,
}

impl Default for Config {
    fn default() -> Self {
        Self { transforms: vec![] }
    }
}
