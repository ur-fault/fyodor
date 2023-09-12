use crossterm::style::ContentStyle;
use unicode_width::UnicodeWidthChar;

use crate::{
    canvas::CanvasLike,
    cell::Cell,
    layout::{Dims, Pos},
};

use super::{extended_impl::Stylable, Drawable};

impl Drawable for char {
    type X = i32;
    type Y = i32;

    fn draw(self, pos: impl Into<Dims>, frame: &mut impl CanvasLike) {
        self.styled(ContentStyle::default()).draw(pos, frame);
    }
}

// impl Drawable for &str {
//     type Pos = Dims;

//     fn draw(&self, pos: Dims, frame: &mut impl CanvasLike) {
//         self.draw(pos, frame);
//     }
// }

impl Drawable for &str {
    type X = i32;
    type Y = i32;

    fn draw(self, pos: impl Into<Dims>, frame: &mut impl CanvasLike) {
        let Pos { x, y } = pos.into();

        let mut i = 0;
        for chr in self.chars() {
            chr.draw((x + i as i32, y), frame);
            i += chr.width().unwrap_or(1) as i32;
        }
    }
}

impl Drawable for String {
    type X = i32;
    type Y = i32;

    fn draw(self, pos: impl Into<Dims>, frame: &mut impl CanvasLike) {
        self.as_str().draw(pos, frame);
    }
}

impl Drawable for Cell {
    type X = i32;
    type Y = i32;

    fn draw(self, pos: impl Into<Dims>, frame: &mut impl CanvasLike) {
        frame.setd(pos, self);
    }
}
