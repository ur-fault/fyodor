pub mod core_impl;
pub mod dbox;
pub mod extended_impl;
pub mod multiline;
pub mod styled;

use crate::{canvas::CanvasLike, layout::Pos};

pub trait Drawable {
    type X;
    type Y;

    fn draw(&self, pos: impl Into<Pos<Self::X, Self::Y>>, frame: &mut impl CanvasLike);
}
