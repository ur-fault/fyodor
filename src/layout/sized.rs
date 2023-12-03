pub use core_def::*;

use crate::{CanvasLike, Drawable};

use super::{axis::Axis, Pos};

pub mod core_def {
    use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

    pub trait KnownWidth {
        fn w(&self) -> i32;
    }

    impl KnownWidth for str {
        fn w(&self) -> i32 {
            UnicodeWidthStr::width(self) as i32
        }
    }

    impl KnownWidth for char {
        fn w(&self) -> i32 {
            UnicodeWidthChar::width(*self).unwrap_or(0) as i32
        }
    }

    impl KnownWidth for String {
        fn w(&self) -> i32 {
            UnicodeWidthStr::width(self.as_str()) as i32
        }
    }

    pub trait KnownHeight {
        fn h(&self) -> i32;
    }

    macro_rules! impl_known_height {
        ($($t:ty),*) => {
            $(
                impl KnownHeight for $t {
                    fn h(&self) -> i32 {
                        1
                    }
                }
            )*
        };
    }

    impl_known_height!(str, char, String);
}

pub enum Anchor {
    Start,
    Center,
    End,
}

pub struct Align {
    pub margin: i32,
    pub anchor: Anchor,
    pub child: i32,
}

impl Align {
    pub fn new(anchor: Anchor, child: i32) -> Self {
        Self {
            margin: 0,
            anchor,
            child,
        }
    }

    pub fn new_x(anchor: Anchor, child: &impl KnownWidth) -> Self {
        Self::new(anchor, child.w())
    }

    pub fn new_y(anchor: Anchor, child: &impl KnownHeight) -> Self {
        Self::new(anchor, child.h())
    }
}

impl Axis for Align {
    fn calc(&self, container: i32) -> i32 {
        match self.anchor {
            Anchor::Start => self.margin,
            Anchor::Center => (container - self.child) / 2,
            Anchor::End => container - self.child - self.margin,
        }
    }
}

pub struct AlignedOnX<T>(T);

impl<D> Drawable for AlignedOnX<D>
where
    D: KnownWidth + Drawable<X = i32>,
{
    type X = Align;
    type Y = D::Y;

    fn draw(self, pos: impl Into<Pos<Self::X, Self::Y>>, frame: &mut impl CanvasLike) {
        let pos: Pos<_, Self::Y> = pos.into();

        let x = pos.x.calc(frame.size().x);

        self.0.draw((x, pos.y), frame);
    }
}

pub struct AlignedOnY<T>(T);

impl<D> Drawable for AlignedOnY<D>
where
    D: KnownHeight + Drawable<Y = i32>,
{
    type X = D::X;
    type Y = Align;

    fn draw(self, pos: impl Into<Pos<Self::X, Self::Y>>, frame: &mut impl CanvasLike) {
        let pos: Pos<Self::X, _> = pos.into();

        let y = pos.y.calc(frame.size().y);

        self.0.draw((pos.x, y), frame);
    }
}
