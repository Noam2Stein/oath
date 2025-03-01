use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{DataEnum, Fields, spanned::Spanned};

use crate::impl_parser_trait;

pub fn impl_try_parse(input: TokenStream) -> TokenStream {
    impl_parser_trait(
        input.into(),
        "oath_parser",
        "TryParse",
        "try_parse",
        quote! {
            parser: &mut ::oath_parser::Parser<impl Iterator<Item = ::oath_tokenizer::TokenTree>>,
            context: ::oath_context::ContextHandle,
        },
        quote! {
            ::oath_parser::PResult<Self>
        },
        None,
        Some(try_parse_enum),
        None,
    )
}

fn try_parse_enum(data: DataEnum) -> TokenStream {
    let peek_variants = data.variants.into_iter().map(|variant| {
        let variant_ident = &variant.ident;

        match &variant.fields {
            Fields::Named(_) => {
                quote_spanned! { variant.span() => compile_error!("expected a single unnamed field"); }
            },
            Fields::Unit => {
                quote_spanned! { variant.span() => compile_error!("expected a single unnamed field"); }
            },
            Fields::Unnamed(fields) => {
                if fields.unnamed.len() != 1 {
                    quote_spanned! { variant.span() => compile_error!("expected a single unnamed field"); }
                } else {
                    quote_spanned! {
                        variant.span() =>

                        if let Some(value) = parser.parse(context) {
                            return Ok(Self::#variant_ident(value));
                        }
                    }
                }
            }
        }
    });

    quote! {
        #(#peek_variants)*
        return Err(());
    }
}
