use super::*;
use serde::{Deserialize, Serialize};
use serde_json;
use cpu_models::HyperSketching as HS;

fn array_to_numpy_array1d<const N: usize>(array: [f32; N]) -> Result<Py<PyArray1<f32>>> {
    let gil = pyo3::Python::acquire_gil();
    let result: &PyArray1<f32> = unsafe{PyArray1::new(gil.python(), [N], false)};
    unsafe{result.as_slice_mut().map_err(|_| "Could not create a mutable slice".to_string())?.copy_from_slice(&array)};
    Ok(result.to_owned())
}

fn matrix_to_numpy_array2d<const N: usize>(matrix: [[f32; N]; N]) -> Result<Py<PyArray2<f32>>> {
    let gil = pyo3::Python::acquire_gil();
    let result: &PyArray2<f32> = unsafe{PyArray2::new(gil.python(), [N, N], false)};
    
    unsafe{std::ptr::copy_nonoverlapping(
        matrix.as_ptr() as *const f32,
        result.as_slice_mut().map_err(|_| "Could not create a mutable slice".to_string())?.as_mut_ptr(),
        N * N,
    )};

    Ok(result.to_owned())
}

#[derive(Serialize, Deserialize, Clone)]
/// HyperSketching models.
enum InnerModel {
    /// HyperSketching model.
    /// HS{precision}_{bits}_{hops}(HS<{precision}, {bits}, {hops}>), {python_macro}
    HS4_4_2(HS<4, 4, 2>), // {python_generated}
    HS4_4_3(HS<4, 4, 3>), // {python_generated}
    HS4_4_4(HS<4, 4, 4>), // {python_generated}
    HS4_4_5(HS<4, 4, 5>), // {python_generated}
    HS4_4_6(HS<4, 4, 6>), // {python_generated}
    HS4_4_7(HS<4, 4, 7>), // {python_generated}
    HS4_5_2(HS<4, 5, 2>), // {python_generated}
    HS4_5_3(HS<4, 5, 3>), // {python_generated}
    HS4_5_4(HS<4, 5, 4>), // {python_generated}
    HS4_5_5(HS<4, 5, 5>), // {python_generated}
    HS4_5_6(HS<4, 5, 6>), // {python_generated}
    HS4_5_7(HS<4, 5, 7>), // {python_generated}
    HS4_6_2(HS<4, 6, 2>), // {python_generated}
    HS4_6_3(HS<4, 6, 3>), // {python_generated}
    HS4_6_4(HS<4, 6, 4>), // {python_generated}
    HS4_6_5(HS<4, 6, 5>), // {python_generated}
    HS4_6_6(HS<4, 6, 6>), // {python_generated}
    HS4_6_7(HS<4, 6, 7>), // {python_generated}
    HS5_4_2(HS<5, 4, 2>), // {python_generated}
    HS5_4_3(HS<5, 4, 3>), // {python_generated}
    HS5_4_4(HS<5, 4, 4>), // {python_generated}
    HS5_4_5(HS<5, 4, 5>), // {python_generated}
    HS5_4_6(HS<5, 4, 6>), // {python_generated}
    HS5_4_7(HS<5, 4, 7>), // {python_generated}
    HS5_5_2(HS<5, 5, 2>), // {python_generated}
    HS5_5_3(HS<5, 5, 3>), // {python_generated}
    HS5_5_4(HS<5, 5, 4>), // {python_generated}
    HS5_5_5(HS<5, 5, 5>), // {python_generated}
    HS5_5_6(HS<5, 5, 6>), // {python_generated}
    HS5_5_7(HS<5, 5, 7>), // {python_generated}
    HS5_6_2(HS<5, 6, 2>), // {python_generated}
    HS5_6_3(HS<5, 6, 3>), // {python_generated}
    HS5_6_4(HS<5, 6, 4>), // {python_generated}
    HS5_6_5(HS<5, 6, 5>), // {python_generated}
    HS5_6_6(HS<5, 6, 6>), // {python_generated}
    HS5_6_7(HS<5, 6, 7>), // {python_generated}
    HS6_4_2(HS<6, 4, 2>), // {python_generated}
    HS6_4_3(HS<6, 4, 3>), // {python_generated}
    HS6_4_4(HS<6, 4, 4>), // {python_generated}
    HS6_4_5(HS<6, 4, 5>), // {python_generated}
    HS6_4_6(HS<6, 4, 6>), // {python_generated}
    HS6_4_7(HS<6, 4, 7>), // {python_generated}
    HS6_5_2(HS<6, 5, 2>), // {python_generated}
    HS6_5_3(HS<6, 5, 3>), // {python_generated}
    HS6_5_4(HS<6, 5, 4>), // {python_generated}
    HS6_5_5(HS<6, 5, 5>), // {python_generated}
    HS6_5_6(HS<6, 5, 6>), // {python_generated}
    HS6_5_7(HS<6, 5, 7>), // {python_generated}
    HS6_6_2(HS<6, 6, 2>), // {python_generated}
    HS6_6_3(HS<6, 6, 3>), // {python_generated}
    HS6_6_4(HS<6, 6, 4>), // {python_generated}
    HS6_6_5(HS<6, 6, 5>), // {python_generated}
    HS6_6_6(HS<6, 6, 6>), // {python_generated}
    HS6_6_7(HS<6, 6, 7>), // {python_generated}
    HS7_4_2(HS<7, 4, 2>), // {python_generated}
    HS7_4_3(HS<7, 4, 3>), // {python_generated}
    HS7_4_4(HS<7, 4, 4>), // {python_generated}
    HS7_4_5(HS<7, 4, 5>), // {python_generated}
    HS7_4_6(HS<7, 4, 6>), // {python_generated}
    HS7_4_7(HS<7, 4, 7>), // {python_generated}
    HS7_5_2(HS<7, 5, 2>), // {python_generated}
    HS7_5_3(HS<7, 5, 3>), // {python_generated}
    HS7_5_4(HS<7, 5, 4>), // {python_generated}
    HS7_5_5(HS<7, 5, 5>), // {python_generated}
    HS7_5_6(HS<7, 5, 6>), // {python_generated}
    HS7_5_7(HS<7, 5, 7>), // {python_generated}
    HS7_6_2(HS<7, 6, 2>), // {python_generated}
    HS7_6_3(HS<7, 6, 3>), // {python_generated}
    HS7_6_4(HS<7, 6, 4>), // {python_generated}
    HS7_6_5(HS<7, 6, 5>), // {python_generated}
    HS7_6_6(HS<7, 6, 6>), // {python_generated}
    HS7_6_7(HS<7, 6, 7>), // {python_generated}
    HS8_4_2(HS<8, 4, 2>), // {python_generated}
    HS8_4_3(HS<8, 4, 3>), // {python_generated}
    HS8_4_4(HS<8, 4, 4>), // {python_generated}
    HS8_4_5(HS<8, 4, 5>), // {python_generated}
    HS8_4_6(HS<8, 4, 6>), // {python_generated}
    HS8_4_7(HS<8, 4, 7>), // {python_generated}
    HS8_5_2(HS<8, 5, 2>), // {python_generated}
    HS8_5_3(HS<8, 5, 3>), // {python_generated}
    HS8_5_4(HS<8, 5, 4>), // {python_generated}
    HS8_5_5(HS<8, 5, 5>), // {python_generated}
    HS8_5_6(HS<8, 5, 6>), // {python_generated}
    HS8_5_7(HS<8, 5, 7>), // {python_generated}
    HS8_6_2(HS<8, 6, 2>), // {python_generated}
    HS8_6_3(HS<8, 6, 3>), // {python_generated}
    HS8_6_4(HS<8, 6, 4>), // {python_generated}
    HS8_6_5(HS<8, 6, 5>), // {python_generated}
    HS8_6_6(HS<8, 6, 6>), // {python_generated}
    HS8_6_7(HS<8, 6, 7>), // {python_generated}
    HS9_4_2(HS<9, 4, 2>), // {python_generated}
    HS9_4_3(HS<9, 4, 3>), // {python_generated}
    HS9_4_4(HS<9, 4, 4>), // {python_generated}
    HS9_4_5(HS<9, 4, 5>), // {python_generated}
    HS9_4_6(HS<9, 4, 6>), // {python_generated}
    HS9_4_7(HS<9, 4, 7>), // {python_generated}
    HS9_5_2(HS<9, 5, 2>), // {python_generated}
    HS9_5_3(HS<9, 5, 3>), // {python_generated}
    HS9_5_4(HS<9, 5, 4>), // {python_generated}
    HS9_5_5(HS<9, 5, 5>), // {python_generated}
    HS9_5_6(HS<9, 5, 6>), // {python_generated}
    HS9_5_7(HS<9, 5, 7>), // {python_generated}
    HS9_6_2(HS<9, 6, 2>), // {python_generated}
    HS9_6_3(HS<9, 6, 3>), // {python_generated}
    HS9_6_4(HS<9, 6, 4>), // {python_generated}
    HS9_6_5(HS<9, 6, 5>), // {python_generated}
    HS9_6_6(HS<9, 6, 6>), // {python_generated}
    HS9_6_7(HS<9, 6, 7>), // {python_generated}
    HS10_4_2(HS<10, 4, 2>), // {python_generated}
    HS10_4_3(HS<10, 4, 3>), // {python_generated}
    HS10_4_4(HS<10, 4, 4>), // {python_generated}
    HS10_4_5(HS<10, 4, 5>), // {python_generated}
    HS10_4_6(HS<10, 4, 6>), // {python_generated}
    HS10_4_7(HS<10, 4, 7>), // {python_generated}
    HS10_5_2(HS<10, 5, 2>), // {python_generated}
    HS10_5_3(HS<10, 5, 3>), // {python_generated}
    HS10_5_4(HS<10, 5, 4>), // {python_generated}
    HS10_5_5(HS<10, 5, 5>), // {python_generated}
    HS10_5_6(HS<10, 5, 6>), // {python_generated}
    HS10_5_7(HS<10, 5, 7>), // {python_generated}
    HS10_6_2(HS<10, 6, 2>), // {python_generated}
    HS10_6_3(HS<10, 6, 3>), // {python_generated}
    HS10_6_4(HS<10, 6, 4>), // {python_generated}
    HS10_6_5(HS<10, 6, 5>), // {python_generated}
    HS10_6_6(HS<10, 6, 6>), // {python_generated}
    HS10_6_7(HS<10, 6, 7>), // {python_generated}
    HS11_4_2(HS<11, 4, 2>), // {python_generated}
    HS11_4_3(HS<11, 4, 3>), // {python_generated}
    HS11_4_4(HS<11, 4, 4>), // {python_generated}
    HS11_4_5(HS<11, 4, 5>), // {python_generated}
    HS11_4_6(HS<11, 4, 6>), // {python_generated}
    HS11_4_7(HS<11, 4, 7>), // {python_generated}
    HS11_5_2(HS<11, 5, 2>), // {python_generated}
    HS11_5_3(HS<11, 5, 3>), // {python_generated}
    HS11_5_4(HS<11, 5, 4>), // {python_generated}
    HS11_5_5(HS<11, 5, 5>), // {python_generated}
    HS11_5_6(HS<11, 5, 6>), // {python_generated}
    HS11_5_7(HS<11, 5, 7>), // {python_generated}
    HS11_6_2(HS<11, 6, 2>), // {python_generated}
    HS11_6_3(HS<11, 6, 3>), // {python_generated}
    HS11_6_4(HS<11, 6, 4>), // {python_generated}
    HS11_6_5(HS<11, 6, 5>), // {python_generated}
    HS11_6_6(HS<11, 6, 6>), // {python_generated}
    HS11_6_7(HS<11, 6, 7>), // {python_generated}
    HS12_4_2(HS<12, 4, 2>), // {python_generated}
    HS12_4_3(HS<12, 4, 3>), // {python_generated}
    HS12_4_4(HS<12, 4, 4>), // {python_generated}
    HS12_4_5(HS<12, 4, 5>), // {python_generated}
    HS12_4_6(HS<12, 4, 6>), // {python_generated}
    HS12_4_7(HS<12, 4, 7>), // {python_generated}
    HS12_5_2(HS<12, 5, 2>), // {python_generated}
    HS12_5_3(HS<12, 5, 3>), // {python_generated}
    HS12_5_4(HS<12, 5, 4>), // {python_generated}
    HS12_5_5(HS<12, 5, 5>), // {python_generated}
    HS12_5_6(HS<12, 5, 6>), // {python_generated}
    HS12_5_7(HS<12, 5, 7>), // {python_generated}
    HS12_6_2(HS<12, 6, 2>), // {python_generated}
    HS12_6_3(HS<12, 6, 3>), // {python_generated}
    HS12_6_4(HS<12, 6, 4>), // {python_generated}
    HS12_6_5(HS<12, 6, 5>), // {python_generated}
    HS12_6_6(HS<12, 6, 6>), // {python_generated}
    HS12_6_7(HS<12, 6, 7>), // {python_generated}
    HS13_4_2(HS<13, 4, 2>), // {python_generated}
    HS13_4_3(HS<13, 4, 3>), // {python_generated}
    HS13_4_4(HS<13, 4, 4>), // {python_generated}
    HS13_4_5(HS<13, 4, 5>), // {python_generated}
    HS13_4_6(HS<13, 4, 6>), // {python_generated}
    HS13_4_7(HS<13, 4, 7>), // {python_generated}
    HS13_5_2(HS<13, 5, 2>), // {python_generated}
    HS13_5_3(HS<13, 5, 3>), // {python_generated}
    HS13_5_4(HS<13, 5, 4>), // {python_generated}
    HS13_5_5(HS<13, 5, 5>), // {python_generated}
    HS13_5_6(HS<13, 5, 6>), // {python_generated}
    HS13_5_7(HS<13, 5, 7>), // {python_generated}
    HS13_6_2(HS<13, 6, 2>), // {python_generated}
    HS13_6_3(HS<13, 6, 3>), // {python_generated}
    HS13_6_4(HS<13, 6, 4>), // {python_generated}
    HS13_6_5(HS<13, 6, 5>), // {python_generated}
    HS13_6_6(HS<13, 6, 6>), // {python_generated}
    HS13_6_7(HS<13, 6, 7>), // {python_generated}
    HS14_4_2(HS<14, 4, 2>), // {python_generated}
    HS14_4_3(HS<14, 4, 3>), // {python_generated}
    HS14_4_4(HS<14, 4, 4>), // {python_generated}
    HS14_4_5(HS<14, 4, 5>), // {python_generated}
    HS14_4_6(HS<14, 4, 6>), // {python_generated}
    HS14_4_7(HS<14, 4, 7>), // {python_generated}
    HS14_5_2(HS<14, 5, 2>), // {python_generated}
    HS14_5_3(HS<14, 5, 3>), // {python_generated}
    HS14_5_4(HS<14, 5, 4>), // {python_generated}
    HS14_5_5(HS<14, 5, 5>), // {python_generated}
    HS14_5_6(HS<14, 5, 6>), // {python_generated}
    HS14_5_7(HS<14, 5, 7>), // {python_generated}
    HS14_6_2(HS<14, 6, 2>), // {python_generated}
    HS14_6_3(HS<14, 6, 3>), // {python_generated}
    HS14_6_4(HS<14, 6, 4>), // {python_generated}
    HS14_6_5(HS<14, 6, 5>), // {python_generated}
    HS14_6_6(HS<14, 6, 6>), // {python_generated}
    HS14_6_7(HS<14, 6, 7>), // {python_generated}
    HS15_4_2(HS<15, 4, 2>), // {python_generated}
    HS15_4_3(HS<15, 4, 3>), // {python_generated}
    HS15_4_4(HS<15, 4, 4>), // {python_generated}
    HS15_4_5(HS<15, 4, 5>), // {python_generated}
    HS15_4_6(HS<15, 4, 6>), // {python_generated}
    HS15_4_7(HS<15, 4, 7>), // {python_generated}
    HS15_5_2(HS<15, 5, 2>), // {python_generated}
    HS15_5_3(HS<15, 5, 3>), // {python_generated}
    HS15_5_4(HS<15, 5, 4>), // {python_generated}
    HS15_5_5(HS<15, 5, 5>), // {python_generated}
    HS15_5_6(HS<15, 5, 6>), // {python_generated}
    HS15_5_7(HS<15, 5, 7>), // {python_generated}
    HS15_6_2(HS<15, 6, 2>), // {python_generated}
    HS15_6_3(HS<15, 6, 3>), // {python_generated}
    HS15_6_4(HS<15, 6, 4>), // {python_generated}
    HS15_6_5(HS<15, 6, 5>), // {python_generated}
    HS15_6_6(HS<15, 6, 6>), // {python_generated}
    HS15_6_7(HS<15, 6, 7>), // {python_generated}
    HS16_4_2(HS<16, 4, 2>), // {python_generated}
    HS16_4_3(HS<16, 4, 3>), // {python_generated}
    HS16_4_4(HS<16, 4, 4>), // {python_generated}
    HS16_4_5(HS<16, 4, 5>), // {python_generated}
    HS16_4_6(HS<16, 4, 6>), // {python_generated}
    HS16_4_7(HS<16, 4, 7>), // {python_generated}
    HS16_5_2(HS<16, 5, 2>), // {python_generated}
    HS16_5_3(HS<16, 5, 3>), // {python_generated}
    HS16_5_4(HS<16, 5, 4>), // {python_generated}
    HS16_5_5(HS<16, 5, 5>), // {python_generated}
    HS16_5_6(HS<16, 5, 6>), // {python_generated}
    HS16_5_7(HS<16, 5, 7>), // {python_generated}
    HS16_6_2(HS<16, 6, 2>), // {python_generated}
    HS16_6_3(HS<16, 6, 3>), // {python_generated}
    HS16_6_4(HS<16, 6, 4>), // {python_generated}
    HS16_6_5(HS<16, 6, 5>), // {python_generated}
    HS16_6_6(HS<16, 6, 6>), // {python_generated}
    HS16_6_7(HS<16, 6, 7>), // {python_generated}
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
    /// normalize_by_symmetric_laplacian: Option<bool>
    ///     Whether to normalize the adjacency matrix by the symmetric Laplacian.
    ///     By default, `false`.
    /// concatenate_features: Option<bool>
    ///     Whether to concatenate the features to the embeddings.
    ///     By default, `false`.
    /// 
    /// Raises
    /// ------------------------
    /// ValueError
    ///     If the provided precision is not in the supported range.
    ///     If the provided bits is not in the supported range.
    ///     The feature concatenation only makes sense if the normalization is enabled.
    fn new(
        number_of_hops: Option<usize>,
        precision: usize,
        bits: usize,
        normalize_by_symmetric_laplacian: Option<bool>,
        concatenate_features: Option<bool>,
    ) -> Result<Self> {
        // Since actually writing the code for the following match would make
        // for very hard to read code, we proceed instead with a Python script.

        match (precision, bits, number_of_hops.unwrap_or(2)) {
            // ({precision}, {bits}, {hops}) => Ok(InnerModel::HS{precision}_{bits}_{hops}(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), {python_macro}
            (4, 4, 2) => Ok(InnerModel::HS4_4_2(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (4, 4, 3) => Ok(InnerModel::HS4_4_3(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (4, 4, 4) => Ok(InnerModel::HS4_4_4(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (4, 4, 5) => Ok(InnerModel::HS4_4_5(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (4, 4, 6) => Ok(InnerModel::HS4_4_6(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (4, 4, 7) => Ok(InnerModel::HS4_4_7(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (4, 5, 2) => Ok(InnerModel::HS4_5_2(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (4, 5, 3) => Ok(InnerModel::HS4_5_3(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (4, 5, 4) => Ok(InnerModel::HS4_5_4(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (4, 5, 5) => Ok(InnerModel::HS4_5_5(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (4, 5, 6) => Ok(InnerModel::HS4_5_6(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (4, 5, 7) => Ok(InnerModel::HS4_5_7(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (4, 6, 2) => Ok(InnerModel::HS4_6_2(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (4, 6, 3) => Ok(InnerModel::HS4_6_3(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (4, 6, 4) => Ok(InnerModel::HS4_6_4(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (4, 6, 5) => Ok(InnerModel::HS4_6_5(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (4, 6, 6) => Ok(InnerModel::HS4_6_6(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (4, 6, 7) => Ok(InnerModel::HS4_6_7(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (5, 4, 2) => Ok(InnerModel::HS5_4_2(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (5, 4, 3) => Ok(InnerModel::HS5_4_3(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (5, 4, 4) => Ok(InnerModel::HS5_4_4(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (5, 4, 5) => Ok(InnerModel::HS5_4_5(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (5, 4, 6) => Ok(InnerModel::HS5_4_6(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (5, 4, 7) => Ok(InnerModel::HS5_4_7(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (5, 5, 2) => Ok(InnerModel::HS5_5_2(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (5, 5, 3) => Ok(InnerModel::HS5_5_3(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (5, 5, 4) => Ok(InnerModel::HS5_5_4(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (5, 5, 5) => Ok(InnerModel::HS5_5_5(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (5, 5, 6) => Ok(InnerModel::HS5_5_6(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (5, 5, 7) => Ok(InnerModel::HS5_5_7(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (5, 6, 2) => Ok(InnerModel::HS5_6_2(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (5, 6, 3) => Ok(InnerModel::HS5_6_3(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (5, 6, 4) => Ok(InnerModel::HS5_6_4(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (5, 6, 5) => Ok(InnerModel::HS5_6_5(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (5, 6, 6) => Ok(InnerModel::HS5_6_6(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (5, 6, 7) => Ok(InnerModel::HS5_6_7(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (6, 4, 2) => Ok(InnerModel::HS6_4_2(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (6, 4, 3) => Ok(InnerModel::HS6_4_3(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (6, 4, 4) => Ok(InnerModel::HS6_4_4(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (6, 4, 5) => Ok(InnerModel::HS6_4_5(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (6, 4, 6) => Ok(InnerModel::HS6_4_6(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (6, 4, 7) => Ok(InnerModel::HS6_4_7(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (6, 5, 2) => Ok(InnerModel::HS6_5_2(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (6, 5, 3) => Ok(InnerModel::HS6_5_3(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (6, 5, 4) => Ok(InnerModel::HS6_5_4(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (6, 5, 5) => Ok(InnerModel::HS6_5_5(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (6, 5, 6) => Ok(InnerModel::HS6_5_6(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (6, 5, 7) => Ok(InnerModel::HS6_5_7(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (6, 6, 2) => Ok(InnerModel::HS6_6_2(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (6, 6, 3) => Ok(InnerModel::HS6_6_3(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (6, 6, 4) => Ok(InnerModel::HS6_6_4(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (6, 6, 5) => Ok(InnerModel::HS6_6_5(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (6, 6, 6) => Ok(InnerModel::HS6_6_6(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (6, 6, 7) => Ok(InnerModel::HS6_6_7(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (7, 4, 2) => Ok(InnerModel::HS7_4_2(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (7, 4, 3) => Ok(InnerModel::HS7_4_3(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (7, 4, 4) => Ok(InnerModel::HS7_4_4(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (7, 4, 5) => Ok(InnerModel::HS7_4_5(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (7, 4, 6) => Ok(InnerModel::HS7_4_6(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (7, 4, 7) => Ok(InnerModel::HS7_4_7(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (7, 5, 2) => Ok(InnerModel::HS7_5_2(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (7, 5, 3) => Ok(InnerModel::HS7_5_3(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (7, 5, 4) => Ok(InnerModel::HS7_5_4(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (7, 5, 5) => Ok(InnerModel::HS7_5_5(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (7, 5, 6) => Ok(InnerModel::HS7_5_6(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (7, 5, 7) => Ok(InnerModel::HS7_5_7(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (7, 6, 2) => Ok(InnerModel::HS7_6_2(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (7, 6, 3) => Ok(InnerModel::HS7_6_3(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (7, 6, 4) => Ok(InnerModel::HS7_6_4(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (7, 6, 5) => Ok(InnerModel::HS7_6_5(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (7, 6, 6) => Ok(InnerModel::HS7_6_6(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (7, 6, 7) => Ok(InnerModel::HS7_6_7(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (8, 4, 2) => Ok(InnerModel::HS8_4_2(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (8, 4, 3) => Ok(InnerModel::HS8_4_3(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (8, 4, 4) => Ok(InnerModel::HS8_4_4(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (8, 4, 5) => Ok(InnerModel::HS8_4_5(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (8, 4, 6) => Ok(InnerModel::HS8_4_6(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (8, 4, 7) => Ok(InnerModel::HS8_4_7(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (8, 5, 2) => Ok(InnerModel::HS8_5_2(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (8, 5, 3) => Ok(InnerModel::HS8_5_3(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (8, 5, 4) => Ok(InnerModel::HS8_5_4(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (8, 5, 5) => Ok(InnerModel::HS8_5_5(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (8, 5, 6) => Ok(InnerModel::HS8_5_6(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (8, 5, 7) => Ok(InnerModel::HS8_5_7(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (8, 6, 2) => Ok(InnerModel::HS8_6_2(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (8, 6, 3) => Ok(InnerModel::HS8_6_3(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (8, 6, 4) => Ok(InnerModel::HS8_6_4(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (8, 6, 5) => Ok(InnerModel::HS8_6_5(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (8, 6, 6) => Ok(InnerModel::HS8_6_6(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (8, 6, 7) => Ok(InnerModel::HS8_6_7(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (9, 4, 2) => Ok(InnerModel::HS9_4_2(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (9, 4, 3) => Ok(InnerModel::HS9_4_3(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (9, 4, 4) => Ok(InnerModel::HS9_4_4(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (9, 4, 5) => Ok(InnerModel::HS9_4_5(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (9, 4, 6) => Ok(InnerModel::HS9_4_6(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (9, 4, 7) => Ok(InnerModel::HS9_4_7(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (9, 5, 2) => Ok(InnerModel::HS9_5_2(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (9, 5, 3) => Ok(InnerModel::HS9_5_3(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (9, 5, 4) => Ok(InnerModel::HS9_5_4(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (9, 5, 5) => Ok(InnerModel::HS9_5_5(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (9, 5, 6) => Ok(InnerModel::HS9_5_6(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (9, 5, 7) => Ok(InnerModel::HS9_5_7(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (9, 6, 2) => Ok(InnerModel::HS9_6_2(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (9, 6, 3) => Ok(InnerModel::HS9_6_3(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (9, 6, 4) => Ok(InnerModel::HS9_6_4(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (9, 6, 5) => Ok(InnerModel::HS9_6_5(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (9, 6, 6) => Ok(InnerModel::HS9_6_6(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (9, 6, 7) => Ok(InnerModel::HS9_6_7(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (10, 4, 2) => Ok(InnerModel::HS10_4_2(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (10, 4, 3) => Ok(InnerModel::HS10_4_3(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (10, 4, 4) => Ok(InnerModel::HS10_4_4(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (10, 4, 5) => Ok(InnerModel::HS10_4_5(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (10, 4, 6) => Ok(InnerModel::HS10_4_6(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (10, 4, 7) => Ok(InnerModel::HS10_4_7(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (10, 5, 2) => Ok(InnerModel::HS10_5_2(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (10, 5, 3) => Ok(InnerModel::HS10_5_3(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (10, 5, 4) => Ok(InnerModel::HS10_5_4(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (10, 5, 5) => Ok(InnerModel::HS10_5_5(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (10, 5, 6) => Ok(InnerModel::HS10_5_6(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (10, 5, 7) => Ok(InnerModel::HS10_5_7(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (10, 6, 2) => Ok(InnerModel::HS10_6_2(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (10, 6, 3) => Ok(InnerModel::HS10_6_3(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (10, 6, 4) => Ok(InnerModel::HS10_6_4(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (10, 6, 5) => Ok(InnerModel::HS10_6_5(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (10, 6, 6) => Ok(InnerModel::HS10_6_6(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (10, 6, 7) => Ok(InnerModel::HS10_6_7(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (11, 4, 2) => Ok(InnerModel::HS11_4_2(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (11, 4, 3) => Ok(InnerModel::HS11_4_3(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (11, 4, 4) => Ok(InnerModel::HS11_4_4(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (11, 4, 5) => Ok(InnerModel::HS11_4_5(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (11, 4, 6) => Ok(InnerModel::HS11_4_6(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (11, 4, 7) => Ok(InnerModel::HS11_4_7(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (11, 5, 2) => Ok(InnerModel::HS11_5_2(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (11, 5, 3) => Ok(InnerModel::HS11_5_3(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (11, 5, 4) => Ok(InnerModel::HS11_5_4(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (11, 5, 5) => Ok(InnerModel::HS11_5_5(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (11, 5, 6) => Ok(InnerModel::HS11_5_6(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (11, 5, 7) => Ok(InnerModel::HS11_5_7(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (11, 6, 2) => Ok(InnerModel::HS11_6_2(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (11, 6, 3) => Ok(InnerModel::HS11_6_3(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (11, 6, 4) => Ok(InnerModel::HS11_6_4(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (11, 6, 5) => Ok(InnerModel::HS11_6_5(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (11, 6, 6) => Ok(InnerModel::HS11_6_6(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (11, 6, 7) => Ok(InnerModel::HS11_6_7(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (12, 4, 2) => Ok(InnerModel::HS12_4_2(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (12, 4, 3) => Ok(InnerModel::HS12_4_3(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (12, 4, 4) => Ok(InnerModel::HS12_4_4(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (12, 4, 5) => Ok(InnerModel::HS12_4_5(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (12, 4, 6) => Ok(InnerModel::HS12_4_6(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (12, 4, 7) => Ok(InnerModel::HS12_4_7(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (12, 5, 2) => Ok(InnerModel::HS12_5_2(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (12, 5, 3) => Ok(InnerModel::HS12_5_3(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (12, 5, 4) => Ok(InnerModel::HS12_5_4(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (12, 5, 5) => Ok(InnerModel::HS12_5_5(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (12, 5, 6) => Ok(InnerModel::HS12_5_6(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (12, 5, 7) => Ok(InnerModel::HS12_5_7(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (12, 6, 2) => Ok(InnerModel::HS12_6_2(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (12, 6, 3) => Ok(InnerModel::HS12_6_3(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (12, 6, 4) => Ok(InnerModel::HS12_6_4(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (12, 6, 5) => Ok(InnerModel::HS12_6_5(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (12, 6, 6) => Ok(InnerModel::HS12_6_6(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (12, 6, 7) => Ok(InnerModel::HS12_6_7(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (13, 4, 2) => Ok(InnerModel::HS13_4_2(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (13, 4, 3) => Ok(InnerModel::HS13_4_3(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (13, 4, 4) => Ok(InnerModel::HS13_4_4(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (13, 4, 5) => Ok(InnerModel::HS13_4_5(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (13, 4, 6) => Ok(InnerModel::HS13_4_6(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (13, 4, 7) => Ok(InnerModel::HS13_4_7(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (13, 5, 2) => Ok(InnerModel::HS13_5_2(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (13, 5, 3) => Ok(InnerModel::HS13_5_3(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (13, 5, 4) => Ok(InnerModel::HS13_5_4(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (13, 5, 5) => Ok(InnerModel::HS13_5_5(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (13, 5, 6) => Ok(InnerModel::HS13_5_6(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (13, 5, 7) => Ok(InnerModel::HS13_5_7(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (13, 6, 2) => Ok(InnerModel::HS13_6_2(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (13, 6, 3) => Ok(InnerModel::HS13_6_3(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (13, 6, 4) => Ok(InnerModel::HS13_6_4(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (13, 6, 5) => Ok(InnerModel::HS13_6_5(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (13, 6, 6) => Ok(InnerModel::HS13_6_6(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (13, 6, 7) => Ok(InnerModel::HS13_6_7(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (14, 4, 2) => Ok(InnerModel::HS14_4_2(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (14, 4, 3) => Ok(InnerModel::HS14_4_3(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (14, 4, 4) => Ok(InnerModel::HS14_4_4(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (14, 4, 5) => Ok(InnerModel::HS14_4_5(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (14, 4, 6) => Ok(InnerModel::HS14_4_6(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (14, 4, 7) => Ok(InnerModel::HS14_4_7(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (14, 5, 2) => Ok(InnerModel::HS14_5_2(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (14, 5, 3) => Ok(InnerModel::HS14_5_3(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (14, 5, 4) => Ok(InnerModel::HS14_5_4(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (14, 5, 5) => Ok(InnerModel::HS14_5_5(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (14, 5, 6) => Ok(InnerModel::HS14_5_6(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (14, 5, 7) => Ok(InnerModel::HS14_5_7(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (14, 6, 2) => Ok(InnerModel::HS14_6_2(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (14, 6, 3) => Ok(InnerModel::HS14_6_3(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (14, 6, 4) => Ok(InnerModel::HS14_6_4(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (14, 6, 5) => Ok(InnerModel::HS14_6_5(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (14, 6, 6) => Ok(InnerModel::HS14_6_6(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (14, 6, 7) => Ok(InnerModel::HS14_6_7(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (15, 4, 2) => Ok(InnerModel::HS15_4_2(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (15, 4, 3) => Ok(InnerModel::HS15_4_3(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (15, 4, 4) => Ok(InnerModel::HS15_4_4(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (15, 4, 5) => Ok(InnerModel::HS15_4_5(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (15, 4, 6) => Ok(InnerModel::HS15_4_6(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (15, 4, 7) => Ok(InnerModel::HS15_4_7(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (15, 5, 2) => Ok(InnerModel::HS15_5_2(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (15, 5, 3) => Ok(InnerModel::HS15_5_3(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (15, 5, 4) => Ok(InnerModel::HS15_5_4(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (15, 5, 5) => Ok(InnerModel::HS15_5_5(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (15, 5, 6) => Ok(InnerModel::HS15_5_6(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (15, 5, 7) => Ok(InnerModel::HS15_5_7(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (15, 6, 2) => Ok(InnerModel::HS15_6_2(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (15, 6, 3) => Ok(InnerModel::HS15_6_3(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (15, 6, 4) => Ok(InnerModel::HS15_6_4(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (15, 6, 5) => Ok(InnerModel::HS15_6_5(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (15, 6, 6) => Ok(InnerModel::HS15_6_6(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (15, 6, 7) => Ok(InnerModel::HS15_6_7(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (16, 4, 2) => Ok(InnerModel::HS16_4_2(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (16, 4, 3) => Ok(InnerModel::HS16_4_3(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (16, 4, 4) => Ok(InnerModel::HS16_4_4(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (16, 4, 5) => Ok(InnerModel::HS16_4_5(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (16, 4, 6) => Ok(InnerModel::HS16_4_6(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (16, 4, 7) => Ok(InnerModel::HS16_4_7(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (16, 5, 2) => Ok(InnerModel::HS16_5_2(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (16, 5, 3) => Ok(InnerModel::HS16_5_3(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (16, 5, 4) => Ok(InnerModel::HS16_5_4(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (16, 5, 5) => Ok(InnerModel::HS16_5_5(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (16, 5, 6) => Ok(InnerModel::HS16_5_6(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (16, 5, 7) => Ok(InnerModel::HS16_5_7(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (16, 6, 2) => Ok(InnerModel::HS16_6_2(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (16, 6, 3) => Ok(InnerModel::HS16_6_3(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (16, 6, 4) => Ok(InnerModel::HS16_6_4(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (16, 6, 5) => Ok(InnerModel::HS16_6_5(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (16, 6, 6) => Ok(InnerModel::HS16_6_6(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
            (16, 6, 7) => Ok(InnerModel::HS16_6_7(HS::new(normalize_by_symmetric_laplacian, concatenate_features)?)), // {python_generated}
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

    /// Returns the number of bits used for the HyperLogLog counters in the model.
    fn get_bits(&self) -> usize {
        match self {
            // InnerModel::HS{precision}_{bits}_{hops}(inner) => inner.get_bits(), {python_macro}
            InnerModel::HS4_4_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_4_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_4_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_4_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_4_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_4_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_5_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_5_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_5_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_5_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_5_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_5_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_6_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_6_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_6_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_6_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_6_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS4_6_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_4_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_4_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_4_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_4_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_4_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_4_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_5_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_5_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_5_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_5_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_5_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_5_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_6_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_6_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_6_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_6_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_6_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS5_6_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_4_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_4_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_4_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_4_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_4_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_4_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_5_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_5_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_5_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_5_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_5_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_5_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_6_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_6_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_6_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_6_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_6_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS6_6_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_4_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_4_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_4_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_4_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_4_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_4_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_5_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_5_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_5_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_5_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_5_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_5_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_6_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_6_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_6_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_6_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_6_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS7_6_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_4_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_4_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_4_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_4_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_4_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_4_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_5_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_5_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_5_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_5_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_5_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_5_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_6_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_6_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_6_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_6_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_6_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS8_6_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_4_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_4_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_4_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_4_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_4_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_4_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_5_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_5_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_5_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_5_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_5_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_5_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_6_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_6_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_6_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_6_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_6_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS9_6_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_4_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_4_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_4_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_4_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_4_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_4_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_5_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_5_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_5_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_5_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_5_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_5_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_6_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_6_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_6_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_6_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_6_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS10_6_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_4_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_4_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_4_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_4_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_4_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_4_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_5_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_5_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_5_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_5_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_5_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_5_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_6_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_6_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_6_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_6_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_6_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS11_6_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_4_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_4_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_4_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_4_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_4_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_4_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_5_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_5_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_5_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_5_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_5_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_5_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_6_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_6_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_6_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_6_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_6_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS12_6_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS13_4_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS13_4_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS13_4_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS13_4_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS13_4_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS13_4_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS13_5_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS13_5_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS13_5_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS13_5_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS13_5_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS13_5_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS13_6_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS13_6_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS13_6_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS13_6_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS13_6_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS13_6_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS14_4_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS14_4_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS14_4_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS14_4_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS14_4_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS14_4_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS14_5_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS14_5_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS14_5_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS14_5_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS14_5_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS14_5_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS14_6_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS14_6_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS14_6_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS14_6_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS14_6_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS14_6_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS15_4_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS15_4_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS15_4_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS15_4_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS15_4_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS15_4_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS15_5_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS15_5_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS15_5_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS15_5_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS15_5_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS15_5_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS15_6_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS15_6_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS15_6_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS15_6_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS15_6_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS15_6_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS16_4_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS16_4_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS16_4_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS16_4_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS16_4_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS16_4_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS16_5_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS16_5_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS16_5_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS16_5_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS16_5_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS16_5_7(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS16_6_2(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS16_6_3(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS16_6_4(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS16_6_5(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS16_6_6(inner) => inner.get_bits(), // {python_generated}
            InnerModel::HS16_6_7(inner) => inner.get_bits(), // {python_generated}
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
            InnerModel::HS4_4_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_5_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_5_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_5_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_5_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_5_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_5_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_6_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_6_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_6_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_6_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_6_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS4_6_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_4_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_4_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_4_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_4_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_4_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_4_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_5_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_5_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_5_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_5_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_5_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_5_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_6_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_6_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_6_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_6_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_6_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS5_6_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_4_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_4_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_4_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_4_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_4_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_4_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_5_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_5_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_5_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_5_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_5_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_5_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_6_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_6_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_6_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_6_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_6_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS6_6_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_4_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_4_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_4_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_4_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_4_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_4_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_5_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_5_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_5_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_5_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_5_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_5_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_6_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_6_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_6_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_6_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_6_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS7_6_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_4_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_4_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_4_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_4_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_4_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_4_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_5_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_5_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_5_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_5_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_5_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_5_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_6_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_6_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_6_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_6_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_6_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS8_6_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_4_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_4_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_4_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_4_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_4_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_4_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_5_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_5_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_5_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_5_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_5_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_5_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_6_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_6_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_6_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_6_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_6_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS9_6_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_4_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_4_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_4_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_4_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_4_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_4_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_5_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_5_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_5_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_5_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_5_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_5_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_6_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_6_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_6_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_6_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_6_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS10_6_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_4_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_4_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_4_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_4_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_4_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_4_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_5_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_5_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_5_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_5_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_5_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_5_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_6_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_6_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_6_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_6_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_6_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS11_6_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_4_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_4_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_4_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_4_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_4_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_4_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_5_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_5_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_5_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_5_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_5_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_5_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_6_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_6_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_6_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_6_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_6_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS12_6_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS13_4_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS13_4_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS13_4_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS13_4_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS13_4_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS13_4_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS13_5_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS13_5_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS13_5_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS13_5_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS13_5_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS13_5_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS13_6_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS13_6_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS13_6_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS13_6_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS13_6_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS13_6_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS14_4_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS14_4_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS14_4_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS14_4_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS14_4_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS14_4_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS14_5_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS14_5_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS14_5_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS14_5_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS14_5_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS14_5_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS14_6_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS14_6_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS14_6_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS14_6_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS14_6_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS14_6_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS15_4_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS15_4_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS15_4_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS15_4_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS15_4_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS15_4_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS15_5_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS15_5_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS15_5_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS15_5_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS15_5_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS15_5_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS15_6_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS15_6_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS15_6_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS15_6_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS15_6_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS15_6_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS16_4_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS16_4_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS16_4_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS16_4_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS16_4_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS16_4_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS16_5_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS16_5_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS16_5_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS16_5_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS16_5_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS16_5_7(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS16_6_2(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS16_6_3(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS16_6_4(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS16_6_5(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS16_6_6(inner) => inner.get_precision(), // {python_generated}
            InnerModel::HS16_6_7(inner) => inner.get_precision(), // {python_generated}
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
            InnerModel::HS4_4_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_5_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_5_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_5_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_5_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_5_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_5_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_6_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_6_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_6_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_6_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_6_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS4_6_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_4_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_4_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_4_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_4_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_4_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_4_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_5_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_5_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_5_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_5_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_5_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_5_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_6_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_6_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_6_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_6_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_6_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS5_6_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_4_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_4_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_4_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_4_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_4_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_4_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_5_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_5_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_5_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_5_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_5_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_5_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_6_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_6_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_6_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_6_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_6_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS6_6_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_4_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_4_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_4_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_4_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_4_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_4_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_5_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_5_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_5_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_5_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_5_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_5_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_6_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_6_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_6_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_6_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_6_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS7_6_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_4_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_4_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_4_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_4_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_4_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_4_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_5_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_5_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_5_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_5_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_5_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_5_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_6_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_6_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_6_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_6_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_6_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS8_6_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_4_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_4_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_4_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_4_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_4_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_4_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_5_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_5_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_5_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_5_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_5_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_5_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_6_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_6_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_6_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_6_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_6_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS9_6_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_4_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_4_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_4_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_4_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_4_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_4_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_5_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_5_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_5_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_5_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_5_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_5_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_6_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_6_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_6_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_6_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_6_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS10_6_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_4_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_4_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_4_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_4_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_4_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_4_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_5_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_5_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_5_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_5_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_5_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_5_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_6_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_6_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_6_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_6_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_6_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS11_6_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_4_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_4_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_4_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_4_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_4_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_4_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_5_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_5_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_5_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_5_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_5_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_5_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_6_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_6_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_6_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_6_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_6_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS12_6_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS13_4_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS13_4_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS13_4_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS13_4_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS13_4_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS13_4_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS13_5_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS13_5_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS13_5_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS13_5_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS13_5_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS13_5_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS13_6_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS13_6_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS13_6_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS13_6_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS13_6_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS13_6_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS14_4_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS14_4_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS14_4_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS14_4_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS14_4_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS14_4_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS14_5_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS14_5_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS14_5_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS14_5_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS14_5_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS14_5_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS14_6_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS14_6_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS14_6_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS14_6_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS14_6_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS14_6_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS15_4_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS15_4_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS15_4_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS15_4_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS15_4_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS15_4_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS15_5_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS15_5_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS15_5_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS15_5_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS15_5_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS15_5_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS15_6_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS15_6_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS15_6_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS15_6_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS15_6_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS15_6_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS16_4_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS16_4_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS16_4_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS16_4_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS16_4_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS16_4_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS16_5_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS16_5_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS16_5_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS16_5_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS16_5_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS16_5_7(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS16_6_2(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS16_6_3(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS16_6_4(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS16_6_5(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS16_6_6(inner) => inner.get_number_of_hops(), // {python_generated}
            InnerModel::HS16_6_7(inner) => inner.get_number_of_hops(), // {python_generated}
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
            InnerModel::HS4_4_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_5_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_5_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_5_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_5_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_5_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_5_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_6_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_6_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_6_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_6_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_6_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS4_6_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_4_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_4_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_4_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_4_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_4_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_4_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_5_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_5_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_5_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_5_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_5_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_5_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_6_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_6_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_6_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_6_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_6_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS5_6_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_4_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_4_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_4_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_4_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_4_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_4_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_5_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_5_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_5_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_5_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_5_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_5_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_6_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_6_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_6_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_6_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_6_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS6_6_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_4_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_4_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_4_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_4_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_4_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_4_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_5_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_5_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_5_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_5_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_5_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_5_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_6_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_6_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_6_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_6_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_6_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS7_6_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_4_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_4_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_4_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_4_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_4_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_4_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_5_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_5_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_5_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_5_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_5_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_5_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_6_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_6_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_6_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_6_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_6_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS8_6_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_4_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_4_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_4_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_4_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_4_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_4_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_5_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_5_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_5_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_5_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_5_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_5_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_6_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_6_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_6_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_6_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_6_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS9_6_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_4_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_4_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_4_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_4_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_4_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_4_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_5_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_5_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_5_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_5_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_5_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_5_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_6_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_6_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_6_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_6_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_6_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS10_6_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_4_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_4_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_4_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_4_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_4_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_4_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_5_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_5_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_5_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_5_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_5_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_5_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_6_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_6_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_6_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_6_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_6_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS11_6_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_4_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_4_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_4_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_4_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_4_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_4_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_5_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_5_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_5_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_5_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_5_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_5_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_6_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_6_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_6_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_6_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_6_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS12_6_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS13_4_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS13_4_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS13_4_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS13_4_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS13_4_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS13_4_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS13_5_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS13_5_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS13_5_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS13_5_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS13_5_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS13_5_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS13_6_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS13_6_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS13_6_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS13_6_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS13_6_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS13_6_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS14_4_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS14_4_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS14_4_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS14_4_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS14_4_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS14_4_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS14_5_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS14_5_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS14_5_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS14_5_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS14_5_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS14_5_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS14_6_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS14_6_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS14_6_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS14_6_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS14_6_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS14_6_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS15_4_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS15_4_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS15_4_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS15_4_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS15_4_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS15_4_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS15_5_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS15_5_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS15_5_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS15_5_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS15_5_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS15_5_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS15_6_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS15_6_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS15_6_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS15_6_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS15_6_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS15_6_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS16_4_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS16_4_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS16_4_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS16_4_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS16_4_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS16_4_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS16_5_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS16_5_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS16_5_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS16_5_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS16_5_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS16_5_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS16_6_2(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS16_6_3(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS16_6_4(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS16_6_5(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS16_6_6(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
            InnerModel::HS16_6_7(inner) => inner.get_normalize_by_symmetric_laplacian(), // {python_generated}
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
            InnerModel::HS4_4_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_5_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_5_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_5_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_5_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_5_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_5_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_6_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_6_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_6_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_6_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_6_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS4_6_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_4_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_4_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_4_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_4_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_4_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_4_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_5_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_5_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_5_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_5_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_5_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_5_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_6_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_6_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_6_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_6_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_6_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS5_6_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_4_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_4_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_4_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_4_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_4_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_4_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_5_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_5_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_5_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_5_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_5_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_5_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_6_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_6_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_6_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_6_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_6_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS6_6_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_4_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_4_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_4_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_4_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_4_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_4_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_5_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_5_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_5_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_5_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_5_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_5_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_6_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_6_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_6_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_6_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_6_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS7_6_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_4_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_4_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_4_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_4_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_4_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_4_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_5_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_5_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_5_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_5_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_5_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_5_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_6_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_6_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_6_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_6_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_6_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS8_6_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_4_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_4_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_4_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_4_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_4_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_4_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_5_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_5_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_5_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_5_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_5_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_5_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_6_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_6_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_6_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_6_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_6_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS9_6_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_4_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_4_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_4_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_4_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_4_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_4_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_5_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_5_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_5_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_5_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_5_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_5_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_6_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_6_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_6_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_6_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_6_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS10_6_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_4_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_4_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_4_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_4_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_4_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_4_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_5_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_5_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_5_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_5_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_5_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_5_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_6_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_6_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_6_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_6_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_6_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS11_6_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_4_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_4_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_4_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_4_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_4_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_4_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_5_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_5_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_5_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_5_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_5_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_5_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_6_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_6_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_6_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_6_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_6_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS12_6_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS13_4_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS13_4_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS13_4_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS13_4_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS13_4_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS13_4_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS13_5_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS13_5_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS13_5_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS13_5_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS13_5_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS13_5_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS13_6_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS13_6_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS13_6_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS13_6_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS13_6_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS13_6_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS14_4_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS14_4_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS14_4_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS14_4_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS14_4_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS14_4_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS14_5_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS14_5_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS14_5_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS14_5_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS14_5_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS14_5_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS14_6_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS14_6_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS14_6_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS14_6_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS14_6_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS14_6_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS15_4_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS15_4_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS15_4_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS15_4_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS15_4_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS15_4_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS15_5_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS15_5_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS15_5_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS15_5_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS15_5_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS15_5_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS15_6_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS15_6_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS15_6_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS15_6_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS15_6_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS15_6_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS16_4_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS16_4_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS16_4_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS16_4_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS16_4_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS16_4_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS16_5_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS16_5_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS16_5_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS16_5_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS16_5_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS16_5_7(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS16_6_2(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS16_6_3(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS16_6_4(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS16_6_5(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS16_6_6(inner) => inner.get_concatenate_features(), // {python_generated}
            InnerModel::HS16_6_7(inner) => inner.get_concatenate_features(), // {python_generated}
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
            InnerModel::HS4_4_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_5_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_5_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_5_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_5_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_5_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_5_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_6_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_6_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_6_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_6_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_6_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS4_6_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_4_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_4_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_4_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_4_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_4_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_4_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_5_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_5_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_5_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_5_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_5_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_5_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_6_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_6_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_6_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_6_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_6_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS5_6_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_4_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_4_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_4_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_4_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_4_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_4_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_5_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_5_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_5_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_5_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_5_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_5_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_6_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_6_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_6_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_6_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_6_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS6_6_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_4_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_4_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_4_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_4_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_4_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_4_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_5_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_5_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_5_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_5_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_5_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_5_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_6_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_6_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_6_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_6_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_6_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS7_6_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_4_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_4_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_4_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_4_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_4_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_4_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_5_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_5_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_5_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_5_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_5_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_5_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_6_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_6_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_6_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_6_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_6_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS8_6_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_4_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_4_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_4_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_4_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_4_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_4_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_5_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_5_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_5_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_5_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_5_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_5_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_6_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_6_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_6_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_6_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_6_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS9_6_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_4_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_4_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_4_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_4_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_4_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_4_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_5_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_5_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_5_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_5_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_5_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_5_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_6_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_6_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_6_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_6_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_6_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS10_6_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_4_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_4_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_4_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_4_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_4_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_4_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_5_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_5_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_5_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_5_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_5_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_5_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_6_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_6_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_6_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_6_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_6_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS11_6_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_4_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_4_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_4_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_4_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_4_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_4_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_5_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_5_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_5_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_5_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_5_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_5_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_6_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_6_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_6_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_6_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_6_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS12_6_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS13_4_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS13_4_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS13_4_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS13_4_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS13_4_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS13_4_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS13_5_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS13_5_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS13_5_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS13_5_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS13_5_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS13_5_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS13_6_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS13_6_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS13_6_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS13_6_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS13_6_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS13_6_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS14_4_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS14_4_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS14_4_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS14_4_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS14_4_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS14_4_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS14_5_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS14_5_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS14_5_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS14_5_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS14_5_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS14_5_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS14_6_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS14_6_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS14_6_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS14_6_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS14_6_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS14_6_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS15_4_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS15_4_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS15_4_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS15_4_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS15_4_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS15_4_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS15_5_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS15_5_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS15_5_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS15_5_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS15_5_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS15_5_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS15_6_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS15_6_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS15_6_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS15_6_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS15_6_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS15_6_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS16_4_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS16_4_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS16_4_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS16_4_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS16_4_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS16_4_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS16_5_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS16_5_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS16_5_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS16_5_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS16_5_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS16_5_7(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS16_6_2(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS16_6_3(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS16_6_4(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS16_6_5(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS16_6_6(inner) => inner.fit(graph), // {python_generated}
            InnerModel::HS16_6_7(inner) => inner.fit(graph), // {python_generated}
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
            InnerModel::HS4_4_2(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS4_4_3(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS4_4_4(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS4_4_5(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS4_4_6(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS4_4_7(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS4_5_2(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS4_5_3(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS4_5_4(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS4_5_5(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS4_5_6(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS4_5_7(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS4_6_2(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS4_6_3(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS4_6_4(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS4_6_5(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS4_6_6(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS4_6_7(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS5_4_2(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS5_4_3(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS5_4_4(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS5_4_5(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS5_4_6(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS5_4_7(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS5_5_2(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS5_5_3(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS5_5_4(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS5_5_5(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS5_5_6(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS5_5_7(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS5_6_2(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS5_6_3(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS5_6_4(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS5_6_5(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS5_6_6(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS5_6_7(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS6_4_2(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS6_4_3(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS6_4_4(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS6_4_5(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS6_4_6(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS6_4_7(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS6_5_2(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS6_5_3(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS6_5_4(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS6_5_5(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS6_5_6(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS6_5_7(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS6_6_2(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS6_6_3(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS6_6_4(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS6_6_5(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS6_6_6(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS6_6_7(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS7_4_2(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS7_4_3(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS7_4_4(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS7_4_5(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS7_4_6(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS7_4_7(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS7_5_2(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS7_5_3(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS7_5_4(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS7_5_5(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS7_5_6(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS7_5_7(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS7_6_2(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS7_6_3(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS7_6_4(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS7_6_5(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS7_6_6(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS7_6_7(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS8_4_2(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS8_4_3(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS8_4_4(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS8_4_5(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS8_4_6(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS8_4_7(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS8_5_2(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS8_5_3(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS8_5_4(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS8_5_5(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS8_5_6(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS8_5_7(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS8_6_2(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS8_6_3(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS8_6_4(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS8_6_5(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS8_6_6(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS8_6_7(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS9_4_2(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS9_4_3(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS9_4_4(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS9_4_5(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS9_4_6(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS9_4_7(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS9_5_2(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS9_5_3(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS9_5_4(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS9_5_5(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS9_5_6(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS9_5_7(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS9_6_2(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS9_6_3(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS9_6_4(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS9_6_5(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS9_6_6(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS9_6_7(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS10_4_2(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS10_4_3(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS10_4_4(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS10_4_5(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS10_4_6(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS10_4_7(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS10_5_2(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS10_5_3(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS10_5_4(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS10_5_5(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS10_5_6(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS10_5_7(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS10_6_2(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS10_6_3(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS10_6_4(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS10_6_5(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS10_6_6(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS10_6_7(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS11_4_2(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS11_4_3(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS11_4_4(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS11_4_5(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS11_4_6(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS11_4_7(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS11_5_2(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS11_5_3(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS11_5_4(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS11_5_5(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS11_5_6(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS11_5_7(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS11_6_2(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS11_6_3(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS11_6_4(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS11_6_5(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS11_6_6(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS11_6_7(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS12_4_2(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS12_4_3(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS12_4_4(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS12_4_5(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS12_4_6(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS12_4_7(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS12_5_2(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS12_5_3(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS12_5_4(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS12_5_5(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS12_5_6(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS12_5_7(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS12_6_2(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS12_6_3(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS12_6_4(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS12_6_5(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS12_6_6(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS12_6_7(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS13_4_2(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS13_4_3(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS13_4_4(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS13_4_5(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS13_4_6(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS13_4_7(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS13_5_2(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS13_5_3(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS13_5_4(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS13_5_5(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS13_5_6(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS13_5_7(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS13_6_2(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS13_6_3(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS13_6_4(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS13_6_5(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS13_6_6(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS13_6_7(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS14_4_2(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS14_4_3(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS14_4_4(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS14_4_5(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS14_4_6(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS14_4_7(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS14_5_2(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS14_5_3(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS14_5_4(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS14_5_5(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS14_5_6(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS14_5_7(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS14_6_2(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS14_6_3(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS14_6_4(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS14_6_5(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS14_6_6(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS14_6_7(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS15_4_2(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS15_4_3(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS15_4_4(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS15_4_5(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS15_4_6(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS15_4_7(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS15_5_2(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS15_5_3(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS15_5_4(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS15_5_5(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS15_5_6(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS15_5_7(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS15_6_2(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS15_6_3(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS15_6_4(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS15_6_5(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS15_6_6(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS15_6_7(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS16_4_2(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS16_4_3(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS16_4_4(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS16_4_5(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS16_4_6(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS16_4_7(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS16_5_2(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS16_5_3(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS16_5_4(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS16_5_5(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS16_5_6(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS16_5_7(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS16_6_2(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS16_6_3(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS16_6_4(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS16_6_5(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS16_6_6(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS16_6_7(inner) => matrix_to_numpy_array2d(inner.get_overlap_cardinalities_from_node_ids(src, dst)?), // {python_generated}
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
            InnerModel::HS4_4_2(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS4_4_3(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS4_4_4(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS4_4_5(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS4_4_6(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS4_4_7(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS4_5_2(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS4_5_3(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS4_5_4(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS4_5_5(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS4_5_6(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS4_5_7(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS4_6_2(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS4_6_3(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS4_6_4(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS4_6_5(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS4_6_6(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS4_6_7(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS5_4_2(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS5_4_3(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS5_4_4(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS5_4_5(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS5_4_6(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS5_4_7(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS5_5_2(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS5_5_3(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS5_5_4(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS5_5_5(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS5_5_6(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS5_5_7(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS5_6_2(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS5_6_3(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS5_6_4(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS5_6_5(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS5_6_6(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS5_6_7(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS6_4_2(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS6_4_3(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS6_4_4(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS6_4_5(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS6_4_6(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS6_4_7(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS6_5_2(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS6_5_3(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS6_5_4(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS6_5_5(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS6_5_6(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS6_5_7(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS6_6_2(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS6_6_3(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS6_6_4(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS6_6_5(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS6_6_6(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS6_6_7(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS7_4_2(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS7_4_3(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS7_4_4(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS7_4_5(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS7_4_6(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS7_4_7(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS7_5_2(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS7_5_3(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS7_5_4(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS7_5_5(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS7_5_6(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS7_5_7(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS7_6_2(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS7_6_3(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS7_6_4(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS7_6_5(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS7_6_6(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS7_6_7(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS8_4_2(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS8_4_3(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS8_4_4(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS8_4_5(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS8_4_6(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS8_4_7(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS8_5_2(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS8_5_3(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS8_5_4(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS8_5_5(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS8_5_6(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS8_5_7(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS8_6_2(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS8_6_3(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS8_6_4(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS8_6_5(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS8_6_6(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS8_6_7(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS9_4_2(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS9_4_3(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS9_4_4(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS9_4_5(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS9_4_6(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS9_4_7(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS9_5_2(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS9_5_3(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS9_5_4(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS9_5_5(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS9_5_6(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS9_5_7(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS9_6_2(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS9_6_3(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS9_6_4(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS9_6_5(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS9_6_6(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS9_6_7(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS10_4_2(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS10_4_3(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS10_4_4(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS10_4_5(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS10_4_6(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS10_4_7(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS10_5_2(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS10_5_3(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS10_5_4(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS10_5_5(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS10_5_6(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS10_5_7(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS10_6_2(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS10_6_3(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS10_6_4(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS10_6_5(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS10_6_6(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS10_6_7(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS11_4_2(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS11_4_3(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS11_4_4(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS11_4_5(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS11_4_6(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS11_4_7(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS11_5_2(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS11_5_3(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS11_5_4(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS11_5_5(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS11_5_6(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS11_5_7(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS11_6_2(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS11_6_3(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS11_6_4(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS11_6_5(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS11_6_6(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS11_6_7(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS12_4_2(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS12_4_3(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS12_4_4(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS12_4_5(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS12_4_6(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS12_4_7(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS12_5_2(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS12_5_3(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS12_5_4(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS12_5_5(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS12_5_6(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS12_5_7(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS12_6_2(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS12_6_3(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS12_6_4(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS12_6_5(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS12_6_6(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS12_6_7(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS13_4_2(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS13_4_3(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS13_4_4(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS13_4_5(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS13_4_6(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS13_4_7(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS13_5_2(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS13_5_3(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS13_5_4(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS13_5_5(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS13_5_6(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS13_5_7(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS13_6_2(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS13_6_3(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS13_6_4(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS13_6_5(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS13_6_6(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS13_6_7(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS14_4_2(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS14_4_3(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS14_4_4(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS14_4_5(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS14_4_6(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS14_4_7(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS14_5_2(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS14_5_3(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS14_5_4(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS14_5_5(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS14_5_6(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS14_5_7(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS14_6_2(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS14_6_3(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS14_6_4(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS14_6_5(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS14_6_6(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS14_6_7(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS15_4_2(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS15_4_3(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS15_4_4(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS15_4_5(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS15_4_6(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS15_4_7(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS15_5_2(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS15_5_3(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS15_5_4(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS15_5_5(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS15_5_6(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS15_5_7(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS15_6_2(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS15_6_3(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS15_6_4(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS15_6_5(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS15_6_6(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS15_6_7(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS16_4_2(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS16_4_3(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS16_4_4(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS16_4_5(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS16_4_6(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS16_4_7(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS16_5_2(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS16_5_3(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS16_5_4(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS16_5_5(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS16_5_6(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS16_5_7(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS16_6_2(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS16_6_3(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS16_6_4(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS16_6_5(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS16_6_6(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
            InnerModel::HS16_6_7(inner) => array_to_numpy_array1d(inner.get_difference_cardinalities_from_node_ids(src, dst)?), // {python_generated}
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
    pub fn get_sketching_for_all_edges(
        &self,
        overlaps: &mut [f32],
        src_differences: &mut [f32],
        dst_differences: &mut [f32],
        graph: &graph::Graph,
    ) -> Result<()> {
        match self {
            // InnerModel::HS{precision}_{bits}_{hops}(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), {python_macro}
            InnerModel::HS4_4_2(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS4_4_3(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS4_4_4(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS4_4_5(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS4_4_6(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS4_4_7(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS4_5_2(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS4_5_3(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS4_5_4(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS4_5_5(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS4_5_6(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS4_5_7(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS4_6_2(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS4_6_3(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS4_6_4(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS4_6_5(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS4_6_6(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS4_6_7(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS5_4_2(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS5_4_3(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS5_4_4(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS5_4_5(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS5_4_6(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS5_4_7(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS5_5_2(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS5_5_3(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS5_5_4(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS5_5_5(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS5_5_6(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS5_5_7(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS5_6_2(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS5_6_3(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS5_6_4(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS5_6_5(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS5_6_6(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS5_6_7(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS6_4_2(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS6_4_3(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS6_4_4(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS6_4_5(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS6_4_6(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS6_4_7(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS6_5_2(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS6_5_3(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS6_5_4(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS6_5_5(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS6_5_6(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS6_5_7(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS6_6_2(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS6_6_3(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS6_6_4(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS6_6_5(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS6_6_6(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS6_6_7(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS7_4_2(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS7_4_3(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS7_4_4(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS7_4_5(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS7_4_6(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS7_4_7(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS7_5_2(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS7_5_3(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS7_5_4(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS7_5_5(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS7_5_6(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS7_5_7(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS7_6_2(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS7_6_3(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS7_6_4(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS7_6_5(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS7_6_6(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS7_6_7(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS8_4_2(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS8_4_3(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS8_4_4(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS8_4_5(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS8_4_6(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS8_4_7(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS8_5_2(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS8_5_3(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS8_5_4(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS8_5_5(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS8_5_6(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS8_5_7(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS8_6_2(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS8_6_3(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS8_6_4(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS8_6_5(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS8_6_6(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS8_6_7(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS9_4_2(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS9_4_3(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS9_4_4(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS9_4_5(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS9_4_6(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS9_4_7(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS9_5_2(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS9_5_3(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS9_5_4(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS9_5_5(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS9_5_6(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS9_5_7(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS9_6_2(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS9_6_3(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS9_6_4(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS9_6_5(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS9_6_6(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS9_6_7(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS10_4_2(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS10_4_3(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS10_4_4(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS10_4_5(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS10_4_6(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS10_4_7(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS10_5_2(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS10_5_3(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS10_5_4(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS10_5_5(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS10_5_6(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS10_5_7(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS10_6_2(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS10_6_3(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS10_6_4(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS10_6_5(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS10_6_6(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS10_6_7(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS11_4_2(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS11_4_3(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS11_4_4(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS11_4_5(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS11_4_6(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS11_4_7(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS11_5_2(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS11_5_3(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS11_5_4(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS11_5_5(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS11_5_6(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS11_5_7(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS11_6_2(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS11_6_3(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS11_6_4(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS11_6_5(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS11_6_6(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS11_6_7(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS12_4_2(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS12_4_3(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS12_4_4(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS12_4_5(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS12_4_6(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS12_4_7(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS12_5_2(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS12_5_3(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS12_5_4(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS12_5_5(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS12_5_6(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS12_5_7(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS12_6_2(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS12_6_3(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS12_6_4(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS12_6_5(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS12_6_6(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS12_6_7(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS13_4_2(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS13_4_3(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS13_4_4(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS13_4_5(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS13_4_6(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS13_4_7(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS13_5_2(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS13_5_3(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS13_5_4(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS13_5_5(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS13_5_6(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS13_5_7(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS13_6_2(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS13_6_3(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS13_6_4(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS13_6_5(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS13_6_6(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS13_6_7(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS14_4_2(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS14_4_3(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS14_4_4(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS14_4_5(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS14_4_6(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS14_4_7(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS14_5_2(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS14_5_3(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS14_5_4(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS14_5_5(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS14_5_6(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS14_5_7(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS14_6_2(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS14_6_3(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS14_6_4(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS14_6_5(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS14_6_6(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS14_6_7(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS15_4_2(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS15_4_3(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS15_4_4(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS15_4_5(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS15_4_6(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS15_4_7(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS15_5_2(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS15_5_3(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS15_5_4(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS15_5_5(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS15_5_6(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS15_5_7(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS15_6_2(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS15_6_3(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS15_6_4(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS15_6_5(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS15_6_6(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS15_6_7(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS16_4_2(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS16_4_3(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS16_4_4(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS16_4_5(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS16_4_6(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS16_4_7(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS16_5_2(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS16_5_3(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS16_5_4(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS16_5_5(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS16_5_6(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS16_5_7(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS16_6_2(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS16_6_3(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS16_6_4(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS16_6_5(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS16_6_6(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
            InnerModel::HS16_6_7(inner) => inner.get_sketching_for_all_edges(overlaps, src_differences, dst_differences, graph), // {python_generated}
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
#[pyo3(text_signature = "(*, number_of_hops=2, precision=6, bits=5, normalize_by_symmetric_laplacian=False, concatenate_features=False)")]
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
    /// normalize_by_symmetric_laplacian: bool = False
    ///     Whether to normalize the adjacency matrix by the symmetric Laplacian.
    ///     By default, `false`.
    /// concatenate_features: bool = False
    ///     Whether to concatenate the features to the embeddings.
    ///     By default, `false`.
    /// 
    /// Raises
    /// ------------------------
    /// ValueError
    ///     If the provided precision is not in the supported range.
    ///     If the provided bits is not in the supported range.
    ///     The feature concatenation only makes sense if the normalization is enabled.
    /// 
    pub fn new(py_kwargs: Option<&PyDict>) -> PyResult<HyperSketching> {
        let py = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, py.python());

        pe!(validate_kwargs(
            kwargs,
            &["number_of_hops", "precision", "bits", "normalize_by_symmetric_laplacian", "concatenate_features"],
        ))?;

        Ok(Self {
            inner: pe!(InnerModel::new(
                extract_value_rust_result!(kwargs, "number_of_hops", usize),
                extract_value_rust_result!(kwargs, "precision", usize).unwrap_or(6),
                extract_value_rust_result!(kwargs, "bits", usize).unwrap_or(5),
                extract_value_rust_result!(kwargs, "normalize_by_symmetric_laplacian", bool),
                extract_value_rust_result!(kwargs, "concatenate_features", bool),
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
    ///   If the model has not been trained.
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
        pe!(self.inner.get_difference_cardinalities_from_node_ids(src, dst))
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

    #[pyo3(text_signature = "($self, graph)")]
    /// Return numpy array with Jaccard coefficients for each edge in the graph.
    ///
    /// Parameters
    /// ----------------
    /// graph: Graph
    ///     The graph whose Jaccard coefficients are to be computed.
    fn get_sketching_for_all_edges(&self, graph: &Graph) -> PyResult<(Py<PyAny>, Py<PyAny>, Py<PyAny>)> {
        let gil = pyo3::Python::acquire_gil();
        if self.get_concatenate_features() {
            let overlaps = unsafe{PyArray4::new(
                gil.python(),
                [graph.get_number_of_directed_edges() as usize, 2, self.get_number_of_hops(), self.get_number_of_hops()],
                false,
            )};
            let src_differences = unsafe{PyArray3::new(
                gil.python(),
                [graph.get_number_of_directed_edges() as usize, 2, self.get_number_of_hops()],
                false,
            )};
            let dst_differences = unsafe{PyArray3::new(
                gil.python(),
                [graph.get_number_of_directed_edges() as usize, 2, self.get_number_of_hops()],
                false,
            )};

            let overlaps_ref = pe!(unsafe{overlaps.as_slice_mut()})?;
            let src_differences_ref = pe!(unsafe{src_differences.as_slice_mut()})?;
            let dst_differences_ref = pe!(unsafe{dst_differences.as_slice_mut()})?;

            pe!(self
                .inner
                .get_sketching_for_all_edges(
                    overlaps_ref,
                    src_differences_ref,
                    dst_differences_ref,
                    &graph.inner,
                ))?;
            
            Ok((
                overlaps.to_owned().into(),
                src_differences.to_owned().into(),
                dst_differences.to_owned().into(),
            ))
        } else {
            let overlaps = unsafe{PyArray3::new(
                gil.python(),
                [graph.get_number_of_directed_edges() as usize, self.get_number_of_hops(), self.get_number_of_hops()],
                false,
            )};
            let src_differences = unsafe{PyArray2::new(
                gil.python(),
                [graph.get_number_of_directed_edges() as usize, self.get_number_of_hops()],
                false,
            )};
            let dst_differences = unsafe{PyArray2::new(
                gil.python(),
                [graph.get_number_of_directed_edges() as usize, self.get_number_of_hops()],
                false,
            )};

            let overlaps_ref = pe!(unsafe{overlaps.as_slice_mut()})?;
            let src_differences_ref = pe!(unsafe{src_differences.as_slice_mut()})?;
            let dst_differences_ref = pe!(unsafe{dst_differences.as_slice_mut()})?;

            pe!(self
                .inner
                .get_sketching_for_all_edges(
                    overlaps_ref,
                    src_differences_ref,
                    dst_differences_ref,
                    &graph.inner,
                ))?;
            
            Ok((
                overlaps.to_owned().into(),
                src_differences.to_owned().into(),
                dst_differences.to_owned().into(),
            ))
        }
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
