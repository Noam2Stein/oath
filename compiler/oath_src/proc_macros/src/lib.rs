use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::{
    parse_macro_input, parse_quote, spanned::Spanned, Data, DataEnum, DataStruct, DeriveInput,
    Fields, GenericParam, Ident, LitInt,
};

#[proc_macro_derive(Spanned, attributes(span, option_spanned))]
pub fn derive_spanned(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
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
        impl #impl_generics ::oath_src::Spanned for #ident #ty_generics #where_clause {
            fn span(&self) -> ::oath_src::Span {
                #parse_output
            }
        }

        impl #impl_generics PartialEq for #ident #ty_generics #where_clause {
            fn eq(&self, other: &Self) -> bool {
                <Self as ::oath_src::Spanned>::span(self).eq(&<Self as ::oath_src::Spanned>::span(other))
            }
        }
        impl #impl_generics Eq for #ident #ty_generics #where_clause {}

        impl #impl_generics PartialOrd for #ident #ty_generics #where_clause {
            fn partial_cmp(&self, other: &Self) -> Option<::std::cmp::Ordering> {
                <Self as ::oath_src::Spanned>::span(self).partial_cmp(&<Self as ::oath_src::Spanned>::span(other))
            }
        }
        impl #impl_generics Ord for #ident #ty_generics #where_clause {
            fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
                <Self as ::oath_src::Spanned>::span(self).cmp(&<Self as ::oath_src::Spanned>::span(other))
            }
        }
    }
    .into()
}

#[proc_macro_derive(OptionSpanned, attributes(span, option_spanned))]
pub fn derive_option_spanned(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        attrs: _,
        vis: _,
        ident,
        mut generics,
        data,
    } = parse_macro_input!(input as DeriveInput);

    for param in &mut generics.params {
        if let GenericParam::Type(type_param) = param {
            type_param
                .bounds
                .push(parse_quote!(::oath_src::OptionSpanned));
        }
    }

    let parse_output = match data {
        Data::Struct(data) => struct_span(data, true),
        Data::Enum(data) => enum_span(data, true),
        Data::Union(_) => quote! { compile_error!("`OptionSpanned` cannot be derived for unions") },
    };

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics ::oath_src::OptionSpanned for #ident #ty_generics #where_clause {
            fn option_span(&self) -> Option<::oath_src::Span> {
                #parse_output
            }
        }

        impl #impl_generics PartialEq for #ident #ty_generics #where_clause {
            fn eq(&self, other: &Self) -> bool {
                <Self as ::oath_src::OptionSpanned>::option_span(self).eq(&<Self as ::oath_src::OptionSpanned>::option_span(other))
            }
        }
        impl #impl_generics Eq for #ident #ty_generics #where_clause {}

        impl #impl_generics PartialOrd for #ident #ty_generics #where_clause {
            fn partial_cmp(&self, other: &Self) -> Option<::std::cmp::Ordering> {
                <Self as ::oath_src::OptionSpanned>::option_span(self).partial_cmp(&<Self as ::oath_src::OptionSpanned>::option_span(other))
            }
        }
        impl #impl_generics Ord for #ident #ty_generics #where_clause {
            fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
                <Self as ::oath_src::OptionSpanned>::option_span(self).cmp(&<Self as ::oath_src::OptionSpanned>::option_span(other))
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
                let field_ident =
                    LitInt::new(&field_index.to_string(), field_ident.span()).to_token_stream();

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

        let field_idents =
            (0..variant.fields.len()).map(|field_index| format_ident!("field_{field_index}"));

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

            if field
                .attrs
                .iter()
                .any(|attr| attr.path().is_ident("option_spanned"))
            {
                quote! {
                    <#field_type as ::oath_src::OptionSpanned>::option_span(&#field_path)
                }
            } else {
                is_always_some = true;

                quote! {
                    Some(<#field_type as ::oath_src::Spanned>::span(&#field_path))
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
            fn helper(a: Option<::oath_src::Span>, b: Option<::oath_src::Span>) -> Option<::oath_src::Span> {
                a.map_or(b, |a| Some(a + b))
            }

            let mut span = None::<::oath_src::Span>;

            #(
                span = helper(span, #field_spans_addition);
            )*

            #output
        }
    }
}
