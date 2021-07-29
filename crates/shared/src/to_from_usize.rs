use std::hash::Hash;
use std::fmt::Display;
use std::ops::{Add, AddAssign, Sub};

/// Trait used for the Vocabulary class.
/// It represent an unsigned integer that can be converted to and from usize.
/// This allows us to save memory using indicies of smaller size than u64
/// and it has no effects on performance because it's optimized away during
/// compilaton.
pub trait ToFromUsize:
    Clone + Display + Ord + Copy + AddAssign + Add + Sub<Output = Self> + Hash
{
    /// create the type from a usize
    fn from_usize(v: usize) -> Self;
    /// create an usize from the type
    fn to_usize(v: Self) -> usize;
    /// Retrun the maximum encodable number
    fn get_max() -> Self;
}

/// Automatically implement the methods needed to convert from and to usize
/// for the given numerical type.
macro_rules! macro_impl_to_from_usize {
    ($($ty:ty)*) => {
        $(
            impl ToFromUsize for $ty {
                #[inline(always)]
                fn from_usize(v: usize) -> $ty {
                    v as $ty
                }
                #[inline(always)]
                fn to_usize(v: $ty) -> usize {
                    v as usize
                }

                #[inline(always)]
                fn get_max() -> $ty {
                    (0 as $ty).wrapping_sub(1)
                }
            }
        )*
    }
}

macro_impl_to_from_usize!(u8 u16 u32 u64 usize);
