use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataEnum, DataStruct, DeriveInput, Fields};

#[proc_macro_derive(Parse)]
pub fn derive_parse(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        attrs: _,
        vis: _,
        ident,
        generics,
        data,
    } = parse_macro_input!(input as DeriveInput);

    let parse_output = match data {
        Data::Enum(data) => parse_enum(data),
        Data::Struct(data) => parse_struct(data),
        Data::Union(_) => quote! { compile_error!("`Parse` cannot be derived for unions") },
    };

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl # impl_generics ::oath_parser::Parse for #ident #ty_generics #where_clause {
            fn parse(
                tokens: &mut ::oath_parser::Parser<impl Iterator<Item = ::oath_tokenizer::TokenTree>>,
                diagnostics: ::oath_diagnostics::DiagnosticsHandle,
            ) -> Self {
                #parse_output
            }
        }
    }
    .into()
}

#[proc_macro_derive(Peek, attributes(dont_peek))]
pub fn derive_peek(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        attrs: _,
        vis: _,
        ident,
        generics,
        data,
    } = parse_macro_input!(input as DeriveInput);

    let parse_output = match data {
        Data::Enum(data) => peek_enum(data),
        Data::Struct(data) => peek_struct(data),
        Data::Union(_) => quote! { compile_error!("`Peek` cannot be derived for unions") },
    };

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl # impl_generics ::oath_parser::Peek for #ident #ty_generics #where_clause {
            fn peek(
                tokens: &mut ::oath_parser::Parser<impl Iterator<Item = ::oath_tokenizer::TokenTree>>,
            ) -> bool {
                #parse_output
            }
        }
    }
    .into()
}

fn parse_struct(data: DataStruct) -> TokenStream {
    match data.fields {
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
    }
}

fn parse_enum(data: DataEnum) -> TokenStream {
    if data.variants.len() == 0 {
        return quote! {
            compile_error!("`Parse` cannot be derived for empty enums")
        };
    };

    let (last_variant, non_last_variants) = {
        let mut variants = data.variants.into_iter().collect::<Vec<_>>();
        (variants.pop().unwrap(), variants)
    };

    let peek_variants = non_last_variants.into_iter().map(|variant| 'peek_variant: {
        let variant_ident = &variant.ident;

        let first_field_type = match variant.fields.iter().next() {
            Some(some) => &some.ty,
            None => break 'peek_variant quote! {
                compile_error!("cannot peek an empty variant")
            },
        };

        let condition  = quote! {
            <#first_field_type as ::oath_parser::Peek>::peek(tokens)
        };

        match variant.fields {
            Fields::Named(fields) => {
                let field_idents = fields.named.iter().map(|field| &field.ident);
                let field_types = fields.named.iter().map(|field| &field.ty);

                quote! {
                    if #condition {
                        return Self::#variant_ident {
                            #(#field_idents: <#field_types as oath_parser::Parse>::parse(tokens, diagnostics),)*
                        };
                    }
                }
            },
            Fields::Unit => {
                unreachable!()
            },
            Fields::Unnamed(fields) => {
                let field_types = fields.unnamed.iter().map(|field| &field.ty);

                quote! {
                    if #condition {
                        return Self::#variant_ident(#(<#field_types as oath_parser::Parse>::parse(tokens, diagnostics)), *);
                    }
                }
            }
        }
    });

    let parse_last_variant = match last_variant.fields {
        Fields::Named(fields) => {
            let variant_ident = &last_variant.ident;
            let field_idents = fields.named.iter().map(|field| &field.ident);
            let field_types = fields.named.iter().map(|field| &field.ty);

            quote! {
                Self::#variant_ident {
                    #(#field_idents: <#field_types as oath_parser::Parse>::parse(tokens, diagnostics),)*
                }
            }
        }
        Fields::Unit => {
            let variant_ident = &last_variant.ident;
            quote! {
                Self::#variant_ident
            }
        }
        Fields::Unnamed(fields) => {
            let variant_ident = &last_variant.ident;
            let field_types = fields.unnamed.iter().map(|field| &field.ty);

            quote! {
                Self::#variant_ident(#(<#field_types as oath_parser::Parse>::parse(tokens, diagnostics)), *)
            }
        }
    };

    quote! {
        #(#peek_variants)*
        #parse_last_variant
    }
}

fn peek_struct(data: DataStruct) -> TokenStream {
    let first_field_type = match data.fields.into_iter().find(|item| {
        !item
            .attrs
            .iter()
            .any(|attrib| attrib.path().is_ident("dont_peek"))
    }) {
        Some(some) => some.ty,
        None => return quote! { compile_error!("`Peek` cannot be derived for empty structs") },
    };

    quote! {
        <#first_field_type as ::oath_parser::Peek>::peek(tokens)
    }
}

fn peek_enum(data: DataEnum) -> TokenStream {
    let peek_variants = data.variants.into_iter().map(|variant| {
        if variant
            .attrs
            .iter()
            .any(|attr| attr.path().is_ident("dont_peek"))
        {
            return quote! {
                false
            };
        }

        let first_field_type = match variant.fields.iter().next() {
            Some(some) => &some.ty,
            None => {
                return quote! {
                    compile_error!("cannot peek an empty variant")
                }
            }
        };

        quote! {
            <#first_field_type as ::oath_parser::Peek>::peek(tokens)
        }
    });

    quote! {
        #(
            if #peek_variants {
                return true;
            }
        )*
        false
    }
}
