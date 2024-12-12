use super::*;

use quote::quote;
use syn::LitStr;

const KEYWORDS: &[&str] = &[
    "unsafe", "mod", "pub", "module", "func", "extern", "async", "type", "struct", "enum", "union",
    "trait", "class", "const", "static", "mut", "var", "if", "else", "match", "loop", "while",
    "for", "as", "extends", "abstract", "virtual", "override", "dyn",
];

pub fn keywords(input: TokenStream1) -> TokenStream1 {
    let input = TokenStream::from(input);

    let keywords = KEYWORDS;

    let keyword_ty_idents = (0..KEYWORDS.len())
        .into_iter()
        .map(|index| keyword_ty(Span::call_site(), index));

    quote! {
        macro_rules! some_temporary_macro {
            ($($str:literal $ty_ident:ident)*) => {
                #input
            }
        }
        some_temporary_macro! { #(#keywords #keyword_ty_idents)* }
    }
    .into()
}

#[allow(non_snake_case)]
pub fn Keyword(input: TokenStream1) -> TokenStream1 {
    let input = parse_macro_input!(input as LitStr);

    let ident = (0..KEYWORDS.len())
        .filter(|index| input.value().as_str() == KEYWORDS[*index])
        .next()
        .map_or_else(
            || {
                Error::new(
                    input.span(),
                    format!("'{}' is not a keyword", input.value()),
                )
                .into_compile_error()
            },
            |index| keyword_ty(input.span(), index).to_token_stream(),
        );

    quote_spanned! { input.span() => keywords::#ident }.into()
}

fn keyword_ty(span: Span, index: usize) -> Ident {
    Ident::new(&format!("TyKeyword{index}"), span)
}
