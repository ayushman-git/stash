use std::time::Duration;

use anyhow::{Context, Result};
use reqwest::redirect::Policy;

pub fn fetch_html(url: &String) -> Result<String> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("Mozilla/5.0 (compatible; Stash/1.0)")
        .redirect(Policy::limited(10))
        .timeout(Duration::from_secs(10))
        .build()
        .context("Failed to build HTTP client")?;

    let response = client
        .get(url)
        .send()
        .context("Failed to send HTTP request")?;

    let html = response.text().context("Failed to read responsy body")?;

    Ok(html)
}
