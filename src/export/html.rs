use anyhow::{Context, Result};
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::db::models::Article;
use crate::ui::formatters::datetime_humanize;

pub fn export_to_html(articles: &[Article], output_path: &Path) -> Result<()> {
    let html = generate_html(articles);
    
    let mut file = File::create(output_path)
        .context(format!("Failed to create file: {}", output_path.display()))?;
    
    file.write_all(html.as_bytes())
        .context("Failed to write HTML to file")?;
    
    Ok(())
}

fn generate_html(articles: &[Article]) -> String {
    let mut html = String::from(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Stash Export</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
            line-height: 1.6;
            color: #333;
            background: #f5f5f5;
            padding: 20px;
        }
        .container {
            max-width: 1200px;
            margin: 0 auto;
            background: white;
            padding: 30px;
            border-radius: 8px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        }
        h1 {
            color: #2c3e50;
            margin-bottom: 30px;
            font-size: 2.5em;
            border-bottom: 3px solid #3498db;
            padding-bottom: 10px;
        }
        .stats {
            display: flex;
            gap: 20px;
            margin-bottom: 30px;
            padding: 20px;
            background: #ecf0f1;
            border-radius: 6px;
        }
        .stat {
            flex: 1;
            text-align: center;
        }
        .stat-number {
            font-size: 2em;
            font-weight: bold;
            color: #3498db;
        }
        .stat-label {
            color: #7f8c8d;
            font-size: 0.9em;
            text-transform: uppercase;
        }
        .article {
            border: 1px solid #e0e0e0;
            border-radius: 6px;
            padding: 20px;
            margin-bottom: 20px;
            background: white;
            transition: box-shadow 0.2s;
        }
        .article:hover {
            box-shadow: 0 4px 8px rgba(0,0,0,0.1);
        }
        .article-header {
            display: flex;
            align-items: center;
            margin-bottom: 12px;
            gap: 10px;
        }
        .article-id {
            background: #3498db;
            color: white;
            padding: 4px 10px;
            border-radius: 4px;
            font-weight: bold;
            font-size: 0.9em;
        }
        .article-title {
            font-size: 1.4em;
            color: #2c3e50;
            text-decoration: none;
            font-weight: 600;
            flex: 1;
        }
        .article-title:hover {
            color: #3498db;
        }
        .badges {
            display: flex;
            gap: 8px;
            margin-bottom: 12px;
        }
        .badge {
            padding: 4px 10px;
            border-radius: 4px;
            font-size: 0.85em;
            font-weight: 500;
        }
        .badge-starred {
            background: #f39c12;
            color: white;
        }
        .badge-read {
            background: #27ae60;
            color: white;
        }
        .badge-unread {
            background: #e74c3c;
            color: white;
        }
        .badge-archived {
            background: #95a5a6;
            color: white;
        }
        .article-meta {
            display: flex;
            gap: 20px;
            color: #7f8c8d;
            font-size: 0.9em;
            margin-bottom: 12px;
        }
        .article-description {
            color: #555;
            margin-bottom: 12px;
        }
        .article-note {
            background: #fff9e6;
            border-left: 4px solid #f39c12;
            padding: 12px;
            margin-top: 12px;
            border-radius: 4px;
        }
        .article-note-title {
            font-weight: bold;
            color: #f39c12;
            margin-bottom: 6px;
        }
        .tags {
            display: flex;
            gap: 8px;
            flex-wrap: wrap;
            margin-top: 12px;
        }
        .tag {
            background: #ecf0f1;
            color: #2c3e50;
            padding: 4px 12px;
            border-radius: 20px;
            font-size: 0.85em;
            font-weight: 500;
        }
        .article-url {
            color: #3498db;
            text-decoration: none;
            word-break: break-all;
        }
        .article-url:hover {
            text-decoration: underline;
        }
        .filter-buttons {
            display: flex;
            gap: 10px;
            margin-bottom: 20px;
        }
        .filter-btn {
            padding: 8px 16px;
            border: 2px solid #3498db;
            background: white;
            color: #3498db;
            border-radius: 6px;
            cursor: pointer;
            font-weight: 500;
            transition: all 0.2s;
        }
        .filter-btn:hover, .filter-btn.active {
            background: #3498db;
            color: white;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>📚 Stash Export</h1>
"#);

    // Calculate stats
    let total = articles.len();
    let unread = articles.iter().filter(|a| !a.read).count();
    let starred = articles.iter().filter(|a| a.starred).count();
    let archived = articles.iter().filter(|a| a.archived).count();
    
    html.push_str(&format!(r#"
        <div class="stats">
            <div class="stat">
                <div class="stat-number">{}</div>
                <div class="stat-label">Total</div>
            </div>
            <div class="stat">
                <div class="stat-number">{}</div>
                <div class="stat-label">Unread</div>
            </div>
            <div class="stat">
                <div class="stat-number">{}</div>
                <div class="stat-label">Starred</div>
            </div>
            <div class="stat">
                <div class="stat-number">{}</div>
                <div class="stat-label">Archived</div>
            </div>
        </div>
        
        <div class="filter-buttons">
            <button class="filter-btn active" onclick="filterArticles('all')">All</button>
            <button class="filter-btn" onclick="filterArticles('unread')">Unread</button>
            <button class="filter-btn" onclick="filterArticles('starred')">Starred</button>
            <button class="filter-btn" onclick="filterArticles('archived')">Archived</button>
        </div>
        
        <div id="articles">
"#, total, unread, starred, archived));
    
    // Generate article cards
    for article in articles {
        let title = html_escape(article.title.as_deref().unwrap_or("Untitled"));
        let url = html_escape(&article.url);
        let site = article.site.as_deref().unwrap_or("Unknown");
        let date = datetime_humanize(article.saved_at);
        
        let mut classes = vec!["article"];
        if article.read { classes.push("read"); } else { classes.push("unread"); }
        if article.starred { classes.push("starred"); }
        if article.archived { classes.push("archived"); }
        
        html.push_str(&format!(r#"
            <div class="{}" data-categories="{}">
                <div class="article-header">
                    <span class="article-id">#{}</span>
                    <a href="{}" class="article-title" target="_blank">{}</a>
                </div>
                <div class="badges">
"#, classes.join(" "), classes.join(" "), article.id, url, title));
        
        if article.starred {
            html.push_str(r#"<span class="badge badge-starred">⭐ Starred</span>"#);
        }
        if article.read {
            html.push_str(r#"<span class="badge badge-read">✓ Read</span>"#);
        } else {
            html.push_str(r#"<span class="badge badge-unread">● Unread</span>"#);
        }
        if article.archived {
            html.push_str(r#"<span class="badge badge-archived">🗑 Archived</span>"#);
        }
        
        html.push_str("</div>");
        
        html.push_str(&format!(r#"
                <div class="article-meta">
                    <span>📍 {}</span>
                    <span>🕒 {}</span>
                </div>
"#, html_escape(site), date));
        
        if let Some(desc) = &article.description {
            html.push_str(&format!(r#"
                <div class="article-description">{}</div>
"#, html_escape(desc)));
        }
        
        if let Some(note) = &article.note {
            html.push_str(&format!(r#"
                <div class="article-note">
                    <div class="article-note-title">📝 Personal Note</div>
                    <div>{}</div>
                </div>
"#, html_escape(note)));
        }
        
        if !article.tags.is_empty() {
            html.push_str(r#"<div class="tags">"#);
            for tag in &article.tags {
                html.push_str(&format!(r#"<span class="tag">{}</span>"#, html_escape(tag)));
            }
            html.push_str("</div>");
        }
        
        html.push_str(r#"
                <div style="margin-top: 12px;">
                    <a href="{url}" class="article-url" target="_blank">{url}</a>
                </div>
            </div>
"#);
    }
    
    html.push_str(r#"
        </div>
    </div>
    
    <script>
        function filterArticles(category) {
            const articles = document.querySelectorAll('.article');
            const buttons = document.querySelectorAll('.filter-btn');
            
            buttons.forEach(btn => btn.classList.remove('active'));
            event.target.classList.add('active');
            
            articles.forEach(article => {
                if (category === 'all') {
                    article.style.display = 'block';
                } else if (article.classList.contains(category)) {
                    article.style.display = 'block';
                } else {
                    article.style.display = 'none';
                }
            });
        }
    </script>
</body>
</html>
"#);
    
    html
}

fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

