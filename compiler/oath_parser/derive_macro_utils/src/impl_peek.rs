use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataEnum, DataStruct};

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

fn peek_enum(data: DataEnum) -> TokenStream {
    let peek_variants = data.variants.into_iter().map(|variant| {
        if variant
            .attrs
            .iter()
            .any(|attr| attr.path().is_ident("dont_peek"))
        {
            return quote! {
                false
            };
        }

        let first_field_type = match variant.fields.iter().next() {
            Some(some) => &some.ty,
            None => {
                return quote! {
                    compile_error!("cannot peek an empty variant")
                };
            }
        };

        quote! {
            <#first_field_type as ::oath_parser::Peek>::peek(parser, context)
        }
    });

    quote! {
        #(
            if #peek_variants {
                return true;
            }
        )*
        false
    }
}
