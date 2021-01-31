use std::borrow::Cow;
use std::cmp::Ordering;
use std::collections::HashMap;

use log::warn;
use proc_macro2::TokenStream;
use proc_macro2::{Ident, Punct, Spacing, Span};
use quote::{quote, ToTokens};
use svd_parser::derive_from::DeriveFrom;

use crate::util;
use anyhow::{anyhow, bail, Context, Result};

use crate::ir::*;

pub fn render(d: &Device, fs: &FieldSet) -> Result<TokenStream> {
    let span = Span::call_site();
    let mut items = TokenStream::new();

    let ty = match fs.bit_size {
        1..=8 => quote!(u8),
        9..=16 => quote!(u16),
        17..=32 => quote!(u32),
        33..=64 => quote!(u64),
        _ => panic!("Invalid bit_size {}", fs.bit_size),
    };

    for f in &fs.fields {
        let name = Ident::new(&f.name, span);
        let name_set = Ident::new(&format!("set_{}", f.name), span);
        let bit_offset = f.bit_offset;
        let bit_size = f.bit_size;
        let mask = util::hex(1u64.wrapping_shl(bit_size).wrapping_sub(1));
        let doc = util::doc(&f.description);
        let field_ty: TokenStream;
        let to_bits: TokenStream;
        let from_bits: TokenStream;

        if let Some(e) = f.enumm {
            let e = d.enums.get(e);

            let enum_ty = match e.bit_size {
                1..=8 => quote!(u8),
                9..=16 => quote!(u16),
                17..=32 => quote!(u32),
                33..=64 => quote!(u64),
                _ => panic!("Invalid bit_size {}", e.bit_size),
            };

            field_ty = util::relative_path(&e.path, &fs.path);
            to_bits = quote!(val.to_bits() as #ty);
            from_bits = quote!(#field_ty::from_bits(val as #enum_ty));
        } else {
            field_ty = match f.bit_size {
                1 => quote!(bool),
                2..=8 => quote!(u8),
                9..=16 => quote!(u16),
                17..=32 => quote!(u32),
                33..=64 => quote!(u64),
                _ => panic!("Invalid bit_size {}", f.bit_size),
            };
            to_bits = quote!(val as #ty);
            from_bits = if f.bit_size == 1 {
                quote!(val != 0)
            } else {
                quote!(val as #field_ty)
            }
        }

        items.extend(quote!(
            #doc
            pub const fn #name(&self) -> #field_ty{
                let val = (self.0 >> #bit_offset) & #mask;
                #from_bits
            }
            #doc
            pub fn #name_set(&mut self, val: #field_ty) {
                self.0 = (self.0 & !(#mask << #bit_offset)) | (((#to_bits) & #mask) << #bit_offset);
            }
        ));
    }

    let name = Ident::new(&fs.path.name, span);
    let doc = util::doc(&fs.description);

    let out = quote! {
        #doc
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        pub struct #name (#ty);

        impl #name {
            pub const fn to_bits(&self) -> #ty {
                self.0
            }
            pub const fn from_bits(val: #ty) -> #name {
                #name(val)
            }

            #items
        }
    };

    Ok(out)
}
