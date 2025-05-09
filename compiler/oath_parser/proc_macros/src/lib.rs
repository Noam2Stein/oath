mod util_derive;
mod util_fields;
use util_derive::*;
use util_fields::*;

mod derive_option_parse;
mod derive_parse;
use derive_option_parse::*;
use derive_parse::*;

#[proc_macro_derive(Parse, attributes(fallback, group))]
pub fn derive_parse_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use syn::{DeriveInput, parse_macro_input};

    derive_parse(&parse_macro_input!(input as DeriveInput)).into()
}

#[proc_macro_derive(OptionParse, attributes(desc, fallback, group))]
pub fn derive_option_parse_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use syn::{DeriveInput, parse_macro_input};

    derive_option_parse(&parse_macro_input!(input as DeriveInput)).into()
}
