use std::iter::Peekable;

use oath_context::*;
use oath_diagnostics::*;
use oath_src::*;
use oath_tokens::*;

mod from_regex_str;
mod raw_tokenizer;
use from_regex_str::*;
use raw_tokenizer::*;

#[allow(private_bounds)]
pub trait TokenizeExt: Seal {
    fn tokenize(&self, context: ContextHandle) -> TokenFile;
}
trait Seal {}

impl Seal for SrcFile {}
impl TokenizeExt for SrcFile {
    fn tokenize(&self, context: ContextHandle) -> TokenFile {
        let mut tokens = Vec::new();

        let mut raw_tokens = self.tokenize_raw(context).peekable();
        while let Some(raw_token) = raw_tokens.next() {
            match raw_token {
                RawToken::Ident(token) => tokens.push(TokenTree::Ident(token)),
                RawToken::Keyword(token) => tokens.push(TokenTree::Keyword(token)),
                RawToken::Punct(token) => tokens.push(TokenTree::Punct(token)),
                RawToken::Literal(token) => tokens.push(TokenTree::Literal(token)),
                RawToken::OpenDelimiter(open_span, open_kind) => tokens.push(TokenTree::Group(
                    tokenize_group(open_span, open_kind, &mut raw_tokens, context),
                )),
                RawToken::CloseDelimiter(close_span, close_kind) => {
                    context.push_error(TokenError::Unopened(close_span, close_kind))
                }
            }
        }

        TokenFile { tokens }
    }
}

fn tokenize_group(
    open_span: Span,
    open_kind: DelimiterKind,
    raw_tokens: &mut Peekable<RawTokenizer>,
    context: ContextHandle,
) -> Group {
    let mut tokens = Vec::new();

    macro_rules! unfinished_group {
        () => {
            Group {
                delimiters: Delimiters::new(
                    open_span,
                    Span::from_start_len(
                        tokens
                            .last()
                            .map_or(open_span.end(), |last| last.span().end()),
                        1,
                    ),
                    open_kind,
                ),
                tokens,
            }
        };
    }

    while let Some(raw_token) = raw_tokens.peek().map(|a| *a) {
        if let RawToken::CloseDelimiter(close_span, close_kind) = raw_token {
            return if close_kind == open_kind {
                raw_tokens.next();
                Group {
                    delimiters: Delimiters::new(open_span, close_span, open_kind),
                    tokens,
                }
            } else {
                context.push_error(TokenError::Unclosed(close_span, open_kind));

                unfinished_group!()
            };
        }

        match raw_tokens.next().unwrap() {
            RawToken::Ident(token) => tokens.push(TokenTree::Ident(token)),
            RawToken::Keyword(token) => tokens.push(TokenTree::Keyword(token)),
            RawToken::Punct(token) => tokens.push(TokenTree::Punct(token)),
            RawToken::Literal(token) => tokens.push(TokenTree::Literal(token)),
            RawToken::OpenDelimiter(open_span, open_kind) => tokens.push(TokenTree::Group(
                tokenize_group(open_span, open_kind, raw_tokens, context),
            )),
            RawToken::CloseDelimiter(_, _) => unreachable!(),
        }
    }

    let close_span = Span::from_end_len(
        tokens
            .last()
            .map_or(open_span.end(), |last| last.span().end()),
        1,
    );

    context.push_error(TokenError::Unclosed(close_span, open_kind));

    unfinished_group!()
}
