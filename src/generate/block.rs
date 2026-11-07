use anyhow::{anyhow, bail, Context, Result};
use proc_macro2::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;

use crate::ir::*;
use crate::util;

use super::sorted;

pub fn render(opts: &super::Options, ir: &IR, b: &Block, path: &str) -> Result<TokenStream> {
    let common_path = opts.common_path();

    let span = Span::call_site();
    let mut items = TokenStream::new();

    for i in sorted(&b.items, |i| (i.byte_offset, i.name.clone())) {
        let name = Ident::new(&i.name, span);
        let offset = util::hex_usize(i.byte_offset as u64);

        let doc = util::doc(&i.description);

        match &i.inner {
            BlockItemInner::Register(r) => {
                let reg_ty = if let Some(fieldset_path) = &r.fieldset {
                    let _f = ir
                        .fieldsets
                        .get(fieldset_path)
                        .ok_or_else(|| anyhow!("Couldn't find fieldset: {fieldset_path}"))?;
                    util::relative_path(fieldset_path, path)
                } else {
                    match r.bit_size {
                        8 => quote!(u8),
                        16 => quote!(u16),
                        32 => quote!(u32),
                        64 => quote!(u64),
                        _ => bail!("Invalid register bit size {}", r.bit_size),
                    }
                };

                let access = match r.access {
                    Access::Read => quote!(#common_path::R),
                    Access::Write => quote!(#common_path::W),
                    Access::ReadWrite => quote!(#common_path::RW),
                };

                let ty = quote!(#common_path::Reg<#reg_ty, #access>);
                if let Some(array) = &i.array {
                    let super::ArrayDescription {
                        array_ty,
                        constructor,
                    } = super::process_ptr_array(
                        array,
                        &quote!(self.ptr.wrapping_add(#offset)),
                        &ty,
                        &common_path,
                    );
                    items.extend(quote!(
                        #doc
                        #[inline(always)]
                        pub const fn #name(self) -> #array_ty {
                            #constructor
                        }
                    ));
                } else {
                    items.extend(quote!(
                        #doc
                        #[inline(always)]
                        pub const fn #name(self) -> #ty {
                            unsafe { #common_path::Reg::from_ptr(self.ptr.wrapping_add(#offset) as _) }
                        }
                    ));
                }
            }
            BlockItemInner::Block(b) => {
                let block_path = &b.block;
                let _b2 = ir
                    .blocks
                    .get(block_path)
                    .with_context(|| format!("Failed to find block {}", block_path))?;

                let ty = util::relative_path(block_path, path);
                if let Some(array) = &i.array {
                    let super::ArrayDescription {
                        array_ty,
                        constructor,
                    } = super::process_ptr_array(
                        array,
                        &quote!(self.ptr.wrapping_add(#offset)),
                        &ty,
                        &common_path,
                    );

                    items.extend(quote!(
                        #doc
                        #[inline(always)]
                        pub const fn #name(self) -> #array_ty {
                            #constructor
                        }
                    ));
                } else {
                    items.extend(quote!(
                        #doc
                        #[inline(always)]
                        pub const fn #name(self) -> #ty {
                            unsafe { #ty::from_ptr(self.ptr.wrapping_add(#offset) as _) }
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
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct #name {
            ptr: *mut u8
        }
        unsafe impl Send for #name {}
        unsafe impl Sync for #name {}
        impl #name {
            #[inline(always)]
            pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
                Self {
                    ptr: ptr as _,
                }
            }

            #[inline(always)]
            pub const fn as_ptr(&self) -> *mut () {
                self.ptr as _
            }

            #items
        }

        impl #common_path::FromPtr for #name {
            unsafe fn from_ptr(ptr: *mut u8) -> Self {
                unsafe { #name::from_ptr(ptr as *mut ()) }
            }
        }
    };

    Ok(out)
}
