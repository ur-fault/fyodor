pub mod axis;
pub mod pos;
pub mod sized;
pub mod strings;

pub use pos::*;

use crate::{canvas::CanvasLike, drawable::Drawable};

pub type Dims = Pos<i32, i32>;

pub trait SelfLayout: Drawable<X = (), Y = ()> {
    fn just_draw(self, frame: &mut impl CanvasLike);
}

impl<D> SelfLayout for D
where
    D: Drawable<X = (), Y = ()>,
{
    fn just_draw(self, frame: &mut impl CanvasLike) {
        self.draw((), frame)
    }
}

pub trait AutoX: Drawable<X = ()> {
    fn draw_on_y(self, y: impl Into<Self::Y>, frame: &mut impl CanvasLike);
}

impl<D> AutoX for D
where
    D: Drawable<X = ()>,
{
    fn draw_on_y(self, y: impl Into<Self::Y>, frame: &mut impl CanvasLike) {
        self.draw(Pos::new_y(y.into()), frame)
    }
}

pub trait AutoY: Drawable<Y = ()> {
    fn draw_on_x(self, x: Self::X, frame: &mut impl CanvasLike);
}

impl<D> AutoY for D
where
    D: Drawable<Y = ()>,
{
    fn draw_on_x(self, x: Self::X, frame: &mut impl CanvasLike) {
        self.draw(Pos::new_x(x.into()), frame)
    }
}
