use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Error, Field, Fields, Ident, spanned::Spanned};

pub fn fields_span(
    fields: &Fields,
    fields_attrs: &[Attribute],
    get_field_path: impl Fn(Option<&Ident>, usize) -> TokenStream,
) -> TokenStream {
    let (span_fields, field_errors) = get_span_fields(fields, fields_attrs);

    let field_spans = span_fields
        .iter()
        .enumerate()
        .map(|(field_index, field)| {
            let field_type = &field.ty;
            let field_path = get_field_path(field.ident.as_ref(), field_index);

            if field.attrs.iter().any(|attr| attr.path().is_ident("option_spanned")) {
                quote! {
                    <#field_type as OptionSpanned>::option_span(&#field_path)
                }
            } else {
                quote! {
                    <#field_type as Spanned>::span(&#field_path)
                }
            }
        })
        .collect::<Vec<_>>();

    quote! {
        {
            #field_errors

            let span = None::<Span>;
            #(
                let span = span.connect(#field_spans);
            )*

            span
        }
    }
}

pub fn fields_option_span(
    fields: &Fields,
    fields_attrs: &[Attribute],
    get_field_path: impl Fn(Option<&Ident>, usize) -> TokenStream,
) -> TokenStream {
    let span = fields_span(fields, fields_attrs, get_field_path);

    quote! {
        Option::<Span>::from({ #span })
    }
}

fn get_span_fields<'f>(fields: &'f Fields, fields_attrs: &[Attribute]) -> (Vec<&'f Field>, TokenStream) {
    let mut span_fields = fields
        .iter()
        .filter(|field| field.attrs.iter().any(|attr| attr.path().is_ident("span")));

    let frame_field = fields_attrs
        .iter()
        .any(|attr| attr.path().is_ident("framed"))
        .then(|| fields.iter().next())
        .unwrap_or_default();

    if let Some(frame_field) = frame_field {
        let errors = span_fields.map(|field| Error::new(field.span(), "multiple span fields").into_compile_error());
        return (vec![frame_field], quote! { #(#errors;)* });
    }

    let span_field = span_fields.next();

    match span_field {
        Some(span_field) => {
            let errors = span_fields.map(|field| Error::new(field.span(), "multiple span fields").into_compile_error());
            (vec![span_field], quote! { #(#errors;)* })
        }
        None => (
            fields
                .iter()
                .filter(|field| !field.attrs.iter().any(|attr| attr.path().is_ident("not_spanned")))
                .collect(),
            quote! {},
        ),
    }
}
