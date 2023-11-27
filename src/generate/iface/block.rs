use anyhow::Result;
use proc_macro2::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;

use crate::generate::{process_array, sorted, split_path, Options};
use crate::ir::*;
use crate::util;

pub fn render(opts: &Options, ir: &IR, b: &Block, path: &str) -> Result<TokenStream> {
    let common_path = opts.common_path();

    let span = Span::call_site();
    let mut items = TokenStream::new();

    let addr_ty = if let Some(addr_size) = &b.address_size {
            match addr_size {
                        1..=8 => quote!(u8),
                        9..=16 => quote!(u16),
                        17..=32 => quote!(u32),
                        33..=64 => quote!(u64),
                        _ => panic!("Invalid addr_size {}", addr_size),
                    }
        } else {
            quote!(u32)
        };
    
    for i in sorted(&b.items, |i| (i.byte_offset, i.name.clone())) {
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

                let ty = quote!(#common_path::Reg<'_, I, #addr_ty, #reg_ty, #access>);
                if let Some(array) = &i.array {
                    let (len, offs_expr) = process_array(array);
                    items.extend(quote!(
                        #doc
                        #[inline(always)]
                        pub fn #name(&mut self, n: usize) -> #ty {
                            assert!(n < #len);
                            #common_path::Reg::new(self.iface, (#offset + #offs_expr) as #addr_ty)
                        }
                    ));
                } else {
                    items.extend(quote!(
                        #doc
                        #[inline(always)]
                        pub fn #name(&mut self) -> #ty {
                            #common_path::Reg::new(self.iface, (#offset) as #addr_ty)
                        }
                    ));
                }
            }
            BlockItemInner::Block(b) => {
                let block_path = &b.block;
                let _b2 = ir.blocks.get(block_path).unwrap();
                let ty = util::relative_path(block_path, path);
                if let Some(array) = &i.array {
                    let (len, offs_expr) = process_array(array);

                    items.extend(quote!(
                        #doc
                        #[inline(always)]
                        pub const fn #name(self, n: usize) -> #ty {
                            assert!(n < #len);
                            unsafe { #ty::from_ptr(self.ptr.add(#offset + #offs_expr) as _) }
                        }
                    ));
                } else {
                    items.extend(quote!(
                        #doc
                        #[inline(always)]
                        pub const fn #name(self) -> #ty {
                            unsafe { #ty::from_ptr(self.ptr.add(#offset) as _) }
                        }
                    ));
                }
            }
        }
    }

    let (_, name) = split_path(path);
    let name = Ident::new(name, span);
    let doc = util::doc(&b.description);
    let out = quote! {
        #doc
        pub struct #name<'a, I> {
            pub iface: &'a mut I,
            pub addr: usize,
        }

        impl<'a, I> #name<'a, I> {
            #items
        }
    };

    Ok(out)
}
