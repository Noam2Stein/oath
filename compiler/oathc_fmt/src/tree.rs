use std::fmt::Write;

use itertools::Itertools;

use super::*;

#[derive(Debug, Clone)]
pub enum FormatTree {
    None,
    TryFailure,

    AtomString(String),
    AtomStr(&'static str),

    Chain(Vec<FormatTree>),
    SpacedChain(Vec<FormatTree>),

    LineChain(Vec<FormatTree>),
    SpacedLineChain(Vec<FormatTree>),

    List(Vec<FormatTree>),
    DotChain(Vec<FormatTree>),
    Assign(Box<FormatTree>, Box<FormatTree>),

    DenseDelims(&'static str, Box<FormatTree>, &'static str),
    SpacedDelims(&'static str, Box<FormatTree>, &'static str),
}

impl FormatTree {
    pub fn format(&self, config: &FormatConfig) -> String {
        let mut s = String::new();

        self.format_inner(&mut s, 0, config).unwrap();

        s
    }
}

impl FormatTree {
    fn unexpanded_len(&self, config: &FormatConfig) -> u32 {
        match self {
            Self::None => 0,
            Self::TryFailure => 0,
            Self::AtomString(str) => str.chars().count() as u32,
            Self::AtomStr(str) => str.chars().count() as u32,

            Self::Chain(items) => items.iter().map(|item| FormatTree::unexpanded_len(item, config)).sum::<u32>(),
            Self::SpacedChain(items) | Self::LineChain(items) | Self::SpacedLineChain(items) => {
                Itertools::intersperse(items.iter().map(|item| FormatTree::unexpanded_len(item, config)), 1).sum::<u32>()
            }

            Self::DotChain(items) => {
                Itertools::intersperse(items.iter().map(|item| FormatTree::unexpanded_len(item, config)), 1).sum::<u32>()
            }
            Self::Assign(lhs, rhs) => lhs.unexpanded_len(config) + 3 + rhs.unexpanded_len(config),

            Self::List(items) => {
                Itertools::intersperse(items.iter().map(|item| FormatTree::unexpanded_len(item, config)), 2).sum::<u32>()
            }

            Self::DenseDelims(open, inner, close) => open.len() as u32 + inner.unexpanded_len(config) + close.len() as u32,
            Self::SpacedDelims(open, inner, close) => {
                open.len() as u32 + 1 + inner.unexpanded_len(config) + 1 + close.len() as u32
            }
        }
    }

    fn should_expand(&self, config: &FormatConfig) -> bool {
        match self {
            Self::Chain(_) | Self::SpacedChain(_) | Self::DotChain(_) | Self::List(_) | Self::Assign(_, _) => {
                self.unexpanded_len(config) > config.max_width
            }

            Self::TryFailure | Self::AtomStr(_) | Self::AtomString(_) | Self::None => false,

            Self::LineChain(_) | Self::SpacedLineChain(_) => true,

            Self::DenseDelims(_, inner, _) | Self::SpacedDelims(_, inner, _) => inner.should_expand(config),
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            Self::None => true,

            Self::TryFailure
            | Self::AtomStr(_)
            | Self::AtomString(_)
            | Self::Assign(_, _)
            | Self::DenseDelims(_, _, _)
            | Self::SpacedDelims(_, _, _) => false,

            Self::Chain(items)
            | Self::DotChain(items)
            | Self::LineChain(items)
            | Self::List(items)
            | Self::SpacedChain(items)
            | Self::SpacedLineChain(items) => items.iter().all(FormatTree::is_empty),
        }
    }

    fn format_inner(&self, s: &mut String, tab_lvl: u32, config: &FormatConfig) -> std::fmt::Result {
        if self.should_expand(config) {
            self.format_expanded(s, tab_lvl, config)
        } else {
            self.format_unexpanded(s, config)
        }
    }

