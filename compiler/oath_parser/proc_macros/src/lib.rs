use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod impl_desc;
mod impl_detect;
mod impl_option_detect;
mod impl_option_parse;
mod impl_parse;
mod impl_parse_error;
mod impl_util;
use impl_desc::impl_desc;
use impl_detect::impl_detect;
use impl_option_detect::impl_option_detect;
use impl_option_parse::impl_option_parse;
use impl_parse::impl_parse;
use impl_parse_error::impl_parse_error;

#[proc_macro_derive(ParseDesc, attributes(desc))]
pub fn derive_desc(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    impl_desc(&input).into()
}

#[proc_macro_derive(Parse, attributes(desc, fallback, option_detect))]
pub fn derive_parse(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let impl_desc = impl_desc(&input);
    let impl_error = impl_parse_error(&input);
    let impl_parse = impl_parse(&input);

    quote! {
        #impl_desc
        #impl_error
        #impl_parse
    }
    .into()
}

#[proc_macro_derive(Detect, attributes(desc, fallback, option_detect))]
pub fn derive_detect(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    impl_detect(&input).into()
}

#[proc_macro_derive(ParseError, attributes(desc, fallback, option_detect))]
pub fn derive_parse_error(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    impl_parse_error(&input).into()
}

#[proc_macro_derive(OptionParse, attributes(desc, fallback, option_detect))]
pub fn derive_option_parse(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let impl_desc = impl_desc(&input);
    let impl_detect = impl_detect(&input);
    let impl_option_parse = impl_option_parse(&input);

    quote! {
        #impl_desc
        #impl_detect
        #impl_option_parse
    }
    .into()
}

#[proc_macro_derive(OptionDetect, attributes(desc, fallback, option_detect))]
pub fn derive_option_detect(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    impl_option_detect(&input).into()
}
