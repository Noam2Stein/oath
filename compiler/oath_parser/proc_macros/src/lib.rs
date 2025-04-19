mod derive_trait;
mod parse_fields;
mod parse_variants;
use derive_trait::*;
use parse_fields::*;
use parse_variants::*;

mod derive_option_parse;
mod derive_parse;
use derive_option_parse::*;
use derive_parse::*;

#[proc_macro_derive(Parse, attributes(fallback, dont_parse))]
pub fn derive_parse_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use syn::{DeriveInput, parse_macro_input};

    derive_parse(&parse_macro_input!(input as DeriveInput)).into()
}

#[proc_macro_derive(OptionParse, attributes(desc, fallback, dont_parse))]
pub fn derive_option_parse_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use syn::{DeriveInput, parse_macro_input};

    impl_option_parse(&parse_macro_input!(input as DeriveInput)).into()
}
