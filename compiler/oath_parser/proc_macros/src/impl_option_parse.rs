use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{spanned::Spanned, DataEnum, DataStruct, DeriveInput};

use crate::impl_util::{condition_parse_fields_if, impl_parser_trait, parse_detected_fields};

pub fn impl_option_parse(input: &DeriveInput) -> TokenStream {
    impl_parser_trait(
        input,
        "oath_parser",
        "OptionParse",
        "option_parse",
        quote! {
            parser: &mut ::oath_parser::Parser<impl ::oath_parser::ParserIterator>,
        },
        quote! {
            Option<Self>
        },
        Some(option_parse_struct),
        Some(option_parse_enum),
        None,
    )
}

fn option_parse_struct(data: &DataStruct) -> TokenStream {
    let detect_fields = condition_parse_fields_if(&data.fields, Span::call_site());
    let parse_detected_fields = parse_detected_fields(&data.fields, Span::call_site());

    quote! {
        if #detect_fields {
            Some(Self #parse_detected_fields)
        } else {
            None
        }
    }
}

fn option_parse_enum(data: &DataEnum) -> TokenStream {
    let variant_ifs = data.variants.iter().map(|variant| {
        let detect_variant = condition_parse_fields_if(&variant.fields, variant.span());
        let parse_detected_variant = parse_detected_fields(&variant.fields, variant.span());

        let variant_ident = &variant.ident;

        quote! {
            if #detect_variant {
                return Some(Self::#variant_ident #parse_detected_variant);
            }
        }
    });

    quote! {
        #(#variant_ifs)*

        None
    }
}
