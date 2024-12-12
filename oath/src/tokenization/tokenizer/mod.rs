use std::{
    fmt::{self, Display, Formatter},
    mem::replace,
};

use super::*;

mod raw;
use raw::*;

#[derive(Debug, Clone, Hash)]
pub struct TokenizedFile {
    pub tokens: Tokens,
    pub end_span: Span,
}
impl Display for TokenizedFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.tokens
            .iter()
            .map(|token| token.to_string())
            .collect::<Box<[String]>>()
            .join(" ")
            .fmt(f)
    }
}
impl TokenizedFile {
    pub fn into_token_iter(self) -> impl TokenIterator {
        self.tokens.into_token_iter(self.end_span)
    }
}

#[derive(Debug, Clone)]
pub struct Tokenizer<'src> {
    raw: RawTokenizer<'src>,
    next: Option<TokenizerNext>,
}
#[derive(Debug, Clone)]
struct TokenizerNext {
    token: TokenTree,
    line_finished: bool,
}
impl<'src> Tokenizer<'src> {
    pub fn new(src: &'src SrcFile) -> Self {
        Self {
            raw: RawTokenizer::new(src),
            next: None,
        }
    }

    pub fn src(&self) -> &'src SrcFile {
        self.raw.src()
    }

    pub fn collect(mut self, errors: &mut ErrorsHandle) -> TokenizedFile {
        let mut lines = Vec::new();
        while self.peek(errors, false).is_some() {
            let mut line = Vec::new();

            while let Some(token) = self.next(errors, true) {
                line.push(token);
            }
            if line.len() > 0 {
                lines.push(line);
            }

            self.next_line(errors);
        }

        TokenizedFile {
            tokens: Tokens::new(lines),
            end_span: self.src().end_span(),
        }
    }
    pub fn peek(&mut self, errors: &mut ErrorsHandle, bound_to_line: bool) -> Option<&TokenTree> {
        self.validate_next(errors);

        self.next.as_ref().map_or(None, |next| {
            if bound_to_line && next.line_finished {
                None
            } else {
                Some(&next.token)
            }
        })
    }
    pub fn next(&mut self, errors: &mut ErrorsHandle, bound_to_line: bool) -> Option<TokenTree> {
        self.validate_next(errors);

        if !self.peek(errors, bound_to_line).is_some() {
            return None;
        }

        replace(&mut self.next, None).map(|next| next.token)
    }
    pub fn next_line(&mut self, errors: &mut ErrorsHandle) {
        self.validate_next(errors);

        if self.peek(errors, true).is_some() {
            let span = self.next_span(errors, false);
            errors.push(Error::new(span, "expected line to end"));

            while self.peek(errors, true).is_some() {
                self.next(errors, false);
            }
        }

        if let Some(next) = &mut self.next {
            next.line_finished = false;
        }
    }

    fn validate_next(&mut self, errors: &mut ErrorsHandle) {
        if self.next.is_some() {
            return;
        }

        self.next = loop {
            let next = next(&mut self.raw, errors, None);
            break match next {
                Some(next) => match next.token {
                    NextToken::Close(_) => {
                        unreachable!()
                    }

                    NextToken::TokenTree(token) => Some(TokenizerNext {
                        token,
                        line_finished: next.line_finished,
                    }),
                },
                None => None,
            };
        };
    }
}

#[derive(Debug, Clone, Hash)]
struct Next {
    token: NextToken,
    line_finished: bool,
}
#[derive(Debug, Clone, Hash)]
enum NextToken {
    TokenTree(TokenTree),
    Close(Span),
}
fn next(
    raw: &mut RawTokenizer,
    errors: &mut ErrorsHandle,
    expected_close: Option<DelimiterType>,
) -> Option<Next> {
    let mut line_finished = false;
    while let Some(RawToken { ty, span }) = raw.next(errors) {
        puncts!(
            return Some(Next {
                token: match ty {
                    RawTokenType::OpenBrace => NextToken::TokenTree(TokenTree::Group(
                        next_group(raw, errors, Delimiters::braces(span, span)),
                    )),
                    RawTokenType::OpenBracket => NextToken::TokenTree(TokenTree::Group(
                        next_group(raw, errors, Delimiters::brackets(span, span)),
                    )),
                    RawTokenType::OpenParen => NextToken::TokenTree(TokenTree::Group(
                        next_group(raw, errors, Delimiters::parens(span, span)),
                    )),
                    RawTokenType::CloseBrace => if expected_close == Some(DelimiterType::Braces) {
                        NextToken::Close(span)
                    } else {
                        errors.push(Error::new(span, "unmatched '}'"));
                        continue;
                    },
                    RawTokenType::CloseBracket => if expected_close == Some(DelimiterType::Brackets) {
                        NextToken::Close(span)
                    } else {
                        errors.push(Error::new(span, "unmatched '}'"));
                        continue;
                    },
                    RawTokenType::CloseParen => if expected_close == Some(DelimiterType::Parens) {
                        NextToken::Close(span)
                    } else {
                        errors.push(Error::new(span, "unmatched '}'"));
                        continue;
                    },
                    RawTokenType::Ident => NextToken::TokenTree({
                        let str = &raw.src()[span];
                        if let Some(keyword) = Keyword::try_from_str(span, str) {
                            TokenTree::Keyword(keyword)
                        } else {
                            TokenTree::Ident(Ident::new(span, str))
                        }
                    }),
                    $(
                        RawTokenType::$ty_ident => NextToken::TokenTree({
                            let str = &raw.src()[span];
                            if let Some(punct) = Punct::try_from_str(span, str) {
                                TokenTree::Punct(punct)
                            } else {
                                errors.push(Error::new(span, format!("'{str}' is not a token")));
                                continue;
                            }
                        }),
                    )*
                    RawTokenType::StringLiteral => NextToken::TokenTree({
                        let str: &str = &raw.src()[span];
                        TokenTree::Literal(Literal::String(
                            StringLiteral::from_str(span, str, errors),
                        ))
                    }),
                    RawTokenType::CharLiteral => NextToken::TokenTree({
                        let str = &raw.src()[span];
                        TokenTree::Literal(Literal::Char(CharLiteral::from_str(
                            span, str, errors,
                        )))
                    }),
                    RawTokenType::IntLiteral => NextToken::TokenTree({
                        let str = &raw.src()[span];
                        TokenTree::Literal(Literal::Int(IntLiteral::from_str(
                            span, str, errors,
                        )))
                    }),
                    RawTokenType::FloatLiteral => NextToken::TokenTree({
                        let str = &raw.src()[span];
                        TokenTree::Literal(Literal::Float(FloatLiteral::from_str(
                            span, str, errors,
                        )))
                    }),
                    RawTokenType::Newline => {
                        line_finished = true;
                        continue;
                    }
                    RawTokenType::Whitespace => continue,
                },
                line_finished,
            })
        );
    }

    None
}

fn next_group(
    raw: &mut RawTokenizer,
    errors: &mut ErrorsHandle,
    mut delimiters: Delimiters,
) -> Group {
    let mut lines = Vec::new();
    let mut line = Vec::new();

    while let Some(next) = next(raw, errors, Some(delimiters.ty())) {
        if next.line_finished && !line.is_empty() {
            lines.push(replace(&mut line, Vec::new()));
        }

        match next.token {
            NextToken::Close(close_span) => {
                delimiters.set_close_span(close_span);
                break;
            }
            NextToken::TokenTree(token) => line.push(token),
        }
    }
    if !line.is_empty() {
        lines.push(line);
    }

    Group {
        delimiters,
        tokens: Tokens { lines },
    }
}
