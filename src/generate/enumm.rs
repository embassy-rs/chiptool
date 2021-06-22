use anyhow::Result;
use proc_macro2::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;

use crate::ir::*;
use crate::util;

pub fn render(_opts: &super::Options, _ir: &IR, e: &Enum, path: &str) -> Result<TokenStream> {
    let span = Span::call_site();
    let mut items = TokenStream::new();

    let ty = match e.bit_size {
        1..=8 => quote!(u8),
        9..=16 => quote!(u16),
        17..=32 => quote!(u32),
        33..=64 => quote!(u64),
        _ => panic!("Invalid bit_size {}", e.bit_size),
    };

    for f in &e.variants {
        let name = Ident::new(&f.name, span);
        let value = util::hex(f.value);
        let doc = util::doc(&f.description);
        items.extend(quote!(
            #doc
            pub const #name: Self = Self(#value);
        ));
    }

    let (_, name) = super::split_path(path);
    let name = Ident::new(name, span);
    let doc = util::doc(&e.description);

    let out = quote! {
        #doc
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct #name (pub #ty);

        impl #name {
            #items
        }
    };

    Ok(out)
}
