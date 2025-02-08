use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IntLiteral {
    int: u128,
    suffix: Option<Ident>,
    span: Span,
}

impl LiteralType for IntLiteral {}
impl TokenType for IntLiteral {}
impl Seal for IntLiteral {}

impl TryFrom<TokenTree> for IntLiteral {
    type Error = ();

    fn try_from(value: TokenTree) -> Result<Self, Self::Error> {
        if let TokenTree::Literal(Literal::Int(output)) = value {
            Ok(output)
        } else {
            Err(())
        }
    }
}
impl<'a> TryFrom<&'a TokenTree> for IntLiteral {
    type Error = ();

    fn try_from(value: &'a TokenTree) -> Result<Self, Self::Error> {
        if let TokenTree::Literal(Literal::Int(value)) = value {
            Ok(*value)
        } else {
            Err(())
        }
    }
}
impl TryFrom<Literal> for IntLiteral {
    type Error = ();

    fn try_from(value: Literal) -> Result<Self, Self::Error> {
        if let Literal::Int(value) = value {
            Ok(value)
        } else {
            Err(())
        }
    }
}

impl Spanned for IntLiteral {
    #[inline(always)]
    fn span(&self) -> Span {
        self.span
    }
}

impl IntLiteral {
    #[inline(always)]
    pub fn new(int: u128, suffix: Option<Ident>, span: Span) -> Self {
        Self { int, suffix, span }
    }

    #[inline(always)]
    pub fn int(self) -> u128 {
        self.int
    }
    #[inline(always)]
    pub fn suffix(self) -> Option<Ident> {
        self.suffix
    }

    pub unsafe fn from_regex_str(str: &str, span: Span, context: ContextHandle) -> Self {
        let suffix_start = str
            .char_indices()
            .find(|(_, char)| !char.is_ascii_digit() && *char != '_')
            .map(|(char_pos, _)| char_pos);

        let int_str = &str[0..suffix_start.unwrap_or(str.len())].replace("_", "");
        let suffix_str = suffix_start.map(|suffix_start| &str[suffix_start..]);

        let int = u128::from_str_radix(int_str, 10).unwrap_or_else(|_| {
            context.push_error(Error::new("out of bounds literal", span));
            1
        });

        let suffix = suffix_str.map_or(None, |suffix_str| {
            Ident::new(suffix_str, span, context).or_else(|| {
                context.push_error(Error::new("expected an ident. found a keyword", span));

                None
            })
        });

        Self { int, suffix, span }
    }
}
