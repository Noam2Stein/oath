use std::iter::Flatten;

use super::*;

#[derive(Debug, Clone, Hash, Default)]
pub struct Tokens {
    pub lines: Vec<Vec<TokenTree>>,
}
impl Tokens {
    pub fn new(lines: Vec<Vec<TokenTree>>) -> Self {
        Self { lines }
    }
}

impl Tokens {
    pub fn iter(&self) -> Flatten<<&Vec<Vec<TokenTree>> as IntoIterator>::IntoIter> {
        self.lines.iter().flatten()
    }
    pub fn iter_mut(&mut self) -> Flatten<<&mut Vec<Vec<TokenTree>> as IntoIterator>::IntoIter> {
        self.lines.iter_mut().flatten()
    }
}
impl IntoIterator for Tokens {
    type Item = TokenTree;
    type IntoIter = Flatten<<Vec<Vec<TokenTree>> as IntoIterator>::IntoIter>;
    fn into_iter(self) -> Self::IntoIter {
        self.lines.into_iter().flatten()
    }
}
impl<'a> IntoIterator for &'a Tokens {
    type Item = &'a TokenTree;
    type IntoIter = Flatten<<&'a Vec<Vec<TokenTree>> as IntoIterator>::IntoIter>;
    fn into_iter(self) -> Self::IntoIter {
        self.lines.iter().flatten()
    }
}
impl<'a> IntoIterator for &'a mut Tokens {
    type Item = &'a mut TokenTree;
    type IntoIter = Flatten<<&'a mut Vec<Vec<TokenTree>> as IntoIterator>::IntoIter>;
    fn into_iter(self) -> Self::IntoIter {
        self.lines.iter_mut().flatten()
    }
}

impl IntoTokenIterator for Tokens {
    fn into_token_iter(mut self, end_span: Span) -> impl TokenIterator {
        self.lines.retain(|line| !line.is_empty());

        self.lines.reverse();
        for line in &mut self.lines {
            line.reverse();
        }

        TokensIter {
            rev_lines: self.lines,
            end_span,
        }
    }
}
struct TokensIter {
    rev_lines: Vec<Vec<TokenTree>>,
    end_span: Span,
}
impl TokenIterator for TokensIter {
    fn end_span(&self) -> Span {
        self.end_span
    }
    fn peek(&mut self, _errors: &mut ErrorsHandle, bound_to_line: bool) -> Option<&TokenTree> {
        if bound_to_line {
            self.rev_lines.last().map_or(None, |line| line.last())
        } else {
            for line in self.rev_lines.iter().rev() {
                if let Some(peek) = line.last() {
                    return Some(peek);
                }
            }

            None
        }
    }
    fn next(&mut self, _errors: &mut ErrorsHandle, bound_to_line: bool) -> Option<TokenTree> {
        if bound_to_line {
            self.rev_lines.last_mut().map_or(None, |line| line.pop())
        } else {
            loop {
                let line = if let Some(line) = self.rev_lines.last_mut() {
                    line
                } else {
                    break None;
                };

                if let Some(next) = line.pop() {
                    break Some(next);
                } else {
                    self.rev_lines.pop();
                };
            }
        }
    }
    fn next_line(&mut self, errors: &mut ErrorsHandle) {
        if let Some(peek) = self.peek(errors, true) {
            errors.push(Error::new(peek.span(), "expected line to end"));
        }

        self.rev_lines.pop();
    }
}
