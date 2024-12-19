use anyhow::Result;
use proc_macro2::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;

use crate::ir::*;
use crate::util;

use super::sorted;

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

    for f in sorted(&fs.fields, |f| (f.bit_offset.clone(), f.name.clone())) {
        let name = Ident::new(&f.name, span);
        let name_set = Ident::new(&format!("set_{}", f.name), span);
        let off_in_reg = f.bit_offset.clone();
        let _bit_size = f.bit_size as usize;
        let mask = util::hex(1u64.wrapping_shl(f.bit_size).wrapping_sub(1));
        let doc = util::doc(&f.description);
        let field_ty: TokenStream;
        let to_bits: TokenStream;
        let from_bits: TokenStream;

        let have_read: bool = f.access == Access::Read || f.access == Access::ReadWrite;
        let have_write: bool = f.access == Access::Write || f.access == Access::ReadWrite;

        if let Some(e_path) = &f.enumm {
            let Some(e) = ir.enums.get(e_path) else {
                panic!("missing enum {}", e_path);
            };

            let enum_ty = match e.bit_size {
                1..=8 => quote!(u8),
                9..=16 => quote!(u16),
                17..=32 => quote!(u32),
                33..=64 => quote!(u64),
                _ => panic!("Invalid bit_size {}", e.bit_size),
            };

            field_ty = util::relative_path(e_path, path);
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

        match off_in_reg {
            BitOffset::Regular(off_in_reg) => {
                let off_in_reg = off_in_reg as usize;
                if let Some(array) = &f.array {
                    let (len, offs_expr) = super::process_array(array);
                    if have_read {
                        items.extend(quote! {
                            #doc
                            #[inline(always)]
                            pub const fn #name(&self, n: usize) -> #field_ty{
                                assert!(n < #len);
                                let offs = #off_in_reg + #offs_expr;
                                let val = (self.0 >> offs) & #mask;
                                #from_bits
                            }
                        });
                    }
                    if have_write {
                        items.extend(quote! {
                            #doc
                            #[inline(always)]
                            pub fn #name_set(&mut self, n: usize, val: #field_ty) {
                                assert!(n < #len);
                                let offs = #off_in_reg + #offs_expr;
                                self.0 = (self.0 & !(#mask << offs)) | (((#to_bits) & #mask) << offs);
                            }
                        });
                    }
                } else {
                    if have_read {
                        items.extend(quote! {
                            #doc
                            #[inline(always)]
                            pub const fn #name(&self) -> #field_ty{
                                let val = (self.0 >> #off_in_reg) & #mask;
                                #from_bits
                            }
                        });
                    }
                    if have_write {
                        items.extend(quote! {
                            #doc
                            #[inline(always)]
                            pub fn #name_set(&mut self, val: #field_ty) {
                                self.0 = (self.0 & !(#mask << #off_in_reg)) | (((#to_bits) & #mask) << #off_in_reg);
                            }
                        });
                    }
                }
            }
            BitOffset::Cursed(ranges) => {
                // offset of "range"s inside register
                let mut off_in_reg: Vec<usize> = Vec::new();
                let mut mask: Vec<TokenStream> = Vec::new();
                // offset to shift "range" value to final value
                // preload first offset as 0,
                // since we order "range" from less to larger, first offset-in-value should always be 0.
                let mut off_in_val: Vec<usize> = vec![0];
                for (index, range) in ranges.iter().enumerate() {
                    off_in_reg.push(*range.start() as usize);
                    mask.push(util::hex(
                        1u64.wrapping_shl(range.end() - range.start() + 1)
                            .wrapping_sub(1),
                    ));

                    // prepare next "range" offset-in-value value
                    if index < ranges.len() - 1 {
                        off_in_val
                            .push(off_in_val[index] + ((range.end() - range.start()) as usize + 1))
                    }
                }

                if let Some(array) = &f.array {
                    let (len, offs_expr) = super::process_array(array);
                    items.extend(quote!(
                        #doc
                        #[inline(always)]
                        pub const fn #name(&self, n: usize) -> #field_ty{
                            assert!(n < #len);
                            let mut val = 0;
                            #(  let offs = #off_in_reg + #offs_expr;
                                val += (((self.0 >> offs) & #mask) << #off_in_val); )*;
                            #from_bits
                        }
                        #doc
                        #[inline(always)]
                        pub fn #name_set(&mut self, n: usize, val: #field_ty) {
                            assert!(n < #len);
                            #( let offs = #off_in_reg + #offs_expr;
                               self.0 = (self.0 & !(#mask << offs)) | (((#to_bits >> #off_in_val) & #mask) << offs); )*;
                        }
                    ));
                } else {
                    items.extend(quote!(
                        #doc
                        #[inline(always)]
                        pub const fn #name(&self) -> #field_ty{
                            let mut val = 0;
                            #( val += (((self.0 >> #off_in_reg) & #mask) << #off_in_val); )*;
                            #from_bits
                        }
                        #doc
                        #[inline(always)]
                        pub fn #name_set(&mut self, val: #field_ty) {
                           #( self.0 = (self.0 & !(#mask << #off_in_reg)) | (((#to_bits >> #off_in_val) & #mask) << #off_in_reg); )*;
                        }
                    ))
                }
            }
        };
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
