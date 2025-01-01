#![recursion_limit = "128"]

use anyhow::{bail, Context, Result};
use chiptool::{generate, svd2ir};
use clap::Parser;
use log::*;
use regex::Regex;
use std::collections::BTreeSet;
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::{fs::File, io::stdout};
use svd_parser::ValidateLevel;

use chiptool::ir::IR;

#[derive(Parser)]
#[clap(version = "1.0", author = "Dirbaio <dirbaio@dirbaio.net>")]
struct Opts {
    #[clap(subcommand)]
    subcommand: Subcommand,
}

#[derive(Parser)]
enum Subcommand {
    Generate(Generate),
    ExtractAll(ExtractAll),
    ExtractPeripheral(ExtractPeripheral),
    Transform(Transform),
    Fmt(Fmt),
    Check(Check),
    GenBlock(GenBlock),
}

/// Extract peripheral from SVD to YAML
#[derive(Parser)]
struct ExtractPeripheral {
    /// SVD file path
    #[clap(long)]
    svd: String,
    /// Peripheral from the SVD
    #[clap(long)]
    peripheral: String,
    /// Transforms file path
    #[clap(long)]
    transform: Vec<String>,
}

/// Extract all peripherals from SVD to YAML
#[derive(Parser)]
struct ExtractAll {
    /// SVD file path
    #[clap(long)]
    svd: String,
    /// Output directory. Each peripheral will be created as a YAML file here.
    #[clap(short, long)]
    output: String,
}

/// Apply transform to YAML
#[derive(Parser)]
struct Transform {
    /// Input YAML path
    #[clap(short, long)]
    input: String,
    /// Output YAML path
    #[clap(short, long)]
    output: String,
    /// Transforms file path
    #[clap(short, long)]
    transform: String,
}

/// Generate a PAC directly from a SVD
#[derive(Parser)]
struct Generate {
    /// SVD file path
    #[clap(long)]
    svd: String,
    /// Transforms file path
    #[clap(long)]
    transform: Vec<String>,
}

/// Reformat a YAML
#[derive(Parser)]
struct Fmt {
    /// Peripheral file path
    files: Vec<String>,
    /// Error if incorrectly formatted, instead of fixing.
    #[clap(long)]
    check: bool,
    /// Remove unused enums
    #[clap(long)]
    remove_unused: bool,
}

/// Check a YAML for errors.
#[derive(Parser)]
struct Check {
    /// Peripheral file path
    files: Vec<String>,

    #[clap(long)]
    allow_register_overlap: bool,
    #[clap(long)]
    allow_field_overlap: bool,
    #[clap(long)]
    allow_enum_dup_value: bool,
    #[clap(long)]
    allow_unused_enums: bool,
    #[clap(long)]
    allow_unused_fieldsets: bool,
}

/// Generate Rust code from a YAML register block
#[derive(Parser)]
struct GenBlock {
    /// Input YAML path
    #[clap(short, long)]
    input: String,
    /// Output YAML path
    #[clap(short, long)]
    output: String,
}

fn main() -> Result<()> {
    env_logger::init();

    let opts: Opts = Opts::parse();

    match opts.subcommand {
        Subcommand::ExtractPeripheral(x) => extract_peripheral(x),
        Subcommand::ExtractAll(x) => extract_all(x),
        Subcommand::Generate(x) => gen(x),
        Subcommand::Transform(x) => transform(x),
        Subcommand::Fmt(x) => fmt(x),
        Subcommand::Check(x) => check(x),
        Subcommand::GenBlock(x) => gen_block(x),
    }
}

fn load_svd(path: &str) -> Result<svd_parser::svd::Device> {
    let xml = &mut String::new();
    File::open(path)
        .context("Cannot open the SVD file")?
        .read_to_string(xml)
        .context("Cannot read the SVD file")?;

    let config = svd_parser::Config::default()
        .expand_properties(true)
        .validate_level(ValidateLevel::Disabled);
    let device = svd_parser::parse_with_config(xml, &config)?;
    Ok(device)
}

