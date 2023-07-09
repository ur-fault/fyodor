use crossterm::style::ContentStyle;
use unicode_width::UnicodeWidthChar;

use crate::canvas::CanvasLike;

use super::renderer::{Cell, Dims};

pub trait Drawable {
    fn draw<F>(&self, pos: Dims, frame: &mut F)
    where
        F: CanvasLike,
    {
        self.draw_styled(pos, frame, ContentStyle::default())
    }
    fn draw_styled<F>(&self, pos: Dims, frame: &mut F, style: ContentStyle)
    where
        F: CanvasLike;
}

impl Drawable for char {
    fn draw_styled<F>(&self, (x, y): Dims, frame: &mut F, style: ContentStyle)
    where
        F: CanvasLike,
    {
        if x >= frame.size().0 || y >= frame.size().1 {
            return;
        }

        let width = self.width().unwrap_or(1) as i32;
        if width == 0 {
            return;
        }

        let cell = Cell::styled(*self, style);

        frame.set((x, y), cell);

        for i in x + 1..x + width {
            frame.set((i, y), Cell::PlaceHolder);
        }
    }
}

impl Drawable for String {
    fn draw<F>(&self, pos: Dims, frame: &mut F)
    where
        F: CanvasLike,
    {
        self.as_str().draw(pos, frame);
    }

    fn draw_styled<F>(&self, pos: Dims, frame: &mut F, style: ContentStyle)
    where
        F: CanvasLike,
    {
        self.as_str().draw_styled(pos, frame, style);
    }
}

impl<'a> Drawable for &'a str {
    fn draw<F>(&self, pos: Dims, frame: &mut F)
    where
        F: CanvasLike,
    {
        let mut i = 0;
        for chr in self.chars() {
            chr.draw((pos.0 + i as i32, pos.1), frame);
            i += chr.width().unwrap_or(1) as i32;
        }
    }

    fn draw_styled<F>(&self, pos: Dims, frame: &mut F, style: ContentStyle)
    where
        F: CanvasLike,
    {
        let mut i = 0;
        for character in self.chars() {
            character.draw_styled((pos.0 + i as i32, pos.1), frame, style);
            i += character.width().unwrap_or(1) as i32;
        }
    }
}

impl Drawable for Cell {
    fn draw<F>(&self, pos: Dims, frame: &mut F)
    where
        F: CanvasLike,
    {
        frame.set(pos, *self);
    }

    fn draw_styled<F>(&self, pos: Dims, frame: &mut F, style: ContentStyle)
    where
        F: CanvasLike,
    {
        let mut cell = *self;
        if let Cell::Content(content) = &mut cell {
            content.style = style;
        }

        frame.set(pos, cell);
    }
}

impl<D: Drawable> Drawable for (D, ContentStyle) {
    fn draw<F>(&self, pos: Dims, frame: &mut F)
    where
        F: CanvasLike,
    {
        self.0.draw_styled(pos, frame, self.1);
    }

    fn draw_styled<F>(&self, pos: Dims, frame: &mut F, style: ContentStyle)
    where
        F: CanvasLike,
    {
        self.0.draw_styled(pos, frame, style);
    }
}
