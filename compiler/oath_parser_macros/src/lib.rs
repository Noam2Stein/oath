use quote::{quote, quote_spanned};
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

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let parse_output = match data {
        Data::Enum(DataEnum {
            enum_token: _,
            brace_token: _,
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

            let peek_variants = non_last_variants.into_iter().map(|variant| 'peek_variant: {
                let variant_ident = &last_variant.ident;

                match variant.fields {
                    Fields::Named(fields) => {
                        if fields.named.len() == 0 {
                            break 'peek_variant quote_spanned! {
                                variant_ident.span() =>
                                compile_error!("Cannot peek a variant with no fields");
                            }
                        }

                        let first_field_type = &fields.named[0].ty;
                        let field_idents = fields.named.iter().map(|field| &field.ident);
                        let field_types = fields.named.iter().map(|field| &field.ty);
    
                        quote! {
                            if <#first_field_type as oath_parser::Peek>::peek(tokens, diagnostics) {
                                return Self::#variant_ident {
                                    #(#field_idents: <#field_types as oath_parser::Parse>::parse(tokens, diagnostics),)*
                                };
                            }
                        }
                    },
                    Fields::Unit => {
                        quote_spanned! {
                            variant_ident.span() =>
                            compile_error!("Cannot peek a variant with no fields");
                        }
                    },
                    Fields::Unnamed(fields) => {
                        if fields.unnamed.len() == 0 {
                            break 'peek_variant quote_spanned! {
                                variant_ident.span() =>
                                compile_error!("Cannot peek a variant with no fields");
                            }
                        }

                        let first_field_type = &fields.unnamed[0].ty;                        
                        let field_types = fields.unnamed.iter().map(|field| &field.ty);
    
                        quote! {
                            if <#first_field_type as oath_parser::Peek>::peek(tokens, diagnostics) {
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
