use anyhow::Result;
use tiny_http::{Response, Server};

use crate::db::models::Article;
use crate::ui::formatters::datetime_humanize;

pub fn render_browser(articles: &[Article], all: bool, archived: bool) -> Result<()> {
    let html = generate_html(articles, all, archived);
    start_server(html)
}

fn generate_html(articles: &[Article], all: bool, archived: bool) -> String {
    let mut article_rows = String::new();
    
    for article in articles {
        let title = article.title.as_deref().unwrap_or("<no title>");
        let site = article.site.as_deref().unwrap_or("-");
        let saved = datetime_humanize(article.saved_at);
        let url = &article.url;
        
        // Determine status badge
        let status_badge = if article.archived {
            r#"<span class="badge badge-archived">Archived</span>"#
        } else if article.read {
            r#"<span class="badge badge-read">Read</span>"#
        } else {
            r#"<span class="badge badge-unread">Unread</span>"#
        };
        
        let star_icon = if article.starred { 
            r#"<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor" style="color: #fbbf24;">
                <path d="M8 .25a.75.75 0 01.673.418l1.882 3.815 4.21.612a.75.75 0 01.416 1.279l-3.046 2.97.719 4.192a.75.75 0 01-1.088.791L8 12.347l-3.766 1.98a.75.75 0 01-1.088-.79l.72-4.194L.818 6.374a.75.75 0 01.416-1.28l4.21-.611L7.327.668A.75.75 0 018 .25z"/>
            </svg>"#
        } else { 
            "" 
        };
        
        // Format tags with different colors
        let tags_html = if article.tags.is_empty() {
            r#"<span class="no-tags">-</span>"#.to_string()
        } else {
            format_tags_colored(&article.tags)
        };
        
        let archived_cell = if all || archived {
            if article.archived {
                r#"<td class="text-center"><span class="icon-archived">🗑️</span></td>"#
            } else {
                r#"<td class="text-center">-</td>"#
            }
        } else {
            ""
        };
        
        article_rows.push_str(&format!(
            r#"
            <tr class="table-row">
                <td class="id-cell">{}</td>
                <td class="star-cell">{}</td>
                <td class="title-cell">
                    <a href="{}" target="_blank" rel="noopener noreferrer">{}</a>
                </td>
                <td class="status-cell">{}</td>
                <td class="site-cell">{}</td>
                <td class="tags-cell">{}</td>
                <td class="date-cell">{}</td>
                {}
                <td class="actions-cell">
                    <button class="action-btn" title="Open">
                        <svg width="14" height="14" viewBox="0 0 16 16" fill="currentColor">
                            <path d="M8 9.5a1.5 1.5 0 100-3 1.5 1.5 0 000 3z"/>
                            <path d="M1.38 8.28a.87.87 0 010-.566 7.003 7.003 0 0113.238 0 .87.87 0 010 .566A7.003 7.003 0 011.379 8.28zM11 8a3 3 0 11-6 0 3 3 0 016 0z"/>
                        </svg>
                    </button>
                    <button class="action-btn" title="Edit">
                        <svg width="14" height="14" viewBox="0 0 16 16" fill="currentColor">
                            <path d="M12.146.146a.5.5 0 01.708 0l3 3a.5.5 0 010 .708l-10 10a.5.5 0 01-.168.11l-5 2a.5.5 0 01-.65-.65l2-5a.5.5 0 01.11-.168l10-10z"/>
                        </svg>
                    </button>
                    <button class="action-btn action-delete" title="Delete">
                        <svg width="14" height="14" viewBox="0 0 16 16" fill="currentColor">
                            <path d="M5.5 5.5A.5.5 0 016 6v6a.5.5 0 01-1 0V6a.5.5 0 01.5-.5zm2.5 0a.5.5 0 01.5.5v6a.5.5 0 01-1 0V6a.5.5 0 01.5-.5zm3 .5a.5.5 0 00-1 0v6a.5.5 0 001 0V6z"/>
                            <path fill-rule="evenodd" d="M14.5 3a1 1 0 01-1 1H13v9a2 2 0 01-2 2H5a2 2 0 01-2-2V4h-.5a1 1 0 01-1-1V2a1 1 0 011-1H6a1 1 0 011-1h2a1 1 0 011 1h3.5a1 1 0 011 1v1z"/>
                        </svg>
                    </button>
                </td>
            </tr>
            "#,
            article.id,
            star_icon,
            url,
            title,
            status_badge,
            site,
            tags_html,
            saved,
            archived_cell
        ));
    }
    
    let archived_header = if all || archived {
        r#"<th class="sortable">ARCHIVED <span class="sort-icon">↕</span></th>"#
    } else {
        ""
    };
    
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Stash - Articles</title>
    <style>
        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }}
        
        body {{
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
            background: #f5f7fa;
            min-height: 100vh;
            color: #1f2937;
        }}
        
        .container {{
            max-width: 1600px;
            margin: 0 auto;
            background: white;
            min-height: 100vh;
        }}
        
        .header {{
            background: white;
            border-bottom: 1px solid #e5e7eb;
            padding: 1.5rem 2rem;
        }}
        
        .header-content {{
            display: flex;
            justify-content: space-between;
            align-items: center;
            max-width: 1600px;
            margin: 0 auto;
        }}
        
        .header h1 {{
            font-size: 1.5rem;
            font-weight: 600;
            color: #111827;
            display: flex;
            align-items: center;
            gap: 0.5rem;
        }}
        
        .header-actions {{
            display: flex;
            gap: 1rem;
            align-items: center;
        }}
        
        .btn {{
            padding: 0.5rem 1rem;
            border-radius: 6px;
            font-size: 0.875rem;
            font-weight: 500;
            border: 1px solid #d1d5db;
            background: white;
            color: #374151;
            cursor: pointer;
            transition: all 0.15s;
        }}
        
        .btn:hover {{
            background: #f9fafb;
            border-color: #9ca3af;
        }}
        
        .btn-primary {{
            background: #3b82f6;
            color: white;
            border-color: #3b82f6;
        }}
        
        .btn-primary:hover {{
            background: #2563eb;
            border-color: #2563eb;
        }}
        
        .table-wrapper {{
            padding: 0;
        }}
        
        .table-header {{
            padding: 1.25rem 2rem;
            border-bottom: 1px solid #e5e7eb;
            display: flex;
            justify-content: space-between;
            align-items: center;
            background: #fafbfc;
        }}
        
        .table-title {{
            font-size: 0.875rem;
            color: #6b7280;
            font-weight: 500;
        }}
        
        .table-controls {{
            display: flex;
            gap: 0.75rem;
            align-items: center;
        }}
        
        .control-btn {{
            padding: 0.375rem 0.75rem;
            border: 1px solid #d1d5db;
            background: white;
            border-radius: 4px;
            font-size: 0.8125rem;
            color: #374151;
            cursor: pointer;
            display: flex;
            align-items: center;
            gap: 0.5rem;
        }}
        
        .control-btn:hover {{
            background: #f9fafb;
        }}
        
        .table-container {{
            overflow-x: auto;
        }}
        
        table {{
            width: 100%;
            border-collapse: separate;
            border-spacing: 0;
        }}
        
        thead {{
            background: #fafbfc;
            border-top: 1px solid #e5e7eb;
            border-bottom: 1px solid #e5e7eb;
        }}
        
        th {{
            text-align: left;
            padding: 0.75rem 1.5rem;
            font-size: 0.75rem;
            font-weight: 600;
            color: #6b7280;
            text-transform: uppercase;
            letter-spacing: 0.5px;
            white-space: nowrap;
        }}
        
        th.sortable {{
            cursor: pointer;
            user-select: none;
        }}
        
        th.sortable:hover {{
            color: #374151;
        }}
        
        .sort-icon {{
            display: inline-block;
            margin-left: 0.25rem;
            color: #9ca3af;
            font-size: 0.75rem;
        }}
        
        td {{
            padding: 1rem 1.5rem;
            border-bottom: 1px solid #f3f4f6;
            font-size: 0.875rem;
            color: #374151;
        }}
        
        .table-row {{
            background: white;
            transition: background 0.15s;
        }}
        
        .table-row:hover {{
            background: #f9fafb;
        }}
        
        .id-cell {{
            font-weight: 600;
            color: #6b7280;
            font-size: 0.8125rem;
        }}
        
        .star-cell {{
            width: 40px;
        }}
        
        .title-cell {{
            font-weight: 500;
            max-width: 500px;
        }}
        
        .title-cell a {{
            color: #111827;
            text-decoration: none;
            transition: color 0.15s;
        }}
        
        .title-cell a:hover {{
            color: #3b82f6;
        }}
        
        .status-cell {{
            width: 100px;
        }}
        
        .badge {{
            display: inline-block;
            padding: 0.25rem 0.625rem;
            border-radius: 9999px;
            font-size: 0.75rem;
            font-weight: 500;
            white-space: nowrap;
        }}
        
        .badge-unread {{
            background: #dbeafe;
            color: #1e40af;
        }}
        
        .badge-read {{
            background: #d1fae5;
            color: #065f46;
        }}
        
        .badge-archived {{
            background: #f3f4f6;
            color: #6b7280;
        }}
        
        .site-cell {{
            color: #6b7280;
            font-size: 0.8125rem;
        }}
        
        .tags-cell {{
            max-width: 250px;
        }}
        
        .tag {{
            display: inline-block;
            padding: 0.25rem 0.625rem;
            border-radius: 4px;
            font-size: 0.75rem;
            font-weight: 500;
            margin-right: 0.375rem;
            margin-bottom: 0.25rem;
            white-space: nowrap;
        }}
        
        .tag-blue {{
            background: #dbeafe;
            color: #1e40af;
        }}
        
        .tag-purple {{
            background: #e9d5ff;
            color: #6b21a8;
        }}
        
        .tag-green {{
            background: #d1fae5;
            color: #065f46;
        }}
        
        .tag-orange {{
            background: #fed7aa;
            color: #92400e;
        }}
        
        .tag-pink {{
            background: #fce7f3;
            color: #9f1239;
        }}
        
        .tag-gray {{
            background: #f3f4f6;
            color: #374151;
        }}
        
        .no-tags {{
            color: #9ca3af;
            font-size: 0.8125rem;
        }}
        
        .date-cell {{
            color: #6b7280;
            font-size: 0.8125rem;
            white-space: nowrap;
        }}
        
        .text-center {{
            text-align: center;
        }}
        
        .icon-archived {{
            font-size: 1rem;
        }}
        
        .actions-cell {{
            width: 120px;
            text-align: right;
        }}
        
        .action-btn {{
            display: inline-flex;
            align-items: center;
            justify-content: center;
            width: 28px;
            height: 28px;
            border: none;
            background: transparent;
            color: #6b7280;
            cursor: pointer;
            border-radius: 4px;
            transition: all 0.15s;
            margin-left: 0.25rem;
        }}
        
        .action-btn:hover {{
            background: #f3f4f6;
            color: #374151;
        }}
        
        .action-btn.action-delete:hover {{
            background: #fee2e2;
            color: #dc2626;
        }}
        
        .footer {{
            padding: 1.5rem 2rem;
            background: #fafbfc;
            border-top: 1px solid #e5e7eb;
            text-align: center;
        }}
        
        .footer-content {{
            display: flex;
            justify-content: space-between;
            align-items: center;
            color: #6b7280;
            font-size: 0.8125rem;
        }}
        
        .pagination {{
            display: flex;
            gap: 0.5rem;
            align-items: center;
        }}
        
        .page-btn {{
            padding: 0.375rem 0.75rem;
            border: 1px solid #d1d5db;
            background: white;
            border-radius: 4px;
            font-size: 0.8125rem;
            color: #374151;
            cursor: pointer;
            min-width: 32px;
            text-align: center;
        }}
        
        .page-btn:hover {{
            background: #f9fafb;
        }}
        
        .page-btn.active {{
            background: #3b82f6;
            color: white;
            border-color: #3b82f6;
        }}
        
        @media (max-width: 768px) {{
            .header {{
                padding: 1rem;
            }}
            
            .header h1 {{
                font-size: 1.25rem;
            }}
            
            .table-header {{
                padding: 1rem;
                flex-direction: column;
                align-items: flex-start;
                gap: 1rem;
            }}
            
            th, td {{
                padding: 0.75rem 1rem;
            }}
            
            .actions-cell {{
                width: 80px;
            }}
        }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <div class="header-content">
                <h1>
                    <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"/>
                        <path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"/>
                    </svg>
                    Stash
                </h1>
                <div class="header-actions">
                    <span style="color: #6b7280; font-size: 0.875rem;">{} articles</span>
                    <button class="btn">Export All Data</button>
                </div>
            </div>
        </div>
        
        <div class="table-wrapper">
            <div class="table-header">
                <div class="table-title">Your saved articles</div>
                <div class="table-controls">
                    <button class="control-btn">
                        <span>Show 8 Row</span>
                        <svg width="12" height="12" viewBox="0 0 12 12" fill="currentColor">
                            <path d="M2.5 4.5L6 8l3.5-3.5"/>
                        </svg>
                    </button>
                    <button class="control-btn">
                        <span>Manage Columns</span>
                    </button>
                    <button class="control-btn">⚙</button>
                    <button class="control-btn">⋮</button>
                </div>
            </div>
            
            <div class="table-container">
                <table>
                    <thead>
                        <tr>
                            <th>ID</th>
                            <th></th>
                            <th class="sortable">TITLE <span class="sort-icon">↕</span></th>
                            <th class="sortable">STATUS <span class="sort-icon">↕</span></th>
                            <th class="sortable">SITE <span class="sort-icon">↕</span></th>
                            <th>TAGS</th>
                            <th class="sortable">SAVED <span class="sort-icon">↕</span></th>
                            {}
                            <th>ACTIONS</th>
                        </tr>
                    </thead>
                    <tbody>
                        {}
                    </tbody>
                </table>
            </div>
        </div>
        
        <div class="footer">
            <div class="footer-content">
                <div>Stash - Press Ctrl+C in terminal to stop the server</div>
                <div class="pagination">
                    <button class="page-btn">1</button>
                    <button class="page-btn">2</button>
                    <button class="page-btn">3</button>
                    <button class="page-btn">4</button>
                    <span>...</span>
                    <button class="page-btn">→</button>
                    <span style="margin-left: 0.5rem;">Go to page</span>
                    <input type="text" class="page-btn" style="width: 50px;" value="1">
                </div>
            </div>
        </div>
    </div>
</body>
</html>"#,
        articles.len(),
        archived_header,
        article_rows
    )
}

