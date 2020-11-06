use std::io::{self, Read};

use anyhow::{Context, Result};
use structopt::StructOpt;

use slack_send::{Mention, Message, Style};

#[derive(StructOpt)]
struct Opt {
    #[structopt(short, long, env = "SLACK_SEND_WEBHOOK_URL")]
    webhook_url: String,
    #[structopt(short, long, default_value, env = "SLACK_SEND_STYLE")]
    style: Style,
    #[structopt(short, long)]
    content: Option<String>,
    #[structopt(short, long, env = "SLACK_SEND_MENTION")]
    mention: Option<Mention>,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let content = if let Some(content) = opt.content {
        content
    } else {
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .context("reading stdin")?;
        buffer
    };

    slack_send::send(
        &opt.webhook_url,
        Message {
            content,
            style: opt.style,
            mention: opt.mention,
        },
    )
}
