use quote::quote;
use syn::{DeriveInput, LitStr};

use super::*;

pub fn derive_parse(input: TokenStream1) -> TokenStream1 {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = &input.ident;

    let block_content = match input.data {
        syn::Data::Struct(data) => {
            let field_idents = data
                .fields
                .iter()
                .map(|field| field.ident.as_ref().unwrap());

            quote! {
                Self {
                    #(#field_idents: oath::parsing::Parse::parse(input, errors, bound_to_line),)*
                }
            }
        }
        syn::Data::Enum(data) => {
            if data.variants.len() == 0 {
                quote! { compiler_error!("can't parse enum with no variants") }
            } else if data.variants.len() == 1 {
                let variant_ident = &data.variants.first().unwrap().ident;
                quote! {
                    Self::#variant_ident(oath::parsing::Parse::parse(input, errors, bound_to_line))
                }
            } else {
                let first_variant_ident = &data.variants.first().unwrap().ident;
                let last_variant_ident = &data.variants.last().unwrap().ident;
                let middle_variant_ident = data
                    .variants
                    .iter()
                    .skip(1)
                    .take(data.variants.len() - 2)
                    .map(|variant| &variant.ident);

                let last_variant_is_error =
                    data.variants.last().unwrap().attrs.iter().any(|attr| {
                        attr.path()
                            .get_ident()
                            .map_or(false, |ident| &ident.to_string() == "error")
                    });

                let error_str = LitStr::new(&format!("expected {ident}"), Span::call_site());

                if last_variant_is_error {
                    quote! {
                        if let Some(output) = oath::parsing::Parse::parse(input, errors, bound_to_line) {
                            Self::#first_variant_ident(output)
                        } #(else if let Some(output) = oath::parsing::Parse::parse(input, errors, bound_to_line) {
                            Self::#middle_variant_ident(output)
                        })* else {
                            let span = input.next_span(errors, bound_to_line);

                            errors.push(oath::diagnostics::Error::new(span, #error_str));

                            Self::#last_variant_ident(span)
                        }
                    }
                } else {
                    quote! {
                        if let Some(output) = oath::parsing::Parse::parse(input, errors, bound_to_line) {
                            Self::#first_variant_ident(output)
                        } #(else if let Some(output) = oath::parsing::Parse::parse(input, errors, bound_to_line) {
                            Self::#middle_variant_ident(output)
                        })* else {
                            Self::#last_variant_ident(oath::parsing::Parse::parse(input, errors, bound_to_line))
                        }
                    }
                }
            }
        }
        syn::Data::Union(_) => quote! { compiler_error!("unions not supported") },
    };

    quote! {
        impl oath::parsing::Parse for #ident {
            fn parse(
                input: &mut impl oath::tokenization::TokenIterator,
                errors: &mut oath::diagnostics::ErrorsHandle,
                bound_to_line: bool,
            ) -> Self {
                #block_content
            }
        }
    }
    .into()
}
