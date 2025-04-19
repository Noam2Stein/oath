use proc_macro2::{Literal, Span, TokenStream};
use quote::{ToTokens, quote, quote_spanned};
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

pub fn split_optional_fields(fields: &Fields) -> (Vec<&Field>, Option<&Field>, Vec<&Field>) {
    let mut fields_iter = fields.iter().peekable();

    let mut dont_parse_fields = Vec::with_capacity(fields.len());

    while fields_iter.peek().map_or(false, |field| {
        field
            .attrs
            .iter()
            .any(|attr| attr.path().is_ident("dont_parse"))
    }) {
        dont_parse_fields.push(fields_iter.next().unwrap());
    }

    let peek_field = fields_iter.next();

    let secondary_fields = fields_iter.collect::<Vec<_>>();

    (dont_parse_fields, peek_field, secondary_fields)
}

pub fn option_parse_fields(fields: &Fields) -> TokenStream {
    let (dont_parse_fields, peek_field, secondary_fields) = split_optional_fields(fields);

    let peek_field = match peek_field {
        Some(peek_field) => peek_field,
        None => {
            return quote! {
                None
            };
        }
    };

    let dont_parse_field_values = dont_parse_fields.iter().map(|field| {
        let field_type = &field.ty;

        quote! {
            <#field_type as ::oath_parser::Parse>::parse_error()
        }
    });

    let peek_field_type = &peek_field.ty;

    let secondary_field_idents = secondary_fields
        .iter()
        .zip((dont_parse_fields.len() + 1)..)
        .map(|(field, field_index)| {
            field.ident.as_ref().map_or_else(
                || Literal::usize_unsuffixed(field_index).into_token_stream(),
                |ident| ident.to_token_stream(),
            )
        })
        .collect::<Vec<_>>();
    let secondary_field_types = secondary_fields.iter().map(|field| &field.ty);
    let secondary_field_init_values = secondary_fields.iter().map(|field| {
        let field_type = &field.ty;

        quote! {
            <#field_type as ::oath_parser::Parse>::parse_error()
        }
    });

    let break_fields = quote! {
        (#(#dont_parse_field_values, )* peek_field, #(#secondary_field_idents), *)
    };

    quote! { 'optional_fields: {
        let mut peek_field = None;
        let peek_field_exit = <#peek_field_type as ::oath_parser::OptionParse>::option_parse(parser, &mut peek_field);

        let peek_field = match peek_field {
            Some(peek_field) => peek_field,
            None => break 'optional_fields None,
        };

        #(
            let mut #secondary_field_idents = #secondary_field_init_values;
        )*

        if peek_field_exit == ::oath_parser::ParseExit::Cut {
            break 'optional_fields Some((#break_fields, ::oath_parser::ParseExit::Cut));
        }

        #(
            match <#secondary_field_types as ::oath_parser::Parse>::parse(parser, &mut #secondary_field_idents) {
                ::oath_parser::ParseExit::Complete => {},
                ::oath_parser::ParseExit::Cut => break 'optional_fields Some((#break_fields, ::oath_parser::ParseExit::Cut)),
            }
        )*

        Some((#break_fields, ::oath_parser::ParseExit::Complete))
    }}
}

fn parse_field(field: &Field) -> TokenStream {
    let field_type = &field.ty;

    quote_spanned! {
        field.span() =>

        <#field_type as ::oath_parser::Parse>::parse(parser)
    }
}
