extern crate edit_distance;
use edit_distance::edit_distance;
use pyo3::types::PyDict;
use std::collections::HashSet;

#[macro_export]
macro_rules! python_exception {
    ($value: expr, $msg: expr) => {
        match $value {
            Ok(v) => Ok(v),
            Err(_) => Err(PyErr::new::<exceptions::ValueError, _>($msg)),
        }
    };
}

#[macro_export]
macro_rules! pyex {
    ($value: expr) => {
        match $value {
            Ok(v) => Ok(v),
            Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
        }
    };
}

#[macro_export]
macro_rules! normalize_kwargs {
    ($kwargs: expr, $py: expr) => {
        match $kwargs {
            Some(v) => v,
            None => PyDict::new($py),
        }
    };
}

#[macro_export]
macro_rules! extract_value {
    ($kwargs: ident, $key: literal, $_type: ty) => {
        match $kwargs.get_item($key) {
            None => Ok(None),
            Some(value) => {
                if value.get_type().name() == "NoneType" {
                    Ok(None)
                } else {
                    match value.extract::<$_type>() {
                        Ok(v) => Ok(Some(v)),
                        Err(_) => Err(format!(
                            "The value passed for {} cannot be casted from {} to {}.",
                            $key,
                            value.get_type().name(),
                            stringify!($_type)
                        )),
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! to_nparray_1d {
    ($gil: expr, $value: expr, $_type: ty) => {
        python_exception!(
            PyArray::from_vec($gil.python(), $value).cast::<$_type>(false),
            format!(
                "The given array cannot be casted to {}.",
                stringify!($_type)
            )
        )?
        .to_owned()
    };
}

#[macro_export]
macro_rules! to_nparray_2d {
    ($gil: expr, $value: expr, $_type: ty) => {
        python_exception!(
            python_exception!(
                PyArray::from_vec2($gil.python(), &$value),
                "The given value cannot be casted to a 2d numpy array."
            )?
            .cast::<$_type>(false),
            format!(
                "The given 2d array cannot be casted to {}.",
                stringify!($_type)
            )
        )?
        .to_owned()
    };
}

pub fn build_walk_parameters_list(parameters: &[&str]) -> Vec<String> {
    let default = vec![
        "min_length",
        "return_weight",
        "explore_weight",
        "change_edge_type_weight",
        "change_node_type_weight",
        "random_state",
        "verbose",
        "iterations",
        "dense_node_mapping",
    ];
    default
        .iter()
        .chain(parameters.iter())
        .map(|x| x.to_string())
        .collect()
}

/// Validate given kwargs.
pub fn validate_kwargs(kwargs: &PyDict, columns: Vec<String>) -> Result<(), String> {
    let mut keys: HashSet<String> = kwargs
        .keys()
        .iter()
        .map(|v| v.extract::<String>().unwrap())
        .collect();
    let columns: HashSet<String> = columns.iter().cloned().collect();
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
            .map(|col| (edit_distance(k, col), col))
            .min_by_key(|x| x.0)
            .unwrap();

        if distance <= 2 {
            err_msg = format!(
                    "The passed argument {} is not a valid one.\n Did you mean {} ?\nThe available ones are: \n{:?}",
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
