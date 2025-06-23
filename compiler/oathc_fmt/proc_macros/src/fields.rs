use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{Attribute, Error, Field, Fields, Ident, spanned::Spanned};

pub fn format_fields(
    fields: &Fields,
    fields_attrs: &[Attribute],
    fields_span: Span,
    get_field_path: impl Fn(Option<&Ident>, usize) -> TokenStream,
) -> TokenStream {
    let format_fields = fields
        .iter()
        .enumerate()
        .map(|(field_idx, field)| format_field(field, get_field_path(field.ident.as_ref(), field_idx)));

    quote_spanned! {
        fields_span =>

        FormatTree::Chain([#(
            #format_fields,
        )*].into())
    }
}

fn format_field(field: &Field, field_path: TokenStream) -> TokenStream {
    let field_type = &field.ty;

    if let Some(attr) = field.attrs.iter().find(|attr| attr.path().is_ident("format_as")) {
        let format_variant = attr
            .meta
            .require_list()
            .map_or_else(|err| err.into_compile_error(), |meta| meta.tokens);

        quote_spanned! {
            field_type.span() =>

            FormatTree::#format_variant
        }
    }

    quote_spanned! {
        field_type.span() =>

        <#field_type as Format>::format(#field_path, interner)
    }
}
