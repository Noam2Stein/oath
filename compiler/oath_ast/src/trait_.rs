use crate::*;

pub enum Trait {
    Question(punct!("?"), Box<Trait>),
    Not(punct!("!"), Box<Trait>),
    Tuple(Span, Vec<Trait>),
    Path(Path),
    Eq(punct!("=="), Expr),
    More(punct!(">"), Expr),
    Less(punct!("<"), Expr),
    MoreEq(punct!(">="), Expr),
    LessEq(punct!("<="), Expr),
    NotEq(punct!("!="), Expr),
}

impl Parse for Trait {
    fn parse(
        parser: &mut Parser<impl Iterator<Item = TokenTree>>,
        context: ContextHandle,
    ) -> Result<Self, ()> {
        if let Some(value) = parser.parse(context)? {
            Ok(Self::Path(value))
        } else if let Some(group) = parser.parse::<Option<Group<Parens>>>(context).unwrap() {
            Ok(Self::Tuple(
                group.span(),
                group
                    .into_parser()
                    .parse_sep_all::<_, punct!(","), false, true>(context)?,
            ))
        } else if let Some(punct) = parser.parse::<Option<punct!("?")>>(context)? {
            match parser.parse::<Trait>(context)? {
                Self::Tuple(a, b) => Ok(Self::Question(punct, Box::new(Trait::Tuple(a, b)))),
                Self::Path(value) => Ok(Self::Question(punct, Box::new(Trait::Path(value)))),
                sub_trait => {
                    context.push_error(SyntaxError::Expected(
                        sub_trait.span(),
                        "either a path or `( )`",
                    ));
                    Ok(Self::Question(
                        punct,
                        Box::new(Trait::Tuple(sub_trait.span(), vec![sub_trait])),
                    ))
                }
            }
        } else if let Some(punct) = parser.parse::<Option<punct!("!")>>(context)? {
            match parser.parse::<Trait>(context)? {
                Self::Tuple(a, b) => Ok(Self::Not(punct, Box::new(Trait::Tuple(a, b)))),
                Self::Path(value) => Ok(Self::Not(punct, Box::new(Trait::Path(value)))),
                sub_trait => {
                    context.push_error(SyntaxError::Expected(
                        sub_trait.span(),
                        "either a path or `( )`",
                    ));
                    Ok(Self::Not(
                        punct,
                        Box::new(Trait::Tuple(sub_trait.span(), vec![sub_trait])),
                    ))
                }
            }
        } else if let Some(punct) = parser.parse::<Option<_>>(context)? {
            Ok(Self::Eq(punct, parser.parse(context)?))
        } else if let Some(punct) = parser.parse::<Option<_>>(context)? {
            Ok(Self::NotEq(punct, parser.parse(context)?))
        } else if let Some(punct) = parser.parse::<Option<_>>(context)? {
            Ok(Self::LessEq(punct, parser.parse(context)?))
        } else if let Some(punct) = parser.parse::<Option<_>>(context)? {
            Ok(Self::MoreEq(punct, parser.parse(context)?))
        } else if let Some(punct) = parser.parse::<Option<_>>(context)? {
            Ok(Self::Less(punct, parser.parse(context)?))
        } else if let Some(punct) = parser.parse::<Option<_>>(context)? {
            Ok(Self::More(punct, parser.parse(context)?))
        } else {
            context.push_error(SyntaxError::Expected(parser.next_span(), "a trait bounds"));
            Err(())
        }
    }
}

impl Spanned for Trait {
    fn span(&self) -> Span {
        match self {
            Self::Eq(a, b) => a.span().connect(b.span()),
            Self::Less(a, b) => a.span().connect(b.span()),
            Self::LessEq(a, b) => a.span().connect(b.span()),
            Self::More(a, b) => a.span().connect(b.span()),
            Self::MoreEq(a, b) => a.span().connect(b.span()),
            Self::Not(a, b) => a.span().connect(b.span()),
            Self::NotEq(a, b) => a.span().connect(b.span()),
            Self::Path(a) => a.span(),
            Self::Question(a, b) => a.span().connect(b.span()),
            Self::Tuple(span, _) => *span,
        }
    }
}
