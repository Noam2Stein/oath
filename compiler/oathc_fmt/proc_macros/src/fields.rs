use proc_macro2::{Span, TokenStream};
use quote::quote_spanned;
use syn::{Attribute, Error, Field, Fields, Ident, spanned::Spanned};

pub fn format_fields(
    fields: &Fields,
    fields_attrs: &[Attribute],
    fields_span: Span,
    get_field_path: impl Fn(Option<&Ident>, usize) -> TokenStream,
) -> TokenStream {
    if fields_attrs.iter().any(|attr| attr.path().is_ident("framed")) {
        if fields.len() == 0 {
            return Error::new(fields_span, "`#[framed]` expects a frame field").into_compile_error();
        }

        let frame_type = &fields.iter().next().unwrap().ty;
        let frame_path = get_field_path(fields.iter().nth(0).unwrap().ident.as_ref(), 0);

        let format_fields = fields
            .iter()
            .enumerate()
            .skip(1)
            .map(|(field_idx, field)| format_field(field, get_field_path(field.ident.as_ref(), field_idx)));

        let variant = if fields_attrs.iter().any(|attr| attr.path().is_ident("dense_delims")) {
            quote_spanned! { frame_type.span() => DenseDelims }
        } else if fields_attrs.iter().any(|attr| attr.path().is_ident("spaced_delims")) {
            quote_spanned! { frame_type.span() => SpacedDelims }
        } else {
            return Error::new(
                fields_span,
                "`#[framed]` expects a either #[dense_delims] or #[spaced_delims]",
            )
            .into_compile_error();
        };

        return quote_spanned! {
            frame_type.span() =>

            FormatTree::#variant(
                #frame_path.delims.open_str(),
                Box::new(FormatTree::Chain([#(#format_fields), *].into())),
                #frame_path.delims.close_str(),
            )
        };
    }

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
            .map_or_else(|err| err.into_compile_error(), |meta| meta.tokens.clone());

        return quote_spanned! {
            field_type.span() =>

            FormatTree::#format_variant(<&#field_type as IntoIterator>::into_iter(#field_path).map(|item| Format::format(item, interner)).collect::<Vec<FormatTree>>())
        };
    }

    quote_spanned! {
        field_type.span() =>

        <#field_type as Format>::format(#field_path, interner)
    }
}
