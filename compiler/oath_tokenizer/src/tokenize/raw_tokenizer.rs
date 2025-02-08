use logos::{Lexer, Logos};

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RawToken {
    Ident(Ident),
    Keyword(Keyword),
    Punct(Punct),
    Literal(Literal),
    OpenDelimiter(Span, DelimiterKind),
    CloseDelimiter(Span, DelimiterKind),
}

pub trait SrcFileTokenizeRawExt: Seal {
    fn tokenize_raw<'src, 'd>(&'src self, context: ContextHandle<'d>) -> RawTokenizer<'src, 'd>;
}

#[derive(Debug, Clone)]
pub struct RawTokenizer<'src, 'd> {
    lexer: Lexer<'src, LogosToken<'src>>,
    context: ContextHandle<'d>,
}

impl SrcFileTokenizeRawExt for SrcFile {
    fn tokenize_raw<'src, 'd>(&'src self, context: ContextHandle<'d>) -> RawTokenizer<'src, 'd> {
        RawTokenizer {
            lexer: LogosToken::lexer(self.as_str()),
            context,
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
                        self.context.push_error(TokenError::UnknownToken(span));

                        continue;
                    }
                };

                Some(with_token_set_expr! {
                        match next {
                            LogosToken::IdentOrKeyword(str) => {
                                match Ident::new_or_keyword(str, span, self.context) {
                                    Ok(ident) => RawToken::Ident(ident),
                                    Err(keyword) => RawToken::Keyword(keyword),
                                }
                            },
                            $(
                                LogosToken::$punct_type => {
                                    RawToken::Punct(Punct::new(PunctKind::$punct_variant, span))
                                },
                            )*
                            LogosToken::IntLiteral(str) => RawToken::Literal(Literal::Int(unsafe { IntLiteral::from_regex_str(str, span, self.context) })),
                            LogosToken::FloatLiteral(str) => RawToken::Literal(Literal::Float(unsafe { FloatLiteral::from_regex_str(str, span, self.context) })),
                            LogosToken::StrLiteral(str) => RawToken::Literal(Literal::Str(unsafe { StrLiteral::from_regex_str(str, span, self.context) })),
                            LogosToken::CharLiteral(str) => RawToken::Literal(Literal::Char(unsafe { CharLiteral::from_regex_str(str, span, self.context) })),
                            $(
                                LogosToken::$delim_open_type => RawToken::OpenDelimiter(span, DelimiterKind::$delim_type),
                                LogosToken::$delim_close_type => RawToken::CloseDelimiter(span, DelimiterKind::$delim_type),
                            )*
                        }
                })
            } else {
                None
            };
        }
    }
}

with_token_set!(
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
