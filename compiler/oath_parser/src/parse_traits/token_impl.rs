use crate::*;

impl OptionParse for Ident {
    fn option_parse(parser: &mut Parser, output: &mut Option<Self>) -> ParseExit {
        if let Some(PeekToken::Ident(token)) = parser.peek() {
            parser.next();

            *output = Some(token);
        }

        ParseExit::Complete
    }

    fn detect(parser: &Parser) -> bool {
        if let Some(PeekToken::Ident(_)) = parser.peek() {
            true
        } else {
            false
        }
    }

    fn desc() -> &'static str {
        "an identifier"
    }
}

impl OptionParse for Literal {
    fn option_parse(parser: &mut Parser, output: &mut Option<Self>) -> ParseExit {
        if let Some(PeekToken::Literal(token)) = parser.peek() {
            parser.next();

            *output = Some(token);
        }

        ParseExit::Complete
    }

    fn detect(parser: &Parser) -> bool {
        if let Some(PeekToken::Literal(_)) = parser.peek() {
            true
        } else {
            false
        }
    }

    fn desc() -> &'static str {
        "a literal"
    }
}
impl OptionParse for IntLiteral {
    fn option_parse(parser: &mut Parser, output: &mut Option<Self>) -> ParseExit {
        if let Some(PeekToken::Literal(Literal::Int(token))) = parser.peek() {
            parser.next();

            *output = Some(token);
        }

        ParseExit::Complete
    }

    fn detect(parser: &Parser) -> bool {
        if let Some(PeekToken::Literal(Literal::Int(_))) = parser.peek() {
            true
        } else {
            false
        }
    }

    fn desc() -> &'static str {
        "an int literal"
    }
}
impl OptionParse for FloatLiteral {
    fn option_parse(parser: &mut Parser, output: &mut Option<Self>) -> ParseExit {
        if let Some(PeekToken::Literal(Literal::Float(token))) = parser.peek() {
            parser.next();

            *output = Some(token);
        }

        ParseExit::Complete
    }

    fn detect(parser: &Parser) -> bool {
        if let Some(PeekToken::Literal(Literal::Float(_))) = parser.peek() {
            true
        } else {
            false
        }
    }

    fn desc() -> &'static str {
        "a float literal"
    }
}
impl OptionParse for StrLiteral {
    fn option_parse(parser: &mut Parser, output: &mut Option<Self>) -> ParseExit {
        if let Some(PeekToken::Literal(Literal::Str(token))) = parser.peek() {
            parser.next();

            *output = Some(token);
        }

        ParseExit::Complete
    }

    fn detect(parser: &Parser) -> bool {
        if let Some(PeekToken::Literal(Literal::Str(_))) = parser.peek() {
            true
        } else {
            false
        }
    }

    fn desc() -> &'static str {
        "a string literal"
    }
}
impl OptionParse for CharLiteral {
    fn option_parse(parser: &mut Parser, output: &mut Option<Self>) -> ParseExit {
        if let Some(PeekToken::Literal(Literal::Char(token))) = parser.peek() {
            parser.next();

            *output = Some(token);
        }

        ParseExit::Complete
    }

    fn detect(parser: &Parser) -> bool {
        if let Some(PeekToken::Literal(Literal::Char(_))) = parser.peek() {
            true
        } else {
            false
        }
    }

    fn desc() -> &'static str {
        "a character literal"
    }
}

impl OptionParse for Keyword {
    fn option_parse(parser: &mut Parser, output: &mut Option<Self>) -> ParseExit {
        if let Some(PeekToken::Keyword(token)) = parser.peek() {
            parser.next();

            *output = Some(token);
        }

        ParseExit::Complete
    }

    fn detect(parser: &Parser) -> bool {
        if let Some(PeekToken::Keyword(_)) = parser.peek() {
            true
        } else {
            false
        }
    }

    fn desc() -> &'static str {
        "a keyword"
    }
}
with_tokens!($(
    impl OptionParse for $keyword_type {
        fn option_parse(parser: &mut Parser, output: &mut Option<Self>) -> ParseExit {
            if let Some(PeekToken::Keyword(token)) = parser.peek() {
                if token.kind == KeywordKind::$keyword_variant {
                    parser.next();
    
                    *output = Some(Self(token.span));
                }
            }
    
            ParseExit::Complete
        }
    
        fn detect(parser: &Parser) -> bool {
            if let Some(PeekToken::Keyword(token)) = parser.peek() {
                token.kind == KeywordKind::$keyword_variant
            } else {
                false
            }
        }
    
        fn desc() -> &'static str {
            concat!("`", $keyword, "`")
        }
    }
)*);

impl OptionParse for Punct {
    fn option_parse(parser: &mut Parser, output: &mut Option<Self>) -> ParseExit {
        if let Some(PeekToken::Punct(token)) = parser.peek() {
            parser.next();

            *output = Some(token);
        }

        ParseExit::Complete
    }

    fn detect(parser: &Parser) -> bool {
        if let Some(PeekToken::Punct(_)) = parser.peek() {
            true
        } else {
            false
        }
    }

    fn desc() -> &'static str {
        "punctuation"
    }
}
with_tokens!($(
    impl OptionParse for $punct_type {
        fn option_parse(parser: &mut Parser, output: &mut Option<Self>) -> ParseExit {
            if let Some(PeekToken::Punct(token)) = parser.peek() {
                if token.kind == PunctKind::$punct_variant {
                    parser.next();
    
                    *output = Some(Self(token.span));
                }
            }
    
            ParseExit::Complete
        }
    
        fn detect(parser: &Parser) -> bool {
            if let Some(PeekToken::Punct(token)) = parser.peek() {
                token.kind == PunctKind::$punct_variant
            } else {
                false
            }
        }
    
        fn desc() -> &'static str {
            concat!("`", $punct, "`")
        }
    }
)*);
