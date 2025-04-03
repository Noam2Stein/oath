use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Attribute, Data, DataEnum, DataStruct, DataUnion, DeriveInput};

pub fn impl_trait(
    input: &DeriveInput,
    trait_ident: &'static str,
    allowed_for_structs: bool,
    allowed_for_enums: bool,
    allowed_for_unions: bool,
    items: impl IntoIterator<Item = TokenStream>,
) -> TokenStream {
    let type_ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let trait_ident = format_ident!("{trait_ident}");

    let cannot_be_derived_error = match input.data {
        Data::Struct(_) if !allowed_for_structs => {
            Some(format!("`{trait_ident}` cannot be derived for structs"))
        }
        Data::Enum(_) if !allowed_for_enums => {
            Some(format!("`{trait_ident}` cannot be derived for enums"))
        }
        Data::Union(_) if !allowed_for_unions => {
            Some(format!("`{trait_ident}` cannot be derived for unions"))
        }
        _ => None,
    };

    if let Some(cannot_be_derived_error) = cannot_be_derived_error {
        return quote! {
            compile_error!(#cannot_be_derived_error);

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
    union_: impl FnOnce(&DataUnion, &Vec<Attribute>) -> O,
) -> O {
    match data {
        Data::Struct(data) => struct_(data, attrs),
        Data::Enum(data) => enum_(data, attrs),
        Data::Union(data) => union_(data, attrs),
    }
}
