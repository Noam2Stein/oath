use oath_parser_derive_macro_utils::{
    impl_desc, impl_detect, impl_parse, impl_peek_ok, impl_try_parse,
};
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

#[proc_macro_derive(OptionParse, attributes(desc))]
pub fn derive_option_parse(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let impl_desc = impl_desc(input.clone().into());
    let impl_try_parse = impl_try_parse(input.clone().into());
    let impl_parse = impl_parse(input.into());

    quote! {
        #impl_desc
        #impl_try_parse
        #impl_parse
    }
    .into()
}

#[proc_macro_derive(TryParse, attributes(desc, try_parse, fallback, error_fallback))]
pub fn derive_try_parse(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let impl_desc = impl_desc(input.clone().into());
    let impl_try_parse = impl_try_parse(input.into());

    quote! {
        #impl_desc
        #impl_try_parse
    }
    .into()
}

#[proc_macro_derive(PeekOk)]
pub fn derive_peek_ok(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    impl_peek_ok(input.into()).into()
}
