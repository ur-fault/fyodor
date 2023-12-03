use crossterm::style::ContentStyle;

use crate::{
    canvas::{CanvasLike, CanvasLikeExt},
    layout::{Dims, Pos},
};

use super::{extended_impl::Stylable, Drawable};

#[derive(Clone, Copy)]
pub struct Dbox {
    pub size: Dims,
}

impl Dbox {
    pub fn new(size: impl Into<Dims>) -> Self {
        Self { size: size.into() }
    }
}

impl Drawable for Dbox {
    type X = i32;
    type Y = i32;

    fn draw(self, pos: impl Into<Dims>, frame: &mut impl CanvasLike) {
        let pos = pos.into();
        self.styled(ContentStyle::default()).draw(pos, frame);
    }
}

impl Drawable for (ContentStyle, Dbox) {
    type X = i32;
    type Y = i32;

    fn draw(self, pos: impl Into<Dims>, frame: &mut impl CanvasLike) {
        let (style, dbox) = self;
        let Dbox {
            size: Pos { x: w, y: h },
        } = dbox;
        let pos = pos.into();
        let Pos { x, y } = pos.clone();

        frame.draw(
            pos,
            format!("╭{}╮", "─".repeat(w as usize - 2)).styled(style),
        );

        for y in y + 1..y + h - 1 {
            frame.draw((x, y), '│'.styled(style));
            frame.draw((x + w - 1, y), '│'.styled(style));
        }

        frame.draw(
            (x, y + h - 1),
            format!("╰{}╯", "─".repeat(w as usize - 2)).styled(style),
        );
    }
}
