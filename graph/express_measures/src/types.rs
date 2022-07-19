use core::fmt::Debug;
use num_traits::{Float, Unsigned};
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
