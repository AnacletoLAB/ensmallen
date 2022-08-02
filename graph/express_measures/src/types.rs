use core::fmt::Debug;
use num_traits::{Float, One, Unsigned, Zero};
use std::{
    iter::Sum,
    ops::{AddAssign, DivAssign, MulAssign, Sub, SubAssign},
};

pub trait ThreadFloat:
    Float
    + Send
    + Sync
    + Copy
    + std::iter::Sum
    + SubAssign<Self>
    + AddAssign<Self>
    + MulAssign<Self>
    + DivAssign<Self>
    + Default
{
}
pub trait ThreadUnsigned: Unsigned + Send + Sync + Copy + TryInto<usize> + Debug {}

impl<T> ThreadFloat for T where
    T: Float
        + Send
        + Sync
        + Copy
        + std::iter::Sum
        + SubAssign<Self>
        + AddAssign<Self>
        + MulAssign<Self>
        + DivAssign<Self>
        + Default
{
}
impl<T> ThreadUnsigned for T where T: Unsigned + Send + Sync + Copy + TryInto<usize> + Debug {}

pub trait Coerced<F>:
    Copy + Send + Sync + PartialOrd + PartialOrd + One + Zero + Sum + Sub<Output = Self>
{
    fn coerce_into(self) -> F;
    fn coerce_from(other: F) -> Self;
}

macro_rules! macro_impl_coerce_into_f32 {
    ($($ty:ty)*) => {
        $(
            impl Coerced<f32> for $ty {
                fn coerce_into(self) -> f32 {
                    self as f32
                }
                fn coerce_from(other: f32) -> Self {
                    other as $ty
                }
            }

            impl Coerced<f64> for $ty {
                fn coerce_into(self) -> f64 {
                    self as f64
                }
                fn coerce_from(other: f64) -> Self {
                    other as $ty
                }
            }
        )*
    }
}

macro_impl_coerce_into_f32!(f32 f64 i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 usize);
