use proc_macro2::{Span, TokenStream, TokenTree};
use quote::{ToTokens, quote, quote_spanned};
use syn::{
    Attribute, Data, DataEnum, DataStruct, DeriveInput, Error, Fields, Meta, parse2,
    spanned::Spanned,
};

use crate::impl_parser_trait;

pub fn impl_detect(input: TokenStream) -> TokenStream {
    impl_parser_trait(
        input.into(),
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

pub fn detect_fields(fields: Fields, attrs: &Vec<Attribute>, span: Span) -> TokenStream {
    let detect_type = if let Some(attr) = attrs.iter().find(|attr| attr.path().is_ident("detect")) {
        match &attr.meta {
            Meta::List(meta) => {
                let mut tokens = meta.tokens.clone().into_iter().peekable();

                if let Some(TokenTree::Punct(first_token)) = tokens.peek() {
                    if first_token.as_char() == '|' {
                        let tokens = tokens.collect::<TokenStream>();
                        return quote! {
                            (#tokens)()
                        };
                    }
                }

                tokens.collect()
            }
            _ => {
                return Error::new(attr.span(), "expected `#[detect(~type~ / ~closure~)]`")
                    .to_compile_error();
            }
        }
    } else {
        fields.into_iter().next().map_or_else(
            || quote_spanned! { span => () },
            |first_field| first_field.ty.to_token_stream(),
        )
    };

    quote! {
        <#detect_type as ::oath_parser::Detect>::detect(parser)
    }
}

fn detect_struct(data: DataStruct) -> TokenStream {
    let first_field_type = match data.fields.into_iter().find(|item| {
        !item
            .attrs
            .iter()
            .any(|attrib| attrib.path().is_ident("dont_detect"))
    }) {
        Some(some) => some.ty,
        None => return quote! { compile_error!("`Detect` cannot be derived for empty structs") },
    };

    quote! {
        <#first_field_type as ::oath_parser::Detect>::detect(parser)
    }
}

pub fn detect_enum(data: DataEnum) -> TokenStream {
    if let Some(attr) = data.variants.iter().find_map(|variant| {
        variant
            .attrs
            .iter()
            .find(|attr| attr.path().is_ident("fallback"))
    }) {
        return Error::new(attr.span(), "`fallback` is not allowed in detectable enums")
            .to_compile_error();
    };

    let detect_variants = data
        .variants
        .iter()
        .filter(|variant| {
            !variant
                .attrs
                .iter()
                .any(|attr| attr.path().is_ident("error_fallback"))
        })
        .map(|variant| detect_fields(variant.fields.clone(), &variant.attrs, variant.span()));

    quote! {
        #((#detect_variants))||*
    }
}

pub fn is_detect_parse(input: TokenStream) -> bool {
    let input = match parse2::<DeriveInput>(input) {
        Ok(ok) => ok,
        Err(_) => return false,
    };

    match input.data {
        Data::Enum(data) => data.variants.iter().any(|variant| {
            variant
                .attrs
                .iter()
                .any(|attr| attr.path().is_ident("error_fallback"))
        }),
        _ => false,
    }
}
