use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{DataEnum, DataStruct, Fields, spanned::Spanned};

use crate::impl_parser_trait;

pub fn impl_parse(input: TokenStream) -> TokenStream {
    impl_parser_trait(
        input.into(),
        "oath_parser",
        "Parse",
        "parse",
        quote! {
            parser: &mut ::oath_parser::Parser<impl Iterator<Item = ::oath_tokenizer::TokenTree>>,
            context: ::oath_context::ContextHandle,
        },
        quote! {
            Self
        },
        Some(parse_struct),
        Some(parse_enum),
        None,
    )
}

fn parse_struct(data: DataStruct) -> TokenStream {
    match data.fields {
        Fields::Named(fields) => {
            let fields = fields.named.into_iter().map(|field| {
                let ident = field.ident.unwrap();
                let ty = field.ty;

                let try_parse_attr = field
                    .attrs
                    .iter()
                    .find(|attr| attr.path().is_ident("try_parse"));

                let parse_attr = field
                    .attrs
                    .iter()
                    .find(|attr| attr.path().is_ident("parse"));

                let parse_expr = if let Some(parse_attr) = parse_attr {
                    let parse_fn = &parse_attr.meta.require_list().unwrap().tokens;
                    quote! {
                        (#parse_fn)()
                    }
                } else if try_parse_attr.is_some() {
                    quote! {
                        <#ty as ::oath_parser::TryParse>::try_parse(parser, context)?
                    }
                } else {
                    quote! {
                        <#ty as ::oath_parser::Parse>::parse(parser, context)
                    }
                };

                if try_parse_attr.is_some() {
                    quote! {
                        #ident: #parse_expr?
                    }
                } else {
                    quote! {
                        #ident: #parse_expr
                    }
                }
            });

            quote! {
                Self {
                    #(#fields,)*
                }
            }
        }
        Fields::Unit => quote! {
            Self
        },
        Fields::Unnamed(fields) => {
            let field_types = fields.unnamed.iter().map(|field| &field.ty);

            quote! {
                Self(#(<#field_types as oath_parser::Parse>::parse(parser, diagnostics)), *)
            }
        }
    }
}

fn parse_enum(data: DataEnum) -> TokenStream {
    if data.variants.len() == 0 {
        return quote! {
            compile_error!("`Parse` cannot be derived for empty enums")
        };
    };

    let (last_variant, non_last_variants) = {
        let mut variants = data.variants.into_iter().collect::<Vec<_>>();
        (variants.pop().unwrap(), variants)
    };

    let peek_variants = non_last_variants.into_iter().map(|variant| {
        let variant_ident = &variant.ident;

        match &variant.fields {
            Fields::Named(_) => {
                quote_spanned! { variant.span() => compile_error!("expected a single unnamed field"); }
            },
            Fields::Unit => {
                quote_spanned! { variant.span() => compile_error!("expected a single unnamed field"); }
            },
            Fields::Unnamed(fields) => {
                if fields.unnamed.len() != 1 {
                    quote_spanned! { variant.span() => compile_error!("expected a single unnamed field"); }
                } else {
                    quote_spanned! {
                        variant.span() =>

                        if let Some(value) = parser.parse(context) {
                            return Self::#variant_ident(value);
                        }
                    }
                }
            }
        }
    });

    let last_variant_ident = &last_variant.ident;
    let parse_last_variant = match &last_variant.fields {
        Fields::Named(fields) => {
            if fields.named.len() == 0 {
                quote_spanned! {
                    last_variant.span() =>

                    Self::#last_variant_ident {}
                }
            } else {
                quote_spanned! { last_variant.span() => compile_error!("expected a single unnamed field") }
            }
        }
        Fields::Unit => {
            quote_spanned! {
                last_variant.span() =>

                Self::#last_variant_ident
            }
        }
        Fields::Unnamed(fields) => {
            if fields.unnamed.len() != 1 {
                quote_spanned! { last_variant.span() => compile_error!("expected a single unnamed field") }
            } else {
                quote_spanned! {
                    last_variant.span() =>

                    Self::#last_variant_ident(parser.parse(context))
                }
            }
        }
    };

    quote! {
        #(#peek_variants)*
        #parse_last_variant
    }
}
