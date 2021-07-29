use crate::types::EnsmallenGraph;
use graph::WalksParameters;
use pyo3::types::PyDict;
use shared::{NodeT, Result, WeightT};
use std::collections::HashMap;
use std::collections::HashSet;
use strsim::levenshtein;

#[macro_export]
macro_rules! normalize_kwargs {
    ($kwargs: expr, $py: expr) => {{
        use pyo3::types::PyDict;
        match $kwargs {
            Some(v) => v,
            None => PyDict::new($py),
        }
    }};
}

#[macro_export]
macro_rules! extract_value {
    ($kwargs: ident, $key: literal, $_type: ty) => {{
        use pyo3::exceptions::PyTypeError;
        use pyo3::PyErr;

        $kwargs
            .get_item($key)
            .map_or(Ok::<_, PyErr>(None), |value| {
                Ok(if value.get_type().name().unwrap() == "NoneType" {
                    None
                } else {
                    Some(value.extract::<$_type>().map_err(|_| {
                        PyTypeError::new_err(format!(
                            "The value passed as parameter {} cannot be casted from {} to {}.",
                            $key,
                            value.get_type().name().unwrap(),
                            stringify!($_type)
                        ))
                    })?)
                })
            })?
    }};
}

// TODO: create a cleaner way.
#[macro_export]
macro_rules! extract_value_rust_result {
    ($kwargs: ident, $key: literal, $_type: ty) => {
        $kwargs
            .get_item($key)
            .map_or(Ok::<_, String>(None), |value| {
                Ok(if value.get_type().name().unwrap() == "NoneType" {
                    None
                } else {
                    Some(value.extract::<$_type>().map_err(|_| {
                        format!(
                            "The value passed as parameter {} cannot be casted from {} to {}.",
                            $key,
                            value.get_type().name().unwrap(),
                            stringify!($_type)
                        )
                    })?)
                })
            })?
    };
}

#[macro_export]
macro_rules! pe {
    ($value: expr) => {{
        use pyo3::exceptions::PyValueError;
        ($value).map_err(|err| PyValueError::new_err(err))
    }};
}

#[macro_export]
macro_rules! to_ndarray_1d {
    ($gil: expr, $value: expr, $_type: ty) => {{
        use numpy::PyArray;
        PyArray::from_vec($gil.python(), $value)
            .cast::<$_type>(false)
            .unwrap()
            .to_owned()
    }};
}

#[macro_export]
macro_rules! to_ndarray_2d {
    ($gil: expr, $value: expr, $_type: ty) => {{
        use numpy::PyArray;
        PyArray::from_vec2($gil.python(), &$value)
            .unwrap()
            .cast::<$_type>(false)
            .unwrap()
            .to_owned()
    }};
}

pub fn build_walk_parameters_list<'a>(parameters: &[&'a str]) -> Vec<&'a str> {
    let default = &[
        "return_weight",
        "explore_weight",
        "change_edge_type_weight",
        "change_node_type_weight",
        "max_neighbours",
        "random_state",
        "iterations",
        "dense_node_mapping",
    ];
    default
        .into_iter()
        .chain(parameters.into_iter())
        .map(|x| *x)
        .collect()
}

/// Validate given kwargs.
pub fn validate_kwargs(kwargs: &PyDict, columns: &[&str]) -> Result<()> {
    let mut keys: HashSet<String> = kwargs
        .keys()
        .iter()
        .map(|v| v.extract::<String>().unwrap())
        .collect();
    let columns: HashSet<String> = columns.into_iter().map(|x| x.to_string()).collect();
    if keys.is_subset(&columns) {
        return Ok(());
    }

    for k in &columns {
        keys.remove(k);
    }
    let mut err_msg = String::new();
    for k in &keys {
        let (distance, column) = columns
            .iter()
            .map(|col| (levenshtein(k, col), col))
            .min_by_key(|x| x.0)
            .unwrap();

        if distance <= 2 {
            err_msg = format!(
                    "The passed argument {} is not valid.\n Did you mean {} ?\nThe available ones are: \n{:?}",
                    k, column, columns
                );
            break;
        }
    }
    if err_msg.is_empty() {
        err_msg = format!(
            "The following arguments are not valid keyword arguments for this function. \n{:?}\n the available ones are: \n{:?}",
            keys, columns
        );
    }
    Err(err_msg)
}

impl EnsmallenGraph {
    pub(crate) fn build_walk_parameters(
        &self,
        walk_length: u64,
        kwargs: &PyDict,
    ) -> Result<WalksParameters> {
        Ok(WalksParameters::new(walk_length)?
            .set_change_edge_type_weight(extract_value_rust_result!(
                kwargs,
                "change_edge_type_weight",
                WeightT
            ))?
            .set_change_node_type_weight(extract_value_rust_result!(
                kwargs,
                "change_node_type_weight",
                WeightT
            ))?
            .set_explore_weight(extract_value_rust_result!(
                kwargs,
                "explore_weight",
                WeightT
            ))?
            .set_return_weight(extract_value_rust_result!(kwargs, "return_weight", WeightT))?
            .set_random_state(extract_value_rust_result!(kwargs, "random_state", usize))
            .set_max_neighbours(extract_value_rust_result!(kwargs, "max_neighbours", NodeT))?
            .set_iterations(extract_value_rust_result!(kwargs, "iterations", NodeT))?
            .set_dense_node_mapping(
                extract_value_rust_result!(kwargs, "dense_node_mapping", HashMap<NodeT, NodeT>),
            ))
    }
}
