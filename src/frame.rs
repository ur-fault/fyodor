use std::{cell::RefCell, rc::Rc};

use crate::{
    canvas::CanvasLike,
    renderer::{Cell, Dims},
};

#[derive(Clone)]
pub struct Frame<'a> {
    pub pos: Dims,
    pub size: Dims,
    pub clip: bool,
    pub parent: Rc<RefCell<dyn 'a + CanvasLike>>,
}

impl<'a> Frame<'a> {
    pub fn new<P: CanvasLike + 'a>(p: P) -> Self {
        Self {
            pos: (0, 0),
            size: (0, 0),
            clip: true,
            parent: Rc::new(RefCell::new(p)),
        }
    }

    pub fn with_size(mut self, size: Dims) -> Self {
        self.size = size;
        self
    }

    pub fn with_pos(mut self, pos: Dims) -> Self {
        self.pos = pos;
        self
    }

    pub fn no_clip(mut self) -> Self {
        self.clip = false;
        self
    }

    pub fn abs_pos(&self) -> Dims {
        (
            self.pos.0 + self.parent.borrow().pos().0,
            self.pos.1 + self.parent.borrow().pos().1,
        )
    }
}

impl<'a> CanvasLike for Frame<'a> {
    fn set(&mut self, pos: Dims, cell: Cell) {
        if self.clip && (pos.0 < 0 || pos.1 < 0 || pos.0 >= self.size.0 || pos.1 >= self.size.1) {
            return;
        }
        self.parent
            .borrow_mut()
            .set((pos.0 + self.pos.0, pos.1 + self.pos.1), cell);
    }

    fn pos(&self) -> Dims {
        self.pos
    }

    fn size(&self) -> Dims {
        self.size
    }
}
