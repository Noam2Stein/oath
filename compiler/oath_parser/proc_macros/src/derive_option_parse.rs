use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{Attribute, DataEnum, DataStruct, DeriveInput, Error, Meta, spanned::Spanned};

use crate::*;

pub fn derive_option_parse(input: &DeriveInput) -> TokenStream {
    let impl_option_parse = impl_trait(
        input,
        "OptionParse",
        [
            impl_trait_fn(
                quote! { fn option_parse(parser: &mut ::oath_parser::Parser<impl ::oath_tokenizer::TokenSource>, output: &mut Option<Self>) -> ParseExit },
                data_split(&input.data, &input.attrs, option_parse_struct, option_parse_enum),
            ),
            impl_trait_fn(
                quote! { fn detect(parser: &::oath_parser::Parser<impl ::oath_tokenizer::TokenSource>) -> Detection },
                data_split(&input.data, &input.attrs, detect_struct, detect_enum),
            ),
        ],
    );

    let impl_desc = impl_trait(
        input,
        "ParseDesc",
        [{
            let desc = eval_desc(input);

            quote! { fn desc() -> &'static str { #desc } }
        }],
    );

    quote! {
        #impl_option_parse
        #impl_desc
    }
}

fn option_parse_struct(data: &DataStruct, _attrs: &[Attribute]) -> TokenStream {
    option_parse_fields(&data.fields, Span::call_site(), &quote! { Self }, &quote! { output })
}

fn detect_struct(data: &DataStruct, _attrs: &[Attribute]) -> TokenStream {
    detect_fields(&data.fields, Span::call_site())
}

fn option_parse_enum(data: &DataEnum, _attrs: &[Attribute]) -> TokenStream {
    let variant_ifs = data.variants.iter().map(|variant| {
        let variant_ident = &variant.ident;

        let option_parse_fields = option_parse_fields(
            &variant.fields,
            variant.ident.span(),
            &quote! { Self::#variant_ident },
            &quote! { &mut variant_output },
        );

        quote! {
            {
                let mut variant_output = None;

                let variant_option_parse_exit = #option_parse_fields;

                if let Some(variant_output) = variant_output {
                    *output = Some(variant_output);

                    return variant_option_parse_exit;
                }
            }
        }
    });

    quote! {
        #(#variant_ifs)*

        ::oath_parser::ParseExit::Complete
    }
}

fn detect_enum(data: &DataEnum, _attrs: &[Attribute]) -> TokenStream {
    let detect_variants = data
        .variants
        .iter()
        .map(|variant| detect_fields(&variant.fields, variant.span()));

    quote! {
        'detect_enum: { #((#detect_variants))|* }
    }
}

fn eval_desc(input: &DeriveInput) -> TokenStream {
    let desc_attr = {
        let mut desc_attrs = input.attrs.iter().filter(|attr| attr.path().is_ident("desc"));

        let desc_attr = match desc_attrs.next() {
            Some(desc_attr) => desc_attr,
            None => {
                return Error::new(Span::call_site(), "expected `#[desc = \"...\"]`").to_compile_error();
            }
        };

        if let Some(second_desc_attr) = desc_attrs.next() {
            return Error::new(second_desc_attr.span(), "multiple `desc` attributes").to_compile_error();
        }

        desc_attr
    };

    match &desc_attr.meta {
        Meta::List(_) | Meta::Path(_) => Error::new(Span::call_site(), "expected `#[desc = \"...\"]`").to_compile_error(),
        Meta::NameValue(meta) => meta.value.to_token_stream(),
    }
}
