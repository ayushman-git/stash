use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub id: i64,
    pub hash: String,
    pub url: String,
    pub canonical_url: String,
    pub title: Option<String>,
    pub site: Option<String>,
    pub description: Option<String>,
    pub favicon_url: Option<String>,
    pub content_markdown: Option<String>,
    pub saved_at: DateTime<Utc>,
    pub last_opened_at: Option<DateTime<Utc>>,
    pub read: bool,
    pub archived: bool,
    pub starred: bool,
    pub note: Option<String>,
    pub tags: Vec<String>,
}

pub struct NewArticle {
    pub hash: String,
    pub url: String,
    pub canonical_url: String,
    pub title: Option<String>,
    pub site: Option<String>,
    pub description: Option<String>,
    pub favicon_url: Option<String>,
    pub content_markdown: Option<String>,
    pub tags: Vec<String>,
}
