use core::fmt::Debug;
use half::f16;
use num_traits::{Float, One, Unsigned, Zero};
use std::{
    iter::Sum,
    ops::{AddAssign, Add, Div, DivAssign, MulAssign, Sub, SubAssign},
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
    Copy
    + Send
    + Sync
    + PartialOrd
    + One
    + Zero
    + Sum
    + Add<Output = Self>
    + Sub<Output = Self>
    + Div<Output = Self>
    + SubAssign
    + AddAssign
    + DivAssign
{
    fn coerce_into(self) -> F;
    fn coerce_from(other: F) -> Self;
}

macro_rules! macro_impl_coerce {
    ($($ty:ty)*) => {
        $(
            impl Coerced<f16> for $ty where Self: Coerced<f32> {
                fn coerce_into(self) -> f16 {
                    f16::from_f32(self.coerce_into())
                }
                fn coerce_from(other: f16) -> Self {
                    <$ty>::coerce_from(other.to_f32())
                }
            }

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

impl Coerced<f16> for f16 {
    fn coerce_into(self) -> f16 {
        self
    }
    fn coerce_from(other: f16) -> Self {
        other
    }
}

impl Coerced<f32> for f16 {
    fn coerce_into(self) -> f32 {
        self.to_f32()
    }
    fn coerce_from(other: f32) -> Self {
        f16::from_f32(other)
    }
}

impl Coerced<f64> for f16 {
    fn coerce_into(self) -> f64 {
        self.to_f64()
    }
    fn coerce_from(other: f64) -> Self {
        f16::from_f64(other)
    }
}

macro_impl_coerce!(f32 f64 i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 usize);