    fn format_unexpanded(&self, s: &mut String, config: &FormatConfig) -> std::fmt::Result {
        match self {
            Self::None | Self::TryFailure => {}
            Self::AtomString(str) => write!(s, "{str}")?,
            Self::AtomStr(str) => write!(s, "{str}")?,

            Self::Chain(items) => format_unexpanded_sep(s, config, items, "")?,
            Self::SpacedChain(items) | Self::LineChain(items) | Self::SpacedLineChain(items) => {
                format_unexpanded_sep(s, config, items, " ")?
            }

            Self::List(items) => format_unexpanded_sep(s, config, items, ", ")?,
            Self::DotChain(items) => format_unexpanded_sep(s, config, items, ".")?,
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
            Self::SpacedDelims(open, inner, close) => {
                write!(s, "{open} ")?;
                inner.format_unexpanded(s, config)?;
                write!(s, " {close}")?;
            }
        };

        Ok(())
    }

    fn format_expanded(&self, s: &mut String, tab_lvl: u32, config: &FormatConfig) -> std::fmt::Result {
        let tabs = "\t".repeat(tab_lvl as usize);

        match self {
            Self::None | Self::TryFailure => {}
            Self::AtomString(str) => write!(s, "{str}")?,
            Self::AtomStr(str) => write!(s, "{str}")?,

            Self::Chain(items) => {
                for item in clean_iter(items) {
                    item.format_inner(s, tab_lvl, config)?;
                }
            }
            Self::SpacedChain(items) => {
                for (item_idx, item) in clean_iter(items).enumerate() {
                    if item_idx > 0 {
                        write!(s, " ")?;
                    }

                    item.format_inner(s, tab_lvl, config)?;
                }
            }

            Self::LineChain(items) => {
                for (item_idx, item) in clean_iter(items).enumerate() {
                    if item_idx > 0 {
                        write!(s, "\n{tabs}")?;
                    }

                    item.format_inner(s, tab_lvl, config)?;
                }
            }
            Self::SpacedLineChain(items) => {
                for (item_idx, item) in clean_iter(items).enumerate() {
                    if item_idx > 0 {
                        write!(s, "\n\n{tabs}")?;
                    }

                    item.format_inner(s, tab_lvl, config)?;
                }
            }

            Self::DotChain(items) => {
                for (item_idx, item) in clean_iter(items).enumerate() {
                    if item_idx == 0 {
                        item.format_inner(s, tab_lvl, config)?;
                    } else {
                        write!(s, "\n{tabs}\t.")?;
                        item.format_inner(s, tab_lvl + 1, config)?;
                    }
                }
            }
            Self::List(items) => {
                for (item_idx, item) in clean_iter(items).enumerate() {
                    if item_idx > 0 {
                        write!(s, "\n{tabs}")?;
                    }

                    item.format_expanded(s, tab_lvl, config)?;
                    write!(s, ",")?;
                }
            }
            Self::Assign(lhs, rhs) => {
                lhs.format_inner(s, tab_lvl, config)?;
                write!(s, "\n{tabs}\t = ")?;
                rhs.format_inner(s, tab_lvl + 1, config)?;
            }

            Self::DenseDelims(open, inner, close) | Self::SpacedDelims(open, inner, close) => {
                write!(s, "{open}\n{tabs}\t")?;
                inner.format_expanded(s, tab_lvl + 1, config)?;
                write!(s, "\n{tabs}{close}")?;
            }
        };

        Ok(())
    }
}

fn format_unexpanded_sep<'f>(
    s: &mut String,
    config: &FormatConfig,
    items: impl IntoIterator<Item = &'f FormatTree>,
    sep: &str,
) -> std::fmt::Result {
    let mut iter = clean_iter(items.into_iter());

    if let Some(first) = iter.next() {
        first.format_unexpanded(s, config)?;
    }

    for item in iter {
        *s += sep;
        item.format_unexpanded(s, config)?;
    }

    Ok(())
}

fn clean_iter<'f>(input: impl IntoIterator<Item = &'f FormatTree>) -> impl Iterator<Item = &'f FormatTree> {
    input.into_iter().filter(|item| !item.is_empty())
}
