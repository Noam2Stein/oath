use super::*;

use quote::quote;
use syn::LitStr;

const PUNCTS: &[&str] = &[
    "::", "!=", "%=", "^=", "&=", "*=", "-=", "==", "+=", "=>", "->", ">=", "<=", "|=", "/=",
    "...", "..", ">>", "<<", "`", "-", "=", "~", "!", "#", "$", "%", "^", "&", "*", "+", "\\", "|",
    ";", ":", ",", ".", "/", "<", ">", "?",
];

pub fn puncts(input: TokenStream1) -> TokenStream1 {
    let input = TokenStream::from(input);

    let puncts = PUNCTS;

    let punct_ty_idents = (0..PUNCTS.len())
        .into_iter()
        .map(|index| punct_ty(Span::call_site(), index));

    quote! {
        macro_rules! some_temporary_macro {
            ($($str:literal $ty_ident:ident)*) => {
                #input
            }
        }
        some_temporary_macro! { #(#puncts #punct_ty_idents)* }
    }
    .into()
}

#[allow(non_snake_case)]
pub fn Punct(input: TokenStream1) -> TokenStream1 {
    let input = parse_macro_input!(input as LitStr);

    let ident = (0..PUNCTS.len())
        .filter(|index| input.value().as_str() == PUNCTS[*index])
        .next()
        .map_or_else(
            || {
                Error::new(input.span(), format!("'{}' is not a punct", input.value()))
                    .into_compile_error()
            },
            |index| punct_ty(input.span(), index).to_token_stream(),
        );

    quote_spanned! { input.span() => puncts::#ident }.into()
}

fn punct_ty(span: Span, index: usize) -> Ident {
    Ident::new(&format!("TyPunct{index}"), span)
}
