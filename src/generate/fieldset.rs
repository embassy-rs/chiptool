use anyhow::Result;
use proc_macro2::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;

use crate::ir::*;
use crate::util;

pub fn render(_opts: &super::Options, ir: &IR, fs: &FieldSet, path: &str) -> Result<TokenStream> {
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
        let bit_offset = f.bit_offset as usize;
        let _bit_size = f.bit_size as usize;
        let mask = util::hex(1u64.wrapping_shl(f.bit_size).wrapping_sub(1));
        let doc = util::doc(&f.description);
        let field_ty: TokenStream;
        let to_bits: TokenStream;
        let from_bits: TokenStream;

        if let Some(e_path) = &f.enum_readwrite {
            let e = ir.enums.get(e_path).unwrap();

            let enum_ty = match e.bit_size {
                1..=8 => quote!(u8),
                9..=16 => quote!(u16),
                17..=32 => quote!(u32),
                33..=64 => quote!(u64),
                _ => panic!("Invalid bit_size {}", e.bit_size),
            };

            field_ty = util::relative_path(e_path, path);
            to_bits = quote!(val.0 as #ty);
            from_bits = quote!(#field_ty(val as #enum_ty));
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

        if let Some(array) = &f.array {
            let (len, offs_expr) = super::process_array(array);
            items.extend(quote!(
                #doc
                #[inline(always)]
                pub fn #name(&self, n: usize) -> #field_ty{
                    assert!(n < #len);
                    let offs = #bit_offset + #offs_expr;
                    let val = (self.0 >> offs) & #mask;
                    #from_bits
                }
                #doc
                #[inline(always)]
                pub fn #name_set(&mut self, n: usize, val: #field_ty) {
                    assert!(n < #len);
                    let offs = #bit_offset + #offs_expr;
                    self.0 = (self.0 & !(#mask << offs)) | (((#to_bits) & #mask) << offs);
                }
            ));
        } else {
            items.extend(quote!(
                #doc
                #[inline(always)]
                pub const fn #name(&self) -> #field_ty{
                    let val = (self.0 >> #bit_offset) & #mask;
                    #from_bits
                }
                #doc
                #[inline(always)]
                pub fn #name_set(&mut self, val: #field_ty) {
                    self.0 = (self.0 & !(#mask << #bit_offset)) | (((#to_bits) & #mask) << #bit_offset);
                }
            ));
        }
    }

    let (_, name) = super::split_path(path);
    let name = Ident::new(name, span);
    let doc = util::doc(&fs.description);

    let out = quote! {
        #doc
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct #name (pub #ty);

        impl #name {
            #items
        }

        impl Default for #name {
            #[inline(always)]
            fn default() -> #name {
                #name(0)
            }
        }
    };

    Ok(out)
}
