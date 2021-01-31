use std::borrow::Cow;
use std::cmp::Ordering;
use std::collections::HashMap;

use proc_macro2::TokenStream;
use proc_macro2::{Ident, Punct, Spacing, Span};
use quote::{quote, ToTokens};

use crate::util::{self, ToSanitizedSnakeCase, ToSanitizedUpperCase, BITS_PER_BYTE};
use anyhow::{anyhow, bail, Context, Result};

use crate::ir::*;

pub fn render(d: &Device, i: &PeripheralInstance) -> Result<TokenStream> {
    let mut out = TokenStream::new();

    let p = d.peripherals.get(i.peripheral);
    let b = d.blocks.get(p.block);

    let span = Span::call_site();
    let name = Ident::new(&i.path.name, span);
    let path = util::relative_path(&b.path, &i.path);
    let address = util::hex(i.base_address as u64);
    let description = util::respace(i.description.as_ref().unwrap_or(&i.path.name));

    out.extend(quote! {
        pub const #name: #path = #path::from_ptr(#address as u32 as _);
    });

    Ok(out)
}
