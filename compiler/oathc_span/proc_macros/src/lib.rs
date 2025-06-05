use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote, quote_spanned};
use syn::{
    Attribute, Data, DataEnum, DataStruct, DeriveInput, Fields, GenericParam, LitInt, parse_macro_input, parse_quote,
    spanned::Spanned,
};

mod fields;
use fields::*;

#[proc_macro_derive(Spanned, attributes(span, not_spanned))]
pub fn spanned_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        attrs,
        vis: _,
        ident,
        mut generics,
        data,
    } = parse_macro_input!(input as DeriveInput);

    for param in &mut generics.params {
        if let GenericParam::Type(type_param) = param {
            type_param.bounds.push(parse_quote!(::oath_src::Spanned));
        }
    }

    let output = match data {
        Data::Struct(data) => struct_span(data, &attrs),
        Data::Enum(data) => enum_span(data),
        Data::Union(_) => quote! { compile_error!("`Spanned` cannot be derived for unions") },
    };

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics Spanned for #ident #ty_generics #where_clause {
            fn span(&self) -> Span {
                #output
            }
        }
        impl #impl_generics OptionSpanned for #ident #ty_generics #where_clause {
            fn option_span(&self) -> Option<Span> {
                Some(<Self as Spanned>::span(self))
            }
        }
    }
    .into()
}

#[proc_macro_derive(OptionSpanned, attributes(span, not_spanned))]
pub fn option_spanned_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        attrs,
        vis: _,
        ident,
        mut generics,
        data,
    } = parse_macro_input!(input as DeriveInput);

    for param in &mut generics.params {
        if let GenericParam::Type(type_param) = param {
            type_param.bounds.push(parse_quote!(::oath_src::OptionSpanned));
        }
    }

    let output = match data {
        Data::Struct(data) => struct_option_span(data, &attrs),
        Data::Enum(data) => enum_option_span(data),
        Data::Union(_) => quote! { compile_error!("`OptionSpanned` cannot be derived for unions") },
    };

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics OptionSpanned for #ident #ty_generics #where_clause {
            fn option_span(&self) -> Option<Span> {
                Option::<Span>::from({ #output })
            }
        }
    }
    .into()
}

fn struct_span(data: DataStruct, attrs: &[Attribute]) -> TokenStream {
    fields_span(&data.fields, attrs, |field_ident, field_index| {
        if field_ident.is_some() {
            quote_spanned! { field_ident.span() => self.#field_ident }
        } else {
            let field_ident = LitInt::new(&field_index.to_string(), field_ident.span()).to_token_stream();

            quote_spanned! { field_ident.span() => self.#field_ident }
        }
    })
}

fn struct_option_span(data: DataStruct, attrs: &[Attribute]) -> TokenStream {
    fields_option_span(&data.fields, attrs, |field_ident, field_index| {
        if field_ident.is_some() {
            quote_spanned! { field_ident.span() => self.#field_ident }
        } else {
            let field_ident = LitInt::new(&field_index.to_string(), field_ident.span()).to_token_stream();

            quote_spanned! { field_ident.span() => self.#field_ident }
        }
    })
}

fn enum_span(data: DataEnum) -> TokenStream {
    let match_variants = data.variants.into_iter().map(|variant| {
        let variant_ident = &variant.ident;

        let original_field_idents = variant.fields.iter().map(|field| &field.ident);

        let field_idents = (0..variant.fields.len()).map(|field_index| format_ident!("field_{field_index}"));

        let span = fields_span(&variant.fields, &variant.attrs, |_, field_index| {
            format_ident!("field_{field_index}").to_token_stream()
        });

        match variant.fields {
            Fields::Named(_) => quote_spanned! {
                variant.span() =>

                Self::#variant_ident { #(#original_field_idents: #field_idents), * } => #span,
            },
            Fields::Unnamed(_) => quote_spanned! {
                variant.span() =>

                Self::#variant_ident(#(#field_idents), *) => #span,
            },
            Fields::Unit => quote_spanned! {
                variant.span() =>

                Self::#variant_ident => #span,
            },
        }
    });

    quote! {
        match self {
            #(#match_variants)*
        }
    }
}

fn enum_option_span(data: DataEnum) -> TokenStream {
    let match_variants = data.variants.into_iter().map(|variant| {
        let variant_ident = &variant.ident;

        let original_field_idents = variant.fields.iter().map(|field| &field.ident);

        let field_idents = (0..variant.fields.len()).map(|field_index| format_ident!("field_{field_index}"));

        let span = fields_option_span(&variant.fields, &variant.attrs, |_, field_index| {
            format_ident!("field_{field_index}").to_token_stream()
        });

        match variant.fields {
            Fields::Named(_) => quote_spanned! {
                variant.span() =>

                Self::#variant_ident { #(#original_field_idents: #field_idents), * } => #span,
            },
            Fields::Unnamed(_) => quote_spanned! {
                variant.span() =>

                Self::#variant_ident(#(#field_idents), *) => #span,
            },
            Fields::Unit => quote_spanned! {
                variant.span() =>

                Self::#variant_ident => #span,
            },
        }
    });

    quote! {
        match self {
            #(#match_variants)*
        }
    }
}
