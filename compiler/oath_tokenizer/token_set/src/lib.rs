use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::Ident;

macro_rules! define_token_set {
    {
        keywords: [$($keyword:ident), * $(,)?],
        delims: [$($delim_open:literal $delim_close:literal $delim_type:ident), * $(,)?],
        puncts: [$($punct:literal $punct_variant:ident), * $(,)?] $(,)?
    } => {
        const KEYWORDS: &[&str] = &[$(stringify!($keyword)), *];

        const DELIMS: &[DelimInfo] = &[$(
            DelimInfo { delim_open: $delim_open, delim_close: $delim_close, delim_type: stringify!($delim_type) },
        )*];

        const PUNCTS: &[PunctInfo] = &[$(
            PunctInfo { punct: $punct, punct_variant: stringify!($punct_variant) }
        ), *];
    };
}

// TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS
// TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS
// TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS
// TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS
// TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS
// TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS
// TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS
// TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS
// TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS
define_token_set!(
    keywords: [
        mod, use, pub, package, super, sys, impl,
        trait, promise, require, neg,
        type, alias, struct, enum, untagged, val, is,
        fn, raw, con, async, panic, lock, undef,
        macro,
        const, static,
        var, mut, smut, excl,
        self, Self,
        assume,
        if, else, match,
        loop, while, for,
        return, break, continue,
    ],
    delims: [
        "(" ")" Parens,
        "[" "]" Brackets,
        "{" "}" Braces,
        "<#" "#>" Angles,
    ],
    puncts: [
        ">>=" ShiftRAssign,
        "<<=" ShiftLAssign,
        "..." DotDotDot,
        "&&" AndAnd,
        "||" OrOr,
        "<<" ShiftL,
        ">>" ShiftR,
        "+=" PlusAssign,
        "-=" MinusAssign,
        "*=" StarAssign,
        "/=" SlashAssign,
        "%=" PercentAssign,
        "&=" AndAssign,
        "|=" OrAssign,
        "^=" CaretAssign,
        "==" EqEq,
        "<=" LessEq,
        ">=" MoreEq,
        "!=" NotEq,
        "::" ColonColon,
        ".." DotDot,
        "->" ArrowRight,
        "+" Plus,
        "-" Minus,
        "*" Star,
        "/" Slash,
        "%" Percent,
        "=" Eq,
        "<" Less,
        ">" More,
        "&" And,
        "|" Or,
        "^" Caret,
        "~" Tilde,
        "?" Question,
        "$" Dollar,
        "," Comma,
        ";" Semi,
        ":" Colon,
        "." Dot,
        "`" Backtick,
        "#" Hash,
        "!" Exclamation,
    ],
);
// TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS
// TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS
// TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS
// TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS
// TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS
// TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS
// TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS
// TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS
// TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS TOKENS

/// provides meta info about all Oath keywords, delimiters, and puncts with `$()*` + `$info` syntax.
///
/// `$keyword:literal`, `$keyword_len:literal`, `$keyword_type:ident`, `$keyword_variant:ident`, "$keyword_category:ident"
///
/// `$punct:literal`, `$punct_type:ident`, `$punct_variant:ident`
///
/// `$delim_open:literal`, `$delim_close:literal`, `$delim_type:ident`, `$delim_fn:ident`, `$delim_open_type`, `$delim_close_type`
#[proc_macro]
pub fn with_token_set(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);

    let keywords = KEYWORDS.into_iter().map(|keyword| {
        let keyword_type = Ident::new(keyword_to_type(&keyword).as_str(), Span::call_site());

        let keyword_variant = Ident::new(keyword_to_variant(&keyword).as_str(), Span::call_site());

        quote! {
            #keyword #keyword_type #keyword_variant
        }
    });

    let delims = DELIMS.into_iter().map(
        |DelimInfo {
             delim_open,
             delim_close,
             delim_type,
         }| {
            let delim_fn = Ident::new(&delim_type.to_lowercase(), Span::call_site());

            let delim_type = format_ident!("{delim_type}");

            let delim_open_type = format_ident!("{delim_type}Open");

            let delim_close_type = format_ident!("{delim_type}Close");

            quote! {
                #delim_open #delim_close #delim_type #delim_fn #delim_open_type #delim_close_type
            }
        },
    );

    let puncts = PUNCTS.into_iter().map(
        |PunctInfo {
             punct,
             punct_variant,
         }| {
            let punct_type = format_ident!("{punct_variant}Punct");

            let punct_variant = format_ident!("{punct_variant}");

            quote! {
                #punct #punct_type #punct_variant
            }
        },
    );

    quote! {
        macro_rules! chemical_plant_act_2 {
            {
                $($keyword:literal $keyword_type:ident $keyword_variant:ident), *;

                $($delim_open:literal $delim_close:literal $delim_type:ident $delim_fn:ident $delim_open_type:ident $delim_close_type:ident), *;

                $($punct:literal $punct_type:ident $punct_variant:ident), *;
            } => {
                #input
            }
        }
        chemical_plant_act_2! {
            #(#keywords), *;

            #(#delims), *;

            #(#puncts), *;
        }
    }.into()
}

#[proc_macro]
pub fn with_token_set_expr(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let with_token_set = TokenStream::from(with_token_set(input));

    quote! {
        { #with_token_set }
    }
    .into()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct PunctInfo {
    pub punct: &'static str,
    pub punct_variant: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct DelimInfo {
    pub delim_open: &'static str,
    pub delim_close: &'static str,
    pub delim_type: &'static str,
}

fn keyword_to_variant(keyword: &str) -> String {
    match keyword {
        "self" => "LowercaseSelf".to_string(),
        "Self" => "UppercaseSelf".to_string(),
        _ => keyword
            .chars()
            .enumerate()
            .map(|(char_index, char)| {
                if char_index == 0 {
                    char.to_ascii_uppercase()
                } else {
                    char
                }
            })
            .collect(),
    }
}
fn keyword_to_type(keyword: &str) -> String {
    match keyword {
        "self" => "LowercaseSelf".to_string(),
        "Self" => "UppercaseSelf".to_string(),
        _ => keyword
            .chars()
            .enumerate()
            .map(|(char_index, char)| {
                if char_index == 0 {
                    char.to_ascii_uppercase()
                } else {
                    char
                }
            })
            .chain("Keyword".chars())
            .collect(),
    }
}
