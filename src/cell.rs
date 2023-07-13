use crossterm::style::ContentStyle;
use unicode_width::UnicodeWidthChar;

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct CellContent {
    pub character: char,
    pub width: u8,
    pub style: ContentStyle,
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub enum Cell {
    #[default]
    PlaceHolder,
    Content(CellContent),
}

impl Cell {
    pub fn styled(c: char, s: ContentStyle) -> Self {
        Cell::Content(CellContent {
            character: c,
            width: c.width().unwrap_or(1) as u8,
            style: s,
        })
    }

    pub fn new(c: char) -> Self {
        Cell::styled(c, ContentStyle::default())
    }
}
