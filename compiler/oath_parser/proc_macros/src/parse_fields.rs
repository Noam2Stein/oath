use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{Attribute, Error, Field, Fields, spanned::Spanned};

pub fn fields_parse_error(fields: &Fields, fields_span: Span) -> TokenStream {
    let members = fields.members();

    let fields_parse_error = fields.iter().map(|field| {
        let field_type = &field.ty;

        quote_spanned! {
            field_type.span() =>
            <#field_type as ::oath_parser::Parse>::parse_error()
        }
    });

    quote_spanned! {
        fields_span => {
            #(#members: #fields_parse_error,)*
        }
    }
}

pub fn parse_fields(fields: &Fields, fields_span: Span) -> TokenStream {
    let members = fields.members();

    let parse_fields = fields.iter().map(|field| {
        let field_type = &field.ty;

        quote_spanned! {
            field_type.span() =>
            <#field_type as oath_parser::Parse>::parse(parser)
        }
    });

    quote_spanned! {
        fields_span => {
            #(#members: #parse_fields,)*
        }
    }
}

pub fn detect_split_fields(
    fields: &Fields,
    fields_span: Span,
) -> Result<(Vec<&Field>, &Field, Vec<&Field>), TokenStream> {
    let mut fields_iter = fields.iter();
    let mut option_detect_fields = Vec::new();

    while let Some(field) = fields_iter.next() {
        if has_attr(&field.attrs, "option_detect") {
            option_detect_fields.push(field);
        } else {
            let detect_field = field;
            let secondary_fields = fields_iter.collect::<Vec<_>>();

            return Ok((option_detect_fields, detect_field, secondary_fields));
        }
    }

    Err(Error::new(fields_span, "expected detectable fields").to_compile_error())
}

pub fn detect_fields(fields: &Fields, fields_span: Span) -> TokenStream {
    let (option_detect_fields, detect_field, _secondary_fields) =
        match detect_split_fields(fields, fields_span) {
            Ok(ok) => ok,
            Err(error) => return error,
        };

    let option_detect_fields = option_detect_fields.into_iter().map(|field| {
        let field_type = &field.ty;

        quote_spanned! {
            field_type.span() =>
            <#field_type as ::oath_parser::OptionDetect>::option_detect(parser)
        }
    });

    let detect_field = {
        let field_type = &detect_field.ty;

        quote_spanned! {
            detect_field.span() =>
            <#field_type as ::oath_parser::OptionParse>::detect(parser)
        }
    };

    quote! {
        (#(#option_detect_fields ||)* #detect_field)
    }
}

pub fn option_detect_fields(fields: &Fields) -> TokenStream {
    let non_option_detect_errors = fields
        .iter()
        .filter(|field| !has_attr(&field.attrs, "option_detect"))
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

pub fn condition_parse_fields_if(fields: &Fields, fields_span: Span) -> TokenStream {
    let (option_detect_fields, detect_field) = 'find_fields: {
        let mut option_detect_fields = Vec::new();

        for field in fields {
            if has_attr(&field.attrs, "option_detect") {
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
            if has_attr(&field.attrs, "option_detect") {
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

fn has_attr<'a>(obj_attrs: impl IntoIterator<Item = &'a Attribute>, attr: &'a str) -> bool {
    obj_attrs
        .into_iter()
        .any(|obj_attr| obj_attr.path().is_ident(attr))
}
