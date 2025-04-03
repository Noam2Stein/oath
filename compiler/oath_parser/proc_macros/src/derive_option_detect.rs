use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, DataEnum, DataStruct, DeriveInput, spanned::Spanned};

use crate::*;

pub fn impl_option_detect(input: &DeriveInput) -> TokenStream {
    impl_trait(
        input,
        "OptionDetect",
        true,
        true,
        false,
        [impl_trait_fn(
            quote! { fn option_detect(parser: &::oath_parser::Parser<impl ::oath_parser::ParserIterator>) -> bool },
            data_split(
                &input.data,
                &input.attrs,
                option_detect_struct,
                option_detect_enum,
                |_, _| unreachable!(),
            ),
        )],
    )
}

fn option_detect_struct(data: &DataStruct, _attrs: &Vec<Attribute>) -> TokenStream {
    option_detect_fields(&data.fields)
}

fn option_detect_enum(data: &DataEnum, _attrs: &Vec<Attribute>) -> TokenStream {
    let (fallback_variant, non_fallback_variants) = match try_fallback_variant(data) {
        Ok(ok) => ok,
        Err(error) => return error,
    };

    let variant_ifs = non_fallback_variants.iter().map(|variant| {
        let detect_variant = detect_fields(&variant.fields, variant.span());

        quote! {
            if #detect_variant {
                return true;
            }
        }
    });

    let fallback = option_detect_fields(&fallback_variant.fields);

    quote! {
        #(#variant_ifs)*

        #fallback
    }
}
