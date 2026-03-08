#![recursion_limit = "128"]

use anyhow::{bail, Context, Result};
use chiptool::{generate, svd2ir};
use clap::{Parser, ValueEnum};
use log::*;
use regex::Regex;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::BTreeSet;
use std::fs;
use std::io;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
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

/// Extract peripheral from SVD to Yaml/Toml/Json IR in stdout
#[derive(Parser)]
struct ExtractPeripheral {
    /// SVD file path
    #[clap(long)]
    #[clap(value_name = "FILE")]
    svd: PathBuf,
    /// Peripheral from the SVD
    #[clap(long)]
    #[clap(value_name = "NAME")]
    peripheral: String,
    /// Peripheral IR format
    #[clap(value_name = "FORMAT")]
    #[clap(long, default_value = "yaml")]
    peripheral_format: Format,
    /// Transform file paths
    #[clap(long)]
    #[clap(value_name = "FILES")]
    transform: Vec<PathBuf>,
    /// Transform IR format
    #[clap(value_name = "FORMAT")]
    #[clap(long, default_value = "auto")]
    transform_format: AutoFormat,
}

/// Extract all peripherals from SVD to Yaml/Toml/Json IR files
#[derive(Parser)]
struct ExtractAll {
    /// SVD file path
    #[clap(long)]
    #[clap(value_name = "FILE")]
    svd: PathBuf,
    /// Output directory. Each peripheral will be created as a Yaml/Toml/Json IR file here.
    #[clap(short, long)]
    #[clap(value_name = "DIRECTORY")]
    output: PathBuf,
    /// Output IR format
    #[clap(long)]
    #[clap(value_name = "FORMAT")]
    #[clap(default_value = "yaml")]
    output_format: Format,
}

/// Apply transform to Yaml/Toml/Json IR
#[derive(Parser)]
struct Transform {
    /// Input IR file path
    #[clap(short, long)]
    #[clap(value_name = "FILE")]
    input: PathBuf,
    /// Input IR format
    #[clap(long)]
    #[clap(value_name = "FORMAT")]
    #[clap(default_value = "auto")]
    input_format: AutoFormat,
    /// Output IR file path
    #[clap(short, long)]
    #[clap(value_name = "FILE")]
    output: PathBuf,
    /// Output IR format
    #[clap(long)]
    #[clap(value_name = "FORMAT")]
    #[clap(default_value = "auto")]
    output_format: AutoFormat,
    /// Transform file path
    #[clap(short, long)]
    #[clap(value_name = "FILE")]
    transform: PathBuf,
    /// Transform IR format
    #[clap(value_name = "FORMAT")]
    #[clap(long, default_value = "auto")]
    transform_format: AutoFormat,
}

/// Generate a PAC directly from a SVD
#[derive(Parser)]
struct Generate {
    /// SVD file path
    #[clap(long)]
    #[clap(value_name = "FILE")]
    svd: PathBuf,
    /// Transform file paths
    #[clap(long)]
    #[clap(value_name = "FILES")]
    transform: Vec<PathBuf>,
    /// Transform IR format
    #[clap(value_name = "FORMAT")]
    #[clap(long, default_value = "auto")]
    transform_format: AutoFormat,
    #[clap(flatten)]
    gen_shared: GenShared,
}

/// Reformat Yaml/Toml/Json IR files
#[derive(Parser)]
struct Fmt {
    /// Peripheral IR file paths
    #[arg(required = true)]
    files: Vec<PathBuf>,

    /// Peripheral IR format
    #[clap(long)]
    #[clap(value_name = "FORMAT")]
    #[clap(default_value = "auto")]
    file_format: AutoFormat,
    /// Error if incorrectly formatted, instead of fixing.
    #[clap(long)]
    check: bool,
    /// Remove unused enums
    #[clap(long)]
    remove_unused: bool,
}

/// Check Yaml/Toml/Json IR files for errors
#[derive(Parser)]
struct Check {
    /// Peripheral IR file paths
    #[arg(required = true)]
    files: Vec<PathBuf>,

