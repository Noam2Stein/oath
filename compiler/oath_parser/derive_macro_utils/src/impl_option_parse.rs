use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{DataEnum, DataStruct, Error, Field, Fields, parse_quote_spanned, spanned::Spanned};

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
        Some(parse_enum),
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
        if let Some(first_field) = <Option<#first_field_type> as ::oath_parser::Parse>::parse(parser) {
            Some(#parse_some)
        } else {
            None
        }
    }
}

fn parse_fields_option(path: TokenStream, fields: &Fields) -> TokenStream {
    let (first_field, other_fields) = {
        let mut other_fields = fields.iter();
        let first_field = if let Some(first_field) = other_fields.next() {
            first_field
        } else {
            return Error::new(fields.span(), "expected at least one field").to_compile_error();
        };
        (first_field, other_fields)
    };

    let first_field_parse_expr = {
        let first_field_ty = &first_field.ty;
        parse_field(&Field {
            attrs: first_field.attrs.clone(),
            colon_token: None,
            vis: first_field.vis.clone(),
            ident: None,
            mutability: first_field.mutability.clone(),
            ty: parse_quote_spanned! { first_field.ty.span() => Option<#first_field_ty> },
        })
    };

    let parse_expr = match fields {
        Fields::Unit => unreachable!(),
        Fields::Named(_) => {
            let parse_other_fields = other_fields.map(parse_field);

            let first_field_ident = &first_field.ident;
            quote! {
                #path {
                    #first_field_ident: first_field,
                    #(#parse_other_fields,)*
                }
            }
        }
        Fields::Unnamed(_) => {
            let parse_other_fields = other_fields.map(parse_field);
            quote! {
                #path(first_field, #(#parse_other_fields), *)
            }
        }
    };

    quote! {
        if let Some(first_field) = #first_field_parse_expr {
            Some({
                #parse_expr
            })
        } else {
            None
        }
    }
}

fn parse_field(field: &Field) -> TokenStream {
    let ident_prefix = field.ident.as_ref().map(|ident| quote! { #ident: });
    let ty = &field.ty;

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
        let try_error_fallback = try_parse_attr.map(|try_parse_attr| {
            Error::new(
                try_parse_attr.span(),
                "`try_parse` is not allowed with `parse`",
            )
            .to_compile_error()
        });

        quote! {
            {
                { #try_error_fallback }
                (#parse_fn)()
            }
        }
    } else if try_parse_attr.is_some() {
        quote_spanned! {
            ty.span() =>
            <#ty as ::oath_parser::TryParse>::try_parse(parser)?
        }
    } else {
        quote_spanned! {
            ty.span() =>
            <#ty as ::oath_parser::Parse>::parse(parser)
        }
    };

    quote! {
        #ident_prefix #parse_expr
    }
}

fn parse_struct(data: DataStruct) -> TokenStream {
    option_parse_fields(quote! { Self }, &data.fields, Span::call_site())
}

fn parse_enum(data: DataEnum) -> TokenStream {
    if data.variants.len() == 0 {
        return quote! {
            compile_error!("`OptionParse` cannot be derived for empty enums")
        };
    };

    let (fallback_variant, fallback_is_error, fallback_errors) = {
        let fallbacks = data
            .variants
            .iter()
            .filter_map(|variant| {
                if variant
                    .attrs
                    .iter()
                    .any(|attr| attr.path().is_ident("fallback"))
                {
                    Some((variant, false))
                } else if variant
                    .attrs
                    .iter()
                    .any(|attr| attr.path().is_ident("error_fallback"))
                {
                    Some((variant, true))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        if let Some((fallback, fallback_is_error)) = fallbacks.first() {
            (
                Some(*fallback),
                *fallback_is_error,
                fallbacks[1..]
                    .iter()
                    .map(|fallback| {
                        Error::new(fallback.0.span(), "only one variant can be a fallback")
                            .to_compile_error()
                    })
                    .collect(),
            )
        } else {
            (None, false, Vec::new())
        }
    };

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

        let variant_ident = &variant.ident;
        let parse_variant = parse_fields_option(quote! { Self::#variant_ident }, &variant.fields);

        quote! {
            #option_else if let Some(variant) = (#parse_variant) {
                variant
            }
        }
    });

    let fallback = if let Some(fallback_variant) = fallback_variant {
        let variant_ident = &fallback_variant.ident;
        if fallback_is_error {
            quote! {
                parser.context().push_error(::oath_context::Error::new(format!("expected {}", <Self as ::oath_parser::ParseDesc>::desc()), parser.peek_span()));

                let span = parser.peek_span();
                parser.next();
                Self::#variant_ident(span)
            }
        } else {
            option_parse_fields(
                quote! { Self::#variant_ident },
                &fallback_variant.fields,
                fallback_variant.span(),
            )
        }
    } else {
        quote! {
            parser.context().push_error(::oath_context::Error::new(format!("expected {}", <Self as ::oath_parser::ParseDesc>::desc()), parser.peek_span()));
            return Err(())
        }
    };

    if variants.len() > 0 {
        quote! {
            #(#variant_ifs)*
            else {
                #fallback
            }

            #(#fallback_errors;)*
        }
    } else {
        quote! {
            #fallback

            #(#fallback_errors;)*
        }
    }
}
