use super::*;
use crate::mmap_numpy_npy::create_memory_mapped_numpy_array;
use crate::mmap_numpy_npy::Dtype;
use crate::primitive_f16::PrimitiveF16;
use cpu_models::HyperSketching as HS;
use cpu_models::MatrixShape;
use half::f16;
use hyperloglog_rs::prelude::*;
use num_traits::Float;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;

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
    HS4_4_1(HS<Precision4, 4, 1>), // {python_generated}
    HS4_4_2(HS<Precision4, 4, 2>),     // {python_generated}
    HS4_4_3(HS<Precision4, 4, 3>),     // {python_generated}
    HS4_4_4(HS<Precision4, 4, 4>),     // {python_generated}
    HS4_4_5(HS<Precision4, 4, 5>),     // {python_generated}
    HS4_4_6(HS<Precision4, 4, 6>),     // {python_generated}
    HS4_4_7(HS<Precision4, 4, 7>),     // {python_generated}
    HS4_4_8(HS<Precision4, 4, 8>),     // {python_generated}
    HS4_4_9(HS<Precision4, 4, 9>),     // {python_generated}
    HS4_4_10(HS<Precision4, 4, 10>),   // {python_generated}
    HS4_5_1(HS<Precision4, 5, 1>),     // {python_generated}
    HS4_5_2(HS<Precision4, 5, 2>),     // {python_generated}
    HS4_5_3(HS<Precision4, 5, 3>),     // {python_generated}
    HS4_5_4(HS<Precision4, 5, 4>),     // {python_generated}
    HS4_5_5(HS<Precision4, 5, 5>),     // {python_generated}
    HS4_5_6(HS<Precision4, 5, 6>),     // {python_generated}
    HS4_5_7(HS<Precision4, 5, 7>),     // {python_generated}
    HS4_5_8(HS<Precision4, 5, 8>),     // {python_generated}
    HS4_5_9(HS<Precision4, 5, 9>),     // {python_generated}
    HS4_5_10(HS<Precision4, 5, 10>),   // {python_generated}
    HS4_6_1(HS<Precision4, 6, 1>),     // {python_generated}
    HS4_6_2(HS<Precision4, 6, 2>),     // {python_generated}
    HS4_6_3(HS<Precision4, 6, 3>),     // {python_generated}
    HS4_6_4(HS<Precision4, 6, 4>),     // {python_generated}
    HS4_6_5(HS<Precision4, 6, 5>),     // {python_generated}
    HS4_6_6(HS<Precision4, 6, 6>),     // {python_generated}
    HS4_6_7(HS<Precision4, 6, 7>),     // {python_generated}
    HS4_6_8(HS<Precision4, 6, 8>),     // {python_generated}
    HS4_6_9(HS<Precision4, 6, 9>),     // {python_generated}
    HS4_6_10(HS<Precision4, 6, 10>),   // {python_generated}
    HS5_4_1(HS<Precision5, 4, 1>),     // {python_generated}
    HS5_4_2(HS<Precision5, 4, 2>),     // {python_generated}
    HS5_4_3(HS<Precision5, 4, 3>),     // {python_generated}
    HS5_4_4(HS<Precision5, 4, 4>),     // {python_generated}
    HS5_4_5(HS<Precision5, 4, 5>),     // {python_generated}
    HS5_4_6(HS<Precision5, 4, 6>),     // {python_generated}
    HS5_4_7(HS<Precision5, 4, 7>),     // {python_generated}
    HS5_4_8(HS<Precision5, 4, 8>),     // {python_generated}
    HS5_4_9(HS<Precision5, 4, 9>),     // {python_generated}
    HS5_4_10(HS<Precision5, 4, 10>),   // {python_generated}
    HS5_5_1(HS<Precision5, 5, 1>),     // {python_generated}
    HS5_5_2(HS<Precision5, 5, 2>),     // {python_generated}
    HS5_5_3(HS<Precision5, 5, 3>),     // {python_generated}
    HS5_5_4(HS<Precision5, 5, 4>),     // {python_generated}
    HS5_5_5(HS<Precision5, 5, 5>),     // {python_generated}
    HS5_5_6(HS<Precision5, 5, 6>),     // {python_generated}
    HS5_5_7(HS<Precision5, 5, 7>),     // {python_generated}
    HS5_5_8(HS<Precision5, 5, 8>),     // {python_generated}
    HS5_5_9(HS<Precision5, 5, 9>),     // {python_generated}
    HS5_5_10(HS<Precision5, 5, 10>),   // {python_generated}
    HS5_6_1(HS<Precision5, 6, 1>),     // {python_generated}
    HS5_6_2(HS<Precision5, 6, 2>),     // {python_generated}
    HS5_6_3(HS<Precision5, 6, 3>),     // {python_generated}
    HS5_6_4(HS<Precision5, 6, 4>),     // {python_generated}
    HS5_6_5(HS<Precision5, 6, 5>),     // {python_generated}
    HS5_6_6(HS<Precision5, 6, 6>),     // {python_generated}
    HS5_6_7(HS<Precision5, 6, 7>),     // {python_generated}
    HS5_6_8(HS<Precision5, 6, 8>),     // {python_generated}
    HS5_6_9(HS<Precision5, 6, 9>),     // {python_generated}
    HS5_6_10(HS<Precision5, 6, 10>),   // {python_generated}
    HS6_4_1(HS<Precision6, 4, 1>),     // {python_generated}
    HS6_4_2(HS<Precision6, 4, 2>),     // {python_generated}
    HS6_4_3(HS<Precision6, 4, 3>),     // {python_generated}
    HS6_4_4(HS<Precision6, 4, 4>),     // {python_generated}
    HS6_4_5(HS<Precision6, 4, 5>),     // {python_generated}
    HS6_4_6(HS<Precision6, 4, 6>),     // {python_generated}
    HS6_4_7(HS<Precision6, 4, 7>),     // {python_generated}
    HS6_4_8(HS<Precision6, 4, 8>),     // {python_generated}
    HS6_4_9(HS<Precision6, 4, 9>),     // {python_generated}
    HS6_4_10(HS<Precision6, 4, 10>),   // {python_generated}
    HS6_5_1(HS<Precision6, 5, 1>),     // {python_generated}
    HS6_5_2(HS<Precision6, 5, 2>),     // {python_generated}
    HS6_5_3(HS<Precision6, 5, 3>),     // {python_generated}
    HS6_5_4(HS<Precision6, 5, 4>),     // {python_generated}
    HS6_5_5(HS<Precision6, 5, 5>),     // {python_generated}
    HS6_5_6(HS<Precision6, 5, 6>),     // {python_generated}
    HS6_5_7(HS<Precision6, 5, 7>),     // {python_generated}
    HS6_5_8(HS<Precision6, 5, 8>),     // {python_generated}
    HS6_5_9(HS<Precision6, 5, 9>),     // {python_generated}
    HS6_5_10(HS<Precision6, 5, 10>),   // {python_generated}
    HS6_6_1(HS<Precision6, 6, 1>),     // {python_generated}
    HS6_6_2(HS<Precision6, 6, 2>),     // {python_generated}
    HS6_6_3(HS<Precision6, 6, 3>),     // {python_generated}
    HS6_6_4(HS<Precision6, 6, 4>),     // {python_generated}
    HS6_6_5(HS<Precision6, 6, 5>),     // {python_generated}
    HS6_6_6(HS<Precision6, 6, 6>),     // {python_generated}
    HS6_6_7(HS<Precision6, 6, 7>),     // {python_generated}
    HS6_6_8(HS<Precision6, 6, 8>),     // {python_generated}
    HS6_6_9(HS<Precision6, 6, 9>),     // {python_generated}
    HS6_6_10(HS<Precision6, 6, 10>),   // {python_generated}
    HS7_4_1(HS<Precision7, 4, 1>),     // {python_generated}
    HS7_4_2(HS<Precision7, 4, 2>),     // {python_generated}
    HS7_4_3(HS<Precision7, 4, 3>),     // {python_generated}
    HS7_4_4(HS<Precision7, 4, 4>),     // {python_generated}
    HS7_4_5(HS<Precision7, 4, 5>),     // {python_generated}
    HS7_4_6(HS<Precision7, 4, 6>),     // {python_generated}
    HS7_4_7(HS<Precision7, 4, 7>),     // {python_generated}
    HS7_4_8(HS<Precision7, 4, 8>),     // {python_generated}
    HS7_4_9(HS<Precision7, 4, 9>),     // {python_generated}
    HS7_4_10(HS<Precision7, 4, 10>),   // {python_generated}
    HS7_5_1(HS<Precision7, 5, 1>),     // {python_generated}
    HS7_5_2(HS<Precision7, 5, 2>),     // {python_generated}
    HS7_5_3(HS<Precision7, 5, 3>),     // {python_generated}
    HS7_5_4(HS<Precision7, 5, 4>),     // {python_generated}
    HS7_5_5(HS<Precision7, 5, 5>),     // {python_generated}
    HS7_5_6(HS<Precision7, 5, 6>),     // {python_generated}
    HS7_5_7(HS<Precision7, 5, 7>),     // {python_generated}
    HS7_5_8(HS<Precision7, 5, 8>),     // {python_generated}
    HS7_5_9(HS<Precision7, 5, 9>),     // {python_generated}
    HS7_5_10(HS<Precision7, 5, 10>),   // {python_generated}
    HS7_6_1(HS<Precision7, 6, 1>),     // {python_generated}
    HS7_6_2(HS<Precision7, 6, 2>),     // {python_generated}
    HS7_6_3(HS<Precision7, 6, 3>),     // {python_generated}
    HS7_6_4(HS<Precision7, 6, 4>),     // {python_generated}
    HS7_6_5(HS<Precision7, 6, 5>),     // {python_generated}
    HS7_6_6(HS<Precision7, 6, 6>),     // {python_generated}
    HS7_6_7(HS<Precision7, 6, 7>),     // {python_generated}
    HS7_6_8(HS<Precision7, 6, 8>),     // {python_generated}
    HS7_6_9(HS<Precision7, 6, 9>),     // {python_generated}
    HS7_6_10(HS<Precision7, 6, 10>),   // {python_generated}
    HS8_4_1(HS<Precision8, 4, 1>),     // {python_generated}
    HS8_4_2(HS<Precision8, 4, 2>),     // {python_generated}
    HS8_4_3(HS<Precision8, 4, 3>),     // {python_generated}
    HS8_4_4(HS<Precision8, 4, 4>),     // {python_generated}
    HS8_4_5(HS<Precision8, 4, 5>),     // {python_generated}
    HS8_4_6(HS<Precision8, 4, 6>),     // {python_generated}
    HS8_4_7(HS<Precision8, 4, 7>),     // {python_generated}
    HS8_4_8(HS<Precision8, 4, 8>),     // {python_generated}
    HS8_4_9(HS<Precision8, 4, 9>),     // {python_generated}
    HS8_4_10(HS<Precision8, 4, 10>),   // {python_generated}
    HS8_5_1(HS<Precision8, 5, 1>),     // {python_generated}
    HS8_5_2(HS<Precision8, 5, 2>),     // {python_generated}
    HS8_5_3(HS<Precision8, 5, 3>),     // {python_generated}
    HS8_5_4(HS<Precision8, 5, 4>),     // {python_generated}
    HS8_5_5(HS<Precision8, 5, 5>),     // {python_generated}
    HS8_5_6(HS<Precision8, 5, 6>),     // {python_generated}
    HS8_5_7(HS<Precision8, 5, 7>),     // {python_generated}
    HS8_5_8(HS<Precision8, 5, 8>),     // {python_generated}
    HS8_5_9(HS<Precision8, 5, 9>),     // {python_generated}
    HS8_5_10(HS<Precision8, 5, 10>),   // {python_generated}
    HS8_6_1(HS<Precision8, 6, 1>),     // {python_generated}
    HS8_6_2(HS<Precision8, 6, 2>),     // {python_generated}
    HS8_6_3(HS<Precision8, 6, 3>),     // {python_generated}
    HS8_6_4(HS<Precision8, 6, 4>),     // {python_generated}
    HS8_6_5(HS<Precision8, 6, 5>),     // {python_generated}
    HS8_6_6(HS<Precision8, 6, 6>),     // {python_generated}
    HS8_6_7(HS<Precision8, 6, 7>),     // {python_generated}
    HS8_6_8(HS<Precision8, 6, 8>),     // {python_generated}
    HS8_6_9(HS<Precision8, 6, 9>),     // {python_generated}
    HS8_6_10(HS<Precision8, 6, 10>),   // {python_generated}
    HS9_4_1(HS<Precision9, 4, 1>),     // {python_generated}
    HS9_4_2(HS<Precision9, 4, 2>),     // {python_generated}
    HS9_4_3(HS<Precision9, 4, 3>),     // {python_generated}
    HS9_4_4(HS<Precision9, 4, 4>),     // {python_generated}
    HS9_4_5(HS<Precision9, 4, 5>),     // {python_generated}
    HS9_4_6(HS<Precision9, 4, 6>),     // {python_generated}
    HS9_4_7(HS<Precision9, 4, 7>),     // {python_generated}
    HS9_4_8(HS<Precision9, 4, 8>),     // {python_generated}
    HS9_4_9(HS<Precision9, 4, 9>),     // {python_generated}
    HS9_4_10(HS<Precision9, 4, 10>),   // {python_generated}
    HS9_5_1(HS<Precision9, 5, 1>),     // {python_generated}
    HS9_5_2(HS<Precision9, 5, 2>),     // {python_generated}
    HS9_5_3(HS<Precision9, 5, 3>),     // {python_generated}
    HS9_5_4(HS<Precision9, 5, 4>),     // {python_generated}
    HS9_5_5(HS<Precision9, 5, 5>),     // {python_generated}
    HS9_5_6(HS<Precision9, 5, 6>),     // {python_generated}
    HS9_5_7(HS<Precision9, 5, 7>),     // {python_generated}
    HS9_5_8(HS<Precision9, 5, 8>),     // {python_generated}
    HS9_5_9(HS<Precision9, 5, 9>),     // {python_generated}
    HS9_5_10(HS<Precision9, 5, 10>),   // {python_generated}
    HS9_6_1(HS<Precision9, 6, 1>),     // {python_generated}
    HS9_6_2(HS<Precision9, 6, 2>),     // {python_generated}
    HS9_6_3(HS<Precision9, 6, 3>),     // {python_generated}
    HS9_6_4(HS<Precision9, 6, 4>),     // {python_generated}
    HS9_6_5(HS<Precision9, 6, 5>),     // {python_generated}
    HS9_6_6(HS<Precision9, 6, 6>),     // {python_generated}
    HS9_6_7(HS<Precision9, 6, 7>),     // {python_generated}
    HS9_6_8(HS<Precision9, 6, 8>),     // {python_generated}
    HS9_6_9(HS<Precision9, 6, 9>),     // {python_generated}
    HS9_6_10(HS<Precision9, 6, 10>),   // {python_generated}
    HS10_4_1(HS<Precision10, 4, 1>),   // {python_generated}
    HS10_4_2(HS<Precision10, 4, 2>),   // {python_generated}
    HS10_4_3(HS<Precision10, 4, 3>),   // {python_generated}
    HS10_4_4(HS<Precision10, 4, 4>),   // {python_generated}
    HS10_4_5(HS<Precision10, 4, 5>),   // {python_generated}
    HS10_4_6(HS<Precision10, 4, 6>),   // {python_generated}
    HS10_4_7(HS<Precision10, 4, 7>),   // {python_generated}
    HS10_4_8(HS<Precision10, 4, 8>),   // {python_generated}
    HS10_4_9(HS<Precision10, 4, 9>),   // {python_generated}
    HS10_4_10(HS<Precision10, 4, 10>), // {python_generated}
    HS10_5_1(HS<Precision10, 5, 1>),   // {python_generated}
    HS10_5_2(HS<Precision10, 5, 2>),   // {python_generated}
    HS10_5_3(HS<Precision10, 5, 3>),   // {python_generated}
    HS10_5_4(HS<Precision10, 5, 4>),   // {python_generated}
    HS10_5_5(HS<Precision10, 5, 5>),   // {python_generated}
    HS10_5_6(HS<Precision10, 5, 6>),   // {python_generated}
    HS10_5_7(HS<Precision10, 5, 7>),   // {python_generated}
    HS10_5_8(HS<Precision10, 5, 8>),   // {python_generated}
    HS10_5_9(HS<Precision10, 5, 9>),   // {python_generated}
    HS10_5_10(HS<Precision10, 5, 10>), // {python_generated}
    HS10_6_1(HS<Precision10, 6, 1>),   // {python_generated}
    HS10_6_2(HS<Precision10, 6, 2>),   // {python_generated}
    HS10_6_3(HS<Precision10, 6, 3>),   // {python_generated}
    HS10_6_4(HS<Precision10, 6, 4>),   // {python_generated}
    HS10_6_5(HS<Precision10, 6, 5>),   // {python_generated}
    HS10_6_6(HS<Precision10, 6, 6>),   // {python_generated}
    HS10_6_7(HS<Precision10, 6, 7>),   // {python_generated}
    HS10_6_8(HS<Precision10, 6, 8>),   // {python_generated}
    HS10_6_9(HS<Precision10, 6, 9>),   // {python_generated}
    HS10_6_10(HS<Precision10, 6, 10>), // {python_generated}
    HS11_4_1(HS<Precision11, 4, 1>),   // {python_generated}
    HS11_4_2(HS<Precision11, 4, 2>),   // {python_generated}
    HS11_4_3(HS<Precision11, 4, 3>),   // {python_generated}
    HS11_4_4(HS<Precision11, 4, 4>),   // {python_generated}
    HS11_4_5(HS<Precision11, 4, 5>),   // {python_generated}
    HS11_4_6(HS<Precision11, 4, 6>),   // {python_generated}
    HS11_4_7(HS<Precision11, 4, 7>),   // {python_generated}
    HS11_4_8(HS<Precision11, 4, 8>),   // {python_generated}
    HS11_4_9(HS<Precision11, 4, 9>),   // {python_generated}
    HS11_4_10(HS<Precision11, 4, 10>), // {python_generated}
    HS11_5_1(HS<Precision11, 5, 1>),   // {python_generated}
    HS11_5_2(HS<Precision11, 5, 2>),   // {python_generated}
    HS11_5_3(HS<Precision11, 5, 3>),   // {python_generated}
    HS11_5_4(HS<Precision11, 5, 4>),   // {python_generated}
    HS11_5_5(HS<Precision11, 5, 5>),   // {python_generated}
    HS11_5_6(HS<Precision11, 5, 6>),   // {python_generated}
    HS11_5_7(HS<Precision11, 5, 7>),   // {python_generated}
    HS11_5_8(HS<Precision11, 5, 8>),   // {python_generated}
    HS11_5_9(HS<Precision11, 5, 9>),   // {python_generated}
    HS11_5_10(HS<Precision11, 5, 10>), // {python_generated}
    HS11_6_1(HS<Precision11, 6, 1>),   // {python_generated}
    HS11_6_2(HS<Precision11, 6, 2>),   // {python_generated}
    HS11_6_3(HS<Precision11, 6, 3>),   // {python_generated}
    HS11_6_4(HS<Precision11, 6, 4>),   // {python_generated}
    HS11_6_5(HS<Precision11, 6, 5>),   // {python_generated}
    HS11_6_6(HS<Precision11, 6, 6>),   // {python_generated}
    HS11_6_7(HS<Precision11, 6, 7>),   // {python_generated}
    HS11_6_8(HS<Precision11, 6, 8>),   // {python_generated}
    HS11_6_9(HS<Precision11, 6, 9>),   // {python_generated}
    HS11_6_10(HS<Precision11, 6, 10>), // {python_generated}
    HS12_4_1(HS<Precision12, 4, 1>),   // {python_generated}
    HS12_4_2(HS<Precision12, 4, 2>),   // {python_generated}
    HS12_4_3(HS<Precision12, 4, 3>),   // {python_generated}
    HS12_4_4(HS<Precision12, 4, 4>),   // {python_generated}
    HS12_4_5(HS<Precision12, 4, 5>),   // {python_generated}
    HS12_4_6(HS<Precision12, 4, 6>),   // {python_generated}
    HS12_4_7(HS<Precision12, 4, 7>),   // {python_generated}
    HS12_4_8(HS<Precision12, 4, 8>),   // {python_generated}
    HS12_4_9(HS<Precision12, 4, 9>),   // {python_generated}
    HS12_4_10(HS<Precision12, 4, 10>), // {python_generated}
    HS12_5_1(HS<Precision12, 5, 1>),   // {python_generated}
    HS12_5_2(HS<Precision12, 5, 2>),   // {python_generated}
    HS12_5_3(HS<Precision12, 5, 3>),   // {python_generated}
    HS12_5_4(HS<Precision12, 5, 4>),   // {python_generated}
    HS12_5_5(HS<Precision12, 5, 5>),   // {python_generated}
    HS12_5_6(HS<Precision12, 5, 6>),   // {python_generated}
    HS12_5_7(HS<Precision12, 5, 7>),   // {python_generated}
    HS12_5_8(HS<Precision12, 5, 8>),   // {python_generated}
    HS12_5_9(HS<Precision12, 5, 9>),   // {python_generated}
    HS12_5_10(HS<Precision12, 5, 10>), // {python_generated}
    HS12_6_1(HS<Precision12, 6, 1>),   // {python_generated}
    HS12_6_2(HS<Precision12, 6, 2>),   // {python_generated}
    HS12_6_3(HS<Precision12, 6, 3>),   // {python_generated}
    HS12_6_4(HS<Precision12, 6, 4>),   // {python_generated}
    HS12_6_5(HS<Precision12, 6, 5>),   // {python_generated}
    HS12_6_6(HS<Precision12, 6, 6>),   // {python_generated}
    HS12_6_7(HS<Precision12, 6, 7>),   // {python_generated}
    HS12_6_8(HS<Precision12, 6, 8>),   // {python_generated}
    HS12_6_9(HS<Precision12, 6, 9>),   // {python_generated}
    HS12_6_10(HS<Precision12, 6, 10>), // {python_generated}
    HS13_4_1(HS<Precision13, 4, 1>),   // {python_generated}
    HS13_4_2(HS<Precision13, 4, 2>),   // {python_generated}
    HS13_4_3(HS<Precision13, 4, 3>),   // {python_generated}
    HS13_4_4(HS<Precision13, 4, 4>),   // {python_generated}
    HS13_4_5(HS<Precision13, 4, 5>),   // {python_generated}
    HS13_4_6(HS<Precision13, 4, 6>),   // {python_generated}
    HS13_4_7(HS<Precision13, 4, 7>),   // {python_generated}
    HS13_4_8(HS<Precision13, 4, 8>),   // {python_generated}
    HS13_4_9(HS<Precision13, 4, 9>),   // {python_generated}
    HS13_4_10(HS<Precision13, 4, 10>), // {python_generated}
    HS13_5_1(HS<Precision13, 5, 1>),   // {python_generated}
    HS13_5_2(HS<Precision13, 5, 2>),   // {python_generated}
    HS13_5_3(HS<Precision13, 5, 3>),   // {python_generated}
    HS13_5_4(HS<Precision13, 5, 4>),   // {python_generated}
    HS13_5_5(HS<Precision13, 5, 5>),   // {python_generated}
    HS13_5_6(HS<Precision13, 5, 6>),   // {python_generated}
    HS13_5_7(HS<Precision13, 5, 7>),   // {python_generated}
    HS13_5_8(HS<Precision13, 5, 8>),   // {python_generated}
    HS13_5_9(HS<Precision13, 5, 9>),   // {python_generated}
    HS13_5_10(HS<Precision13, 5, 10>), // {python_generated}
    HS13_6_1(HS<Precision13, 6, 1>),   // {python_generated}
    HS13_6_2(HS<Precision13, 6, 2>),   // {python_generated}
    HS13_6_3(HS<Precision13, 6, 3>),   // {python_generated}
    HS13_6_4(HS<Precision13, 6, 4>),   // {python_generated}
    HS13_6_5(HS<Precision13, 6, 5>),   // {python_generated}
    HS13_6_6(HS<Precision13, 6, 6>),   // {python_generated}
    HS13_6_7(HS<Precision13, 6, 7>),   // {python_generated}
    HS13_6_8(HS<Precision13, 6, 8>),   // {python_generated}
    HS13_6_9(HS<Precision13, 6, 9>),   // {python_generated}
    HS13_6_10(HS<Precision13, 6, 10>), // {python_generated}
    HS14_4_1(HS<Precision14, 4, 1>),   // {python_generated}
    HS14_4_2(HS<Precision14, 4, 2>),   // {python_generated}
    HS14_4_3(HS<Precision14, 4, 3>),   // {python_generated}
    HS14_4_4(HS<Precision14, 4, 4>),   // {python_generated}
    HS14_4_5(HS<Precision14, 4, 5>),   // {python_generated}
    HS14_4_6(HS<Precision14, 4, 6>),   // {python_generated}
    HS14_4_7(HS<Precision14, 4, 7>),   // {python_generated}
    HS14_4_8(HS<Precision14, 4, 8>),   // {python_generated}
    HS14_4_9(HS<Precision14, 4, 9>),   // {python_generated}
    HS14_4_10(HS<Precision14, 4, 10>), // {python_generated}
    HS14_5_1(HS<Precision14, 5, 1>),   // {python_generated}
    HS14_5_2(HS<Precision14, 5, 2>),   // {python_generated}
    HS14_5_3(HS<Precision14, 5, 3>),   // {python_generated}
    HS14_5_4(HS<Precision14, 5, 4>),   // {python_generated}
    HS14_5_5(HS<Precision14, 5, 5>),   // {python_generated}
    HS14_5_6(HS<Precision14, 5, 6>),   // {python_generated}
    HS14_5_7(HS<Precision14, 5, 7>),   // {python_generated}
    HS14_5_8(HS<Precision14, 5, 8>),   // {python_generated}
    HS14_5_9(HS<Precision14, 5, 9>),   // {python_generated}
    HS14_5_10(HS<Precision14, 5, 10>), // {python_generated}
    HS14_6_1(HS<Precision14, 6, 1>),   // {python_generated}
    HS14_6_2(HS<Precision14, 6, 2>),   // {python_generated}
    HS14_6_3(HS<Precision14, 6, 3>),   // {python_generated}
    HS14_6_4(HS<Precision14, 6, 4>),   // {python_generated}
    HS14_6_5(HS<Precision14, 6, 5>),   // {python_generated}
    HS14_6_6(HS<Precision14, 6, 6>),   // {python_generated}
    HS14_6_7(HS<Precision14, 6, 7>),   // {python_generated}
    HS14_6_8(HS<Precision14, 6, 8>),   // {python_generated}
    HS14_6_9(HS<Precision14, 6, 9>),   // {python_generated}
    HS14_6_10(HS<Precision14, 6, 10>), // {python_generated}
    HS15_4_1(HS<Precision15, 4, 1>),   // {python_generated}
    HS15_4_2(HS<Precision15, 4, 2>),   // {python_generated}
    HS15_4_3(HS<Precision15, 4, 3>),   // {python_generated}
    HS15_4_4(HS<Precision15, 4, 4>),   // {python_generated}
    HS15_4_5(HS<Precision15, 4, 5>),   // {python_generated}
    HS15_4_6(HS<Precision15, 4, 6>),   // {python_generated}
    HS15_4_7(HS<Precision15, 4, 7>),   // {python_generated}
    HS15_4_8(HS<Precision15, 4, 8>),   // {python_generated}
    HS15_4_9(HS<Precision15, 4, 9>),   // {python_generated}
    HS15_4_10(HS<Precision15, 4, 10>), // {python_generated}
    HS15_5_1(HS<Precision15, 5, 1>),   // {python_generated}
    HS15_5_2(HS<Precision15, 5, 2>),   // {python_generated}
    HS15_5_3(HS<Precision15, 5, 3>),   // {python_generated}
    HS15_5_4(HS<Precision15, 5, 4>),   // {python_generated}
    HS15_5_5(HS<Precision15, 5, 5>),   // {python_generated}
    HS15_5_6(HS<Precision15, 5, 6>),   // {python_generated}
    HS15_5_7(HS<Precision15, 5, 7>),   // {python_generated}
    HS15_5_8(HS<Precision15, 5, 8>),   // {python_generated}
    HS15_5_9(HS<Precision15, 5, 9>),   // {python_generated}
    HS15_5_10(HS<Precision15, 5, 10>), // {python_generated}
    HS15_6_1(HS<Precision15, 6, 1>),   // {python_generated}
    HS15_6_2(HS<Precision15, 6, 2>),   // {python_generated}
    HS15_6_3(HS<Precision15, 6, 3>),   // {python_generated}
    HS15_6_4(HS<Precision15, 6, 4>),   // {python_generated}
    HS15_6_5(HS<Precision15, 6, 5>),   // {python_generated}
    HS15_6_6(HS<Precision15, 6, 6>),   // {python_generated}
    HS15_6_7(HS<Precision15, 6, 7>),   // {python_generated}
    HS15_6_8(HS<Precision15, 6, 8>),   // {python_generated}
    HS15_6_9(HS<Precision15, 6, 9>),   // {python_generated}
    HS15_6_10(HS<Precision15, 6, 10>), // {python_generated}
    HS16_4_1(HS<Precision16, 4, 1>),   // {python_generated}
    HS16_4_2(HS<Precision16, 4, 2>),   // {python_generated}
    HS16_4_3(HS<Precision16, 4, 3>),   // {python_generated}
    HS16_4_4(HS<Precision16, 4, 4>),   // {python_generated}
    HS16_4_5(HS<Precision16, 4, 5>),   // {python_generated}
    HS16_4_6(HS<Precision16, 4, 6>),   // {python_generated}
    HS16_4_7(HS<Precision16, 4, 7>),   // {python_generated}
    HS16_4_8(HS<Precision16, 4, 8>),   // {python_generated}
    HS16_4_9(HS<Precision16, 4, 9>),   // {python_generated}
    HS16_4_10(HS<Precision16, 4, 10>), // {python_generated}
    HS16_5_1(HS<Precision16, 5, 1>),   // {python_generated}
    HS16_5_2(HS<Precision16, 5, 2>),   // {python_generated}
    HS16_5_3(HS<Precision16, 5, 3>),   // {python_generated}
    HS16_5_4(HS<Precision16, 5, 4>),   // {python_generated}
    HS16_5_5(HS<Precision16, 5, 5>),   // {python_generated}
    HS16_5_6(HS<Precision16, 5, 6>),   // {python_generated}
    HS16_5_7(HS<Precision16, 5, 7>),   // {python_generated}
    HS16_5_8(HS<Precision16, 5, 8>),   // {python_generated}
    HS16_5_9(HS<Precision16, 5, 9>),   // {python_generated}
    HS16_5_10(HS<Precision16, 5, 10>), // {python_generated}
    HS16_6_1(HS<Precision16, 6, 1>),   // {python_generated}
    HS16_6_2(HS<Precision16, 6, 2>),   // {python_generated}
    HS16_6_3(HS<Precision16, 6, 3>),   // {python_generated}
    HS16_6_4(HS<Precision16, 6, 4>),   // {python_generated}
    HS16_6_5(HS<Precision16, 6, 5>),   // {python_generated}
    HS16_6_6(HS<Precision16, 6, 6>),   // {python_generated}
    HS16_6_7(HS<Precision16, 6, 7>),   // {python_generated}
    HS16_6_8(HS<Precision16, 6, 8>),   // {python_generated}
    HS16_6_9(HS<Precision16, 6, 9>),   // {python_generated}
    HS16_6_10(HS<Precision16, 6, 10>), // {python_generated}
}

