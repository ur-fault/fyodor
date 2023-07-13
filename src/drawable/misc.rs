use crossterm::style::ContentStyle;
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

use crate::{canvas::CanvasLike, cell::Cell, renderer::Dims};

use super::Drawable;

impl<'a> Drawable for (&'a str, ContentStyle) {
    fn draw(&self, pos: Dims, frame: &mut impl CanvasLike) {
        let mut i = 0;
        for chr in self.0.chars() {
            (chr, self.1).draw((pos.0 + i as i32, pos.1), frame);
            i += chr.width().unwrap_or(1) as i32;
        }
    }
}

impl Drawable for (char, ContentStyle) {
    fn draw(&self, (x, y): Dims, frame: &mut impl CanvasLike) {
        let style = self.1;

        if x >= frame.size().0 || y >= frame.size().1 {
            return;
        }

        let width = self.0.width().unwrap_or(1) as i32;
        if width == 0 {
            return;
        }

        let cell = Cell::styled(self.0, style);

        frame.set((x, y), cell);

        for i in x + 1..x + width {
            frame.set((i, y), Cell::PlaceHolder);
        }
    }
}

#[derive(Clone, Copy)]
pub struct CenteredString<'a>(&'a str);

impl<'a> CenteredString<'a> {
    pub fn new(s: &'a str) -> Self {
        Self(s)
    }
}

impl<'a> Drawable for CenteredString<'a> {
    fn draw(&self, (_, y): Dims, frame: &mut impl CanvasLike) {
        (*self, ContentStyle::default()).draw((0, y), frame);
    }
}

impl<'a> Drawable for (CenteredString<'a>, ContentStyle) {
    fn draw(&self, (_, y): Dims, frame: &mut impl CanvasLike) {
        let x = (frame.size().0 - self.0 .0.width() as i32) / 2;
        (self.0 .0, self.1).draw((x, y), frame);
    }
}

pub trait CenteredStringExt<'a> {
    fn center(self) -> CenteredString<'a>;
}

impl<'a> CenteredStringExt<'a> for &'a str {
    fn center(self) -> CenteredString<'a> {
        CenteredString::new(self)
    }
}

#[derive(Clone, Copy)]
pub struct RightAlignedString<'a>(&'a str);

impl<'a> RightAlignedString<'a> {
    pub fn new(s: &'a str) -> Self {
        Self(s)
    }
}

impl<'a> Drawable for RightAlignedString<'a> {
    fn draw(&self, (_, y): Dims, frame: &mut impl CanvasLike) {
        (*self, ContentStyle::default()).draw((0, y), frame);
    }
}

impl<'a> Drawable for (RightAlignedString<'a>, ContentStyle) {
    fn draw(&self, (_, y): Dims, frame: &mut impl CanvasLike) {
        let x = frame.size().0 - self.0 .0.width() as i32;
        (self.0 .0, self.1).draw((x, y), frame);
    }
}

pub trait RightAlignedStringExt<'a> {
    fn right(self) -> RightAlignedString<'a>;
}

impl<'a> RightAlignedStringExt<'a> for &'a str {
    fn right(self) -> RightAlignedString<'a> {
        RightAlignedString::new(self)
    }
}