fn format_tags_colored(tags: &[String]) -> String {
    let colors = ["blue", "purple", "green", "orange", "pink", "gray"];
    
    tags.iter()
        .enumerate()
        .map(|(i, tag)| {
            let color = colors[i % colors.len()];
            format!(r#"<span class="tag tag-{}">{}</span>"#, color, tag)
        })
        .collect::<Vec<_>>()
        .join("")
}

fn start_server(html: String) -> Result<()> {
    let server = Server::http("127.0.0.1:8080")
        .map_err(|e| anyhow::anyhow!("Failed to start server on localhost:8080: {}", e))?;
    
    println!("🌐 Server running on http://localhost:8080");
    println!("📖 Opening browser...");
    println!("💡 Press Ctrl+C to stop the server\n");
    
    // Open the browser
    if let Err(e) = browser::that("http://localhost:8080") {
        eprintln!("⚠️  Failed to open browser automatically: {}", e);
        eprintln!("   Please open http://localhost:8080 manually");
    }
    
    // Handle requests
    for request in server.incoming_requests() {
        let response = Response::from_string(html.clone())
            .with_header(
                tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..])
                    .unwrap()
            );
        
        if let Err(e) = request.respond(response) {
            eprintln!("Error responding to request: {}", e);
        }
    }
    
    Ok(())
}

