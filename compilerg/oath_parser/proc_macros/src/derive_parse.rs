use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Attribute, DataEnum, DataStruct, DeriveInput, Error, Variant, spanned::Spanned};

use crate::*;

pub fn derive_parse(input: &DeriveInput) -> TokenStream {
    let impl_option_parse = impl_trait(
        input,
        "OptionParse",
        [
            impl_trait_fn(
                quote! { fn option_parse(parser: &mut ::oath_parser::Parser<impl ::oath_tokenizer::Tokenizer>, output: &mut Option<Self>) -> ParseExit },
                data_split(&input.data, &input.attrs, option_parse_struct, option_parse_enum),
            ),
            impl_trait_fn(
                quote! { fn detect(parser: &::oath_parser::Parser<impl ::oath_tokenizer::Tokenizer>) -> Detection },
                data_split(&input.data, &input.attrs, detect_struct, detect_enum),
            ),
        ],
    );

    let impl_parse = impl_trait(
        input,
        "Parse",
        [
            impl_trait_fn(
                quote! { fn parse(parser: &mut ::oath_parser::Parser<impl ::oath_tokenizer::Tokenizer>, output: &mut Self) -> ::oath_parser::ParseExit },
                data_split(&input.data, &input.attrs, parse_struct, parse_enum),
            ),
            impl_trait_fn(
                quote! { fn parse_error() -> Self },
                data_split(&input.data, &input.attrs, struct_parse_error, enum_parse_error),
            ),
        ],
    );

    quote! {
        #impl_option_parse
        #impl_parse
    }
}

fn parse_struct(data: &DataStruct, attrs: &[Attribute]) -> TokenStream {
    parse_fields(&data.fields, Span::call_site(), attrs, &quote! { Self }, &quote! { output })
}

fn struct_parse_error(data: &DataStruct, _attrs: &[Attribute]) -> TokenStream {
    fields_parse_error(&data.fields, Span::call_site(), &quote! { Self })
}

fn option_parse_struct(data: &DataStruct, attrs: &[Attribute]) -> TokenStream {
    option_parse_fields(&data.fields, Span::call_site(), attrs, &quote! { Self }, &quote! { output })
}

fn detect_struct(data: &DataStruct, attrs: &[Attribute]) -> TokenStream {
    detect_fields(&data.fields, Span::call_site(), attrs)
}

fn parse_enum(data: &DataEnum, _attrs: &[Attribute]) -> TokenStream {
    let (fallback_variant, non_fallback_variants) = match fallback_split_variants(data) {
        Ok(ok) => ok,
        Err(error) => return error,
    };

    let variant_ifs = non_fallback_variants.iter().map(|variant| {
        let variant_ident = &variant.ident;

        let option_parse_fields = option_parse_fields(
            &variant.fields,
            variant.ident.span(),
            &variant.attrs,
            &quote! { Self::#variant_ident },
            &quote! { &mut variant_output },
        );

        quote! {
            {
                let mut variant_output = None;

                let variant_option_parse_exit = #option_parse_fields;

                if let Some(variant_output) = variant_output {
                    *output = variant_output;

                    return variant_option_parse_exit;
                }
            }
        }
    });

    let parse_fallback = {
        let variant_ident = &fallback_variant.ident;

        parse_fields(
            &fallback_variant.fields,
            fallback_variant.ident.span(),
            &fallback_variant.attrs,
            &quote! { Self::#variant_ident },
            &quote! { output },
        )
    };

    quote! {
        #(#variant_ifs)*

        #parse_fallback
    }
}

fn enum_parse_error(data: &DataEnum, _attrs: &[Attribute]) -> TokenStream {
    let (fallback_variant, _non_fallback_variants) = match fallback_split_variants(data) {
        Ok(ok) => ok,
        Err(error) => return error,
    };

    let fallback_variant_ident = &fallback_variant.ident;

    fields_parse_error(
        &fallback_variant.fields,
        fallback_variant.span(),
        &quote! { Self::#fallback_variant_ident },
    )
}

fn option_parse_enum(data: &DataEnum, _attrs: &[Attribute]) -> TokenStream {
    let (fallback_variant, non_fallback_variants) = match fallback_split_variants(data) {
        Ok(ok) => ok,
        Err(error) => return error,
    };

    let variant_ifs = non_fallback_variants.iter().map(|variant| {
        let variant_ident = &variant.ident;

        let option_parse_fields = option_parse_fields(
            &variant.fields,
            variant.ident.span(),
            &variant.attrs,
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

    let option_parse_fallback = {
        let variant_ident = &fallback_variant.ident;

        option_parse_fields(
            &fallback_variant.fields,
            fallback_variant.ident.span(),
            &fallback_variant.attrs,
            &quote! { Self::#variant_ident },
            &quote! { output },
        )
    };

    quote! {
        #(#variant_ifs)*

        #option_parse_fallback
    }
}

fn detect_enum(data: &DataEnum, _attrs: &[Attribute]) -> TokenStream {
    let (_fallback_variant, non_fallback_variants) = match fallback_split_variants(data) {
        Ok(ok) => ok,
        Err(error) => return error,
    };

    let detect_variants = non_fallback_variants
        .iter()
        .map(|variant| detect_fields(&variant.fields, variant.span(), &variant.attrs));

    quote! {
        'detect_enum: { Detection::EmptyDetected #(| #detect_variants)* }
    }
}

fn fallback_split_variants(data: &DataEnum) -> Result<(&Variant, Vec<&Variant>), TokenStream> {
    let mut fallback_variant = None;
    let mut non_fallback_variants = Vec::with_capacity(data.variants.len());

    for variant in &data.variants {
        if variant.attrs.iter().any(|attr| attr.path().is_ident("fallback")) && fallback_variant.is_none() {
            fallback_variant = Some(variant);
        } else {
            non_fallback_variants.push(variant);
        }
    }

    match fallback_variant {
        Some(fallback_variant) => Ok((fallback_variant, non_fallback_variants)),
        None => Err(Error::new(data.enum_token.span, "expected a variant marked `#[fallback]`").into_compile_error()),
    }
}
