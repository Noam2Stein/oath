use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{spanned::Spanned, DataEnum, DataStruct, DeriveInput, Error};

use crate::impl_util::{
    detect_fields, has_attrib, impl_parser_trait, parse_detected_fields, parse_fields,
};

pub fn impl_parse(input: &DeriveInput) -> TokenStream {
    impl_parser_trait(
        input,
        "oath_parser",
        "Parse",
        "parse",
        quote! {
            parser: &mut ::oath_parser::Parser<impl ::oath_parser::ParserIterator>,
        },
        quote! {
            Self
        },
        Some(parse_struct),
        Some(parse_enum),
        None,
    )
}

fn parse_struct(data: &DataStruct) -> TokenStream {
    parse_fields(&data.fields, Span::call_site())
}

fn parse_enum(data: &DataEnum) -> TokenStream {
    let (fallback_variant, non_fallback_variants, multiple_fallback_errors) = {
        let mut variant_iter = data.variants.iter();

        let mut non_fallback_variants = Vec::new();
        let mut fallback_variant = None;
        let mut multiple_fallback_errors = Vec::new();

        while let Some(variant) = variant_iter.next() {
            if has_attrib(&variant.attrs, "fallback") {
                fallback_variant = Some(variant);
            } else {
                non_fallback_variants.push(variant);
            }
        }

        while let Some(variant) = variant_iter.next() {
            if has_attrib(&variant.attrs, "fallback") {
                multiple_fallback_errors.push(
                    Error::new(variant.span(), "only one fallback variant is allowed")
                        .to_compile_error(),
                );
            } else {
                non_fallback_variants.push(variant);
            }
        }

        if let Some(fallback_variant) = fallback_variant {
            (
                fallback_variant,
                non_fallback_variants,
                multiple_fallback_errors,
            )
        } else {
            return Error::new(Span::call_site(), "expected a fallback variant").to_compile_error();
        }
    };

    let variant_ifs = non_fallback_variants.iter().map(|variant| {
        let detect_variant = detect_fields(&variant.fields, variant.span());
        let parse_detected_variant = parse_detected_fields(&variant.fields, variant.span());

        let variant_ident = &variant.ident;

        quote! {
            if #detect_variant {
                return Self::#variant_ident #parse_detected_variant;
            }
        }
    });

    let fallback = {
        let variant_ident = &fallback_variant.ident;
        let parse_variant_fields = parse_fields(&fallback_variant.fields, fallback_variant.span());

        quote! {
            Self::#variant_ident #parse_variant_fields
        }
    };

    quote! {
        #(#multiple_fallback_errors;)*

        #(#variant_ifs)*

        #fallback
    }
}
