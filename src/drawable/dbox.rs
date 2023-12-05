use crossterm::style::ContentStyle;

use crate::{
    canvas::CanvasLike,
    layout::{Dims, Pos},
};

use super::{styled::Stylable, Drawable};

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

    fn draw(&self, pos: impl Into<Dims>, frame: &mut impl CanvasLike) {
        let pos = pos.into();
        self.styled(ContentStyle::default()).draw(pos, frame);
    }
}

impl Drawable for (ContentStyle, &Dbox) {
    type X = i32;
    type Y = i32;

    fn draw(&self, pos: impl Into<Dims>, frame: &mut impl CanvasLike) {
        let (style, dbox) = self;
        let Dbox {
            size: Pos { x: w, y: h },
        } = **dbox;
        let pos @ Pos { x, y } = pos.into().clone();

        format!("╭{}╮", "─".repeat(w as usize - 2))
            .styled(*style)
            .draw(pos, frame);

        for y in y + 1..y + h - 1 {
            // frame.draw_on((x, y), '│'.styled(*style));
            '│'.styled(*style).draw((x, y), frame);
            // frame.draw_on((x + w - 1, y), '│'.styled(*style));
            '│'.styled(*style).draw((x + w - 1, y), frame);
        }

        // frame.draw(
        //     (x, y + h - 1),
        //     format!("╰{}╯", "─".repeat(w as usize - 2)).styled(*style),
        // );
        format!("╰{}╯", "─".repeat(w as usize - 2))
            .styled(*style)
            .draw((x, y + h - 1), frame);
    }
}