fn load_config(path: &str) -> Result<Config> {
    let config = fs::read(path).context("Cannot read the config file")?;
    serde_yaml::from_slice(&config).context("cannot deserialize config")
}

fn extract_peripheral(args: ExtractPeripheral) -> Result<()> {
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

    // Descriptions in SVD's contain a lot of noise and weird formatting. Clean them up.
    let description_cleanups = [
        // Fix weird newline spam in descriptions.
        (Regex::new("[ \n]+").unwrap(), " "),
        // Fix weird tab and cr spam in descriptions.
        (Regex::new("[\r\t]+").unwrap(), " "),
        // Replace double-space (end of sentence) with period.
        (
            Regex::new(r"(?<first_sentence>.*?)[\s]{2}(?<next_sentence>.*)").unwrap(),
            "$first_sentence. $next_sentence",
        ),
        // Make sure every description ends with a period.
        (
            Regex::new(r"(?<full_description>.*)(?<last_character>[\s'[^\.\s']])$").unwrap(),
            "$full_description$last_character.",
        ),
        // Eliminate space characters between end of description and the closing period.
        (
            Regex::new(r"(?<full_description>.*)\s\.$").unwrap(),
            "$full_description.",
        ),
    ];
    for (re, rep) in description_cleanups.iter() {
        chiptool::transform::map_descriptions(&mut ir, |d| re.replace_all(d, *rep).into_owned())?;
    }

    for transform in args.transform {
        apply_transform(&mut ir, transform)?;
    }

    // Ensure consistent sort order in the YAML.
    chiptool::transform::sort::Sort {}.run(&mut ir).unwrap();

    serde_yaml::to_writer(stdout(), &ir).unwrap();
    Ok(())
}

fn extract_all(args: ExtractAll) -> Result<()> {
    std::fs::create_dir_all(&args.output)?;

    let svd = load_svd(&args.svd)?;

    for p in &svd.peripherals {
        if p.derived_from.is_some() {
            continue;
        }

        let mut ir = IR::new();
        chiptool::svd2ir::convert_peripheral(&mut ir, p)?;

        // Fix weird newline spam in descriptions.
        let re = Regex::new("[ \n]+").unwrap();
        chiptool::transform::map_descriptions(&mut ir, |d| re.replace_all(d, " ").into_owned())?;

        // Ensure consistent sort order in the YAML.
        chiptool::transform::sort::Sort {}.run(&mut ir).unwrap();

        let f = File::create(PathBuf::from(&args.output).join(format!("{}.yaml", p.name)))?;
        serde_yaml::to_writer(f, &ir).unwrap();
    }

    Ok(())
}

fn gen(args: Generate) -> Result<()> {
    let svd = load_svd(&args.svd)?;
    let mut ir = svd2ir::convert_svd(&svd)?;

    // Fix weird newline spam in descriptions.
    let re = Regex::new("[ \n]+").unwrap();
    chiptool::transform::map_descriptions(&mut ir, |d| re.replace_all(d, " ").into_owned())?;

    for transform in args.transform {
        apply_transform(&mut ir, transform)?;
    }

    let generate_opts = generate::Options {
        common_module: generate::CommonModule::Builtin,
    };
    let items = generate::render(&ir, &generate_opts).unwrap();
    fs::write("lib.rs", items.to_string())?;

    let device_x = generate::render_device_x(&ir, ir.devices.values().next().unwrap())?;
    fs::write("device.x", device_x)?;

    Ok(())
}

fn transform(args: Transform) -> Result<()> {
    let data = fs::read(&args.input)?;
    let mut ir: IR = serde_yaml::from_slice(&data)?;
    apply_transform(&mut ir, args.transform)?;

    let data = serde_yaml::to_string(&ir)?;
    fs::write(&args.output, data.as_bytes())?;

    Ok(())
}

