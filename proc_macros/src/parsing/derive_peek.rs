use quote::quote;
use syn::{DeriveInput, Type, Variant};

use super::*;

pub fn derive_peek(input: TokenStream1) -> TokenStream1 {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = &input.ident;

    let block_content = match input.data {
        syn::Data::Struct(data) => {
            let field_ty = &data.fields.iter().next().unwrap().ty;

            quote! {
                <#field_ty as oath::parsing::Peek>::peek(input, errors, bound_to_line)
            }
        }
        syn::Data::Enum(data) => {
            fn variant_ty(variant: &Variant) -> &Type {
                match &variant.fields {
                    syn::Fields::Unnamed(fields) => {
                        if fields.unnamed.len() == 1 {
                            &fields.unnamed.first().unwrap().ty
                        } else {
                            panic!("expected a single unnamed type")
                        }
                    }
                    _ => panic!("expected a single unnamed type"),
                }
            }

            let first_variant_ty = variant_ty(data.variants.first().unwrap());
            let last_variant_ty = variant_ty(data.variants.last().unwrap());
            let middle_variant_ty = data
                .variants
                .iter()
                .skip(1)
                .take(data.variants.len() - 2)
                .map(|variant| variant_ty(variant));

            let last_variant_is_error = data.variants.last().unwrap().attrs.iter().any(|attr| {
                attr.path()
                    .get_ident()
                    .map_or(false, |ident| &ident.to_string() == "error")
            });

            if last_variant_is_error {
                quote! {
                    <#first_variant_ty as oath::parsing::Peek>::peek(input, errors, bound_to_line)
                        #(|| <#middle_variant_ty as oath::parsing::Peek>::peek(input, errors, bound_to_line))*
                }
            } else {
                quote! {
                    <#first_variant_ty as oath::parsing::Peek>::peek(input, errors, bound_to_line)
                        #(|| <#middle_variant_ty as oath::parsing::Peek>::peek(input, errors, bound_to_line))*
                        || <#last_variant_ty as oath::parsing::Peek>::peek(input, errors, bound_to_line)
                }
            }
        }
        syn::Data::Union(_) => quote! { compiler_error!("unions not supported") },
    };

    quote! {
        impl oath::parsing::Peek for #ident {
            fn peek(
                input: &mut impl oath::tokenization::TokenIterator,
                errors: &mut oath::diagnostics::ErrorsHandle,
                bound_to_line: bool
            ) -> bool {
                #block_content
            }
        }
    }
    .into()
}
