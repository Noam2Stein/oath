use proc_macro::TokenStream as TokenStream1;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote_spanned, ToTokens};
use syn::{parse_macro_input, Error};

mod parsing;
mod tokenization;

macro_rules! export {
    ($(#[$meta:meta])* $ident:ident in $($path:ident)::*) => {
        $(#[$meta])*
        #[proc_macro]
        #[allow(non_snake_case)]
        pub fn $ident(tokens: TokenStream1) -> TokenStream1 {
            $($path::)*$ident(tokens)
        }
    };
}
macro_rules! export_derive {
    ($(#[$meta:meta])* $ident:ident($($tt:tt)*) in $($path:ident)::*) => {
        $(#[$meta])*
        #[proc_macro_derive($($tt)*)]
        #[allow(non_snake_case)]
        pub fn $ident(tokens: TokenStream1) -> TokenStream1 {
            $($path::)*$ident(tokens)
        }
    };
}

export!(keywords in tokenization::keyword);
export!(Keyword in tokenization::keyword);
export!(puncts in tokenization::punct);
export!(Punct in tokenization::punct);

export_derive!(derive_parse(Parse, attributes(error)) in parsing::derive_parse);
export_derive!(derive_peek(Peek) in parsing::derive_peek);
