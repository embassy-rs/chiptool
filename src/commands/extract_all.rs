use crate::commands::{apply_transform, clean_up_ir, extract_peripheral, load_svd, ExtractShared};
use crate::ir::{self, IR};
use crate::svd2ir;
use anyhow::{anyhow, bail, Result};
use clap::{Parser, ValueEnum};
use std::collections::{BTreeSet, HashSet};
use std::fs::File;
use std::path::{Path, PathBuf};

/// Extract all peripherals from SVD to YAML
#[derive(Parser)]
pub struct ExtractAll {
    #[clap(flatten)]
    pub extract_shared: ExtractShared,

    /// The method employed to extract the data.
    #[clap(long, default_value = "peripheral")]
    pub mode: ExtractionMode,

    /// Output directory. Each peripheral will be created as a YAML file here.
    #[clap(short, long)]
    pub output: PathBuf,
}

#[derive(Clone, Copy, ValueEnum)]
pub enum ExtractionMode {
    /// The peripherals are extracted from the SVD directly, and transforms are applied on these individually.
    Peripheral,
    /// The SVD is extracted in its entirety and the transform are applied to that single IR tree.
    ///
    /// Afterwards the blocks associated with peripherals are extracted for this common IR.
    Block,
}

pub fn extract_all(args: ExtractAll) -> Result<()> {
    match args.mode {
        ExtractionMode::Peripheral => extract_per_peripheral(&args.extract_shared, &args.output),
        ExtractionMode::Block => extract_per_block(&args.extract_shared, &args.output),
    }
}

/// Extract each peripheral directly from the SVD, apply transform, and write to disk.
fn extract_per_peripheral(extract_shared: &ExtractShared, output_dir: &Path) -> Result<()> {
    std::fs::create_dir_all(output_dir)?;

    let svd = load_svd(&extract_shared.svd)?;

    for p in &svd.peripherals {
        if p.derived_from.is_some() {
            continue;
        }

        let mut ir = extract_peripheral(p, extract_shared.namespaces)?;

        for transform in extract_shared.transform.iter() {
            apply_transform(&mut ir, transform)?;
        }

        if ir.blocks.is_empty() {
            continue;
        }

        let f = File::create(PathBuf::from(output_dir).join(format!(
                "{}.yaml",
                // Take the shortest block name as the file name
                ir.blocks
                    .keys()
                    .reduce(|acc, val| if val.len() < acc.len() { val } else { acc })
                    .unwrap()
            )))?;
        serde_yaml::to_writer(f, &ir).unwrap();
    }

    Ok(())
}

/// Extract the entire SVD, apply transform, and then figure out which blocks correspond to the peripherals before writing to disk.
fn extract_per_block(extract_shared: &ExtractShared, output_dir: &Path) -> Result<()> {
    std::fs::create_dir_all(output_dir)?;

    let svd = load_svd(&extract_shared.svd)?;
    let mut ir = svd2ir::convert_svd(&svd, extract_shared.namespaces)?;

    clean_up_ir(&mut ir)?;

    for transform in extract_shared.transform.iter() {
        apply_transform(&mut ir, transform)?;
    }

    let blocks: HashSet<String> = ir
        .devices
        .iter()
        .flat_map(|(_, device)| device.peripherals.iter().flat_map(|p| &p.block))
        .cloned()
        .collect();

    for block_name in blocks {
        let block_ir = extract_relevant(&block_name, &ir)?;

        let f = File::create(PathBuf::from(output_dir).join(format!("{}.yaml", block_name)))?;
        serde_yaml::to_writer(f, &block_ir).unwrap();
    }

    Ok(())
}

/// Extract from an IR everything associated with a block.
fn extract_relevant(block_name: &str, ir: &IR) -> Result<IR> {
    #[derive(Default)]
    struct Walker {
        todos: BTreeSet<String>,
        visited: BTreeSet<String>,
    }

    impl Walker {
        pub fn schedule(&mut self, name: &str) -> Result<()> {
            if self.visited.contains(name) {
                bail!("Recursive definition found for {}", name);
            }
            self.todos.insert(name.to_string());
            Ok(())
        }

        pub fn next(&mut self) -> Option<String> {
            let name = self.todos.iter().next().cloned();

            if let Some(name) = &name {
                self.todos.remove(name);
                assert!(self.visited.insert(name.to_string()));
            }

            name
        }
    }

    #[derive(Default)]
    struct State {
        blocks: Walker,
        fieldsets: Walker,
        enums: Walker,
    }

    let mut state = State::default();
    state.blocks.schedule(block_name)?;

    while let Some(block_name) = state.blocks.next() {
        let block = ir
            .blocks
            .get(&block_name)
            .ok_or_else(|| anyhow!("Failed to find reference to block {}", block_name))?;

        if let Some(extends) = &block.extends {
            state.blocks.schedule(extends)?;
        }

        for item in block.items.iter() {
            match &item.inner {
                ir::BlockItemInner::Block(block_item_block) => {
                    state.blocks.schedule(&block_item_block.block)?
                }
                ir::BlockItemInner::Register(register) => {
                    if let Some(fieldset) = &register.fieldset {
                        state.fieldsets.schedule(fieldset)?;
                    }
                }
            }
        }
    }

    while let Some(fieldset_name) = state.fieldsets.next() {
        let fieldset = ir
            .fieldsets
            .get(&fieldset_name)
            .ok_or_else(|| anyhow!("Failed to find reference to fieldset {}", fieldset_name))?;

        if let Some(extends) = &fieldset.extends {
            state.fieldsets.schedule(extends)?;
        }

        for item in fieldset.fields.iter() {
            if let Some(enumm) = &item.enumm {
                state.enums.schedule(enumm)?;
            }
        }
    }

    let mut result = IR::new();
    result.blocks = ir
        .blocks
        .iter()
        .filter(|(name, _)| state.blocks.visited.contains(*name))
        .map(|(name, item)| (name.clone(), item.clone()))
        .collect();

    result.fieldsets = ir
        .fieldsets
        .iter()
        .filter(|(name, _)| state.fieldsets.visited.contains(*name))
        .map(|(name, item)| (name.clone(), item.clone()))
        .collect();

    result.enums = ir
        .enums
        .iter()
        .filter(|(name, _)| state.enums.todos.contains(*name)) // Note we have not visited the enums
        .map(|(name, item)| (name.clone(), item.clone()))
        .collect();

    Ok(result)
}
