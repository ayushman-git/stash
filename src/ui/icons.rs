pub enum Icons {
    Star,
    Tick,
    CircleEmpty,
    Deleted
}

impl Icons {
    pub const fn glyph(&self) -> &'static str {
        match self {
            Self::Star => "\u{f005}",
            Self::Tick => "\u{eab2}",
            Self::CircleEmpty => "\u{f4c3}",
            Self::Deleted => "\u{f00d}",
        }
    }
}
