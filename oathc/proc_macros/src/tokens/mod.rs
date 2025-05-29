use derive_syn_parse::Parse;
use oathc_token_definitions::with_tokens_expr;
use proc_macro2::Group;
use quote::quote;
use syn::{Ident, LitStr, parse_macro_input};

pub fn keyword(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    #[derive(Parse)]
    struct Input {
        keyword: LitStr,
        init: Option<Group>,
    }

    let Input { keyword, init } = parse_macro_input!(input as Input);

    let keyword_type = Ident::new(keyword_to_type(keyword.value().as_str()).as_str(), keyword.span());

    quote! {
        ::oath_tokens::#keyword_type #init
    }
    .into()
}

pub fn punct(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    #[derive(Parse)]
    struct Input {
        punct: LitStr,
        init: Option<Group>,
    }

    let Input { punct, init } = parse_macro_input!(input as Input);

    let punct_type = Ident::new(
        with_tokens_expr! {
            match punct.value().as_str() {
                $($punct => stringify!($punct_type),)*
                non_punct => panic!("`{non_punct}` is not a punct"),
            }
        },
        punct.span(),
    );

    quote! {
        ::oath_tokens::#punct_type #init
    }
    .into()
}

pub fn delims(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    #[derive(Parse)]
    struct Input {
        delims: LitStr,
    }

    let Input { delims } = parse_macro_input!(input as Input);

    let delims_type = Ident::new(
        with_tokens_expr! {
            match delims.value().as_str() {
                $(concat!($delim_open, " ", $delim_close) => stringify!($delims_type),)*
                non_delims => panic!("`{non_delims}` is not a delimiter"),
            }
        },
        delims.span(),
    );

    quote! {
        ::oath_tokens::#delims_type
    }
    .into()
}

pub fn open(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    #[derive(Parse)]
    struct Input {
        open: LitStr,
        init: Option<Group>,
    }

    let Input { open, init } = parse_macro_input!(input as Input);

    let open_type = Ident::new(
        with_tokens_expr! {
            match open.value().as_str() {
                $($delim_open => stringify!($delim_open_type),)*
                non_open => panic!("`{non_open}` is not an open delimiter"),
            }
        },
        open.span(),
    );

    quote! {
        ::oath_tokens::#open_type #init
    }
    .into()
}

pub fn close(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    #[derive(Parse)]
    struct Input {
        close: LitStr,
        init: Option<Group>,
    }

    let Input { close, init } = parse_macro_input!(input as Input);

    let close_type = Ident::new(
        with_tokens_expr! {
            match close.value().as_str() {
                $($delim_close => stringify!($delim_close_type),)*
                non_close => panic!("`{non_close}` is not an open delimiter"),
            }
        },
        close.span(),
    );

    quote! {
        ::oath_tokens::#close_type #init
    }
    .into()
}

fn keyword_to_type(keyword: &str) -> String {
    keyword
        .chars()
        .enumerate()
        .map(
            |(char_index, char)| {
                if char_index == 0 { char.to_ascii_uppercase() } else { char }
            },
        )
        .chain("Keyword".chars())
        .collect()
}
