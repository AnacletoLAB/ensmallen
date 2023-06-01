use half::f16;
use num_traits::{Float, Num, NumCast, One, ToPrimitive, Zero};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign};
use core::iter::Sum;
use core::num::FpCategory;
use hyperloglog_rs::prelude::*;

// We make this type fully transparent.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub(crate) struct PrimitiveF16 {
    value: f16,
}

impl Primitive<f32> for PrimitiveF16 {
    #[inline(always)]
    fn convert(self) -> f32 {
        self.value.to_f32()
    }

    #[inline(always)]
    fn reverse(other: f32) -> Self {
        Self {
            value: f16::from_f32(other),
        }
    }
}

impl Add for PrimitiveF16 {
    type Output = Self;

    #[inline(always)]
    fn add(self, other: Self) -> Self {
        Self {
            value: self.value + other.value,
        }
    }
}

impl AddAssign for PrimitiveF16 {
    #[inline(always)]
    fn add_assign(&mut self, other: Self) {
        self.value += other.value
    }
}

impl Sub for PrimitiveF16 {
    type Output = Self;

    #[inline(always)]
    fn sub(self, other: Self) -> Self {
        Self {
            value: self.value - other.value,
        }
    }
}

impl SubAssign for PrimitiveF16 {
    #[inline(always)]
    fn sub_assign(&mut self, other: Self) {
        self.value -= other.value
    }
}

impl Mul for PrimitiveF16 {
    type Output = Self;

    #[inline(always)]
    fn mul(self, other: Self) -> Self {
        Self {
            value: self.value * other.value,
        }
    }
}

impl MulAssign for PrimitiveF16 {
    #[inline(always)]
    fn mul_assign(&mut self, other: Self) {
        self.value *= other.value
    }
}

impl Div for PrimitiveF16 {
    type Output = Self;

    #[inline(always)]
    fn div(self, other: Self) -> Self {
        Self {
            value: self.value / other.value,
        }
    }
}

impl DivAssign for PrimitiveF16 {
    #[inline(always)]
    fn div_assign(&mut self, other: Self) {
        self.value /= other.value
    }
}

impl Rem for PrimitiveF16 {
    type Output = Self;

    #[inline(always)]
    fn rem(self, other: Self) -> Self {
        Self {
            value: self.value % other.value,
        }
    }
}

impl PartialEq for PrimitiveF16 {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialOrd for PrimitiveF16 {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Eq for PrimitiveF16 {}

impl Default for PrimitiveF16 {
    #[inline(always)]
    fn default() -> Self {
        Self {
            value: f16::from_f32(0.0),
        }
    }
}

impl Zero for PrimitiveF16 {
    #[inline(always)]
    fn zero() -> Self {
        Self {
            value: f16::from_f32(0.0),
        }
    }

    #[inline(always)]
    fn is_zero(&self) -> bool {
        self.value == f16::from_f32(0.0)
    }
}

impl One for PrimitiveF16 {
    #[inline(always)]
    fn one() -> Self {
        Self {
            value: f16::from_f32(1.0),
        }
    }

    #[inline(always)]
    fn is_one(&self) -> bool {
        self.value == f16::from_f32(1.0)
    }
}

impl Num for PrimitiveF16 {
    type FromStrRadixErr = <f32 as Num>::FromStrRadixErr;

    #[inline(always)]
    fn from_str_radix(str: &str, radix: u32) -> core::result::Result<Self, Self::FromStrRadixErr> {
        f32::from_str_radix(str, radix).map(|x| Self {
            value: f16::from_f32(x),
        })
    }
}

impl Neg for PrimitiveF16 {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self {
        Self { value: -self.value }
    }
}

impl NumCast for PrimitiveF16 {
    #[inline(always)]
    fn from<T: ToPrimitive>(n: T) -> Option<Self> {
        n.to_f32().map(|x| Self {
            value: f16::from_f32(x),
        })
    }
}

impl ToPrimitive for PrimitiveF16 {
    #[inline(always)]
    fn to_i64(&self) -> Option<i64> {
        self.value.to_i64()
    }

    #[inline(always)]
    fn to_u64(&self) -> Option<u64> {
        self.value.to_u64()
    }

    #[inline(always)]
    fn to_isize(&self) -> Option<isize> {
        self.value.to_isize()
    }

