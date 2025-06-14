use super::*;

pub type List<T> = Trailing<T, ListSep>;

#[derive(Debug, Clone, Copy, OptionParse)]
#[desc = "`,` / `;`"]
pub enum ListSep {
    Comma(punct!(",")),
    Semi(punct!(";")),
}
