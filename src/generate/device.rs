use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use util::ToSanitizedSnakeCase;

use crate::util::{self, ToSanitizedUpperCase};
use anyhow::Result;

use crate::ir::*;

pub fn render(ir: &IR, d: &Device) -> Result<TokenStream> {
    let mut out = TokenStream::new();
    let span = Span::call_site();

    let mut interrupts_sorted = d.interrupts.clone();
    interrupts_sorted.sort_by_key(|i| i.value);

    let mut interrupts = TokenStream::new();
    let mut peripherals = TokenStream::new();
    let mut vectors = TokenStream::new();
    let mut names = vec![];

    let mut pos = 0;
    for i in &interrupts_sorted {
        while pos < i.value {
            vectors.extend(quote!(Vector { _reserved: 0 },));
            pos += 1;
        }
        pos += 1;

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
        vectors.extend(quote!(Vector { _handler: #name_uc },));
        names.push(name_uc);
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

    let n = util::unsuffixed(pos as u64);
    out.extend(quote!(
        #[derive(Copy, Clone, Debug, PartialEq, Eq)]
        pub enum Interrupt {
            #interrupts
        }

        #[cfg(feature = "rt")]
        extern "C" {
            #(fn #names();)*
        }

        #[doc(hidden)]
        pub union Vector {
            _handler: unsafe extern "C" fn(),
            _reserved: u32,
        }

        #[cfg(feature = "rt")]
        #[doc(hidden)]
        #[link_section = ".vector_table.interrupts"]
        #[no_mangle]
        pub static __INTERRUPTS: [Vector; #n] = [
            #vectors
        ];


        unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
            #[inline(always)]
            fn number(self) -> u16 {
                self as u16
            }
        }

        #peripherals
    ));

    if let Some(cpu) = d.cpu.as_ref() {
        let bits = util::unsuffixed(u64::from(cpu.nvic_priority_bits));

        out.extend(quote! {
            ///Number available in the NVIC for configuring priority
            pub const NVIC_PRIO_BITS: u8 = #bits;
        });
    }

    Ok(out)
}
