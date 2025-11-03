use anyhow::Result;
use cursive::align::HAlign;
use cursive::event::Key;
use cursive::traits::*;
use cursive::utils::markup::StyledString;
use cursive::views::{Dialog, LinearLayout, NamedView, Panel, ScrollView, SelectView, TextView};
use cursive::Cursive;
use rusqlite::Connection;
use std::cell::RefCell;
use std::rc::Rc;

use crate::db::models::Article;
use crate::db::queries;
use crate::ui::formatters::datetime_humanize;
use crate::ui::theme::Theme;

// browser is the `open` crate aliased in Cargo.toml
extern crate browser;

// Filter state to track if showing all articles or just unread
#[derive(Clone)]
struct FilterState {
    show_all: bool,
}

pub fn launch_tui(conn: Connection) -> Result<()> {
    let mut siv = cursive::default();
    
    // Detect terminal theme
    let detected_theme = Theme::detect();
    
    // Set up theme with colors based on detected theme
    let mut theme = siv.current_theme().clone();
    theme.palette.set_color("background", detected_theme.tui_background());
    theme.palette.set_color("view", detected_theme.tui_background());
    theme.palette.set_color("primary", detected_theme.tui_primary());
    theme.palette.set_color("secondary", detected_theme.tui_secondary());
    theme.palette.set_color("tertiary", detected_theme.tui_tertiary());
    theme.palette.set_color("highlight", detected_theme.tui_highlight());
    theme.palette.set_color("highlight_text", detected_theme.tui_highlight_text());
    theme.palette.set_color("highlight_inactive", detected_theme.tui_highlight_inactive());
    theme.shadow = false;  // Cleaner look without shadows
    siv.set_theme(theme);
    
    // Initialize filter state (default: show only unread)
    let filter_state = Rc::new(RefCell::new(FilterState { show_all: false }));
    
    // Store connection, filter state, and detected theme in user data
    siv.set_user_data((conn, filter_state.clone(), detected_theme));
    
    // Load articles
    let show_all = filter_state.borrow().show_all;
    let conn_ref = &siv.user_data::<(Connection, Rc<RefCell<FilterState>>, Theme)>().unwrap().0;
    let articles = queries::list_articles(conn_ref, 100, show_all)?;
    
    if articles.is_empty() {
        siv.add_layer(
            Dialog::text("No articles found.\n\nAdd articles with: stash add <url>")
                .title("Stash TUI")
                .button("Quit", |s| s.quit()),
        );
    } else {
        build_article_list(&mut siv, articles);
    }
    
    siv.run();
    
    Ok(())
}

fn build_article_list(siv: &mut Cursive, articles: Vec<Article>) {
    // Get the theme
    let theme = siv.user_data::<(Connection, Rc<RefCell<FilterState>>, Theme)>().unwrap().2;
    
    let mut select = SelectView::<Article>::new()
        .h_align(HAlign::Left)
        .autojump();
    
    // Populate the list
    for article in articles {
        let label = format_article_line(&article, theme);
        select.add_item(label, article);
    }
    
    // Add key bindings
    select.set_on_submit(move |s, article: &Article| {
        open_article(s, article.id);
    });
    
    let select = select
        .with_name("select")
        .scrollable()
        .full_screen();
    
    // Get filter state to show in title
    let filter_state = siv.user_data::<(Connection, Rc<RefCell<FilterState>>, Theme)>().unwrap().1.clone();
    let title = if filter_state.borrow().show_all {
        "Stash - All Articles"
    } else {
        "Stash - Unread Articles"
    };
    
    let layout = LinearLayout::vertical()
        .child(build_header(theme))
        .child(Panel::new(select).title(title).with_name("panel"))
        .child(build_footer(theme));
    
    // Add global key bindings
    siv.add_global_callback('q', |s| s.quit());
    siv.add_global_callback(Key::Esc, |s| s.quit());
    siv.add_global_callback('o', on_open);
    siv.add_global_callback(Key::Enter, on_open);
    siv.add_global_callback('r', on_mark_read);
    siv.add_global_callback('u', on_mark_unread);
    siv.add_global_callback('s', toggle_star);
    siv.add_global_callback('a', toggle_filter);
    siv.add_global_callback('A', toggle_archive);
    siv.add_global_callback('j', move_down);
    siv.add_global_callback('k', move_up);
    siv.add_global_callback('R', refresh_list);
    
    siv.add_fullscreen_layer(layout);
}

