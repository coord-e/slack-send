use std::fmt::Display;

use anyhow::{bail, Result};
use reqwest::{blocking::Client, IntoUrl};
use serde_json::json;

pub fn send(url: impl IntoUrl, message: impl Display) -> Result<()> {
    let response = Client::builder()
        .build()?
        .post(url)
        .json(&json!({ "text": message.to_string() }))
        .send()?;
    if !response.status().is_success() {
        bail!("request failed with {}", response.status());
    }
    Ok(())
}
