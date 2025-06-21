use std::fmt::Write;

use super::*;

#[derive(Debug, Clone)]
pub enum FormatTree {
    Atom(String),
    Chain(Vec<FormatTree>),
    DotChain(Vec<FormatTree>),
    DenseDelims(char, Box<FormatTree>, char),
    SpacedDelims(char, Box<FormatTree>, char),
    DenseDelimsList(char, Vec<FormatTree>, char),
    SpacedDelimsList(char, Vec<FormatTree>, char),
    DenseDelimsStmts(char, Vec<FormatTree>, char),
    SpacedDelimsStmts(char, Vec<FormatTree>, char),
    Items(Vec<FormatTree>),
}

impl FormatTree {
    pub fn unexpanded_len(&self) -> u32 {
        match self {
            Self::Atom(str) => str.chars().count() as u32,
            Self::Chain(items) => items.iter().map(|item| item.unexpanded_len()).sum::<u32>(),
            Self::DotChain(items) => items.iter().map(|item| item.unexpanded_len() + 1).sum::<u32>() - 1,
            Self::DenseDelims(_, inner, _) => 2 + inner.unexpanded_len(),
            Self::SpacedDelims(_, inner, _) => 4 + inner.unexpanded_len(),
            Self::DenseDelimsList(_, items, _) => 2 + items.iter().map(|item| item.unexpanded_len() + 2).sum::<u32>() - 2,
            Self::SpacedDelimsList(_, items, _) => 4 + items.iter().map(|item| item.unexpanded_len() + 2).sum::<u32>() - 2,
            Self::DenseDelimsStmts(_, items, _) => 2 + items.iter().map(|item| item.unexpanded_len() + 2).sum::<u32>(),
            Self::SpacedDelimsStmts(_, items, _) => 4 + items.iter().map(|item| item.unexpanded_len() + 2).sum::<u32>(),
            Self::Items(items) => items.iter().map(|item| item.unexpanded_len()).max().unwrap_or(0),
        }
    }

    pub fn format(&self, s: &mut String, tab_lvl: u32, config: &FormatConfig) -> std::fmt::Result {
        if self.unexpanded_len() > config.max_width {
            self.format_expanded(s, tab_lvl, config)
        } else {
            self.format_unexpanded(s, tab_lvl, config)
        }
    }

    pub fn format_unexpanded(&self, s: &mut String, tab_lvl: u32, config: &FormatConfig) -> std::fmt::Result {
        match self {
            Self::Atom(str) => write!(s, "{str}")?,

            Self::Chain(items) => {
                for item in items {
                    item.format_unexpanded(s, tab_lvl, config)?;
                }
            }

            Self::DotChain(items) => {
                for item in items {
                    item.format_unexpanded(s, tab_lvl, config)?;
                    write!(s, ".")?;
                }

                s.pop();
            }

            Self::DenseDelims(open, inner, close) => {
                write!(s, "{open}")?;
                inner.format_unexpanded(s, tab_lvl, config)?;
                write!(s, "{close}")?;
            }

            Self::SpacedDelims(open, inner, close) => {
                write!(s, "{open} ")?;
                inner.format_unexpanded(s, tab_lvl, config)?;
                write!(s, " {close}")?;
            }

            Self::DenseDelimsList(open, items, close) => {
                write!(s, "{open}")?;

                for item in items {
                    item.format_unexpanded(s, tab_lvl, config)?;
                    write!(s, ", ")?;
                }
                s.pop();
                s.pop();

                write!(s, "{close}")?;
            }

            Self::SpacedDelimsList(open, items, close) => {
                write!(s, "{open} ")?;

                for item in items {
                    item.format_unexpanded(s, tab_lvl, config)?;
                    write!(s, ", ")?;
                }
                s.pop();
                s.pop();

                write!(s, " {close}")?;
            }

            Self::DenseDelimsStmts(open, items, close) => {
                write!(s, "{open}")?;

                for item in items {
                    item.format_unexpanded(s, tab_lvl, config)?;
                    write!(s, "; ")?;
                }
                s.pop();

                write!(s, "{close}")?;
            }

            Self::SpacedDelimsStmts(open, items, close) => {
                write!(s, "{open} ")?;

                for item in items {
                    item.format_unexpanded(s, tab_lvl, config)?;
                    write!(s, "; ")?;
                }

                write!(s, "{close}")?;
            }

            Self::Items(items) => {
                for item in items {
                    write!(s, "\n")?;
                    item.format_unexpanded(s, tab_lvl, config)?;
                    write!(s, "\n")?;
                }
            }
        };

        Ok(())
    }

    pub fn format_expanded(&self, s: &mut String, tab_lvl: u32, config: &FormatConfig) -> std::fmt::Result {
        match self {
            Self::Atom(str) => write!(s, "{str}")?,

            Self::Chain(items) => {
                for item in items {
                    item.format_unexpanded(s, tab_lvl, config)?;
                }
            }

            Self::DotChain(items) => {
                for item in items {
                    item.format_unexpanded(s, tab_lvl, config)?;
                    write!(s, ".")?;
                }

                s.pop();
            }

            Self::DenseDelims(open, inner, close) => {
                write!(s, "{open}")?;
                inner.format_unexpanded(s, tab_lvl, config)?;
                write!(s, "{close}")?;
            }

            Self::SpacedDelims(open, inner, close) => {
                write!(s, "{open} ")?;
                inner.format_unexpanded(s, tab_lvl, config)?;
                write!(s, " {close}")?;
            }

            Self::DenseDelimsList(open, items, close) => {
                write!(s, "{open}")?;

                for item in items {
                    item.format_unexpanded(s, tab_lvl, config)?;
                    write!(s, ", ")?;
                }
                s.pop();
                s.pop();

                write!(s, "{close}")?;
            }

            Self::SpacedDelimsList(open, items, close) => {
                write!(s, "{open} ")?;

                for item in items {
                    item.format_unexpanded(s, tab_lvl, config)?;
                    write!(s, ", ")?;
                }
                s.pop();
                s.pop();

                write!(s, " {close}")?;
            }

            Self::DenseDelimsStmts(open, items, close) => {
                write!(s, "{open}")?;

                for item in items {
                    item.format_unexpanded(s, tab_lvl, config)?;
                    write!(s, "; ")?;
                }
                s.pop();

                write!(s, "{close}")?;
            }

            Self::SpacedDelimsStmts(open, items, close) => {
                write!(s, "{open} ")?;

                for item in items {
                    item.format_unexpanded(s, tab_lvl, config)?;
                    write!(s, "; ")?;
                }

                write!(s, "{close}")?;
            }

            Self::Items(items) => {
                for item in items {
                    write!(s, "\n")?;
                    item.format_unexpanded(s, tab_lvl, config)?;
                    write!(s, "\n")?;
                }
            }
        };

        Ok(())
    }
}
