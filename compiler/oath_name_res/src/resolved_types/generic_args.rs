use oath_src::{Span, Spanned};

use super::Expr;

#[derive(Debug, Clone, Spanned)]
pub struct GenericArgs(#[span] pub Span, pub Vec<Expr>);
