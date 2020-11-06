use std::fmt::{self, Display};

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Style {
    Code,
    List,
    Quote,
    Text,
}

impl Default for Style {
    fn default() -> Self {
        Style::Text
    }
}

impl Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Style::Code => f.write_str("code"),
            Style::List => f.write_str("list"),
            Style::Quote => f.write_str("quote"),
            Style::Text => f.write_str("text"),
        }
    }
}

#[derive(Debug, Error, Clone)]
#[error("invalid send style")]
pub struct ParseStyleError(());

impl std::str::FromStr for Style {
    type Err = ParseStyleError;

    fn from_str(s: &str) -> Result<Style, Self::Err> {
        match s {
            "code" | "Code" => Ok(Style::Code),
            "list" | "List" => Ok(Style::List),
            "quote" | "Quote" => Ok(Style::Quote),
            "text" | "Text" => Ok(Style::Text),
            _ => Err(ParseStyleError(())),
        }
    }
}
