use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{Error, Field, Fields, spanned::Spanned};

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

    let parse_fields = fields.iter().map(parse_field);

    quote_spanned! {
        fields_span => {
            #(#members: #parse_fields,)*
        }
    }
}

pub fn detect_fields(fields: &Fields, fields_span: Span) -> TokenStream {
    let first_field = match fields.iter().next() {
        Some(some) => some,
        None => return Error::new(fields_span, "cannot detect zero fields").into_compile_error(),
    };

    let detect_first_field = {
        let field_type = &first_field.ty;

        quote_spanned! {
            first_field.span() =>

            <#field_type as ::oath_parser::OptionParse>::detect(parser)
        }
    };

    quote! {
        #detect_first_field
    }
}

pub fn option_parse_fields(
    fields: &Fields,
    fields_span: Span,
    value_from_fields: impl FnOnce(TokenStream) -> TokenStream,
) -> TokenStream {
    let first_field = match fields.iter().next() {
        Some(some) => some,
        None => return Error::new(fields_span, "cannot detect zero fields").into_compile_error(),
    };

    let option_parse_first_field = option_parse_field(first_field);
    let parse_other_fields = fields.iter().skip(1).map(parse_field);

    let (first_member, other_members) = {
        let mut members_iter = fields.members().into_iter();
        let first_member = members_iter.next();

        (first_member, members_iter)
    };

    let fields = quote_spanned! {
        fields_span => {
            #first_member: first_field,
            #(#other_members: #parse_other_fields,)*
        }
    };

    let value = value_from_fields(fields);

    quote! {
        (if let Some(first_field) = #option_parse_first_field {
            Some(#value)
        } else {
            None
        })
    }
}

fn parse_field(field: &Field) -> TokenStream {
    let field_type = &field.ty;

    quote_spanned! {
        field.span() =>

        <#field_type as ::oath_parser::Parse>::parse(parser)
    }
}

fn option_parse_field(field: &Field) -> TokenStream {
    let field_type = &field.ty;

    quote_spanned! {
        field.span() =>

        <#field_type as ::oath_parser::OptionParse>::option_parse(parser)
    }
}
