use crate::canvas::CanvasLike;

use super::Drawable;

pub trait SelfLayout: Drawable<Pos = ()> {
    fn just_draw(&self, frame: &mut impl CanvasLike);
}

impl<D> SelfLayout for D
where
    D: Drawable<Pos = ()>,
{
    fn just_draw(&self, frame: &mut impl CanvasLike) {
        self.draw((), frame)
    }
}

pub trait SelfLayoutX: Drawable<Pos = i32> {
    fn draw_on_x(&self, x: i32, frame: &mut impl CanvasLike);
}

impl<D> SelfLayoutX for D
where
    D: Drawable<Pos = i32>,
{
    fn draw_on_x(&self, x: i32, frame: &mut impl CanvasLike) {
        self.draw(x, frame)
    }
}

pub trait SelfLayoutY: Drawable<Pos = i32> {
    fn draw_on_y(&self, y: i32, frame: &mut impl CanvasLike);
}

impl<D> SelfLayoutY for D
where
    D: Drawable<Pos = i32>,
{
    fn draw_on_y(&self, y: i32, frame: &mut impl CanvasLike) {
        self.draw(y, frame)
    }
}
