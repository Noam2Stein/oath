use std::sync::Arc;

use logos::{Lexer, Logos};

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RawToken {
    Ident(Ident),
    Keyword(Keyword),
    Punct(Punct),
    Literal(Literal),
    OpenDelimiter(OpenDelimiter),
    CloseDelimiter(CloseDelimiter),
}

#[derive(Debug, Clone)]
pub struct RawTokenizer<'src> {
    lexer: Lexer<'src, LogosToken<'src>>,
    context: Arc<Context>,
}

impl<'src> RawTokenizer<'src> {
    pub fn new(src: &'src str, context: Arc<Context>) -> Self {
        let lexer = LogosToken::lexer(src);

        Self { lexer, context }
    }

    pub fn next(&mut self) -> Option<RawToken> {
        loop {
            break if let Some(next) = self.lexer.next() {
                let span = Span::from_start_end(
                    index_to_pos(self.lexer.source(), self.lexer.span().start),
                    index_to_pos(self.lexer.source(), self.lexer.span().end),
                );

                let next = match next {
                    Ok(ok) => ok,
                    Err(_) => {
                        self.context.push_error(TokenError::UnknownToken(span));

                        continue;
                    }
                };

                Some(with_tokens_expr! {
                        match next {
                            LogosToken::IdentOrKeyword(str) => {
                                match Ident::new_or_keyword(str, span, &mut self.context.interner.write().unwrap()) {
                                    Ok(ident) => RawToken::Ident(ident),
                                    Err(keyword) => RawToken::Keyword(keyword),
                                }
                            },
                            $(
                                LogosToken::$punct_type => {
                                    RawToken::Punct(Punct::new(span, PunctKind::$punct_variant))
                                },
                            )*
                            LogosToken::IntLiteral(str) => RawToken::Literal(Literal::Int(IntLiteral::from_regex_str(span, str, &self.context))),
                            LogosToken::FloatLiteral(str) => RawToken::Literal(Literal::Float(FloatLiteral::from_regex_str(span, str, &self.context))),
                            LogosToken::StrLiteral(str) => RawToken::Literal(Literal::Str(StrLiteral::from_regex_str(span, str, &self.context))),
                            LogosToken::CharLiteral(str) => RawToken::Literal(Literal::Char(CharLiteral::from_regex_str(span, str, &self.context))),
                            $(
                                LogosToken::$delim_open_type => RawToken::OpenDelimiter(OpenDelimiter::$delim_fn(span)),
                                LogosToken::$delim_close_type => RawToken::CloseDelimiter(CloseDelimiter::$delim_fn(span)),
                            )*
                        }
                })
            } else {
                None
            };
        }
    }

    pub fn context(&self) -> &Arc<Context> {
        &self.context
    }
}

with_tokens!(
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Logos)]
    #[logos(skip r"[ \t\n\r\f]+")]
    enum LogosToken<'src> {
        #[regex(r"[a-zA-Z_@][a-zA-Z_@0-9]*")]
        IdentOrKeyword(&'src str),
        $(
            #[token($punct)]
            $punct_type,
        )*
        #[regex("[0-9][0-9_@a-zA-Z]*")]
        IntLiteral(&'src str),
        #[regex(r"[0-9][0-9_]*\.[0-9_@a-zA-Z]+")]
        FloatLiteral(&'src str),
        #[regex("\".*\"")]
        StrLiteral(&'src str),
        #[regex("'.'")]
        CharLiteral(&'src str),
        $(
            #[token($delim_open)]
            $delim_open_type,
            #[token($delim_close)]
            $delim_close_type,
        )*
    }
);

fn index_to_pos(str: &str, index: usize) -> Position {
    let mut line = 0;
    let mut last_line_start = 0;

    for newline in str[0..index]
        .char_indices()
        .filter_map(|(index, char)| if char == '\n' { Some(index) } else { None })
    {
        line += 1;
        last_line_start = newline + 1;
    }

    Position {
        line,
        char: (index - last_line_start) as u32,
    }
}
