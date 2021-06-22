mod block;
mod device;
mod enumm;
mod fieldset;

use anyhow::Result;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::collections::HashMap;

use crate::ir::*;

pub const COMMON_MODULE: &[u8] = include_bytes!("common.rs");

struct Module {
    items: TokenStream,
    children: HashMap<String, Module>,
}

impl Module {
    fn new() -> Self {
        Self {
            // Default mod contents
            items: quote!(),
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

pub struct Options {
    pub common_path: syn::Path,
}

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
        "Peripheral access API (generated using svd2rust v{}{})",
        env!("CARGO_PKG_VERSION"),
        commit_info
    );

    root.items.extend(quote!(
        #![no_std]
        #![doc=#doc]
    ));

    for (p, d) in ir.devices.iter() {
        let (mods, _) = split_path(p);
        root.get_by_path(&mods)
            .items
            .extend(device::render(opts, ir, d, p)?);
    }

    for (p, b) in ir.blocks.iter() {
        let (mods, _) = split_path(p);
        root.get_by_path(&mods)
            .items
            .extend(block::render(opts, ir, b, p)?);
    }

    for (p, fs) in ir.fieldsets.iter() {
        let (mods, _) = split_path(p);
        root.get_by_path(&mods)
            .items
            .extend(fieldset::render(opts, ir, fs, p)?);
    }

    for (p, e) in ir.enums.iter() {
        let (mods, _) = split_path(p);
        root.get_by_path(&mods)
            .items
            .extend(enumm::render(opts, ir, e, p)?);
    }

    /*
    let generic_file = std::str::from_utf8(include_bytes!("generic.rs"))?;
    let tokens = syn::parse_file(generic_file)?.into_token_stream();

    let generic_mod = root.get_by_path(&["generic"]);
    generic_mod.items = TokenStream::new(); // Remove default contents
    generic_mod.items.extend(tokens);
     */

    let _x: syn::Path = syn::parse_str("asdf").unwrap();

    root.render()
}

fn split_path(s: &str) -> (Vec<&str>, &str) {
    let mut v: Vec<&str> = s.split("::").collect();
    let n = v.pop().unwrap();
    (v, n)
}

fn process_array(array: &Array) -> (usize, TokenStream) {
    match array {
        Array::Regular(array) => {
            let len = array.len as usize;
            let stride = array.stride as usize;
            let offs_expr = quote!(n*#stride);
            (len, offs_expr)
        }
        Array::Cursed(array) => {
            let len = array.offsets.len();
            let offsets = array
                .offsets
                .iter()
                .map(|&x| x as usize)
                .collect::<Vec<_>>();
            let offs_expr = quote!(([#(#offsets),*][n] as usize));
            (len, offs_expr)
        }
    }
}
