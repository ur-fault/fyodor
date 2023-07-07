use std::marker::PhantomData;

use crate::{
    canvas::Canvas,
    renderer::{Cell, Dims},
};

pub trait FrameLike {
    // fn draw(&mut self, pos: Dims, content: impl Drawable);
    fn set(&mut self, pos: Dims, cell: Cell);
    fn pos(&self) -> Dims;
    fn size(&self) -> Dims;
}

pub struct Frame<P>
where
    P: FrameLike,
{
    pub parent: P,
    pub rel_pos: Dims,
    pub size: Dims,
    pub clip: bool,
}

impl<P> Frame<P>
where
    P: FrameLike,
{
    pub fn to_abs(&self, pos: Dims) -> Dims {
        (pos.0 + self.rel_pos.0, pos.1 + self.rel_pos.1)
    }
}

impl<P> FrameLike for Frame<P>
where
    P: FrameLike,
{
    fn set(&mut self, pos: Dims, cell: Cell) {
        if !self.clip {
            self.parent.set(self.to_abs(pos), cell);
            return;
        }
        if pos.0 >= 0 && pos.1 >= 0 && pos.0 < self.size.0 && pos.1 < self.size.1 {
            self.parent.set(self.to_abs(pos), cell);
        }
    }

    fn pos(&self) -> Dims {
        self.rel_pos
    }

    fn size(&self) -> Dims {
        self.size
    }
}

impl FrameLike for Canvas {
    fn set(&mut self, pos: Dims, cell: Cell) {
        Canvas::set(self, pos, cell); // Otherwise it would be recursive, since target would be FrameLike::draw
    }

    fn pos(&self) -> Dims {
        (0, 0)
    }

    fn size(&self) -> Dims {
        self.size
    }
}

impl<F> FrameLike for &mut F
where
    F: FrameLike,
{
    fn set(&mut self, pos: Dims, cell: Cell) {
        F::set(self, pos, cell); // Otherwise it would be recursive, since target would be FrameLike::draw
    }

    fn pos(&self) -> Dims {
        F::pos(self)
    }

    fn size(&self) -> Dims {
        F::size(self)
    }
}

#[derive(Copy, Debug)]
pub struct FrameBuilder<P>
where
    P: FrameLike,
{
    clip: bool,
    rel_pos: Dims,
    size: Dims,
    p: PhantomData<P>,
}

impl<P> Clone for FrameBuilder<P>
where
    P: FrameLike,
{
    fn clone(&self) -> Self {
        FrameBuilder {
            clip: self.clip,
            rel_pos: self.rel_pos,
            size: self.size,
            p: PhantomData,
        }
    }
}

impl<P> FrameBuilder<P>
where
    P: FrameLike,
{
    pub fn new() -> Self {
        FrameBuilder {
            clip: true,
            rel_pos: (0, 0),
            size: (0, 0),
            p: PhantomData,
        }
    }

    pub fn no_clip(mut self) -> Self {
        self.clip = false;
        self
    }

    pub fn with_pos(mut self, pos: Dims) -> Self {
        self.rel_pos = pos;
        self
    }

    pub fn with_size(mut self, size: Dims) -> Self {
        self.size = size;
        self
    }

    pub fn build(self, p: P) -> Frame<P> {
        Frame {
            parent: p,
            rel_pos: self.rel_pos,
            size: self.size,
            clip: self.clip,
        }
    }
}
