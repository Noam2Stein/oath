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
    if fields_attrs.iter().any(|attr| attr.path().is_ident("framed")) {
        if fields.len() == 0 {
            return Error::new(fields_span, "`#[framed]` expects a frame field").into_compile_error();
        }

        return Error::new(fields_span, "`#[framed]` is not allowed in `Parse`").into_compile_error();
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
                    ParseExit::Complete => {},
                    ParseExit::Cut => {
                        #set_output;

                        break 'parse_fields ParseExit::Cut;
                    },
                }
            )*

            #set_output;

            ParseExit::Complete
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

pub fn option_parse_fields(
    fields: &Fields,
    fields_span: Span,
    fields_attrs: &[Attribute],
    fields_path: &TokenStream,
    output: &TokenStream,
) -> TokenStream {
    if fields_attrs.iter().any(|attr| attr.path().is_ident("framed")) {
        if fields.len() == 0 {
            return Error::new(fields_span, "`#[framed]` expects a frame field").into_compile_error();
        }

        let frame_type = &fields.iter().next().unwrap().ty;
        let frame_field_ident = fields.members().next().unwrap();

        let field_idents = fields.members().skip(1).collect::<Vec<_>>();

        let field_let_idents = fields
            .iter()
            .enumerate()
            .skip(1)
            .map(|(i, _)| format_ident!("field_{i}"))
            .collect::<Vec<_>>();

        let field_parse_errors = fields.iter().skip(1).map(field_parse_error).collect::<Vec<_>>();

        let parse_fields = fields
            .iter()
            .skip(1)
            .zip(&field_let_idents)
            .map(|(field, field_let_ident)| parse_field(field, &quote! { &mut #field_let_ident }))
            .collect::<Vec<_>>();

        return quote_spanned! {
            fields_span =>

            {
                let mut frame_output = None;

                let parse_exit = <#frame_type>::option_parse_frame(
                    parser,
                    &mut frame_output,
                    |parser|  {
                        #[allow(unused_parens)]
                        let (#(mut #field_let_idents), *) = (#(#field_parse_errors), *);

                        #[allow(unused_labels)]
                        let parse_exit = 'parse_fields: {
                            #(
                                match #parse_fields {
                                    ParseExit::Complete => {},
                                    ParseExit::Cut => {
                                        break 'parse_fields ParseExit::Cut;
                                    },
                                }
                            )*

                            ParseExit::Complete
                        };

                        ((#(#field_let_idents), *), parse_exit)
                    },
                    |parser|  {
                        #[allow(unused_parens)]
                        let (#(mut #field_let_idents), *) = (#(#field_parse_errors), *);

                        #[allow(unused_labels)]
                        let parse_exit = 'parse_fields: {
                            #(
                                match #parse_fields {
                                    ParseExit::Complete => {},
                                    ParseExit::Cut => {
                                        break 'parse_fields ParseExit::Cut;
                                    },
                                }
                            )*

                            ParseExit::Complete
                        };

                        ((#(#field_let_idents), *), parse_exit)
                    },
                );

                #[allow(unused_parens)]
                if let Some((frame, (#(#field_let_idents), *))) = frame_output {
                    *#output = Some(#fields_path {
                        #frame_field_ident: frame,
                        #(
                            #field_idents: #field_let_idents,
                        )*
                    });
                }

                parse_exit
            }
        };
    }

    if fields.len() == 0 {
        return quote_spanned! {
            fields_span =>

            {
                *#output = Some(#fields_path {});

                ParseExit::Complete
            }
        };
    }

    let detect = detect_fields(fields, fields_span, fields_attrs);

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

        { #[allow(unused_labels)] 'option_parse_fields: {
            if #detect == Detection::NotDetected {
                break 'option_parse_fields ParseExit::Complete;
            }

            let mut primary_field = None;
            let primary_field_exit = #option_parse_primary_field;
            let primary_field = primary_field.unwrap();

            #(
                let mut #secondary_field_let_idents = #secondary_field_parse_errors;
            )*

            if primary_field_exit == ParseExit::Cut {
                #set_output;

                break 'option_parse_fields ParseExit::Cut;
            }

            #(
                match #parse_secondary_fields {
                    ParseExit::Complete => {},
                    ParseExit::Cut => {
                        #set_output;

                        break 'option_parse_fields ParseExit::Cut;
                    },
                }
            )*

            #set_output;

            ParseExit::Complete
        }}
    }
}