    /// Peripheral IR format
    #[clap(long)]
    #[clap(value_name = "FORMAT")]
    #[clap(default_value = "auto")]
    file_format: AutoFormat,
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

/// Generate Rust code from a Yaml/Toml/Json IR register block
#[derive(Parser)]
struct GenBlock {
    /// Input IR file path
    #[clap(short, long)]
    #[clap(value_name = "FILE")]
    input: PathBuf,
    /// Input IR format
    #[clap(long)]
    #[clap(value_name = "FORMAT")]
    #[clap(default_value = "auto")]
    input_format: AutoFormat,
    /// Output Rust file path
    #[clap(short, long)]
    #[clap(value_name = "FILE")]
    output: PathBuf,
    #[clap(flatten)]
    gen_shared: GenShared,
}

#[derive(Parser)]
struct GenShared {
    /// Use an external `common` module.
    #[clap(long)]
    #[clap(value_name = "MODULE_PATH")]
    common_module: Option<ModulePath>,
    /// Specify the feature name used in the generated code to conditionally enable defmt support.
    #[clap(long)]
    #[clap(value_name = "FEATURE")]
    #[clap(default_value = "defmt")]
    #[clap(conflicts_with = "no_defmt")]
    #[clap(conflicts_with = "yes_defmt")]
    defmt_feature: String,
    /// Do not add defmt support to the generated code at all.
    #[clap(long)]
    #[clap(conflicts_with = "yes_defmt")]
    no_defmt: bool,
    /// Add defmt support to the generated code unconditionally.
    #[clap(long)]
    yes_defmt: bool,
}

#[derive(Copy, Clone, ValueEnum)]
enum AutoFormat {
    Auto,
    Yaml,
    Toml,
    Json,
}
impl AutoFormat {
    pub fn get_format(&self, path: &Path) -> Result<Format> {
        let format = match self {
            Self::Auto => {
                let some_file_name = path.file_name().and_then(|x| x.to_str());
                match some_file_name {
                    Some(x) if x.ends_with(Format::Yaml.ext()) => Format::Yaml,
                    Some(x) if x.ends_with(Format::Toml.ext()) => Format::Toml,
                    Some(x) if x.ends_with(Format::Json.ext()) => Format::Json,
                    _ => bail!("Cannot determine the format from the path {path:?}"),
                }
            }
            Self::Yaml => Format::Yaml,
            Self::Toml => Format::Toml,
            Self::Json => Format::Json,
        };
        Ok(format)
    }
}

#[derive(ValueEnum, Copy, Clone)]
enum Format {
    Yaml,
    Toml,
    Json,
}
impl Format {
    pub fn ext(&self) -> &'static str {
        match self {
            Self::Yaml => ".yaml",
            Self::Toml => ".toml",
            Self::Json => ".json",
        }
    }
    pub fn load_from_path<T: DeserializeOwned>(&self, path: &Path) -> Result<T> {
        let f = File::open(path).with_context(|| format!("Cannot open the file {path:?}"))?;
        self.load_from_reader(f)
    }
    pub fn load_from_reader<R: Read, T: DeserializeOwned>(&self, reader: R) -> Result<T> {
        match self {
            Self::Yaml => serde_yaml::from_reader(reader).context("Cannot deserialize the Yaml"),
            Self::Toml => {
                let s = io::read_to_string(reader).context("Cannot read the TOml")?;
                toml::from_str(&s).context("Cannot deserialize the Toml")
            }
            Self::Json => serde_json::from_reader(reader).context("Cannot deserialize the Json"),
        }
    }
    pub fn save_to_path<T: Serialize>(&self, path: &Path, value: &T) -> Result<()> {
        let f = File::create(path).with_context(|| format!("Cannot create the file {path:?}"))?;
        self.save_to_writer(f, value)
    }
    pub fn save_to_writer<W: Write, T: Serialize>(&self, mut writer: W, value: &T) -> Result<()> {
        match self {
            Self::Yaml => {
                serde_yaml::to_writer(writer, value).context("Cannot serialize the Yaml")?
            }
            Self::Toml => {
                let data = toml::to_string(value).context("Cannot serialize the Toml")?;
                writer
                    .write_all(data.as_bytes())
                    .context("Cannot write the Toml")?;
            }
            Self::Json => {
                serde_json::to_writer_pretty(writer, value).context("Cannot serialize the Json")?
            }
        }
        Ok(())
    }
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

fn load_svd(path: &Path) -> Result<svd_parser::svd::Device> {
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

fn load_config(path: &Path, config_format: AutoFormat) -> Result<Config> {
    config_format
        .get_format(path)
        .and_then(|format| format.load_from_path(path))
        .with_context(|| format!("Cannot read the config file {path:?}"))
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
        apply_transform(&mut ir, &transform, args.transform_format)?;
    }

    // Ensure consistent sort order in the YAML.
    chiptool::transform::sort::Sort {}.run(&mut ir).unwrap();

    args.peripheral_format.save_to_writer(stdout(), &ir)?;
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

        let filename = format!("{}{}", p.name, args.output_format.ext());
        let path = args.output.join(filename);
        args.output_format.save_to_path(&path, &ir)?;
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
        apply_transform(&mut ir, &transform, args.transform_format)?;
    }

    let generate_opts = get_generate_opts(args.gen_shared)?;
    let items = generate::render(&ir, &generate_opts).unwrap();
    fs::write("lib.rs", items.to_string())?;

