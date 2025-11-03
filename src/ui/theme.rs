use comfy_table::Color;
use std::time::Duration;

// Import cursive Color for TUI
use cursive::theme::Color as CursiveColor;

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

    // TUI-specific colors (using cursive's Color type)
    
    /// Get the background color for TUI
    pub fn tui_background(&self) -> CursiveColor {
        match self {
            Theme::Dark => CursiveColor::Rgb(15, 15, 20),      // Dark blue-black
            Theme::Light => CursiveColor::TerminalDefault,     // Use terminal default
        }
    }

    /// Get the primary text color for TUI
    pub fn tui_primary(&self) -> CursiveColor {
        match self {
            Theme::Dark => CursiveColor::Rgb(220, 220, 230),   // Soft white
            Theme::Light => CursiveColor::TerminalDefault,     // Use terminal default
        }
    }

    /// Get the secondary text color for TUI
    pub fn tui_secondary(&self) -> CursiveColor {
        match self {
            Theme::Dark => CursiveColor::Rgb(100, 100, 120),   // Dim gray
            Theme::Light => CursiveColor::TerminalDefault,     // Use terminal default
        }
    }

    /// Get the tertiary text color for TUI
    pub fn tui_tertiary(&self) -> CursiveColor {
        match self {
            Theme::Dark => CursiveColor::Rgb(60, 60, 75),      // Darker gray
            Theme::Light => CursiveColor::TerminalDefault,     // Use terminal default
        }
    }

    /// Get the highlight color for TUI (selected row)
    pub fn tui_highlight(&self) -> CursiveColor {
        match self {
            Theme::Dark => CursiveColor::Rgb(70, 130, 180),    // Steel blue
            Theme::Light => CursiveColor::Rgb(200, 220, 240),  // Light blue highlight
        }
    }

    /// Get the highlight inactive color for TUI
    pub fn tui_highlight_inactive(&self) -> CursiveColor {
        match self {
            Theme::Dark => CursiveColor::Rgb(50, 50, 65),
            Theme::Light => CursiveColor::Rgb(220, 220, 220),  // Light gray
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

