use super::*;

impl PeekRef for TokenTree {
    fn peek_ref<'a>(
        input: &'a mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> Option<&'a Self> {
        input.peek(errors, bound_to_line)
    }
}
impl PeekRef for Group {
    fn peek_ref<'a>(
        input: &'a mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> Option<&'a Self> {
        TokenTree::peek_ref(input, errors, bound_to_line).map_or(None, |peek| {
            if let TokenTree::Group(peek) = peek {
                Some(peek)
            } else {
                None
            }
        })
    }
}
impl PeekRef for Braces {
    fn peek_ref<'a>(
        input: &'a mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> Option<&'a Self> {
        Group::peek_ref(input, errors, bound_to_line).map_or(None, |group| {
            match &group.delimiters {
                Delimiters::Braces(output) => Some(output),
                _ => None,
            }
        })
    }
}
impl PeekRef for Brackets {
    fn peek_ref<'a>(
        input: &'a mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> Option<&'a Self> {
        Group::peek_ref(input, errors, bound_to_line).map_or(None, |group| {
            match &group.delimiters {
                Delimiters::Brackets(output) => Some(output),
                _ => None,
            }
        })
    }
}
impl PeekRef for Parens {
    fn peek_ref<'a>(
        input: &'a mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> Option<&'a Self> {
        Group::peek_ref(input, errors, bound_to_line).map_or(None, |group| {
            match &group.delimiters {
                Delimiters::Parens(output) => Some(output),
                _ => None,
            }
        })
    }
}
impl PeekRef for Ident {
    fn peek_ref<'a>(
        input: &'a mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> Option<&'a Self> {
        TokenTree::peek_ref(input, errors, bound_to_line).map_or(None, |peek| {
            if let TokenTree::Ident(peek) = peek {
                Some(peek)
            } else {
                None
            }
        })
    }
}
impl PeekRef for Keyword {
    fn peek_ref<'a>(
        input: &'a mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> Option<&'a Self> {
        TokenTree::peek_ref(input, errors, bound_to_line).map_or(None, |peek| {
            if let TokenTree::Keyword(peek) = peek {
                Some(peek)
            } else {
                None
            }
        })
    }
}
impl PeekRef for Literal {
    fn peek_ref<'a>(
        input: &'a mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> Option<&'a Self> {
        TokenTree::peek_ref(input, errors, bound_to_line).map_or(None, |peek| {
            if let TokenTree::Literal(peek) = peek {
                Some(peek)
            } else {
                None
            }
        })
    }
}
impl PeekRef for StringLiteral {
    fn peek_ref<'a>(
        input: &'a mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> Option<&'a Self> {
        Literal::peek_ref(input, errors, bound_to_line).map_or(None, |peek| {
            if let Literal::String(peek) = peek {
                Some(peek)
            } else {
                None
            }
        })
    }
}
impl PeekRef for CharLiteral {
    fn peek_ref<'a>(
        input: &'a mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> Option<&'a Self> {
        Literal::peek_ref(input, errors, bound_to_line).map_or(None, |peek| {
            if let Literal::Char(peek) = peek {
                Some(peek)
            } else {
                None
            }
        })
    }
}
impl PeekRef for IntLiteral {
    fn peek_ref<'a>(
        input: &'a mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> Option<&'a Self> {
        Literal::peek_ref(input, errors, bound_to_line).map_or(None, |peek| {
            if let Literal::Int(peek) = peek {
                Some(peek)
            } else {
                None
            }
        })
    }
}
impl PeekRef for FloatLiteral {
    fn peek_ref<'a>(
        input: &'a mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> Option<&'a Self> {
        Literal::peek_ref(input, errors, bound_to_line).map_or(None, |peek| {
            if let Literal::Float(peek) = peek {
                Some(peek)
            } else {
                None
            }
        })
    }
}
impl PeekRef for Punct {
    fn peek_ref<'a>(
        input: &'a mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> Option<&'a Self> {
        TokenTree::peek_ref(input, errors, bound_to_line).map_or(None, |peek| {
            if let TokenTree::Punct(peek) = peek {
                Some(peek)
            } else {
                None
            }
        })
    }
}

keywords!(
    $(
        impl PeekRef for Keyword!($str) {
            fn peek_ref<'a>(
                input: &'a mut impl TokenIterator,
                errors: &mut ErrorsHandle,
                bound_to_line: bool,
            ) -> Option<&'a Self> {
                Keyword::peek_ref(input, errors, bound_to_line).map_or(None, |peek| {
                    if let Keyword::$ty_ident(peek) = peek {
                        Some(peek)
                    } else {
                        None
                    }
                })
            }
        }
    )*
);

puncts!(
    $(
        impl PeekRef for Punct!($str) {
            fn peek_ref<'a>(
                input: &'a mut impl TokenIterator,
                errors: &mut ErrorsHandle,
                bound_to_line: bool,
            ) -> Option<&'a Self> {
                Punct::peek_ref(input, errors, bound_to_line).map_or(None, |peek| {
                    if let Punct::$ty_ident(peek) = peek {
                        Some(peek)
                    } else {
                        None
                    }
                })
            }
        }
    )*
);
