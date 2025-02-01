use quote::quote;
use syn::{parse_macro_input, DeriveInput, LitStr};

#[proc_macro_derive(Desc, attributes(desc))]
pub fn desc(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        attrs,
        vis: _,
        ident,
        generics,
        data: _,
    } = parse_macro_input!(input as DeriveInput);

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let desc = attrs.iter().find_map(|attr| {
        if attr.path().is_ident("desc") {
            Some(attr.parse_args::<LitStr>().unwrap())
        } else {
            None
        }
    });

    quote! {
        impl #impl_generics ::oath_diagnostics::Desc for #ident #ty_generics #where_clause {
            fn desc() -> &'static str {
                #desc
            }
        }
    }
    .into()
}
