use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use crate::{
    cell::Cell,
    drawable::Drawable,
    layout::{sized::{KnownWidth, KnownHeight}, Dims, Pos},
};

pub struct Buffer {
    buffer: Vec<Vec<Cell>>,
    size: Dims,
}

impl Buffer {
    pub fn new(size: impl Into<Dims>) -> Self {
        let size = size.into();
        let mut buffer = Vec::new();
        for _ in 0..size.y {
            buffer.push(vec![Cell::new(' '); size.x as usize]);
        }
        Buffer { buffer, size }
    }

    pub fn buf_ref(&self) -> &[Vec<Cell>] {
        &self.buffer
    }

    pub fn buf_mut(&mut self) -> &mut [Vec<Cell>] {
        &mut self.buffer
    }

    pub fn resize(&mut self, size: impl Into<Dims>) {
        let size = size.into();
        if self.size == size {
            return;
        }

        self.size = size;
        self.buffer.resize(size.y as usize, Vec::new());
        for row in self.buffer.iter_mut() {
            row.resize(size.x as usize, Cell::new(' '));
        }
    }

    pub fn size(&self) -> Dims {
        self.size
    }
}

pub trait CanvasLike: KnownWidth + KnownHeight {
    fn set(&mut self, pos: Dims, cell: Cell);
    fn pos(&self) -> Dims;
    fn size(&self) -> Dims;

    // we need Self: Sized so that rust knows that we are
    // not using this in a trait object
    fn setd(&mut self, pos: impl Into<Dims>, cell: Cell)
    where
        Self: Sized,
    {
        self.set(pos.into(), cell);
    }

    fn fill(&mut self, cell: Cell) {
        let size = self.size();
        for y in 0..size.y {
            for x in 0..size.x {
                self.set(Pos::new(x, y), cell);
            }
        }
    }
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

    pub fn from_dims(size: impl Into<Dims>) -> Self {
        Self::new(Buffer::new(size))
    }

    pub fn set(&mut self, pos: impl Into<Dims>, cell: Cell) {
        let pos = pos.into();
        if let Some(c) = self
            .buffer
            .borrow_mut() // from RefCell
            .buf_mut() // from Buffer
            .get_mut(pos.y as usize) // from &mut [Vec<Cell>]
            .and_then(|r| r.get_mut(pos.x as usize))
        {
            *c = cell;
        }
    }

    pub fn size(&self) -> Dims {
        self.buffer.borrow().size()
    }

    pub fn get(&self, pos: impl Into<Dims>) -> Option<Cell> {
        let pos = pos.into();
        self.buffer
            .borrow() // from RefCell
            .buf_ref() // from Buffer
            .get(pos.y as usize) // from &[Vec<Cell>]
            .and_then(|r| r.get(pos.x as usize))
            .copied()
    }

    pub fn get_buf(&self) -> Ref<Buffer> {
        self.buffer.borrow()
    }

    pub fn resize(&mut self, size: impl Into<Dims>) {
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
        (0, 0).into()
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

pub trait CanvasLikeExt: CanvasLike {
    fn draw<D: Drawable>(&mut self, pos: impl Into<Pos<D::X, D::Y>>, content: D);
}

impl<C> CanvasLikeExt for C
where
    C: CanvasLike,
{
    fn draw<D: Drawable>(&mut self, pos: impl Into<Pos<D::X, D::Y>>, content: D) {
        content.draw(pos, self);
    }
}
