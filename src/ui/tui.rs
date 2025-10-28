use anyhow::Result;
use cursive::align::HAlign;
use cursive::event::Key;
use cursive::theme::{Color, ColorStyle};
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

// browser is the `open` crate aliased in Cargo.toml
extern crate browser;

// Filter state to track if showing all articles or just unread
#[derive(Clone)]
struct FilterState {
    show_all: bool,
}

pub fn launch_tui(conn: Connection) -> Result<()> {
    let mut siv = cursive::default();
    
    // Set up modern theme with better colors
    let mut theme = siv.current_theme().clone();
    theme.palette.set_color("background", Color::Rgb(15, 15, 20));  // Dark blue-black
    theme.palette.set_color("view", Color::Rgb(15, 15, 20));
    theme.palette.set_color("primary", Color::Rgb(220, 220, 230));  // Soft white
    theme.palette.set_color("secondary", Color::Rgb(100, 100, 120));  // Dim gray
    theme.palette.set_color("tertiary", Color::Rgb(60, 60, 75));  // Darker gray
    theme.palette.set_color("highlight", Color::Rgb(70, 130, 180));  // Steel blue
    theme.palette.set_color("highlight_inactive", Color::Rgb(50, 50, 65));
    theme.shadow = false;  // Cleaner look without shadows
    siv.set_theme(theme);
    
    // Initialize filter state (default: show only unread)
    let filter_state = Rc::new(RefCell::new(FilterState { show_all: false }));
    
    // Store connection and filter state in user data
    siv.set_user_data((conn, filter_state.clone()));
    
    // Load articles
    let show_all = filter_state.borrow().show_all;
    let conn_ref = &siv.user_data::<(Connection, Rc<RefCell<FilterState>>)>().unwrap().0;
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
    let mut select = SelectView::<Article>::new()
        .h_align(HAlign::Left)
        .autojump();
    
    // Populate the list
    for article in articles {
        let label = format_article_line(&article);
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
    let filter_state = siv.user_data::<(Connection, Rc<RefCell<FilterState>>)>().unwrap().1.clone();
    let title = if filter_state.borrow().show_all {
        "Stash - All Articles"
    } else {
        "Stash - Unread Articles"
    };
    
    let layout = LinearLayout::vertical()
        .child(build_header())
        .child(Panel::new(select).title(title).with_name("panel"))
        .child(build_footer());
    
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

fn format_article_line(article: &Article) -> StyledString {
    let mut styled = StyledString::new();
    
    // ID column with subtle color
    let id = format!("{:>4}  ", article.id);
    styled.append_styled(id, ColorStyle::new(Color::Rgb(100, 100, 120), Color::Rgb(15, 15, 20)));
    
    // Star icon with golden color if starred, dim gray otherwise
    let star = if article.starred { "★" } else { "☆" };
    let star_color = if article.starred {
        Color::Rgb(255, 200, 60)  // Golden
    } else {
        Color::Rgb(60, 60, 75)  // Dim gray
    };
    styled.append_styled(format!("{}  ", star), ColorStyle::new(star_color, Color::Rgb(15, 15, 20)));
    
    // Read status icon with color
    let status = if article.read { "✓" } else { "●" };
    let status_color = if article.read {
        Color::Rgb(100, 180, 100)  // Green
    } else {
        Color::Rgb(100, 180, 255)  // Cyan blue
    };
    styled.append_styled(format!("{}  ", status), ColorStyle::new(status_color, Color::Rgb(15, 15, 20)));
    
    // Main text color - dimmed for read articles, bright for unread
    let text_color = if article.read {
        Color::Rgb(120, 120, 135)  // Dimmed
    } else {
        Color::Rgb(220, 220, 230)  // Bright
    };
    
    // Title column
    let title = article.title.as_deref().unwrap_or("<no title>");
    let title_str = format!("{:62}", truncate(title, 60));
    styled.append_styled(title_str, ColorStyle::new(text_color, Color::Rgb(15, 15, 20)));
    
    // Site column with slight emphasis
    let site = article.site.as_deref().unwrap_or("-");
    let site_color = if article.read {
        Color::Rgb(100, 100, 120)
    } else {
        Color::Rgb(140, 180, 220)  // Light blue
    };
    let site_str = format!(" {:27}", truncate(site, 25));
    styled.append_styled(site_str, ColorStyle::new(site_color, Color::Rgb(15, 15, 20)));
    
    // Date column
    let date = datetime_humanize(article.saved_at);
    let date_str = format!(" {:12}", date);
    styled.append_styled(date_str, ColorStyle::new(Color::Rgb(140, 140, 150), Color::Rgb(15, 15, 20)));
    
    // Tags in accent color
    if !article.tags.is_empty() {
        let tags = format!("  [{}]", article.tags.join(", "));
        styled.append_styled(tags, ColorStyle::new(Color::Rgb(180, 140, 200), Color::Rgb(15, 15, 20)));
    }
    
    styled
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        format!("{:width$}", s, width = max_len)
    } else {
        format!("{}…", &s[..max_len - 1])
    }
}

fn build_header() -> TextView {
    let mut header = StyledString::new();
    
    // Column headers with subtle color
    let header_color = Color::Rgb(140, 140, 160);
    let bg_color = Color::Rgb(15, 15, 20);
    
    header.append_styled("  ID  ", ColorStyle::new(header_color, bg_color));
    header.append_styled("   ", ColorStyle::new(header_color, bg_color));
    header.append_styled("   ", ColorStyle::new(header_color, bg_color));
    header.append_styled("TITLE", ColorStyle::new(header_color, bg_color));
    header.append_styled(" ".repeat(58), ColorStyle::new(header_color, bg_color));
    header.append_styled("SOURCE", ColorStyle::new(header_color, bg_color));
    header.append_styled(" ".repeat(22), ColorStyle::new(header_color, bg_color));
    header.append_styled("SAVED", ColorStyle::new(header_color, bg_color));
    header.append_styled(" ".repeat(8), ColorStyle::new(header_color, bg_color));
    header.append_styled("TAGS", ColorStyle::new(header_color, bg_color));
    
    TextView::new(header)
}

fn build_footer() -> TextView {
    let mut footer = StyledString::new();
    
    let key_color = Color::Rgb(100, 180, 255);  // Cyan for keys
    let text_color = Color::Rgb(180, 180, 190);  // Light gray for text
    let sep_color = Color::Rgb(60, 60, 75);  // Dim separator
    let bg_color = Color::Rgb(25, 25, 35);  // Slightly lighter background
    
    footer.append_styled("  ", ColorStyle::new(text_color, bg_color));
    
    // Navigation group
    footer.append_styled("o/Enter", ColorStyle::new(key_color, bg_color));
    footer.append_styled(" Open  ", ColorStyle::new(text_color, bg_color));
    footer.append_styled("│ ", ColorStyle::new(sep_color, bg_color));
    
    // Status group
    footer.append_styled("r", ColorStyle::new(key_color, bg_color));
    footer.append_styled(" Read  ", ColorStyle::new(text_color, bg_color));
    footer.append_styled("u", ColorStyle::new(key_color, bg_color));
    footer.append_styled(" Unread  ", ColorStyle::new(text_color, bg_color));
    footer.append_styled("│ ", ColorStyle::new(sep_color, bg_color));
    
    // Actions group
    footer.append_styled("s", ColorStyle::new(key_color, bg_color));
    footer.append_styled(" Star  ", ColorStyle::new(text_color, bg_color));
    footer.append_styled("A", ColorStyle::new(key_color, bg_color));
    footer.append_styled(" Archive  ", ColorStyle::new(text_color, bg_color));
    footer.append_styled("│ ", ColorStyle::new(sep_color, bg_color));
    
    // View controls
    footer.append_styled("a", ColorStyle::new(key_color, bg_color));
    footer.append_styled(" Filter  ", ColorStyle::new(text_color, bg_color));
    footer.append_styled("R", ColorStyle::new(key_color, bg_color));
    footer.append_styled(" Refresh  ", ColorStyle::new(text_color, bg_color));
    footer.append_styled("│ ", ColorStyle::new(sep_color, bg_color));
    
    // Exit
    footer.append_styled("q/Esc", ColorStyle::new(key_color, bg_color));
    footer.append_styled(" Quit  ", ColorStyle::new(text_color, bg_color));
    
    TextView::new(footer)
}

fn on_open(s: &mut Cursive) {
    if let Some(article) = get_selected_article(s) {
        open_article(s, article.id);
    }
}

fn open_article(s: &mut Cursive, id: i64) {
    let (conn, _) = s.user_data::<(Connection, Rc<RefCell<FilterState>>)>().unwrap();
    
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
        let (conn, _) = s.user_data::<(Connection, Rc<RefCell<FilterState>>)>().unwrap();
        if let Err(e) = queries::set_read_by_ids(conn, &[article.id], true) {
            show_error(s, &format!("Failed to mark as read: {}", e));
        } else {
            refresh_list(s);
        }
    }
}

fn on_mark_unread(s: &mut Cursive) {
    if let Some(article) = get_selected_article(s) {
        let (conn, _) = s.user_data::<(Connection, Rc<RefCell<FilterState>>)>().unwrap();
        if let Err(e) = queries::set_read_by_ids(conn, &[article.id], false) {
            show_error(s, &format!("Failed to mark as unread: {}", e));
        } else {
            refresh_list(s);
        }
    }
}

fn toggle_star(s: &mut Cursive) {
    if let Some(article) = get_selected_article(s) {
        let (conn, _) = s.user_data::<(Connection, Rc<RefCell<FilterState>>)>().unwrap();
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
        let (conn, _) = s.user_data::<(Connection, Rc<RefCell<FilterState>>)>().unwrap();
        
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
    let (_, filter_state) = s.user_data::<(Connection, Rc<RefCell<FilterState>>)>().unwrap();
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
    
    // Reload articles (borrow conn and filter state separately)
    let (articles, show_all) = {
        let (conn, filter_state) = s.user_data::<(Connection, Rc<RefCell<FilterState>>)>().unwrap();
        let show_all = filter_state.borrow().show_all;
        (queries::list_articles(conn, 100, show_all), show_all)
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
                    let label = format_article_line(article);
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

