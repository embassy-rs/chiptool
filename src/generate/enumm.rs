use std::collections::BTreeMap;

use anyhow::Result;
use proc_macro2::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;

use crate::ir::*;
use crate::util::{self, StringExt};

use super::sorted;

pub fn render(opts: &super::Options, _ir: &IR, e: &Enum, path: &str) -> Result<TokenStream> {
    let span = Span::call_site();

    // For very "sparse" enums, generate a newtype wrapping the uX.
    // In particular, we generate a newtype if:
    // - there'd be 100 or more "reserved" cases, AND
    // - there'd be 50% or more "reserved" cases.
    let variant_count = e.variants.len() as u64;
    let reserved_count = (1u64 << e.bit_size) - variant_count;
    let newtype = reserved_count >= 100 && reserved_count >= variant_count;

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
        let mut item_names_str = Vec::with_capacity(e.variants.len());
        let mut item_values = Vec::with_capacity(e.variants.len());

        for f in sorted(&e.variants, |f| (f.value, f.name.clone())) {
            let name = Ident::new(&f.name, span);
            let value = util::hex(f.value);

            item_names_str.push(&f.name);
            item_values.push(value.clone());

            let doc = util::doc(&f.description);
            items.extend(quote!(
                #doc
                pub const #name: Self = Self(#value);
            ));
        }

        let defmt = opts.defmt_feature.as_ref().map(|defmt_feature| {
            quote! {
                #[cfg(feature = #defmt_feature)]
                impl defmt::Format for #name {
                    fn format(&self, f: defmt::Formatter) {
                        match self.0 {
                            #(
                                #item_values => defmt::write!(f, #item_names_str),
                            )*
                            other => defmt::write!(f, "0x{:02X}", other),
                        }
                    }
                }
            }
        });

        out.extend(quote! {
            #doc
            #[repr(transparent)]
            #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
            pub struct #name (#ty);

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

            impl core::fmt::Debug for #name {
                fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                    match self.0 {
                        #(
                            #item_values => f.write_str(#item_names_str),
                        )*
                        other => core::write!(f, "0x{:02X}", other),
                    }
                }
            }

            #defmt
        });
    } else {
        let variants: BTreeMap<_, _> = e.variants.iter().map(|v| (v.value, v)).collect();
        let mut items = TokenStream::new();
        for val in 0..(1 << e.bit_size) {
            if let Some(f) = variants.get(&val) {
                let name = Ident::new(&f.name.to_sanitized_pascal_case(), span);
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

        let defmt = opts.defmt_feature.as_ref().map(|defmt_feature| {
            quote! {
                #[cfg_attr(feature = #defmt_feature, derive(defmt::Format))]
            }
        });

        out.extend(quote! {
            #doc
            #[repr(#ty)]
            #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
            #defmt
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
