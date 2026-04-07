use crate::commands::{
    apply_transform, clean_up_ir, get_generate_opts, load_svd, GenerateShared, NamespaceMode,
};
use crate::{generate, svd2ir};
use anyhow::{bail, Context, Result};
use clap::Parser;
use std::fs;
use std::fs::File;
use std::path::PathBuf;

/// Generate a PAC directly from a SVD
#[derive(Parser)]
pub struct Generate {
    /// SVD file path.
    #[clap(long)]
    pub svd: PathBuf,
    /// Transforms file paths.
    #[clap(long)]
    pub transform: Vec<PathBuf>,
    /// Namespaces added to each extracted peripheral.
    #[clap(long, value_enum, default_value = "block-with-regs-vals")]
    pub namespaces: NamespaceMode,
    #[clap(flatten)]
    pub gen_shared: GenerateShared,

    /// Output YAML path for the whole IR. Useful for debugging
    #[clap(long)]
    pub debug_ir_output: Option<PathBuf>,
    /// Output directory of the PAC files.
    #[clap(long)]
    pub output: Option<PathBuf>,
}

pub fn generate(args: Generate) -> Result<()> {
    let svd =
        load_svd(&args.svd).with_context(|| format!("loading svd at {}", args.svd.display()))?;

    let include_regs_vals = match args.namespaces {
        NamespaceMode::None => bail!("Not allowed to generate code without namespaces"), // TODO perhaps allow
        NamespaceMode::Block => false,
        NamespaceMode::BlockWithRegsVals => true,
    };

    let mut ir = svd2ir::convert_svd(&svd, include_regs_vals)
        .with_context(|| format!("converting svd at {}", args.svd.display()))?;

    clean_up_ir(&mut ir)?;

    for transform in args.transform {
        apply_transform(&mut ir, transform)?;
    }

    if let Some(path) = args.debug_ir_output {
        let f = File::create(&path)
            .with_context(|| format!("creating IR output yaml at {}", path.display()))?;
        serde_yaml::to_writer(f, &ir)
            .with_context(|| format!("writing IR output yaml at {}", path.display()))?;
    }
    let generate_opts = get_generate_opts(args.gen_shared)?;

    let output = if let Some(output) = args.output {
        output
    } else {
        std::env::current_dir()?
    };

    let items = generate::render(&ir, &generate_opts).unwrap();
    fs::write(output.join("lib.rs"), items.to_string())?;

    let device_x = generate::render_device_x(&ir, ir.devices.values().next().unwrap())?;
    fs::write(output.join("device.x"), device_x)?;

    Ok(())
}
