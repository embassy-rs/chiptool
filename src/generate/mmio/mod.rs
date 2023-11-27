
mod block;
mod device;
mod enumm;
mod fieldset;

use anyhow::Result;
use proc_macro2::TokenStream;
use quote::quote;
use std::str::FromStr;

use crate::ir::*;

use super::{Options, Module, CommonModule, sorted_map, split_path};

pub const COMMON_MODULE: &[u8] = include_bytes!("common.rs");

pub fn render(ir: &IR, opts: &Options) -> Result<TokenStream> {   
    let mut root = Module::new();
    root.items = TokenStream::new(); // Remove default contents

    let commit_info = {
        let tmp = include_str!(concat!(env!("OUT_DIR"), "/commit-info.txt"));

        if tmp.is_empty() {
            " (untracked)"
        } else {
            tmp
        }
    };

    let doc = format!(
        "Peripheral access API (generated using chiptool v{}{})",
        env!("CARGO_PKG_VERSION"),
        commit_info
    );

    root.items.extend(quote!(
        #![no_std]
        #![doc=#doc]
    ));

    for (p, d) in sorted_map(&ir.devices, |name, _| name.clone()) {
        let (mods, _) = split_path(p);
        root.get_by_path(&mods)
            .items
            .extend(device::render(opts, ir, d, p)?);
    }

    for (p, b) in sorted_map(&ir.blocks, |name, _| name.clone()) {
        let (mods, _) = split_path(p);
        root.get_by_path(&mods)
            .items
            .extend(block::render(opts, ir, b, p)?);
    }

    for (p, fs) in sorted_map(&ir.fieldsets, |name, _| name.clone()) {
        let (mods, _) = split_path(p);
        root.get_by_path(&mods)
            .items
            .extend(fieldset::render(opts, ir, fs, p)?);
    }

    for (p, e) in sorted_map(&ir.enums, |name, _| name.clone()) {
        let (mods, _) = split_path(p);
        root.get_by_path(&mods)
            .items
            .extend(enumm::render(opts, ir, e, p)?);
    }

    match &opts.common_module {
        CommonModule::Builtin => {
            let tokens =
                TokenStream::from_str(std::str::from_utf8(COMMON_MODULE).unwrap()).unwrap();

            let module = root.get_by_path(&["common"]);
            module.items = TokenStream::new(); // Remove default contents
            module.items.extend(tokens);
        }
        CommonModule::External(_) => {}
    }

    root.render()
}
