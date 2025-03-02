use proc_macro2::TokenStream;
use quote::quote;
use syn::DataEnum;

use crate::{impl_parser_trait, parse_enum};

pub fn impl_try_parse(input: TokenStream) -> TokenStream {
    impl_parser_trait(
        input.into(),
        "oath_parser",
        "TryParse",
        "try_parse",
        quote! {
            parser: &mut ::oath_parser::Parser<impl Iterator<Item = ::oath_tokenizer::TokenTree>>,
            context: ::oath_context::ContextHandle,
        },
        quote! {
            ::oath_parser::PResult<Self>
        },
        None,
        Some(try_parse_enum),
        None,
    )
}

fn try_parse_enum(data: DataEnum) -> TokenStream {
    let parse = parse_enum(data);

    quote! {
        Ok({
            #parse
        })
    }
}
