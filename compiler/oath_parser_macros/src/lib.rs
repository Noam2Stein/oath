use quote::quote;
use syn::{parse_macro_input, Data, DataEnum, DataStruct, DeriveInput, Field, Fields, FieldsNamed};

#[proc_macro_derive(Parse)]
pub fn derive_parse(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        attrs: _,
        vis: _,
        ident,
        generics,
        data,
    } = parse_macro_input!(input as DeriveInput);

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let parse_output = match data {
        Data::Enum(DataEnum {
            enum_token,
            brace_token,
            variants,
        }) => 'parse_output: {
            if variants.len() == 0 {
                break 'parse_output quote! {
                    compile_error!("`Parse` cannot be derived for enums with no variants")
                };
            }

            let (last_variant, non_last_variants) = {
                let mut variants = variants.into_iter().collect::<Vec<_>>();
                (variants.pop().unwrap(), variants)
            };

            let last_variant_ident = &last_variant.ident;
            let last_variant_type = &last_variant.;

            let parse_last_variant = match last_variant.fields {
                Fields::Named(fields) => {
                    let field_idents = fields.named.iter().map(|field| &field.ident);
                    let field_types = fields.named.iter().map(|field| &field.ty);
    
                    quote! {
                        Self::#last_variant_ident {
                            #(#field_idents: <#field_types as oath_parser::Parse>::parse(tokens, diagnostics),)*
                        }
                    }
                }
                Fields::Unit => {
                    quote! {
                        Self::#last_variant_ident
                    }
                }
                Fields::Unnamed(fields) => {
                    let field_types = fields.unnamed.iter().map(|field| &field.ty);

                    quote! {
                        Self::#last_variant_ident(#(<#field_types as oath_parser::Parse>::parse(tokens, diagnostics)), *)
                    }
                }
            };

            quote! {
                #(
                    if let Some(value) = <Option<#non_last_variant_types> as oath_parser::Parse>::parse(tokens, diagnostics) {
                        return Self::#non_last_variant_idents(value);
                    }
                )*
                #parse_last_variant
            }
        }
        Data::Struct(DataStruct {
            struct_token: _,
            fields,
            semi_token: _,
        }) => match fields {
            Fields::Named(fields) => {
                let field_idents = fields.named.iter().map(|field| &field.ident);
                let field_types = fields.named.iter().map(|field| &field.ty);

                quote! {
                    Self {
                        #(#field_idents: <#field_types as oath_parser::Parse>::parse(tokens, diagnostics),)*
                    }
                }
            }
            Fields::Unit => quote! {
                Self
            },
            Fields::Unnamed(fields) => {
                let field_types = fields.unnamed.iter().map(|field| &field.ty);

                quote! {
                    Self(#(<#field_types as oath_parser::Parse>::parse(tokens, diagnostics)), *)
                }
            }
        },
        Data::Union(_) => quote! {
            compile_error!("`Parse` cannot be derived for unions")
        },
    };

    quote! {
        impl # impl_generics oath_parser::Parse for #ident #ty_generics #where_clause {
            fn parse(
                tokens: &mut std::iter::Peekable<impl Iterator<Item = oath_tokenizer::TokenTree>>,
                diagnostics: oath_diagnostics::DiagnosticsHandle,
            ) -> Self {
                #parse_output
            }
        }
    }
    .into()
}
