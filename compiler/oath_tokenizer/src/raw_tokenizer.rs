use logos::{Lexer, Logos};
use oath_src::{Span, SrcFile};

use crate::{Ident, Keyword, Literal, Punct, Seal};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RawToken {
    Ident(Ident),
    Keyword(Keyword),
    Punct(Punct),
    Literal(Literal),
    ParenOpen(Span),
    ParenClose(Span),
    BracketOpen(Span),
    BracketClose(Span),
    BraceOpen(Span),
    BraceClose(Span),
    Unknown(Span),
}

pub trait SrcFileTokenizeRawExt: Seal {
    fn tokenize_raw(&self) -> RawTokenizer;
}

#[derive(Debug, Clone)]
pub struct RawTokenizer<'src> {
    lexer: Lexer<'src, LogosToken<'src>>,
}

impl SrcFileTokenizeRawExt for SrcFile {
    fn tokenize_raw(&self) -> RawTokenizer {
        RawTokenizer {
            lexer: LogosToken::lexer(self.as_str()),
        }
    }
}

impl<'src> Iterator for RawTokenizer<'src> {
    type Item = RawToken;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
impl<'src> RawTokenizer<'src> {
    #[inline(always)]
    pub fn src(&self) -> &'src SrcFile {
        SrcFile::from_str(self.lexer.source())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Logos)]
enum LogosToken<'src> {
    IdentOrKeyword(&'src str),
    Literal(&'src str),
    ParenOpen,
    ParenClose,
    BracketOpen,
    BracketClose,
    BraceOpen,
    BraceClose,
    Punct,
}
