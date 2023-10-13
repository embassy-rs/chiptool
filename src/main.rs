#![recursion_limit = "128"]

use anyhow::{bail, Context, Result};
use chiptool::{generate, svd2ir};
use clap::Parser;
use log::*;
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::{fs::File, io::stdout};

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
    transform: Option<String>,
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
    transform: Option<String>,
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

    for t in &config.transforms {
        info!("running: {:?}", t);
        t.run(&mut ir)?;
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

fn transform(args: Transform) -> Result<()> {
    let data = fs::read(&args.input)?;
    let mut ir: IR = serde_yaml::from_slice(&data)?;
    let config = load_config(&args.transform)?;
    for t in &config.transforms {
        info!("running: {:?}", t);
        t.run(&mut ir)?;
    }
    let data = serde_yaml::to_string(&ir)?;
    fs::write(&args.output, data.as_bytes())?;

    Ok(())
}

fn fmt(args: Fmt) -> Result<()> {
    for file in args.files {
        let got_data = fs::read(&file)?;
        let mut ir: IR = serde_yaml::from_slice(&got_data)?;

        if args.remove_unused {
            let mut used_enums = HashSet::new();
            for (_, fs) in &mut ir.fieldsets {
                for f in &mut fs.fields {
                    if let Some(enumm) = &f.enumm {
                        used_enums.insert(enumm.clone());
                    }
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

        for (_, b) in &mut ir.blocks {
            cleanup(&mut b.description);
            for i in &mut b.items {
                cleanup(&mut i.description);
            }
        }

        for (_, b) in &mut ir.fieldsets {
            cleanup(&mut b.description);
            for i in &mut b.fields {
                cleanup(&mut i.description);
            }
        }

        for (_, b) in &mut ir.enums {
            cleanup(&mut b.description);
            for i in &mut b.variants {
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
    for file in args.files {
        let got_data = fs::read(&file)?;
        let ir: IR = serde_yaml::from_slice(&got_data)?;

        let mut printed = false;
        let mut error = move |s: String| {
            if !printed {
                printed = true;
                println!("{}:", &file);
            }
            println!("    {}", s);
        };

        for (name, b) in &ir.blocks {
            for (i1, i2) in Pairs::new(b.items.iter()) {
                if i1.byte_offset == i2.byte_offset {
                    error(format!(
                        "block {}: registers overlap: {} {}",
                        name, i1.name, i2.name
                    ));
                }
            }
        }

        for (name, e) in &ir.enums {
            for (i1, i2) in Pairs::new(e.variants.iter()) {
                if i1.value == i2.value {
                    error(format!(
                        "enum {}: variants with same value: {} {}",
                        name, i1.name, i2.name
                    ));
                }
            }
        }

        for (name, f) in &ir.fieldsets {
            for (i1, i2) in Pairs::new(f.fields.iter()) {
                if i2.bit_offset + i2.bit_size > i1.bit_offset
                    && i1.bit_offset + i1.bit_size > i2.bit_offset
                {
                    error(format!(
                        "fieldset {}: fields overlap: {} {}",
                        name, i1.name, i2.name
                    ));
                }
            }
        }
    }
    Ok(())
}

fn gen_block(args: GenBlock) -> Result<()> {
    let data = fs::read(&args.input)?;
    let mut ir: IR = serde_yaml::from_slice(&data)?;

    chiptool::transform::Sanitize {}.run(&mut ir).unwrap();

    // Ensure consistent sort order in the YAML.
    chiptool::transform::sort::Sort {}.run(&mut ir).unwrap();

    let generate_opts = generate::Options {
        common_module: generate::CommonModule::Builtin,
    };
    let items = generate::render(&ir, &generate_opts).unwrap();
    fs::write(&args.output, items.to_string())?;

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

// ==============

struct Pairs<U: Iterator + Clone> {
    head: Option<U::Item>,
    tail: U,
    next: U,
}

impl<U: Iterator + Clone> Pairs<U> {
    fn new(mut iter: U) -> Self {
        let head = iter.next();
        Pairs {
            head,
            tail: iter.clone(),
            next: iter,
        }
    }
}

impl<U: Iterator + Clone> Iterator for Pairs<U>
where
    U::Item: Clone,
{
    type Item = (U::Item, U::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let a = self.head.as_ref()?.clone();

        if let Some(b) = self.tail.next() {
            return Some((a, b));
        }

        match self.next.next() {
            Some(new_head) => {
                self.head = Some(new_head);
                self.tail = self.next.clone();
                self.next()
            }
            None => None,
        }
    }
}
