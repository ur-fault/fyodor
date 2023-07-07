use crossterm::style::ContentStyle;
use unicode_width::UnicodeWidthChar;

use crate::{
    drawable::Drawable,
    renderer::{Cell, Dims},
};

pub struct Canvas {
    pub buffer: Vec<Vec<Cell>>,
    pub size: Dims,
}

impl Canvas {
    pub fn new(size: Dims) -> Self {
        let mut buffer = Vec::new();
        for _ in 0..size.1 {
            buffer.push(vec![Cell::new(' '); size.0 as usize]);
        }
        Canvas { buffer, size }
    }
    pub fn set(&mut self, pos: Dims, cell: Cell) {
        // self.buffer[pos.1 as usize][pos.0 as usize] = cell;
        if let Some(c) = self
            .buffer
            .get_mut(pos.1 as usize)
            .and_then(|r| r.get_mut(pos.0 as usize))
        {
            *c = cell;
        }
    }

    pub fn draw(&mut self, pos: Dims, content: impl Drawable) {
        content.draw(pos, self);
    }

    pub fn resize(&mut self, size: Dims) {
        if self.size == size {
            return;
        }

        self.size = size;
        self.buffer.resize(size.1 as usize, Vec::new());
        for row in self.buffer.iter_mut() {
            row.resize(size.0 as usize, Cell::new(' '));
        }
    }

    pub fn clear(&mut self) {
        for row in self.buffer.iter_mut() {
            for cell in row.iter_mut() {
                *cell = Cell::new(' ');
            }
        }
    }
}

impl std::ops::Index<Dims> for Canvas {
    type Output = Cell;

    fn index(&self, index: Dims) -> &Self::Output {
        &self.buffer[index.1 as usize][index.0 as usize]
    }
}

impl std::ops::Index<i32> for Canvas {
    type Output = [Cell];

    fn index(&self, index: i32) -> &Self::Output {
        &self.buffer[index as usize]
    }
}
