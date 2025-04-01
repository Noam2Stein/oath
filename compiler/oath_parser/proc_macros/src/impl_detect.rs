use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, Attribute, DataEnum, DataStruct, DeriveInput};

use crate::impl_util::{detect_fields, impl_parser_trait};

pub fn impl_detect(input: &DeriveInput) -> TokenStream {
    impl_parser_trait(
        input,
        "oath_parser",
        "Detect",
        "detect",
        quote! {
            parser: &::oath_parser::Parser<impl ::oath_parser::ParserIterator>,
        },
        quote! {
            bool
        },
        Some(detect_struct),
        Some(detect_enum),
        None,
    )
}

fn detect_struct(data: &DataStruct, attrs: &Vec<Attribute>) -> TokenStream {
    detect_fields(&data.fields, attrs, Span::call_site())
}

fn detect_enum(data: &DataEnum, _attrs: &Vec<Attribute>) -> TokenStream {
    let fallback_errors = data
    .variants
    .iter()
    .map(|variant| {
        variant.attrs.iter().filter(|attr| {
            attr.path().is_ident("fallback") || attr.path().is_ident("error_fallback")
        })
    })
    .flatten()
    .map(|attr: &Attribute| {
        quote_spanned! {
            attr.span() =>
            compile_error!("`Detect` cannot be derived for enums with `fallback` / `error_fallback`")
        }
    });

    let detect_variants = data
        .variants
        .iter()
        .map(|variant| detect_fields(&variant.fields, &variant.attrs, variant.span()));

    quote! {
        {
            #(#fallback_errors;)*
            #((#detect_variants))||*
        }
    }
}
