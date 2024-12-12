use crate::source::Spanned;

use super::*;

impl Parse for TokenTree {
    fn parse(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> Self {
        if let Some(output) = input.next(errors, bound_to_line) {
            output
        } else {
            errors.push(Error::new(input.end_span(), "expected a token tree"));

            Self::Ident(Ident::new(input.end_span(), "_"))
        }
    }
}
impl Parse for Group {
    fn parse(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> Self {
        if let Some(output) = input.next(errors, bound_to_line) {
            if let TokenTree::Group(output) = output {
                output
            } else {
                errors.push(Error::new(output.span(), "expected a group"));

                Self {
                    delimiters: Delimiters::parens(output.span(), output.span()),
                    tokens: Tokens::default(),
                }
            }
        } else {
            errors.push(Error::new(input.end_span(), "expected a group"));

            Self {
                delimiters: Delimiters::parens(input.end_span(), input.end_span()),
                tokens: Tokens::default(),
            }
        }
    }
}
impl Parse for Ident {
    fn parse(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> Self {
        if let Some(output) = input.next(errors, bound_to_line) {
            if let TokenTree::Ident(output) = output {
                output
            } else if let TokenTree::Keyword(keyword) = output {
                errors.push(Error::new(keyword.span(), "expected an ident"));

                Self::new(keyword.span(), format!("@{keyword}"))
            } else {
                errors.push(Error::new(output.span(), "expected an ident"));

                Self::new(output.span(), "_")
            }
        } else {
            errors.push(Error::new(input.end_span(), "expected an ident"));

            Self::new(input.end_span(), "_")
        }
    }
}
impl Parse for Keyword {
    fn parse(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> Self {
        if let Some(output) = input.next(errors, bound_to_line) {
            if let TokenTree::Keyword(output) = output {
                output
            } else {
                errors.push(Error::new(output.span(), "expected a keyword"));

                Self::Error(output.span())
            }
        } else {
            errors.push(Error::new(input.end_span(), "expected a keyword"));

            Self::Error(input.end_span())
        }
    }
}
impl Parse for Literal {
    fn parse(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> Self {
        if let Some(output) = input.next(errors, bound_to_line) {
            if let TokenTree::Literal(output) = output {
                output
            } else {
                errors.push(Error::new(output.span(), "expected a literal"));

                Self::Error(output.span())
            }
        } else {
            errors.push(Error::new(input.end_span(), "expected a literal"));

            Self::Error(input.end_span())
        }
    }
}
impl Parse for StringLiteral {
    fn parse(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> Self {
        if let Some(output) = input.next(errors, bound_to_line) {
            if let TokenTree::Literal(output) = output {
                if let Literal::String(output) = output {
                    output
                } else {
                    errors.push(Error::new(output.span(), "expected a string literal"));

                    Self::new(output.span(), "")
                }
            } else {
                errors.push(Error::new(output.span(), "expected a string literal"));

                Self::new(output.span(), "")
            }
        } else {
            errors.push(Error::new(input.end_span(), "expected a string literal"));

            Self::new(input.end_span(), "")
        }
    }
}
impl Parse for CharLiteral {
    fn parse(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> Self {
        if let Some(output) = input.next(errors, bound_to_line) {
            if let TokenTree::Literal(output) = output {
                if let Literal::Char(output) = output {
                    output
                } else {
                    errors.push(Error::new(output.span(), "expected a char literal"));

                    Self::new(output.span(), ' ')
                }
            } else {
                errors.push(Error::new(output.span(), "expected a char literal"));

                Self::new(output.span(), ' ')
            }
        } else {
            errors.push(Error::new(input.end_span(), "expected a char literal"));

            Self::new(input.end_span(), ' ')
        }
    }
}
impl Parse for IntLiteral {
    fn parse(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> Self {
        if let Some(output) = input.next(errors, bound_to_line) {
            if let TokenTree::Literal(output) = output {
                if let Literal::Int(output) = output {
                    output
                } else {
                    errors.push(Error::new(output.span(), "expected an int literal"));

                    Self::unsuffixed(output.span(), 1)
                }
            } else {
                errors.push(Error::new(output.span(), "expected an int literal"));

                Self::unsuffixed(output.span(), 1)
            }
        } else {
            errors.push(Error::new(input.end_span(), "expected an int literal"));

            Self::unsuffixed(input.end_span(), 1)
        }
    }
}
impl Parse for FloatLiteral {
    fn parse(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> Self {
        if let Some(output) = input.next(errors, bound_to_line) {
            if let TokenTree::Literal(output) = output {
                if let Literal::Float(output) = output {
                    output
                } else {
                    errors.push(Error::new(output.span(), "expected a float literal"));

                    Self::unsuffixed(output.span(), 1, 0)
                }
            } else {
                errors.push(Error::new(output.span(), "expected a float literal"));

                Self::unsuffixed(output.span(), 1, 0)
            }
        } else {
            errors.push(Error::new(input.end_span(), "expected a float literal"));

            Self::unsuffixed(input.end_span(), 1, 0)
        }
    }
}
impl Parse for Punct {
    fn parse(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> Self {
        if let Some(output) = input.next(errors, bound_to_line) {
            if let TokenTree::Punct(output) = output {
                output
            } else {
                errors.push(Error::new(output.span(), "expected a punct"));

                Self::Error(output.span())
            }
        } else {
            errors.push(Error::new(input.end_span(), "expected a punct"));

            Self::Error(input.end_span())
        }
    }
}

keywords!(
    $(impl Parse for Keyword!($str) {
        fn parse(
            input: &mut impl TokenIterator,
            errors: &mut ErrorsHandle,
            bound_to_line: bool,
        ) -> Self {
            if let Some(output) = input.next(errors, bound_to_line) {
                if let TokenTree::Keyword(output) = output {
                    if let Keyword::$ty_ident(output) = output {
                        output
                    } else {
                        errors.push(Error::new(output.span(), format!("expected '{}'", $str)));

                        if Self::peek(input, errors, bound_to_line) {
                            Self::parse(input, errors, bound_to_line)
                        } else {
                            Self(output.span())
                        }
                    }
                } else {
                    errors.push(Error::new(output.span(), format!("expected '{}'", $str)));

                    if Self::peek(input, errors, bound_to_line) {
                        Self::parse(input, errors, bound_to_line)
                    } else {
                        Self(output.span())
                    }
                }
            } else {
                errors.push(Error::new(input.end_span(), format!("expected '{}'", $str)));

                Self(input.end_span())
            }
        }
    })*
);

puncts!(
    $(impl Parse for Punct!($str) {
        fn parse(
            input: &mut impl TokenIterator,
            errors: &mut ErrorsHandle,
            bound_to_line: bool,
        ) -> Self {
            if let Some(output) = input.next(errors, bound_to_line) {
                if let TokenTree::Punct(output) = output {
                    if let Punct::$ty_ident(output) = output {
                        output
                    } else {
                        errors.push(Error::new(output.span(), format!("expected '{}'", $str)));

                        if Self::peek(input, errors, bound_to_line) {
                            Self::parse(input, errors, bound_to_line)
                        } else {
                            Self(output.span())
                        }
                    }
                } else {
                    errors.push(Error::new(output.span(), format!("expected '{}'", $str)));

                    if Self::peek(input, errors, bound_to_line) {
                        Self::parse(input, errors, bound_to_line)
                    } else {
                        Self(output.span())
                    }
                }
            } else {
                errors.push(Error::new(input.end_span(), format!("expected '{}'", $str)));

                Self(input.end_span())
            }
        }
    })*
);
