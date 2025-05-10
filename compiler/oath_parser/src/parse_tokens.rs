use super::*;

// IDENT

impl OptionParse for Ident {
    fn option_parse(parser: &mut Parser<impl Tokenizer>, output: &mut Option<Self>) -> ParseExit {
        if let Some(PeekToken::Ident(token)) = parser.peek() {
            parser.next();

            *output = Some(token);
        }

        ParseExit::Complete
    }

    fn detect(parser: &Parser<impl Tokenizer>) -> Detection {
        if let Some(PeekToken::Ident(_)) = parser.peek() {
            Detection::Detected
        } else {
            Detection::NotDetected
        }
    }
}
impl ParseDesc for Ident {
    fn desc() -> &'static str {
        "an identifier"
    }
}

// LITERAL

impl OptionParse for Literal {
    fn option_parse(parser: &mut Parser<impl Tokenizer>, output: &mut Option<Self>) -> ParseExit {
        if let Some(PeekToken::Literal(token)) = parser.peek() {
            parser.next();

            match &token {
                Self::Int(_) | Self::Float(_) => parser.context().highlight(token.span(), HighlightColor::Yellow),
                _ => {}
            }

            *output = Some(token);
        }

        ParseExit::Complete
    }

    fn detect(parser: &Parser<impl Tokenizer>) -> Detection {
        if let Some(PeekToken::Literal(_)) = parser.peek() {
            Detection::Detected
        } else {
            Detection::NotDetected
        }
    }
}
impl ParseDesc for Literal {
    fn desc() -> &'static str {
        "a literal"
    }
}

impl OptionParse for IntLiteral {
    fn option_parse(parser: &mut Parser<impl Tokenizer>, output: &mut Option<Self>) -> ParseExit {
        if let Some(PeekToken::Literal(Literal::Int(token))) = parser.peek() {
            parser.next();

            *output = Some(token);
        }

        ParseExit::Complete
    }

    fn detect(parser: &Parser<impl Tokenizer>) -> Detection {
        if let Some(PeekToken::Literal(Literal::Int(_))) = parser.peek() {
            Detection::Detected
        } else {
            Detection::NotDetected
        }
    }
}
impl ParseDesc for IntLiteral {
    fn desc() -> &'static str {
        "an int literal"
    }
}

impl OptionParse for FloatLiteral {
    fn option_parse(parser: &mut Parser<impl Tokenizer>, output: &mut Option<Self>) -> ParseExit {
        if let Some(PeekToken::Literal(Literal::Float(token))) = parser.peek() {
            parser.next();

            *output = Some(token);
        }

        ParseExit::Complete
    }

    fn detect(parser: &Parser<impl Tokenizer>) -> Detection {
        if let Some(PeekToken::Literal(Literal::Float(_))) = parser.peek() {
            Detection::Detected
        } else {
            Detection::NotDetected
        }
    }
}
impl ParseDesc for FloatLiteral {
    fn desc() -> &'static str {
        "a float literal"
    }
}

impl OptionParse for StrLiteral {
    fn option_parse(parser: &mut Parser<impl Tokenizer>, output: &mut Option<Self>) -> ParseExit {
        if let Some(PeekToken::Literal(Literal::Str(token))) = parser.peek() {
            parser.next();

            *output = Some(token);
        }

        ParseExit::Complete
    }

    fn detect(parser: &Parser<impl Tokenizer>) -> Detection {
        if let Some(PeekToken::Literal(Literal::Str(_))) = parser.peek() {
            Detection::Detected
        } else {
            Detection::NotDetected
        }
    }
}
impl ParseDesc for StrLiteral {
    fn desc() -> &'static str {
        "a string literal"
    }
}

impl OptionParse for CharLiteral {
    fn option_parse(parser: &mut Parser<impl Tokenizer>, output: &mut Option<Self>) -> ParseExit {
        if let Some(PeekToken::Literal(Literal::Char(token))) = parser.peek() {
            parser.next();

            *output = Some(token);
        }

        ParseExit::Complete
    }

    fn detect(parser: &Parser<impl Tokenizer>) -> Detection {
        if let Some(PeekToken::Literal(Literal::Char(_))) = parser.peek() {
            Detection::Detected
        } else {
            Detection::NotDetected
        }
    }
}
impl ParseDesc for CharLiteral {
    fn desc() -> &'static str {
        "a character literal"
    }
}

// KEYWORD

