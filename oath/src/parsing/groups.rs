use super::*;

#[derive(Debug, Clone, Hash)]
pub struct GroupBraces {
    pub delimiters: Braces,
    pub tokens: Tokens,
}
#[derive(Debug, Clone, Hash)]
pub struct GroupBrackets {
    pub delimiters: Brackets,
    pub tokens: Tokens,
}
#[derive(Debug, Clone, Hash)]
pub struct GroupParens {
    pub delimiters: Parens,
    pub tokens: Tokens,
}

impl Parse for GroupBraces {
    fn parse(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> Self {
        if Self::peek(input, errors, bound_to_line) {
            if let Some(output) = input.next(errors, bound_to_line) {
                if let TokenTree::Group(Group { delimiters, tokens }) = output {
                    if let Delimiters::Braces(delimiters) = delimiters {
                        Self { delimiters, tokens }
                    } else {
                        unreachable!()
                    }
                } else {
                    unreachable!()
                }
            } else {
                unreachable!()
            }
        } else {
            let span = input.peek_span(errors, bound_to_line);
            errors.push(Error::new(span, "expected braces"));

            if Group::peek(input, errors, bound_to_line) {
                let Group { delimiters, tokens } = Group::parse(input, errors, bound_to_line);
                Self {
                    delimiters: Braces::new(delimiters.open_span(), delimiters.close_span()),
                    tokens,
                }
            } else {
                Self {
                    delimiters: Braces::new(span, span),
                    tokens: Tokens::default(),
                }
            }
        }
    }
}
impl Parse for GroupBrackets {
    fn parse(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> Self {
        if Self::peek(input, errors, bound_to_line) {
            if let Some(output) = input.next(errors, bound_to_line) {
                if let TokenTree::Group(Group { delimiters, tokens }) = output {
                    if let Delimiters::Brackets(delimiters) = delimiters {
                        Self { delimiters, tokens }
                    } else {
                        unreachable!()
                    }
                } else {
                    unreachable!()
                }
            } else {
                unreachable!()
            }
        } else {
            let span = input.peek_span(errors, bound_to_line);
            errors.push(Error::new(span, "expected brackets"));

            if Group::peek(input, errors, bound_to_line) {
                let Group { delimiters, tokens } = Group::parse(input, errors, bound_to_line);
                Self {
                    delimiters: Brackets::new(delimiters.open_span(), delimiters.close_span()),
                    tokens,
                }
            } else {
                Self {
                    delimiters: Brackets::new(span, span),
                    tokens: Tokens::default(),
                }
            }
        }
    }
}
impl Parse for GroupParens {
    fn parse(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> Self {
        if Self::peek(input, errors, bound_to_line) {
            if let Some(output) = input.next(errors, bound_to_line) {
                if let TokenTree::Group(Group { delimiters, tokens }) = output {
                    if let Delimiters::Parens(delimiters) = delimiters {
                        Self { delimiters, tokens }
                    } else {
                        unreachable!()
                    }
                } else {
                    unreachable!()
                }
            } else {
                unreachable!()
            }
        } else {
            let span = input.peek_span(errors, bound_to_line);
            errors.push(Error::new(span, "expected parentheses"));

            if Group::peek(input, errors, bound_to_line) {
                let Group { delimiters, tokens } = Group::parse(input, errors, bound_to_line);
                Self {
                    delimiters: Parens::new(delimiters.open_span(), delimiters.close_span()),
                    tokens,
                }
            } else {
                Self {
                    delimiters: Parens::new(span, span),
                    tokens: Tokens::default(),
                }
            }
        }
    }
}

impl Peek for GroupBraces {
    fn peek(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> bool {
        Group::peek_ref(input, errors, bound_to_line).map_or(false, |group| {
            if let Delimiters::Braces(_) = group.delimiters {
                true
            } else {
                false
            }
        })
    }
}
impl Peek for GroupBrackets {
    fn peek(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> bool {
        Group::peek_ref(input, errors, bound_to_line).map_or(false, |group| {
            if let Delimiters::Brackets(_) = group.delimiters {
                true
            } else {
                false
            }
        })
    }
}
impl Peek for GroupParens {
    fn peek(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> bool {
        Group::peek_ref(input, errors, bound_to_line).map_or(false, |group| {
            if let Delimiters::Parens(_) = group.delimiters {
                true
            } else {
                false
            }
        })
    }
}
