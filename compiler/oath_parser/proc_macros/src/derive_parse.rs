use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Attribute, DataEnum, DataStruct, DeriveInput, spanned::Spanned};

use crate::*;

pub fn derive_parse(input: &DeriveInput) -> TokenStream {
    impl_trait(
        input,
        "Parse",
        true,
        true,
        false,
        [
            impl_trait_fn(
                quote! { fn parse(parser: &mut ::oath_parser::Parser<impl ::oath_parser::ParserIterator>) -> Self },
                data_split(
                    &input.data,
                    &input.attrs,
                    parse_struct,
                    parse_enum,
                    |_, _| unreachable!(),
                ),
            ),
            impl_trait_fn(
                quote! { fn parse_error() -> Self },
                data_split(
                    &input.data,
                    &input.attrs,
                    struct_parse_error,
                    enum_parse_error,
                    |_, _| unreachable!(),
                ),
            ),
        ],
    )
}

fn parse_struct(data: &DataStruct, _attrs: &Vec<Attribute>) -> TokenStream {
    let parse_fields = parse_fields(&data.fields, Span::call_site());

    quote! {
        Self #parse_fields
    }
}

fn struct_parse_error(data: &DataStruct, _attrs: &Vec<Attribute>) -> TokenStream {
    let fields = fields_parse_error(&data.fields, Span::call_site());

    quote! {
        Self #fields
    }
}

fn parse_enum(data: &DataEnum, _attrs: &Vec<Attribute>) -> TokenStream {
    let (fallback_variant, non_fallback_variants) = match try_fallback_variant(data) {
        Ok(ok) => ok,
        Err(error) => return error,
    };

    let variant_ifs = non_fallback_variants.iter().map(|variant| {
        let detect_variant = condition_parse_fields_if(&variant.fields, variant.span());
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
        #(#variant_ifs)*

        #fallback
    }
}

fn enum_parse_error(data: &DataEnum, _attrs: &Vec<Attribute>) -> TokenStream {
    let (fallback_variant, _non_fallback_variants) = match try_fallback_variant(data) {
        Ok(ok) => ok,
        Err(error) => return error,
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
        #fallback
    }
}
