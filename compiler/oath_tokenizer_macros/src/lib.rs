use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput, Error, Fields, Ident};

#[proc_macro_derive(TokenDowncast)]
pub fn derive_token_downcast(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let self_ident = &input.ident;
    let (_, ty_generics, _) = input.generics.split_for_impl();

    match input.data {
        Data::Enum(data) => {
            let variant_impls = data.variants.iter().map(|variant| match &variant.fields {
                Fields::Named(fields) => Error::new(
                    fields.span(),
                    "enums with named fields cannot derive `TokenDowncast`",
                )
                .into_compile_error(),
                Fields::Unit => TokenStream::new(),
                Fields::Unnamed(fields) => {
                    let variant_ident = &variant.ident;
                    let field_types = fields.unnamed.iter();
                    let field_idents = (0..fields.unnamed.len()).map(|i| Ident::new(format!("field_{i}").as_str(), fields.unnamed[i].span())).collect::<Box<[_]>>();

                    let impl_generics = input.generics.params.iter();
                    let where_clause = input.generics.where_clause.as_ref().map(|where_clause| where_clause.predicates.clone()).unwrap_or_default().into_iter();

                    quote_spanned! {
                        variant.span() =>

                        #[allow(unused_parens)]
                        impl<FromType, #(#impl_generics), *> crate::TokenDowncastFrom<FromType> for (#(#field_types)*)
                        where
                            #self_ident #ty_generics: crate::TokenDowncastFrom<FromType>,
                            #(#where_clause,)*
                        {
                            fn downcast_from(value: FromType) -> Option<Self> {
                                crate::TokenDowncast::downcast(value).map_or(None, |value|
                                    if let #self_ident::#variant_ident(#(#field_idents), *) = value {
                                        Some((#(#field_idents), *))
                                    } else {
                                        None
                                    }
                                )
                            }
                            fn downcast_from_ref(value: &FromType) -> Option<&Self> {
                                crate::TokenDowncast::downcast_ref(value).map_or(None, |value|
                                    if let #self_ident::#variant_ident(#(#field_idents), *) = value {
                                        Some((#(#field_idents), *))
                                    } else {
                                        None
                                    }
                                )
                            }
                        }
                    }
                }
            });

            quote! {
                #(#variant_impls)*
            }
        }
        Data::Struct(data) => Error::new(
            data.struct_token.span,
            "structs cannot derive `TokenDowncast`",
        )
        .into_compile_error(),
        Data::Union(data) => Error::new(
            data.union_token.span,
            "unions cannot derive `TokenDowncast`",
        )
        .into_compile_error(),
    }.into()
}
