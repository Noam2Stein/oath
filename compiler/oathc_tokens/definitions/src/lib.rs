use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::Ident;

macro_rules! define_tokens {
    {
        blue_keywords: [$($blue_keyword:ident), * $(,)?],
        pink_keywords: [$($pink_keyword:ident), * $(,)?],
        delims: [$($delim_open:literal $delim_close:literal $delim_name:ident), * $(,)?],
        puncts: [$($punct:literal $punct_variant:ident), * $(,)?] $(,)?
    } => {
        const KEYWORDS: &[&str] = &[$(stringify!($blue_keyword),)* $(stringify!($pink_keyword)), *];
        const BLUE_KEYWORDS: &[&str] = &[$(stringify!($blue_keyword)), *];
        const PINK_KEYWORDS: &[&str] = &[$(stringify!($pink_keyword)), *];

        const DELIMS: &[DelimInfo] = &[$(
            DelimInfo { delim_open: $delim_open, delim_close: $delim_close, delim_name: stringify!($delim_name) },
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
define_tokens!(
    blue_keywords: [
        parent, mod, use, declare, static, type, struct, enum, trait, impl, sys, fn,
        pub, open, raw, con, async, panic, lock, undef, runtime, comptime,
        promise, require, untagged, val, is,
        mut, sole, smut,
        try, self, Self, out, let, run,
    ],
    pink_keywords: [
        if, else, then, match,
        loop, while, until, for, in,
        eval, return, break, continue,
        assume,
    ],
    delims: [
        "(" ")" Paren,
        "[" "]" Bracket,
        "{" "}" Brace,
    ],
    puncts: [
        ">>=" ShiftRAssign,
        "<<=" ShiftLAssign,
        "..=" DotDotEq,
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
        "!=" NotEq,
        "::" ColonColon,
        ".." DotDot,
        "->" ArrowRight,
        "<=" LessEq,
        ">=" MoreEq,
        "#!" HashExclamation,
        "<" Less,
        ">" More,
        "+" Plus,
        "-" Minus,
        "*" Star,
        "/" Slash,
        "%" Percent,
        "=" Eq,
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
        "'" Apostrophe,
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
/// `$keyword:literal`, `$keyword_type:ident`, `$keyword_variant:ident`
/// `$blue_keyword:literal`, `$blue_keyword_type:ident`, `$blue_keyword_variant:ident`
/// `$pink_keyword:literal`, `$pink_keyword_type:ident`, `$pink_keyword_variant:ident`
///
/// `$punct:literal`, `$punct_type:ident`, `$punct_variant:ident`
///
/// `$delim_open:literal`, `$delim_close:literal`
/// `$delim_type:ident`, `$delim_open_type:ident`, `$delim_close_type:ident`
/// `$delims_fn:ident`, `$delim_fn:ident`
#[proc_macro]
pub fn with_tokens(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);

    let keywords = KEYWORDS.into_iter().map(|keyword| {
        let keyword_type = Ident::new(keyword_to_type(&keyword).as_str(), Span::call_site());

        let keyword_variant = Ident::new(keyword_to_variant(&keyword).as_str(), Span::call_site());

        quote! {
            #keyword #keyword_type #keyword_variant
        }
    });

    let blue_keywords = BLUE_KEYWORDS.into_iter().map(|keyword| {
        let keyword_type = Ident::new(keyword_to_type(&keyword).as_str(), Span::call_site());

        let keyword_variant = Ident::new(keyword_to_variant(&keyword).as_str(), Span::call_site());

        quote! {
            #keyword #keyword_type #keyword_variant
        }
    });

    let pink_keywords = PINK_KEYWORDS.into_iter().map(|keyword| {
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
             delim_name,
         }| {
            let delims_type = format_ident!("{delim_name}s");
            let delim_type = format_ident!("{delim_name}");
            let delim_open_type = format_ident!("Open{delim_name}");
            let delim_close_type = format_ident!("Close{delim_name}");

            let delim_fn = Ident::new(&delim_name.to_lowercase(), Span::call_site());
            let delims_fn = Ident::new(&format!("{}s", delim_name.to_lowercase()), Span::call_site());

            quote! {
                #delim_open #delim_close
                #delims_type #delim_type #delim_open_type #delim_close_type
                #delims_fn #delim_fn
            }
        },
    );

    let puncts = PUNCTS.into_iter().map(|PunctInfo { punct, punct_variant }| {
        let punct_type = format_ident!("{punct_variant}Punct");

        let punct_variant = format_ident!("{punct_variant}");

        quote! {
            #punct #punct_type #punct_variant
        }
    });

    quote! {
        macro_rules! chemical_plant_act_2 {
            {
                $($keyword:literal $keyword_type:ident $keyword_variant:ident), *;
                $($blue_keyword:literal $blue_keyword_type:ident $blue_keyword_variant:ident), *;
                $($pink_keyword:literal $pink_keyword_type:ident $pink_keyword_variant:ident), *;

                $(
                    $delim_open:literal $delim_close:literal
                    $delims_type:ident $delim_type:ident $delim_open_type:ident $delim_close_type:ident
                    $delims_fn:ident $delim_fn:ident
                ), *;

                $($punct:literal $punct_type:ident $punct_variant:ident), *;
            } => {
                #input
            }
        }
        chemical_plant_act_2! {
            #(#keywords), *;
            #(#blue_keywords), *;
            #(#pink_keywords), *;

            #(#delims), *;

            #(#puncts), *;
        }
    }
    .into()
}

#[proc_macro]
pub fn with_tokens_expr(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let with_token_set = TokenStream::from(with_tokens(input));

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
    pub delim_name: &'static str,
}

fn keyword_to_variant(keyword: &str) -> String {
    match keyword {
        "self" => "LowercaseSelf".to_string(),
        "Self" => "UppercaseSelf".to_string(),
        _ => keyword
            .chars()
            .enumerate()
            .map(
                |(char_index, char)| {
                    if char_index == 0 { char.to_ascii_uppercase() } else { char }
                },
            )
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
            .map(
                |(char_index, char)| {
                    if char_index == 0 { char.to_ascii_uppercase() } else { char }
                },
            )
            .chain("Keyword".chars())
            .collect(),
    }
}
