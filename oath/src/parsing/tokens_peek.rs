use super::*;

impl Peek for TokenTree {
    fn peek(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> bool {
        Self::peek_ref(input, errors, bound_to_line).is_some()
    }
}
impl Peek for Group {
    fn peek(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> bool {
        Self::peek_ref(input, errors, bound_to_line).is_some()
    }
}
impl Peek for Braces {
    fn peek(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> bool {
        Self::peek_ref(input, errors, bound_to_line).is_some()
    }
}
impl Peek for Brackets {
    fn peek(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> bool {
        Self::peek_ref(input, errors, bound_to_line).is_some()
    }
}
impl Peek for Parens {
    fn peek(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> bool {
        Self::peek_ref(input, errors, bound_to_line).is_some()
    }
}
impl Peek for Ident {
    fn peek(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> bool {
        Self::peek_ref(input, errors, bound_to_line).is_some()
    }
}
impl Peek for Keyword {
    fn peek(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> bool {
        Self::peek_ref(input, errors, bound_to_line).is_some()
    }
}
impl Peek for Literal {
    fn peek(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> bool {
        Self::peek_ref(input, errors, bound_to_line).is_some()
    }
}
impl Peek for StringLiteral {
    fn peek(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> bool {
        Self::peek_ref(input, errors, bound_to_line).is_some()
    }
}
impl Peek for CharLiteral {
    fn peek(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> bool {
        Self::peek_ref(input, errors, bound_to_line).is_some()
    }
}
impl Peek for IntLiteral {
    fn peek(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> bool {
        Self::peek_ref(input, errors, bound_to_line).is_some()
    }
}
impl Peek for FloatLiteral {
    fn peek(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> bool {
        Self::peek_ref(input, errors, bound_to_line).is_some()
    }
}
impl Peek for Punct {
    fn peek(
        input: &mut impl TokenIterator,
        errors: &mut ErrorsHandle,
        bound_to_line: bool,
    ) -> bool {
        Self::peek_ref(input, errors, bound_to_line).is_some()
    }
}

keywords!(
    $(
        impl Peek for Keyword!($str) {
            fn peek(input: &mut impl TokenIterator, errors: &mut ErrorsHandle, bound_to_line: bool) -> bool {
                Self::peek_ref(input, errors, bound_to_line).is_some()
            }
        }
    )*
);

puncts!(
    $(
        impl Peek for Punct!($str) {
            fn peek(input: &mut impl TokenIterator, errors: &mut ErrorsHandle, bound_to_line: bool) -> bool {
                Self::peek_ref(input, errors, bound_to_line).is_some()
            }
        }
    )*
);
