use crossterm::style::ContentStyle;
use unicode_width::UnicodeWidthChar;

use crate::canvas::CanvasLike;

use super::renderer::{Cell, Dims};

pub trait Drawable {
    fn draw(&self, pos: Dims, frame: &mut impl CanvasLike);
}

impl Drawable for char {
    fn draw(&self, pos: Dims, frame: &mut impl CanvasLike) {
        (*self, ContentStyle::default()).draw(pos, frame);
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

impl Drawable for String {
    fn draw(&self, pos: Dims, frame: &mut impl CanvasLike) {
        self.as_str().draw(pos, frame);
    }
}

impl Drawable for (String, ContentStyle) {
    fn draw(&self, pos: Dims, frame: &mut impl CanvasLike) {
        (self.0.as_str(), self.1).draw((pos.0, pos.1), frame);
    }
}

impl<'a> Drawable for &'a str {
    fn draw(&self, pos: Dims, frame: &mut impl CanvasLike) {
        let mut i = 0;
        for chr in self.chars() {
            chr.draw((pos.0 + i as i32, pos.1), frame);
            i += chr.width().unwrap_or(1) as i32;
        }
    }
}

impl<'a> Drawable for (&'a str, ContentStyle) {
    fn draw(&self, pos: Dims, frame: &mut impl CanvasLike) {
        let mut i = 0;
        for chr in self.0.chars() {
            (chr, self.1).draw((pos.0 + i as i32, pos.1), frame);
            i += chr.width().unwrap_or(1) as i32;
        }
    }
}

impl Drawable for Cell {
    fn draw(&self, pos: Dims, frame: &mut impl CanvasLike) {
        frame.set(pos, *self);
    }
}
