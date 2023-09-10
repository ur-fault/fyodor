pub mod core_impl;
pub mod extended_impl;
pub mod mics;

use crate::canvas::CanvasLike;

pub trait Drawable {
    type Pos;

    fn draw(&self, pos: Self::Pos, frame: &mut impl CanvasLike);
}
