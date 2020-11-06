use std::fmt::{self, Display};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mention {
    Here,
    Everyone,
    Channel,
    User(String),
}

impl std::str::FromStr for Mention {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Mention, Self::Err> {
        match s {
            "here" | "Here" => Ok(Mention::Here),
            "everyone" | "Everyone" => Ok(Mention::Everyone),
            "channel" | "Channel" => Ok(Mention::Channel),
            s => Ok(Mention::User(s.to_string())),
        }
    }
}

impl Display for Mention {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Mention::Here => f.write_str("<!here|here>"),
            Mention::Everyone => f.write_str("<!everyone>"),
            Mention::Channel => f.write_str("<!channel>"),
            Mention::User(user) => write!(f, "<@{}>", user),
        }
    }
}
