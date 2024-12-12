use logos::{Lexer, Logos};

use super::*;

puncts!(
    #[derive(Logos, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum RawTokenType {
        #[token("{")]
        OpenBrace,
        #[token("[")]
        OpenBracket,
        #[token("(")]
        OpenParen,
        #[token("}")]
        CloseBrace,
        #[token("]")]
        CloseBracket,
        #[token(")")]
        CloseParen,
        #[regex(r"[\p{L}_@][\p{L}_@1234567890]*")]
        Ident,
        #[regex(r#""((\\([nrt0\"\\\\])?)|[^"])*"?"#)]
        StringLiteral,
        #[regex(r#"'((\\([nrt0\"\\\\])?)|[^"])'?"#)]
        CharLiteral,
        #[regex(r"[1234567890][1234567890_]*([\p{L}_][\p{L}_1234567890]*)?")]
        IntLiteral,
        #[regex(
            r"[1234567890][1234567890_]*\.[1234567890][1234567890_]*([\p{L}_][\p{L}_1234567890]*)?"
        )]
        FloatLiteral,
        $(
            #[token($str)]
            $ty_ident,
        )*
        #[regex(r"\n")]
        Newline,
        #[regex(r"[ \t\f]+")]
        Whitespace,
    }
);

#[derive(Debug, Clone, Copy, Hash)]
pub struct RawToken {
    pub ty: RawTokenType,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct RawTokenizer<'src> {
    src: &'src SrcFile,
    lexer: Lexer<'src, RawTokenType>,
}
impl<'src> RawTokenizer<'src> {
    pub fn new(src: &'src SrcFile) -> Self {
        Self {
            src,
            lexer: RawTokenType::lexer(src.str()),
        }
    }

    pub fn src(&self) -> &'src SrcFile {
        self.src
    }

    pub fn next(&mut self, errors: &mut ErrorsHandle) -> Option<RawToken> {
        loop {
            break if let Some(next) = self.lexer.next() {
                let span = self.src.span_from_range(self.lexer.span());

                if let Ok(next) = next {
                    Some(RawToken { ty: next, span })
                } else {
                    errors.push(Error::new(
                        span,
                        format!("raw tokenizer failed on '{}'", self.lexer.slice()),
                    ));
                    continue;
                }
            } else {
                None
            };
        }
    }
}
