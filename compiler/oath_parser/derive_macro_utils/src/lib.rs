use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Data, DataEnum, DataStruct, DataUnion, DeriveInput, Ident, Meta, Token, parse::Parser, parse2,
    punctuated::Punctuated,
};

mod impl_desc;
mod impl_parse;
mod impl_peek;
mod impl_try_parse;
pub use impl_desc::*;
pub use impl_parse::*;
pub use impl_peek::*;
pub use impl_try_parse::*;

pub fn impl_parser_trait(
    input: TokenStream,
    crate_ident: &'static str,
    trait_ident: &'static str,
    fn_ident: &'static str,
    params: TokenStream,
    output: TokenStream,
    eval_struct: Option<fn(DataStruct) -> TokenStream>,
    eval_enum: Option<fn(DataEnum) -> TokenStream>,
    eval_union: Option<fn(DataUnion) -> TokenStream>,
) -> TokenStream {
    let DeriveInput {
        attrs: _,
        vis: _,
        ident,
        generics,
        data,
    } = match parse2(input) {
        Ok(input) => input,
        Err(error) => return error.into_compile_error(),
    };

    let eval = match data {
        Data::Struct(data) => match eval_struct {
            Some(fn_) => fn_(data),
            None => quote! { compile_error!("`{}` cannot be derived for structs", #trait_ident) },
        },
        Data::Enum(data) => match eval_enum {
            Some(fn_) => fn_(data),
            None => quote! { compile_error!("`{}` cannot be derived for enums", #trait_ident) },
        },
        Data::Union(data) => match eval_union {
            Some(fn_) => fn_(data),
            None => quote! { compile_error!("`{}` cannot be derived for unions", #trait_ident) },
        },
    };

    let crate_ident = format_ident!("{crate_ident}");
    let trait_ident = format_ident!("{trait_ident}");
    let fn_ident = format_ident!("{fn_ident}");

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics ::#crate_ident::#trait_ident for #ident #ty_generics #where_clause {
            fn #fn_ident(#params) -> #output {
                #eval
            }
        }
    }
    .into()
}

pub fn derives(input: TokenStream) -> Vec<String> {
    let DeriveInput {
        attrs,
        vis: _,
        ident: _,
        generics: _,
        data: _,
    } = parse2(input).unwrap();

    attrs
        .iter()
        .map(|attr| {
            if attr.path().is_ident("derive") {
                match &attr.meta {
                    Meta::List(meta) => Punctuated::<Ident, Token![,]>::parse_terminated
                        .parse2(meta.tokens.clone())
                        .unwrap()
                        .into_iter()
                        .map(|ident| ident.to_string())
                        .collect(),
                    _ => Vec::new(),
                }
            } else {
                Vec::new()
            }
        })
        .flatten()
        .collect()
}
