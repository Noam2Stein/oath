use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Ident, LitInt, LitStr};

macro_rules! define_keywords {
    ($($category:ident: [ $($str:ident), * $(,)? ]), * $(,)?) => {
        const KEYWORD_CATEGORIES: &[&str] = &[$(stringify!($category)), *];

        const KEYWORDS: &[KeywordInfo] = &[$(
            $(
                KeywordInfo { str: stringify!($str), category: stringify!($category) },
            )*
        )*];
    };
}
macro_rules! define_puncts {
    ($($str:literal $name:ident), * $(,)?) => {
        const PUNCTS: &[PunctInfo] = &[$(
            PunctInfo { str: $str, name: stringify!($name) }
        ), *];
    };
}
macro_rules! define_delimiters {
    ($($open_str:literal $close_str:literal $ty:ident), * $(,)?) => {
        const DELIMITERS: &[DelimiterInfo] = &[$(
            DelimiterInfo { open_str: $open_str, close_str: $close_str, ty: stringify!($ty) }
        ), *];
    }
}

define_keywords!(
    Other: [
        mod, use, pub, package, super,
        trait, promise, require,
        type, struct, union, untagged, val,
        fn, raw, con, async,
        macro,
        const, static,
        var, mut, smut, excl,
    ],
    Flow: [
        assume,
        if, else, match,
        return, break, continue,
    ],
);
define_puncts!(
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
    "::" ColonColon,
    ".." DotDot,
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
    "#" Num,
);
define_delimiters!(
    "(" ")" Parens,
    "[" "]" Brackets,
    "{" "}" Braces,
    "<#" "#>" Angles,
);

/// provides meta info about all Oath keywords with `$()*` + `$info` syntax.
///
/// `$keyword:ident`, `$keyword_len:literal`, `$keyword_type:ident`, `$keyword_variant:ident`, `$keyword_category:ident`
#[proc_macro]
pub fn with_keywords(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);

    let macro_input = KEYWORDS.into_iter().map(|keyword_info| {
        let keyword = Ident::new(&keyword_info.str, Span::call_site());

        let keyword_len = LitInt::new(
            keyword_info.str.len().to_string().as_str(),
            Span::call_site(),
        );

        let keyword_type = Ident::new(
            keyword_to_type(keyword_info.str).as_str(),
            Span::call_site(),
        );

        let keyword_variant = Ident::new(
            keyword_to_variant(keyword_info.str).as_str(),
            Span::call_site(),
        );

        let keyword_category = Ident::new(
            keyword_to_variant(&keyword_info.category).as_str(),
            Span::call_site(),
        );

        quote! {
            #keyword #keyword_len #keyword_type #keyword_variant #keyword_category,
        }
    });

    quote! {
        macro_rules! collage_of_water {
            ($($keyword:ident $keyword_len:literal $keyword_type:ident $keyword_variant:ident $keyword_category:ident, )*) => {
                #input
            }
        }
        collage_of_water! {
            #(#macro_input)*
        }
    }.into()
}

/// provides meta info about all Oath keyword categories with `$()*` + `$info` syntax.
///
/// `$category:ident`
#[proc_macro]
pub fn with_keyword_categories(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);

    let macro_input = KEYWORD_CATEGORIES.into_iter().map(|category| {
        let category = Ident::new(keyword_to_variant(category).as_str(), Span::call_site());

        quote! {
            #category,
        }
    });

    quote! {
        macro_rules! olino {
            ($($category:ident, )*) => {
                #input
            }
        }
        olino! {
            #(#macro_input)*
        }
    }
    .into()
}

/// provides meta info about all Oath puncts with `$()*` + `$info` syntax.
///
/// `$punct:literal`, `$punct_len:literal`, `$punct_type:ident`, `$punct_variant:ident`
#[proc_macro]
pub fn with_puncts(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);

    let macro_input = PUNCTS.into_iter().map(|punct_info| {
        let punct = LitStr::new(&punct_info.str, Span::call_site());

        let punct_len = LitInt::new(punct_info.str.len().to_string().as_str(), Span::call_site());

        let punct_type = Ident::new(
            format!("{}Punct", punct_info.name).as_str(),
            Span::call_site(),
        );

        let punct_variant = Ident::new(punct_info.name, Span::call_site());

        quote! {
            #punct #punct_len #punct_type #punct_variant,
        }
    });

    quote! {
        macro_rules! double_market {
            ($($punct:literal $punct_len:literal $punct_type:ident $punct_variant:ident, )*) => {
                #input
            }
        }
        double_market! {
            #(#macro_input)*
        }
    }
    .into()
}

/// provides meta info about all Oath delimiters with `$()*` + `$info` syntax.
///
/// `$open_delim:literal`, `$close_delim:literal`, `$delim_type:ident`, `$delim_fn:ident`
#[proc_macro]
pub fn with_delimiters(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);

    let macro_input = DELIMITERS.into_iter().map(|delim_info| {
        let open_delim = LitStr::new(&delim_info.open_str, Span::call_site());

        let close_delim = LitStr::new(&delim_info.close_str, Span::call_site());

        let delim_type = Ident::new(&delim_info.ty, Span::call_site());

        let delim_fn = Ident::new(&delim_info.ty.to_lowercase(), Span::call_site());

        quote! {
            #open_delim #close_delim #delim_type #delim_fn,
        }
    });

    quote! {
        macro_rules! why {
            ($($open_delim:literal $close_delim:literal $delim_type:ident $delim_fn:ident, )*) => {
                #input
            }
        }
        why! {
            #(#macro_input)*
        }
    }
    .into()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct KeywordInfo {
    pub str: &'static str,
    pub category: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct PunctInfo {
    pub str: &'static str,
    pub name: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct DelimiterInfo {
    pub open_str: &'static str,
    pub close_str: &'static str,
    pub ty: &'static str,
}

fn keyword_to_variant(keyword: &str) -> String {
    keyword
        .chars()
        .enumerate()
        .map(|(char_index, char)| {
            if char_index == 0 {
                char.to_ascii_uppercase()
            } else {
                char
            }
        })
        .collect()
}
fn keyword_to_type(keyword: &str) -> String {
    keyword
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
        .collect()
}
