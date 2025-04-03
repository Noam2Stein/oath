mod derive_trait;
mod parse_fields;
mod parse_variants;
use derive_trait::*;
use parse_fields::*;
use parse_variants::*;

mod derive_option_detect;
mod derive_parse;
mod impl_option_parse;
use derive_option_detect::*;
use derive_parse::*;
use impl_option_parse::*;

#[proc_macro_derive(Parse, attributes(desc, fallback, try_parse, option_detect))]
pub fn derive_parse_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use syn::{DeriveInput, parse_macro_input};

    derive_parse(&parse_macro_input!(input as DeriveInput)).into()
}

#[proc_macro_derive(OptionParse, attributes(desc, fallback, try_parse, option_detect))]
pub fn derive_option_parse_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use syn::{DeriveInput, parse_macro_input};

    impl_option_parse(&parse_macro_input!(input as DeriveInput)).into()
}

#[proc_macro_derive(OptionDetect, attributes(desc, fallback, try_parse, option_detect))]
pub fn derive_option_detect_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use syn::{DeriveInput, parse_macro_input};

    impl_option_detect(&parse_macro_input!(input as DeriveInput)).into()
}
