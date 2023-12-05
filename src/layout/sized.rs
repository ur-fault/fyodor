use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

use crate::Dims;

pub trait KnownWidth {
    fn w(&self) -> i32;
}

impl KnownWidth for str {
    fn w(&self) -> i32 {
        UnicodeWidthStr::width(self) as i32
    }
}

impl KnownWidth for &str {
    fn w(&self) -> i32 {
        UnicodeWidthStr::width(*self) as i32
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

impl_known_height!(str, &str, char, String);

pub trait FullyKnown: KnownWidth + KnownHeight {
    fn dims(&self) -> Dims {
        Dims::new(self.w(), self.h())
    }
}

impl<T> FullyKnown for T where T: KnownWidth + KnownHeight {}
