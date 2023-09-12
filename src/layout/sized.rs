pub use core_def::*;

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

    impl KnownWidth for &str {
        fn w(&self) -> i32 {
            UnicodeWidthStr::width(*self) as i32
        }
    }
}

pub enum Anchor {
    Start,
    Center,
    End,
}

pub struct Aligned {
    pub margin: i32,
    pub anchor: Anchor,
    pub container: i32,
    pub child: i32,
}

impl Aligned {
    pub fn new(anchor: Anchor, container: i32, child: i32) -> Self {
        Self {
            margin: 0,
            anchor,
            container,
            child,
        }
    }

    pub fn new_x(anchor: Anchor, container: &impl KnownWidth, child: &impl KnownWidth) -> Self {
        Self::new(anchor, container.w(), child.w())
    }
}
