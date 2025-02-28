use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use syn::{
    parse_macro_input, spanned::Spanned, Data, DataEnum, DataStruct, DeriveInput, Fields, Meta,
};

#[proc_macro_derive(Desc, attributes(desc))]
pub fn derive_desc(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        attrs,
        vis: _,
        ident,
        generics,
        data: _,
    } = parse_macro_input!(input as DeriveInput);

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let desc = attrs.iter().find_map(|attr| {
        if attr.path().is_ident("desc") {
            Some(match &attr.meta {
                Meta::List(meta) => {
                    quote_spanned! { meta.span() => compile_error!("expected `#[desc = \"...\"]`") }
                }
                Meta::Path(meta) => {
                    quote_spanned! { meta.span() => compile_error!("expected `#[desc = \"...\"]`") }
                }
                Meta::NameValue(meta) => meta.value.to_token_stream(),
            })
        } else {
            None
        }
    });

    quote! {
        impl #impl_generics ::oath_parser::Desc for #ident #ty_generics #where_clause {
            fn desc() -> &'static str {
                #desc
            }
        }
    }
    .into()
}

#[proc_macro_derive(Parse, attributes(try_parse))]
pub fn derive_parse(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        attrs: _,
        vis: _,
        ident,
        generics,
        data,
    } = parse_macro_input!(input as DeriveInput);

    let parse_output = match data {
        Data::Enum(data) => parse_enum(data),
        Data::Struct(data) => parse_struct(data),
        Data::Union(_) => quote! { compile_error!("`Parse` cannot be derived for unions") },
    };

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl # impl_generics ::oath_parser::Parse for #ident #ty_generics #where_clause {
            fn parse(
                parser: &mut ::oath_parser::Parser<impl Iterator<Item = ::oath_tokenizer::TokenTree>>,
                context: ::oath_context::ContextHandle,
            ) -> Self {
                #parse_output
            }
        }
    }
    .into()
}

#[proc_macro_derive(TryParse, attributes(try_parse))]
pub fn derive_try_parse(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        attrs: _,
        vis: _,
        ident,
        generics,
        data,
    } = parse_macro_input!(input as DeriveInput);

    let parse_output = match data {
        Data::Enum(data) => try_parse_enum(data),
        Data::Struct(data) => parse_struct(data),
        Data::Union(_) => quote! { compile_error!("`Parse` cannot be derived for unions") },
    };

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl # impl_generics ::oath_parser::TryParse for #ident #ty_generics #where_clause {
            fn try_parse(
                parser: &mut ::oath_parser::Parser<impl Iterator<Item = ::oath_tokenizer::TokenTree>>,
                context: ::oath_context::ContextHandle,
            ) -> Result<Self, ()> {
                #[allow(unreachable_code)]
                Ok({ #parse_output })
            }
        }
    }
    .into()
}

#[proc_macro_derive(Peek, attributes(dont_peek))]
pub fn derive_peek(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        attrs: _,
        vis: _,
        ident,
        generics,
        data,
    } = parse_macro_input!(input as DeriveInput);

    let parse_output = match data {
        Data::Enum(data) => peek_enum(data),
        Data::Struct(data) => peek_struct(data),
        Data::Union(_) => quote! { compile_error!("`Peek` cannot be derived for unions") },
    };

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl # impl_generics ::oath_parser::Peek for #ident #ty_generics #where_clause {
            fn peek(
                parser: &mut ::oath_parser::Parser<impl Iterator<Item = ::oath_tokenizer::TokenTree>>,
                context: ::oath_context::ContextHandle,
            ) -> bool {
                #parse_output
            }
        }
    }
    .into()
}

#[proc_macro_derive(PeekOk)]
pub fn derive_peek_ok(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        attrs: _,
        vis: _,
        ident,
        generics,
        data: _,
    } = parse_macro_input!(input as DeriveInput);

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl # impl_generics ::oath_parser::PeekOk for #ident #ty_generics #where_clause {}
    }
    .into()
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

fn try_parse_enum(data: DataEnum) -> TokenStream {
    let peek_variants = data.variants.into_iter().map(|variant| {
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
                            return Ok(Self::#variant_ident(value));
                        }
                    }
                }
            }
        }
    });

    quote! {
        #(#peek_variants)*
        return Err(());
    }
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
                }
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
