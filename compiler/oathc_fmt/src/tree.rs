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
    SpacedChain(FormatSpacing, Vec<FormatTree>),

    DenseDelims(&'static str, Box<FormatTree>, String, &'static str),
    SpacedDelims(&'static str, Box<FormatTree>, String, &'static str),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormatSpacing {
    Space,
    Line,
    TwoLines,
    SpaceOrLine,
    SpaceOrLineTab,
    LineOrTwo,
    Colon,
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
            Self::SpacedChain(spacing, items) => {
                let space_len = match spacing {
                    FormatSpacing::Colon => 2,

                    FormatSpacing::Line
                    | FormatSpacing::LineOrTwo
                    | FormatSpacing::Space
                    | FormatSpacing::SpaceOrLine
                    | FormatSpacing::SpaceOrLineTab
                    | FormatSpacing::TwoLines => 1,
                };

                Itertools::intersperse(items.iter().map(|item| FormatTree::unexpanded_len(item, config)), space_len).sum::<u32>()
            }

            Self::DenseDelims(open, inner, _, close) => open.len() as u32 + inner.unexpanded_len(config) + close.len() as u32,
            Self::SpacedDelims(open, inner, _, close) => {
                open.len() as u32 + 1 + inner.unexpanded_len(config) + 1 + close.len() as u32
            }
        }
    }

    fn should_expand(&self, config: &FormatConfig) -> bool {
        match self {
            Self::Chain(items) => {
                self.unexpanded_len(config) > config.max_width || items.iter().any(|item| item.should_expand(config))
            }

            Self::SpacedChain(spacing, items) => match spacing {
                FormatSpacing::Line | FormatSpacing::TwoLines | FormatSpacing::LineOrTwo => true,

                FormatSpacing::Colon | FormatSpacing::SpaceOrLine | FormatSpacing::SpaceOrLineTab => {
                    self.unexpanded_len(config) > config.max_width || items.iter().any(|item| item.should_expand(config))
                }

                FormatSpacing::Space => false,
            },

            Self::TryFailure | Self::AtomStr(_) | Self::AtomString(_) | Self::None => false,

            Self::DenseDelims(_, inner, _, _) | Self::SpacedDelims(_, inner, _, _) => inner.should_expand(config),
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            Self::None => true,

            Self::TryFailure
            | Self::AtomStr(_)
            | Self::AtomString(_)
            | Self::DenseDelims(_, _, _, _)
            | Self::SpacedDelims(_, _, _, _) => false,

            Self::Chain(items) | Self::SpacedChain(_, items) => items.iter().all(FormatTree::is_empty),
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
            Self::SpacedChain(spacing, items) => {
                let space = match spacing {
                    FormatSpacing::Colon => ", ",
                    FormatSpacing::Line
                    | FormatSpacing::LineOrTwo
                    | FormatSpacing::Space
                    | FormatSpacing::SpaceOrLine
                    | FormatSpacing::SpaceOrLineTab
                    | FormatSpacing::TwoLines => "",
                };

                format_unexpanded_sep(s, config, items, space)?;
            }

            Self::DenseDelims(open, inner, leftovers, close) => {
                write!(s, "{open}")?;
                inner.format_unexpanded(s, config)?;
                write!(s, "{leftovers}{close}")?;
            }
            Self::SpacedDelims(open, inner, leftovers, close) => {
                write!(s, "{open} ")?;
                inner.format_unexpanded(s, config)?;
                write!(s, " {leftovers}{close}")?;
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
            Self::SpacedChain(spacing, items) => {
                for (item_idx, item) in clean_iter(items).enumerate() {
                    if item_idx > 0 {
                        match spacing {
                            FormatSpacing::Colon => write!(s, ",\n{tabs}")?,
                            FormatSpacing::Line => write!(s, "\n{tabs}")?,
                            FormatSpacing::LineOrTwo => write!(s, "\n{tabs}")?,
                            FormatSpacing::Space => write!(s, " ")?,
                            FormatSpacing::SpaceOrLine => write!(s, "\n{tabs}")?,
                            FormatSpacing::SpaceOrLineTab => write!(s, "\n{tabs}\t")?,
                            FormatSpacing::TwoLines => write!(s, "\n\n{tabs}")?,
                        }
                    }

                    item.format_inner(s, tab_lvl, config)?;
                }

                match spacing {
                    FormatSpacing::Colon => write!(s, ",\n{tabs}")?,

                    FormatSpacing::Line
                    | FormatSpacing::LineOrTwo
                    | FormatSpacing::Space
                    | FormatSpacing::SpaceOrLine
                    | FormatSpacing::SpaceOrLineTab
                    | FormatSpacing::TwoLines => {}
                }
            }

            Self::DenseDelims(open, inner, leftovers, close) | Self::SpacedDelims(open, inner, leftovers, close) => {
                write!(s, "{open}\n{tabs}\t")?;
                inner.format_expanded(s, tab_lvl + 1, config)?;
                write!(s, "\n{tabs}{leftovers}")?;
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
