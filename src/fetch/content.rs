use html2md::parse_html;

pub fn convert_html_to_md(html: &str) -> Option<String> {
    if html.trim().is_empty() {
        return None;
    }

    let markdown = parse_html(&html);
    
    if markdown.trim().is_empty() {
        None
    } else {
        Some(markdown)
    }
}