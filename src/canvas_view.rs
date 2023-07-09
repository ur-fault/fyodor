use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

use crate::{frame::CanvasLike, renderer::Dims};

pub struct CanvasContainer<'a> {
    inner: Rc<RefCell<dyn CanvasLike + 'a>>,
}

impl<'a> CanvasContainer<'a> {
    pub fn new<C>(canvas: C) -> Self
    where
        C: CanvasLike + 'a,
    {
        Self {
            inner: Rc::new(RefCell::new(canvas)),
        }
    }

    pub fn borrow(&self) -> Ref<'_, dyn CanvasLike + 'a> {
        self.inner.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<'_, dyn CanvasLike + 'a> {
        self.inner.borrow_mut()
    }
}
