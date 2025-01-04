use std::iter::Peekable;

use crate::{
    raw_tokenizer::{RawToken, RawTokenizer, SingleDelimiter, SrcFileTokenizeRawExt},
    Braces, Brackets, Delimiters, Group, Parens, Seal, TokenFile, TokenTree,
};
use oath_diagnostics::{DiagnosticsHandle, Error};
use oath_src::{SpanLengthed, Spanned, SrcFile};

#[allow(private_bounds)]
pub trait SrcFileTokenizeExt: Seal {
    fn tokenize(&self, diagnostics: DiagnosticsHandle) -> TokenFile;
}

impl Seal for SrcFile {}
impl SrcFileTokenizeExt for SrcFile {
    fn tokenize(&self, diagnostics: DiagnosticsHandle) -> TokenFile {
        let mut tokens = Vec::new();

        let mut raw_tokens = self.tokenize_raw(diagnostics).peekable();
        while let Some(raw_token) = raw_tokens.next() {
            match raw_token {
                RawToken::Ident(token) => tokens.push(TokenTree::Ident(token)),
                RawToken::Keyword(token) => tokens.push(TokenTree::Keyword(token)),
                RawToken::Punct(token) => tokens.push(TokenTree::Punct(token)),
                RawToken::Literal(token) => tokens.push(TokenTree::Literal(token)),
                RawToken::OpenDelimiter(open) => tokens.push(TokenTree::Group(tokenize_group(
                    open,
                    &mut raw_tokens,
                    diagnostics,
                ))),
                RawToken::CloseDelimiter(close) => diagnostics.push_error(match close {
                    SingleDelimiter::Paren(span) => Error::UnopenedParen(span),
                    SingleDelimiter::Bracket(span) => Error::UnopenedBracket(span),
                    SingleDelimiter::Brace(span) => Error::UnopenedBrace(span),
                }),
            }
        }

        TokenFile { tokens }
    }
}

fn tokenize_group(
    open_delimiter: SingleDelimiter,
    raw_tokens: &mut Peekable<RawTokenizer>,
    diagnostics: DiagnosticsHandle,
) -> Group {
    let mut tokens = Vec::new();

    macro_rules! unfinished_group {
        () => {
            Group {
                delimiters: match open_delimiter {
                    SingleDelimiter::Paren(open_span) => Delimiters::Parens(Parens::new(
                        open_span,
                        SpanLengthed::from_start(
                            tokens
                                .last()
                                .map_or(open_span.end(), |last| last.span().end()),
                        ),
                    )),
                    SingleDelimiter::Bracket(open_span) => Delimiters::Brackets(Brackets::new(
                        open_span,
                        SpanLengthed::from_start(
                            tokens
                                .last()
                                .map_or(open_span.end(), |last| last.span().end()),
                        ),
                    )),
                    SingleDelimiter::Brace(open_span) => Delimiters::Braces(Braces::new(
                        open_span,
                        SpanLengthed::from_start(
                            tokens
                                .last()
                                .map_or(open_span.end(), |last| last.span().end()),
                        ),
                    )),
                },
                tokens,
            }
        };
    }

    while let Some(raw_token) = raw_tokens.peek() {
        if let RawToken::CloseDelimiter(close_delimiter) = raw_token {
            return if let Some(delimiters) = close_delimiter.pair(open_delimiter) {
                raw_tokens.next();
                Group { delimiters, tokens }
            } else {
                diagnostics.push_error(match open_delimiter {
                    SingleDelimiter::Paren(span) => Error::UnclosedParen(span),
                    SingleDelimiter::Bracket(span) => Error::UnclosedBracket(span),
                    SingleDelimiter::Brace(span) => Error::UnclosedBrace(span),
                });
                unfinished_group!()
            };
        }

        match raw_tokens.next().unwrap() {
            RawToken::Ident(token) => tokens.push(TokenTree::Ident(token)),
            RawToken::Keyword(token) => tokens.push(TokenTree::Keyword(token)),
            RawToken::Punct(token) => tokens.push(TokenTree::Punct(token)),
            RawToken::Literal(token) => tokens.push(TokenTree::Literal(token)),
            RawToken::OpenDelimiter(open) => tokens.push(TokenTree::Group(tokenize_group(
                open,
                raw_tokens,
                diagnostics,
            ))),
            RawToken::CloseDelimiter(_) => unreachable!(),
        }
    }

    diagnostics.push_error(match open_delimiter {
        SingleDelimiter::Paren(span) => Error::UnclosedParen(span),
        SingleDelimiter::Bracket(span) => Error::UnclosedBracket(span),
        SingleDelimiter::Brace(span) => Error::UnclosedBrace(span),
    });

    unfinished_group!()
}
