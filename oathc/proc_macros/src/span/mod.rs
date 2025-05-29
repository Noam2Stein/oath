use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote, quote_spanned};
use syn::{
    Data, DataEnum, DataStruct, DeriveInput, Fields, GenericParam, Ident, LitInt, parse_macro_input, parse_quote,
    spanned::Spanned,
};

pub fn spanned_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        attrs: _,
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

    let parse_output = match data {
        Data::Struct(data) => struct_span(data, false),
        Data::Enum(data) => enum_span(data, false),
        Data::Union(_) => quote! { compile_error!("`Spanned` cannot be derived for unions") },
    };

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics Spanned for #ident #ty_generics #where_clause {
            fn span(&self) -> Span {
                #parse_output
            }
        }
    }
    .into()
}

pub fn option_spanned_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        attrs: _,
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

    let parse_output = match data {
        Data::Struct(data) => struct_span(data, true),
        Data::Enum(data) => enum_span(data, true),
        Data::Union(_) => quote! { compile_error!("`OptionSpanned` cannot be derived for unions") },
    };

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics OptionSpanned for #ident #ty_generics #where_clause {
            fn option_span(&self) -> Option<Span> {
                #parse_output
            }
        }
    }
    .into()
}

fn struct_span(data: DataStruct, expect_option: bool) -> TokenStream {
    fields_span(
        &data.fields,
        |field_ident, field_index| {
            if field_ident.is_some() {
                quote_spanned! { field_ident.span() => self.#field_ident }
            } else {
                let field_ident = LitInt::new(&field_index.to_string(), field_ident.span()).to_token_stream();

                quote_spanned! { field_ident.span() => self.#field_ident }
            }
        },
        expect_option,
    )
}

fn enum_span(data: DataEnum, expect_option: bool) -> TokenStream {
    let match_variants = data.variants.into_iter().map(|variant| {
        let variant_ident = &variant.ident;

        let original_field_idents = variant.fields.iter().map(|field| &field.ident);

        let field_idents = (0..variant.fields.len()).map(|field_index| format_ident!("field_{field_index}"));

        let span = fields_span(
            &variant.fields,
            |_, field_index| format_ident!("field_{field_index}").to_token_stream(),
            expect_option,
        );

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

fn fields_span(
    fields: &Fields,
    get_field_path: impl Fn(Option<&Ident>, usize) -> TokenStream,
    expect_option: bool,
) -> TokenStream {
    let span_field = fields
        .iter()
        .zip(0..)
        .find(|(field, _)| field.attrs.iter().any(|attr| attr.path().is_ident("span")));

    let span_fields = if let Some(span_field) = span_field {
        vec![span_field]
    } else {
        fields.iter().zip(0..).collect()
    };

    let mut is_always_some = false;

    let field_spans_addition = span_fields
        .iter()
        .map(|(field, field_index)| {
            let field_type = &field.ty;
            let field_path = get_field_path(field.ident.as_ref(), *field_index);

            if field.attrs.iter().any(|attr| attr.path().is_ident("option_spanned")) {
                quote! {
                    <#field_type as OptionSpanned>::option_span(&#field_path)
                }
            } else {
                is_always_some = true;

                quote! {
                    Some(<#field_type as Spanned>::span(&#field_path))
                }
            }
        })
        .collect::<Vec<_>>();

    let output = if is_always_some && !expect_option {
        quote! {
            span.unwrap()
        }
    } else {
        quote! {
            span
        }
    };

    quote! {
        {
            let mut span = None::<Span>;

            #(
                span = Span::connect(span, #field_spans_addition);
            )*

            #output
        }
    }
}