impl InnerModel {
    /// Return a new instance of the HyperSketching model.
    ///
    /// Parameters
    /// ------------------------
    /// unbiased: Option<bool>
    ///     Whether to use the unbiased algorithm for the sketching,
    ///     which is slower but does not leak the information of whether
    ///     an edge is present or not to the sketch.
    ///     By default, `false`.
    /// exact: Option<bool>
    ///     Whether to use the exact algorithm for the sketching,
    ///     which is slower and requires more memory but is more accurate
    ///     as uses Sets instead of HyperLogLog counters.
    ///     By default, `false`.
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
    ///     By default, `true`.
    /// include_selfloops: Option<bool>
    ///     Whether to include the selfloops in the sketches.
    ///     By default, `false`.
    /// include_typed_graphlets: Option<bool>
    ///     Whether to include the typed graphlets in the sketches.
    ///     By default, `false`.
    /// random_state: Option<u64>
    ///    The random state to be employed.
    /// number_of_random_integers: Option<usize>
    ///    The number of random integers to be employed.
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
        unbiased: Option<bool>,
        exact: Option<bool>,
        number_of_hops: Option<usize>,
        precision: usize,
        bits: usize,
        include_node_types: Option<bool>,
        include_edge_types: Option<bool>,
        include_edge_ids: Option<bool>,
        include_node_ids: Option<bool>,
        include_selfloops: Option<bool>,
        include_typed_graphlets: Option<bool>,
        random_state: Option<u64>,
        number_of_random_integers: Option<usize>,
        normalize_by_symmetric_laplacian: Option<bool>,
        concatenate_features: Option<bool>,
        dtype: Option<String>,
    ) -> Result<Self> {
        // Since actually writing the code for the following match would make
        // for very hard to read code, we proceed instead with a Python script.

        match (precision, bits, number_of_hops.unwrap_or(2)) {
            // ({precision}, {bits}, {hops}) => Ok(InnerModel::HS{precision}_{bits}_{hops}(HS::new(unbiased, exact, include_node_types, include_edge_types, include_edge_ids, include_node_ids, include_selfloops, include_typed_graphlets, random_state, number_of_random_integers, normalize_by_symmetric_laplacian, concatenate_features, dtype)?)), {python_macro}
            (4, 4, 1) => Ok(InnerModel::HS4_4_1(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (4, 4, 2) => Ok(InnerModel::HS4_4_2(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (4, 4, 3) => Ok(InnerModel::HS4_4_3(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (4, 4, 4) => Ok(InnerModel::HS4_4_4(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (4, 4, 5) => Ok(InnerModel::HS4_4_5(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (4, 4, 6) => Ok(InnerModel::HS4_4_6(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (4, 4, 7) => Ok(InnerModel::HS4_4_7(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (4, 4, 8) => Ok(InnerModel::HS4_4_8(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (4, 4, 9) => Ok(InnerModel::HS4_4_9(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (4, 4, 10) => Ok(InnerModel::HS4_4_10(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (4, 5, 1) => Ok(InnerModel::HS4_5_1(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (4, 5, 2) => Ok(InnerModel::HS4_5_2(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (4, 5, 3) => Ok(InnerModel::HS4_5_3(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (4, 5, 4) => Ok(InnerModel::HS4_5_4(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (4, 5, 5) => Ok(InnerModel::HS4_5_5(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (4, 5, 6) => Ok(InnerModel::HS4_5_6(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (4, 5, 7) => Ok(InnerModel::HS4_5_7(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (4, 5, 8) => Ok(InnerModel::HS4_5_8(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (4, 5, 9) => Ok(InnerModel::HS4_5_9(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (4, 5, 10) => Ok(InnerModel::HS4_5_10(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (4, 6, 1) => Ok(InnerModel::HS4_6_1(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (4, 6, 2) => Ok(InnerModel::HS4_6_2(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (4, 6, 3) => Ok(InnerModel::HS4_6_3(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (4, 6, 4) => Ok(InnerModel::HS4_6_4(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (4, 6, 5) => Ok(InnerModel::HS4_6_5(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (4, 6, 6) => Ok(InnerModel::HS4_6_6(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (4, 6, 7) => Ok(InnerModel::HS4_6_7(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (4, 6, 8) => Ok(InnerModel::HS4_6_8(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (4, 6, 9) => Ok(InnerModel::HS4_6_9(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (4, 6, 10) => Ok(InnerModel::HS4_6_10(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (5, 4, 1) => Ok(InnerModel::HS5_4_1(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (5, 4, 2) => Ok(InnerModel::HS5_4_2(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (5, 4, 3) => Ok(InnerModel::HS5_4_3(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (5, 4, 4) => Ok(InnerModel::HS5_4_4(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (5, 4, 5) => Ok(InnerModel::HS5_4_5(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (5, 4, 6) => Ok(InnerModel::HS5_4_6(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (5, 4, 7) => Ok(InnerModel::HS5_4_7(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (5, 4, 8) => Ok(InnerModel::HS5_4_8(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (5, 4, 9) => Ok(InnerModel::HS5_4_9(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (5, 4, 10) => Ok(InnerModel::HS5_4_10(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (5, 5, 1) => Ok(InnerModel::HS5_5_1(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (5, 5, 2) => Ok(InnerModel::HS5_5_2(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (5, 5, 3) => Ok(InnerModel::HS5_5_3(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (5, 5, 4) => Ok(InnerModel::HS5_5_4(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (5, 5, 5) => Ok(InnerModel::HS5_5_5(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (5, 5, 6) => Ok(InnerModel::HS5_5_6(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (5, 5, 7) => Ok(InnerModel::HS5_5_7(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (5, 5, 8) => Ok(InnerModel::HS5_5_8(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (5, 5, 9) => Ok(InnerModel::HS5_5_9(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (5, 5, 10) => Ok(InnerModel::HS5_5_10(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (5, 6, 1) => Ok(InnerModel::HS5_6_1(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (5, 6, 2) => Ok(InnerModel::HS5_6_2(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (5, 6, 3) => Ok(InnerModel::HS5_6_3(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (5, 6, 4) => Ok(InnerModel::HS5_6_4(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (5, 6, 5) => Ok(InnerModel::HS5_6_5(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (5, 6, 6) => Ok(InnerModel::HS5_6_6(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (5, 6, 7) => Ok(InnerModel::HS5_6_7(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (5, 6, 8) => Ok(InnerModel::HS5_6_8(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (5, 6, 9) => Ok(InnerModel::HS5_6_9(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (5, 6, 10) => Ok(InnerModel::HS5_6_10(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (6, 4, 1) => Ok(InnerModel::HS6_4_1(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (6, 4, 2) => Ok(InnerModel::HS6_4_2(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (6, 4, 3) => Ok(InnerModel::HS6_4_3(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (6, 4, 4) => Ok(InnerModel::HS6_4_4(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (6, 4, 5) => Ok(InnerModel::HS6_4_5(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (6, 4, 6) => Ok(InnerModel::HS6_4_6(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (6, 4, 7) => Ok(InnerModel::HS6_4_7(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (6, 4, 8) => Ok(InnerModel::HS6_4_8(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (6, 4, 9) => Ok(InnerModel::HS6_4_9(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (6, 4, 10) => Ok(InnerModel::HS6_4_10(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (6, 5, 1) => Ok(InnerModel::HS6_5_1(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (6, 5, 2) => Ok(InnerModel::HS6_5_2(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (6, 5, 3) => Ok(InnerModel::HS6_5_3(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (6, 5, 4) => Ok(InnerModel::HS6_5_4(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (6, 5, 5) => Ok(InnerModel::HS6_5_5(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (6, 5, 6) => Ok(InnerModel::HS6_5_6(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (6, 5, 7) => Ok(InnerModel::HS6_5_7(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (6, 5, 8) => Ok(InnerModel::HS6_5_8(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (6, 5, 9) => Ok(InnerModel::HS6_5_9(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (6, 5, 10) => Ok(InnerModel::HS6_5_10(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (6, 6, 1) => Ok(InnerModel::HS6_6_1(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (6, 6, 2) => Ok(InnerModel::HS6_6_2(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (6, 6, 3) => Ok(InnerModel::HS6_6_3(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (6, 6, 4) => Ok(InnerModel::HS6_6_4(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (6, 6, 5) => Ok(InnerModel::HS6_6_5(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (6, 6, 6) => Ok(InnerModel::HS6_6_6(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (6, 6, 7) => Ok(InnerModel::HS6_6_7(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (6, 6, 8) => Ok(InnerModel::HS6_6_8(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (6, 6, 9) => Ok(InnerModel::HS6_6_9(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (6, 6, 10) => Ok(InnerModel::HS6_6_10(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (7, 4, 1) => Ok(InnerModel::HS7_4_1(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (7, 4, 2) => Ok(InnerModel::HS7_4_2(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (7, 4, 3) => Ok(InnerModel::HS7_4_3(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (7, 4, 4) => Ok(InnerModel::HS7_4_4(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (7, 4, 5) => Ok(InnerModel::HS7_4_5(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (7, 4, 6) => Ok(InnerModel::HS7_4_6(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (7, 4, 7) => Ok(InnerModel::HS7_4_7(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (7, 4, 8) => Ok(InnerModel::HS7_4_8(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (7, 4, 9) => Ok(InnerModel::HS7_4_9(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (7, 4, 10) => Ok(InnerModel::HS7_4_10(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (7, 5, 1) => Ok(InnerModel::HS7_5_1(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (7, 5, 2) => Ok(InnerModel::HS7_5_2(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (7, 5, 3) => Ok(InnerModel::HS7_5_3(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (7, 5, 4) => Ok(InnerModel::HS7_5_4(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (7, 5, 5) => Ok(InnerModel::HS7_5_5(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (7, 5, 6) => Ok(InnerModel::HS7_5_6(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (7, 5, 7) => Ok(InnerModel::HS7_5_7(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (7, 5, 8) => Ok(InnerModel::HS7_5_8(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (7, 5, 9) => Ok(InnerModel::HS7_5_9(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (7, 5, 10) => Ok(InnerModel::HS7_5_10(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (7, 6, 1) => Ok(InnerModel::HS7_6_1(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (7, 6, 2) => Ok(InnerModel::HS7_6_2(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (7, 6, 3) => Ok(InnerModel::HS7_6_3(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (7, 6, 4) => Ok(InnerModel::HS7_6_4(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (7, 6, 5) => Ok(InnerModel::HS7_6_5(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (7, 6, 6) => Ok(InnerModel::HS7_6_6(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (7, 6, 7) => Ok(InnerModel::HS7_6_7(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (7, 6, 8) => Ok(InnerModel::HS7_6_8(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (7, 6, 9) => Ok(InnerModel::HS7_6_9(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (7, 6, 10) => Ok(InnerModel::HS7_6_10(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (8, 4, 1) => Ok(InnerModel::HS8_4_1(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (8, 4, 2) => Ok(InnerModel::HS8_4_2(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (8, 4, 3) => Ok(InnerModel::HS8_4_3(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (8, 4, 4) => Ok(InnerModel::HS8_4_4(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (8, 4, 5) => Ok(InnerModel::HS8_4_5(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (8, 4, 6) => Ok(InnerModel::HS8_4_6(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (8, 4, 7) => Ok(InnerModel::HS8_4_7(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (8, 4, 8) => Ok(InnerModel::HS8_4_8(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (8, 4, 9) => Ok(InnerModel::HS8_4_9(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (8, 4, 10) => Ok(InnerModel::HS8_4_10(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (8, 5, 1) => Ok(InnerModel::HS8_5_1(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (8, 5, 2) => Ok(InnerModel::HS8_5_2(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (8, 5, 3) => Ok(InnerModel::HS8_5_3(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (8, 5, 4) => Ok(InnerModel::HS8_5_4(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (8, 5, 5) => Ok(InnerModel::HS8_5_5(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (8, 5, 6) => Ok(InnerModel::HS8_5_6(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (8, 5, 7) => Ok(InnerModel::HS8_5_7(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (8, 5, 8) => Ok(InnerModel::HS8_5_8(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (8, 5, 9) => Ok(InnerModel::HS8_5_9(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (8, 5, 10) => Ok(InnerModel::HS8_5_10(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (8, 6, 1) => Ok(InnerModel::HS8_6_1(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (8, 6, 2) => Ok(InnerModel::HS8_6_2(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (8, 6, 3) => Ok(InnerModel::HS8_6_3(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (8, 6, 4) => Ok(InnerModel::HS8_6_4(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (8, 6, 5) => Ok(InnerModel::HS8_6_5(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (8, 6, 6) => Ok(InnerModel::HS8_6_6(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (8, 6, 7) => Ok(InnerModel::HS8_6_7(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (8, 6, 8) => Ok(InnerModel::HS8_6_8(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (8, 6, 9) => Ok(InnerModel::HS8_6_9(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (8, 6, 10) => Ok(InnerModel::HS8_6_10(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (9, 4, 1) => Ok(InnerModel::HS9_4_1(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (9, 4, 2) => Ok(InnerModel::HS9_4_2(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (9, 4, 3) => Ok(InnerModel::HS9_4_3(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (9, 4, 4) => Ok(InnerModel::HS9_4_4(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (9, 4, 5) => Ok(InnerModel::HS9_4_5(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (9, 4, 6) => Ok(InnerModel::HS9_4_6(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (9, 4, 7) => Ok(InnerModel::HS9_4_7(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (9, 4, 8) => Ok(InnerModel::HS9_4_8(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (9, 4, 9) => Ok(InnerModel::HS9_4_9(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (9, 4, 10) => Ok(InnerModel::HS9_4_10(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (9, 5, 1) => Ok(InnerModel::HS9_5_1(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (9, 5, 2) => Ok(InnerModel::HS9_5_2(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (9, 5, 3) => Ok(InnerModel::HS9_5_3(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (9, 5, 4) => Ok(InnerModel::HS9_5_4(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (9, 5, 5) => Ok(InnerModel::HS9_5_5(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (9, 5, 6) => Ok(InnerModel::HS9_5_6(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (9, 5, 7) => Ok(InnerModel::HS9_5_7(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (9, 5, 8) => Ok(InnerModel::HS9_5_8(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (9, 5, 9) => Ok(InnerModel::HS9_5_9(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (9, 5, 10) => Ok(InnerModel::HS9_5_10(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (9, 6, 1) => Ok(InnerModel::HS9_6_1(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (9, 6, 2) => Ok(InnerModel::HS9_6_2(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (9, 6, 3) => Ok(InnerModel::HS9_6_3(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (9, 6, 4) => Ok(InnerModel::HS9_6_4(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (9, 6, 5) => Ok(InnerModel::HS9_6_5(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (9, 6, 6) => Ok(InnerModel::HS9_6_6(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (9, 6, 7) => Ok(InnerModel::HS9_6_7(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (9, 6, 8) => Ok(InnerModel::HS9_6_8(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (9, 6, 9) => Ok(InnerModel::HS9_6_9(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (9, 6, 10) => Ok(InnerModel::HS9_6_10(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (10, 4, 1) => Ok(InnerModel::HS10_4_1(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (10, 4, 2) => Ok(InnerModel::HS10_4_2(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (10, 4, 3) => Ok(InnerModel::HS10_4_3(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (10, 4, 4) => Ok(InnerModel::HS10_4_4(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (10, 4, 5) => Ok(InnerModel::HS10_4_5(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (10, 4, 6) => Ok(InnerModel::HS10_4_6(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (10, 4, 7) => Ok(InnerModel::HS10_4_7(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (10, 4, 8) => Ok(InnerModel::HS10_4_8(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (10, 4, 9) => Ok(InnerModel::HS10_4_9(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (10, 4, 10) => Ok(InnerModel::HS10_4_10(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (10, 5, 1) => Ok(InnerModel::HS10_5_1(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (10, 5, 2) => Ok(InnerModel::HS10_5_2(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (10, 5, 3) => Ok(InnerModel::HS10_5_3(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (10, 5, 4) => Ok(InnerModel::HS10_5_4(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (10, 5, 5) => Ok(InnerModel::HS10_5_5(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (10, 5, 6) => Ok(InnerModel::HS10_5_6(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (10, 5, 7) => Ok(InnerModel::HS10_5_7(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (10, 5, 8) => Ok(InnerModel::HS10_5_8(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (10, 5, 9) => Ok(InnerModel::HS10_5_9(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (10, 5, 10) => Ok(InnerModel::HS10_5_10(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (10, 6, 1) => Ok(InnerModel::HS10_6_1(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (10, 6, 2) => Ok(InnerModel::HS10_6_2(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (10, 6, 3) => Ok(InnerModel::HS10_6_3(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (10, 6, 4) => Ok(InnerModel::HS10_6_4(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (10, 6, 5) => Ok(InnerModel::HS10_6_5(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (10, 6, 6) => Ok(InnerModel::HS10_6_6(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (10, 6, 7) => Ok(InnerModel::HS10_6_7(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (10, 6, 8) => Ok(InnerModel::HS10_6_8(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (10, 6, 9) => Ok(InnerModel::HS10_6_9(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (10, 6, 10) => Ok(InnerModel::HS10_6_10(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (11, 4, 1) => Ok(InnerModel::HS11_4_1(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (11, 4, 2) => Ok(InnerModel::HS11_4_2(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (11, 4, 3) => Ok(InnerModel::HS11_4_3(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (11, 4, 4) => Ok(InnerModel::HS11_4_4(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (11, 4, 5) => Ok(InnerModel::HS11_4_5(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (11, 4, 6) => Ok(InnerModel::HS11_4_6(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (11, 4, 7) => Ok(InnerModel::HS11_4_7(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (11, 4, 8) => Ok(InnerModel::HS11_4_8(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (11, 4, 9) => Ok(InnerModel::HS11_4_9(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (11, 4, 10) => Ok(InnerModel::HS11_4_10(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (11, 5, 1) => Ok(InnerModel::HS11_5_1(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (11, 5, 2) => Ok(InnerModel::HS11_5_2(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (11, 5, 3) => Ok(InnerModel::HS11_5_3(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (11, 5, 4) => Ok(InnerModel::HS11_5_4(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (11, 5, 5) => Ok(InnerModel::HS11_5_5(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (11, 5, 6) => Ok(InnerModel::HS11_5_6(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (11, 5, 7) => Ok(InnerModel::HS11_5_7(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (11, 5, 8) => Ok(InnerModel::HS11_5_8(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (11, 5, 9) => Ok(InnerModel::HS11_5_9(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (11, 5, 10) => Ok(InnerModel::HS11_5_10(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (11, 6, 1) => Ok(InnerModel::HS11_6_1(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (11, 6, 2) => Ok(InnerModel::HS11_6_2(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (11, 6, 3) => Ok(InnerModel::HS11_6_3(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (11, 6, 4) => Ok(InnerModel::HS11_6_4(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (11, 6, 5) => Ok(InnerModel::HS11_6_5(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (11, 6, 6) => Ok(InnerModel::HS11_6_6(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (11, 6, 7) => Ok(InnerModel::HS11_6_7(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (11, 6, 8) => Ok(InnerModel::HS11_6_8(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (11, 6, 9) => Ok(InnerModel::HS11_6_9(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (11, 6, 10) => Ok(InnerModel::HS11_6_10(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (12, 4, 1) => Ok(InnerModel::HS12_4_1(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (12, 4, 2) => Ok(InnerModel::HS12_4_2(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (12, 4, 3) => Ok(InnerModel::HS12_4_3(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (12, 4, 4) => Ok(InnerModel::HS12_4_4(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (12, 4, 5) => Ok(InnerModel::HS12_4_5(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (12, 4, 6) => Ok(InnerModel::HS12_4_6(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (12, 4, 7) => Ok(InnerModel::HS12_4_7(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (12, 4, 8) => Ok(InnerModel::HS12_4_8(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (12, 4, 9) => Ok(InnerModel::HS12_4_9(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (12, 4, 10) => Ok(InnerModel::HS12_4_10(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (12, 5, 1) => Ok(InnerModel::HS12_5_1(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (12, 5, 2) => Ok(InnerModel::HS12_5_2(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (12, 5, 3) => Ok(InnerModel::HS12_5_3(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (12, 5, 4) => Ok(InnerModel::HS12_5_4(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (12, 5, 5) => Ok(InnerModel::HS12_5_5(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (12, 5, 6) => Ok(InnerModel::HS12_5_6(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (12, 5, 7) => Ok(InnerModel::HS12_5_7(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (12, 5, 8) => Ok(InnerModel::HS12_5_8(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (12, 5, 9) => Ok(InnerModel::HS12_5_9(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (12, 5, 10) => Ok(InnerModel::HS12_5_10(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (12, 6, 1) => Ok(InnerModel::HS12_6_1(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (12, 6, 2) => Ok(InnerModel::HS12_6_2(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (12, 6, 3) => Ok(InnerModel::HS12_6_3(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (12, 6, 4) => Ok(InnerModel::HS12_6_4(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (12, 6, 5) => Ok(InnerModel::HS12_6_5(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (12, 6, 6) => Ok(InnerModel::HS12_6_6(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (12, 6, 7) => Ok(InnerModel::HS12_6_7(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (12, 6, 8) => Ok(InnerModel::HS12_6_8(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (12, 6, 9) => Ok(InnerModel::HS12_6_9(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (12, 6, 10) => Ok(InnerModel::HS12_6_10(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (13, 4, 1) => Ok(InnerModel::HS13_4_1(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (13, 4, 2) => Ok(InnerModel::HS13_4_2(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (13, 4, 3) => Ok(InnerModel::HS13_4_3(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (13, 4, 4) => Ok(InnerModel::HS13_4_4(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (13, 4, 5) => Ok(InnerModel::HS13_4_5(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (13, 4, 6) => Ok(InnerModel::HS13_4_6(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (13, 4, 7) => Ok(InnerModel::HS13_4_7(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (13, 4, 8) => Ok(InnerModel::HS13_4_8(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (13, 4, 9) => Ok(InnerModel::HS13_4_9(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (13, 4, 10) => Ok(InnerModel::HS13_4_10(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (13, 5, 1) => Ok(InnerModel::HS13_5_1(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (13, 5, 2) => Ok(InnerModel::HS13_5_2(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (13, 5, 3) => Ok(InnerModel::HS13_5_3(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (13, 5, 4) => Ok(InnerModel::HS13_5_4(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (13, 5, 5) => Ok(InnerModel::HS13_5_5(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (13, 5, 6) => Ok(InnerModel::HS13_5_6(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (13, 5, 7) => Ok(InnerModel::HS13_5_7(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (13, 5, 8) => Ok(InnerModel::HS13_5_8(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (13, 5, 9) => Ok(InnerModel::HS13_5_9(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (13, 5, 10) => Ok(InnerModel::HS13_5_10(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (13, 6, 1) => Ok(InnerModel::HS13_6_1(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (13, 6, 2) => Ok(InnerModel::HS13_6_2(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (13, 6, 3) => Ok(InnerModel::HS13_6_3(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (13, 6, 4) => Ok(InnerModel::HS13_6_4(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (13, 6, 5) => Ok(InnerModel::HS13_6_5(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (13, 6, 6) => Ok(InnerModel::HS13_6_6(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (13, 6, 7) => Ok(InnerModel::HS13_6_7(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (13, 6, 8) => Ok(InnerModel::HS13_6_8(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (13, 6, 9) => Ok(InnerModel::HS13_6_9(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (13, 6, 10) => Ok(InnerModel::HS13_6_10(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (14, 4, 1) => Ok(InnerModel::HS14_4_1(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (14, 4, 2) => Ok(InnerModel::HS14_4_2(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (14, 4, 3) => Ok(InnerModel::HS14_4_3(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (14, 4, 4) => Ok(InnerModel::HS14_4_4(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (14, 4, 5) => Ok(InnerModel::HS14_4_5(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (14, 4, 6) => Ok(InnerModel::HS14_4_6(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (14, 4, 7) => Ok(InnerModel::HS14_4_7(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (14, 4, 8) => Ok(InnerModel::HS14_4_8(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (14, 4, 9) => Ok(InnerModel::HS14_4_9(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (14, 4, 10) => Ok(InnerModel::HS14_4_10(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (14, 5, 1) => Ok(InnerModel::HS14_5_1(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (14, 5, 2) => Ok(InnerModel::HS14_5_2(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (14, 5, 3) => Ok(InnerModel::HS14_5_3(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (14, 5, 4) => Ok(InnerModel::HS14_5_4(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (14, 5, 5) => Ok(InnerModel::HS14_5_5(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (14, 5, 6) => Ok(InnerModel::HS14_5_6(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (14, 5, 7) => Ok(InnerModel::HS14_5_7(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (14, 5, 8) => Ok(InnerModel::HS14_5_8(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (14, 5, 9) => Ok(InnerModel::HS14_5_9(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (14, 5, 10) => Ok(InnerModel::HS14_5_10(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (14, 6, 1) => Ok(InnerModel::HS14_6_1(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (14, 6, 2) => Ok(InnerModel::HS14_6_2(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (14, 6, 3) => Ok(InnerModel::HS14_6_3(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (14, 6, 4) => Ok(InnerModel::HS14_6_4(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (14, 6, 5) => Ok(InnerModel::HS14_6_5(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (14, 6, 6) => Ok(InnerModel::HS14_6_6(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (14, 6, 7) => Ok(InnerModel::HS14_6_7(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (14, 6, 8) => Ok(InnerModel::HS14_6_8(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (14, 6, 9) => Ok(InnerModel::HS14_6_9(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (14, 6, 10) => Ok(InnerModel::HS14_6_10(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (15, 4, 1) => Ok(InnerModel::HS15_4_1(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (15, 4, 2) => Ok(InnerModel::HS15_4_2(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (15, 4, 3) => Ok(InnerModel::HS15_4_3(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (15, 4, 4) => Ok(InnerModel::HS15_4_4(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (15, 4, 5) => Ok(InnerModel::HS15_4_5(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (15, 4, 6) => Ok(InnerModel::HS15_4_6(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (15, 4, 7) => Ok(InnerModel::HS15_4_7(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (15, 4, 8) => Ok(InnerModel::HS15_4_8(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (15, 4, 9) => Ok(InnerModel::HS15_4_9(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (15, 4, 10) => Ok(InnerModel::HS15_4_10(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (15, 5, 1) => Ok(InnerModel::HS15_5_1(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (15, 5, 2) => Ok(InnerModel::HS15_5_2(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (15, 5, 3) => Ok(InnerModel::HS15_5_3(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (15, 5, 4) => Ok(InnerModel::HS15_5_4(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (15, 5, 5) => Ok(InnerModel::HS15_5_5(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (15, 5, 6) => Ok(InnerModel::HS15_5_6(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (15, 5, 7) => Ok(InnerModel::HS15_5_7(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (15, 5, 8) => Ok(InnerModel::HS15_5_8(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (15, 5, 9) => Ok(InnerModel::HS15_5_9(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (15, 5, 10) => Ok(InnerModel::HS15_5_10(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (15, 6, 1) => Ok(InnerModel::HS15_6_1(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (15, 6, 2) => Ok(InnerModel::HS15_6_2(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (15, 6, 3) => Ok(InnerModel::HS15_6_3(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (15, 6, 4) => Ok(InnerModel::HS15_6_4(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (15, 6, 5) => Ok(InnerModel::HS15_6_5(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (15, 6, 6) => Ok(InnerModel::HS15_6_6(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (15, 6, 7) => Ok(InnerModel::HS15_6_7(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (15, 6, 8) => Ok(InnerModel::HS15_6_8(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (15, 6, 9) => Ok(InnerModel::HS15_6_9(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (15, 6, 10) => Ok(InnerModel::HS15_6_10(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (16, 4, 1) => Ok(InnerModel::HS16_4_1(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (16, 4, 2) => Ok(InnerModel::HS16_4_2(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (16, 4, 3) => Ok(InnerModel::HS16_4_3(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (16, 4, 4) => Ok(InnerModel::HS16_4_4(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (16, 4, 5) => Ok(InnerModel::HS16_4_5(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (16, 4, 6) => Ok(InnerModel::HS16_4_6(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (16, 4, 7) => Ok(InnerModel::HS16_4_7(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (16, 4, 8) => Ok(InnerModel::HS16_4_8(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (16, 4, 9) => Ok(InnerModel::HS16_4_9(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (16, 4, 10) => Ok(InnerModel::HS16_4_10(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (16, 5, 1) => Ok(InnerModel::HS16_5_1(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (16, 5, 2) => Ok(InnerModel::HS16_5_2(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (16, 5, 3) => Ok(InnerModel::HS16_5_3(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (16, 5, 4) => Ok(InnerModel::HS16_5_4(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (16, 5, 5) => Ok(InnerModel::HS16_5_5(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (16, 5, 6) => Ok(InnerModel::HS16_5_6(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (16, 5, 7) => Ok(InnerModel::HS16_5_7(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (16, 5, 8) => Ok(InnerModel::HS16_5_8(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (16, 5, 9) => Ok(InnerModel::HS16_5_9(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (16, 5, 10) => Ok(InnerModel::HS16_5_10(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (16, 6, 1) => Ok(InnerModel::HS16_6_1(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (16, 6, 2) => Ok(InnerModel::HS16_6_2(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (16, 6, 3) => Ok(InnerModel::HS16_6_3(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (16, 6, 4) => Ok(InnerModel::HS16_6_4(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (16, 6, 5) => Ok(InnerModel::HS16_6_5(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (16, 6, 6) => Ok(InnerModel::HS16_6_6(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (16, 6, 7) => Ok(InnerModel::HS16_6_7(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (16, 6, 8) => Ok(InnerModel::HS16_6_8(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (16, 6, 9) => Ok(InnerModel::HS16_6_9(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            (16, 6, 10) => Ok(InnerModel::HS16_6_10(HS::new(
                unbiased,
                exact,
                include_node_types,
                include_edge_types,
                include_edge_ids,
                include_node_ids,
                include_selfloops,
                include_typed_graphlets,
                random_state,
                number_of_random_integers,
                normalize_by_symmetric_laplacian,
                concatenate_features,
                dtype,
            )?)), // {python_generated}
            _ => {
                return Err(format!(
                    concat!(
                        "The HyperSketching model supports precisions ranging from 4 ",
                        "to 16 and bits ranging from 4 to 6, and hops from 1 to 10. ",
                        "Provided precision: {}, bits: {}."
                    ),
                    precision, bits
                ))
            }
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
            InnerModel::HS4_4_1(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_4_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_4_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_4_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_4_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_4_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_4_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_4_8(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_4_9(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_4_10(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_5_1(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_5_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_5_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_5_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_5_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_5_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_5_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_5_8(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_5_9(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_5_10(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_6_1(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_6_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_6_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_6_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_6_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_6_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_6_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_6_8(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_6_9(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_6_10(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_4_1(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_4_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_4_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_4_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_4_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_4_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_4_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_4_8(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_4_9(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_4_10(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_5_1(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_5_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_5_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_5_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_5_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_5_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_5_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_5_8(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_5_9(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_5_10(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_6_1(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_6_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_6_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_6_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_6_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_6_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_6_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_6_8(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_6_9(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_6_10(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_4_1(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_4_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_4_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_4_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_4_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_4_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_4_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_4_8(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_4_9(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_4_10(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_5_1(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_5_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_5_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_5_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_5_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_5_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_5_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_5_8(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_5_9(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_5_10(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_6_1(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_6_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_6_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_6_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_6_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_6_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_6_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_6_8(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_6_9(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_6_10(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_4_1(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_4_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_4_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_4_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_4_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_4_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_4_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_4_8(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_4_9(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_4_10(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_5_1(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_5_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_5_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_5_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_5_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_5_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_5_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_5_8(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_5_9(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_5_10(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_6_1(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_6_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_6_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_6_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_6_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_6_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_6_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_6_8(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_6_9(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_6_10(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_4_1(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_4_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_4_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_4_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_4_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_4_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_4_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_4_8(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_4_9(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_4_10(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_5_1(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_5_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_5_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_5_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_5_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_5_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_5_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_5_8(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_5_9(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_5_10(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_6_1(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_6_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_6_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_6_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_6_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_6_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_6_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_6_8(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_6_9(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_6_10(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_4_1(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_4_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_4_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_4_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_4_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_4_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_4_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_4_8(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_4_9(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_4_10(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_5_1(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_5_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_5_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_5_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_5_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_5_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_5_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_5_8(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_5_9(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_5_10(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_6_1(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_6_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_6_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_6_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_6_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_6_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_6_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_6_8(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_6_9(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_6_10(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_4_1(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_4_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_4_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_4_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_4_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_4_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_4_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_4_8(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_4_9(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_4_10(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_5_1(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_5_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_5_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_5_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_5_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_5_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_5_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_5_8(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_5_9(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_5_10(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_6_1(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_6_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_6_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_6_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_6_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_6_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_6_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_6_8(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_6_9(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_6_10(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_4_1(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_4_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_4_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_4_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_4_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_4_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_4_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_4_8(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_4_9(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_4_10(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_5_1(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_5_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_5_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_5_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_5_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_5_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_5_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_5_8(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_5_9(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_5_10(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_6_1(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_6_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_6_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_6_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_6_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_6_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_6_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_6_8(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_6_9(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_6_10(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_4_1(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_4_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_4_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_4_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_4_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_4_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_4_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_4_8(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_4_9(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_4_10(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_5_1(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_5_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_5_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_5_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_5_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_5_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_5_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_5_8(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_5_9(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_5_10(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_6_1(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_6_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_6_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_6_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_6_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_6_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_6_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_6_8(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_6_9(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_6_10(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS13_4_1(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS13_4_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS13_4_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS13_4_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS13_4_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS13_4_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS13_4_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS13_4_8(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS13_4_9(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS13_4_10(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS13_5_1(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS13_5_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS13_5_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS13_5_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS13_5_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS13_5_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS13_5_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS13_5_8(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS13_5_9(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS13_5_10(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS13_6_1(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS13_6_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS13_6_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS13_6_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS13_6_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS13_6_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS13_6_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS13_6_8(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS13_6_9(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS13_6_10(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS14_4_1(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS14_4_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS14_4_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS14_4_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS14_4_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS14_4_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS14_4_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS14_4_8(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS14_4_9(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS14_4_10(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS14_5_1(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS14_5_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS14_5_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS14_5_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS14_5_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS14_5_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS14_5_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS14_5_8(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS14_5_9(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS14_5_10(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS14_6_1(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS14_6_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS14_6_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS14_6_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS14_6_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS14_6_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS14_6_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS14_6_8(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS14_6_9(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS14_6_10(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS15_4_1(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS15_4_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS15_4_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS15_4_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS15_4_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS15_4_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS15_4_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS15_4_8(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS15_4_9(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS15_4_10(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS15_5_1(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS15_5_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS15_5_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS15_5_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS15_5_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS15_5_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS15_5_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS15_5_8(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS15_5_9(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS15_5_10(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS15_6_1(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS15_6_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS15_6_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS15_6_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS15_6_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS15_6_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS15_6_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS15_6_8(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS15_6_9(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS15_6_10(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS16_4_1(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS16_4_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS16_4_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS16_4_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS16_4_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS16_4_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS16_4_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS16_4_8(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS16_4_9(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS16_4_10(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS16_5_1(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS16_5_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS16_5_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS16_5_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS16_5_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS16_5_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS16_5_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS16_5_8(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS16_5_9(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS16_5_10(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS16_6_1(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS16_6_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS16_6_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS16_6_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS16_6_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS16_6_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS16_6_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS16_6_8(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS16_6_9(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS16_6_10(inner) => inner.fit(graph), // {python_generated}
        }
    }

    /// Returns the estimated Sketching for all edges.
    ///
    /// Parameters
    /// ------------------------
    /// edge_features: &mut [f32]
    ///     The array where to store the estimated Sketching for all edges.
    /// support: &Graph
    ///     The graph whose topology is to be learned.
    /// edge_iterator: I
    ///    The iterator over the edges whose Sketching is to be estimated.
    ///
    /// Raises
    /// ------------------------
    /// ValueError
    ///     If the provided arrays are not of the right size.
    ///     If the model has not been trained.
    ///
    unsafe fn compute_sketching_from_iterator<I, F: Float + Primitive<f32>>(
        &self,
        edge_features: &mut [F],
        support: &graph::Graph,
        edge_iterator: I,
    ) -> Result<()>
    where
        I: IndexedParallelIterator<Item = (NodeT, NodeT)>,
    {
        match self {
            // InnerModel::HS{precision}_{bits}_{hops}(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(edge_features, support, edge_iterator), {python_macro}
            InnerModel::HS4_4_1(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS4_4_2(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS4_4_3(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS4_4_4(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS4_4_5(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS4_4_6(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS4_4_7(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS4_4_8(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS4_4_9(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS4_4_10(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS4_5_1(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS4_5_2(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS4_5_3(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS4_5_4(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS4_5_5(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS4_5_6(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS4_5_7(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS4_5_8(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS4_5_9(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS4_5_10(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS4_6_1(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS4_6_2(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS4_6_3(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS4_6_4(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS4_6_5(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS4_6_6(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS4_6_7(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS4_6_8(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS4_6_9(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS4_6_10(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS5_4_1(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS5_4_2(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS5_4_3(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS5_4_4(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS5_4_5(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS5_4_6(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS5_4_7(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS5_4_8(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS5_4_9(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS5_4_10(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS5_5_1(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS5_5_2(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS5_5_3(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS5_5_4(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS5_5_5(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS5_5_6(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS5_5_7(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS5_5_8(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS5_5_9(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS5_5_10(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS5_6_1(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS5_6_2(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS5_6_3(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS5_6_4(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS5_6_5(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS5_6_6(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS5_6_7(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS5_6_8(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS5_6_9(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS5_6_10(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS6_4_1(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS6_4_2(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS6_4_3(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS6_4_4(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS6_4_5(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS6_4_6(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS6_4_7(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS6_4_8(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS6_4_9(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS6_4_10(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS6_5_1(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS6_5_2(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS6_5_3(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS6_5_4(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS6_5_5(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS6_5_6(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS6_5_7(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS6_5_8(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS6_5_9(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS6_5_10(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS6_6_1(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS6_6_2(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS6_6_3(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS6_6_4(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS6_6_5(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS6_6_6(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS6_6_7(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS6_6_8(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS6_6_9(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS6_6_10(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS7_4_1(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS7_4_2(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS7_4_3(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS7_4_4(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS7_4_5(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS7_4_6(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS7_4_7(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS7_4_8(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS7_4_9(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS7_4_10(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS7_5_1(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS7_5_2(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS7_5_3(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS7_5_4(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS7_5_5(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS7_5_6(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS7_5_7(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS7_5_8(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS7_5_9(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS7_5_10(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS7_6_1(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS7_6_2(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS7_6_3(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS7_6_4(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS7_6_5(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS7_6_6(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS7_6_7(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS7_6_8(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS7_6_9(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS7_6_10(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS8_4_1(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS8_4_2(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS8_4_3(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS8_4_4(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS8_4_5(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS8_4_6(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS8_4_7(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS8_4_8(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS8_4_9(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS8_4_10(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS8_5_1(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS8_5_2(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS8_5_3(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS8_5_4(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS8_5_5(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS8_5_6(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS8_5_7(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS8_5_8(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS8_5_9(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS8_5_10(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS8_6_1(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS8_6_2(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS8_6_3(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS8_6_4(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS8_6_5(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS8_6_6(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS8_6_7(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS8_6_8(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS8_6_9(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS8_6_10(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS9_4_1(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS9_4_2(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS9_4_3(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS9_4_4(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS9_4_5(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS9_4_6(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS9_4_7(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS9_4_8(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS9_4_9(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS9_4_10(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS9_5_1(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS9_5_2(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS9_5_3(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS9_5_4(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS9_5_5(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS9_5_6(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS9_5_7(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS9_5_8(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS9_5_9(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS9_5_10(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS9_6_1(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS9_6_2(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS9_6_3(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS9_6_4(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS9_6_5(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS9_6_6(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS9_6_7(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS9_6_8(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS9_6_9(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS9_6_10(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS10_4_1(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS10_4_2(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS10_4_3(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS10_4_4(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS10_4_5(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS10_4_6(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS10_4_7(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS10_4_8(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS10_4_9(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS10_4_10(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS10_5_1(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS10_5_2(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS10_5_3(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS10_5_4(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS10_5_5(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS10_5_6(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS10_5_7(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS10_5_8(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS10_5_9(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS10_5_10(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS10_6_1(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS10_6_2(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS10_6_3(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS10_6_4(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS10_6_5(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS10_6_6(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS10_6_7(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS10_6_8(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS10_6_9(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS10_6_10(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS11_4_1(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS11_4_2(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS11_4_3(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS11_4_4(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS11_4_5(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS11_4_6(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS11_4_7(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS11_4_8(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS11_4_9(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS11_4_10(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS11_5_1(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS11_5_2(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS11_5_3(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS11_5_4(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS11_5_5(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS11_5_6(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS11_5_7(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS11_5_8(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS11_5_9(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS11_5_10(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS11_6_1(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS11_6_2(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS11_6_3(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS11_6_4(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS11_6_5(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS11_6_6(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS11_6_7(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS11_6_8(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS11_6_9(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS11_6_10(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS12_4_1(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS12_4_2(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS12_4_3(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS12_4_4(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS12_4_5(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS12_4_6(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS12_4_7(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS12_4_8(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS12_4_9(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS12_4_10(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS12_5_1(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS12_5_2(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS12_5_3(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS12_5_4(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS12_5_5(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS12_5_6(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS12_5_7(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS12_5_8(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS12_5_9(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS12_5_10(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS12_6_1(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS12_6_2(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS12_6_3(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS12_6_4(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS12_6_5(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS12_6_6(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS12_6_7(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS12_6_8(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS12_6_9(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS12_6_10(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS13_4_1(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS13_4_2(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS13_4_3(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS13_4_4(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS13_4_5(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS13_4_6(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS13_4_7(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS13_4_8(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS13_4_9(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS13_4_10(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS13_5_1(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS13_5_2(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS13_5_3(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS13_5_4(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS13_5_5(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS13_5_6(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS13_5_7(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS13_5_8(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS13_5_9(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS13_5_10(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS13_6_1(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS13_6_2(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS13_6_3(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS13_6_4(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS13_6_5(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS13_6_6(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS13_6_7(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS13_6_8(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS13_6_9(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS13_6_10(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS14_4_1(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS14_4_2(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS14_4_3(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS14_4_4(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS14_4_5(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS14_4_6(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS14_4_7(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS14_4_8(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS14_4_9(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS14_4_10(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS14_5_1(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS14_5_2(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS14_5_3(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS14_5_4(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS14_5_5(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS14_5_6(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS14_5_7(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS14_5_8(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS14_5_9(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS14_5_10(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS14_6_1(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS14_6_2(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS14_6_3(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS14_6_4(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS14_6_5(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS14_6_6(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS14_6_7(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS14_6_8(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS14_6_9(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS14_6_10(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS15_4_1(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS15_4_2(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS15_4_3(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS15_4_4(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS15_4_5(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS15_4_6(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS15_4_7(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS15_4_8(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS15_4_9(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS15_4_10(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS15_5_1(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS15_5_2(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS15_5_3(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS15_5_4(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS15_5_5(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS15_5_6(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS15_5_7(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS15_5_8(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS15_5_9(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS15_5_10(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS15_6_1(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS15_6_2(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS15_6_3(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS15_6_4(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS15_6_5(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS15_6_6(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS15_6_7(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS15_6_8(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS15_6_9(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS15_6_10(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS16_4_1(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS16_4_2(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS16_4_3(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS16_4_4(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS16_4_5(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS16_4_6(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS16_4_7(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS16_4_8(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS16_4_9(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS16_4_10(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS16_5_1(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS16_5_2(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS16_5_3(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS16_5_4(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS16_5_5(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS16_5_6(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS16_5_7(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS16_5_8(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS16_5_9(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS16_5_10(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS16_6_1(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS16_6_2(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS16_6_3(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS16_6_4(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS16_6_5(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS16_6_6(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS16_6_7(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS16_6_8(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS16_6_9(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
            InnerModel::HS16_6_10(inner) => inner.get_sketching_for_all_edges_unchecked::<I, F>(
                edge_features,
                support,
                edge_iterator,
            ), // {python_generated}
        }
    }

    /// Returns the data type to be used for the sketches
    fn get_dtype(&self) -> &str {
        match self {
            // InnerModel::HS{precision}_{bits}_{hops}(inner) => inner.get_dtype(), {python_macro}
            InnerModel::HS4_4_1(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS4_4_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS4_4_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS4_4_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS4_4_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS4_4_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS4_4_7(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS4_4_8(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS4_4_9(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS4_4_10(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS4_5_1(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS4_5_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS4_5_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS4_5_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS4_5_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS4_5_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS4_5_7(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS4_5_8(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS4_5_9(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS4_5_10(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS4_6_1(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS4_6_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS4_6_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS4_6_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS4_6_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS4_6_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS4_6_7(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS4_6_8(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS4_6_9(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS4_6_10(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS5_4_1(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS5_4_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS5_4_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS5_4_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS5_4_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS5_4_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS5_4_7(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS5_4_8(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS5_4_9(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS5_4_10(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS5_5_1(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS5_5_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS5_5_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS5_5_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS5_5_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS5_5_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS5_5_7(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS5_5_8(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS5_5_9(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS5_5_10(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS5_6_1(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS5_6_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS5_6_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS5_6_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS5_6_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS5_6_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS5_6_7(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS5_6_8(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS5_6_9(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS5_6_10(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS6_4_1(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS6_4_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS6_4_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS6_4_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS6_4_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS6_4_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS6_4_7(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS6_4_8(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS6_4_9(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS6_4_10(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS6_5_1(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS6_5_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS6_5_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS6_5_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS6_5_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS6_5_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS6_5_7(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS6_5_8(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS6_5_9(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS6_5_10(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS6_6_1(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS6_6_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS6_6_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS6_6_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS6_6_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS6_6_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS6_6_7(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS6_6_8(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS6_6_9(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS6_6_10(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS7_4_1(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS7_4_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS7_4_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS7_4_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS7_4_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS7_4_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS7_4_7(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS7_4_8(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS7_4_9(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS7_4_10(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS7_5_1(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS7_5_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS7_5_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS7_5_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS7_5_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS7_5_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS7_5_7(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS7_5_8(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS7_5_9(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS7_5_10(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS7_6_1(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS7_6_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS7_6_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS7_6_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS7_6_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS7_6_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS7_6_7(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS7_6_8(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS7_6_9(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS7_6_10(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS8_4_1(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS8_4_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS8_4_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS8_4_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS8_4_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS8_4_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS8_4_7(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS8_4_8(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS8_4_9(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS8_4_10(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS8_5_1(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS8_5_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS8_5_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS8_5_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS8_5_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS8_5_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS8_5_7(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS8_5_8(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS8_5_9(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS8_5_10(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS8_6_1(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS8_6_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS8_6_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS8_6_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS8_6_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS8_6_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS8_6_7(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS8_6_8(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS8_6_9(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS8_6_10(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS9_4_1(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS9_4_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS9_4_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS9_4_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS9_4_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS9_4_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS9_4_7(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS9_4_8(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS9_4_9(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS9_4_10(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS9_5_1(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS9_5_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS9_5_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS9_5_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS9_5_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS9_5_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS9_5_7(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS9_5_8(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS9_5_9(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS9_5_10(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS9_6_1(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS9_6_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS9_6_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS9_6_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS9_6_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS9_6_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS9_6_7(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS9_6_8(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS9_6_9(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS9_6_10(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS10_4_1(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS10_4_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS10_4_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS10_4_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS10_4_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS10_4_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS10_4_7(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS10_4_8(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS10_4_9(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS10_4_10(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS10_5_1(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS10_5_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS10_5_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS10_5_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS10_5_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS10_5_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS10_5_7(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS10_5_8(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS10_5_9(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS10_5_10(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS10_6_1(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS10_6_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS10_6_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS10_6_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS10_6_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS10_6_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS10_6_7(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS10_6_8(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS10_6_9(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS10_6_10(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS11_4_1(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS11_4_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS11_4_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS11_4_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS11_4_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS11_4_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS11_4_7(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS11_4_8(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS11_4_9(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS11_4_10(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS11_5_1(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS11_5_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS11_5_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS11_5_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS11_5_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS11_5_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS11_5_7(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS11_5_8(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS11_5_9(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS11_5_10(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS11_6_1(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS11_6_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS11_6_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS11_6_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS11_6_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS11_6_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS11_6_7(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS11_6_8(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS11_6_9(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS11_6_10(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS12_4_1(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS12_4_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS12_4_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS12_4_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS12_4_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS12_4_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS12_4_7(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS12_4_8(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS12_4_9(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS12_4_10(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS12_5_1(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS12_5_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS12_5_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS12_5_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS12_5_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS12_5_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS12_5_7(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS12_5_8(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS12_5_9(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS12_5_10(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS12_6_1(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS12_6_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS12_6_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS12_6_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS12_6_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS12_6_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS12_6_7(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS12_6_8(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS12_6_9(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS12_6_10(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS13_4_1(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS13_4_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS13_4_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS13_4_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS13_4_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS13_4_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS13_4_7(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS13_4_8(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS13_4_9(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS13_4_10(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS13_5_1(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS13_5_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS13_5_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS13_5_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS13_5_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS13_5_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS13_5_7(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS13_5_8(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS13_5_9(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS13_5_10(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS13_6_1(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS13_6_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS13_6_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS13_6_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS13_6_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS13_6_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS13_6_7(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS13_6_8(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS13_6_9(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS13_6_10(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS14_4_1(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS14_4_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS14_4_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS14_4_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS14_4_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS14_4_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS14_4_7(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS14_4_8(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS14_4_9(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS14_4_10(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS14_5_1(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS14_5_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS14_5_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS14_5_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS14_5_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS14_5_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS14_5_7(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS14_5_8(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS14_5_9(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS14_5_10(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS14_6_1(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS14_6_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS14_6_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS14_6_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS14_6_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS14_6_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS14_6_7(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS14_6_8(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS14_6_9(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS14_6_10(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS15_4_1(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS15_4_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS15_4_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS15_4_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS15_4_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS15_4_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS15_4_7(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS15_4_8(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS15_4_9(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS15_4_10(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS15_5_1(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS15_5_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS15_5_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS15_5_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS15_5_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS15_5_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS15_5_7(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS15_5_8(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS15_5_9(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS15_5_10(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS15_6_1(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS15_6_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS15_6_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS15_6_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS15_6_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS15_6_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS15_6_7(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS15_6_8(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS15_6_9(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS15_6_10(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS16_4_1(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS16_4_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS16_4_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS16_4_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS16_4_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS16_4_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS16_4_7(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS16_4_8(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS16_4_9(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS16_4_10(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS16_5_1(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS16_5_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS16_5_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS16_5_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS16_5_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS16_5_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS16_5_7(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS16_5_8(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS16_5_9(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS16_5_10(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS16_6_1(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS16_6_2(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS16_6_3(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS16_6_4(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS16_6_5(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS16_6_6(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS16_6_7(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS16_6_8(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS16_6_9(inner) => inner.get_dtype(), // {python_generated}
            InnerModel::HS16_6_10(inner) => inner.get_dtype(), // {python_generated}
        }
    }

    /// Returns the number of bits used for the HyperLogLog counters in the model.
    fn get_bits(&self) -> usize {
        match self {
            // InnerModel::HS{precision}_{bits}_{hops}(inner) => inner.get_bits(), {python_macro}
            InnerModel::HS4_4_1(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_4_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_4_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_4_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_4_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_4_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_4_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_4_8(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_4_9(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_4_10(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_5_1(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_5_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_5_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_5_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_5_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_5_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_5_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_5_8(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_5_9(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_5_10(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_6_1(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_6_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_6_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_6_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_6_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_6_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_6_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_6_8(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_6_9(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_6_10(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_4_1(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_4_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_4_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_4_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_4_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_4_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_4_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_4_8(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_4_9(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_4_10(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_5_1(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_5_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_5_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_5_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_5_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_5_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_5_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_5_8(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_5_9(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_5_10(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_6_1(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_6_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_6_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_6_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_6_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_6_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_6_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_6_8(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_6_9(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_6_10(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_4_1(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_4_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_4_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_4_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_4_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_4_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_4_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_4_8(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_4_9(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_4_10(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_5_1(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_5_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_5_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_5_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_5_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_5_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_5_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_5_8(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_5_9(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_5_10(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_6_1(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_6_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_6_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_6_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_6_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_6_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_6_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_6_8(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_6_9(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_6_10(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_4_1(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_4_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_4_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_4_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_4_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_4_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_4_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_4_8(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_4_9(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_4_10(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_5_1(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_5_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_5_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_5_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_5_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_5_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_5_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_5_8(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_5_9(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_5_10(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_6_1(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_6_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_6_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_6_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_6_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_6_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_6_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_6_8(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_6_9(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_6_10(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_4_1(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_4_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_4_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_4_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_4_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_4_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_4_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_4_8(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_4_9(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_4_10(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_5_1(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_5_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_5_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_5_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_5_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_5_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_5_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_5_8(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_5_9(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_5_10(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_6_1(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_6_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_6_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_6_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_6_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_6_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_6_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_6_8(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_6_9(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_6_10(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_4_1(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_4_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_4_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_4_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_4_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_4_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_4_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_4_8(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_4_9(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_4_10(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_5_1(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_5_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_5_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_5_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_5_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_5_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_5_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_5_8(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_5_9(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_5_10(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_6_1(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_6_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_6_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_6_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_6_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_6_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_6_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_6_8(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_6_9(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_6_10(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_4_1(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_4_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_4_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_4_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_4_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_4_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_4_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_4_8(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_4_9(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_4_10(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_5_1(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_5_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_5_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_5_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_5_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_5_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_5_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_5_8(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_5_9(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_5_10(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_6_1(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_6_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_6_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_6_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_6_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_6_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_6_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_6_8(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_6_9(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_6_10(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_4_1(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_4_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_4_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_4_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_4_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_4_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_4_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_4_8(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_4_9(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_4_10(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_5_1(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_5_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_5_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_5_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_5_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_5_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_5_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_5_8(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_5_9(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_5_10(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_6_1(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_6_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_6_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_6_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_6_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_6_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_6_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_6_8(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_6_9(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_6_10(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_4_1(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_4_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_4_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_4_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_4_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_4_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_4_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_4_8(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_4_9(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_4_10(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_5_1(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_5_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_5_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_5_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_5_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_5_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_5_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_5_8(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_5_9(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_5_10(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_6_1(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_6_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_6_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_6_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_6_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_6_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_6_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_6_8(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_6_9(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_6_10(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS13_4_1(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS13_4_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS13_4_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS13_4_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS13_4_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS13_4_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS13_4_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS13_4_8(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS13_4_9(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS13_4_10(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS13_5_1(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS13_5_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS13_5_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS13_5_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS13_5_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS13_5_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS13_5_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS13_5_8(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS13_5_9(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS13_5_10(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS13_6_1(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS13_6_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS13_6_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS13_6_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS13_6_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS13_6_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS13_6_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS13_6_8(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS13_6_9(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS13_6_10(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS14_4_1(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS14_4_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS14_4_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS14_4_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS14_4_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS14_4_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS14_4_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS14_4_8(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS14_4_9(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS14_4_10(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS14_5_1(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS14_5_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS14_5_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS14_5_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS14_5_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS14_5_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS14_5_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS14_5_8(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS14_5_9(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS14_5_10(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS14_6_1(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS14_6_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS14_6_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS14_6_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS14_6_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS14_6_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS14_6_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS14_6_8(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS14_6_9(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS14_6_10(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS15_4_1(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS15_4_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS15_4_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS15_4_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS15_4_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS15_4_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS15_4_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS15_4_8(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS15_4_9(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS15_4_10(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS15_5_1(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS15_5_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS15_5_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS15_5_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS15_5_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS15_5_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS15_5_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS15_5_8(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS15_5_9(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS15_5_10(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS15_6_1(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS15_6_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS15_6_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS15_6_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS15_6_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS15_6_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS15_6_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS15_6_8(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS15_6_9(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS15_6_10(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS16_4_1(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS16_4_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS16_4_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS16_4_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS16_4_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS16_4_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS16_4_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS16_4_8(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS16_4_9(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS16_4_10(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS16_5_1(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS16_5_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS16_5_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS16_5_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS16_5_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS16_5_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS16_5_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS16_5_8(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS16_5_9(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS16_5_10(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS16_6_1(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS16_6_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS16_6_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS16_6_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS16_6_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS16_6_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS16_6_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS16_6_8(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS16_6_9(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS16_6_10(inner) => inner.get_bits(), // {python_generated}
        }
    }

    /// Returns the precision used for the HyperLogLog counters in the model.
    fn get_precision(&self) -> usize {
        match self {
            // InnerModel::HS{precision}_{bits}_{hops}(inner) => inner.get_precision(), {python_macro}
            InnerModel::HS4_4_1(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_4_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_4_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_4_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_4_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_4_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_4_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_4_8(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_4_9(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_4_10(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_5_1(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_5_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_5_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_5_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_5_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_5_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_5_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_5_8(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_5_9(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_5_10(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_6_1(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_6_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_6_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_6_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_6_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_6_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_6_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_6_8(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_6_9(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_6_10(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_4_1(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_4_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_4_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_4_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_4_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_4_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_4_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_4_8(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_4_9(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_4_10(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_5_1(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_5_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_5_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_5_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_5_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_5_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_5_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_5_8(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_5_9(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_5_10(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_6_1(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_6_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_6_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_6_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_6_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_6_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_6_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_6_8(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_6_9(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_6_10(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_4_1(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_4_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_4_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_4_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_4_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_4_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_4_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_4_8(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_4_9(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_4_10(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_5_1(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_5_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_5_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_5_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_5_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_5_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_5_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_5_8(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_5_9(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_5_10(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_6_1(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_6_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_6_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_6_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_6_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_6_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_6_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_6_8(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_6_9(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_6_10(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_4_1(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_4_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_4_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_4_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_4_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_4_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_4_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_4_8(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_4_9(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_4_10(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_5_1(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_5_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_5_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_5_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_5_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_5_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_5_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_5_8(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_5_9(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_5_10(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_6_1(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_6_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_6_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_6_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_6_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_6_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_6_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_6_8(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_6_9(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_6_10(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_4_1(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_4_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_4_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_4_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_4_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_4_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_4_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_4_8(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_4_9(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_4_10(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_5_1(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_5_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_5_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_5_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_5_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_5_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_5_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_5_8(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_5_9(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_5_10(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_6_1(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_6_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_6_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_6_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_6_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_6_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_6_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_6_8(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_6_9(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_6_10(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_4_1(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_4_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_4_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_4_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_4_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_4_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_4_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_4_8(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_4_9(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_4_10(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_5_1(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_5_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_5_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_5_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_5_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_5_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_5_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_5_8(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_5_9(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_5_10(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_6_1(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_6_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_6_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_6_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_6_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_6_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_6_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_6_8(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_6_9(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_6_10(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_4_1(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_4_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_4_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_4_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_4_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_4_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_4_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_4_8(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_4_9(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_4_10(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_5_1(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_5_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_5_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_5_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_5_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_5_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_5_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_5_8(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_5_9(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_5_10(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_6_1(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_6_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_6_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_6_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_6_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_6_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_6_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_6_8(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_6_9(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_6_10(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_4_1(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_4_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_4_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_4_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_4_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_4_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_4_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_4_8(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_4_9(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_4_10(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_5_1(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_5_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_5_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_5_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_5_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_5_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_5_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_5_8(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_5_9(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_5_10(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_6_1(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_6_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_6_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_6_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_6_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_6_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_6_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_6_8(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_6_9(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_6_10(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_4_1(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_4_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_4_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_4_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_4_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_4_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_4_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_4_8(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_4_9(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_4_10(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_5_1(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_5_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_5_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_5_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_5_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_5_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_5_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_5_8(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_5_9(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_5_10(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_6_1(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_6_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_6_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_6_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_6_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_6_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_6_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_6_8(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_6_9(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_6_10(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS13_4_1(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS13_4_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS13_4_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS13_4_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS13_4_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS13_4_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS13_4_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS13_4_8(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS13_4_9(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS13_4_10(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS13_5_1(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS13_5_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS13_5_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS13_5_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS13_5_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS13_5_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS13_5_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS13_5_8(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS13_5_9(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS13_5_10(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS13_6_1(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS13_6_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS13_6_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS13_6_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS13_6_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS13_6_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS13_6_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS13_6_8(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS13_6_9(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS13_6_10(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS14_4_1(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS14_4_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS14_4_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS14_4_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS14_4_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS14_4_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS14_4_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS14_4_8(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS14_4_9(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS14_4_10(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS14_5_1(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS14_5_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS14_5_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS14_5_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS14_5_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS14_5_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS14_5_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS14_5_8(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS14_5_9(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS14_5_10(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS14_6_1(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS14_6_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS14_6_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS14_6_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS14_6_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS14_6_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS14_6_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS14_6_8(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS14_6_9(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS14_6_10(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS15_4_1(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS15_4_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS15_4_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS15_4_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS15_4_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS15_4_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS15_4_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS15_4_8(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS15_4_9(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS15_4_10(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS15_5_1(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS15_5_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS15_5_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS15_5_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS15_5_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS15_5_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS15_5_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS15_5_8(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS15_5_9(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS15_5_10(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS15_6_1(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS15_6_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS15_6_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS15_6_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS15_6_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS15_6_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS15_6_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS15_6_8(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS15_6_9(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS15_6_10(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS16_4_1(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS16_4_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS16_4_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS16_4_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS16_4_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS16_4_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS16_4_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS16_4_8(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS16_4_9(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS16_4_10(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS16_5_1(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS16_5_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS16_5_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS16_5_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS16_5_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS16_5_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS16_5_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS16_5_8(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS16_5_9(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS16_5_10(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS16_6_1(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS16_6_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS16_6_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS16_6_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS16_6_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS16_6_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS16_6_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS16_6_8(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS16_6_9(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS16_6_10(inner) => inner.get_precision(), // {python_generated}
        }
    }

    /// Returns the number of hops used for the HyperLogLog counters in the model.
    fn get_number_of_hops(&self) -> usize {
        match self {
            // InnerModel::HS{precision}_{bits}_{hops}(inner) => inner.get_number_of_hops(), {python_macro}
            InnerModel::HS4_4_1(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_4_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_4_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_4_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_4_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_4_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_4_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_4_8(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_4_9(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_4_10(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_5_1(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_5_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_5_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_5_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_5_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_5_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_5_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_5_8(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_5_9(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_5_10(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_6_1(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_6_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_6_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_6_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_6_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_6_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_6_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_6_8(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_6_9(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_6_10(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_4_1(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_4_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_4_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_4_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_4_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_4_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_4_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_4_8(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_4_9(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_4_10(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_5_1(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_5_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_5_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_5_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_5_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_5_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_5_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_5_8(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_5_9(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_5_10(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_6_1(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_6_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_6_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_6_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_6_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_6_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_6_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_6_8(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_6_9(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_6_10(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_4_1(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_4_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_4_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_4_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_4_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_4_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_4_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_4_8(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_4_9(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_4_10(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_5_1(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_5_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_5_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_5_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_5_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_5_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_5_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_5_8(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_5_9(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_5_10(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_6_1(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_6_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_6_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_6_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_6_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_6_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_6_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_6_8(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_6_9(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_6_10(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_4_1(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_4_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_4_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_4_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_4_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_4_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_4_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_4_8(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_4_9(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_4_10(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_5_1(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_5_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_5_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_5_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_5_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_5_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_5_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_5_8(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_5_9(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_5_10(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_6_1(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_6_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_6_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_6_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_6_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_6_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_6_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_6_8(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_6_9(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_6_10(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_4_1(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_4_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_4_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_4_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_4_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_4_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_4_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_4_8(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_4_9(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_4_10(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_5_1(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_5_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_5_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_5_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_5_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_5_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_5_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_5_8(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_5_9(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_5_10(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_6_1(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_6_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_6_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_6_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_6_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_6_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_6_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_6_8(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_6_9(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_6_10(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_4_1(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_4_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_4_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_4_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_4_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_4_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_4_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_4_8(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_4_9(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_4_10(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_5_1(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_5_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_5_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_5_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_5_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_5_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_5_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_5_8(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_5_9(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_5_10(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_6_1(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_6_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_6_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_6_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_6_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_6_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_6_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_6_8(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_6_9(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_6_10(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_4_1(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_4_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_4_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_4_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_4_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_4_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_4_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_4_8(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_4_9(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_4_10(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_5_1(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_5_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_5_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_5_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_5_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_5_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_5_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_5_8(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_5_9(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_5_10(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_6_1(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_6_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_6_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_6_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_6_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_6_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_6_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_6_8(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_6_9(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_6_10(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_4_1(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_4_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_4_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_4_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_4_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_4_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_4_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_4_8(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_4_9(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_4_10(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_5_1(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_5_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_5_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_5_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_5_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_5_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_5_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_5_8(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_5_9(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_5_10(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_6_1(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_6_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_6_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_6_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_6_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_6_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_6_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_6_8(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_6_9(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_6_10(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_4_1(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_4_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_4_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_4_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_4_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_4_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_4_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_4_8(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_4_9(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_4_10(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_5_1(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_5_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_5_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_5_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_5_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_5_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_5_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_5_8(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_5_9(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_5_10(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_6_1(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_6_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_6_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_6_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_6_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_6_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_6_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_6_8(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_6_9(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_6_10(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS13_4_1(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS13_4_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS13_4_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS13_4_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS13_4_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS13_4_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS13_4_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS13_4_8(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS13_4_9(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS13_4_10(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS13_5_1(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS13_5_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS13_5_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS13_5_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS13_5_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS13_5_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS13_5_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS13_5_8(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS13_5_9(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS13_5_10(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS13_6_1(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS13_6_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS13_6_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS13_6_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS13_6_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS13_6_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS13_6_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS13_6_8(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS13_6_9(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS13_6_10(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS14_4_1(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS14_4_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS14_4_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS14_4_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS14_4_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS14_4_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS14_4_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS14_4_8(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS14_4_9(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS14_4_10(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS14_5_1(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS14_5_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS14_5_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS14_5_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS14_5_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS14_5_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS14_5_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS14_5_8(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS14_5_9(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS14_5_10(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS14_6_1(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS14_6_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS14_6_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS14_6_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS14_6_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS14_6_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS14_6_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS14_6_8(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS14_6_9(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS14_6_10(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS15_4_1(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS15_4_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS15_4_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS15_4_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS15_4_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS15_4_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS15_4_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS15_4_8(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS15_4_9(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS15_4_10(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS15_5_1(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS15_5_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS15_5_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS15_5_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS15_5_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS15_5_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS15_5_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS15_5_8(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS15_5_9(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS15_5_10(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS15_6_1(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS15_6_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS15_6_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS15_6_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS15_6_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS15_6_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS15_6_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS15_6_8(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS15_6_9(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS15_6_10(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS16_4_1(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS16_4_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS16_4_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS16_4_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS16_4_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS16_4_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS16_4_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS16_4_8(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS16_4_9(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS16_4_10(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS16_5_1(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS16_5_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS16_5_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS16_5_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS16_5_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS16_5_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS16_5_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS16_5_8(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS16_5_9(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS16_5_10(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS16_6_1(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS16_6_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS16_6_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS16_6_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS16_6_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS16_6_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS16_6_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS16_6_8(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS16_6_9(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS16_6_10(inner) => inner.get_number_of_hops(), // {python_generated}
        }
    }

    /// Returns whether the features will be normalized using the symmetric Laplacian.
    fn get_normalize_by_symmetric_laplacian(&self) -> bool {
        match self {
            // InnerModel::HS{precision}_{bits}_{hops}(inner) => inner.get_normalize_by_symmetric_laplacian(), {python_macro}
            InnerModel::HS4_4_1(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_4_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_4_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_4_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_4_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_4_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_4_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_4_8(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_4_9(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_4_10(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_5_1(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_5_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_5_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_5_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_5_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_5_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_5_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_5_8(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_5_9(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_5_10(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_6_1(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_6_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_6_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_6_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_6_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_6_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_6_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_6_8(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_6_9(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_6_10(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_4_1(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_4_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_4_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_4_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_4_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_4_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_4_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_4_8(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_4_9(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_4_10(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_5_1(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_5_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_5_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_5_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_5_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_5_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_5_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_5_8(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_5_9(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_5_10(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_6_1(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_6_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_6_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_6_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_6_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_6_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_6_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_6_8(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_6_9(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_6_10(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_4_1(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_4_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_4_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_4_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_4_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_4_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_4_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_4_8(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_4_9(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_4_10(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_5_1(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_5_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_5_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_5_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_5_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_5_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_5_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_5_8(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_5_9(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_5_10(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_6_1(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_6_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_6_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_6_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_6_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_6_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_6_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_6_8(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_6_9(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_6_10(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_4_1(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_4_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_4_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_4_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_4_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_4_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_4_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_4_8(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_4_9(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_4_10(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_5_1(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_5_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_5_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_5_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_5_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_5_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_5_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_5_8(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_5_9(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_5_10(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_6_1(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_6_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_6_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_6_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_6_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_6_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_6_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_6_8(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_6_9(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_6_10(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_4_1(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_4_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_4_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_4_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_4_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_4_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_4_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_4_8(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_4_9(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_4_10(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_5_1(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_5_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_5_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_5_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_5_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_5_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_5_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_5_8(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_5_9(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_5_10(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_6_1(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_6_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_6_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_6_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_6_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_6_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_6_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_6_8(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_6_9(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_6_10(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_4_1(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_4_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_4_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_4_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_4_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_4_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_4_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_4_8(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_4_9(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_4_10(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_5_1(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_5_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_5_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_5_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_5_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_5_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_5_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_5_8(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_5_9(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_5_10(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_6_1(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_6_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_6_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_6_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_6_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_6_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_6_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_6_8(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_6_9(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_6_10(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_4_1(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_4_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_4_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_4_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_4_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_4_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_4_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_4_8(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_4_9(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_4_10(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_5_1(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_5_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_5_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_5_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_5_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_5_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_5_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_5_8(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_5_9(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_5_10(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_6_1(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_6_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_6_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_6_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_6_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_6_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_6_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_6_8(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_6_9(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_6_10(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_4_1(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_4_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_4_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_4_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_4_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_4_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_4_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_4_8(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_4_9(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_4_10(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_5_1(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_5_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_5_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_5_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_5_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_5_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_5_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_5_8(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_5_9(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_5_10(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_6_1(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_6_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_6_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_6_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_6_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_6_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_6_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_6_8(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_6_9(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_6_10(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_4_1(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_4_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_4_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_4_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_4_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_4_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_4_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_4_8(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_4_9(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_4_10(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_5_1(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_5_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_5_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_5_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_5_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_5_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_5_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_5_8(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_5_9(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_5_10(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_6_1(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_6_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_6_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_6_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_6_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_6_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_6_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_6_8(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_6_9(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_6_10(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS13_4_1(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS13_4_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS13_4_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS13_4_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS13_4_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS13_4_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS13_4_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS13_4_8(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS13_4_9(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS13_4_10(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS13_5_1(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS13_5_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS13_5_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS13_5_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS13_5_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS13_5_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS13_5_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS13_5_8(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS13_5_9(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS13_5_10(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS13_6_1(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS13_6_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS13_6_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS13_6_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS13_6_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS13_6_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS13_6_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS13_6_8(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS13_6_9(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS13_6_10(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS14_4_1(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS14_4_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS14_4_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS14_4_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS14_4_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS14_4_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS14_4_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS14_4_8(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS14_4_9(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS14_4_10(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS14_5_1(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS14_5_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS14_5_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS14_5_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS14_5_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS14_5_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS14_5_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS14_5_8(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS14_5_9(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS14_5_10(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS14_6_1(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS14_6_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS14_6_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS14_6_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS14_6_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS14_6_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS14_6_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS14_6_8(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS14_6_9(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS14_6_10(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS15_4_1(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS15_4_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS15_4_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS15_4_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS15_4_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS15_4_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS15_4_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS15_4_8(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS15_4_9(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS15_4_10(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS15_5_1(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS15_5_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS15_5_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS15_5_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS15_5_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS15_5_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS15_5_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS15_5_8(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS15_5_9(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS15_5_10(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS15_6_1(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS15_6_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS15_6_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS15_6_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS15_6_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS15_6_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS15_6_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS15_6_8(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS15_6_9(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS15_6_10(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS16_4_1(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS16_4_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS16_4_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS16_4_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS16_4_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS16_4_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS16_4_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS16_4_8(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS16_4_9(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS16_4_10(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS16_5_1(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS16_5_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS16_5_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS16_5_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS16_5_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS16_5_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS16_5_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS16_5_8(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS16_5_9(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS16_5_10(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS16_6_1(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS16_6_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS16_6_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS16_6_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS16_6_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS16_6_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS16_6_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS16_6_8(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS16_6_9(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS16_6_10(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
        }
    }

    /// Returns whether the features will be concatenated to the embeddings.
    fn get_concatenate_features(&self) -> bool {
        match self {
            // InnerModel::HS{precision}_{bits}_{hops}(inner) => inner.get_concatenate_features(), {python_macro}
            InnerModel::HS4_4_1(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_4_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_4_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_4_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_4_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_4_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_4_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_4_8(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_4_9(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_4_10(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_5_1(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_5_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_5_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_5_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_5_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_5_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_5_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_5_8(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_5_9(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_5_10(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_6_1(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_6_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_6_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_6_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_6_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_6_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_6_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_6_8(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_6_9(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_6_10(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_4_1(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_4_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_4_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_4_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_4_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_4_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_4_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_4_8(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_4_9(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_4_10(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_5_1(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_5_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_5_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_5_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_5_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_5_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_5_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_5_8(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_5_9(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_5_10(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_6_1(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_6_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_6_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_6_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_6_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_6_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_6_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_6_8(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_6_9(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_6_10(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_4_1(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_4_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_4_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_4_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_4_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_4_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_4_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_4_8(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_4_9(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_4_10(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_5_1(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_5_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_5_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_5_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_5_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_5_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_5_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_5_8(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_5_9(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_5_10(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_6_1(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_6_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_6_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_6_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_6_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_6_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_6_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_6_8(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_6_9(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_6_10(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_4_1(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_4_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_4_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_4_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_4_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_4_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_4_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_4_8(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_4_9(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_4_10(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_5_1(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_5_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_5_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_5_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_5_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_5_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_5_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_5_8(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_5_9(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_5_10(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_6_1(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_6_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_6_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_6_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_6_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_6_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_6_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_6_8(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_6_9(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_6_10(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_4_1(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_4_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_4_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_4_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_4_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_4_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_4_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_4_8(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_4_9(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_4_10(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_5_1(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_5_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_5_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_5_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_5_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_5_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_5_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_5_8(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_5_9(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_5_10(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_6_1(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_6_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_6_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_6_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_6_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_6_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_6_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_6_8(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_6_9(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_6_10(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_4_1(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_4_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_4_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_4_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_4_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_4_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_4_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_4_8(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_4_9(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_4_10(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_5_1(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_5_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_5_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_5_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_5_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_5_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_5_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_5_8(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_5_9(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_5_10(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_6_1(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_6_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_6_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_6_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_6_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_6_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_6_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_6_8(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_6_9(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_6_10(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_4_1(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_4_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_4_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_4_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_4_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_4_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_4_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_4_8(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_4_9(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_4_10(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_5_1(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_5_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_5_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_5_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_5_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_5_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_5_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_5_8(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_5_9(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_5_10(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_6_1(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_6_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_6_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_6_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_6_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_6_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_6_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_6_8(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_6_9(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_6_10(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_4_1(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_4_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_4_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_4_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_4_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_4_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_4_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_4_8(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_4_9(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_4_10(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_5_1(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_5_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_5_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_5_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_5_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_5_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_5_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_5_8(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_5_9(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_5_10(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_6_1(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_6_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_6_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_6_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_6_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_6_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_6_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_6_8(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_6_9(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_6_10(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_4_1(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_4_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_4_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_4_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_4_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_4_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_4_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_4_8(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_4_9(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_4_10(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_5_1(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_5_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_5_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_5_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_5_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_5_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_5_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_5_8(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_5_9(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_5_10(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_6_1(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_6_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_6_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_6_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_6_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_6_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_6_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_6_8(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_6_9(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_6_10(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS13_4_1(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS13_4_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS13_4_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS13_4_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS13_4_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS13_4_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS13_4_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS13_4_8(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS13_4_9(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS13_4_10(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS13_5_1(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS13_5_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS13_5_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS13_5_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS13_5_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS13_5_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS13_5_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS13_5_8(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS13_5_9(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS13_5_10(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS13_6_1(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS13_6_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS13_6_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS13_6_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS13_6_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS13_6_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS13_6_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS13_6_8(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS13_6_9(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS13_6_10(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS14_4_1(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS14_4_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS14_4_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS14_4_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS14_4_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS14_4_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS14_4_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS14_4_8(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS14_4_9(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS14_4_10(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS14_5_1(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS14_5_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS14_5_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS14_5_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS14_5_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS14_5_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS14_5_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS14_5_8(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS14_5_9(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS14_5_10(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS14_6_1(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS14_6_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS14_6_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS14_6_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS14_6_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS14_6_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS14_6_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS14_6_8(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS14_6_9(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS14_6_10(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS15_4_1(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS15_4_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS15_4_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS15_4_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS15_4_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS15_4_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS15_4_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS15_4_8(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS15_4_9(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS15_4_10(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS15_5_1(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS15_5_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS15_5_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS15_5_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS15_5_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS15_5_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS15_5_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS15_5_8(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS15_5_9(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS15_5_10(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS15_6_1(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS15_6_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS15_6_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS15_6_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS15_6_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS15_6_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS15_6_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS15_6_8(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS15_6_9(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS15_6_10(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS16_4_1(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS16_4_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS16_4_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS16_4_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS16_4_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS16_4_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS16_4_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS16_4_8(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS16_4_9(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS16_4_10(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS16_5_1(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS16_5_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS16_5_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS16_5_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS16_5_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS16_5_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS16_5_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS16_5_8(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS16_5_9(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS16_5_10(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS16_6_1(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS16_6_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS16_6_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS16_6_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS16_6_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS16_6_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS16_6_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS16_6_8(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS16_6_9(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS16_6_10(inner) => inner.get_concatenate_features(), // {python_generated}
        }
    }

    /// Returns the number of features that are used to represent each edge.
    fn get_number_of_features(&self) -> usize {
        (self.get_number_of_hops() * self.get_number_of_hops() + self.get_number_of_hops() * 2)
            * if self.get_concatenate_features() {
                2
            } else {
                1
            }
    }

    /// Returns the estimated Sketching for all edges.
    ///
    /// Parameters
    /// ------------------------
    /// support: &Graph
    ///     The graph whose topology is to be learned.
    /// edge_features_path: Option<&str>
    ///     The path where to store the estimated edge features.
    /// edge_iterator: I
    ///     The iterator over the edges to be considered.
    ///
    /// Raises
    /// ------------------------
    /// ValueError
    ///     If the provided arrays are not of the right size.
    ///     If the model has not been trained.
    ///
    unsafe fn get_sketching_from_iterator<I>(
        &self,
        support: &graph::Graph,
        edge_features_path: Option<&str>,
        edge_iterator: I,
    ) -> PyResult<Py<PyAny>>
    where
        I: IndexedParallelIterator<Item = (NodeT, NodeT)>,
    {
        let gil = Python::acquire_gil();

        let edge_feature_shape =
            MatrixShape::BiDimensional(edge_iterator.len(), self.get_number_of_features());

        match self.get_dtype() {
            "f16" => {
                let edge_features = create_memory_mapped_numpy_array(
                    gil.python(),
                    edge_features_path,
                    Dtype::F16,
                    &<MatrixShape as Into<Vec<isize>>>::into(edge_feature_shape),
                    false,
                );

                let array = edge_features.cast_as::<PyArray2<f16>>(gil.python())?;
                let edge_features_ref = unsafe { array.as_slice_mut()? };

                let edge_features_ref = unsafe {
                    core::mem::transmute::<&mut [f16], &mut [PrimitiveF16]>(edge_features_ref)
                };

                // We always use the racing version of the fit transfor
                // as we generally do not care about memory collisions.
                pe!(self.compute_sketching_from_iterator::<I, PrimitiveF16>(
                    edge_features_ref,
                    &support,
                    edge_iterator
                ))?;

                Ok(edge_features)
            }
            "f32" => {
                let edge_features = create_memory_mapped_numpy_array(
                    gil.python(),
                    edge_features_path,
                    Dtype::F32,
                    &<MatrixShape as Into<Vec<isize>>>::into(edge_feature_shape),
                    false,
                );

                let array = edge_features.cast_as::<PyArray2<f32>>(gil.python())?;
                let edge_features_ref = unsafe { array.as_slice_mut()? };

                // We always use the racing version of the fit transfor
                // as we generally do not care about memory collisions.
                pe!(self.compute_sketching_from_iterator::<I, f32>(
                    edge_features_ref,
                    &support,
                    edge_iterator
                ))?;

                Ok(edge_features)
            }
            dtype => pe!(Err(format!(
                concat!(
                    "The provided dtype {} is not supported. The supported ",
                    "data types are `f16` or `f32`."
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
    text_signature = "(*, unbiased=False, exact=False, number_of_hops=2, precision=6, bits=5, include_node_types=False, include_edge_types=False, include_edge_ids=False, include_node_ids=True, include_selfloops=True, include_typed_graphlets=False, normalize_by_symmetric_laplacian=False, concatenate_features=False, dtype=str)"
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
    /// unbiased: bool = False
    ///     Whether to use the unbiased algorithm.
    ///     By default, `false`.
    /// exact: bool = False
    ///     Whether to use the exact algorithm.
    ///     By default, `false`.
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
    /// include_node_id: bool = True
    ///     Whether to include the node id in the sketches.
    ///     By default, `true`.
    /// include_selfloops: bool = True
    ///     Whether to include the selfloops in the sketches.
    ///     By default, `false`.
    /// include_typed_graphlets: bool = False
    ///     Whether to include the typed graphlets in the sketches.
    ///     By default, `false`.
    /// random_state: int = 42
    ///     The random state to use.
    /// number_of_random_integers: usize = 0
    ///     The number of random integers to use.
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
                "unbiased",
                "exact",
                "number_of_hops",
                "precision",
                "bits",
                "include_node_types",
                "include_edge_types",
                "include_edge_ids",
                "include_node_ids",
                "include_selfloops",
                "include_typed_graphlets",
                "random_state",
                "number_of_random_integers",
                "normalize_by_symmetric_laplacian",
                "concatenate_features",
                "dtype"
            ],
        ))?;

        Ok(Self {
            inner: pe!(InnerModel::new(
                extract_value_rust_result!(kwargs, "unbiased", bool),
                extract_value_rust_result!(kwargs, "exact", bool),
                extract_value_rust_result!(kwargs, "number_of_hops", usize),
                extract_value_rust_result!(kwargs, "precision", usize).unwrap_or(6),
                extract_value_rust_result!(kwargs, "bits", usize).unwrap_or(5),
                extract_value_rust_result!(kwargs, "include_node_types", bool),
                extract_value_rust_result!(kwargs, "include_edge_types", bool),
                extract_value_rust_result!(kwargs, "include_edge_ids", bool),
                extract_value_rust_result!(kwargs, "include_node_ids", bool),
                extract_value_rust_result!(kwargs, "include_selfloops", bool),
                extract_value_rust_result!(kwargs, "include_typed_graphlets", bool),
                extract_value_rust_result!(kwargs, "random_state", u64),
                extract_value_rust_result!(kwargs, "number_of_random_integers", usize),
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
        text_signature = "($self, graph, support, edge_features, follow_graph_directionality=True)"
    )]
    /// Return numpy array with sketches for each edge in the graph.
    ///
    /// Parameters
    /// ----------------
    /// graph: Graph
    ///     The graph whose sketches are to be computed.
    /// support: Graph
    ///     The graph whose topology is to be employed.
    /// edge_features_path: Optional[str]
    ///     The path where to store the estimated edge features.
    /// follow_graph_directionality: bool = True
    ///     Whether to follow the graph directionality.
    ///     This means that if the graph is directed, the edges will be considered in the direction of the graph.
    ///
    /// Raises
    /// ----------------
    /// ValueError
    ///    If the model has not been trained.
    ///
    fn get_sketching_for_all_edges(
        &self,
        graph: &Graph,
        support: &Graph,
        edge_features_path: Option<String>,
        follow_graph_directionality: Option<bool>,
    ) -> PyResult<Py<PyAny>> {
        let follow_graph_directionality = follow_graph_directionality.unwrap_or(true);
        if graph.is_directed() || !follow_graph_directionality {
            pe!(unsafe {
                self.inner.get_sketching_from_iterator(
                    &support.inner,
                    edge_features_path.as_deref(),
                    graph
                        .inner
                        .par_iter_directed_edge_node_ids()
                        .map(|(_, src, dst)| (src, dst)),
                )
            })
        } else {
            pe!(unsafe {
                self.inner.get_sketching_from_iterator(
                    &support.inner,
                    edge_features_path.as_deref(),
                    graph.inner.par_iter_upper_triangular_edge_node_ids(),
                )
            })
        }
    }

    #[pyo3(text_signature = "($self, graph, sources, destinations, edge_features)")]
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
    /// edge_features: Optional[str]
    ///     The path where to store the estimated edge features.
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
        edge_features: Option<String>,
    ) -> PyResult<Py<PyAny>> {
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

        // We check that all the provided sources are in the graph.
        if sources_ref
            .par_iter()
            .any(|&node| node >= graph.inner.get_number_of_nodes())
        {
            return Err(PyValueError::new_err(
                "Some of the provided sources are not in the graph.",
            ));
        }

        // We check that all the provided destinations are in the graph.
        if destinations_ref
            .par_iter()
            .any(|&node| node >= graph.inner.get_number_of_nodes())
        {
            return Err(PyValueError::new_err(
                "Some of the provided destinations are not in the graph.",
            ));
        }

        pe!(unsafe {
            self.inner.get_sketching_from_iterator(
                &graph.inner,
                edge_features.as_deref(),
                sources_ref
                    .par_iter()
                    .copied()
                    .zip(destinations_ref.par_iter().copied()),
            )
        })
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
