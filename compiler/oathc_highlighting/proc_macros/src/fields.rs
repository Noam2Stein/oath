use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::{Fields, Member, spanned::Spanned};

pub fn highlight_fields(fields: &Fields, mut get_field: impl FnMut(Member, usize) -> TokenStream) -> TokenStream {
    let highlight_fields = fields
        .members()
        .zip(fields.iter())
        .enumerate()
        .filter(|(_, (_, field))| field.attrs.iter().any(|attr| attr.path().is_ident("highlightable")))
        .map(|(idx, (member, field))| {
            let field_path = get_field(member, idx);
            let field_type = &field.ty;

            quote_spanned! {
                field.ident.span() =>
                <#field_type as Highlightable>::highlight(#field_path, color, h);
            }
        });

    quote_spanned! {
        fields.span() => {#(
            #highlight_fields
        )*}
    }
}
