use std::time::Duration;

use anyhow::{Context, Result};
use reqwest::{redirect::Policy, Url};

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

    if !response.status().is_success() {
        anyhow::bail!("HTTP request failed with status: {}", response.status());
    }

    let html = response.text().context("Failed to read response body")?;

    if html.trim().is_empty() {
        anyhow::bail!("Received empty response body");
    }

    Ok(html)
}

pub fn extract_site(url: &str) -> Option<String> {
    let url = Url::parse(&url).ok()?;
    let host = url.host_str()?;
    let host = host.strip_prefix("www.").unwrap_or(host);

    Some(host.to_string())
}
