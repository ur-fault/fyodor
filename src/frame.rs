use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

use crate::{
    canvas::CanvasLike,
    cell::Cell,
    layout::{
        sized::{KnownHeight, KnownWidth},
        Dims, Pos,
    },
};

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
            rel_pos: (0, 0).into(),
            size: p.size(),
            clip: true,
            parent: Rc::new(RefCell::new(p)),
        }
    }

    pub fn with_size(mut self, size: impl Into<Dims>) -> Self {
        self.size = size.into();
        self
    }

    pub fn with_pos(mut self, pos: impl Into<Dims>) -> Self {
        self.rel_pos = pos.into();
        self
    }

    pub fn no_clip(mut self) -> Self {
        self.clip = false;
        self
    }

    pub fn abs_pos(&self) -> Dims {
        Pos::new(
            self.rel_pos.x + self.parent.borrow().pos().x,
            self.rel_pos.y + self.parent.borrow().pos().y,
        )
    }

    pub fn centered(mut self, size: impl Into<Dims>) -> Self {
        let size = size.into();
        self.rel_pos = (self.size - size) / 2;
        self.size = size;
        self
    }

    #[inline(always)]
    pub fn l(mut self, width: i32) -> Self {
        self.size.x = width;
        self
    }

    #[inline(always)]
    pub fn r(mut self, width: i32) -> Self {
        self.rel_pos.x = self.size.x - width;
        self.size.x = width;
        self
    }

    #[inline(always)]
    pub fn t(mut self, height: i32) -> Self {
        self.size.y = height;
        self
    }

    #[inline(always)]
    pub fn b(mut self, height: i32) -> Self {
        self.rel_pos.y = self.size.y - height;
        self.size.y = height;
        self
    }

    #[inline(always)]
    pub fn mx(mut self, m: i32) -> Self {
        self.rel_pos.x += m;
        self.size.x = self.size.x - 2 * m;
        self
    }

    #[inline(always)]
    pub fn my(mut self, m: i32) -> Self {
        self.rel_pos.y += m;
        self.size.y = self.size.y - 2 * m;
        self
    }

    #[inline(always)]
    pub fn ml(mut self, m: i32) -> Self {
        self.rel_pos.x += m;
        self.size.x -= m;
        self
    }

    #[inline(always)]
    pub fn mr(mut self, m: i32) -> Self {
        self.size.x -= m;
        self
    }

    #[inline(always)]
    pub fn mt(mut self, m: i32) -> Self {
        self.rel_pos.y += m;
        self.size.y -= m;
        self
    }

    #[inline(always)]
    pub fn mb(mut self, m: i32) -> Self {
        self.size.y -= m;
        self
    }

    #[inline(always)]
    pub fn parent_mut(&mut self) -> RefMut<dyn CanvasLike> {
        self.parent.borrow_mut()
    }
}

impl<C> KnownWidth for C
where
    C: CanvasLike,
{
    fn w(&self) -> i32 {
        self.size().x
    }
}

impl<C> KnownHeight for C
where
    C: CanvasLike,
{
    fn h(&self) -> i32 {
        self.size().y
    }
}

impl<'a> CanvasLike for Frame<'a> {
    fn set(&mut self, pos: Dims, cell: Cell) {
        if self.clip && (pos.x < 0 || pos.y < 0 || pos.x >= self.size.x || pos.y >= self.size.y) {
            return;
        }
        self.parent.borrow_mut().set(pos + self.rel_pos, cell);
    }

    fn pos(&self) -> Dims {
        self.rel_pos
    }

    fn size(&self) -> Dims {
        self.size
    }
}
