use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::{
    parse_macro_input, spanned::Spanned, Data, DataEnum, DataStruct, DeriveInput, Fields, Ident,
};

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
        Data::Struct(data) => struct_span(data),
        Data::Enum(data) => enum_span(data),
        Data::Union(_) => quote! { compile_error!("`Span` cannot be derived for unions") },
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
    fields_span(
        &data.fields,
        |field_ident, _| quote_spanned! { field_ident.span() => self.#field_ident },
        Span::call_site(),
    )
}

fn enum_span(data: DataEnum) -> TokenStream {
    let match_variants = data.variants.into_iter().map(|variant| {
        let variant_ident = &variant.ident;

        let field_idents =
            (0..variant.fields.len()).map(|field_index| format_ident!("field_{field_index}"));

        let span = fields_span(
            &variant.fields,
            |_, field_index| format_ident!("field_{field_index}").to_token_stream(),
            variant.span(),
        );

        quote_spanned! {
            variant.span() =>

            Self::#variant_ident(#(#field_idents), *) => #span,
        }
    });

    quote! {
        match self {
            #(#match_variants)*
        }
    }
}

fn fields_span(
    fields: &Fields,
    get_field_path: impl Fn(Option<&Ident>, usize) -> TokenStream,
    span: Span,
) -> TokenStream {
    let span_field = fields
        .iter()
        .zip(0..)
        .find(|(field, _)| field.attrs.iter().any(|attr| attr.path().is_ident("span")));

    let mut span_fields = if let Some(span_field) = span_field {
        vec![span_field]
    } else {
        fields.iter().zip(0..).collect()
    };

    if span_fields.is_empty() {
        return quote_spanned! {
            span =>
            compile_error!("expected fields")
        };
    }

    let (base_field, base_field_index) = span_fields.pop().unwrap();
    let base_field_ty = &base_field.ty;
    let base_field_path = get_field_path(base_field.ident.as_ref(), base_field_index);

    let field_types = span_fields.iter().map(|(field, _)| &field.ty);

    let field_paths = span_fields
        .iter()
        .map(|(field, field_index)| get_field_path(field.ident.as_ref(), *field_index));

    quote! {
        {
            let mut span = <#base_field_ty as ::oath_src::Spanned>::span(&#base_field_path);
            #(
                span += <#field_types as ::oath_src::Spanned>::span(&#field_paths);
            )*

            span
        }
    }
}
