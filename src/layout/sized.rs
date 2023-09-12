pub use core_def::*;

use crate::{canvas::CanvasLike, drawable::Drawable};

// use super::{axis::AxisY, AxisX};

pub mod core_def {
    use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

    pub trait KnownWidth {
        fn width(&self) -> u32;
    }

    impl KnownWidth for str {
        fn width(&self) -> u32 {
            UnicodeWidthStr::width(self) as u32
        }
    }

    impl KnownWidth for char {
        fn width(&self) -> u32 {
            UnicodeWidthChar::width(*self).unwrap_or(0) as u32
        }
    }

    impl KnownWidth for String {
        fn width(&self) -> u32 {
            UnicodeWidthStr::width(self.as_str()) as u32
        }
    }

    impl KnownWidth for &str {
        fn width(&self) -> u32 {
            UnicodeWidthStr::width(*self) as u32
        }
    }
}

pub struct RightAligned<T: Sized>(pub T);

impl<T: Sized + KnownWidth> RightAligned<T> {
    pub fn new(t: T) -> Self {
        Self(t)
    }
}

impl<T: Sized + KnownWidth> KnownWidth for RightAligned<T> {
    fn width(&self) -> u32 {
        self.0.width()
    }
}

// impl<T> Drawable for RightAligned<T>
// where
//     T: Sized + KnownWidth + Drawable,
//     T::Pos: AxisY + AxisX + From<(i32, <T::Pos as AxisY>::Dim)>,
//     <T::Pos as AxisX>::Dim: From<i32>,
//     <T::Pos as AxisY>::Dim: AddAxisX,
// {
//     type Pos = <T::Pos as AxisY>::Dim;

//     fn draw(self, y: Self::Pos, frame: &mut impl CanvasLike) {
//         let x = frame.size().0 - self.width() as i32;
//         let pos = y.add_x(x);
//         self.0.draw(pos, frame)
//     }
// }

// impl<T> Drawable for RightAligned<T> where T: Sized + KnownWidth + Drawable {
//     type Pos = <T::Pos as AxisY>::DimY;


// }
