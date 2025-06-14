use super::*;

// IDENT

impl OptionParse for Ident {
    fn option_parse(parser: &mut impl Tokenizer, output: &mut Option<Self>) -> ParseExit {
        if let Some(PeekToken::Ident(_)) = parser.peek() {
            *output = Some(match parser.next() {
                Some(LazyToken::Ident(token)) => token,
                _ => unreachable!(),
            });
        }

        ParseExit::Complete
    }

    fn detect(parser: &impl Tokenizer) -> Detection {
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
    fn option_parse(parser: &mut impl Tokenizer, output: &mut Option<Self>) -> ParseExit {
        if let Some(PeekToken::Literal(_)) = parser.peek() {
            let token = match parser.next() {
                Some(LazyToken::Literal(token)) => token,
                _ => unreachable!(),
            };

            match &token {
                Self::Int(_) | Self::Float(_) => token.span().highlight(HighlightColor::Yellow, parser.highlights()),
                _ => {}
            }

            *output = Some(token);
        }

        ParseExit::Complete
    }

    fn detect(parser: &impl Tokenizer) -> Detection {
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
    fn option_parse(parser: &mut impl Tokenizer, output: &mut Option<Self>) -> ParseExit {
        if let Some(PeekToken::Literal(Literal::Int(_))) = parser.peek() {
            let token = match parser.next() {
                Some(LazyToken::Literal(Literal::Int(token))) => token,
                _ => unreachable!(),
            };

            token.span().highlight(HighlightColor::Yellow, parser.highlights());

            *output = Some(token);
        }

        ParseExit::Complete
    }

    fn detect(parser: &impl Tokenizer) -> Detection {
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
    fn option_parse(parser: &mut impl Tokenizer, output: &mut Option<Self>) -> ParseExit {
        if let Some(PeekToken::Literal(Literal::Float(_))) = parser.peek() {
            let token = match parser.next() {
                Some(LazyToken::Literal(Literal::Float(token))) => token,
                _ => unreachable!(),
            };

            token.span().highlight(HighlightColor::Yellow, parser.highlights());

            *output = Some(token);
        }

        ParseExit::Complete
    }

    fn detect(parser: &impl Tokenizer) -> Detection {
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
    fn option_parse(parser: &mut impl Tokenizer, output: &mut Option<Self>) -> ParseExit {
        if let Some(PeekToken::Literal(Literal::Str(_))) = parser.peek() {
            *output = Some(match parser.next() {
                Some(LazyToken::Literal(Literal::Str(token))) => token,
                _ => unreachable!(),
            });
        }

        ParseExit::Complete
    }

    fn detect(parser: &impl Tokenizer) -> Detection {
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
    fn option_parse(parser: &mut impl Tokenizer, output: &mut Option<Self>) -> ParseExit {
        if let Some(PeekToken::Literal(Literal::Char(_))) = parser.peek() {
            *output = Some(match parser.next() {
                Some(LazyToken::Literal(Literal::Char(token))) => token,
                _ => unreachable!(),
            });
        }

        ParseExit::Complete
    }

    fn detect(parser: &impl Tokenizer) -> Detection {
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
    fn option_parse(parser: &mut impl Tokenizer, output: &mut Option<Self>) -> ParseExit {
        if let Some(PeekToken::Keyword(_)) = parser.peek() {
            *output = Some(match parser.next() {
                Some(LazyToken::Keyword(token)) => token,
                _ => unreachable!(),
            });
        }

        ParseExit::Complete
    }

    fn detect(parser: &impl Tokenizer) -> Detection {
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
        fn option_parse(parser: &mut impl Tokenizer, output: &mut Option<Self>) -> ParseExit {
            if let Some(PeekToken::Keyword(Keyword { kind: KeywordKind::$keyword_variant, span: _ })) = parser.peek() {
                *output = Some(Self(match parser.next() {
                    Some(LazyToken::Keyword(token)) => token.span,
                    _ => unreachable!(),
                }));
            }
    
            ParseExit::Complete
        }
    
        fn detect(parser: &impl Tokenizer) -> Detection {
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
    fn option_parse(parser: &mut impl Tokenizer, output: &mut Option<Self>) -> ParseExit {
        if let Some(PeekToken::Punct(_)) = parser.peek() {
            *output = Some(match parser.next() {
                Some(LazyToken::Punct(token)) => token,
                _ => unreachable!(),
            });
        }

        ParseExit::Complete
    }

    fn detect(parser: &impl Tokenizer) -> Detection {
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
        fn option_parse(parser: &mut impl Tokenizer, output: &mut Option<Self>) -> ParseExit {
            if let Some(PeekToken::Punct(Punct { kind: PunctKind::$punct_variant, span: _ })) = parser.peek() {
                *output = Some(Self(match parser.next() {
                    Some(LazyToken::Punct(token)) => token.span,
                    _ => unreachable!(),
                }));
            }
    
            ParseExit::Complete
        }
    
        fn detect(parser: &impl Tokenizer) -> Detection {
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

impl FrameDelimiters for Delimiters {
    fn option_parse_frame<Inner, T: Tokenizer>(
        parser: &mut T,
        output: &mut Option<(Frame<Self>, Inner)>,
        _parse_outside: impl FnOnce(&mut T) -> (Inner, ParseExit),
        parse_inside: impl FnOnce(&mut GroupTokenizer) -> (Inner, ParseExit),
    ) -> ParseExit {
        let mut parser = if Self::detect_frame(parser) == Detection::Detected {
            match parser.next() {
                Some(LazyToken::Group(group)) => group,
                _ => unreachable!(),
            }
        } else {
            return ParseExit::Complete;
        };

        let mut leftovers = Leftovers::parse_error();

        let (value, exit) = parse_inside(&mut parser);
        match exit {
            ParseExit::Complete => {
                Leftovers::parse(&mut parser, &mut leftovers);
            }
            ParseExit::Cut => {}
        };

        let frame = Frame {
            delims: parser.finish().try_into().unwrap(),
            leftovers,
        };

        *output = Some((frame, value));

        ParseExit::Complete
    }

    fn detect_frame(parser: &impl Tokenizer) -> Detection {
        if let Some(PeekToken::Group(_)) = parser.peek() {
            Detection::Detected
        } else {
            Detection::NotDetected
        }
    }
}

with_tokens!($(
    impl FrameDelimiters for $delims_type {
        fn option_parse_frame<Inner, T: Tokenizer>(
            parser: &mut T,
            output: &mut Option<(Frame<Self>, Inner)>,
            _parse_outside: impl FnOnce(&mut T) -> (Inner, ParseExit),
            parse_inside: impl FnOnce(&mut GroupTokenizer) -> (Inner, ParseExit),
        ) -> ParseExit {
            let mut parser = if Self::detect_frame(parser) == Detection::Detected {
                match parser.next() {
                    Some(LazyToken::Group(group)) => group,
                    _ => unreachable!(),
                }
            } else {
                return ParseExit::Complete;
            };

            let mut leftovers = Leftovers::parse_error();            

            let (value, exit) = parse_inside(&mut parser);
            match exit {
                ParseExit::Complete => {
                    Leftovers::parse(&mut parser, &mut leftovers);
                },
                ParseExit::Cut => {}
            };

            let frame = Frame {
                delims: parser.finish().try_into().unwrap(),
                leftovers,
            };

            *output = Some((frame, value));

            ParseExit::Complete
        }

        fn detect_frame(parser: &impl Tokenizer) -> Detection {
            if let Some(PeekToken::Group(group)) = parser.peek() {
                if group.kind == DelimiterKind::$delims_type {
                    Detection::Detected
                } else {
                    Detection::NotDetected
                }
            } else {
                Detection::NotDetected
            }
        }
    }
)*);
