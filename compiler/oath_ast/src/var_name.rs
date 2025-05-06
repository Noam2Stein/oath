use crate::*;

#[derive(Debug, Clone, OptionSpanned, Parse)]
pub enum VarName {
    #[group]
    Tuple(OpenParen, Vec<VarName>),
    #[fallback]
    Ident(
        #[option_spanned] Option<keyword!("mut")>,
        #[option_spanned] Try<Ident>,
        #[option_spanned] Option<Expr>,
    ),
}
