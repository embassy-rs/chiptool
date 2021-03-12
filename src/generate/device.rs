use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use util::ToSanitizedSnakeCase;

use crate::util::{self, ToSanitizedUpperCase};
use crate::Target;
use anyhow::Result;

use crate::ir::*;

pub fn render(ir: &IR, d: &Device) -> Result<TokenStream> {
    let mut out = TokenStream::new();
    let span = Span::call_site();

    let mut interrupts = TokenStream::new();
    let mut peripherals = TokenStream::new();

    for i in &d.interrupts {
        let name_uc = Ident::new(&i.name.to_sanitized_upper_case(), span);
        let description = format!(
            "{} - {}",
            i.value,
            i.description
                .as_ref()
                .map(|s| util::respace(s))
                .as_ref()
                .map(|s| util::escape_brackets(s))
                .unwrap_or_else(|| i.name.clone())
        );

        let value = util::unsuffixed(i.value as u64);

        interrupts.extend(quote! {
            #[doc = #description]
            #name_uc = #value,
        });
    }

    for p in &d.peripherals {
        let b = ir.blocks.get(p.block);
        let name = Ident::new(&p.name, span);
        let path = util::relative_path(&b.path, &d.path);
        let address = util::hex(p.base_address as u64);
        let doc = util::doc(&p.description);

        peripherals.extend(quote! {
            #doc
            pub const #name: #path = #path(#address as u32 as _);
        });
    }

    out.extend(quote!(
        pub enum Interrupt {
            #interrupts
        }

        #peripherals
    ));

    Ok(out)
}
