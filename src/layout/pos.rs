use std::ops::{Add, Div, Mul, Sub};

use super::axis::Axis;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pos<X, Y> {
    pub x: X,
    pub y: Y,
}

impl<X, Y> Pos<X, Y> {
    pub fn new(x: X, y: Y) -> Self {
        Self { x, y }
    }

    pub fn with_x<X2: Axis>(self, x: X2) -> Pos<X2, Y> {
        Pos::new(x, self.y)
    }

    pub fn with_y<Y2: Axis>(self, y: Y2) -> Pos<X, Y2> {
        Pos::new(self.x, y)
    }

    pub fn transmute(self) -> Pos<Y, X> {
        Pos::new(self.y, self.x)
    }

    pub fn to<X2, Y2>(self) -> Pos<X2, Y2>
    where
        X: Into<X2>,
        Y: Into<Y2>,
    {
        Pos::new(self.x.into(), self.y.into())
    }
}

impl<X> Pos<X, ()> {
    pub fn new_x(x: X) -> Pos<X, ()> {
        Pos::new(x, ())
    }
}

impl<Y> Pos<(), Y> {
    pub fn new_y(y: Y) -> Pos<(), Y> {
        Pos::new((), y)
    }
}

impl<A> Pos<A, A>
where
    A: Axis + Clone,
{
    pub fn sq(a: A) -> Self {
        Self::new(a.clone(), a)
    }
}

impl<X, Y> Pos<X, Y>
where
    X: Axis + Copy,
{
    pub fn x(&self) -> impl Axis + Copy {
        self.x
    }
}

impl<X, Y> Pos<X, Y>
where
    Y: Axis + Copy,
{
    pub fn y(&self) -> impl Axis + Copy {
        self.y
    }
}

impl<X, Y> Pos<X, Y>
where
    X: Axis,
    Y: Axis,
{
    pub fn calc_both(self, container: Pos<i32, i32>) -> (i32, i32) {
        (self.x.calc(container.x), self.y.calc(container.y))
    }
}

impl<X, Y> From<(X, Y)> for Pos<X, Y> {
    fn from((x, y): (X, Y)) -> Self {
        Self::new(x, y)
    }
}

impl From<()> for Pos<(), ()> {
    fn from(_: ()) -> Self {
        Self::new((), ())
    }
}

impl<X, Y> Add for Pos<X, Y>
where
    X: Add,
    Y: Add,
{
    type Output = Pos<X::Output, Y::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        Pos::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<X, Y> Mul for Pos<X, Y>
where
    X: Mul,
    Y: Mul,
{
    type Output = Pos<X::Output, Y::Output>;

    fn mul(self, rhs: Self) -> Self::Output {
        Pos::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl<X, Y> Sub for Pos<X, Y>
where
    X: Sub,
    Y: Sub,
{
    type Output = Pos<X::Output, Y::Output>;

    fn sub(self, rhs: Self) -> Self::Output {
        Pos::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<X, Y> Div for Pos<X, Y>
where
    X: Div,
    Y: Div,
{
    type Output = Pos<X::Output, Y::Output>;

    fn div(self, rhs: Self) -> Self::Output {
        Pos::new(self.x / rhs.x, self.y / rhs.y)
    }
}
