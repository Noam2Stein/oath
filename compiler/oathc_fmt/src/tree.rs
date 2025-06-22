use std::fmt::Write;

use itertools::Itertools;

use super::*;

#[derive(Debug, Clone)]
pub enum FormatTree {
    None,

    Atom(String),
    Chain(Vec<FormatTree>),
    LineChain(Vec<FormatTree>),
    SpacedLineChain(Vec<FormatTree>),

    DotChain(Vec<FormatTree>),
    Assign(Box<FormatTree>, Box<FormatTree>),

    DenseDelims(char, Box<FormatTree>, char),
    DenseDelimsList(char, Vec<FormatTree>, char),

    SpacedDelims(char, Box<FormatTree>, char),
    SpacedDelimsList(char, Vec<FormatTree>, char),
    SpacedDelimsStmts(char, Vec<FormatTree>, char),
}

impl FormatTree {
    pub fn unexpanded_len(&self) -> u32 {
        match self {
            Self::None => 0,

            Self::Atom(str) => str.chars().count() as u32,
            Self::Chain(items) => items.iter().map(FormatTree::unexpanded_len).sum::<u32>(),
            Self::LineChain(items) | Self::SpacedLineChain(items) => {
                Itertools::intersperse(items.iter().map(FormatTree::unexpanded_len), 1).sum::<u32>()
            }

            Self::DotChain(items) => Itertools::intersperse(items.iter().map(FormatTree::unexpanded_len), 1).sum::<u32>(),
            Self::Assign(lhs, rhs) => lhs.unexpanded_len() + 3 + rhs.unexpanded_len(),

            Self::DenseDelims(_, inner, _) => 2 + inner.unexpanded_len(),
            Self::DenseDelimsList(_, items, _) => {
                2 + Itertools::intersperse(items.iter().map(FormatTree::unexpanded_len), 2).sum::<u32>()
            }

            Self::SpacedDelims(_, inner, _) => 4 + inner.unexpanded_len(),
            Self::SpacedDelimsList(_, items, _) => {
                4 + Itertools::intersperse(items.iter().map(FormatTree::unexpanded_len), 2).sum::<u32>()
            }
            Self::SpacedDelimsStmts(_, items, _) => {
                4 + Itertools::intersperse(items.iter().map(FormatTree::unexpanded_len), 2).sum::<u32>() + 1
            }
        }
    }

    pub fn format(&self, s: &mut String, tab_lvl: u32, config: &FormatConfig) -> std::fmt::Result {
        if self.unexpanded_len() > config.max_width {
            self.format_expanded(s, tab_lvl, config)
        } else {
            self.format_unexpanded(s, config)
        }
    }

    pub fn format_unexpanded(&self, s: &mut String, config: &FormatConfig) -> std::fmt::Result {
        match self {
            Self::None => {}

            Self::Atom(str) => write!(s, "{str}")?,

            Self::Chain(items) => {
                for item in items {
                    item.format_unexpanded(s, config)?;
                }
            }

            Self::LineChain(items) | Self::SpacedLineChain(items) => {
                for (item_idx, item) in items.iter().enumerate() {
                    if item_idx > 0 {
                        write!(s, " ")?;
                    }

                    item.format_unexpanded(s, config)?;
                }
            }

            Self::DotChain(items) => {
                for item in items {
                    item.format_unexpanded(s, config)?;
                    write!(s, ".")?;
                }

                s.pop();
            }

            Self::Assign(lhs, rhs) => {
                lhs.format_unexpanded(s, config)?;
                write!(s, " = ")?;
                rhs.format_unexpanded(s, config)?;
            }

            Self::DenseDelims(open, inner, close) => {
                write!(s, "{open}")?;
                inner.format_unexpanded(s, config)?;
                write!(s, "{close}")?;
            }

            Self::DenseDelimsList(open, items, close) => {
                write!(s, "{open}")?;

                for (item_idx, item) in items.iter().enumerate() {
                    if item_idx > 0 {
                        write!(s, ", ")?;
                    }

                    item.format_unexpanded(s, config)?;
                }

                write!(s, "{close}")?;
            }

            Self::SpacedDelims(open, inner, close) => {
                write!(s, "{open} ")?;
                inner.format_unexpanded(s, config)?;
                write!(s, " {close}")?;
            }

            Self::SpacedDelimsList(open, items, close) => {
                write!(s, "{open} ")?;

                for (item_idx, item) in items.iter().enumerate() {
                    if item_idx > 0 {
                        write!(s, ", ")?;
                    }

                    item.format_unexpanded(s, config)?;
                }

                write!(s, " {close}")?;
            }

            Self::SpacedDelimsStmts(open, items, close) => {
                write!(s, "{open} ")?;

                for item in items {
                    item.format_unexpanded(s, config)?;
                    write!(s, "; ")?;
                }

                write!(s, "{close}")?;
            }
        };

        Ok(())
    }

    pub fn format_expanded(&self, s: &mut String, tab_lvl: u32, config: &FormatConfig) -> std::fmt::Result {
        let tabs = "\t".repeat(tab_lvl as usize);

        match self {
            Self::Atom(str) => write!(s, "{str}")?,

            Self::Chain(items) => {
                for item in items {
                    item.format(s, tab_lvl, config)?;
                }
            }

            Self::DotChain(items) => {
                items[0].format(s, tab_lvl, config)?;

                for item in items.iter().skip(1) {
                    write!(s, "\n{tabs}\t.")?;

                    item.format(s, tab_lvl + 1, config)?;
                }
            }

            Self::DenseDelims(open, inner, close) => {
                write!(s, "{open}\n{tabs}\t")?;
                inner.format(s, tab_lvl + 1, config)?;
                write!(s, "\n{tabs}{close}")?;
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