fn format_article_line(article: &Article, _theme: Theme) -> StyledString {
    let mut line = String::new();
    
    // ID column
    line.push_str(&format!("{:>4}  ", article.id));
    
    // Star icon
    let star = if article.starred { "★" } else { "☆" };
    line.push_str(&format!("{}  ", star));
    
    // Read status icon
    let status = if article.read { "✓" } else { "●" };
    line.push_str(&format!("{}  ", status));
    
    // Note indicator
    let note_icon = if article.note.is_some() { "📝" } else { " " };
    line.push_str(&format!("{}  ", note_icon));
    
    // Title column
    let title = article.title.as_deref().unwrap_or("<no title>");
    line.push_str(&format!("{:58}", truncate(title, 56)));
    
    // Site column
    let site = article.site.as_deref().unwrap_or("-");
    line.push_str(&format!(" {:27}", truncate(site, 25)));
    
    // Date column
    let date = datetime_humanize(article.saved_at);
    line.push_str(&format!(" {:12}", date));
    
    // Tags
    if !article.tags.is_empty() {
        let tags = format!("  [{}]", article.tags.join(", "));
        line.push_str(&tags);
    }
    
    // Note preview (if exists and space permits)
    if let Some(note) = &article.note {
        let note_preview = note.replace('\n', " ");
        let preview = if note_preview.len() > 40 {
            format!("  💬 {}...", &note_preview[..37])
        } else {
            format!("  💬 {}", note_preview)
        };
        line.push_str(&preview);
    }
    
    StyledString::plain(line)
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        format!("{:width$}", s, width = max_len)
    } else {
        format!("{}…", &s[..max_len - 1])
    }
}

fn build_header(_theme: Theme) -> TextView {
    let header = format!(
        "  ID       TITLE{}SOURCE{}SAVED{}TAGS/NOTES",
        " ".repeat(54),
        " ".repeat(22),
        " ".repeat(8)
    );
    
    TextView::new(header)
}

fn build_footer(_theme: Theme) -> TextView {
    let footer = "  o/Enter Open  │ r Read  u Unread  │ s Star  A Archive  │ a Filter  R Refresh  │ q/Esc Quit  ";
    
    TextView::new(footer)
}

fn on_open(s: &mut Cursive) {
    if let Some(article) = get_selected_article(s) {
        open_article(s, article.id);
    }
}

fn open_article(s: &mut Cursive, id: i64) {
    let (conn, _, _) = s.user_data::<(Connection, Rc<RefCell<FilterState>>, Theme)>().unwrap();
    
    // Get the article
    if let Ok(Some(article)) = queries::get_article_by_id(conn, id) {
        // Mark as read
        let _ = queries::mark_read_by_ids(conn, &[id]);
        
        // Open in browser
        if let Err(e) = browser::that(&article.url) {
            show_error(s, &format!("Failed to open browser: {}", e));
        } else {
            // Refresh the list to show updated read status
            refresh_list(s);
        }
    }
}

fn on_mark_read(s: &mut Cursive) {
    if let Some(article) = get_selected_article(s) {
        let (conn, _, _) = s.user_data::<(Connection, Rc<RefCell<FilterState>>, Theme)>().unwrap();
        if let Err(e) = queries::set_read_by_ids(conn, &[article.id], true) {
            show_error(s, &format!("Failed to mark as read: {}", e));
        } else {
            refresh_list(s);
        }
    }
}

fn on_mark_unread(s: &mut Cursive) {
    if let Some(article) = get_selected_article(s) {
        let (conn, _, _) = s.user_data::<(Connection, Rc<RefCell<FilterState>>, Theme)>().unwrap();
        if let Err(e) = queries::set_read_by_ids(conn, &[article.id], false) {
            show_error(s, &format!("Failed to mark as unread: {}", e));
        } else {
            refresh_list(s);
        }
    }
}

