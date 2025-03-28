use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, quote_spanned};
use syn::{
    spanned::Spanned, Attribute, Data, DataEnum, DataStruct, DataUnion, DeriveInput, Error, Fields,
};

pub fn impl_parser_trait(
    input: &DeriveInput,
    crate_ident: &'static str,
    trait_ident: &'static str,
    fn_ident: &'static str,
    fn_params: TokenStream,
    fn_output: TokenStream,
    struct_impl: Option<fn(&DataStruct) -> TokenStream>,
    enum_impl: Option<fn(&DataEnum) -> TokenStream>,
    union_impl: Option<fn(&DataUnion) -> TokenStream>,
) -> TokenStream {
    let eval = match &input.data {
        Data::Struct(data) => match struct_impl {
            Some(struct_impl) => struct_impl(data),
            None => {
                let error_message = format!("`{}` cannot be derived for structs", trait_ident);
                quote! { compile_error!(#error_message) }
            }
        },
        Data::Enum(data) => match enum_impl {
            Some(enum_impl) => enum_impl(data),
            None => {
                let error_message = format!("`{}` cannot be derived for enums", trait_ident);
                quote! { compile_error!(#error_message) }
            }
        },
        Data::Union(data) => match union_impl {
            Some(union_impl) => union_impl(data),
            None => {
                let error_message = format!("`{}` cannot be derived for unions", trait_ident);
                quote! { compile_error!(#error_message) }
            }
        },
    };

    let type_ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let crate_ident = format_ident!("{crate_ident}");
    let trait_ident = format_ident!("{trait_ident}");
    let fn_ident = format_ident!("{fn_ident}");

    quote! {
        impl #impl_generics ::#crate_ident::#trait_ident for #type_ident #ty_generics #where_clause {
            fn #fn_ident(#fn_params) -> #fn_output {
                #eval
            }
        }
    }
    .into()
}

pub fn parse_fields(fields: &Fields, fields_span: Span) -> TokenStream {
    let parse_fields = fields.iter().map(|field| {
        let field_type = &field.ty;

        quote_spanned! {
            field_type.span() =>
            <#field_type as oath_parser::Parse>::parse(parser)
        }
    });

    match fields {
        Fields::Named(_) => {
            let field_idents = fields.iter().map(|field| &field.ident);

            quote_spanned! {
                fields_span =>
                {
                    #(#field_idents: #parse_fields,)*
                }
            }
        }
        Fields::Unnamed(_) => {
            quote_spanned! {
                fields_span =>
                (#(#parse_fields), *)
            }
        }
        Fields::Unit => {
            quote! {}
        }
    }
}

pub fn detect_fields(fields: &Fields, fields_span: Span) -> TokenStream {
    let (option_detect_fields, detect_field) = 'find_fields: {
        let mut option_detect_fields = Vec::new();

        for field in fields {
            if has_attrib(&field.attrs, "option_detect") {
                option_detect_fields.push(field);
            } else {
                break 'find_fields (option_detect_fields, field);
            }
        }

        return Error::new(fields_span, "expected detectable fields").to_compile_error();
    };

    let option_detect_fields = option_detect_fields.into_iter().map(|field| {
        let field_type = &field.ty;

        quote_spanned! {
            field_type.span() =>
            <#field_type as oath_parser::OptionDetect>::option_detect(parser)
        }
    });

    let detect_field = {
        let field_type = &detect_field.ty;

        quote_spanned! {
            detect_field.span() =>
            <#field_type as oath_parser::Detect>::detect(parser)
        }
    };

    quote! {
        (#(#option_detect_fields ||)* #detect_field)
    }
}

pub fn condition_parse_fields_if(fields: &Fields, fields_span: Span) -> TokenStream {
    let (option_detect_fields, detect_field) = 'find_fields: {
        let mut option_detect_fields = Vec::new();

        for field in fields {
            if has_attrib(&field.attrs, "option_detect") {
                option_detect_fields.push(field);
            } else {
                break 'find_fields (option_detect_fields, field);
            }
        }

        return Error::new(fields_span, "expected detectable fields").to_compile_error();
    };

    if option_detect_fields.is_empty() {
        let field_type = &detect_field.ty;

        quote_spanned! {
            detect_field.span() =>
            let Some(first_field) = <#field_type as oath_parser::OptionParse>::option_parse(parser)
        }
    } else {
        let option_detect_fields = option_detect_fields.into_iter().map(|field| {
            let field_type = &field.ty;

            quote_spanned! {
                field_type.span() =>
                <#field_type as oath_parser::OptionDetect>::option_detect(parser)
            }
        });

        let detect_field = {
            let field_type = &detect_field.ty;

            quote_spanned! {
                detect_field.span() =>
                <#field_type as oath_parser::Detect>::detect(parser)
            }
        };

        quote! {
            (#(#option_detect_fields ||)* #detect_field)
        }
    }
}

pub fn parse_detected_fields(fields: &Fields, fields_span: Span) -> TokenStream {
    let (option_detect_fields, detect_field, secondary_fields) = {
        let mut fields_iter = fields.iter();

        let mut option_detect_fields = Vec::new();
        let mut detect_field = None;

        while let Some(field) = fields_iter.next() {
            if has_attrib(&field.attrs, "option_detect") {
                option_detect_fields.push(field);
            } else {
                detect_field = Some(field);
                break;
            }
        }

        let secondary_fields = fields_iter.collect::<Vec<_>>();

        if let Some(detect_field) = detect_field {
            (option_detect_fields, detect_field, secondary_fields)
        } else {
            return quote! { unreachable!() };
        }
    };

    let parse_option_detect_fields = option_detect_fields.iter().map(|field| {
        let field_type = &field.ty;

        quote_spanned! {
            field_type.span() =>
            <#field_type as oath_parser::Parse>::parse(parser)
        }
    });

    let parse_detect_field = if option_detect_fields.is_empty() {
        quote_spanned! {
            detect_field.span() =>
            first_field
        }
    } else {
        let field_type = &detect_field.ty;

        quote_spanned! {
            detect_field.span() =>
            <#field_type as oath_parser::Parse>::parse(parser)
        }
    };

    let parse_secondary_fields = secondary_fields.iter().map(|field| {
        let field_type = &field.ty;

        quote_spanned! {
            field_type.span() =>
            <#field_type as oath_parser::Parse>::parse(parser)
        }
    });

    match fields {
        Fields::Named(_) => {
            let option_detect_field_idents = option_detect_fields.iter().map(|field| &field.ident);
            let detect_field_ident = &detect_field.ident;
            let secondary_field_idents = secondary_fields.iter().map(|field| &field.ident);

            quote_spanned! {
                fields_span =>
                {
                    #(#option_detect_field_idents: #parse_option_detect_fields,)*
                    #detect_field_ident: #parse_detect_field,
                    #(#secondary_field_idents: #parse_secondary_fields,)*
                }
            }
        }
        Fields::Unnamed(_) => {
            quote_spanned! {
                fields_span =>
                (
                    #(#parse_option_detect_fields,)*
                    #parse_detect_field,
                    #(#parse_secondary_fields,)*
                )
            }
        }
        Fields::Unit => {
            quote! {}
        }
    }
}

pub fn option_detect_fields(fields: &Fields) -> TokenStream {
    let non_option_detect_errors = fields
        .iter()
        .filter(|field| !has_attrib(&field.attrs, "option_detect"))
        .map(|field| {
            Error::new(field.span(), "field must be marked `option_detect`").to_compile_error()
        });

    let option_detect_fields = fields.iter().map(|field| {
        let field_type = &field.ty;

        quote_spanned! {
            field_type.span() =>
            <#field_type as oath_parser::OptionDetect>::option_detect(parser)
        }
    });

    quote! {
        {
            #(#non_option_detect_errors;)*

            #(#option_detect_fields)||*
        }
    }
}

pub fn has_attrib<'a>(attribs: impl IntoIterator<Item = &'a Attribute>, attrib: &'a str) -> bool {
    attribs.into_iter().any(|attr| attr.path().is_ident(attrib))
}
