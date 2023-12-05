use crossterm::style::ContentStyle;
use unicode_width::UnicodeWidthChar;

use crate::{
    canvas::CanvasLike,
    cell::Cell,
    layout::{Dims, Pos},
};

use super::Drawable;

impl<D> Drawable for &D
where
    D: Drawable,
{
    type X = D::X;
    type Y = D::Y;

    fn draw(&self, pos: impl Into<Pos<Self::X, Self::Y>>, frame: &mut impl CanvasLike) {
        (*self).draw(pos, frame);
    }
}

impl Drawable for (ContentStyle, &&str) {
    type X = i32;
    type Y = i32;

    fn draw(&self, pos: impl Into<Dims>, frame: &mut impl CanvasLike) {
        let pos = pos.into();

        let mut i = 0;
        let (style, string) = self;
        for chr in string.chars() {
            (*style, &chr).draw((pos.x + i as i32, pos.y), frame);
            i += chr.width().unwrap_or(0) as i32;
        }
    }
}

impl Drawable for (ContentStyle, &char) {
    type X = i32;
    type Y = i32;

    fn draw(&self, pos: impl Into<Dims>, frame: &mut impl CanvasLike) {
        let Pos { x, y } = pos.into();
        let (style, chr) = *self;

        if x >= frame.size().x || y >= frame.size().y {
            return;
        }

        let width = chr.width().unwrap_or(0) as i32;
        if width == 0 {
            return;
        }

        let cell = Cell::styled(*chr, style);

        frame.setd((x, y), cell);

        for i in x + 1..x + width {
            frame.setd((i, y), Cell::PlaceHolder);
        }
    }
}

impl Drawable for (ContentStyle, &String) {
    type X = i32;
    type Y = i32;

    fn draw(&self, pos: impl Into<Dims>, frame: &mut impl CanvasLike) {
        (self.0, &self.1.as_str()).draw(pos, frame);
    }
}
