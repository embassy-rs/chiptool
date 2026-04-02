//! Commands as exposed in the chiptool binary.
//!
//! For when using chiptool as a library, these commands are exposed for ease of use.

use crate::{ir::IR, svd2ir};

use anyhow::{bail, Context, Result};
use clap::Parser;
use log::*;
use std::fs;
use std::io::Read;
use svd_parser::ValidateLevel;

pub mod check;
pub mod extract_all;
pub mod extract_peripheral;
pub mod fmt;
pub mod gen_block;
pub mod gen_common;
pub mod generate;
pub mod transform;

#[derive(Parser)]
pub struct GenShared {
    /// Use an external `common` module.
    #[clap(long)]
    #[clap(value_name = "MODULE_PATH")]
    pub common_module: Option<ModulePath>,
    /// Specify the feature name used in the generated code to conditionally enable defmt support.
    #[clap(long)]
    #[clap(value_name = "FEATURE")]
    #[clap(default_value = "defmt")]
    #[clap(conflicts_with = "no_defmt")]
    #[clap(conflicts_with = "yes_defmt")]
    pub defmt_feature: String,
    /// Do not add defmt support to the generated code at all.
    #[clap(long)]
    #[clap(conflicts_with = "yes_defmt")]
    pub no_defmt: bool,
    /// Add defmt support to the generated code unconditionally.
    #[clap(long)]
    pub yes_defmt: bool,
    /// Don't put a `#![no_std]` attribute on the generated files.
    #[clap(long)]
    pub skip_no_std: bool,
}

fn clean_up_ir(ir: &mut IR) -> Result<(), anyhow::Error> {
    crate::transform::clean_descriptions::CleanDescriptions {}.run(ir)
}

/// Extract a peripheral from SVD, clean it up and apply specified transform files.
///
/// Applies final sorting after applying the transform.
pub fn process_peripheral(
    p: &svd_parser::svd::Peripheral,
    transform: &[String],
) -> Result<IR, anyhow::Error> {
    let mut ir = IR::new();
    svd2ir::convert_peripheral(&mut ir, p)?;
    clean_up_ir(&mut ir)?;
    for transform in transform.iter() {
        crate::commands::apply_transform(&mut ir, transform).context(transform.to_string())?;
    }

    // Ensure consistent sort order in the YAML.
    crate::transform::sort::Sort {}.run(&mut ir).unwrap();
    Ok(ir)
}

fn load_svd(path: &str) -> Result<svd_parser::svd::Device> {
    let xml = &mut String::new();
    fs::File::open(path)
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
    let config = fs::read(path).with_context(|| format!("Cannot read the config file: {path}"))?;
    serde_yaml::from_slice(&config).context("cannot deserialize config")
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
struct Config {
    #[serde(default)]
    includes: Vec<String>,
    #[serde(default)]
    transforms: Vec<crate::transform::Transform>,
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

/// Struct holding a valid module path as a string.
///
/// Implements `FromStr` so it can be used directly as command line argument.
#[derive(Clone)]
pub struct ModulePath {
    pub path: String,
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

fn get_generate_opts(args: GenShared) -> Result<crate::generate::Options> {
    let common_module = match args.common_module {
        None => crate::generate::CommonModule::Builtin,
        Some(module) => crate::generate::CommonModule::External(module.tokens()),
    };

    let defmt = match (args.no_defmt, args.yes_defmt) {
        (true, false) => crate::generate::DefmtOption::Disabled,
        (false, true) => crate::generate::DefmtOption::Enabled,
        (false, false) => crate::generate::DefmtOption::Feature(args.defmt_feature),
        (true, true) => bail!("--no-defmt and --yes-defmt are mutually exclusive"),
    };

    let opts = crate::generate::Options::default()
        .with_common_module(common_module)
        .with_defmt(defmt)
        .with_skip_no_std(args.skip_no_std);
    Ok(opts)
}
