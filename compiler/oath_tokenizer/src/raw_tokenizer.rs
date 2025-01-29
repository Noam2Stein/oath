use logos::{Lexer, Logos};
use oath_diagnostics::{DiagnosticsHandle, Error};
use oath_src::{Position, Span, SrcFile};

use crate::{
    with_puncts, Braces, Brackets, CharLiteral, Delimiters, FloatLiteral, Ident, IntLiteral,
    Keyword, Literal, Parens, Punct, Seal, StrLiteral,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RawToken {
    Ident(Ident),
    Keyword(Keyword),
    Punct(Punct),
    Literal(Literal),
    OpenDelimiter(SingleDelimiter),
    CloseDelimiter(SingleDelimiter),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SingleDelimiter {
    Paren(Span),
    Bracket(Span),
    Brace(Span),
}
impl SingleDelimiter {
    pub fn pair(self, close: Self) -> Option<Delimiters> {
        match self {
            Self::Paren(open_span) => match close {
                Self::Paren(close_span) => {
                    Some(Delimiters::Parens(Parens::new(open_span, close_span)))
                }
                _ => None,
            },
            Self::Bracket(open_span) => match close {
                Self::Bracket(close_span) => {
                    Some(Delimiters::Brackets(Brackets::new(open_span, close_span)))
                }
                _ => None,
            },
            Self::Brace(open_span) => match close {
                Self::Brace(close_span) => {
                    Some(Delimiters::Braces(Braces::new(open_span, close_span)))
                }
                _ => None,
            },
        }
    }
}

pub trait SrcFileTokenizeRawExt: Seal {
    fn tokenize_raw<'src, 'd>(
        &'src self,
        diagnostics: DiagnosticsHandle<'d>,
    ) -> RawTokenizer<'src, 'd>;
}

#[derive(Debug, Clone)]
pub struct RawTokenizer<'src, 'd> {
    lexer: Lexer<'src, LogosToken<'src>>,
    diagnostics: DiagnosticsHandle<'d>,
}

impl SrcFileTokenizeRawExt for SrcFile {
    fn tokenize_raw<'src, 'd>(
        &'src self,
        diagnostics: DiagnosticsHandle<'d>,
    ) -> RawTokenizer<'src, 'd> {
        RawTokenizer {
            lexer: LogosToken::lexer(self.as_str()),
            diagnostics,
        }
    }
}

impl<'src, 'd> Iterator for RawTokenizer<'src, 'd> {
    type Item = RawToken;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            break if let Some(next) = self.lexer.next() {
                let span = Span::from_start_end(
                    index_to_pos(self.lexer.source(), self.lexer.span().start),
                    index_to_pos(self.lexer.source(), self.lexer.span().end),
                );

                let next = match next {
                    Ok(ok) => ok,
                    Err(_) => {
                        self.diagnostics.push_error(Error::UnknownToken, span);

                        continue;
                    }
                };

                Some({
                    macro_rules! use_puncts {
                        ($($punct:literal($punct_len:literal $punct_variant:ident $punct_type:ident),)*) => {
                            match next {
                                LogosToken::IdentOrKeyword(str) => {
                                    match Ident::new_or_keyword(str, span) {
                                        Ok(ident) => RawToken::Ident(ident),
                                        Err(keyword) => RawToken::Keyword(keyword),
                                    }
                                },
                                $(
                                    LogosToken::$punct_type => {
                                        RawToken::Punct(Punct::$punct_variant(crate::$punct_type(span)))
                                    },
                                )*
                                LogosToken::IntLiteral(str) => RawToken::Literal(Literal::Int(unsafe { IntLiteral::from_regex_str(str, span, self.diagnostics) })),
                                LogosToken::FloatLiteral(str) => RawToken::Literal(Literal::Float(unsafe { FloatLiteral::from_regex_str(str, span, self.diagnostics) })),
                                LogosToken::StrLiteral(str) => RawToken::Literal(Literal::Str(unsafe { StrLiteral::from_regex_str(str, span, self.diagnostics) })),
                                LogosToken::CharLiteral(str) => RawToken::Literal(Literal::Char(unsafe { CharLiteral::from_regex_str(str, span, self.diagnostics) })),
                                LogosToken::ParenOpen => RawToken::OpenDelimiter(SingleDelimiter::Paren(span)),
                                LogosToken::BracketOpen => RawToken::OpenDelimiter(SingleDelimiter::Bracket(span)),
                                LogosToken::BraceOpen => RawToken::OpenDelimiter(SingleDelimiter::Brace(span)),
                                LogosToken::ParenClose => RawToken::CloseDelimiter(SingleDelimiter::Paren(span)),
                                LogosToken::BracketClose => RawToken::CloseDelimiter(SingleDelimiter::Bracket(span)),
                                LogosToken::BraceClose => RawToken::CloseDelimiter(SingleDelimiter::Brace(span)),
                            }
                        };
                    }
                    with_puncts!(use_puncts)
                })
            } else {
                None
            };
        }
    }
}

macro_rules! use_puncts {
    ($($punct:literal($punct_len:literal $punct_variant:ident $punct_type:ident),)*) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Logos)]
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
            #[token("(")]
            ParenOpen,
            #[token(")")]
            ParenClose,
            #[token("[")]
            BracketOpen,
            #[token("]")]
            BracketClose,
            #[token("{")]
            BraceOpen,
            #[token("}")]
            BraceClose,
        }
    }
}
with_puncts!(use_puncts);

fn index_to_pos(str: &str, index: usize) -> Position {
    let mut line = 0;
    let mut last_line_start = 0;

    for newline in str[0..index]
        .replace("\r", "")
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
