use comfy_table::Color;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Dark,
    Light,
}

impl Theme {
    /// Detect the terminal theme by querying the terminal background color
    pub fn detect() -> Self {
        // Try to detect terminal background with a short timeout
        match termbg::theme(Duration::from_millis(100)) {
            Ok(termbg::Theme::Light) => Theme::Light,
            Ok(termbg::Theme::Dark) => Theme::Dark,
            // Default to dark theme if detection fails
            Err(_) => Theme::Dark,
        }
    }

    /// Get the color for table headers
    pub fn header_color(&self) -> Color {
        match self {
            Theme::Dark => Color::Cyan,
            Theme::Light => Color::Blue,
        }
    }

    /// Get the color for unread articles
    pub fn unread_color(&self) -> Color {
        match self {
            Theme::Dark => Color::White,
            Theme::Light => Color::Black,
        }
    }

    /// Get the color for read/archived articles
    pub fn read_color(&self) -> Color {
        match self {
            Theme::Dark => Color::DarkGrey,
            Theme::Light => Color::Grey,
        }
    }

    /// Get the color for starred articles
    pub fn starred_color(&self) -> Color {
        match self {
            Theme::Dark => Color::Yellow,
            Theme::Light => Color::DarkYellow,
        }
    }

    /// Get the color for archived indicator
    pub fn archived_color(&self) -> Color {
        match self {
            Theme::Dark => Color::DarkRed,
            Theme::Light => Color::Red,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_detection() {
        // Just ensure detection doesn't panic
        let _theme = Theme::detect();
    }

    #[test]
    fn test_color_schemes() {
        let dark = Theme::Dark;
        let light = Theme::Light;

        // Ensure different colors for different themes
        assert_ne!(dark.unread_color(), light.unread_color());
        assert_ne!(dark.read_color(), light.read_color());
    }
}