fn toggle_star(s: &mut Cursive) {
    if let Some(article) = get_selected_article(s) {
        let (conn, _, _) = s.user_data::<(Connection, Rc<RefCell<FilterState>>, Theme)>().unwrap();
        let new_starred = !article.starred;
        if let Err(e) = queries::set_starred_by_ids(conn, &[article.id], new_starred) {
            show_error(s, &format!("Failed to toggle star: {}", e));
        } else {
            refresh_list(s);
        }
    }
}

fn toggle_archive(s: &mut Cursive) {
    if let Some(article) = get_selected_article(s) {
        let (conn, _, _) = s.user_data::<(Connection, Rc<RefCell<FilterState>>, Theme)>().unwrap();
        
        if article.archived {
            // Unarchive and mark as unread
            if let Err(e) = queries::unarchive_by_ids(conn, &[article.id]) {
                show_error(s, &format!("Failed to unarchive: {}", e));
            } else {
                refresh_list(s);
            }
        } else {
            // Archive
            if let Err(e) = queries::archive_by_ids(conn, &[article.id]) {
                show_error(s, &format!("Failed to archive: {}", e));
            } else {
                refresh_list(s);
            }
        }
    }
}

fn toggle_filter(s: &mut Cursive) {
    let (_, filter_state, _) = s.user_data::<(Connection, Rc<RefCell<FilterState>>, Theme)>().unwrap();
    // Toggle the filter state
    let new_show_all = {
        let current = filter_state.borrow().show_all;
        !current
    };
    filter_state.borrow_mut().show_all = new_show_all;
    refresh_list(s);
}

fn get_selected_article(s: &mut Cursive) -> Option<Article> {
    s.call_on_name("select", |view: &mut SelectView<Article>| {
        view.selection().map(|rc| rc.as_ref().clone())
    })
    .flatten()
}

fn move_down(s: &mut Cursive) {
    s.call_on_name("select", |view: &mut SelectView<Article>| {
        let selected = view.selected_id().unwrap_or(0);
        if selected < view.len().saturating_sub(1) {
            view.set_selection(selected + 1);
        }
    });
}

fn move_up(s: &mut Cursive) {
    s.call_on_name("select", |view: &mut SelectView<Article>| {
        let selected = view.selected_id().unwrap_or(0);
        if selected > 0 {
            view.set_selection(selected - 1);
        }
    });
}

fn refresh_list(s: &mut Cursive) {
    // Get the currently selected ID (if any)
    let selected_id = s
        .call_on_name("select", |view: &mut SelectView<Article>| {
            view.selection().map(|article| article.id)
        })
        .flatten();
    
    // Reload articles (borrow conn, filter state, and theme separately)
    let (articles, show_all, theme) = {
        let (conn, filter_state, theme) = s.user_data::<(Connection, Rc<RefCell<FilterState>>, Theme)>().unwrap();
        let show_all = filter_state.borrow().show_all;
        (queries::list_articles(conn, 100, show_all), show_all, *theme)
    };
    
    // Update panel title
    let title = if show_all {
        "Stash - All Articles"
    } else {
        "Stash - Unread Articles"
    };
    
    s.call_on_name("panel", |view: &mut Panel<ScrollView<NamedView<SelectView<Article>>>>| {
        view.set_title(title);
    });
    
    match articles {
        Ok(articles) => {
            s.call_on_name("select", |view: &mut SelectView<Article>| {
                view.clear();
                
                let mut new_selection_idx = None;
                for (idx, article) in articles.iter().enumerate() {
                    let label = format_article_line(article, theme);
                    view.add_item(label, article.clone());
                    
                    // Track if this is the previously selected article
                    if Some(article.id) == selected_id {
                        new_selection_idx = Some(idx);
                    }
                }
                
                // Restore selection if possible
                if let Some(idx) = new_selection_idx {
                    view.set_selection(idx);
                }
            });
        }
        Err(e) => {
            show_error(s, &format!("Failed to refresh list: {}", e));
        }
    }
}

fn show_error(s: &mut Cursive, message: &str) {
    s.add_layer(
        Dialog::text(message)
            .title("Error")
            .button("Ok", |s| {
                s.pop_layer();
            }),
    );
}

