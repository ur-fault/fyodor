use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

pub struct Multiline {
    pub lines: Vec<String>,
    pub width: u32,
}

impl Multiline {
    pub fn new(s: impl Into<String>) -> Self {
        let s: String = s.into();
        let lines = s
            .split('\n')
            .map(|s| {
                s.chars()
                    .filter(|chr| chr.width().unwrap_or(0) > 0)
                    .collect::<String>()
            })
            .collect::<Vec<_>>();

        let width = lines.iter().map(|s| s.width() as u32).max().unwrap_or(0);
        Self { lines, width }
    }
}
