use super::*;
use cpu_models::FeatureSlice;
use graph::{NodeT, WalksParameters, WeightT};
use numpy::PyArray2;

/// Return new walk parameters object from provided kwargs.
pub(crate) fn build_walk_parameters(kwargs: &PyDict) -> PyResult<WalksParameters> {
    let walk_length = extract_value_rust_result!(kwargs, "walk_length", u64);
    Ok(pe!(pe!(pe!(pe!(pe!(pe!(pe!(walk_length
        .map_or_else(
            || Ok(WalksParameters::default()),
            |walk_length| WalksParameters::new(walk_length),
        ))?
    .set_change_edge_type_weight(
        extract_value_rust_result!(kwargs, "change_edge_type_weight", WeightT)
    ))?
    .set_change_node_type_weight(
        extract_value_rust_result!(kwargs, "change_node_type_weight", WeightT)
    ))?
    .set_explore_weight(extract_value_rust_result!(
        kwargs,
        "explore_weight",
        WeightT
    )))?
    .set_return_weight(extract_value_rust_result!(
        kwargs,
        "return_weight",
        WeightT
    )))?
    .set_random_state(extract_value_rust_result!(kwargs, "random_state", usize))
    .set_max_neighbours(extract_value_rust_result!(
        kwargs,
        "max_neighbours",
        NodeT
    )))?
    .set_normalize_by_degree(extract_value_rust_result!(
        kwargs,
        "normalize_by_degree",
        bool
    ))
    .set_iterations(extract_value_rust_result!(
        kwargs,
        "iterations",
        NodeT
    )))?)
}

macro_rules! impl_normalize_features {
    ($($dtype:ty : $enum_dtype:ident),*) => {
        pub(crate) fn normalize_features<'a>(
            gil: &'a GILGuard,
            node_features: &'a [Py<PyAny>],
        ) -> PyResult<(Vec<NumpyArray<'a>>, Vec<usize>, Vec<FeatureSlice<'a>>)> {
            let mut numpy_references: Vec<NumpyArray> = Vec::new();
            let mut dimensions: Vec<usize> = Vec::new();
            let mut slices: Vec<FeatureSlice> = Vec::new();

            for node_feature in node_features.iter() {
                let node_feature = node_feature.as_ref(gil.python());
                $(
                    if let Ok(node_feature) = <&PyArray2<$dtype>>::extract(&node_feature) {
                        if !node_feature.is_c_contiguous(){
                            return pe!(Err(
                                concat!(
                                    "The provided vector is not a contiguos vector in ",
                                    "C orientation."
                                )
                            ));
                        }

                        dimensions.push(node_feature.shape()[1]);
                        slices.push(FeatureSlice::$enum_dtype(unsafe{node_feature.as_slice()?}));
                        numpy_references.push(NumpyArray::$enum_dtype(node_feature));

                        continue;
                    }
                )*
                return pe!(Err(concat!(
                    "The provided node features are not supported ",
                    "in the perceptron!!"
                ).to_string()));
            }
            Ok((numpy_references, dimensions, slices))
        }
    };
}

impl_normalize_features! {
    u8 : U8,
    u16 : U16,
    u32 : U32,
    u64 : U64,
    i8 : I8,
    i16 : I16,
    i32 : I32,
    i64 : I64,
    f32 : F32,
    f64 : F64
}
