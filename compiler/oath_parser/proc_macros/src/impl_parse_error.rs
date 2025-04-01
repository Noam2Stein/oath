use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{spanned::Spanned, DataEnum, DataStruct, DeriveInput, Error};

use crate::impl_util::{fields_parse_error, has_attr, impl_parser_trait};

pub fn impl_parse_error(input: &DeriveInput) -> TokenStream {
    impl_parser_trait(
        input,
        "oath_parser",
        "ParseError",
        "parse_error",
        quote! {},
        quote! {
            Self
        },
        Some(struct_parse_error),
        Some(enum_parse_error),
        None,
    )
}

fn struct_parse_error(data: &DataStruct) -> TokenStream {
    let fields = fields_parse_error(&data.fields, Span::call_site());

    quote! {
        Self #fields
    }
}

fn enum_parse_error(data: &DataEnum) -> TokenStream {
    let (fallback_variant, multiple_fallback_errors) = {
        let mut variant_iter = data.variants.iter();

        let mut fallback_variant = None;
        let mut multiple_fallback_errors = Vec::new();

        while let Some(variant) = variant_iter.next() {
            if has_attr(&variant.attrs, "fallback") {
                fallback_variant = Some(variant);
                break;
            }
        }

        while let Some(variant) = variant_iter.next() {
            if has_attr(&variant.attrs, "fallback") {
                multiple_fallback_errors.push(
                    Error::new(variant.span(), "only one fallback variant is allowed")
                        .to_compile_error(),
                );
            }
        }

        if let Some(fallback_variant) = fallback_variant {
            (fallback_variant, multiple_fallback_errors)
        } else {
            return Error::new(Span::call_site(), "expected a fallback variant").to_compile_error();
        }
    };

    let fallback = {
        let variant_ident = &fallback_variant.ident;
        let fields_parse_error =
            fields_parse_error(&fallback_variant.fields, fallback_variant.span());

        quote! {
            Self::#variant_ident #fields_parse_error
        }
    };

    quote! {
        #(#multiple_fallback_errors;)*

        #fallback
    }
}
