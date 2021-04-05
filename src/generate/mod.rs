mod block;
//mod device;
mod enumm;
mod fieldset;

use std::collections::HashMap;

use anyhow::Result;
use proc_macro2::{Ident, Span, TokenStream};
use quote::ToTokens;
use quote::{quote, TokenStreamExt};

use crate::ir::*;

struct Module {
    items: TokenStream,
    children: HashMap<String, Module>,
}

impl Module {
    fn new() -> Self {
        Self {
            // Default mod contents
            items: quote!(
                use crate::generic::*;
            ),
            children: HashMap::new(),
        }
    }

    fn get_by_path(&mut self, path: &[&str]) -> &mut Module {
        if path.is_empty() {
            return self;
        }

        self.children
            .entry(path[0].to_owned())
            .or_insert_with(|| Module::new())
            .get_by_path(&path[1..])
    }

    fn render(self) -> Result<TokenStream> {
        let span = Span::call_site();

        let mut res = TokenStream::new();
        res.extend(self.items);

        for (name, module) in self.children.into_iter() {
            let name = Ident::new(&name, span);
            let contents = module.render()?;
            res.extend(quote! {
                pub mod #name {
                    #contents
                }
            });
        }
        Ok(res)
    }
}

pub fn render(ir: &IR) -> Result<TokenStream> {
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
        "Peripheral access API (generated using svd2rust v{}{})",
        env!("CARGO_PKG_VERSION"),
        commit_info
    );

    root.items.extend(quote!(
        #![no_std]
        #![doc=#doc]
    ));

    /*
    for (_, d) in ir.devices.iter() {
        root.get_by_path(&d.path.modules)
            .items
            .extend(device::render(ir, d)?);
    }
     */

    for (p, b) in ir.blocks.iter() {
        let (mods, _) = split_path(p);
        root.get_by_path(&mods)
            .items
            .extend(block::render(ir, b, p)?);
    }

    for (p, fs) in ir.fieldsets.iter() {
        let (mods, _) = split_path(p);
        root.get_by_path(&mods)
            .items
            .extend(fieldset::render(ir, fs, p)?);
    }

    for (p, e) in ir.enums.iter() {
        let (mods, _) = split_path(p);
        root.get_by_path(&mods)
            .items
            .extend(enumm::render(ir, e, p)?);
    }

    let generic_file = std::str::from_utf8(include_bytes!("generic.rs"))?;
    let tokens = syn::parse_file(generic_file)?.into_token_stream();

    let generic_mod = root.get_by_path(&["generic"]);
    generic_mod.items = TokenStream::new(); // Remove default contents
    generic_mod.items.extend(tokens);

    root.render()
}

fn split_path(s: &str) -> (Vec<&str>, &str) {
    let mut v: Vec<&str> = s.split("::").collect();
    let n = v.pop().unwrap();
    (v, n)
}
