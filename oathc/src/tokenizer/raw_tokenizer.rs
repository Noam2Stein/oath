use std::path::Path;

use logos::{Lexer, Logos};

use super::*;

#[derive(Debug)]
pub enum RawToken {
    Ident(Ident),
    Keyword(Keyword),
    Punct(Punct),
    Literal(Literal),
    OpenDelimiter(OpenDelimiter),
    CloseDelimiter(CloseDelimiter),
    Unknown(DiagnosticHandle),
}

#[derive(Debug)]
pub struct RawTokenizer<'ctx> {
    lexer: Lexer<'ctx, LogosToken<'ctx>>,
    path: &'ctx Path,
    interner: &'ctx Interner,
    diagnostics: &'ctx Diagnostics,
    highlights: &'ctx mut Vec<HighlightInfo>,
}

impl<'ctx> RawTokenizer<'ctx> {
    pub fn new(
        src: &'ctx str,
        path: &'ctx (impl AsRef<Path> + ?Sized),
        interner: &'ctx Interner,
        diagnostics: &'ctx Diagnostics,
        highlights: &'ctx mut Vec<HighlightInfo>,
    ) -> Self {
        let lexer = LogosToken::lexer(src);

        Self {
            lexer,
            path: path.as_ref(),
            interner,
            diagnostics,
            highlights,
        }
    }

    pub fn next(&mut self) -> Option<RawToken> {
        loop {
            break if let Some(next) = self.lexer.next() {
                let span = Span::from_start_end(
                    index_to_pos(self.lexer.source(), self.lexer.span().start),
                    index_to_pos(self.lexer.source(), self.lexer.span().end),
                );

                Some(with_tokens_expr! {
                        match next {
                            Ok(LogosToken::IdentOrKeyword(str)) => {
                                match Ident::new_or_keyword(str, span, &self.interner) {
                                    Ok(ident) => RawToken::Ident(ident),
                                    Err(keyword) => RawToken::Keyword(keyword),
                                }
                            },
                            $(
                                Ok(LogosToken::$punct_type) => {
                                    RawToken::Punct(Punct::new(span, PunctKind::$punct_variant))
                                },
                            )*
                            Ok(LogosToken::IntLiteral(str)) => RawToken::Literal(Literal::Int(IntLiteral::from_regex_str(span, str, self.path, self.interner, self.diagnostics))),
                            Ok(LogosToken::FloatLiteral(str)) => RawToken::Literal(Literal::Float(FloatLiteral::from_regex_str(span, str, self.path, self.interner, self.diagnostics))),
                            Ok(LogosToken::StrLiteral(str)) => RawToken::Literal(Literal::Str(StrLiteral::from_regex_str(span, str, self.path, self.interner, self.diagnostics))),
                            Ok(LogosToken::CharLiteral(str)) => RawToken::Literal(Literal::Char(CharLiteral::from_regex_str(span, str, self.path, self.interner, self.diagnostics))),
                            $(
                                Ok(LogosToken::$delim_open_type) => RawToken::OpenDelimiter(OpenDelimiter::$delim_fn(span)),
                                Ok(LogosToken::$delim_close_type) => RawToken::CloseDelimiter(CloseDelimiter::$delim_fn(span)),
                            )*
                            Err(_) => RawToken::Unknown(self.diagnostics.push_error(self.path.clone(), TokenError::UnknownToken(span)))
                        }
                })
            } else {
                None
            };
        }
    }

    pub fn path(&self) -> &'ctx Path {
        self.path
    }
    pub fn interner(&self) -> &'ctx Interner {
        self.interner
    }
    pub fn diagnostics(&self) -> &'ctx Diagnostics {
        self.diagnostics
    }
    pub fn highlights(&mut self) -> &mut Vec<HighlightInfo> {
        self.highlights
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
        #[regex(r#""([^"\\]|\\.)*""#)]
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
