use derive_syn_parse::Parse;
use oath_keywords_puncts::with_puncts;
use proc_macro2::{Group, TokenStream};
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput, Error, Fields, Ident, LitStr};

#[proc_macro]
pub fn keyword(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    #[derive(Parse)]
    struct Input {
        keyword: LitStr,
        init: Option<Group>,
    }

    let Input { keyword, init } = parse_macro_input!(input as Input);

    let keyword_type = Ident::new(
        keyword_to_type(keyword.value().as_str()).as_str(),
        keyword.span(),
    );

    quote! {
        ::oath_tokenizer::#keyword_type #init
    }
    .into()
}

#[proc_macro]
pub fn punct(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    #[derive(Parse)]
    struct Input {
        punct: LitStr,
        init: Option<Group>,
    }

    let Input { punct, init } = parse_macro_input!(input as Input);

    let punct_type = Ident::new(
        {
            with_puncts! {
                match punct.value().as_str() {
                    $($punct => stringify!($punct_type),)*
                    non_punct => panic!("`{non_punct}` is not a punct"),
                }
            }
        },
        punct.span(),
    );

    quote! {
        ::oath_tokenizer::#punct_type #init
    }
    .into()
}

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

fn keyword_to_type(keyword: &str) -> String {
    keyword
        .chars()
        .enumerate()
        .map(|(char_index, char)| {
            if char_index == 0 {
                char.to_ascii_uppercase()
            } else {
                char
            }
        })
        .chain("Keyword".chars())
        .collect()
}
