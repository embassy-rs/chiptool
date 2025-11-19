mod block;
mod device;
mod enumm;
mod fieldset;

use anyhow::Result;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::collections::BTreeMap;
use std::str::FromStr;

use crate::ir::*;

pub use device::render_device_x;

pub const COMMON_MODULE: &[u8] = include_bytes!("common.rs");

struct Module {
    items: TokenStream,
    children: BTreeMap<String, Module>,
}

impl Module {
    fn new() -> Self {
        Self {
            // Default mod contents
            items: quote!(),
            children: BTreeMap::new(),
        }
    }

    fn get_by_path(&mut self, path: &[&str]) -> &mut Module {
        if path.is_empty() {
            return self;
        }

        self.children
            .entry(path[0].to_owned())
            .or_insert_with(Module::new)
            .get_by_path(&path[1..])
    }

    fn render(&self) -> Result<TokenStream> {
        let span = Span::call_site();

        let mut res = TokenStream::new();
        res.extend(self.items.clone());

        for (name, module) in sorted_map(&self.children, |name, _| name.clone()) {
            let name = Ident::new(name, span);
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

#[derive(Debug, Default)]
pub enum CommonModule {
    #[default]
    Builtin,
    External(TokenStream),
}

#[derive(clap::ValueEnum, Debug, Default, Clone, Copy, PartialEq)]
pub enum Target {
    #[default]
    CortexM,
    Riscv,
}

/// Options for the code generator.
///
/// See the individual methods for the different options you can change.
#[derive(Debug)]
pub struct Options {
    common_module: CommonModule,
    defmt: DefmtOption,
    target: Target,
}

/// Option for generating code for `defmt` support.
#[derive(Debug, Clone)]
pub enum DefmtOption {
    /// No code referencing `defmt` will be generated.
    Disabled,
    /// Support for `defmt` will be gated behind this feature.
    Feature(String),
    /// Support for `defmt` will be included unconditionally.
    Enabled,
}

impl Default for Options {
    fn default() -> Self {
        Self::new()
    }
}

impl Options {
    /// Create new options with all values set to the default.
    ///
    /// This will use a builtin common module,
    /// and adds `defmt` support to the generated code gated behind a `feature = "defmt"` flag.
    pub fn new() -> Self {
        Self {
            common_module: CommonModule::Builtin,
            defmt: DefmtOption::Feature("defmt".to_owned()),
            target: Target::default(),
        }
    }

    /// Get the path to the common module.
    fn common_path(&self) -> TokenStream {
        match &self.common_module {
            CommonModule::Builtin => TokenStream::from_str("crate::common").unwrap(),
            CommonModule::External(path) => path.clone(),
        }
    }

    /// Get the configuration of the common module.
    pub fn common_module(&self) -> &CommonModule {
        &self.common_module
    }

    /// Set the common module to use.
    ///
    /// Specify [`CommonModule::Builtin`] for a built-in common module,
    /// or [`CommonModule::External`] to use an external common module.
    pub fn with_common_module(mut self, common_module: CommonModule) -> Self {
        self.common_module = common_module;
        self
    }

    /// Set the option for adding defmt support in the generated code.
    pub fn with_defmt(mut self, defmt: DefmtOption) -> Self {
        self.defmt = defmt;
        self
    }

    /// Get the option for adding `defmt` support in the generated code.
    pub fn defmt(&self) -> &DefmtOption {
        &self.defmt
    }

    /// Select what kind fo target to generate code for.
    pub fn with_target(mut self, target: Target) -> Self {
        self.target = target;
        self
    }
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
        "Peripheral access API (generated using chiptool v{}{})",
        env!("CARGO_PKG_VERSION"),
        commit_info
    );

    root.items.extend(quote!(
        #![allow(non_camel_case_types)]
        #![allow(non_snake_case)]
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

fn sorted<'a, T: 'a, F, Z>(
    v: impl IntoIterator<Item = &'a T>,
    by: F,
) -> impl IntoIterator<Item = &'a T>
where
    F: Fn(&T) -> Z,
    Z: Ord,
{
    let mut v = v.into_iter().collect::<Vec<_>>();
    v.sort_by_key(|v| by(*v));
    v
}

fn sorted_map<'a, K: 'a, V: 'a, F, Z>(
    v: impl IntoIterator<Item = (&'a K, &'a V)>,
    by: F,
) -> impl IntoIterator<Item = (&'a K, &'a V)>
where
    F: Fn(&K, &V) -> Z,
    Z: Ord,
{
    let mut v = v.into_iter().collect::<Vec<_>>();
    v.sort_by_key(|&(k, v)| by(k, v));
    v
}

fn with_defmt_cfg<F>(defmt: &DefmtOption, f: F) -> Option<TokenStream>
where
    F: FnOnce() -> TokenStream,
{
    match defmt {
        DefmtOption::Disabled => None,
        DefmtOption::Feature(feature) => {
            let body = f();
            Some(quote! {
                #[cfg(feature = #feature)]
                #body
            })
        }
        DefmtOption::Enabled => Some(f()),
    }
}

fn with_defmt_cfg_attr(defmt: &DefmtOption, attr: TokenStream) -> Option<TokenStream> {
    match defmt {
        DefmtOption::Disabled => None,
        DefmtOption::Feature(feature) => Some(quote! { #[cfg_attr(feature = #feature, #attr)] }),
        DefmtOption::Enabled => Some(quote! { #[#attr] }),
    }
}
