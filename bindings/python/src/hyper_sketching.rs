use super::*;
use crate::mmap_numpy_npy::create_memory_mapped_numpy_array;
use crate::mmap_numpy_npy::Dtype;
use core::iter::Sum;
use core::num::FpCategory;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign};
use cpu_models::HyperSketching as HS;
use cpu_models::MatrixShape;
use half::f16;
use hyperloglog_rs::prelude::*;
use num_traits::{Float, Num, NumCast, One, ToPrimitive, Zero};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;

// We make this type fully transparent.
#[repr(transparent)]
#[derive(Clone, Copy)]
struct PrimitiveF16 {
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

fn array_to_numpy_array1d<const N: usize>(array: [f32; N]) -> Result<Py<PyArray1<f32>>> {
    let gil = pyo3::Python::acquire_gil();
    let result: &PyArray1<f32> = unsafe { PyArray1::new(gil.python(), [N], false) };
    unsafe {
        result
            .as_slice_mut()
            .map_err(|_| "Could not create a mutable slice".to_string())?
            .copy_from_slice(&array)
    };
    Ok(result.to_owned())
}

fn matrix_to_numpy_array2d<const N: usize>(matrix: [[f32; N]; N]) -> Result<Py<PyArray2<f32>>> {
    let gil = pyo3::Python::acquire_gil();
    let result: &PyArray2<f32> = unsafe { PyArray2::new(gil.python(), [N, N], false) };

    unsafe {
        std::ptr::copy_nonoverlapping(
            matrix.as_ptr() as *const f32,
            result
                .as_slice_mut()
                .map_err(|_| "Could not create a mutable slice".to_string())?
                .as_mut_ptr(),
            N * N,
        )
    };

    Ok(result.to_owned())
}

#[derive(Serialize, Deserialize, Clone)]
/// HyperSketching models.
enum InnerModel {
    /// HyperSketching model.
    /// HS{precision}_{bits}_{hops}(HS<Precision{precision}, {bits}, {hops}>), {python_macro}
    HS4_4_2(HS<Precision4, 4, 2>), // {python_generated}
    HS4_4_3(HS<Precision4, 4, 3>),   // {python_generated}
    HS4_4_4(HS<Precision4, 4, 4>),   // {python_generated}
    HS4_4_5(HS<Precision4, 4, 5>),   // {python_generated}
    HS4_4_6(HS<Precision4, 4, 6>),   // {python_generated}
    HS4_5_2(HS<Precision4, 5, 2>),   // {python_generated}
    HS4_5_3(HS<Precision4, 5, 3>),   // {python_generated}
    HS4_5_4(HS<Precision4, 5, 4>),   // {python_generated}
    HS4_5_5(HS<Precision4, 5, 5>),   // {python_generated}
    HS4_5_6(HS<Precision4, 5, 6>),   // {python_generated}
    HS4_6_2(HS<Precision4, 6, 2>),   // {python_generated}
    HS4_6_3(HS<Precision4, 6, 3>),   // {python_generated}
    HS4_6_4(HS<Precision4, 6, 4>),   // {python_generated}
    HS4_6_5(HS<Precision4, 6, 5>),   // {python_generated}
    HS4_6_6(HS<Precision4, 6, 6>),   // {python_generated}
    HS5_4_2(HS<Precision5, 4, 2>),   // {python_generated}
    HS5_4_3(HS<Precision5, 4, 3>),   // {python_generated}
    HS5_4_4(HS<Precision5, 4, 4>),   // {python_generated}
    HS5_4_5(HS<Precision5, 4, 5>),   // {python_generated}
    HS5_4_6(HS<Precision5, 4, 6>),   // {python_generated}
    HS5_5_2(HS<Precision5, 5, 2>),   // {python_generated}
    HS5_5_3(HS<Precision5, 5, 3>),   // {python_generated}
    HS5_5_4(HS<Precision5, 5, 4>),   // {python_generated}
    HS5_5_5(HS<Precision5, 5, 5>),   // {python_generated}
    HS5_5_6(HS<Precision5, 5, 6>),   // {python_generated}
    HS5_6_2(HS<Precision5, 6, 2>),   // {python_generated}
    HS5_6_3(HS<Precision5, 6, 3>),   // {python_generated}
    HS5_6_4(HS<Precision5, 6, 4>),   // {python_generated}
    HS5_6_5(HS<Precision5, 6, 5>),   // {python_generated}
    HS5_6_6(HS<Precision5, 6, 6>),   // {python_generated}
    HS6_4_2(HS<Precision6, 4, 2>),   // {python_generated}
    HS6_4_3(HS<Precision6, 4, 3>),   // {python_generated}
    HS6_4_4(HS<Precision6, 4, 4>),   // {python_generated}
    HS6_4_5(HS<Precision6, 4, 5>),   // {python_generated}
    HS6_4_6(HS<Precision6, 4, 6>),   // {python_generated}
    HS6_5_2(HS<Precision6, 5, 2>),   // {python_generated}
    HS6_5_3(HS<Precision6, 5, 3>),   // {python_generated}
    HS6_5_4(HS<Precision6, 5, 4>),   // {python_generated}
    HS6_5_5(HS<Precision6, 5, 5>),   // {python_generated}
    HS6_5_6(HS<Precision6, 5, 6>),   // {python_generated}
    HS6_6_2(HS<Precision6, 6, 2>),   // {python_generated}
    HS6_6_3(HS<Precision6, 6, 3>),   // {python_generated}
    HS6_6_4(HS<Precision6, 6, 4>),   // {python_generated}
    HS6_6_5(HS<Precision6, 6, 5>),   // {python_generated}
    HS6_6_6(HS<Precision6, 6, 6>),   // {python_generated}
    HS7_4_2(HS<Precision7, 4, 2>),   // {python_generated}
    HS7_4_3(HS<Precision7, 4, 3>),   // {python_generated}
    HS7_4_4(HS<Precision7, 4, 4>),   // {python_generated}
    HS7_4_5(HS<Precision7, 4, 5>),   // {python_generated}
    HS7_4_6(HS<Precision7, 4, 6>),   // {python_generated}
    HS7_5_2(HS<Precision7, 5, 2>),   // {python_generated}
    HS7_5_3(HS<Precision7, 5, 3>),   // {python_generated}
    HS7_5_4(HS<Precision7, 5, 4>),   // {python_generated}
    HS7_5_5(HS<Precision7, 5, 5>),   // {python_generated}
    HS7_5_6(HS<Precision7, 5, 6>),   // {python_generated}
    HS7_6_2(HS<Precision7, 6, 2>),   // {python_generated}
    HS7_6_3(HS<Precision7, 6, 3>),   // {python_generated}
    HS7_6_4(HS<Precision7, 6, 4>),   // {python_generated}
    HS7_6_5(HS<Precision7, 6, 5>),   // {python_generated}
    HS7_6_6(HS<Precision7, 6, 6>),   // {python_generated}
    HS8_4_2(HS<Precision8, 4, 2>),   // {python_generated}
    HS8_4_3(HS<Precision8, 4, 3>),   // {python_generated}
    HS8_4_4(HS<Precision8, 4, 4>),   // {python_generated}
    HS8_4_5(HS<Precision8, 4, 5>),   // {python_generated}
    HS8_4_6(HS<Precision8, 4, 6>),   // {python_generated}
    HS8_5_2(HS<Precision8, 5, 2>),   // {python_generated}
    HS8_5_3(HS<Precision8, 5, 3>),   // {python_generated}
    HS8_5_4(HS<Precision8, 5, 4>),   // {python_generated}
    HS8_5_5(HS<Precision8, 5, 5>),   // {python_generated}
    HS8_5_6(HS<Precision8, 5, 6>),   // {python_generated}
    HS8_6_2(HS<Precision8, 6, 2>),   // {python_generated}
    HS8_6_3(HS<Precision8, 6, 3>),   // {python_generated}
    HS8_6_4(HS<Precision8, 6, 4>),   // {python_generated}
    HS8_6_5(HS<Precision8, 6, 5>),   // {python_generated}
    HS8_6_6(HS<Precision8, 6, 6>),   // {python_generated}
    HS9_4_2(HS<Precision9, 4, 2>),   // {python_generated}
    HS9_4_3(HS<Precision9, 4, 3>),   // {python_generated}
    HS9_4_4(HS<Precision9, 4, 4>),   // {python_generated}
    HS9_4_5(HS<Precision9, 4, 5>),   // {python_generated}
    HS9_4_6(HS<Precision9, 4, 6>),   // {python_generated}
    HS9_5_2(HS<Precision9, 5, 2>),   // {python_generated}
    HS9_5_3(HS<Precision9, 5, 3>),   // {python_generated}
    HS9_5_4(HS<Precision9, 5, 4>),   // {python_generated}
    HS9_5_5(HS<Precision9, 5, 5>),   // {python_generated}
    HS9_5_6(HS<Precision9, 5, 6>),   // {python_generated}
    HS9_6_2(HS<Precision9, 6, 2>),   // {python_generated}
    HS9_6_3(HS<Precision9, 6, 3>),   // {python_generated}
    HS9_6_4(HS<Precision9, 6, 4>),   // {python_generated}
    HS9_6_5(HS<Precision9, 6, 5>),   // {python_generated}
    HS9_6_6(HS<Precision9, 6, 6>),   // {python_generated}
    HS10_4_2(HS<Precision10, 4, 2>), // {python_generated}
    HS10_4_3(HS<Precision10, 4, 3>), // {python_generated}
    HS10_4_4(HS<Precision10, 4, 4>), // {python_generated}
    HS10_4_5(HS<Precision10, 4, 5>), // {python_generated}
    HS10_4_6(HS<Precision10, 4, 6>), // {python_generated}
    HS10_5_2(HS<Precision10, 5, 2>), // {python_generated}
    HS10_5_3(HS<Precision10, 5, 3>), // {python_generated}
    HS10_5_4(HS<Precision10, 5, 4>), // {python_generated}
    HS10_5_5(HS<Precision10, 5, 5>), // {python_generated}
    HS10_5_6(HS<Precision10, 5, 6>), // {python_generated}
    HS10_6_2(HS<Precision10, 6, 2>), // {python_generated}
    HS10_6_3(HS<Precision10, 6, 3>), // {python_generated}
    HS10_6_4(HS<Precision10, 6, 4>), // {python_generated}
    HS10_6_5(HS<Precision10, 6, 5>), // {python_generated}
    HS10_6_6(HS<Precision10, 6, 6>), // {python_generated}
    HS11_4_2(HS<Precision11, 4, 2>), // {python_generated}
    HS11_4_3(HS<Precision11, 4, 3>), // {python_generated}
    HS11_4_4(HS<Precision11, 4, 4>), // {python_generated}
    HS11_4_5(HS<Precision11, 4, 5>), // {python_generated}
    HS11_4_6(HS<Precision11, 4, 6>), // {python_generated}
    HS11_5_2(HS<Precision11, 5, 2>), // {python_generated}
    HS11_5_3(HS<Precision11, 5, 3>), // {python_generated}
    HS11_5_4(HS<Precision11, 5, 4>), // {python_generated}
    HS11_5_5(HS<Precision11, 5, 5>), // {python_generated}
    HS11_5_6(HS<Precision11, 5, 6>), // {python_generated}
    HS11_6_2(HS<Precision11, 6, 2>), // {python_generated}
    HS11_6_3(HS<Precision11, 6, 3>), // {python_generated}
    HS11_6_4(HS<Precision11, 6, 4>), // {python_generated}
    HS11_6_5(HS<Precision11, 6, 5>), // {python_generated}
    HS11_6_6(HS<Precision11, 6, 6>), // {python_generated}
    HS12_4_2(HS<Precision12, 4, 2>), // {python_generated}
    HS12_4_3(HS<Precision12, 4, 3>), // {python_generated}
    HS12_4_4(HS<Precision12, 4, 4>), // {python_generated}
    HS12_4_5(HS<Precision12, 4, 5>), // {python_generated}
    HS12_4_6(HS<Precision12, 4, 6>), // {python_generated}
    HS12_5_2(HS<Precision12, 5, 2>), // {python_generated}
    HS12_5_3(HS<Precision12, 5, 3>), // {python_generated}
    HS12_5_4(HS<Precision12, 5, 4>), // {python_generated}
    HS12_5_5(HS<Precision12, 5, 5>), // {python_generated}
    HS12_5_6(HS<Precision12, 5, 6>), // {python_generated}
    HS12_6_2(HS<Precision12, 6, 2>), // {python_generated}
    HS12_6_3(HS<Precision12, 6, 3>), // {python_generated}
    HS12_6_4(HS<Precision12, 6, 4>), // {python_generated}
    HS12_6_5(HS<Precision12, 6, 5>), // {python_generated}
    HS12_6_6(HS<Precision12, 6, 6>), // {python_generated}
}

impl InnerModel {
    /// Return a new instance of the HyperSketching model.
    ///
    /// Parameters
    /// ------------------------
    /// number_of_hops: Option<usize>
    ///     The number of hops for the Sketches. By default, `2`.
    /// precision: usize
    ///     The precision of the HyperLogLog counters. By default, `6`.
    ///     The supported values range from `4` to `16`.
    /// bits: usize
    ///     The number of bits of the HyperLogLog counters. By default, `5`.
    ///     The supported values range from `4` to `6`.
    /// include_node_types: Option<bool>
    ///     Whether to include the node types in the sketches.
    ///     By default, `false`.
    /// include_edge_types: Option<bool>
    ///     Whether to include the edge types in the sketches.
    ///     By default, `false`.
    /// include_edge_ids: Option<bool>
    ///     Whether to include the edge ids in the sketches.
    ///     By default, `false`.
    /// include_node_ids: Option<bool>
    ///     Whether to include the node ids in the sketches.
    ///     By default, `false`.
    /// include_typed_graphlets: Option<bool>
    ///     Whether to include the typed graphlets in the sketches.
    ///     By default, `false`.
    /// normalize_by_symmetric_laplacian: Option<bool>
    ///     Whether to normalize the adjacency matrix by the symmetric Laplacian.
    ///     By default, `false`.
    /// concatenate_features: Option<bool>
    ///     Whether to concatenate the features to the sketches.
    ///     By default, `false`.
    /// dtype: Option<String>
    ///     The data type to be employed, by default f32.
    ///     The supported values are f16, f32 and f64.
    ///
    /// Raises
    /// ------------------------
    /// ValueError
    ///     If the provided precision is not in the supported range.
    ///     If the provided bits is not in the supported range.
    ///     The feature concatenation only makes sense if the normalization is enabled.
    ///     If the edge ids are requested but the number of HOPS is only one.
    ///     If none of the include flags is enabled.
    fn new(
        number_of_hops: Option<usize>,
        precision: usize,
        bits: usize,
        include_node_types: Option<bool>,
        include_edge_types: Option<bool>,
        include_edge_ids: Option<bool>,
        include_node_ids: Option<bool>,
        include_typed_graphlets: Option<bool>,
        normalize_by_symmetric_laplacian: Option<bool>,
        concatenate_features: Option<bool>,
        dtype: Option<String>,
    ) -> Result<Self> {
        // Since actually writing the code for the following match would make
        // for very hard to read code, we proceed instead with a Python script.

        match (precision, bits, number_of_hops.unwrap_or(2)) {
            // ({precision}, {bits}, {hops}) => Ok(InnerModel::HS{precision}_{bits}_{hops}(HS::new(include_node_types, include_edge_types, include_edge_ids, include_node_ids, include_typed_graphlets, normalize_by_symmetric_laplacian, concatenate_features, dtype)?)), {python_macro}
            (4, 4, 2) => Ok(InnerModel::HS4_4_2(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (4, 4, 3) => Ok(InnerModel::HS4_4_3(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (4, 4, 4) => Ok(InnerModel::HS4_4_4(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (4, 4, 5) => Ok(InnerModel::HS4_4_5(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (4, 4, 6) => Ok(InnerModel::HS4_4_6(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (4, 5, 2) => Ok(InnerModel::HS4_5_2(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (4, 5, 3) => Ok(InnerModel::HS4_5_3(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (4, 5, 4) => Ok(InnerModel::HS4_5_4(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (4, 5, 5) => Ok(InnerModel::HS4_5_5(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (4, 5, 6) => Ok(InnerModel::HS4_5_6(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (4, 6, 2) => Ok(InnerModel::HS4_6_2(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (4, 6, 3) => Ok(InnerModel::HS4_6_3(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (4, 6, 4) => Ok(InnerModel::HS4_6_4(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (4, 6, 5) => Ok(InnerModel::HS4_6_5(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (4, 6, 6) => Ok(InnerModel::HS4_6_6(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (5, 4, 2) => Ok(InnerModel::HS5_4_2(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (5, 4, 3) => Ok(InnerModel::HS5_4_3(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (5, 4, 4) => Ok(InnerModel::HS5_4_4(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (5, 4, 5) => Ok(InnerModel::HS5_4_5(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (5, 4, 6) => Ok(InnerModel::HS5_4_6(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (5, 5, 2) => Ok(InnerModel::HS5_5_2(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (5, 5, 3) => Ok(InnerModel::HS5_5_3(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (5, 5, 4) => Ok(InnerModel::HS5_5_4(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (5, 5, 5) => Ok(InnerModel::HS5_5_5(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (5, 5, 6) => Ok(InnerModel::HS5_5_6(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (5, 6, 2) => Ok(InnerModel::HS5_6_2(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (5, 6, 3) => Ok(InnerModel::HS5_6_3(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (5, 6, 4) => Ok(InnerModel::HS5_6_4(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (5, 6, 5) => Ok(InnerModel::HS5_6_5(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (5, 6, 6) => Ok(InnerModel::HS5_6_6(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (6, 4, 2) => Ok(InnerModel::HS6_4_2(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (6, 4, 3) => Ok(InnerModel::HS6_4_3(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (6, 4, 4) => Ok(InnerModel::HS6_4_4(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (6, 4, 5) => Ok(InnerModel::HS6_4_5(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (6, 4, 6) => Ok(InnerModel::HS6_4_6(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (6, 5, 2) => Ok(InnerModel::HS6_5_2(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (6, 5, 3) => Ok(InnerModel::HS6_5_3(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (6, 5, 4) => Ok(InnerModel::HS6_5_4(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (6, 5, 5) => Ok(InnerModel::HS6_5_5(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (6, 5, 6) => Ok(InnerModel::HS6_5_6(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (6, 6, 2) => Ok(InnerModel::HS6_6_2(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (6, 6, 3) => Ok(InnerModel::HS6_6_3(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (6, 6, 4) => Ok(InnerModel::HS6_6_4(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (6, 6, 5) => Ok(InnerModel::HS6_6_5(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (6, 6, 6) => Ok(InnerModel::HS6_6_6(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (7, 4, 2) => Ok(InnerModel::HS7_4_2(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (7, 4, 3) => Ok(InnerModel::HS7_4_3(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (7, 4, 4) => Ok(InnerModel::HS7_4_4(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (7, 4, 5) => Ok(InnerModel::HS7_4_5(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (7, 4, 6) => Ok(InnerModel::HS7_4_6(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (7, 5, 2) => Ok(InnerModel::HS7_5_2(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (7, 5, 3) => Ok(InnerModel::HS7_5_3(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (7, 5, 4) => Ok(InnerModel::HS7_5_4(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (7, 5, 5) => Ok(InnerModel::HS7_5_5(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (7, 5, 6) => Ok(InnerModel::HS7_5_6(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (7, 6, 2) => Ok(InnerModel::HS7_6_2(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (7, 6, 3) => Ok(InnerModel::HS7_6_3(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (7, 6, 4) => Ok(InnerModel::HS7_6_4(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (7, 6, 5) => Ok(InnerModel::HS7_6_5(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (7, 6, 6) => Ok(InnerModel::HS7_6_6(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (8, 4, 2) => Ok(InnerModel::HS8_4_2(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (8, 4, 3) => Ok(InnerModel::HS8_4_3(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (8, 4, 4) => Ok(InnerModel::HS8_4_4(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (8, 4, 5) => Ok(InnerModel::HS8_4_5(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (8, 4, 6) => Ok(InnerModel::HS8_4_6(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (8, 5, 2) => Ok(InnerModel::HS8_5_2(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (8, 5, 3) => Ok(InnerModel::HS8_5_3(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (8, 5, 4) => Ok(InnerModel::HS8_5_4(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (8, 5, 5) => Ok(InnerModel::HS8_5_5(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (8, 5, 6) => Ok(InnerModel::HS8_5_6(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (8, 6, 2) => Ok(InnerModel::HS8_6_2(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (8, 6, 3) => Ok(InnerModel::HS8_6_3(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (8, 6, 4) => Ok(InnerModel::HS8_6_4(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (8, 6, 5) => Ok(InnerModel::HS8_6_5(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (8, 6, 6) => Ok(InnerModel::HS8_6_6(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (9, 4, 2) => Ok(InnerModel::HS9_4_2(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (9, 4, 3) => Ok(InnerModel::HS9_4_3(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (9, 4, 4) => Ok(InnerModel::HS9_4_4(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (9, 4, 5) => Ok(InnerModel::HS9_4_5(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (9, 4, 6) => Ok(InnerModel::HS9_4_6(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (9, 5, 2) => Ok(InnerModel::HS9_5_2(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (9, 5, 3) => Ok(InnerModel::HS9_5_3(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (9, 5, 4) => Ok(InnerModel::HS9_5_4(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (9, 5, 5) => Ok(InnerModel::HS9_5_5(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (9, 5, 6) => Ok(InnerModel::HS9_5_6(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (9, 6, 2) => Ok(InnerModel::HS9_6_2(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (9, 6, 3) => Ok(InnerModel::HS9_6_3(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (9, 6, 4) => Ok(InnerModel::HS9_6_4(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (9, 6, 5) => Ok(InnerModel::HS9_6_5(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (9, 6, 6) => Ok(InnerModel::HS9_6_6(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (10, 4, 2) => Ok(InnerModel::HS10_4_2(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (10, 4, 3) => Ok(InnerModel::HS10_4_3(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (10, 4, 4) => Ok(InnerModel::HS10_4_4(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (10, 4, 5) => Ok(InnerModel::HS10_4_5(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (10, 4, 6) => Ok(InnerModel::HS10_4_6(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (10, 5, 2) => Ok(InnerModel::HS10_5_2(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (10, 5, 3) => Ok(InnerModel::HS10_5_3(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (10, 5, 4) => Ok(InnerModel::HS10_5_4(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (10, 5, 5) => Ok(InnerModel::HS10_5_5(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (10, 5, 6) => Ok(InnerModel::HS10_5_6(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (10, 6, 2) => Ok(InnerModel::HS10_6_2(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (10, 6, 3) => Ok(InnerModel::HS10_6_3(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (10, 6, 4) => Ok(InnerModel::HS10_6_4(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (10, 6, 5) => Ok(InnerModel::HS10_6_5(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (10, 6, 6) => Ok(InnerModel::HS10_6_6(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (11, 4, 2) => Ok(InnerModel::HS11_4_2(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (11, 4, 3) => Ok(InnerModel::HS11_4_3(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (11, 4, 4) => Ok(InnerModel::HS11_4_4(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (11, 4, 5) => Ok(InnerModel::HS11_4_5(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (11, 4, 6) => Ok(InnerModel::HS11_4_6(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (11, 5, 2) => Ok(InnerModel::HS11_5_2(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (11, 5, 3) => Ok(InnerModel::HS11_5_3(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (11, 5, 4) => Ok(InnerModel::HS11_5_4(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (11, 5, 5) => Ok(InnerModel::HS11_5_5(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (11, 5, 6) => Ok(InnerModel::HS11_5_6(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (11, 6, 2) => Ok(InnerModel::HS11_6_2(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (11, 6, 3) => Ok(InnerModel::HS11_6_3(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (11, 6, 4) => Ok(InnerModel::HS11_6_4(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (11, 6, 5) => Ok(InnerModel::HS11_6_5(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (11, 6, 6) => Ok(InnerModel::HS11_6_6(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (12, 4, 2) => Ok(InnerModel::HS12_4_2(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (12, 4, 3) => Ok(InnerModel::HS12_4_3(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (12, 4, 4) => Ok(InnerModel::HS12_4_4(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (12, 4, 5) => Ok(InnerModel::HS12_4_5(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (12, 4, 6) => Ok(InnerModel::HS12_4_6(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (12, 5, 2) => Ok(InnerModel::HS12_5_2(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (12, 5, 3) => Ok(InnerModel::HS12_5_3(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (12, 5, 4) => Ok(InnerModel::HS12_5_4(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (12, 5, 5) => Ok(InnerModel::HS12_5_5(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (12, 5, 6) => Ok(InnerModel::HS12_5_6(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (12, 6, 2) => Ok(InnerModel::HS12_6_2(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (12, 6, 3) => Ok(InnerModel::HS12_6_3(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (12, 6, 4) => Ok(InnerModel::HS12_6_4(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (12, 6, 5) => Ok(InnerModel::HS12_6_5(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (12, 6, 6) => Ok(InnerModel::HS12_6_6(HS::new(
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_typed_graphlets,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            _ => {
                return Err(format!(
                    concat!(
                        "The HyperSketching model supports precisions ranging from 4 ",
                        "to 16 and bits ranging from 4 to 6, and hops from 2 to 7. ",
                        "Provided precision: {}, bits: {}."
                    ),
                    precision, bits
                ))
            }
        }
    }

    /// Returns the data type to be used for the sketches
    fn get_dtype(&self) -> &str {
        match self {
            // InnerModel::HS{precision}_{bits}_{hops}(inner) => inner.get_dtype(), {python_macro}
            InnerModel::HS4_4_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS4_4_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS4_4_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS4_4_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS4_4_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS4_5_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS4_5_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS4_5_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS4_5_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS4_5_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS4_6_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS4_6_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS4_6_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS4_6_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS4_6_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS5_4_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS5_4_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS5_4_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS5_4_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS5_4_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS5_5_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS5_5_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS5_5_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS5_5_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS5_5_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS5_6_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS5_6_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS5_6_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS5_6_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS5_6_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS6_4_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS6_4_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS6_4_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS6_4_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS6_4_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS6_5_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS6_5_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS6_5_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS6_5_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS6_5_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS6_6_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS6_6_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS6_6_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS6_6_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS6_6_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS7_4_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS7_4_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS7_4_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS7_4_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS7_4_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS7_5_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS7_5_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS7_5_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS7_5_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS7_5_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS7_6_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS7_6_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS7_6_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS7_6_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS7_6_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS8_4_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS8_4_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS8_4_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS8_4_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS8_4_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS8_5_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS8_5_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS8_5_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS8_5_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS8_5_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS8_6_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS8_6_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS8_6_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS8_6_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS8_6_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS9_4_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS9_4_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS9_4_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS9_4_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS9_4_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS9_5_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS9_5_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS9_5_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS9_5_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS9_5_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS9_6_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS9_6_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS9_6_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS9_6_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS9_6_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS10_4_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS10_4_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS10_4_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS10_4_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS10_4_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS10_5_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS10_5_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS10_5_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS10_5_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS10_5_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS10_6_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS10_6_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS10_6_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS10_6_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS10_6_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS11_4_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS11_4_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS11_4_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS11_4_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS11_4_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS11_5_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS11_5_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS11_5_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS11_5_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS11_5_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS11_6_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS11_6_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS11_6_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS11_6_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS11_6_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS12_4_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS12_4_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS12_4_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS12_4_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS12_4_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS12_5_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS12_5_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS12_5_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS12_5_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS12_5_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS12_6_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS12_6_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS12_6_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS12_6_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS12_6_6(inner) => inner.get_dtype(), // {python_generated}
        }
    }

    /// Returns the number of bits used for the HyperLogLog counters in the model.
    fn get_bits(&self) -> usize {
        match self {
            // InnerModel::HS{precision}_{bits}_{hops}(inner) => inner.get_bits(), {python_macro}
            InnerModel::HS4_4_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_4_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_4_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_4_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_4_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_5_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_5_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_5_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_5_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_5_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_6_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_6_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_6_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_6_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_6_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_4_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_4_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_4_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_4_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_4_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_5_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_5_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_5_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_5_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_5_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_6_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_6_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_6_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_6_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_6_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_4_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_4_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_4_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_4_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_4_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_5_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_5_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_5_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_5_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_5_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_6_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_6_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_6_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_6_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_6_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_4_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_4_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_4_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_4_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_4_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_5_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_5_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_5_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_5_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_5_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_6_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_6_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_6_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_6_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_6_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_4_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_4_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_4_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_4_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_4_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_5_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_5_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_5_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_5_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_5_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_6_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_6_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_6_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_6_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_6_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_4_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_4_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_4_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_4_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_4_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_5_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_5_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_5_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_5_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_5_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_6_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_6_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_6_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_6_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_6_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_4_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_4_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_4_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_4_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_4_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_5_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_5_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_5_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_5_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_5_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_6_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_6_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_6_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_6_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_6_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_4_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_4_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_4_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_4_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_4_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_5_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_5_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_5_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_5_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_5_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_6_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_6_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_6_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_6_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_6_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_4_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_4_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_4_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_4_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_4_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_5_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_5_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_5_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_5_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_5_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_6_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_6_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_6_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_6_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_6_6(inner) => inner.get_bits(), // {python_generated}
        }
    }

    /// Returns the precision used for the HyperLogLog counters in the model.
    fn get_precision(&self) -> usize {
        match self {
            // InnerModel::HS{precision}_{bits}_{hops}(inner) => inner.get_precision(), {python_macro}
            InnerModel::HS4_4_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_4_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_4_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_4_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_4_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_5_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_5_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_5_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_5_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_5_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_6_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_6_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_6_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_6_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_6_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_4_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_4_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_4_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_4_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_4_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_5_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_5_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_5_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_5_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_5_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_6_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_6_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_6_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_6_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_6_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_4_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_4_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_4_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_4_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_4_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_5_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_5_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_5_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_5_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_5_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_6_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_6_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_6_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_6_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_6_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_4_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_4_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_4_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_4_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_4_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_5_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_5_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_5_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_5_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_5_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_6_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_6_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_6_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_6_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_6_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_4_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_4_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_4_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_4_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_4_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_5_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_5_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_5_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_5_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_5_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_6_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_6_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_6_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_6_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_6_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_4_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_4_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_4_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_4_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_4_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_5_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_5_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_5_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_5_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_5_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_6_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_6_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_6_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_6_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_6_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_4_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_4_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_4_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_4_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_4_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_5_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_5_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_5_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_5_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_5_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_6_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_6_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_6_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_6_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_6_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_4_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_4_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_4_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_4_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_4_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_5_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_5_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_5_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_5_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_5_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_6_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_6_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_6_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_6_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_6_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_4_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_4_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_4_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_4_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_4_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_5_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_5_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_5_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_5_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_5_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_6_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_6_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_6_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_6_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_6_6(inner) => inner.get_precision(), // {python_generated}
        }
    }

    /// Returns the number of hops used for the HyperLogLog counters in the model.
    fn get_number_of_hops(&self) -> usize {
        match self {
            // InnerModel::HS{precision}_{bits}_{hops}(inner) => inner.get_number_of_hops(), {python_macro}
            InnerModel::HS4_4_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_4_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_4_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_4_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_4_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_5_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_5_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_5_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_5_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_5_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_6_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_6_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_6_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_6_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_6_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_4_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_4_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_4_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_4_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_4_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_5_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_5_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_5_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_5_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_5_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_6_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_6_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_6_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_6_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_6_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_4_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_4_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_4_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_4_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_4_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_5_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_5_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_5_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_5_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_5_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_6_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_6_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_6_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_6_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_6_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_4_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_4_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_4_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_4_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_4_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_5_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_5_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_5_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_5_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_5_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_6_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_6_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_6_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_6_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_6_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_4_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_4_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_4_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_4_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_4_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_5_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_5_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_5_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_5_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_5_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_6_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_6_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_6_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_6_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_6_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_4_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_4_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_4_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_4_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_4_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_5_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_5_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_5_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_5_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_5_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_6_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_6_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_6_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_6_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_6_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_4_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_4_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_4_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_4_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_4_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_5_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_5_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_5_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_5_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_5_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_6_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_6_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_6_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_6_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_6_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_4_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_4_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_4_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_4_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_4_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_5_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_5_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_5_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_5_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_5_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_6_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_6_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_6_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_6_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_6_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_4_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_4_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_4_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_4_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_4_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_5_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_5_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_5_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_5_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_5_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_6_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_6_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_6_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_6_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_6_6(inner) => inner.get_number_of_hops(), // {python_generated}
        }
    }

    /// Returns whether the features will be normalized using the symmetric Laplacian.
    fn get_normalize_by_symmetric_laplacian(&self) -> bool {
        match self {
            // InnerModel::HS{precision}_{bits}_{hops}(inner) => inner.get_normalize_by_symmetric_laplacian(), {python_macro}
            InnerModel::HS4_4_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_4_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_4_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_4_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_4_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_5_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_5_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_5_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_5_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_5_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_6_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_6_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_6_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_6_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_6_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_4_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_4_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_4_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_4_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_4_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_5_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_5_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_5_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_5_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_5_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_6_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_6_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_6_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_6_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_6_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_4_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_4_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_4_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_4_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_4_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_5_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_5_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_5_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_5_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_5_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_6_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_6_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_6_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_6_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_6_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_4_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_4_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_4_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_4_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_4_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_5_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_5_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_5_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_5_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_5_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_6_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_6_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_6_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_6_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_6_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_4_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_4_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_4_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_4_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_4_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_5_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_5_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_5_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_5_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_5_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_6_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_6_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_6_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_6_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_6_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_4_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_4_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_4_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_4_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_4_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_5_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_5_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_5_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_5_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_5_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_6_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_6_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_6_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_6_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_6_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_4_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_4_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_4_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_4_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_4_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_5_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_5_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_5_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_5_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_5_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_6_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_6_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_6_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_6_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_6_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_4_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_4_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_4_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_4_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_4_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_5_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_5_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_5_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_5_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_5_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_6_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_6_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_6_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_6_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_6_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_4_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_4_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_4_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_4_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_4_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_5_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_5_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_5_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_5_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_5_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_6_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_6_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_6_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_6_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_6_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
        }
    }

    /// Returns whether the features will be concatenated to the embeddings.
    fn get_concatenate_features(&self) -> bool {
        match self {
            // InnerModel::HS{precision}_{bits}_{hops}(inner) => inner.get_concatenate_features(), {python_macro}
            InnerModel::HS4_4_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_4_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_4_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_4_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_4_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_5_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_5_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_5_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_5_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_5_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_6_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_6_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_6_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_6_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_6_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_4_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_4_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_4_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_4_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_4_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_5_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_5_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_5_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_5_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_5_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_6_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_6_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_6_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_6_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_6_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_4_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_4_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_4_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_4_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_4_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_5_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_5_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_5_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_5_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_5_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_6_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_6_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_6_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_6_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_6_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_4_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_4_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_4_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_4_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_4_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_5_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_5_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_5_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_5_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_5_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_6_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_6_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_6_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_6_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_6_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_4_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_4_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_4_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_4_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_4_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_5_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_5_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_5_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_5_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_5_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_6_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_6_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_6_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_6_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_6_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_4_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_4_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_4_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_4_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_4_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_5_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_5_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_5_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_5_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_5_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_6_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_6_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_6_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_6_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_6_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_4_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_4_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_4_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_4_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_4_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_5_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_5_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_5_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_5_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_5_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_6_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_6_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_6_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_6_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_6_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_4_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_4_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_4_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_4_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_4_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_5_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_5_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_5_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_5_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_5_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_6_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_6_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_6_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_6_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_6_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_4_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_4_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_4_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_4_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_4_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_5_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_5_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_5_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_5_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_5_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_6_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_6_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_6_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_6_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_6_6(inner) => inner.get_concatenate_features(), // {python_generated}
        }
    }

    /// Fit the HyperSketching model to the provided graph.
    ///
    /// Parameters
    /// ------------------------
    /// graph: &Graph
    ///    The graph whose topology is to be learned.
    fn fit(&mut self, graph: &graph::Graph) -> Result<()> {
        match self {
            // InnerModel::HS{precision}_{bits}_{hops}(inner) => inner.fit(graph), {python_macro}
            InnerModel::HS4_4_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_4_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_4_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_4_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_4_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_5_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_5_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_5_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_5_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_5_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_6_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_6_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_6_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_6_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_6_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_4_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_4_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_4_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_4_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_4_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_5_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_5_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_5_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_5_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_5_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_6_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_6_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_6_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_6_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_6_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_4_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_4_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_4_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_4_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_4_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_5_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_5_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_5_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_5_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_5_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_6_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_6_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_6_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_6_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_6_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_4_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_4_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_4_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_4_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_4_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_5_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_5_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_5_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_5_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_5_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_6_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_6_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_6_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_6_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_6_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_4_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_4_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_4_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_4_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_4_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_5_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_5_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_5_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_5_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_5_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_6_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_6_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_6_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_6_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_6_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_4_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_4_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_4_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_4_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_4_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_5_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_5_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_5_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_5_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_5_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_6_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_6_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_6_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_6_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_6_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_4_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_4_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_4_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_4_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_4_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_5_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_5_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_5_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_5_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_5_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_6_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_6_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_6_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_6_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_6_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_4_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_4_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_4_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_4_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_4_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_5_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_5_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_5_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_5_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_5_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_6_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_6_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_6_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_6_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_6_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_4_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_4_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_4_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_4_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_4_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_5_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_5_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_5_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_5_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_5_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_6_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_6_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_6_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_6_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_6_6(inner) => inner.fit(graph), // {python_generated}
        }
    }

    /// Returns the exclusive overlap cardinality between two nodes.
    ///
    /// Parameters
    /// ------------------------
    /// src: usize
    ///     The source node.
    /// dst: usize
    ///     The destination node.
    ///
    /// Raises
    /// ------------------------
    /// ValueError
    ///     If the provided nodes are not in the graph.
    ///     If the model has not been trained.
    fn get_overlap_cardinalities_from_node_ids(
        &self,
        src: usize,
        dst: usize,
    ) -> Result<Py<PyArray2<f32>>> {
        match self {
            // InnerModel::HS{precision}_{bits}_{hops}(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), {python_macro}
            InnerModel::HS4_4_2(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS4_4_3(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS4_4_4(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS4_4_5(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS4_4_6(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS4_5_2(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS4_5_3(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS4_5_4(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS4_5_5(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS4_5_6(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS4_6_2(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS4_6_3(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS4_6_4(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS4_6_5(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS4_6_6(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS5_4_2(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS5_4_3(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS5_4_4(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS5_4_5(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS5_4_6(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS5_5_2(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS5_5_3(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS5_5_4(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS5_5_5(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS5_5_6(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS5_6_2(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS5_6_3(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS5_6_4(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS5_6_5(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS5_6_6(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS6_4_2(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS6_4_3(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS6_4_4(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS6_4_5(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS6_4_6(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS6_5_2(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS6_5_3(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS6_5_4(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS6_5_5(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS6_5_6(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS6_6_2(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS6_6_3(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS6_6_4(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS6_6_5(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS6_6_6(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS7_4_2(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS7_4_3(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS7_4_4(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS7_4_5(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS7_4_6(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS7_5_2(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS7_5_3(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS7_5_4(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS7_5_5(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS7_5_6(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS7_6_2(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS7_6_3(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS7_6_4(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS7_6_5(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS7_6_6(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS8_4_2(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS8_4_3(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS8_4_4(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS8_4_5(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS8_4_6(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS8_5_2(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS8_5_3(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS8_5_4(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS8_5_5(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS8_5_6(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS8_6_2(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS8_6_3(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS8_6_4(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS8_6_5(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS8_6_6(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS9_4_2(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS9_4_3(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS9_4_4(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS9_4_5(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS9_4_6(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS9_5_2(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS9_5_3(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS9_5_4(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS9_5_5(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS9_5_6(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS9_6_2(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS9_6_3(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS9_6_4(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS9_6_5(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS9_6_6(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS10_4_2(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS10_4_3(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS10_4_4(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS10_4_5(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS10_4_6(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS10_5_2(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS10_5_3(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS10_5_4(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS10_5_5(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS10_5_6(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS10_6_2(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS10_6_3(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS10_6_4(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS10_6_5(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS10_6_6(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS11_4_2(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS11_4_3(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS11_4_4(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS11_4_5(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS11_4_6(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS11_5_2(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS11_5_3(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS11_5_4(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS11_5_5(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS11_5_6(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS11_6_2(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS11_6_3(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS11_6_4(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS11_6_5(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS11_6_6(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS12_4_2(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS12_4_3(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS12_4_4(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS12_4_5(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS12_4_6(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS12_5_2(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS12_5_3(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS12_5_4(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS12_5_5(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS12_5_6(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS12_6_2(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS12_6_3(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS12_6_4(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS12_6_5(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS12_6_6(inner) => {
                matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
        }
    }

    /// Returns the estimated exclusive differences cardinality between two nodes.
    ///
    /// Parameters
    /// ------------------------
    /// src: usize
    ///     The source node.
    /// dst: usize
    ///     The destination node.
    ///
    /// Raises
    /// ------------------------
    /// ValueError
    ///     If the provided nodes are not in the graph.
    ///     If the model has not been trained.
    ///
    fn get_difference_cardinalities_from_node_ids(
        &self,
        src: usize,
        dst: usize,
    ) -> Result<Py<PyArray1<f32>>> {
        match self {
            // InnerModel::HS{precision}_{bits}_{hops}(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), {python_macro}
            InnerModel::HS4_4_2(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS4_4_3(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS4_4_4(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS4_4_5(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS4_4_6(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS4_5_2(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS4_5_3(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS4_5_4(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS4_5_5(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS4_5_6(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS4_6_2(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS4_6_3(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS4_6_4(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS4_6_5(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS4_6_6(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS5_4_2(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS5_4_3(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS5_4_4(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS5_4_5(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS5_4_6(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS5_5_2(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS5_5_3(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS5_5_4(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS5_5_5(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS5_5_6(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS5_6_2(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS5_6_3(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS5_6_4(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS5_6_5(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS5_6_6(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS6_4_2(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS6_4_3(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS6_4_4(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS6_4_5(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS6_4_6(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS6_5_2(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS6_5_3(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS6_5_4(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS6_5_5(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS6_5_6(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS6_6_2(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS6_6_3(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS6_6_4(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS6_6_5(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS6_6_6(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS7_4_2(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS7_4_3(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS7_4_4(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS7_4_5(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS7_4_6(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS7_5_2(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS7_5_3(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS7_5_4(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS7_5_5(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS7_5_6(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS7_6_2(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS7_6_3(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS7_6_4(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS7_6_5(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS7_6_6(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS8_4_2(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS8_4_3(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS8_4_4(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS8_4_5(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS8_4_6(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS8_5_2(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS8_5_3(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS8_5_4(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS8_5_5(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS8_5_6(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS8_6_2(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS8_6_3(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS8_6_4(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS8_6_5(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS8_6_6(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS9_4_2(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS9_4_3(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS9_4_4(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS9_4_5(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS9_4_6(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS9_5_2(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS9_5_3(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS9_5_4(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS9_5_5(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS9_5_6(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS9_6_2(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS9_6_3(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS9_6_4(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS9_6_5(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS9_6_6(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS10_4_2(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS10_4_3(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS10_4_4(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS10_4_5(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS10_4_6(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS10_5_2(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS10_5_3(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS10_5_4(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS10_5_5(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS10_5_6(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS10_6_2(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS10_6_3(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS10_6_4(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS10_6_5(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS10_6_6(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS11_4_2(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS11_4_3(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS11_4_4(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS11_4_5(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS11_4_6(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS11_5_2(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS11_5_3(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS11_5_4(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS11_5_5(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS11_5_6(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS11_6_2(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS11_6_3(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS11_6_4(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS11_6_5(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS11_6_6(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS12_4_2(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS12_4_3(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS12_4_4(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS12_4_5(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS12_4_6(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS12_5_2(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS12_5_3(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS12_5_4(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS12_5_5(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS12_5_6(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS12_6_2(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS12_6_3(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS12_6_4(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS12_6_5(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
            InnerModel::HS12_6_6(inner) => {
                array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?)
            } // {python_generated}
        }
    }

    /// Returns the estimated Sketching for all edges.
    ///
    /// Parameters
    /// ------------------------
    /// overlaps: &mut [f32]
    ///     The array where to store the estimated overlaps.
    /// src_differences: &mut [f32]
    ///     The array where to store the estimated source differences.
    /// dst_differences: &mut [f32]
    ///     The array where to store the estimated destination differences.
    /// graph: &Graph
    ///     The graph whose topology is to be learned.
    ///
    /// Raises
    /// ------------------------
    /// ValueError
    ///     If the provided arrays are not of the right size.
    ///     If the model has not been trained.
    ///
    fn compute_sketching_from_iterator<I, F: Float + Primitive<f32>>(
        &self,
        overlaps: &mut [F],
        src_differences: &mut [F],
        dst_differences: &mut [F],
        graph: &graph::Graph,
        edge_iterator: I,
    ) -> Result<()>
    where
        I: IndexedParallelIterator<Item = (NodeT, NodeT)>,
    {
        match self {
            // InnerModel::HS{precision}_{bits}_{hops}(inner) => inner.get_sketching_for_all_edges::<I, F>(overlaps, src_differences, dst_differences, graph, edge_iterator), {python_macro}
            InnerModel::HS4_4_2(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS4_4_3(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS4_4_4(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS4_4_5(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS4_4_6(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS4_5_2(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS4_5_3(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS4_5_4(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS4_5_5(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS4_5_6(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS4_6_2(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS4_6_3(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS4_6_4(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS4_6_5(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS4_6_6(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS5_4_2(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS5_4_3(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS5_4_4(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS5_4_5(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS5_4_6(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS5_5_2(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS5_5_3(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS5_5_4(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS5_5_5(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS5_5_6(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS5_6_2(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS5_6_3(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS5_6_4(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS5_6_5(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS5_6_6(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS6_4_2(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS6_4_3(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS6_4_4(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS6_4_5(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS6_4_6(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS6_5_2(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS6_5_3(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS6_5_4(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS6_5_5(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS6_5_6(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS6_6_2(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS6_6_3(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS6_6_4(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS6_6_5(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS6_6_6(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS7_4_2(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS7_4_3(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS7_4_4(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS7_4_5(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS7_4_6(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS7_5_2(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS7_5_3(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS7_5_4(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS7_5_5(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS7_5_6(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS7_6_2(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS7_6_3(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS7_6_4(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS7_6_5(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS7_6_6(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS8_4_2(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS8_4_3(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS8_4_4(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS8_4_5(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS8_4_6(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS8_5_2(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS8_5_3(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS8_5_4(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS8_5_5(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS8_5_6(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS8_6_2(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS8_6_3(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS8_6_4(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS8_6_5(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS8_6_6(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS9_4_2(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS9_4_3(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS9_4_4(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS9_4_5(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS9_4_6(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS9_5_2(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS9_5_3(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS9_5_4(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS9_5_5(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS9_5_6(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS9_6_2(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS9_6_3(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS9_6_4(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS9_6_5(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS9_6_6(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS10_4_2(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS10_4_3(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS10_4_4(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS10_4_5(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS10_4_6(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS10_5_2(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS10_5_3(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS10_5_4(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS10_5_5(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS10_5_6(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS10_6_2(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS10_6_3(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS10_6_4(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS10_6_5(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS10_6_6(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS11_4_2(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS11_4_3(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS11_4_4(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS11_4_5(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS11_4_6(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS11_5_2(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS11_5_3(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS11_5_4(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS11_5_5(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS11_5_6(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS11_6_2(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS11_6_3(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS11_6_4(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS11_6_5(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS11_6_6(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS12_4_2(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS12_4_3(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS12_4_4(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS12_4_5(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS12_4_6(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS12_5_2(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS12_5_3(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS12_5_4(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS12_5_5(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS12_5_6(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS12_6_2(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS12_6_3(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS12_6_4(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS12_6_5(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS12_6_6(inner) => inner.get_sketching_for_all_edges::<I, F>(
                overlaps,
                src_differences,
                dst_differences,
                graph,
                edge_iterator,
            ), // {python_generated}
        }
    }

    /// Returns the estimated Sketching for all edges.
    ///
    /// Parameters
    /// ------------------------
    /// graph: &Graph
    ///     The graph whose topology is to be learned.
    /// overlap_path: Option<&str>
    ///     The path where to store the estimated overlaps.
    /// left_difference_path: Option<&str>
    ///     The path where to store the estimated source differences.
    /// right_difference_path: Option<&str>
    ///     The path where to store the estimated destination differences.
    /// edge_iterator: I
    ///    The iterator over the edges to be considered.
    ///
    /// Raises
    /// ------------------------
    /// ValueError
    ///     If the provided arrays are not of the right size.
    ///     If the model has not been trained.
    ///
    fn get_sketching_from_iterator<I>(
        &self,
        graph: &graph::Graph,
        overlap_path: Option<&str>,
        left_difference_path: Option<&str>,
        right_difference_path: Option<&str>,
        edge_iterator: I,
    ) -> PyResult<(Py<PyAny>, Py<PyAny>, Py<PyAny>)>
    where
        I: IndexedParallelIterator<Item = (NodeT, NodeT)>,
    {
        let gil = Python::acquire_gil();

        let [overlap_shape, left_diff_shape, right_diff_shape] = if self.get_concatenate_features()
        {
            [
                MatrixShape::FourDimensional(
                    edge_iterator.len(),
                    2,
                    self.get_number_of_hops(),
                    self.get_number_of_hops(),
                ),
                MatrixShape::ThreeDimensional(edge_iterator.len(), 2, self.get_number_of_hops()),
                MatrixShape::ThreeDimensional(edge_iterator.len(), 2, self.get_number_of_hops()),
            ]
        } else {
            [
                MatrixShape::ThreeDimensional(
                    edge_iterator.len(),
                    self.get_number_of_hops(),
                    self.get_number_of_hops(),
                ),
                MatrixShape::BiDimensional(edge_iterator.len(), self.get_number_of_hops()),
                MatrixShape::BiDimensional(edge_iterator.len(), self.get_number_of_hops()),
            ]
        };

        match self.get_dtype() {
            "f16" => {
                let overlaps = create_memory_mapped_numpy_array(
                    gil.python(),
                    overlap_path,
                    Dtype::F16,
                    &<MatrixShape as Into<Vec<isize>>>::into(overlap_shape),
                    false,
                );

                let left_difference = create_memory_mapped_numpy_array(
                    gil.python(),
                    left_difference_path,
                    Dtype::F16,
                    &<MatrixShape as Into<Vec<isize>>>::into(left_diff_shape),
                    false,
                );

                let right_difference = create_memory_mapped_numpy_array(
                    gil.python(),
                    right_difference_path,
                    Dtype::F16,
                    &<MatrixShape as Into<Vec<isize>>>::into(right_diff_shape),
                    false,
                );

                let mut array2d = Vec::new();
                let mut array3d = Vec::new();
                let mut array4d = Vec::new();

                let overlaps_ref = if self.get_concatenate_features() {
                    let array = overlaps.cast_as::<PyArray4<f16>>(gil.python())?;
                    let array_ref = unsafe { array.as_slice_mut()? };
                    array4d.push(array);
                    array_ref
                } else {
                    let array = overlaps.cast_as::<PyArray3<f16>>(gil.python())?;
                    let array_ref = unsafe { array.as_slice_mut()? };
                    array3d.push(array);
                    array_ref
                };

                let left_ref = if self.get_concatenate_features() {
                    let array = left_difference.cast_as::<PyArray3<f16>>(gil.python())?;
                    let array_ref = unsafe { array.as_slice_mut()? };
                    array3d.push(array);
                    array_ref
                } else {
                    let array = left_difference.cast_as::<PyArray2<f16>>(gil.python())?;
                    let array_ref = unsafe { array.as_slice_mut()? };
                    array2d.push(array);
                    array_ref
                };

                let right_ref = if self.get_concatenate_features() {
                    let array = right_difference.cast_as::<PyArray3<f16>>(gil.python())?;
                    let array_ref = unsafe { array.as_slice_mut()? };
                    array3d.push(array);
                    array_ref
                } else {
                    let array = right_difference.cast_as::<PyArray2<f16>>(gil.python())?;
                    let array_ref = unsafe { array.as_slice_mut()? };
                    array2d.push(array);
                    array_ref
                };

                let overlaps_ref = unsafe {
                    core::mem::transmute::<&mut [f16], &mut [PrimitiveF16]>(overlaps_ref)
                };

                let left_ref =
                    unsafe { core::mem::transmute::<&mut [f16], &mut [PrimitiveF16]>(left_ref) };

                let right_ref =
                    unsafe { core::mem::transmute::<&mut [f16], &mut [PrimitiveF16]>(right_ref) };

                // We always use the racing version of the fit transfor
                // as we generally do not care about memory collisions.
                pe!(self.compute_sketching_from_iterator::<I, PrimitiveF16>(
                    overlaps_ref,
                    left_ref,
                    right_ref,
                    &graph,
                    edge_iterator
                ))?;

                Ok((overlaps, left_difference, right_difference))
            }
            "f32" => {
                let overlaps = create_memory_mapped_numpy_array(
                    gil.python(),
                    overlap_path,
                    Dtype::F32,
                    &<MatrixShape as Into<Vec<isize>>>::into(overlap_shape),
                    false,
                );

                let left_difference = create_memory_mapped_numpy_array(
                    gil.python(),
                    left_difference_path,
                    Dtype::F32,
                    &<MatrixShape as Into<Vec<isize>>>::into(left_diff_shape),
                    false,
                );

                let right_difference = create_memory_mapped_numpy_array(
                    gil.python(),
                    right_difference_path,
                    Dtype::F32,
                    &<MatrixShape as Into<Vec<isize>>>::into(right_diff_shape),
                    false,
                );

                let mut array2d = Vec::new();
                let mut array3d = Vec::new();
                let mut array4d = Vec::new();

                let overlaps_ref = if self.get_concatenate_features() {
                    let array = overlaps.cast_as::<PyArray4<f32>>(gil.python())?;
                    let array_ref = unsafe { array.as_slice_mut()? };
                    array4d.push(array);
                    array_ref
                } else {
                    let array = overlaps.cast_as::<PyArray3<f32>>(gil.python())?;
                    let array_ref = unsafe { array.as_slice_mut()? };
                    array3d.push(array);
                    array_ref
                };

                let left_ref = if self.get_concatenate_features() {
                    let array = left_difference.cast_as::<PyArray3<f32>>(gil.python())?;
                    let array_ref = unsafe { array.as_slice_mut()? };
                    array3d.push(array);
                    array_ref
                } else {
                    let array = left_difference.cast_as::<PyArray2<f32>>(gil.python())?;
                    let array_ref = unsafe { array.as_slice_mut()? };
                    array2d.push(array);
                    array_ref
                };

                let right_ref = if self.get_concatenate_features() {
                    let array = right_difference.cast_as::<PyArray3<f32>>(gil.python())?;
                    let array_ref = unsafe { array.as_slice_mut()? };
                    array3d.push(array);
                    array_ref
                } else {
                    let array = right_difference.cast_as::<PyArray2<f32>>(gil.python())?;
                    let array_ref = unsafe { array.as_slice_mut()? };
                    array2d.push(array);
                    array_ref
                };

                // We always use the racing version of the fit transfor
                // as we generally do not care about memory collisions.
                pe!(self.compute_sketching_from_iterator(
                    overlaps_ref,
                    left_ref,
                    right_ref,
                    &graph,
                    edge_iterator
                ))?;

                Ok((overlaps, left_difference, right_difference))
            }
            "f64" => {
                let overlaps = create_memory_mapped_numpy_array(
                    gil.python(),
                    overlap_path,
                    Dtype::F64,
                    &<MatrixShape as Into<Vec<isize>>>::into(overlap_shape),
                    false,
                );

                let left_difference = create_memory_mapped_numpy_array(
                    gil.python(),
                    left_difference_path,
                    Dtype::F64,
                    &<MatrixShape as Into<Vec<isize>>>::into(left_diff_shape),
                    false,
                );

                let right_difference = create_memory_mapped_numpy_array(
                    gil.python(),
                    right_difference_path,
                    Dtype::F64,
                    &<MatrixShape as Into<Vec<isize>>>::into(right_diff_shape),
                    false,
                );

                let mut array2d = Vec::new();
                let mut array3d = Vec::new();
                let mut array4d = Vec::new();

                let overlaps_ref = if self.get_concatenate_features() {
                    let array = overlaps.cast_as::<PyArray4<f64>>(gil.python())?;
                    let array_ref = unsafe { array.as_slice_mut()? };
                    array4d.push(array);
                    array_ref
                } else {
                    let array = overlaps.cast_as::<PyArray3<f64>>(gil.python())?;
                    let array_ref = unsafe { array.as_slice_mut()? };
                    array3d.push(array);
                    array_ref
                };

                let left_ref = if self.get_concatenate_features() {
                    let array = left_difference.cast_as::<PyArray3<f64>>(gil.python())?;
                    let array_ref = unsafe { array.as_slice_mut()? };
                    array3d.push(array);
                    array_ref
                } else {
                    let array = left_difference.cast_as::<PyArray2<f64>>(gil.python())?;
                    let array_ref = unsafe { array.as_slice_mut()? };
                    array2d.push(array);
                    array_ref
                };

                let right_ref = if self.get_concatenate_features() {
                    let array = right_difference.cast_as::<PyArray3<f64>>(gil.python())?;
                    let array_ref = unsafe { array.as_slice_mut()? };
                    array3d.push(array);
                    array_ref
                } else {
                    let array = right_difference.cast_as::<PyArray2<f64>>(gil.python())?;
                    let array_ref = unsafe { array.as_slice_mut()? };
                    array2d.push(array);
                    array_ref
                };

                // We always use the racing version of the fit transfor
                // as we generally do not care about memory collisions.
                pe!(self.compute_sketching_from_iterator(
                    overlaps_ref,
                    left_ref,
                    right_ref,
                    &graph,
                    edge_iterator
                ))?;

                Ok((overlaps, left_difference, right_difference))
            }
            dtype => pe!(Err(format!(
                concat!(
                    "The provided dtype {} is not supported. The supported ",
                    "data types are `f16`, `f32` and `f64`."
                ),
                dtype
            ))),
        }
    }

    pub fn dump(&self, path: &str) -> Result<()> {
        serde_json::to_writer(
            std::fs::File::create(path).map_err(|e| e.to_string())?,
            self,
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn dumps(&self) -> Result<String> {
        serde_json::to_string(self).map_err(|e| e.to_string())
    }

    pub fn load(path: &str) -> Result<Self> {
        serde_json::from_reader(std::fs::File::open(path).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())
    }

    pub fn loads(json: &str) -> Result<Self> {
        serde_json::from_str(json).map_err(|e| e.to_string())
    }
}

/// HyperSketching model.
#[pyclass]
#[derive(Clone)]
#[pyo3(
    text_signature = "(*, number_of_hops=2, precision=6, bits=5, include_node_types=False, include_edge_types=False, include_edge_ids=False, include_node_ids=True, include_typed_graphlets=False, normalize_by_symmetric_laplacian=False, concatenate_features=False, dtype=str)"
)]
pub struct HyperSketching {
    inner: InnerModel,
}

#[pymethods]
impl HyperSketching {
    #[new]
    #[args(py_kwargs = "**")]
    /// Return a new instance of the HyperSketching model.
    ///
    /// Parameters
    /// ------------------------
    /// number_of_hops: int = 2
    ///     The number of hops for the Sketches.
    /// precision: int = 6
    ///     The precision of the HyperLogLog counters.
    ///     The supported values range from 4 to 16.
    /// bits: int = 5
    ///     The number of bits of the HyperLogLog counters.
    ///     The supported values range from 4 to 6.
    /// include_node_type: bool = False
    ///     Whether to include the node type in the sketches.
    ///     By default, `false`.
    /// include_edge_type: bool = False
    ///     Whether to include the edge type in the sketches.
    ///     By default, `false`.
    /// include_edge_id: bool = False
    ///     Whether to include the edge id in the sketches.
    ///     By default, `false`.
    /// include_node_id: bool = False
    ///     Whether to include the node id in the sketches.
    ///     By default, `false`.
    /// include_typed_graphlets: bool = False
    ///     Whether to include the typed graphlets in the sketches.
    ///     By default, `false`.
    /// normalize_by_symmetric_laplacian: bool = False
    ///     Whether to normalize the adjacency matrix by the symmetric Laplacian.
    ///     By default, `false`.
    /// concatenate_features: bool = False
    ///     Whether to concatenate the features to the sketches.
    ///     By default, `false`.
    /// dtype: str = "f32"
    ///     The data type to use for the sketches.
    ///     The supported values are `f16`, `f32` and `f64`.
    ///
    /// Raises
    /// ------------------------
    /// ValueError
    ///     If the provided precision is not in the supported range.
    ///     If the provided bits is not in the supported range.
    ///     The feature concatenation only makes sense if the normalization is enabled.
    ///     If edge ids are included in the sketches, but only two hops are used.
    ///
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<HyperSketching> {
        let py = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, py.python());

        pe!(validate_kwargs(
            kwargs,
            &[
                "number_of_hops",
                "precision",
                "bits",
                "include_node_types",
                "include_edge_types",
                "include_edge_ids",
                "include_node_ids",
                "include_typed_graphlets",
                "normalize_by_symmetric_laplacian",
                "concatenate_features",
                "dtype"
            ],
        ))?;

        Ok(Self {
            inner: pe!(InnerModel::new(
                extract_value_rust_result!(kwargs, "number_of_hops", usize),
                extract_value_rust_result!(kwargs, "precision", usize).unwrap_or(6),
                extract_value_rust_result!(kwargs, "bits", usize).unwrap_or(5),
                extract_value_rust_result!(kwargs, "include_node_types", bool),
                extract_value_rust_result!(kwargs, "include_edge_types", bool),
                extract_value_rust_result!(kwargs, "include_edge_ids", bool),
                extract_value_rust_result!(kwargs, "include_node_ids", bool),
                extract_value_rust_result!(kwargs, "include_typed_graphlets", bool),
                extract_value_rust_result!(kwargs, "normalize_by_symmetric_laplacian", bool),
                extract_value_rust_result!(kwargs, "concatenate_features", bool),
                extract_value_rust_result!(kwargs, "dtype", String),
            ))?,
        })
    }
}

#[pymethods]
impl HyperSketching {
    #[args(py_kwargs = "**")]
    #[pyo3(text_signature = "($self, graph)")]
    /// Fit the HyperSketching model to the provided graph.
    ///
    /// Parameters
    /// ---------
    /// graph: Graph
    ///     The graph whose topology is to be learned.
    ///
    /// Raises
    /// ---------
    /// ValueError
    ///    If node types are not provided in the graph and the model is configured to include them.
    ///    If edge types are not provided in the graph and the model is configured to include them.
    fn fit(&mut self, graph: &Graph) -> PyResult<()> {
        pe!(self.inner.fit(&graph.inner,))
    }

    #[args(py_kwargs = "**")]
    #[pyo3(text_signature = "($self, src, dst)")]
    /// Returns the exclusive overlap cardinality between two nodes.
    ///
    /// Parameters
    /// ------------------------
    /// src: int
    ///    The source node.
    /// dst: int
    ///   The destination node.
    ///
    /// Raises
    /// ------------------------
    /// ValueError
    ///    If the provided nodes are not in the graph.
    ///    If the model has not been trained.
    ///
    fn get_overlap_cardinalities_from_node_ids(
        &self,
        src: usize,
        dst: usize,
    ) -> PyResult<Py<PyArray2<f32>>> {
        pe!(self.inner.get_overlap_cardinalities_from_node_ids(src, dst))
    }

    #[args(py_kwargs = "**")]
    #[pyo3(text_signature = "($self, src, dst)")]
    /// Returns the estimated exclusive differences cardinality between two nodes.
    ///
    /// Parameters
    /// ------------------------
    /// src: int
    ///    The source node.
    /// dst: int
    ///   The destination node.
    ///
    /// Raises
    /// ------------------------
    /// ValueError
    ///    If the provided nodes are not in the graph.
    ///   If the model has not been trained.
    ///
    fn get_difference_cardinalities_from_node_ids(
        &self,
        src: usize,
        dst: usize,
    ) -> PyResult<Py<PyArray1<f32>>> {
        pe!(self
            .inner
            .get_difference_cardinalities_from_node_ids(src, dst))
    }

    #[args(py_kwargs = "**")]
    #[pyo3(text_signature = "($self,)")]
    /// Returns whether the features will be normalized using the symmetric Laplacian.
    fn get_normalize_by_symmetric_laplacian(&self) -> bool {
        self.inner.get_normalize_by_symmetric_laplacian()
    }

    #[args(py_kwargs = "**")]
    #[pyo3(text_signature = "($self,)")]
    /// Returns whether the features will be concatenated to the embeddings.
    fn get_concatenate_features(&self) -> bool {
        self.inner.get_concatenate_features()
    }

    #[args(py_kwargs = "**")]
    #[pyo3(text_signature = "($self,)")]
    /// Returns the number of hops for the Sketches.
    fn get_number_of_hops(&self) -> usize {
        self.inner.get_number_of_hops()
    }

    #[args(py_kwargs = "**")]
    #[pyo3(text_signature = "($self,)")]
    /// Returns the precision of the HyperLogLog counters.
    fn get_precision(&self) -> usize {
        self.inner.get_precision()
    }

    #[args(py_kwargs = "**")]
    #[pyo3(text_signature = "($self,)")]
    /// Returns the number of bits of the HyperLogLog counters.
    fn get_bits(&self) -> usize {
        self.inner.get_bits()
    }

    #[pyo3(
        text_signature = "($self, graph, overlap_path, left_difference_path, right_difference_path)"
    )]
    /// Return numpy array with sketches for each edge in the graph.
    ///
    /// Parameters
    /// ----------------
    /// graph: Graph
    ///     The graph whose sketches are to be computed.
    /// overlap_path: Optional[str]
    ///     The path where to store the estimated overlaps.
    /// left_difference_path: Optional[str]
    ///     The path where to store the estimated source differences.
    /// right_difference_path: Optional[str]
    ///     The path where to store the estimated destination differences.
    ///
    /// Raises
    /// ----------------
    /// ValueError
    ///    If the model has not been trained.
    ///
    fn get_sketching_for_all_edges(
        &self,
        graph: &Graph,
        overlap_path: Option<String>,
        left_difference_path: Option<String>,
        right_difference_path: Option<String>,
    ) -> PyResult<(Py<PyAny>, Py<PyAny>, Py<PyAny>)> {
        pe!(self.inner.get_sketching_from_iterator(
            &graph.inner,
            overlap_path.as_deref(),
            left_difference_path.as_deref(),
            right_difference_path.as_deref(),
            graph
                .inner
                .par_iter_directed_edge_node_ids()
                .map(|(_, src, dst)| { (src, dst) }),
        ))
    }

    #[pyo3(
        text_signature = "($self, graph, sources, destinations, overlap_path, left_difference_path, right_difference_path)"
    )]
    /// Return numpy array with sketches for each edge in the graph.
    ///
    /// Parameters
    /// ----------------
    /// graph: Graph
    ///     The graph whose sketches are to be computed.
    /// sources: np.ndarray[NodeT]
    ///     The source nodes.
    /// destinations: np.ndarray[NodeT]
    ///     The destination nodes.
    /// overlap_path: Optional[str]
    ///     The path where to store the estimated overlaps.
    /// left_difference_path: Optional[str]
    ///     The path where to store the estimated source differences.
    /// right_difference_path: Optional[str]
    ///     The path where to store the estimated destination differences.
    ///
    /// Raises
    /// ----------------
    /// ValueError
    ///    If the model has not been trained.
    ///    If the provided sources and destinations do not have the same length.
    ///    If the provided sources are not in the graph.
    ///    If the provided destinations are not in the graph.
    fn get_sketching_from_edge_node_ids(
        &self,
        graph: &Graph,
        sources: Py<PyArray1<NodeT>>,
        destinations: Py<PyArray1<NodeT>>,
        overlap_path: Option<String>,
        left_difference_path: Option<String>,
        right_difference_path: Option<String>,
    ) -> PyResult<(Py<PyAny>, Py<PyAny>, Py<PyAny>)> {
        let gil = pyo3::Python::acquire_gil();
        let sources = sources.as_ref(gil.python());
        let sources_ref = unsafe {
            sources
                .as_slice()
                .map_err(|e| PyValueError::new_err(e.to_string()))?
        };

        let destinations = destinations.as_ref(gil.python());
        let destinations_ref = unsafe {
            destinations
                .as_slice()
                .map_err(|e| PyValueError::new_err(e.to_string()))?
        };

        if sources_ref.len() != destinations_ref.len() {
            return Err(PyValueError::new_err(format!(
                concat!(
                    "The provided sources and destinations do not have the same length. ",
                    "The provided sources have length {} and the provided destinations have length {}."
                ),
                sources_ref.len(),
                destinations_ref.len(),
            )));
        }

        pe!(self.inner.get_sketching_from_iterator(
            &graph.inner,
            overlap_path.as_deref(),
            left_difference_path.as_deref(),
            right_difference_path.as_deref(),
            sources_ref
                .par_iter()
                .copied()
                .zip(destinations_ref.par_iter().copied()),
        ))
    }

    #[staticmethod]
    #[pyo3(text_signature = "(path,)")]
    /// Loads model from the provided path.
    ///
    /// Parameters
    /// ----------------
    /// path: str
    ///     Path from where to load the model.
    fn load(path: String) -> PyResult<Self> {
        Ok(HyperSketching {
            inner: pe!(InnerModel::load(path.as_ref()))?,
        })
    }

    #[staticmethod]
    #[pyo3(text_signature = "(json,)")]
    /// Loads model from provided JSON string.
    ///
    /// Parameters
    /// ----------------
    /// json: str
    ///     JSON string containing model metadata.
    fn loads(json: String) -> PyResult<Self> {
        Ok(HyperSketching {
            inner: pe!(InnerModel::loads(json.as_str()))?,
        })
    }

    #[pyo3(text_signature = "(&self, path)")]
    /// Dump model to the provided path.
    ///
    /// Parameters
    /// ----------------
    /// path: str
    ///     Path where to dump the model.
    fn dump(&self, path: String) -> PyResult<()> {
        pe!(self.inner.dump(path.as_ref()))
    }

    #[pyo3(text_signature = "(&self)")]
    /// Dumps model to JSON string.
    fn dumps(&self) -> PyResult<String> {
        pe!(self.inner.dumps())
    }
}
