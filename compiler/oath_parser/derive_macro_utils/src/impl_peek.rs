use proc_macro2::{Span, TokenStream, TokenTree};
use quote::{ToTokens, quote, quote_spanned};
use syn::{Attribute, DataEnum, DataStruct, Error, Fields, Meta, spanned::Spanned};

use crate::impl_parser_trait;

pub fn impl_peek(input: TokenStream) -> TokenStream {
    impl_parser_trait(
        input.into(),
        "oath_parser",
        "Peek",
        "peek",
        quote! {
            parser: &mut ::oath_parser::Parser<impl Iterator<Item = ::oath_tokenizer::TokenTree>>,
            context: ::oath_context::ContextHandle,
        },
        quote! {
            bool
        },
        Some(peek_struct),
        Some(peek_enum),
        None,
    )
}

pub fn peek_fields(fields: Fields, attrs: &Vec<Attribute>, span: Span) -> TokenStream {
    let peek_type = if let Some(attr) = attrs.iter().find(|attr| attr.path().is_ident("peek")) {
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
                return Error::new(attr.span(), "expected `#[peek(~type~ / ~closure~)]`")
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
        <#peek_type as ::oath_parser::Peek>::peek(parser, context)
    }
}

fn peek_struct(data: DataStruct) -> TokenStream {
    let first_field_type = match data.fields.into_iter().find(|item| {
        !item
            .attrs
            .iter()
            .any(|attrib| attrib.path().is_ident("dont_peek"))
    }) {
        Some(some) => some.ty,
        None => return quote! { compile_error!("`Peek` cannot be derived for empty structs") },
    };

    quote! {
        <#first_field_type as ::oath_parser::Peek>::peek(parser, context)
    }
}

pub fn peek_enum(data: DataEnum) -> TokenStream {
    if let Some(attr) = data.variants.iter().find_map(|variant| {
        variant
            .attrs
            .iter()
            .find(|attr| attr.path().is_ident("fallback"))
    }) {
        return Error::new(attr.span(), "`fallback` is not allowed in peekable enums")
            .to_compile_error();
    };

    let peek_variants = data
        .variants
        .iter()
        .map(|variant| peek_fields(variant.fields.clone(), &variant.attrs, variant.span()));

    quote! {
        #((#peek_variants))||*
    }
}
