use oath_parser_derive_macro_utils::{impl_desc, impl_parse, impl_peek, impl_try_parse};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Desc, attributes(desc))]
pub fn derive_desc(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    impl_desc(input.into()).into()
}

#[proc_macro_derive(Parse, attributes(desc, try_parse, fallback))]
pub fn derive_parse(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
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

#[proc_macro_derive(Peek, attributes(desc, try_parse, fallback))]
pub fn derive_peek(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let impl_desc = impl_desc(input.clone().into());
    let impl_try_parse = impl_try_parse(input.clone().into());
    let impl_peek = impl_peek(input.into());

    quote! {
        #impl_desc
        #impl_try_parse
        #impl_peek
    }
    .into()
}

#[proc_macro_derive(TryParse, attributes(desc, try_parse, fallback))]
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
    let DeriveInput {
        attrs: _,
        vis: _,
        ident,
        generics,
        data: _,
    } = parse_macro_input!(input as DeriveInput);

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl # impl_generics ::oath_parser::PeekOk for #ident #ty_generics #where_clause {}
    }
    .into()
}
