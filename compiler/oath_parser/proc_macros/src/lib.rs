use oath_parser_derive_macro_utils::{impl_desc, impl_detect, impl_option_parse, impl_parse};
use quote::quote;

#[proc_macro_derive(ParseDesc, attributes(desc))]
pub fn derive_desc(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    impl_desc(input.into()).into()
}

#[proc_macro_derive(Parse, attributes(desc, try_parse, fallback, error_fallback))]
pub fn derive_parse(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    impl_parse(input.into()).into()
}

#[proc_macro_derive(Detect, attributes(desc, try_parse, fallback, error_fallback))]
pub fn derive_detect(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    impl_detect(input.into()).into()
}

#[proc_macro_derive(OptionParse, attributes(desc, try_parse, fallback, error_fallback))]
pub fn derive_option_parse(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let impl_desc = impl_desc(input.clone().into());
    let impl_detect = impl_detect(input.clone().into());
    let impl_option_parse = impl_option_parse(input.clone().into());

    quote! {
        #impl_desc
        #impl_detect
        #impl_option_parse
    }
    .into()
}
