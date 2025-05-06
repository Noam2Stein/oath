use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, quote_spanned};
use syn::{Attribute, Error, Field, Fields, spanned::Spanned};

// PARSE

pub fn parse_fields(
    fields: &Fields,
    fields_span: Span,
    fields_attrs: &[Attribute],
    fields_path: &TokenStream,
    output: &TokenStream,
) -> TokenStream {
    if fields_attrs.iter().any(|attr| attr.path().is_ident("group")) {
        return Error::new(fields_span, "`#[group]` is not allowed in `Parse`").into_compile_error();
    }

    let field_idents = fields.members().collect::<Vec<_>>();

    let field_let_idents = fields
        .iter()
        .enumerate()
        .map(|(i, _)| format_ident!("field_{i}"))
        .collect::<Vec<_>>();

    let field_parse_errors = fields.iter().map(field_parse_error);

    let parse_fields = fields
        .iter()
        .zip(&field_let_idents)
        .map(|(field, field_let_ident)| parse_field(field, &quote! { &mut #field_let_ident }));

    let set_output = quote! {
        *#output = #fields_path {#(
            #field_idents: #field_let_idents,
        )*}
    };

    quote_spanned! {
        fields_span =>

        { #[allow(unused_labels)] 'parse_fields: {
            #(
                let mut #field_let_idents = #field_parse_errors;
            )*

            #(
                match #parse_fields {
                    ::oath_parser::ParseExit::Complete => {},
                    ::oath_parser::ParseExit::Cut => {
                        #set_output;

                        break 'parse_fields ::oath_parser::ParseExit::Cut;
                    },
                }
            )*

            #set_output;

            ::oath_parser::ParseExit::Complete
        }}
    }
}

pub fn fields_parse_error(fields: &Fields, fields_span: Span, fields_path: &TokenStream) -> TokenStream {
    let field_idents = fields.members();

    let fields_parse_error = fields.iter().map(field_parse_error);

    quote_spanned! {
        fields_span =>

        #fields_path {
            #(#field_idents: #fields_parse_error,)*
        }
    }
}

// OPTION PARSE

pub fn option_parse_fields(fields: &Fields, fields_span: Span, fields_path: &TokenStream, output: &TokenStream) -> TokenStream {
    if fields.len() == 0 {
        return Error::new(fields_span, "`OptionParse` requires at least one field").into_compile_error();
    }

    let primary_field = fields.iter().next().unwrap();
    let primary_field_ident = &fields.members().next().unwrap();
    let option_parse_primary_field = option_parse_field(primary_field, &quote! { &mut primary_field });

    let secondary_field_idents = fields.members().skip(1).collect::<Vec<_>>();

    let secondary_field_let_idents = fields
        .iter()
        .skip(1)
        .enumerate()
        .map(|(i, _)| format_ident!("secondary_field_{i}"))
        .collect::<Vec<_>>();

    let secondary_field_parse_errors = fields.iter().skip(1).map(field_parse_error);

    let parse_secondary_fields = fields
        .iter()
        .skip(1)
        .zip(&secondary_field_let_idents)
        .map(|(field, field_let_ident)| parse_field(field, &quote! { &mut #field_let_ident }));

    let set_output = quote! {
        *#output = Some(#fields_path {
            #primary_field_ident: primary_field,
            #(
                #secondary_field_idents: #secondary_field_let_idents,
            )*
        })
    };

    quote_spanned! {
        fields_span =>

        { #[allow(unused_labels)] 'parse_fields: {
            let mut primary_field = None;

            let primary_field_exit = #option_parse_primary_field;

            let primary_field = match primary_field {
                Some(primary_field) => primary_field,
                None => break 'parse_fields primary_field_exit,
            };

            #(
                let mut #secondary_field_let_idents = #secondary_field_parse_errors;
            )*

            if primary_field_exit == ::oath_parser::ParseExit::Cut {
                #set_output;

                break 'parse_fields ::oath_parser::ParseExit::Cut;
            }

            #(
                match #parse_secondary_fields {
                    ::oath_parser::ParseExit::Complete => {},
                    ::oath_parser::ParseExit::Cut => {
                        #set_output;

                        break 'parse_fields ::oath_parser::ParseExit::Cut;
                    },
                }
            )*

            #set_output;

            ::oath_parser::ParseExit::Complete
        }}
    }
}

pub fn detect_fields(fields: &Fields, fields_span: Span) -> TokenStream {
    if fields.len() == 0 {
        return Error::new(fields_span, "`OptionParse` requires at least one field").into_compile_error();
    }

    let primary_field = fields.iter().next().unwrap();
    let detect_primary_field = detect_field(primary_field);

    quote! {
        #detect_primary_field
    }
}

//
//
// HELPERS
//
//

// PARSE

fn parse_field(field: &Field, output: &TokenStream) -> TokenStream {
    let field_type = &field.ty;

    if field.attrs.iter().any(|attr| attr.path().is_ident("rep")) {
        return quote_spanned! {
            field.span() =>

            parser.parse_rep(#output)
        };
    }

    if field.attrs.iter().any(|attr| attr.path().is_ident("trl")) {
        return quote_spanned! {
            field.span() =>

            parser.parse_trl(#output)
        };
    }

    if field.attrs.iter().any(|attr| attr.path().is_ident("sep")) {
        return quote_spanned! {
            field.span() =>

            parser.try_parse_sep(#output)
        };
    }

    quote_spanned! {
        field.span() =>

        <#field_type as ::oath_parser::Parse>::parse(parser, #output)
    }
}

fn field_parse_error(field: &Field) -> TokenStream {
    if field.attrs.iter().any(|attr| attr.path().is_ident("rep")) {
        return quote_spanned! {
            field.span() =>

            Vec::new()
        };
    }

    if field.attrs.iter().any(|attr| attr.path().is_ident("trl")) {
        return quote_spanned! {
            field.span() =>

            Vec::new()
        };
    }

    if field.attrs.iter().any(|attr| attr.path().is_ident("sep")) {
        return quote_spanned! {
            field.span() =>

            ::oath_parser::Try::Failure
        };
    }

    let field_type = &field.ty;

    quote_spanned! {
        field.span() =>

        <#field_type as ::oath_parser::Parse>::parse_error()
    }
}

// OPTION PARSE

fn option_parse_field(field: &Field, output: &TokenStream) -> TokenStream {
    if field.attrs.iter().any(|attr| attr.path().is_ident("rep")) {
        return Error::new(field.span(), "`#[rep] is not `OptionParse`").into_compile_error();
    }

    if field.attrs.iter().any(|attr| attr.path().is_ident("trl")) {
        return Error::new(field.span(), "`#[trl] is not `OptionParse`").into_compile_error();
    }

    if field.attrs.iter().any(|attr| attr.path().is_ident("sep")) {
        return quote_spanned! {
            field.span() =>

            parser.option_parse_sep(#output)
        };
    }

    let field_type = &field.ty;

    quote_spanned! {
        field.span() =>

        <#field_type as ::oath_parser::OptionParse>::option_parse(parser, #output)
    }
}

fn detect_field(field: &Field) -> TokenStream {
    let field_type = &field.ty;

    quote_spanned! {
        field.span() =>

        <#field_type as ::oath_parser::OptionParse>::detect(parser)
    }
}
