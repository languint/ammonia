use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitInt, Token, parse_macro_input};

struct Input {
    name: Ident,
    _comma: Token![,],
    count: LitInt,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
            _comma: input.parse()?,
            count: input.parse()?,
        })
    }
}

/// Generate error codes from `E0001`, to `E{count}`
///
/// # Panics
/// This macro panics if the `count` parameter is not valid base 10
#[proc_macro]
pub fn gen_error_codes(input: TokenStream) -> TokenStream {
    let Input { name, count, .. } = parse_macro_input!(input);

    let count_value: u32 = count
        .base10_parse()
        .expect("Failed to parse count as base10!");

    let digits = count.to_string().len();

    let variants = (1..=count_value).map(|n| {
        let s = format!("E{n:0digits$}");
        let ident = Ident::new(&s, proc_macro2::Span::call_site());
        quote! { #ident }
    });

    let expanded = quote! {
        #[derive(Debug, PartialEq, Eq)]
        pub enum #name {
            #(#variants),*
        }
    };

    expanded.into()
}
