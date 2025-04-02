use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, format_ident, quote, quote_spanned};
use syn::{
    Attribute, Data, DataEnum, DataStruct, DeriveInput, Error, Expr, Fields, GenericParam, Ident,
    LitInt, Token, parse_macro_input, parse_quote, punctuated::Punctuated, spanned::Spanned,
};

#[proc_macro_derive(InternedDisplay, attributes(display))]
pub fn derive_interned_display(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        attrs,
        vis: _,
        ident,
        mut generics,
        data,
    } = parse_macro_input!(input as DeriveInput);

    for param in &mut generics.params {
        if let GenericParam::Type(type_param) = param {
            type_param.bounds.push(parse_quote!(::oath_src::Spanned));
        }
    }

    let output_expr = match data {
        Data::Struct(data) => struct_span(&data, &attrs),
        Data::Enum(data) => enum_span(&data),
        Data::Union(_) => Err(Error::new(
            Span::call_site(),
            "`InternedDisplay` cannot be derived for unions",
        )),
    };

    let output_expr = match output_expr {
        Ok(ok) => ok,
        Err(err) => err.to_compile_error(),
    };

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics ::oath_interner::InternedDisplay for #ident #ty_generics #where_clause {
            fn interned_fmt(&self, f: &mut ::std::fmt::Formatter, interner: &::oath_interner::Interner) -> ::std::fmt::Result {
                #output_expr
            }
        }
    }
    .into()
}

fn struct_span(data: &DataStruct, attrs: &Vec<Attribute>) -> Result<TokenStream, Error> {
    fields_span(
        &data.fields,
        attrs,
        |field_ident, field_index| {
            if field_ident.is_some() {
                quote_spanned! { field_ident.span() => self.#field_ident }
            } else {
                let field_ident =
                    LitInt::new(&field_index.to_string(), field_ident.span()).to_token_stream();

                quote_spanned! { field_ident.span() => self.#field_ident }
            }
        },
        Span::call_site(),
    )
}

fn enum_span(data: &DataEnum) -> Result<TokenStream, Error> {
    let match_variants = data.variants.iter().map(|variant| {
        let variant_ident = &variant.ident;

        let original_field_idents = variant.fields.iter().map(|field| &field.ident);

        let field_idents =
            (0..variant.fields.len()).map(|field_index| format_ident!("field_{field_index}"));

        let output_expr = match fields_span(
            &variant.fields,
            &variant.attrs,
            |_, field_index| format_ident!("field_{field_index}").to_token_stream(),
            variant.span(),
        ){
            Ok(ok) => ok,
            Err(err) => err.to_compile_error(),
        };

        match variant.fields {
            Fields::Named(_) => quote_spanned! {
                variant.span() =>

                Self::#variant_ident { #(#original_field_idents: #field_idents), * } => #output_expr,
            },
            Fields::Unnamed(_) => quote_spanned! {
                variant.span() =>

                Self::#variant_ident(#(#field_idents), *) => #output_expr,
            },
            Fields::Unit => quote_spanned! {
                variant.span() =>

                Self::#variant_ident => #output_expr,
            },
        }
    });

    Ok(quote! {
        match self {
            #(#match_variants)*
        }
    })
}

fn fields_span(
    fields: &Fields,
    fields_attrs: &Vec<Attribute>,
    get_field_path: impl Fn(Option<&Ident>, usize) -> TokenStream,
    fields_span: Span,
) -> Result<TokenStream, Error> {
    let (display_attr, display_attr_errors) = {
        let mut display_attr_iter = fields_attrs
            .iter()
            .filter(|attr| attr.path().is_ident("display"));
        let output = display_attr_iter.next();

        let display_attr_errors = display_attr_iter.map(|attr| {
            Error::new(attr.span(), "multiple `#[display(...)]` attributes").to_compile_error()
        });

        (output, display_attr_errors)
    };

    if let Some(display_attr) = display_attr {
        let list = display_attr.meta.require_list()?;

        let mut items = list
            .parse_args_with(Punctuated::<Expr, Token![,]>::parse_terminated)?
            .into_iter();

        let literal = items.next();

        Ok(quote_spanned! {
            fields_span =>

            write!(f, #literal, #(::oath_interner::Interned(&#items, interner)), *)
        })
    } else if fields.len() != 1 {
        Err(Error::new(
            fields_span,
            "expected a single field or `#[display(...)]`",
        ))
    } else {
        let field = fields.iter().next().unwrap();

        let field_type = &field.ty;
        let field_path = get_field_path(field.ident.as_ref(), 0);

        Ok(quote_spanned! {
            field_type.span() =>

            {
                #(#display_attr_errors;)*

                <#field_type as ::oath_interner::InternedDisplay>::interned_fmt(#field_path, f, interner)
            }
        })
    }
}
