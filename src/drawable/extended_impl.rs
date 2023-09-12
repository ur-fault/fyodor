use crossterm::style::ContentStyle;
use unicode_width::UnicodeWidthChar;

use crate::{
    canvas::CanvasLike,
    cell::Cell,
    layout::{Dims, Pos},
};

use super::Drawable;

impl<'a> Drawable for (ContentStyle, &'a str) {
    type X = i32;
    type Y = i32;

    fn draw(self, pos: impl Into<Dims>, frame: &mut impl CanvasLike) {
        let pos = pos.into();

        let mut i = 0;
        let (style, string) = self;
        for chr in string.chars() {
            (style, chr).draw((pos.x + i as i32, pos.y), frame);
            i += chr.width().unwrap_or(0) as i32;
        }
    }
}

impl Drawable for (ContentStyle, char) {
    type X = i32;
    type Y = i32;

    fn draw(self, pos: impl Into<Dims>, frame: &mut impl CanvasLike) {
        let Pos { x, y } = pos.into();
        let (style, chr) = self;

        if x >= frame.size().x || y >= frame.size().y {
            return;
        }

        let width = chr.width().unwrap_or(0) as i32;
        if width == 0 {
            return;
        }

        let cell = Cell::styled(chr, style);

        frame.setd((x, y), cell);

        for i in x + 1..x + width {
            frame.setd((i, y), Cell::PlaceHolder);
        }
    }
}

pub trait Stylable: Drawable + Sized {
    fn styled(self, style: ContentStyle) -> (ContentStyle, Self);
    fn styled_ref(&self, style: ContentStyle) -> (ContentStyle, &Self);
}

impl<D, X, Y> Stylable for D
where
    D: Drawable<X = X, Y = Y>,
    (ContentStyle, D): Drawable,
{
    fn styled(self, style: ContentStyle) -> (ContentStyle, Self) {
        (style, self)
    }

    fn styled_ref(&self, style: ContentStyle) -> (ContentStyle, &Self) {
        (style, self)
    }
}

// pub struct X(pub i32);
// pub struct Y(pub i32);

// impl<D> Drawable for (D, X) {
//     fn draw(self, pos: Self::Pos, frame: &mut impl CanvasLike) {
//         self.0.draw((self.1 .0, pos).into(), frame);
//     }
// }

// impl<D: Drawable<Pos = Dims>> Drawable for (D, Y) {
//     type Pos = i32;

//     fn draw(self, pos: Self::Pos, frame: &mut impl CanvasLike) {
//         self.0.draw((pos, self.1 .0).into(), frame);
//     }
// }