pub fn detect_fields(fields: &Fields, fields_span: Span, fields_attrs: &[Attribute]) -> TokenStream {
    if fields_attrs.iter().any(|attr| attr.path().is_ident("framed")) {
        if fields.len() == 0 {
            return Error::new(fields_span, "`#[framed]` expects a frame field").into_compile_error();
        }

        let field_type = &fields.iter().next().unwrap().ty;

        return quote_spanned! {
            field_type.span() =>

            <#field_type>::detect_frame(parser)
        };
    }

    let detect_fields = fields.iter().map(detect_field);

    quote_spanned! {
        fields_span =>

        'detect_fields: {
            #(
                match #detect_fields {
                    Detection::Detected => break 'detect_fields Detection::Detected,
                    Detection::NotDetected => break 'detect_fields Detection::NotDetected,
                    Detection::EmptyDetected => {},
                }
            )*

            Detection::NotDetected
        }
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

    let parse = if let Some(attr) = field.attrs.iter().find(|attr| attr.path().is_ident("parse_as")) {
        let parse_type = attr
            .meta
            .require_list()
            .map(|list| list.tokens.clone())
            .unwrap_or_else(|err| err.into_compile_error());

        quote_spanned! {
            parse_type.span() => {
                let mut temp_output = <#parse_type as Parse>::parse_error();
                let exit = <#parse_type as Parse>::parse(parser, &mut temp_output);

                *#output = <#parse_type as Into<#field_type>>::into(temp_output);

                exit
            }
        }
    } else {
        quote_spanned! {
            field_type.span() =>

            <#field_type as Parse>::parse(parser, #output)
        }
    };

    let highlight = field.attrs.iter().find(|attr| attr.path().is_ident("highlight")).map(|attr| {
        let color = match attr.meta.require_list() {
            Ok(ok) => &ok.tokens,
            Err(err) => return err.into_compile_error(),
        };

        quote_spanned! {
            field_type.span() =>

            <#field_type as Highlightable>::highlight(#output, #color, &mut parser.highlights());
        }
    });

    quote_spanned! {
        field.span() =>

        {
            let exit = #parse;

            #highlight

            exit
        }
    }
}

fn field_parse_error(field: &Field) -> TokenStream {
    let field_type = &field.ty;

    if let Some(attr) = field.attrs.iter().find(|attr| attr.path().is_ident("parse_as")) {
        let parse_type = attr
            .meta
            .require_list()
            .map(|list| list.tokens.clone())
            .unwrap_or_else(|err| err.into_compile_error());

        quote_spanned! {
            parse_type.span() =>

            <#parse_type as Into<#field_type>>::into(<#parse_type as Parse>::parse_error())
        }
    } else {
        quote_spanned! {
            field_type.span() =>

            <#field_type as Parse>::parse_error()
        }
    }
}

// OPTION PARSE

fn option_parse_field(field: &Field, output: &TokenStream) -> TokenStream {
    let field_type = &field.ty;

    let parse = if let Some(attr) = field.attrs.iter().find(|attr| attr.path().is_ident("parse_as")) {
        let parse_type = attr
            .meta
            .require_list()
            .map(|list| list.tokens.clone())
            .unwrap_or_else(|err| err.into_compile_error());

        quote_spanned! {
            parse_type.span() => {
                let mut temp_output = None;
                let exit = <#parse_type as OptionParse>::option_parse(parser, &mut temp_output);

                *#output = temp_output.map(|temp| <#parse_type as Into<#field_type>>::into(temp));

                exit
            }
        }
    } else {
        quote_spanned! {
            field_type.span() =>

            <#field_type as OptionParse>::option_parse(parser, #output)
        }
    };

    let highlight = field.attrs.iter().find(|attr| attr.path().is_ident("highlight")).map(|attr| {
        let color = match attr.meta.require_list() {
            Ok(ok) => &ok.tokens,
            Err(err) => return err.into_compile_error(),
        };

        quote_spanned! {
            field_type.span() =>

            <Option<#field_type> as Highlightable>::highlight(#output, #color, &mut parser.highlights());
        }
    });

    quote_spanned! {
        field.span() =>

        {
            let exit = #parse;

            #highlight

            exit
        }
    }
}

fn detect_field(field: &Field) -> TokenStream {
    if let Some(attr) = field.attrs.iter().find(|attr| attr.path().is_ident("parse_as")) {
        let parse_type = attr
            .meta
            .require_list()
            .map(|list| list.tokens.clone())
            .unwrap_or_else(|err| err.into_compile_error());

        quote_spanned! {
            parse_type.span() =>

            <#parse_type as OptionParse>::detect(parser)
        }
    } else {
        let field_type = &field.ty;

        quote_spanned! {
            field_type.span() =>

            <#field_type as OptionParse>::detect(parser)
        }
    }
}