fn fmt(args: Fmt) -> Result<()> {
    for file in args.files {
        let got_data = fs::read(&file)?;
        let mut ir: IR = serde_yaml::from_slice(&got_data)?;

        if args.remove_unused {
            let mut used_enums = BTreeSet::new();
            for fs in ir.fieldsets.values_mut() {
                for f in fs.fields.iter_mut().filter(|f| f.enumm.is_some()) {
                    used_enums.insert(f.enumm.as_ref().unwrap().clone());
                }
            }

            ir.enums.retain(|name, _| used_enums.contains(name));
        }

        // Ensure consistent sort order in the YAML.
        chiptool::transform::sort::Sort {}.run(&mut ir).unwrap();

        // Trim all descriptions

        let cleanup = |s: &mut Option<String>| {
            if let Some(s) = s.as_mut() {
                *s = s.trim().to_string()
            }
        };

        for b in ir.blocks.values_mut() {
            cleanup(&mut b.description);
            for i in b.items.iter_mut() {
                cleanup(&mut i.description);
            }
        }

        for b in ir.fieldsets.values_mut() {
            cleanup(&mut b.description);
            for i in b.fields.iter_mut() {
                cleanup(&mut i.description);
            }
        }

        for b in ir.enums.values_mut() {
            cleanup(&mut b.description);
            for i in b.variants.iter_mut() {
                cleanup(&mut i.description);
            }
        }

        let want_data = serde_yaml::to_string(&ir)?;

        if got_data != want_data.as_bytes() {
            if args.check {
                bail!("File {} is not correctly formatted", &file);
            } else {
                fs::write(&file, want_data)?;
            }
        }
    }
    Ok(())
}

fn check(args: Check) -> Result<()> {
    let opts = chiptool::validate::Options {
        allow_enum_dup_value: args.allow_enum_dup_value,
        allow_field_overlap: args.allow_field_overlap,
        allow_register_overlap: args.allow_register_overlap,
        allow_unused_enums: args.allow_unused_enums,
        allow_unused_fieldsets: args.allow_unused_fieldsets,
    };

    let mut fails = 0;

    for file in args.files {
        let got_data = fs::read(&file)?;
        let ir: IR = serde_yaml::from_slice(&got_data)?;
        let errs = chiptool::validate::validate(&ir, opts.clone());
        fails += errs.len();
        for e in errs {
            println!("{}: {}", &file, e);
        }
    }

    if fails != 0 {
        bail!("{} failures", fails)
    }

    Ok(())
}

fn gen_block(args: GenBlock) -> Result<()> {
    let data = fs::read(&args.input)?;
    let mut ir: IR = serde_yaml::from_slice(&data)?;

    chiptool::transform::sanitize::Sanitize {}
        .run(&mut ir)
        .unwrap();

    // Ensure consistent sort order in the YAML.
    chiptool::transform::sort::Sort {}.run(&mut ir).unwrap();

    let generate_opts = generate::Options {
        common_module: generate::CommonModule::Builtin,
    };
    let items = generate::render(&ir, &generate_opts).unwrap();
    fs::write(&args.output, items.to_string())?;

    Ok(())
}
#[derive(Default, serde::Serialize, serde::Deserialize)]
struct Config {
    #[serde(default)]
    includes: Vec<String>,
    #[serde(default)]
    transforms: Vec<chiptool::transform::Transform>,
}

fn apply_transform<P: AsRef<std::path::Path>>(ir: &mut IR, p: P) -> anyhow::Result<()> {
    info!("applying transform {}", p.as_ref().display());
    let config = load_config(p.as_ref().to_str().unwrap())?;

    for include in &config.includes {
        let subp = p.as_ref().parent().unwrap().join(include);
        apply_transform(ir, subp)?;
    }
    for transform in &config.transforms {
        info!("running {:?}", transform);
        transform.run(ir)?;
    }

    Ok(())
}
