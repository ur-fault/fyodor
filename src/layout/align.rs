use super::{
    axis::Axis,
    sized::{KnownHeight, KnownWidth, FullyKnown},
    Pos,
};

use crossterm::style::ContentStyle;

use crate::{drawable::styled::Stylable, CanvasLike, Drawable};

pub enum Align {
    Start,
    Center,
    End,
}

impl Axis for Align {
    fn calc(&self, item: i32, container: i32) -> i32 {
        match self {
            Self::Start => 0,
            Self::Center => (container - item) / 2,
            Self::End => container - item,
        }
    }
}

pub struct AlignedOnX<T>(pub T);

impl<T> Clone for AlignedOnX<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<D> Drawable for AlignedOnX<D>
where
    D: KnownWidth + Drawable<X = i32>,
{
    type X = Align;
    type Y = D::Y;

    fn draw(&self, pos: impl Into<Pos<Self::X, Self::Y>>, frame: &mut impl CanvasLike) {
        let pos: Pos<_, Self::Y> = pos.into();

        let x = pos.x.calc(self.0.w(), frame.size().x);

        self.0.draw((x, pos.y), frame);
    }
}

impl<D> Drawable for (ContentStyle, &AlignedOnX<D>)
where
    D: Stylable + KnownWidth,
    for<'a> (ContentStyle, &'a D): Drawable<X = i32, Y = D::Y>,
{
    type X = Align;
    type Y = D::Y;

    fn draw(&self, pos: impl Into<Pos<Self::X, Self::Y>>, frame: &mut impl CanvasLike) {
        let Pos { x, y } = pos.into();
        let x = x.calc(self.1 .0.w(), frame.size().x);
        (self.0, &self.1 .0).draw((x, y), frame);
    }
}

pub struct AlignedOnY<T>(pub T);

impl<T> Clone for AlignedOnY<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<D> Drawable for AlignedOnY<D>
where
    D: KnownHeight + Drawable<Y = i32>,
{
    type X = D::X;
    type Y = Align;

    fn draw(&self, pos: impl Into<Pos<Self::X, Self::Y>>, frame: &mut impl CanvasLike) {
        let pos: Pos<Self::X, _> = pos.into();

        let y = pos.y.calc(self.0.h(), frame.size().y);

        self.0.draw((pos.x, y), frame);
    }
}

impl<D> Drawable for (ContentStyle, &AlignedOnY<D>)
where
    D: Stylable + KnownHeight,
    for<'a> (ContentStyle, &'a D): Drawable<X = D::X, Y = i32>,
{
    type X = D::X;
    type Y = Align;

    fn draw(&self, pos: impl Into<Pos<Self::X, Self::Y>>, frame: &mut impl CanvasLike) {
        let Pos { x, y } = pos.into();
        let y = y.calc(self.1 .0.h(), frame.size().y);
        (self.0, &self.1 .0).draw((x, y), frame);
    }
}

pub struct Aligned<T>(pub T);

impl<T> Clone for Aligned<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<D> Drawable for Aligned<D>
where
    D: FullyKnown + Drawable<X = i32, Y = i32>,
{
    type X = Align;
    type Y = Align;

    fn draw(&self, pos: impl Into<Pos<Self::X, Self::Y>>, frame: &mut impl CanvasLike) {
        let pos: Pos<Self::X, _> = pos.into();

        let pos = pos.calc_both(self.0.dims(), frame.size());

        self.0.draw(pos, frame);
    }
}

impl<D> Drawable for (ContentStyle, &Aligned<D>)
where
    D: Stylable + KnownHeight,
    for<'a> (ContentStyle, &'a D): Drawable<X = D::X, Y = i32>,
{
    type X = D::X;
    type Y = Align;

    fn draw(&self, pos: impl Into<Pos<Self::X, Self::Y>>, frame: &mut impl CanvasLike) {
        let Pos { x, y } = pos.into();
        let y = y.calc(self.1 .0.h(), frame.size().y);
        (self.0, &self.1 .0).draw((x, y), frame);
    }
}
