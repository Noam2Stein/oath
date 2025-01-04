use logos::{Lexer, Logos};
use oath_diagnostics::{DiagnosticsHandle, Error};
use oath_src::{Position, Span, SpanLengthed, SpanLined, SrcFile};

use crate::{
    with_puncts, Braces, Brackets, Delimiters, Ident, Keyword, Literal, Parens, Punct, Seal,
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
    Paren(SpanLengthed<1>),
    Bracket(SpanLengthed<1>),
    Brace(SpanLengthed<1>),
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
    pub fn complete(self, close_span: SpanLengthed<1>) -> Delimiters {
        match self {
            Self::Paren(open_span) => Delimiters::Parens(Parens::new(open_span, close_span)),
            Self::Bracket(open_span) => Delimiters::Brackets(Brackets::new(open_span, close_span)),
            Self::Brace(open_span) => Delimiters::Braces(Braces::new(open_span, close_span)),
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
                        self.diagnostics.push_error(Error::UnknownToken(span));

                        continue;
                    }
                };

                Some({
                    with_puncts!($(punct punct_len punct_variant punct_type): match next {
                        LogosToken::IdentOrKeyword(str) => {
                            match Ident::new_or_keyword(str, span.lined().unwrap()) {
                                Ok(ident) => RawToken::Ident(ident),
                                Err(keyword) => RawToken::Keyword(keyword),
                                },
                                $(

                                )*
                            }
                    })
                })
            } else {
                None
            };
        }
    }
}
impl<'src, 'd> RawTokenizer<'src, 'd> {
    #[inline(always)]
    pub fn src(&self) -> &'src SrcFile {
        SrcFile::from_str(self.lexer.source())
    }
}

with_puncts!($(punct punct_len punct_variant punct_type):
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Logos)]
    #[logos(extras = (usize, usize))]
    enum LogosToken<'src> {
        IdentOrKeyword(&'src str),
        $(
            #[token($punct)]
            $punct_type,
        )*
        Literal(&'src str),
        ParenOpen,
        ParenClose,
        BracketOpen,
        BracketClose,
        BraceOpen,
        BraceClose,
    }
);

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
