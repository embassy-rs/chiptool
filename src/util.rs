use anyhow::{anyhow, Result};
use inflections::Inflect;
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::{quote, ToTokens};
use std::str::FromStr;

pub const BITS_PER_BYTE: u32 = 8;

/// List of chars that some vendors use in their peripheral/field names but
/// that are not valid in Rust ident
const INVALID_CHARS: &[char] = &['(', ')', '[', ']', '/', ' ', '-'];

static KEYWORDS: &[&str] = &[
    "abstract", "as", "async", "await", "become", "box", "break", "const", "continue", "crate",
    "do", "dyn", "else", "enum", "extern", "false", "final", "fn", "for", "if", "impl", "in",
    "let", "loop", "macro", "match", "mod", "move", "mut", "override", "priv", "pub", "ref",
    "return", "self", "Self", "static", "struct", "super", "trait", "true", "try", "type",
    "typeof", "unsafe", "unsized", "use", "virtual", "where", "while", "yield",
];

/// Make `s` a valid identifier, making the minimal changes (no case changes)
fn sanitize_ident(s: String) -> String {
    let mut s = s.replace(INVALID_CHARS, "");
    if KEYWORDS.contains(&&*s) {
        s.push('_');
        s
    } else if s.starts_with(char::is_numeric) {
        format!("_{}", s)
    } else {
        s
    }
}

pub trait StringExt {
    fn to_sanitized_pascal_case(&self) -> String;
    fn to_sanitized_upper_case(&self) -> String;
    fn to_sanitized_snake_case(&self) -> String;
}

impl StringExt for str {
    fn to_sanitized_snake_case(&self) -> String {
        sanitize_ident(self.to_snake_case())
    }

    fn to_sanitized_upper_case(&self) -> String {
        sanitize_ident(self.to_upper_case())
    }

    fn to_sanitized_pascal_case(&self) -> String {
        sanitize_ident(self.to_pascal_case())
    }
}

pub fn respace(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn escape_brackets(s: &str) -> String {
    s.split('[')
        .fold("".to_string(), |acc, x| {
            if acc.is_empty() {
                x.to_string()
            } else if acc.ends_with('\\') {
                acc + "[" + x
            } else {
                acc + "\\[" + x
            }
        })
        .split(']')
        .fold("".to_string(), |acc, x| {
            if acc.is_empty() {
                x.to_string()
            } else if acc.ends_with('\\') {
                acc + "]" + x
            } else {
                acc + "\\]" + x
            }
        })
}

pub fn replace_suffix(name: &str, suffix: &str) -> String {
    if name.contains("[%s]") {
        name.replace("[%s]", suffix)
    } else {
        name.replace("%s", suffix)
    }
}

pub fn hex_str(n: u64) -> String {
    let (h4, h3, h2, h1) = (
        (n >> 48) & 0xffff,
        (n >> 32) & 0xffff,
        (n >> 16) & 0xffff,
        n & 0xffff,
    );
    if h4 != 0 {
        format!("0x{:04x}_{:04x}_{:04x}_{:04x}", h4, h3, h2, h1)
    } else if h3 != 0 {
        format!("0x{:04x}_{:04x}_{:04x}", h3, h2, h1)
    } else if h2 != 0 {
        format!("0x{:04x}_{:04x}", h2, h1)
    } else if h1 & 0xff00 != 0 {
        format!("0x{:04x}", h1)
    } else if h1 != 0 {
        format!("0x{:02x}", h1 & 0xff)
    } else {
        "0x0".to_string()
    }
}

/// Turns `n` into an unsuffixed separated hex token
pub fn hex(n: u64) -> TokenStream {
    TokenStream::from_str(&hex_str(n)).unwrap()
}

/// Turns `n` into an unsuffixed separated hex token
pub fn hex_usize(n: u64) -> TokenStream {
    TokenStream::from_str(&format!("{}usize", hex_str(n))).unwrap()
}

/// Turns `n` into an unsuffixed token
pub fn unsuffixed(n: u64) -> TokenStream {
    Literal::u64_unsuffixed(n).into_token_stream()
}

pub fn unsuffixed_or_bool(n: u64, width: u32) -> TokenStream {
    if width == 1 {
        Ident::new(if n == 0 { "false" } else { "true" }, Span::call_site()).into_token_stream()
    } else {
        unsuffixed(n)
    }
}

pub trait U32Ext {
    fn to_ty(&self) -> Result<Ident>;
    fn to_ty_width(&self) -> Result<u32>;
}

impl U32Ext for u32 {
    fn to_ty(&self) -> Result<Ident> {
        Ok(Ident::new(
            match *self {
                1 => "bool",
                2..=8 => "u8",
                9..=16 => "u16",
                17..=32 => "u32",
                33..=64 => "u64",
                _ => {
                    return Err(anyhow!(
                        "can't convert {} bits into a Rust integral type",
                        *self
                    ))
                }
            },
            Span::call_site(),
        ))
    }

    fn to_ty_width(&self) -> Result<u32> {
        Ok(match *self {
            1 => 1,
            2..=8 => 8,
            9..=16 => 16,
            17..=32 => 32,
            33..=64 => 64,
            _ => {
                return Err(anyhow!(
                    "can't convert {} bits into a Rust integral type width",
                    *self
                ))
            }
        })
    }
}

pub fn build_rs() -> TokenStream {
    quote! {
        use std::env;
        use std::fs::File;
        use std::io::Write;
        use std::path::PathBuf;

        fn main() {
            if env::var_os("CARGO_FEATURE_RT").is_some() {
                // Put the linker script somewhere the linker can find it
                let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
                File::create(out.join("device.x"))
                    .unwrap()
                    .write_all(include_bytes!("device.x"))
                    .unwrap();
                println!("cargo:rustc-link-search={}", out.display());

                println!("cargo:rerun-if-changed=device.x");
            }

            println!("cargo:rerun-if-changed=build.rs");
        }
    }
}

/// Return a relative path to access a from b.
pub fn relative_path(a: &str, b: &str) -> TokenStream {
    let a: Vec<&str> = a.split("::").collect();
    let b: Vec<&str> = b.split("::").collect();

    let mut ma = &a[..a.len() - 1];
    let mut mb = &b[..b.len() - 1];
    while !ma.is_empty() && !mb.is_empty() && ma[0] == mb[0] {
        ma = &ma[1..];
        mb = &mb[1..];
    }

    let mut res = TokenStream::new();

    // for each item left in b, append a `super`
    for _ in mb {
        res.extend(quote!(super::));
    }

    // for each item in a, append it
    for ident in ma {
        let ident = Ident::new(ident, Span::call_site());
        res.extend(quote!(#ident::));
    }

    let ident = Ident::new(a[a.len() - 1], Span::call_site());
    res.extend(quote!(#ident));

    res
}

pub fn doc(doc: &Option<String>) -> TokenStream {
    if let Some(doc) = doc {
        let doc = doc.replace("\\n", "\n");
        let doc = respace(&doc);
        let doc = escape_brackets(&doc);
        quote!(#[doc=#doc])
    } else {
        quote!()
    }
}
