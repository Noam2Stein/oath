use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Attribute, Data, DataEnum, DataStruct, DeriveInput};

pub fn impl_trait(
    input: &DeriveInput,
    trait_ident: &'static str,
    items: impl IntoIterator<Item = TokenStream>,
) -> TokenStream {
    let type_ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let trait_ident = format_ident!("{trait_ident}");

    if let Data::Union(_) = &input.data {
        let error = format!("`{trait_ident}` cannot be derived for unions");

        return quote! {
            compile_error!(#error);

            impl #impl_generics ::oath_parser::#trait_ident for #type_ident #ty_generics #where_clause {}
        };
    }

    let items = items.into_iter();

    quote! {
        impl #impl_generics ::oath_parser::#trait_ident for #type_ident #ty_generics #where_clause {
            #(#items)*
        }
    }
    .into()
}
pub fn impl_trait_fn(sig: TokenStream, output: TokenStream) -> TokenStream {
    quote! {
        #sig {
            #output
        }
    }
}
pub fn data_split<O>(
    data: &Data,
    attrs: &Vec<Attribute>,
    struct_: impl FnOnce(&DataStruct, &Vec<Attribute>) -> O,
    enum_: impl FnOnce(&DataEnum, &Vec<Attribute>) -> O,
) -> O {
    match data {
        Data::Struct(data) => struct_(data, attrs),
        Data::Enum(data) => enum_(data, attrs),
        Data::Union(_) => unreachable!(),
    }
}
