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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_html_to_md_basic_heading() {
        let html = "<h1>Hello World</h1>";
        let result = convert_html_to_md(html);
        assert!(result.is_some());
        assert!(result.unwrap().contains("Hello World"));
    }

    #[test]
    fn test_convert_html_to_md_paragraph() {
        let html = "<p>This is a paragraph.</p>";
        let result = convert_html_to_md(html);
        assert!(result.is_some());
        assert!(result.unwrap().contains("This is a paragraph"));
    }

    #[test]
    fn test_convert_html_to_md_link() {
        let html = r#"<a href="https://example.com">Example</a>"#;
        let result = convert_html_to_md(html);
        assert!(result.is_some());
    }

    #[test]
    fn test_convert_html_to_md_empty_returns_none() {
        assert_eq!(convert_html_to_md(""), None);
    }

    #[test]
    fn test_convert_html_to_md_whitespace_only_returns_none() {
        assert_eq!(convert_html_to_md("   \n  \t  "), None);
    }

    #[test]
    fn test_convert_html_to_md_trims_whitespace() {
        let html = "  <p>Content</p>  ";
        let result = convert_html_to_md(html);
        assert!(result.is_some());
    }

    #[test]
    fn test_convert_html_to_md_complex_structure() {
        let html = r#"
            <article>
                <h1>Title</h1>
                <p>First paragraph</p>
                <ul>
                    <li>Item 1</li>
                    <li>Item 2</li>
                </ul>
            </article>
        "#;
        let result = convert_html_to_md(html);
        assert!(result.is_some());
        let markdown = result.unwrap();
        assert!(markdown.contains("Title"));
        assert!(markdown.contains("First paragraph"));
    }
}