use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DataEnum, DataStruct, DataUnion, DeriveInput, Ident, parse2};

mod impl_desc;
mod impl_detect;
mod impl_option_parse;
mod impl_parse;
pub use impl_desc::*;
pub use impl_detect::*;
pub use impl_option_parse::*;
pub use impl_parse::*;

fn impl_parser_trait(
    input: TokenStream,
    crate_ident: &'static str,
    trait_ident: &'static str,
    self_type: fn(&Ident) -> TokenStream,
    fn_ident: &'static str,
    params: TokenStream,
    output: TokenStream,
    eval_struct: Option<fn(DataStruct) -> TokenStream>,
    eval_enum: Option<fn(DataEnum) -> TokenStream>,
    eval_union: Option<fn(DataUnion) -> TokenStream>,
) -> TokenStream {
    let DeriveInput {
        attrs: _,
        vis: _,
        ident,
        generics,
        data,
    } = match parse2(input) {
        Ok(input) => input,
        Err(error) => return error.into_compile_error(),
    };

    let eval = match data {
        Data::Struct(data) => match eval_struct {
            Some(fn_) => fn_(data),
            None => {
                let error_message = format!("`{}` cannot be derived for structs", trait_ident);
                quote! { compile_error!(#error_message) }
            }
        },
        Data::Enum(data) => match eval_enum {
            Some(fn_) => fn_(data),
            None => {
                let error_message = format!("`{}` cannot be derived for enums", trait_ident);
                quote! { compile_error!(#error_message) }
            }
        },
        Data::Union(data) => match eval_union {
            Some(fn_) => fn_(data),
            None => {
                let error_message = format!("`{}` cannot be derived for unions", trait_ident);
                quote! { compile_error!(#error_message) }
            }
        },
    };

    let crate_ident = format_ident!("{crate_ident}");
    let trait_ident = format_ident!("{trait_ident}");
    let fn_ident = format_ident!("{fn_ident}");

    let self_type = self_type(&ident);

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics ::#crate_ident::#trait_ident for #self_type #ty_generics #where_clause {
            fn #fn_ident(#params) -> #output {
                #eval
            }
        }
    }
    .into()
}