    #[inline(always)]
    fn to_i8(&self) -> Option<i8> {
        self.value.to_i8()
    }

    #[inline(always)]
    fn to_i16(&self) -> Option<i16> {
        self.value.to_i16()
    }

    #[inline(always)]
    fn to_i32(&self) -> Option<i32> {
        self.value.to_i32()
    }

    #[inline(always)]
    fn to_usize(&self) -> Option<usize> {
        self.value.to_usize()
    }

    #[inline(always)]
    fn to_u8(&self) -> Option<u8> {
        self.value.to_u8()
    }

    #[inline(always)]
    fn to_u16(&self) -> Option<u16> {
        self.value.to_u16()
    }

    #[inline(always)]
    fn to_u32(&self) -> Option<u32> {
        self.value.to_u32()
    }

    #[inline(always)]
    fn to_f32(&self) -> Option<f32> {
        Some(self.value.to_f32())
    }

    #[inline(always)]
    fn to_f64(&self) -> Option<f64> {
        Some(self.value.to_f64())
    }
}

impl MaxMin for PrimitiveF16 {
    #[inline(always)]
    fn get_max(self, other: Self) -> Self {
        Self {
            value: self.value.max(other.value),
        }
    }

    #[inline(always)]
    fn get_min(self, other: Self) -> Self {
        Self {
            value: self.value.min(other.value),
        }
    }
}

impl Sum for PrimitiveF16 {
    #[inline(always)]
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |acc, x| acc + x)
    }
}

impl Float for PrimitiveF16 {
    #[inline(always)]
    fn nan() -> Self {
        Self {
            value: f16::from_f32(f32::nan()),
        }
    }

    #[inline(always)]
    fn infinity() -> Self {
        Self {
            value: f16::from_f32(f32::infinity()),
        }
    }

    #[inline(always)]
    fn neg_infinity() -> Self {
        Self {
            value: f16::from_f32(f32::neg_infinity()),
        }
    }

    #[inline(always)]
    fn neg_zero() -> Self {
        Self {
            value: f16::from_f32(f32::neg_zero()),
        }
    }

    #[inline(always)]
    fn min_value() -> Self {
        Self {
            value: f16::from_f32(f32::min_value()),
        }
    }

    #[inline(always)]
    fn min_positive_value() -> Self {
        Self {
            value: f16::from_f32(f32::min_positive_value()),
        }
    }

    #[inline(always)]
    fn max_value() -> Self {
        Self {
            value: f16::from_f32(f32::max_value()),
        }
    }

    #[inline(always)]
    fn is_nan(self) -> bool {
        self.value.is_nan()
    }

    #[inline(always)]
    fn is_infinite(self) -> bool {
        self.value.is_infinite()
    }

    #[inline(always)]
    fn is_finite(self) -> bool {
        self.value.is_finite()
    }

    #[inline(always)]
    fn is_normal(self) -> bool {
        self.value.is_normal()
    }

    #[inline(always)]
    fn classify(self) -> FpCategory {
        self.value.classify()
    }

    #[inline(always)]
    fn floor(self) -> Self {
        Self {
            value: self.value.floor(),
        }
    }

    #[inline(always)]
    fn ceil(self) -> Self {
        Self {
            value: self.value.ceil(),
        }
    }

    #[inline(always)]
    fn round(self) -> Self {
        Self {
            value: self.value.round(),
        }
    }

    #[inline(always)]
    fn trunc(self) -> Self {
        Self {
            value: self.value.trunc(),
        }
    }

    #[inline(always)]
    fn fract(self) -> Self {
        Self {
            value: self.value.fract(),
        }
    }

    #[inline(always)]
    fn abs(self) -> Self {
        Self {
            value: self.value.abs(),
        }
    }

    #[inline(always)]
    fn signum(self) -> Self {
        Self {
            value: self.value.signum(),
        }
    }

    #[inline(always)]
    fn is_sign_positive(self) -> bool {
        self.value.is_sign_positive()
    }

    #[inline(always)]
    fn is_sign_negative(self) -> bool {
        self.value.is_sign_negative()
    }

    #[inline(always)]
    fn powi(self, n: i32) -> Self {
        Self {
            value: self.value.powi(n),
        }
    }

    #[inline(always)]
    fn powf(self, n: Self) -> Self {
        Self {
            value: self.value.powf(n.value),
        }
    }

