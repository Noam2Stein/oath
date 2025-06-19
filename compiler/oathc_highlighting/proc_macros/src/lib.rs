use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote, quote_spanned};
use syn::{Data, DataEnum, DataStruct, DeriveInput, Fields, parse_macro_input, spanned::Spanned};

mod fields;
use fields::*;

#[proc_macro_derive(Highlightable, attributes(highlightable))]
pub fn highlightable_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        attrs: _,
        vis: _,
        ident,
        generics,
        data,
    } = parse_macro_input!(input as DeriveInput);

    let output = match data {
        Data::Struct(data) => highlight_struct(data),
        Data::Enum(data) => highlight_enum(data),
        Data::Union(_) => quote! { compile_error!("`Highlightable` cannot be derived for unions") },
    };

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics Highlightable for #ident #ty_generics #where_clause {
            fn highlight(&self, color: HighlightColor, h: &mut Vec<Highlight>) {
                #output
            }
        }
    }
    .into()
}

fn highlight_struct(data: DataStruct) -> TokenStream {
    highlight_fields(&data.fields, |member, _| {
        quote_spanned! {
            member.span() =>

            &self.#member
        }
    })
}

fn highlight_enum(data: DataEnum) -> TokenStream {
    let match_variants = data.variants.into_iter().map(|variant| {
        let variant_ident = &variant.ident;

        let original_field_idents = variant.fields.iter().map(|field| &field.ident);

        let field_idents = (0..variant.fields.len()).map(|field_index| format_ident!("field_{field_index}"));

        let output = highlight_fields(&variant.fields, |_, idx| format_ident!("field_{idx}").into_token_stream());

        match variant.fields {
            Fields::Named(_) => quote_spanned! {
                variant.span() =>

                Self::#variant_ident { #(#original_field_idents: #field_idents), * } => #output,
            },
            Fields::Unnamed(_) => quote_spanned! {
                variant.span() =>

                Self::#variant_ident(#(#field_idents), *) => #output,
            },
            Fields::Unit => quote_spanned! {
                variant.span() =>

                Self::#variant_ident => #output,
            },
        }
    });

    quote! {
        match self {
            #(#match_variants)*
        }
    }
}
