use oath_keywords_puncts::{keyword_to_type, keyword_to_variant, KEYWORDS, PUNCTS};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Ident, LitInt, LitStr};

/// provides meta info about all Oath keywords with `$()*` + `$info` syntax.
///
/// `$keyword:ident`, `$keyword_len:literal`, `$keyword_type:ident`, `$keyword_variant:ident`
#[proc_macro]
pub fn with_keywords(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);

    let macro_input = KEYWORDS.into_iter().map(|keyword_info| {
        let keyword = Ident::new(&keyword_info.str, Span::call_site());

        let keyword_len = LitInt::new(
            keyword_info.str.len().to_string().as_str(),
            Span::call_site(),
        );

        let keyword_type = Ident::new(
            keyword_to_type(keyword_info.str).as_str(),
            Span::call_site(),
        );

        let keyword_variant = Ident::new(
            keyword_to_variant(keyword_info.str).as_str(),
            Span::call_site(),
        );

        quote! {
            #keyword #keyword_len #keyword_type #keyword_variant,
        }
    });

    quote! {
        macro_rules! collage_of_water {
            ($($keyword:ident $keyword_len:literal $keyword_type:ident $keyword_variant:ident, )*) => {
                #input
            }
        }
        collage_of_water! {
            #(#macro_input)*
        }
    }.into()
}

/// provides meta info about all Oath puncts with `$()*` + `$info` syntax.
///
/// `$punct:literal`, `$punct_len:literal`, `$punct_type:ident`, `$punct_variant:ident`
#[proc_macro]
pub fn with_puncts(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);

    let macro_input = PUNCTS.into_iter().map(|punct_info| {
        let punct = LitStr::new(&punct_info.str, Span::call_site());

        let punct_len = LitInt::new(punct_info.str.len().to_string().as_str(), Span::call_site());

        let punct_type = Ident::new(
            format!("{}Punct", punct_info.name).as_str(),
            Span::call_site(),
        );

        let punct_variant = Ident::new(punct_info.name, Span::call_site());

        quote! {
            #punct #punct_len #punct_type #punct_variant,
        }
    });

    quote! {
        macro_rules! double_market {
            ($($punct:literal $punct_len:literal $punct_type:ident $punct_variant:ident, )*) => {
                #input
            }
        }
        double_market! {
            #(#macro_input)*
        }
    }
    .into()
}
