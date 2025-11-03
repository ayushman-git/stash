use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub defaults: Defaults,
    
    #[serde(default)]
    pub colors: Colors,
    
    #[serde(default)]
    pub fetch: Fetch,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            defaults: Defaults::default(),
            colors: Colors::default(),
            fetch: Fetch::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Defaults {
    #[serde(default = "default_editor")]
    pub editor: String,
    
    #[serde(default = "default_browser")]
    pub browser: String,
    
    #[serde(default = "default_output_format")]
    pub output_format: String,
    
    #[serde(default = "default_list_limit")]
    pub list_limit: i64,
    
    #[serde(default = "default_auto_read")]
    pub auto_read: bool,
}

impl Default for Defaults {
    fn default() -> Self {
        Self {
            editor: default_editor(),
            browser: default_browser(),
            output_format: default_output_format(),
            list_limit: default_list_limit(),
            auto_read: default_auto_read(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Colors {
    #[serde(default = "default_theme")]
    pub theme: String,
}

impl Default for Colors {
    fn default() -> Self {
        Self {
            theme: default_theme(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fetch {
    #[serde(default = "default_timeout_seconds")]
    pub timeout_seconds: u64,
    
    #[serde(default = "default_follow_redirects")]
    pub follow_redirects: bool,
    
    #[serde(default = "default_user_agent")]
    pub user_agent: String,
}

impl Default for Fetch {
    fn default() -> Self {
        Self {
            timeout_seconds: default_timeout_seconds(),
            follow_redirects: default_follow_redirects(),
            user_agent: default_user_agent(),
        }
    }
}

// Default functions for serde
fn default_editor() -> String {
    std::env::var("EDITOR")
        .or_else(|_| std::env::var("VISUAL"))
        .unwrap_or_else(|_| "vim".to_string())
}

fn default_browser() -> String {
    "default".to_string()
}

fn default_output_format() -> String {
    "table".to_string()
}

fn default_list_limit() -> i64 {
    10
}

fn default_auto_read() -> bool {
    true
}

fn default_theme() -> String {
    "auto".to_string()
}

fn default_timeout_seconds() -> u64 {
    10
}

fn default_follow_redirects() -> bool {
    true
}

fn default_user_agent() -> String {
    "Stash/0.1.0".to_string()
}

