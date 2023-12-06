use std::collections::HashMap;

use anyhow::Result;
use proc_macro2::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;

use crate::ir::*;
use crate::util;
use crate::generate::sorted;

pub fn render(_opts: &super::Options, _ir: &IR, e: &Enum, path: &str) -> Result<TokenStream> {
    let span = Span::call_site();

    // For very "sparse" enums, generate a newtype wrapping the uX.
    let newtype = e.bit_size > 8 || (e.variants.len() < 6 && e.bit_size > 4);

    let ty = match e.bit_size {
        1..=8 => quote!(u8),
        9..=16 => quote!(u16),
        17..=32 => quote!(u32),
        33..=64 => quote!(u64),
        _ => panic!("Invalid bit_size {}", e.bit_size),
    };

    let (_, name) = super::split_path(path);
    let name = Ident::new(name, span);
    let doc = util::doc(&e.description);
    let mask = util::hex(1u64.wrapping_shl(e.bit_size).wrapping_sub(1));

    let mut out = TokenStream::new();

    if newtype {
        let mut items = TokenStream::new();

        for f in sorted(&e.variants, |f| (f.value, f.name.clone())) {
            let name = Ident::new(&f.name, span);
            let value = util::hex(f.value);
            let doc = util::doc(&f.description);
            items.extend(quote!(
                #doc
                pub const #name: Self = Self(#value);
            ));
        }

        out.extend(quote! {
            #doc
            #[repr(transparent)]
            #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
            pub struct #name (pub #ty);

            impl #name {
                #items
            }

            impl #name {
                pub const fn from_bits(val: #ty) -> #name {
                    Self(val & #mask)
                }

                pub const fn to_bits(self) -> #ty {
                    self.0
                }
            }
        });
    } else {
        let variants: HashMap<_, _> = e.variants.iter().map(|v| (v.value, v)).collect();
        let mut items = TokenStream::new();
        for val in 0..(1 << e.bit_size) {
            if let Some(f) = variants.get(&val) {
                let name = Ident::new(&f.name, span);
                let value = util::hex(f.value);
                let doc = util::doc(&f.description);
                items.extend(quote!(
                    #doc
                    #name = #value,
                ));
            } else {
                let name = Ident::new(&format!("_RESERVED_{:x}", val), span);
                let value = util::hex(val);
                items.extend(quote!(
                    #name = #value,
                ));
            }
        }

        out.extend(quote! {
            #doc
            #[repr(#ty)]
            #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
            pub enum #name {
                #items
            }

            impl #name {
                #[inline(always)]
                pub const fn from_bits(val: #ty) -> #name {
                    unsafe { core::mem::transmute(val & #mask) }
                }

                #[inline(always)]
                pub const fn to_bits(self) -> #ty {
                    unsafe { core::mem::transmute(self) }
                }
            }
        });
    }

    out.extend(quote! {
        impl From<#ty> for #name {
            #[inline(always)]
            fn from(val: #ty) -> #name {
                #name::from_bits(val)
            }
        }

        impl From<#name> for #ty {
            #[inline(always)]
            fn from(val: #name) -> #ty {
                #name::to_bits(val)
            }
        }
    });


    Ok(out)
}
