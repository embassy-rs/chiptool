use anyhow::Result;
use proc_macro2::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;

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
                    match r.bit_size {
                        1..=8 => quote!(u8),
                        9..=16 => quote!(u16),
                        17..=32 => quote!(u32),
                        33..=64 => quote!(u64),
                        _ => panic!("Invalid bit_size {}", r.bit_size),
                    }
                };

                let access = match r.access {
                    Access::Read => quote!(#common_path::R),
                    Access::Write => quote!(#common_path::W),
                    Access::ReadWrite => quote!(#common_path::RW),
                };

                let ty = quote!(#common_path::Reg<'_, I, #reg_ty, #access>);
                if let Some(array) = &i.array {
                    let (len, offs_expr) = super::process_array(array);
                    items.extend(quote!(
                        #doc
                        #[inline(always)]
                        pub fn #name(&mut self, n: usize) -> #ty {
                            assert!(n < #len);
                            unsafe { #common_path::Reg::new(self.iface, self.addr + #offset + #offs_expr) }
                        }
                    ));
                } else {
                    items.extend(quote!(
                        #doc
                        #[inline(always)]
                        pub fn #name(&mut self) -> #ty {
                            unsafe { #common_path::Reg::new(self.iface, self.addr + #offset) }
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
    let name = Ident::new(name, span);
    let doc = util::doc(&b.description);
    let out = quote! {
        #doc
        pub struct #name<'a, I> {
            pub iface: &'a mut I,
            pub addr: usize,
        }

        impl<'a, I: Interface> #name<'a, I> {
            #items
        }
    };

    Ok(out)
}
