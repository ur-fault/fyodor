use std::{cell::RefCell, rc::Rc};

use crate::{canvas::CanvasLike, cell::Cell, renderer::Dims};

#[derive(Clone)]
pub struct Frame<'a> {
    pub rel_pos: Dims,
    pub size: Dims,
    pub clip: bool,
    pub parent: Rc<RefCell<dyn 'a + CanvasLike>>,
}

impl<'a> Frame<'a> {
    pub fn new<P: CanvasLike + 'a>(p: P) -> Self {
        Self {
            rel_pos: (0, 0),
            size: p.size(),
            clip: true,
            parent: Rc::new(RefCell::new(p)),
        }
    }

    pub fn with_size(mut self, size: Dims) -> Self {
        self.size = size;
        self
    }

    pub fn with_pos(mut self, pos: Dims) -> Self {
        self.rel_pos = pos;
        self
    }

    pub fn no_clip(mut self) -> Self {
        self.clip = false;
        self
    }

    pub fn abs_pos(&self) -> Dims {
        (
            self.rel_pos.0 + self.parent.borrow().pos().0,
            self.rel_pos.1 + self.parent.borrow().pos().1,
        )
    }

    pub fn clear(&mut self) {
        self.fill(Cell::new(' '));
    }

    pub fn fill(&mut self, cell: Cell) {
        for x in 0..self.size.0 {
            for y in 0..self.size.1 {
                self.set((x, y), cell);
            }
        }
    }

    pub fn centered(mut self, size: Dims) -> Self {
        self.rel_pos = ((self.size.0 - size.0) / 2, (self.size.1 - size.1) / 2);
        self.size = size;
        self
    }

    #[inline(always)]
    pub fn l(mut self, width: i32) -> Self {
        self.size.0 = width;
        self
    }

    #[inline(always)]
    pub fn r(mut self, width: i32) -> Self {
        self.rel_pos.0 = self.size.0 - width;
        self.size.0 = width;
        self
    }

    #[inline(always)]
    pub fn t(mut self, height: i32) -> Self {
        self.size.1 = height;
        self
    }

    #[inline(always)]
    pub fn b(mut self, height: i32) -> Self {
        self.rel_pos.1 = self.size.1 - height;
        self.size.1 = height;
        self
    }

    #[inline(always)]
    pub fn mx(mut self, m: i32) -> Self {
        self.rel_pos.0 += m;
        self.size.0 = self.size.0 - 2 * m;
        self
    }

    #[inline(always)]
    pub fn my(mut self, m: i32) -> Self {
        self.rel_pos.1 += m;
        self.size.1 = self.size.1 - 2 * m;
        self
    }
}

impl<'a> CanvasLike for Frame<'a> {
    fn set(&mut self, pos: Dims, cell: Cell) {
        if self.clip && (pos.0 < 0 || pos.1 < 0 || pos.0 >= self.size.0 || pos.1 >= self.size.1) {
            return;
        }
        self.parent
            .borrow_mut()
            .set((pos.0 + self.rel_pos.0, pos.1 + self.rel_pos.1), cell);
    }

    fn pos(&self) -> Dims {
        self.rel_pos
    }

    fn size(&self) -> Dims {
        self.size
    }
}
