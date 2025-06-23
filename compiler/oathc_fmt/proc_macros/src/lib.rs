use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote, quote_spanned};
use syn::{
    Attribute, Data, DataEnum, DataStruct, DeriveInput, Fields, GenericParam, LitInt, parse_macro_input, parse_quote,
    spanned::Spanned,
};

mod fields;
use fields::*;

#[proc_macro_derive(Format, attributes(format_as, dense_delims, spaced_delims))]
pub fn format_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        attrs,
        vis: _,
        ident,
        mut generics,
        data,
    } = parse_macro_input!(input as DeriveInput);

    for param in &mut generics.params {
        if let GenericParam::Type(type_param) = param {
            type_param.bounds.push(parse_quote!(Format));
        }
    }

    let output = match data {
        Data::Struct(data) => format_struct(data, &attrs),
        Data::Enum(data) => format_enum(data),
        Data::Union(_) => quote! { compile_error!("`Format` cannot be derived for unions") },
    };

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics Format for #ident #ty_generics #where_clause {
            fn format(&self, interner: &Interner) -> FormatTree {
                #output
            }
        }
    }
    .into()
}

fn format_struct(data: DataStruct, attrs: &[Attribute]) -> TokenStream {
    format_fields(&data.fields, attrs, data.struct_token.span(), |field_ident, field_index| {
        if field_ident.is_some() {
            quote_spanned! { field_ident.span() => &self.#field_ident }
        } else {
            let field_ident = LitInt::new(&field_index.to_string(), field_ident.span()).to_token_stream();

            quote_spanned! { field_ident.span() => &self.#field_ident }
        }
    })
}

fn format_enum(data: DataEnum) -> TokenStream {
    let match_variants = data.variants.into_iter().map(|variant| {
        let variant_ident = &variant.ident;

        let original_field_idents = variant.fields.iter().map(|field| &field.ident);

        let field_idents = (0..variant.fields.len()).map(|field_index| format_ident!("field_{field_index}"));

        let output = format_fields(&variant.fields, &variant.attrs, variant.ident.span(), |_, field_index| {
            format_ident!("field_{field_index}").to_token_stream()
        });

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