impl OptionParse for Keyword {
    fn option_parse(parser: &mut Parser<impl Tokenizer>, output: &mut Option<Self>) -> ParseExit {
        if let Some(PeekToken::Keyword(token)) = parser.peek() {
            parser.next();

            *output = Some(token);
        }

        ParseExit::Complete
    }

    fn detect(parser: &Parser<impl Tokenizer>) -> Detection {
        if let Some(PeekToken::Keyword(_)) = parser.peek() {
            Detection::Detected
        } else {
            Detection::NotDetected
        }
    }
}
impl ParseDesc for Keyword {
    fn desc() -> &'static str {
        "a keyword"
    }
}

with_tokens!($(
    impl OptionParse for $keyword_type {
        fn option_parse(parser: &mut Parser<impl Tokenizer>, output: &mut Option<Self>) -> ParseExit {
            if let Some(PeekToken::Keyword(token)) = parser.peek() {
                if token.kind == KeywordKind::$keyword_variant {
                    parser.next();
    
                    *output = Some(Self(token.span));
                }
            }
    
            ParseExit::Complete
        }
    
        fn detect(parser: &Parser<impl Tokenizer>) -> Detection {
            if let Some(PeekToken::Keyword(token)) = parser.peek() {
                if token.kind == KeywordKind::$keyword_variant {
                    Detection::Detected
                } else {
                    Detection::NotDetected
                }
            } else {
                Detection::NotDetected
            }
        }
    }
    impl ParseDesc for $keyword_type {
        fn desc() -> &'static str {
            concat!("`", $keyword, "`")
        }
    }
)*);

// PUNCT

impl OptionParse for Punct {
    fn option_parse(parser: &mut Parser<impl Tokenizer>, output: &mut Option<Self>) -> ParseExit {
        if let Some(PeekToken::Punct(token)) = parser.peek() {
            parser.next();

            *output = Some(token);
        }

        ParseExit::Complete
    }

    fn detect(parser: &Parser<impl Tokenizer>) -> Detection {
        if let Some(PeekToken::Punct(_)) = parser.peek() {
            Detection::Detected
        } else {
            Detection::NotDetected
        }
    }
}
impl ParseDesc for Punct {
    fn desc() -> &'static str {
        "punctuation"
    }
}

with_tokens!($(
    impl OptionParse for $punct_type {
        fn option_parse(parser: &mut Parser<impl Tokenizer>, output: &mut Option<Self>) -> ParseExit {
            if let Some(PeekToken::Punct(token)) = parser.peek() {
                if token.kind == PunctKind::$punct_variant {
                    parser.next();
    
                    *output = Some(Self(token.span));
                }
            }
    
            ParseExit::Complete
        }
    
        fn detect(parser: &Parser<impl Tokenizer>) -> Detection {
            if let Some(PeekToken::Punct(token)) = parser.peek() {
                if token.kind == PunctKind::$punct_variant {
                    Detection::Detected
                } else {
                    Detection::NotDetected
                }
            } else {
                Detection::NotDetected
            }
        }
    
    
    }
    impl ParseDesc for $punct_type {
        fn desc() -> &'static str {
            concat!("`", $punct, "`")
        }
    }
)*);

impl OptionParse for Delimiters {
    fn option_parse(parser: &mut Parser<impl Tokenizer>, output: &mut Option<Self>) -> ParseExit {
        if Self::detect(parser) == Detection::Detected {
            *output = Some(match parser.next() {
                Some(LazyToken::Group(mut group)) => Self::new(group.open().span(), group.close().span(), group.open().kind),
                _ => unreachable!(),
            })
        }

        ParseExit::Complete
    }

    fn detect(parser: &Parser<impl Tokenizer>) -> Detection {
        match parser.peek() {
            Some(PeekToken::Group(_)) => Detection::Detected,
            _ => Detection::NotDetected,
        }
    }
}
with_tokens!($(
    impl OptionParse for $delims_type {
        fn option_parse(parser: &mut Parser<impl Tokenizer>, output: &mut Option<Self>) -> ParseExit {
            if Self::detect(parser) == Detection::Detected {
                *output = Some(match parser.next() {
                    Some(LazyToken::Group(mut group)) => Self::new(group.open().span(), group.close().span()),
                    _ => unreachable!(),
                })
            }

            ParseExit::Complete
        }

        fn detect(parser: &Parser<impl Tokenizer>) -> Detection {
            match parser.peek() {
                Some(PeekToken::Group(open)) => if open.kind == DelimiterKind::$delims_type {
                    Detection::Detected
                } else {
                    Detection::NotDetected
                },
                _ => Detection::NotDetected,
            }
        }
    }
)*);
