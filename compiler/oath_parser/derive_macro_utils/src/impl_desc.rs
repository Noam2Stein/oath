use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{DeriveInput, Error, Meta, parse2, spanned::Spanned};

pub fn impl_desc(input: TokenStream) -> TokenStream {
    let input = match parse2::<DeriveInput>(input) {
        Ok(input) => input,
        Err(error) => return error.into_compile_error(),
    };

    let DeriveInput {
        attrs: _,
        vis: _,
        ident,
        generics,
        data: _,
    } = &input;

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let desc = eval_desc(&input);

    quote! {
        impl #impl_generics ::oath_parser::ParseDesc for #ident #ty_generics #where_clause {
            fn desc() -> &'static str {
                #desc
            }
        }
    }
    .into()
}

fn eval_desc(
    DeriveInput {
        attrs,
        vis: _,
        ident: _,
        generics: _,
        data: _,
    }: &DeriveInput,
) -> TokenStream {
    let desc_attr = {
        let mut desc_attrs = attrs
            .into_iter()
            .filter(|attr| attr.path().is_ident("desc"));
        let desc_attr = match desc_attrs.next() {
            Some(desc_attr) => desc_attr,
            None => {
                return Error::new(Span::call_site(), "expected `#[desc = \"...\"]`")
                    .to_compile_error();
            }
        };

        if let Some(second_desc_attr) = desc_attrs.next() {
            return Error::new(second_desc_attr.span(), "multiple `desc` attributes")
                .to_compile_error();
        }

        desc_attr
    };

    match &desc_attr.meta {
        Meta::List(_) => {
            Error::new(Span::call_site(), "expected `#[desc = \"...\"]`").to_compile_error()
        }
        Meta::Path(_) => {
            Error::new(Span::call_site(), "expected `#[desc = \"...\"]`").to_compile_error()
        }
        Meta::NameValue(meta) => meta.value.to_token_stream(),
    }
}
