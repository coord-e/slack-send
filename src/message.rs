use std::fmt::{self, Display};

use crate::{Mention, Style};

pub struct Message {
    pub content: String,
    pub style: Style,
    pub mention: Option<Mention>,
}

impl Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(mention) = &self.mention {
            writeln!(f, "{}", mention)?;
        }

        match self.style {
            Style::Code => write!(f, "```{}```", self.content),
            Style::Quote => {
                for line in self.content.lines() {
                    writeln!(f, "> {}", line)?;
                }
                Ok(())
            }
            Style::List => {
                for line in self.content.lines() {
                    writeln!(f, "- {}", line)?;
                }
                Ok(())
            }
            Style::Text => write!(f, "{}", self.content),
        }
    }
}
