use proc_macro2::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, DataEnum, DataStruct, DeriveInput, Error};

use crate::impl_util::{detect_fields, has_attr, impl_parser_trait, option_detect_fields};

pub fn impl_option_detect(input: &DeriveInput) -> TokenStream {
    impl_parser_trait(
        input,
        "oath_parser",
        "OptionDetect",
        "option_detect",
        quote! {
            parser: &::oath_parser::Parser<impl ::oath_parser::ParserIterator>,
        },
        quote! {
            bool
        },
        Some(option_detect_struct),
        Some(option_detect_enum),
        None,
    )
}

fn option_detect_struct(data: &DataStruct) -> TokenStream {
    option_detect_fields(&data.fields)
}

fn option_detect_enum(data: &DataEnum) -> TokenStream {
    let detect_variants = data.variants.iter().map(|variant| {
        if has_attr(&variant.attrs, "fallback") {
            if variant.fields.len() > 0 {
                Error::new(
                    variant.span(),
                    "`OptionDetect` requires fallbacks to have no fields",
                )
                .to_compile_error()
            } else {
                quote! { false }
            }
        } else {
            detect_fields(&variant.fields, variant.span())
        }
    });

    quote! {
        (#((#detect_variants))||*)
    }
}
