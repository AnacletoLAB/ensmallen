use core::fmt::Debug;
use half::f16;
use num_traits::{Coerced, Float, Unsigned};
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

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
    + Coerced<Self>
    + Coerced<f16>
    + Coerced<f32>
    + Coerced<f64>
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
        + Coerced<Self>
        + Coerced<f16>
        + Coerced<f32>
        + Coerced<f64>
{
}
impl<T> ThreadUnsigned for T where T: Unsigned + Send + Sync + Copy + TryInto<usize> + Debug {}
