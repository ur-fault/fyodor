use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use crate::{cell::Cell, drawable::Drawable, renderer::Dims};

pub struct Buffer {
    buffer: Vec<Vec<Cell>>,
    size: Dims,
}

impl Buffer {
    pub fn new(size: Dims) -> Self {
        let mut buffer = Vec::new();
        for _ in 0..size.1 {
            buffer.push(vec![Cell::new(' '); size.0 as usize]);
        }
        Buffer { buffer, size }
    }

    pub fn buf_ref(&self) -> &[Vec<Cell>] {
        &self.buffer
    }

    pub fn buf_mut(&mut self) -> &mut [Vec<Cell>] {
        &mut self.buffer
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

    pub fn size(&self) -> Dims {
        self.size
    }
}

pub trait CanvasLike {
    fn set(&mut self, pos: Dims, cell: Cell);
    fn pos(&self) -> Dims;
    fn size(&self) -> Dims;
}

#[derive(Clone)]
pub struct Canvas {
    pub buffer: Rc<RefCell<Buffer>>,
}

impl Canvas {
    pub fn new(buf: Buffer) -> Self {
        Self {
            buffer: Rc::new(RefCell::new(buf)),
        }
    }

    pub fn from_dims(size: Dims) -> Self {
        Self::new(Buffer::new(size))
    }

    pub fn set(&mut self, pos: Dims, cell: Cell) {
        if let Some(c) = self
            .buffer
            .borrow_mut() // from RefCell
            .buf_mut() // from Buffer
            .get_mut(pos.1 as usize) // from &mut [Vec<Cell>]
            .and_then(|r| r.get_mut(pos.0 as usize))
        {
            *c = cell;
        }
    }

    pub fn size(&self) -> Dims {
        self.buffer.borrow().size()
    }

    pub fn get(&self, pos: Dims) -> Option<Cell> {
        self.buffer
            .borrow() // from RefCell
            .buf_ref() // from Buffer
            .get(pos.1 as usize) // from &[Vec<Cell>]
            .and_then(|r| r.get(pos.0 as usize))
            .copied()
    }

    pub fn get_buf(&self) -> Ref<Buffer> {
        self.buffer.borrow()
    }

    pub fn draw(&mut self, pos: Dims, content: impl Drawable) {
        content.draw(pos, self)
    }

    pub fn resize(&mut self, size: Dims) {
        self.buffer.borrow_mut().resize(size);
    }

    pub fn clear(&mut self) {
        for row in self.buffer.borrow_mut().buf_mut().iter_mut() {
            for cell in row.iter_mut() {
                *cell = Cell::new(' ');
            }
        }
    }
}

impl CanvasLike for Canvas {
    fn set(&mut self, pos: Dims, cell: Cell) {
        Canvas::set(self, pos, cell); // Otherwise it would be recursive
    }
    fn pos(&self) -> Dims {
        (0, 0)
    }

    fn size(&self) -> Dims {
        self.size()
    }
}

impl<T> CanvasLike for &mut T
where
    T: CanvasLike,
{
    fn set(&mut self, pos: Dims, cell: Cell) {
        (**self).set(pos, cell);
    }

    fn pos(&self) -> Dims {
        (**self).pos()
    }

    fn size(&self) -> Dims {
        (**self).size()
    }
}
