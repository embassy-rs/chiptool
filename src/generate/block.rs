use anyhow::Result;
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::quote;
use std::collections::HashMap;

use crate::ir::*;
use crate::util;

pub fn render(opts: &super::Options, ir: &IR, b: &Block, path: &str) -> Result<TokenStream> {
    let common_path = opts.common_path();

    let span = Span::call_site();
    let mut items = TokenStream::new();

    for i in &b.items {
        let name = Ident::new(&i.name, span);
        let offset = i.byte_offset as usize;

        let doc = util::doc(&i.description);

        match &i.inner {
            BlockItemInner::Register(r) => {
                let reg_ty = if let Some(fieldset_path) = &r.fieldset {
                    let _f = ir.fieldsets.get(fieldset_path).unwrap();
                    util::relative_path(fieldset_path, path)
                } else {
                    quote!(u32) // todo
                };

                let access = match r.access {
                    Access::Read => quote!(#common_path::R),
                    Access::Write => quote!(#common_path::W),
                    Access::ReadWrite => quote!(#common_path::RW),
                };

                let ty = quote!(#common_path::Reg<#reg_ty, #access>);
                if let Some(array) = &i.array {
                    let (len, offs_expr) = super::process_array(array);
                    items.extend(quote!(
                        #doc
                        #[inline(always)]
                        pub fn #name(self, n: usize) -> #ty {
                            assert!(n < #len);
                            unsafe { #common_path::Reg::from_ptr(self.0.add(#offset + #offs_expr)) }
                        }
                    ));
                } else {
                    items.extend(quote!(
                        #doc
                        #[inline(always)]
                        pub fn #name(self) -> #ty {
                            unsafe { #common_path::Reg::from_ptr(self.0.add(#offset)) }
                        }
                    ));
                }
            }
            BlockItemInner::Block(b) => {
                let block_path = &b.block;
                let _b2 = ir.blocks.get(block_path).unwrap();
                let ty = util::relative_path(block_path, path);
                if let Some(array) = &i.array {
                    let (len, offs_expr) = super::process_array(array);

                    items.extend(quote!(
                        #doc
                        #[inline(always)]
                        pub fn #name(self, n: usize) -> #ty {
                            assert!(n < #len);
                            unsafe { #ty(self.0.add(#offset + #offs_expr)) }
                        }
                    ));
                } else {
                    items.extend(quote!(
                        #doc
                        #[inline(always)]
                        pub fn #name(self) -> #ty {
                            unsafe { #ty(self.0.add(#offset)) }
                        }
                    ));
                }
            }
        }
    }

    let (_, name) = super::split_path(path);
    let unknown = Literal::string(&format!("Unknown {}", name));
    let name = Ident::new(name, span);
    let doc = util::doc(&b.description);

    let addrs = get_addrs(ir, b);
    let addrs = addrs.iter().map(|(addr, block_name)| {
        let addr = Literal::u32_suffixed(*addr);
        let block_name = Literal::string(block_name);
        quote!(#addr => f.write_str(#block_name),)
    });

    let out = quote! {
        #doc
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct #name (pub *mut u8);
        unsafe impl Send for #name {}
        unsafe impl Sync for #name {}
        impl #name {
            #items
        }

        impl core::fmt::Display for #name {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
                match self.0 {
                    #(
                        #addrs
                    )*
                    _ => f.write_str(#unknown),
                }
            }
        }
    };

    Ok(out)
}

fn get_addrs(_ir: &IR, _b: &Block) -> HashMap<u32, String> {
    todo!()
}
