use core::fmt::Debug;
use num_traits::{Float, Unsigned};
use std::ops::{AddAssign, MulAssign};

pub trait ThreadFloat:
    Float + Send + Sync + Copy + std::iter::Sum + AddAssign<Self> + MulAssign<Self> + Default
{
}
pub trait ThreadUnsigned: Unsigned + Send + Sync + Copy + TryInto<usize> + Debug {}

impl<T> ThreadFloat for T where
    T: Float + Send + Sync + Copy + std::iter::Sum + AddAssign<Self> + MulAssign<Self> + Default
{
}
impl<T> ThreadUnsigned for T where T: Unsigned + Send + Sync + Copy + TryInto<usize> + Debug {}
