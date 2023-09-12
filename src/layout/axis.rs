pub trait Axis {
    fn calc(&self, container: i32) -> i32;
}

macro_rules! impl_axis {
    ($($t:ty)*) => {
        $(
            impl Axis for $t {
                fn calc(&self, _: i32) -> i32 {
                    i32::try_from(*self).expect("Could not convert to i32")
                }
            }
        )*
    };
}

impl_axis!(i32 u32 i64 u64 i128 u128 i16 u16 i8 u8 usize isize);