    #[inline(always)]
    fn sqrt(self) -> Self {
        Self {
            value: self.value.sqrt(),
        }
    }

    #[inline(always)]
    fn exp(self) -> Self {
        Self {
            value: self.value.exp(),
        }
    }

    #[inline(always)]
    fn exp2(self) -> Self {
        Self {
            value: self.value.exp2(),
        }
    }

    #[inline(always)]
    fn ln(self) -> Self {
        Self {
            value: self.value.ln(),
        }
    }

    #[inline(always)]
    fn log(self, base: Self) -> Self {
        Self {
            value: self.value.log(base.value),
        }
    }

    #[inline(always)]
    fn log2(self) -> Self {
        Self {
            value: self.value.log2(),
        }
    }

    #[inline(always)]
    fn log10(self) -> Self {
        Self {
            value: self.value.log10(),
        }
    }

    #[inline(always)]
    fn max(self, other: Self) -> Self {
        Self {
            value: self.value.max(other.value),
        }
    }

    #[inline(always)]
    fn min(self, other: Self) -> Self {
        Self {
            value: self.value.min(other.value),
        }
    }

    #[inline(always)]
    fn cbrt(self) -> Self {
        Self {
            value: self.value.cbrt(),
        }
    }

    #[inline(always)]
    fn hypot(self, other: Self) -> Self {
        Self {
            value: self.value.hypot(other.value),
        }
    }

    #[inline(always)]
    fn sin(self) -> Self {
        Self {
            value: self.value.sin(),
        }
    }

    #[inline(always)]
    fn cos(self) -> Self {
        Self {
            value: self.value.cos(),
        }
    }

    #[inline(always)]
    fn tan(self) -> Self {
        Self {
            value: self.value.tan(),
        }
    }

    #[inline(always)]
    fn asin(self) -> Self {
        Self {
            value: self.value.asin(),
        }
    }

    #[inline(always)]
    fn acos(self) -> Self {
        Self {
            value: self.value.acos(),
        }
    }

    #[inline(always)]
    fn atan(self) -> Self {
        Self {
            value: self.value.atan(),
        }
    }

    #[inline(always)]
    fn atan2(self, other: Self) -> Self {
        Self {
            value: self.value.atan2(other.value),
        }
    }

    #[inline(always)]
    fn sin_cos(self) -> (Self, Self) {
        let (sin, cos) = self.value.sin_cos();
        (Self { value: sin }, Self { value: cos })
    }

    #[inline(always)]
    fn exp_m1(self) -> Self {
        Self {
            value: self.value.exp_m1(),
        }
    }

    #[inline(always)]
    fn ln_1p(self) -> Self {
        Self {
            value: self.value.ln_1p(),
        }
    }

    #[inline(always)]
    fn sinh(self) -> Self {
        Self {
            value: self.value.sinh(),
        }
    }

    #[inline(always)]
    fn cosh(self) -> Self {
        Self {
            value: self.value.cosh(),
        }
    }

    #[inline(always)]
    fn tanh(self) -> Self {
        Self {
            value: self.value.tanh(),
        }
    }

    #[inline(always)]
    fn asinh(self) -> Self {
        Self {
            value: self.value.asinh(),
        }
    }

    #[inline(always)]
    fn acosh(self) -> Self {
        Self {
            value: self.value.acosh(),
        }
    }

    #[inline(always)]
    fn atanh(self) -> Self {
        Self {
            value: self.value.atanh(),
        }
    }

    #[inline(always)]
    fn integer_decode(self) -> (u64, i16, i8) {
        self.value.integer_decode()
    }

    #[inline(always)]
    fn epsilon() -> Self {
        Self {
            value: f16::from_f32(f32::epsilon()),
        }
    }

    #[inline(always)]
    fn to_degrees(self) -> Self {
        Self {
            value: self.value.to_degrees(),
        }
    }

    #[inline(always)]
    fn to_radians(self) -> Self {
        Self {
            value: self.value.to_radians(),
        }
    }

    #[inline(always)]
    fn abs_sub(self, other: Self) -> Self {
        Self {
            value: self.value.abs_sub(other.value),
        }
    }

    #[inline(always)]
    fn mul_add(self, a: Self, b: Self) -> Self {
        Self {
            value: self.value.mul_add(a.value, b.value),
        }
    }

    #[inline(always)]
    fn recip(self) -> Self {
        Self {
            value: self.value.recip(),
        }
    }
}