    let device_x = generate::render_device_x(&ir, ir.devices.values().next().unwrap())?;
    fs::write("device.x", device_x)?;

    Ok(())
}

fn transform(args: Transform) -> Result<()> {
    let mut ir: IR = args
        .input_format
        .get_format(&args.input)
        .and_then(|format| format.load_from_path(&args.input))
        .with_context(|| format!("Cannot load the input IR file {:?}", args.input))?;

    apply_transform(&mut ir, &args.transform, args.transform_format)?;

    args.output_format
        .get_format(&args.output)
        .and_then(|format| format.save_to_path(&args.output, &ir))
        .with_context(|| format!("Cannot save the output IR file {:?}", args.output))?;

    Ok(())
}

fn fmt(args: Fmt) -> Result<()> {
    for file in args.files {
        let got_data =
            fs::read(&file).with_context(|| format!("Cannot read the IR file {file:?}"))?;
        let mut ir: IR = args
            .file_format
            .get_format(&file)
            .and_then(|format| format.load_from_reader(got_data.as_slice()))
            .with_context(|| format!("Cannot load the IR file {file:?}"))?;

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

        let mut want_data = Vec::new();
        args.file_format
            .get_format(&file)
            .and_then(|format| format.save_to_writer(&mut want_data, &ir))
            .with_context(|| format!("Cannot serialize the IR of file {file:?}"))?;

        if got_data != want_data.as_slice() {
            if args.check {
                bail!("File {:?} is not correctly formatted", &file);
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
        let ir: IR = args
            .file_format
            .get_format(&file)
            .and_then(|format| format.load_from_path(&file))
            .with_context(|| format!("Cannot load the IR file {file:?}"))?;
        let errs = chiptool::validate::validate(&ir, opts.clone());
        fails += errs.len();
        for e in errs {
            println!("{:?}: {}", &file, e);
        }
    }

    if fails != 0 {
        bail!("{} failures", fails)
    }

    Ok(())
}

fn gen_block(args: GenBlock) -> Result<()> {
    let mut ir: IR = args
        .input_format
        .get_format(&args.input)
        .and_then(|format| format.load_from_path(&args.input))
        .with_context(|| format!("Cannot load the input IR file {:?}", args.input))?;

    chiptool::transform::sanitize::Sanitize {}
        .run(&mut ir)
        .unwrap();

    // Ensure consistent sort order in the YAML.
    chiptool::transform::sort::Sort {}.run(&mut ir).unwrap();

    let generate_opts = get_generate_opts(args.gen_shared)?;
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

fn apply_transform(ir: &mut IR, p: &Path, transform_format: AutoFormat) -> Result<()> {
    info!("applying transform {:?}", p);
    let config = load_config(p, transform_format)?;

    for include in &config.includes {
        let subp = p.join(include);
        apply_transform(ir, &subp, transform_format)?;
    }
    for transform in &config.transforms {
        info!("running {:?}", transform);
        transform.run(ir)?;
    }

    Ok(())
}

/// Struct holding a valid module path as a string.
///
/// Implements `FromStr` so it can be used directly as command line argument.
#[derive(Clone)]
struct ModulePath {
    path: String,
}

impl ModulePath {
    /// Get the module path as a TokenStream.
    fn tokens(&self) -> proc_macro2::TokenStream {
        self.path.parse().unwrap()
    }
}

impl std::str::FromStr for ModulePath {
    type Err = anyhow::Error;

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        data.parse::<proc_macro2::TokenStream>()
            .map_err(|e| anyhow::anyhow!("{e}"))?;

        for (i, component) in data.split("::").enumerate() {
            if component.is_empty() && i != 0 {
                anyhow::bail!("path components can not be empty")
            }
            for (i, c) in component.chars().enumerate() {
                if c.is_alphabetic() || c == '_' {
                    continue;
                }
                if i > 0 && c.is_alphanumeric() {
                    continue;
                }
                anyhow::bail!("path components may only consist of letters, digits and underscore")
            }
        }

        Ok(Self { path: data.into() })
    }
}

fn get_generate_opts(args: GenShared) -> Result<generate::Options> {
    let common_module = match args.common_module {
        None => generate::CommonModule::Builtin,
        Some(module) => generate::CommonModule::External(module.tokens()),
    };

    let defmt = match (args.no_defmt, args.yes_defmt) {
        (true, false) => generate::DefmtOption::Disabled,
        (false, true) => generate::DefmtOption::Enabled,
        (false, false) => generate::DefmtOption::Feature(args.defmt_feature),
        (true, true) => bail!("--no-defmt and --yes-defmt are mutually exclusive"),
    };

    let opts = generate::Options::default()
        .with_common_module(common_module)
        .with_defmt(defmt);
    Ok(opts)
}
