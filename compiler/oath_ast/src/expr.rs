use crate::*;

pub enum Expr {
    Path(Path),
    Literal(Literal),
    Tuple(Span, Vec<Expr>),
    Neg(punct!("-"), Box<Expr>),
    Not(punct!("!"), Box<Expr>),
    Deref(punct!("*"), Box<Expr>),
}

impl Parse for Expr {
    fn parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
    ) -> Result<Self, ()> {
        if let Some(value) = parser.parse(context)? {
            Ok(Self::Path(value))
        } else if let Some(value) = parser.parse(context)? {
            Ok(Self::Literal(value))
        } else if let Some(group) = parser.parse::<Option<Group<Parens>>>(context).unwrap() {
            Ok(Self::Tuple(
                group.span(),
                group
                    .into_parser()
                    .parse_sep_all::<_, punct!(","), false, true>(context)?,
            ))
        } else if let Some(op) = parser.parse(context)? {
            Ok(Self::Neg(op, parser.parse(context)?))
        } else if let Some(op) = parser.parse(context)? {
            Ok(Self::Not(op, parser.parse(context)?))
        } else if let Some(op) = parser.parse(context)? {
            Ok(Self::Deref(op, parser.parse(context)?))
        } else {
            context.push_error(SyntaxError::Expected(parser.next_span(), "an expr"));
            Err(())
        }
    }
}

impl Peek for Expr {
    fn peek(parser: &mut Parser<impl Iterator<Item = TokenTree>>, context: ContextHandle) -> bool {
        parser.peek::<Path>(context)
            || parser.peek::<Literal>(context)
            || parser.peek::<Group>(context)
            || parser.peek::<punct!("-")>(context)
            || parser.peek::<punct!("!")>(context)
            || parser.peek::<punct!("*")>(context)
    }
}

impl Spanned for Expr {
    fn span(&self) -> Span {
        match self {
            Self::Deref(a, b) => a.span().connect(b.span()),
            Self::Literal(a) => a.span(),
            Self::Neg(a, b) => a.span().connect(b.span()),
            Self::Not(a, b) => a.span().connect(b.span()),
            Self::Path(a) => a.span(),
            Self::Tuple(span, _) => *span,
        }
    }
}
