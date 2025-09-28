use pyo3::types::PyDict;
use std::collections::HashSet;
use strsim::levenshtein;

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
    };
}

#[macro_export]
macro_rules! extract_value_rust_result {
    ($kwargs: ident, $key: literal, $_type: ty) => {
        pe!($kwargs
            .get_item($key)
            .map_or(Ok::<_, PyErr>(None), |value| {
                Ok(if value.get_type().name().unwrap() == "NoneType" {
                    None
                } else {
                    Some(pe!(value.extract::<$_type>().map_err(|_| {
                        format!(
                            "The value passed as parameter {} cannot be casted from {} to {}.",
                            $key,
                            value.get_type().name().unwrap(),
                            stringify!($_type)
                        )
                    }))?)
                })
            }))?
    };
}

#[macro_export]
macro_rules! pe {
    ($value: expr) => {
        ($value).map_err(|err| PyValueError::new_err(err))
    };
}

#[macro_export]
macro_rules! to_ndarray_1d {
    ($gil: expr, $value: expr, $_type: ty) => {
        PyArray::from_vec($gil.python(), $value)
            .cast::<$_type>(false)
            .unwrap()
            .to_owned()
    };
}

#[macro_export]
macro_rules! to_ndarray_2d {
    ($gil: expr, $value: expr, $_type: ty) => {
        PyArray::from_vec2($gil.python(), &$value)
            .unwrap()
            .cast::<$_type>(false)
            .unwrap()
            .to_owned()
    };
}

#[macro_export]
macro_rules! to_ndarray_3d {
    ($gil: expr, $value: expr, $_type: ty) => {
        PyArray::from_vec3($gil.python(), &$value)
            .unwrap()
            .cast::<$_type>(false)
            .unwrap()
            .to_owned()
    };
}

/// Return the parameters valid when building a walk parameter object.
pub fn build_walk_parameters_list<'a>(parameters: &[&'a str]) -> Vec<&'a str> {
    let default = &[
        "return_weight",
        "explore_weight",
        "change_edge_type_weight",
        "change_node_type_weight",
        "max_neighbours",
        "random_state",
        "iterations",
        "normalize_by_degree",
        "walk_length",
    ];
    default
        .iter()
        .chain(parameters).copied()
        .collect()
}

/// Validate given kwargs.
pub fn validate_kwargs(kwargs: &PyDict, columns: &[&str]) -> Result<(), String> {
    let mut keys: HashSet<String> = kwargs
        .keys()
        .iter()
        .map(|v| v.extract::<String>().unwrap())
        .collect();
    let columns: HashSet<String> = columns.iter().map(|x| x.to_string()).collect();
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
