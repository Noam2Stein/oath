use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataEnum, DataStruct, Fields, spanned::Spanned};

use crate::{impl_parser_trait, peek_fields};

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

pub fn parse_fields(path: TokenStream, fields: Fields) -> TokenStream {
    match fields {
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
                #path {
                    #(#fields,)*
                }
            }
        }
        Fields::Unit => quote! {
            #path
        },
        Fields::Unnamed(fields) => {
            let field_types = fields.unnamed.iter().map(|field| &field.ty);

            quote! {
                #path(#(<#field_types as oath_parser::Parse>::parse(parser, context)), *)
            }
        }
    }
}

fn parse_struct(data: DataStruct) -> TokenStream {
    parse_fields(quote! { Self }, data.fields)
}

pub fn parse_enum(data: DataEnum) -> TokenStream {
    if data.variants.len() == 0 {
        return quote! {
            compile_error!("`Parse` cannot be derived for empty enums")
        };
    };

    let fallback_variant = data.variants.iter().find(|variant| {
        variant
            .attrs
            .iter()
            .any(|attr| attr.path().is_ident("fallback"))
    });

    let variants = {
        let mut variants = data.variants.iter().collect::<Vec<_>>();
        if let Some(fallback_variant) = fallback_variant {
            variants.retain(|variant| variant.ident != fallback_variant.ident);
        }

        variants
    };

    let variant_ifs = variants.iter().zip(0..).map(|(variant, variant_index)| {
        let option_else = if variant_index > 0 {
            Some(quote! { else })
        } else {
            None
        };

        let peek_variant = peek_fields(variant.fields.clone(), &variant.attrs, variant.span());
        let variant_ident = &variant.ident;
        let parse_variant = parse_fields(quote! { Self::#variant_ident }, variant.fields.clone());

        quote! {
            #option_else if #peek_variant {
                #parse_variant
            }
        }
    });

    let fallback = if let Some(fallback_variant) = fallback_variant {
        let variant_ident = &fallback_variant.ident;
        parse_fields(
            quote! { Self::#variant_ident },
            fallback_variant.fields.clone(),
        )
    } else {
        quote! {
            context.push_error(::oath_context::Error::new(format!("expected {}", <Self as ::oath_parser::Desc>::desc()), parser.next_span()));
            return Err(())
        }
    };

    if variants.len() > 0 {
        quote! {
            #(#variant_ifs)*
            else {
                #fallback
            }
        }
    } else {
        fallback
    }
}
