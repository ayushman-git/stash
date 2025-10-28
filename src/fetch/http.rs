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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_site_removes_www() {
        assert_eq!(
            extract_site("https://www.example.com/article"),
            Some("example.com".to_string())
        );
    }

    #[test]
    fn test_extract_site_keeps_subdomain() {
        assert_eq!(
            extract_site("https://api.github.com/repos"),
            Some("api.github.com".to_string())
        );
    }

    #[test]
    fn test_extract_site_handles_port() {
        assert_eq!(
            extract_site("http://localhost:8080/path"),
            Some("localhost".to_string())
        );
    }

    #[test]
    fn test_extract_site_handles_ip_address() {
        assert_eq!(
            extract_site("http://192.168.1.1/path"),
            Some("192.168.1.1".to_string())
        );
    }

    #[test]
    fn test_extract_site_returns_none_for_invalid_url() {
        assert_eq!(extract_site("not-a-valid-url"), None);
    }

    #[test]
    fn test_extract_site_handles_file_url() {
        assert_eq!(extract_site("file:///home/user/file.html"), None);
    }

    #[test]
    fn test_extract_site_basic_domain() {
        assert_eq!(
            extract_site("https://example.com"),
            Some("example.com".to_string())
        );
    }
}
