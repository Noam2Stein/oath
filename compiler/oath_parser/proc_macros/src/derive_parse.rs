use proc_macro2::{Literal, Span, TokenStream};
use quote::quote;
use syn::{Attribute, DataEnum, DataStruct, DeriveInput, spanned::Spanned};

use crate::*;

pub fn derive_parse(input: &DeriveInput) -> TokenStream {
    impl_trait(
        input,
        "Parse",
        [
            impl_trait_fn(
                quote! { fn parse(parser: &mut ::oath_parser::Parser, output: &mut Self) -> ::oath_parser::ParseExit },
                data_split(&input.data, &input.attrs, parse_struct, parse_enum),
            ),
            impl_trait_fn(
                quote! { fn parse_error() -> Self },
                data_split(&input.data, &input.attrs, struct_parse_error, enum_parse_error),
            ),
        ],
    )
}

fn parse_struct(data: &DataStruct, _attrs: &Vec<Attribute>) -> TokenStream {
    let field_idents = data.fields.members();
    let field_types = data.fields.iter().map(|field| &field.ty);

    quote! {
        #(
            match <#field_types as ::oath_parser::Parse>::parse(parser, &mut output.#field_idents) {
                ::oath_parser::ParseExit::Complete => {},
                ::oath_parser::ParseExit::Cut => return ::oath_parser::ParseExit::Cut,
            }
        )*

        ::oath_parser::ParseExit::Complete
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
        let option_parse_fields = option_parse_fields(&variant.fields);
        let variant_ident = &variant.ident;
        let members = variant.fields.members();
        let field_indicies = (0..).map(Literal::usize_unsuffixed);

        quote! {
            if let Some(fields) = #option_parse_fields {
                *output = Self::#variant_ident {
                    #(#members: fields.0.#field_indicies,)*
                };

                return fields.1;
            }
        }
    });

    let _fallback = {
        let variant_ident = &fallback_variant.ident;
        let parse_variant_fields = parse_fields(&fallback_variant.fields, fallback_variant.span());

        quote! {
            Self::#variant_ident #parse_variant_fields
        }
    };

    quote! {
        #(#variant_ifs)*

        ::oath_parser::ParseExit::Complete
    }
}

fn enum_parse_error(data: &DataEnum, _attrs: &Vec<Attribute>) -> TokenStream {
    let (fallback_variant, _non_fallback_variants) = match try_fallback_variant(data) {
        Ok(ok) => ok,
        Err(error) => return error,
    };

    let fallback = {
        let variant_ident = &fallback_variant.ident;
        let fields_parse_error = fields_parse_error(&fallback_variant.fields, fallback_variant.span());

        quote! {
            Self::#variant_ident #fields_parse_error
        }
    };

    quote! {
        #fallback
    }
}
