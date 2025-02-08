use derive_syn_parse::Parse;
use oath_token_set::with_token_set_expr;
use proc_macro2::Group;
use quote::quote;
use syn::{parse_macro_input, Ident, LitStr};

#[proc_macro]
pub fn keyword(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    #[derive(Parse)]
    struct Input {
        keyword: LitStr,
        init: Option<Group>,
    }

    let Input { keyword, init } = parse_macro_input!(input as Input);

    let keyword_type = Ident::new(
        keyword_to_type(keyword.value().as_str()).as_str(),
        keyword.span(),
    );

    quote! {
        ::oath_tokenizer::#keyword_type #init
    }
    .into()
}

#[proc_macro]
pub fn punct(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    #[derive(Parse)]
    struct Input {
        punct: LitStr,
        init: Option<Group>,
    }

    let Input { punct, init } = parse_macro_input!(input as Input);

    let punct_type = Ident::new(
        with_token_set_expr! {
            match punct.value().as_str() {
                $($punct => stringify!($punct_type),)*
                non_punct => panic!("`{non_punct}` is not a punct"),
            }
        },
        punct.span(),
    );

    quote! {
        ::oath_tokenizer::#punct_type #init
    }
    .into()
}

fn keyword_to_type(keyword: &str) -> String {
    keyword
        .chars()
        .enumerate()
        .map(|(char_index, char)| {
            if char_index == 0 {
                char.to_ascii_uppercase()
            } else {
                char
            }
        })
        .chain("Keyword".chars())
        .collect()
}
