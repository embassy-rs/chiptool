use std::collections::HashMap;

use anyhow::Result;
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::quote;

use crate::ir::*;
use crate::util::{self, ToSanitizedUpperCase};

pub fn render(_opts: &super::Options, ir: &IR, d: &Device, path: &str) -> Result<TokenStream> {
    let mut out = TokenStream::new();
    let span = Span::call_site();

    let mut interrupts_sorted = d.interrupts.clone();
    interrupts_sorted.sort_by_key(|i| i.value);

    let mut interrupts = TokenStream::new();
    let mut peripherals = TokenStream::new();
    let mut vectors = TokenStream::new();
    let mut names = vec![];
    let mut addrs: HashMap<_, Vec<(_, _)>> = HashMap::new();

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
        let name = Ident::new(&p.name, span);
        let address = util::hex(p.base_address as u64);
        let doc = util::doc(&p.description);

        if let Some(block_name) = &p.block {
            let _b = ir.blocks.get(block_name);

            addrs
                .entry((block_name, path))
                .or_default()
                .push((p.base_address, p.name.clone()));

            let path = util::relative_path(block_name, path);

            peripherals.extend(quote! {
                #doc
                pub const #name: #path = #path(#address as u32 as _);
            });
        } else {
            peripherals.extend(quote! {
                #doc
                pub const #name: *mut () = #address as u32 as _;
            });
        }
    }

    for ((block_name, path), addrs) in addrs {
        let unknown = Literal::string(&format!("Unknown {}", block_name));

        let path = util::relative_path(block_name, path);

        let addrs = addrs
            .iter()
            .map(|(addr, name)| quote! { #addr => f.write_str(#name), });

        peripherals.extend(quote! {
            impl core::fmt::Display for #path {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
                    match self.0 as _ {
                        #(
                            #addrs
                        )*
                        _ => f.write_str(#unknown),
                    }
                }
            }
        });
    }

    let n = util::unsuffixed(pos as u64);
    out.extend(quote!(
        #[derive(Copy, Clone, Debug, PartialEq, Eq)]
        pub enum Interrupt {
            #interrupts
        }

        unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
            #[inline(always)]
            fn number(self) -> u16 {
                self as u16
            }
        }

        #[cfg(feature = "rt")]
        mod _vectors {
            extern "C" {
                #(fn #names();)*
            }

            pub union Vector {
                _handler: unsafe extern "C" fn(),
                _reserved: u32,
            }

            #[link_section = ".vector_table.interrupts"]
            #[no_mangle]
            pub static __INTERRUPTS: [Vector; #n] = [
                #vectors
            ];
        }

        #peripherals
    ));

    /*
    if let Some(cpu) = d.cpu.as_ref() {
        let bits = util::unsuffixed(u64::from(cpu.nvic_priority_bits));

        out.extend(quote! {
            ///Number available in the NVIC for configuring priority
            pub const NVIC_PRIO_BITS: u8 = #bits;
        });
    }
     */

    Ok(out)
}
