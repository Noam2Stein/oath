use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned, Data, DataEnum, DataStruct, DeriveInput, Fields};

#[proc_macro_derive(Spanned, attributes(span, spanned))]
pub fn derive_spanned(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        attrs: _,
        vis: _,
        ident,
        generics,
        data,
    } = parse_macro_input!(input as DeriveInput);

    let parse_output = match data {
        Data::Enum(data) => enum_span(data),
        Data::Struct(data) => struct_span(data),
        Data::Union(_) => quote! { compile_error!("`Peek` cannot be derived for unions") },
    };

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl # impl_generics ::oath_src::Spanned for #ident #ty_generics #where_clause {
            fn span(&self) -> Span {
                #parse_output
            }
        }
    }
    .into()
}

fn struct_span(data: DataStruct) -> TokenStream {
    let mut output = None;

    for field in data.fields {
        if let Some(attr) = field.attrs.iter().find(|attr| attr.path().is_ident("span")) {
            if output.is_some() {
                return quote_spanned! { attr.span() => compile_error!("multiple #[span] / #[spanned]") };
            } else {
                let field_ident = &field.ident;
                output = Some(quote_spanned! {
                    field.span() =>
                    self.#field_ident
                });
            }
        }

        if let Some(attr) = field
            .attrs
            .iter()
            .find(|attr| attr.path().is_ident("spanned"))
        {
            if output.is_some() {
                return quote_spanned! { attr.span() => compile_error!("multiple #[span] / #[spanned]") };
            } else {
                let field_ident = &field.ident;
                let field_ty = &field.ty;
                output = Some(quote_spanned! {
                    field.span() =>
                    <#field_ty as ::oath_src::Spanned>::span(self.#field_ident)
                });
            }
        }
    }

    if let Some(output) = output {
        output
    } else {
        quote! {
            compile_error!("expected a field to have #[span] / #[spanned]")
        }
    }
}

fn enum_span(data: DataEnum) -> TokenStream {
    let match_variants = data.variants.into_iter().map(|variant| {
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
                    let ty = &fields.unnamed.first().unwrap().ty;
                    quote_spanned! {
                        variant.span() =>

                        Self::#variant_ident(value) => <#ty as ::oath_src::Spanned>::span(value),
                    }
                }
            }
        }
    });

    quote! {
        match self {
            #(#match_variants)*
        }
    }
}
