#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeywordInfo {
    pub str: &'static str,
    pub ty: KeywordCategory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum KeywordCategory {
    Flow,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PunctInfo {
    pub str: &'static str,
    pub name: &'static str,
}

macro_rules! define_keywords {
    ($($ty:ident: [ $($str:ident), * $(,)? ]), * $(,)?) => {
        pub const KEYWORDS: &[KeywordInfo] = &[$(
            $(
                KeywordInfo { str: stringify!($str), ty: KeywordCategory::$ty },
            )*
        )*];
    };
}
macro_rules! define_puncts {
    ($($str:literal $name:ident), * $(,)?) => {
        pub const PUNCTS: &[PunctInfo] = &[$(
            PunctInfo { str: $str, name: stringify!($name) }
        ), *];
    };
}

define_keywords!(
    Other: [
        mod, use, pub, package, super,
        trait, promise, require,
        type, struct, union, untagged,
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
);

pub fn keyword_to_variant(keyword: &str) -> String {
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
pub fn keyword_to_type(keyword: &str) -> String {
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
