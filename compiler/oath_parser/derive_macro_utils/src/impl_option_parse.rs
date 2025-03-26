use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{DataEnum, DataStruct, Field, Fields, spanned::Spanned};

use crate::impl_parser_trait;

pub fn impl_option_parse(input: TokenStream) -> TokenStream {
    impl_parser_trait(
        input.into(),
        "oath_parser",
        "OptionParse",
        "option_parse",
        quote! {
            parser: &mut ::oath_parser::Parser<impl ::oath_parser::ParserIterator>,
        },
        quote! {
            Option<Self>
        },
        Some(parse_struct),
        Some(option_parse_enum),
        None,
    )
}

fn option_parse_fields(path: TokenStream, fields: &Fields, span: Span) -> TokenStream {
    if fields.len() == 0 {
        return quote_spanned! {
            span =>
            compile_error!("expected fields")
        };
    }

    let first_field = fields.iter().next().unwrap();
    let first_field_type = &first_field.ty;

    let parse_fields = fields.iter().skip(1).map(parse_field);

    let parse_some = match fields {
        Fields::Named(_) => {
            let first_field_ident = first_field.ident.as_ref().unwrap();

            quote! {
                #path {
                    #first_field_ident: first_field,
                    #(#parse_fields,)*
                }
            }
        }
        Fields::Unit => unreachable!(),
        Fields::Unnamed(_) => {
            quote! {
                #path(first_field, #(#parse_fields), *)
            }
        }
    };

    quote! {
        (if let Some(first_field) = <Option<#first_field_type> as ::oath_parser::Parse>::parse(parser) {
            Some(#parse_some)
        } else {
            None
        })
    }
}

fn parse_field(field: &Field) -> TokenStream {
    let ident_prefix = field.ident.as_ref().map(|ident| quote! { #ident: });
    let ty = &field.ty;

    quote_spanned! {
        ty.span() =>
        #ident_prefix <#ty as ::oath_parser::Parse>::parse(parser)
    }
}

fn parse_struct(data: DataStruct) -> TokenStream {
    option_parse_fields(quote! { Self }, &data.fields, Span::call_site())
}

fn option_parse_enum(data: DataEnum) -> TokenStream {
    if data.variants.len() == 0 {
        return quote! {
            None
        };
    };

    let variants = data.variants.iter().filter(|variant| {
        !variant
            .attrs
            .iter()
            .any(|attr| attr.path().is_ident("error_fallback") || attr.path().is_ident("fallback"))
    });

    let variant_ifs = variants.zip(0..).map(|(variant, variant_index)| {
        let option_else = (variant_index > 0).then(|| quote! { else });

        let variant_ident = &variant.ident;
        let parse_variant = option_parse_fields(
            quote! { Self::#variant_ident },
            &variant.fields,
            variant.span(),
        );

        quote! {
            #option_else if let Some(variant) = #parse_variant {
                Some(variant)
            }
        }
    });

    quote! {
        #(#variant_ifs)*
        else {
            None
        }
    }
}
