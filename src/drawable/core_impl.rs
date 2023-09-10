use crossterm::style::ContentStyle;
use unicode_width::UnicodeWidthChar;

use crate::{canvas::CanvasLike, cell::Cell, renderer::Dims};

use super::Drawable;

impl Drawable for char {
    type Pos = Dims;

    fn draw(&self, pos: Dims, frame: &mut impl CanvasLike) {
        (*self, ContentStyle::default()).draw(pos, frame);
    }
}

impl Drawable for String {
    type Pos = Dims;

    fn draw(&self, pos: Dims, frame: &mut impl CanvasLike) {
        self.as_str().draw(pos, frame);
    }
}

impl Drawable for (String, ContentStyle) {
    type Pos = Dims;

    fn draw(&self, pos: Dims, frame: &mut impl CanvasLike) {
        (self.0.as_str(), self.1).draw((pos.0, pos.1), frame);
    }
}

impl<'a> Drawable for &'a str {
    type Pos = Dims;

    fn draw(&self, pos: Dims, frame: &mut impl CanvasLike) {
        let mut i = 0;
        for chr in self.chars() {
            chr.draw((pos.0 + i as i32, pos.1), frame);
            i += chr.width().unwrap_or(1) as i32;
        }
    }
}

impl Drawable for Cell {
    type Pos = Dims;

    fn draw(&self, pos: Dims, frame: &mut impl CanvasLike) {
        frame.set(pos, *self);
    }
}
