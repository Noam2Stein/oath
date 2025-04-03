use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{Attribute, DataEnum, DataStruct, DeriveInput, Error, Meta, spanned::Spanned};

use crate::*;

pub fn impl_option_parse(input: &DeriveInput) -> TokenStream {
    impl_trait(
        input,
        "OptionParse",
        true,
        true,
        false,
        [
            impl_trait_fn(
                quote! { fn option_parse(parser: &mut ::oath_parser::Parser<impl ::oath_parser::ParserIterator>) -> Option<Self> },
                data_split(
                    &input.data,
                    &input.attrs,
                    option_parse_struct,
                    option_parse_enum,
                    |_, _| unreachable!(),
                ),
            ),
            {
                let desc = eval_desc(input);

                quote! { fn desc() -> &'static str { #desc } }
            },
            impl_trait_fn(
                quote! { fn detect(parser: &::oath_parser::Parser<impl ::oath_parser::ParserIterator>) -> bool },
                data_split(
                    &input.data,
                    &input.attrs,
                    detect_struct,
                    detect_enum,
                    |_, _| unreachable!(),
                ),
            ),
        ],
    )
}

fn option_parse_struct(data: &DataStruct, _attrs: &Vec<Attribute>) -> TokenStream {
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

fn detect_struct(data: &DataStruct, _attrs: &Vec<Attribute>) -> TokenStream {
    detect_fields(&data.fields, Span::call_site())
}

fn option_parse_enum(data: &DataEnum, _attrs: &Vec<Attribute>) -> TokenStream {
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

fn detect_enum(data: &DataEnum, _attrs: &Vec<Attribute>) -> TokenStream {
    let detect_variants = data
        .variants
        .iter()
        .map(|variant| detect_fields(&variant.fields, variant.span()));

    quote! {
        { #((#detect_variants))||* }
    }
}

fn eval_desc(input: &DeriveInput) -> TokenStream {
    let desc_attr = {
        let mut desc_attrs = input
            .attrs
            .iter()
            .filter(|attr| attr.path().is_ident("desc"));

        let desc_attr = match desc_attrs.next() {
            Some(desc_attr) => desc_attr,
            None => {
                return Error::new(Span::call_site(), "expected `#[desc = \"...\"]`")
                    .to_compile_error();
            }
        };

        if let Some(second_desc_attr) = desc_attrs.next() {
            return Error::new(second_desc_attr.span(), "multiple `desc` attributes")
                .to_compile_error();
        }

        desc_attr
    };

    match &desc_attr.meta {
        Meta::List(_) | Meta::Path(_) => {
            Error::new(Span::call_site(), "expected `#[desc = \"...\"]`").to_compile_error()
        }
        Meta::NameValue(meta) => meta.value.to_token_stream(),
    }
}
