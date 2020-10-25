#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::{wrap_pyfunction, wrap_pymodule};
mod macros {
    extern crate edit_distance;
    use edit_distance::edit_distance;
    use pyo3::types::PyDict;
    use std::collections::HashSet;
    use rayon::iter::IndexedParallelIterator;
    use rayon::prelude::*;
    use numpy::{PyArray2};
    use pyo3::prelude::*;
    struct AWEFULWORKAROUND<'a, T> {
        t: &'a PyArray2<T>,
    }
    unsafe impl<'a, T> Sync for AWEFULWORKAROUND<'a, T> {}
    pub fn iter_to_nparray_2d<'a, T: numpy::Element + Send + Sync>(
        py: Python,
        cols: usize,
        rows: usize,
        iter: impl IndexedParallelIterator<Item = Vec<T>> + 'a,
    ) -> &PyArray2<T> {
        let mut array = AWEFULWORKAROUND {
            t: PyArray2::new(py, [cols, rows], false),
        };
        unsafe {
            iter.enumerate().for_each(|(y, vy)| {
                vy.iter()
                    .enumerate()
                    .for_each(|(x, vyx)| *(array.t.uget_mut([y, x])) = vyx.clone())
            });
        }
        array.t
    }
    pub fn build_walk_parameters_list(parameters: &[&str]) -> Vec<String> {
        let default = <[_]>::into_vec(box [
            "min_length",
            "return_weight",
            "explore_weight",
            "change_edge_type_weight",
            "change_node_type_weight",
            "random_state",
            "verbose",
            "iterations",
            "dense_node_mapping",
        ]);
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
                err_msg = {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &[
                            "The passed argument ",
                            " is not a valid one.\n Did you mean ",
                            " ?\nThe available ones are: \n",
                        ],
                        &match (&k, &column, &columns) {
                            (arg0, arg1, arg2) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg2, ::core::fmt::Debug::fmt),
                            ],
                        },
                    ));
                    res
                };
                break;
            }
        }
        if err_msg.is_empty() {
            err_msg = {
                let res = :: alloc :: fmt :: format (:: core :: fmt :: Arguments :: new_v1 (& ["The following arguments are not valid keyword arguments for this function. \n" , "\n the available ones are: \n"] , & match (& keys , & columns) { (arg0 , arg1) => [:: core :: fmt :: ArgumentV1 :: new (arg0 , :: core :: fmt :: Debug :: fmt) , :: core :: fmt :: ArgumentV1 :: new (arg1 , :: core :: fmt :: Debug :: fmt)] , })) ;
                res
            };
        }
        Err(err_msg)
    }
}
pub(crate) use crate::macros::*;
mod edge_file_writer {
    use super::*;
    use graph::EdgeFileWriter;
    use pyo3::types::PyDict;
    impl EnsmallenGraph {
        /// Write to disk the edges (and optionally the metadata) of the graph.
        ///
        /// Parameters
        /// ------------------------
        /// path: str,
        ///     Path where to save the edges and their metadata.
        /// verbose: bool = True,
        ///     Wether to show a loading bar while writing to file.
        /// separator: str = "\t",
        ///     What separator to use while writing out to file.
        /// header: bool = True,
        ///     Wether to write out the header of the file.
        /// sources_column_number: int = 0,
        ///     The column number where to write out the .
        /// sources_column: str = "subject",
        ///     The name of the column where to write out the .
        /// destinations_column_number: int = 1,
        ///     The column number where to write out the .
        /// destinations_column: str = "object",
        ///     The name of the column where to write out the .
        /// edge_types_column_number: int = 2,
        ///     The column number where to write out the .
        /// edges_type_column: str = "label",
        ///     The name of the column where to write out the .
        /// weights_column_number: int = 3,
        ///     The column number where to write out the .
        /// weights_column: str = "weight",
        ///     The name of the column where to write out the .
        /// numeric_node_ids: bool = False,
        ///     Wethever to save the internal numeric Ids instead of the string names.
        /// directed: bool = False,
        ///     Wethever to save graph as directed or undirected.
        ///
        /// Raises
        /// ------------------------
        /// TODO: update the set of exceptions
        ///
        fn dump_edges(&self, path: String, py_kwargs: Option<&PyDict>) -> PyResult<()> {
            let py = pyo3::Python::acquire_gil();
            let kwargs = match py_kwargs {
                Some(v) => v,
                None => PyDict::new(py.python()),
            };
            match validate_kwargs(
                kwargs,
                [
                    "verbose",
                    "separator",
                    "header",
                    "sources_column_number",
                    "sources_column",
                    "destinations_column_number",
                    "destinations_column",
                    "weights_column_number",
                    "weights_column",
                    "edge_types_column_number",
                    "edges_type_column",
                    "numeric_node_ids",
                    "directed",
                ]
                .iter()
                .map(|x| x.to_string())
                .collect(),
            ) {
                Ok(v) => Ok(v),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }?;
            let writer = EdgeFileWriter::new(path)
                .set_verbose(match match kwargs.get_item("verbose") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<bool>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (&"verbose", &value.get_type().name(), &"bool") {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                } {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?)
                .set_separator(match match kwargs.get_item("separator") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<String>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (&"separator", &value.get_type().name(), &"String") {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                } {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?)
                .set_header(match match kwargs.get_item("header") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<bool>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (&"header", &value.get_type().name(), &"bool") {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                } {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?)
                .set_directed(match match kwargs.get_item("directed") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<bool>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (&"directed", &value.get_type().name(), &"bool") {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                } {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?)
                .set_sources_column_number(match match kwargs.get_item("sources_column_number") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<usize>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (
                                            &"sources_column_number",
                                            &value.get_type().name(),
                                            &"usize",
                                        ) {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                } {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?)
                .set_sources_column(match match kwargs.get_item("sources_column") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<String>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (
                                            &"sources_column",
                                            &value.get_type().name(),
                                            &"String",
                                        ) {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                } {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?)
                .set_destinations_column_number(match match kwargs
                    .get_item("destinations_column_number")
                {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<usize>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (
                                            &"destinations_column_number",
                                            &value.get_type().name(),
                                            &"usize",
                                        ) {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                } {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?)
                .set_destinations_column(match match kwargs.get_item("destinations_column") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<String>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (
                                            &"destinations_column",
                                            &value.get_type().name(),
                                            &"String",
                                        ) {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                } {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?)
                .set_weights_column_number(match match kwargs.get_item("weights_column_number") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<usize>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (
                                            &"weights_column_number",
                                            &value.get_type().name(),
                                            &"usize",
                                        ) {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                } {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?)
                .set_weights_column(match match kwargs.get_item("weights_column") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<String>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (
                                            &"weights_column",
                                            &value.get_type().name(),
                                            &"String",
                                        ) {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                } {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?)
                .set_edge_types_column_number(match match kwargs
                    .get_item("edge_types_column_number")
                {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<usize>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (
                                            &"edge_types_column_number",
                                            &value.get_type().name(),
                                            &"usize",
                                        ) {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                } {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?)
                .set_numeric_node_ids(match match kwargs.get_item("numeric_node_ids") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<bool>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (
                                            &"numeric_node_ids",
                                            &value.get_type().name(),
                                            &"bool",
                                        ) {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                } {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?)
                .set_edge_types_column(match match kwargs.get_item("edges_type_column") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<String>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (
                                            &"edges_type_column",
                                            &value.get_type().name(),
                                            &"String",
                                        ) {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                } {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?);
            match writer.dump(&self.graph) {
                Ok(v) => Ok(v),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }
        }
    }
    #[used]
    #[allow(non_upper_case_globals)]
    #[link_section = ".init_array"]
    #[allow(non_upper_case_globals)]
    static __init9027645084178861759: extern "C" fn() = {
        #[link_section = ".text.startup"]
        extern "C" fn __init9027645084178861759() {
            pyo3::inventory::submit({
                {
                    type Inventory =
                        <EnsmallenGraph as pyo3::class::methods::HasMethodsInventory>::Methods;
                    <Inventory as pyo3::class::methods::PyMethodsInventory>::new(<[_]>::into_vec(
                        box [pyo3::class::PyMethodDefType::Method({
                            unsafe extern "C" fn __wrap(
                                _slf: *mut pyo3::ffi::PyObject,
                                _args: *mut pyo3::ffi::PyObject,
                                _kwargs: *mut pyo3::ffi::PyObject,
                            ) -> *mut pyo3::ffi::PyObject {
                                const _LOCATION: &'static str = "EnsmallenGraph.dump_edges()";
                                {
                                    let pool = ::pyo3::GILPool::new();
                                    let unwind_safe_py =
                                        std::panic::AssertUnwindSafe(pool.python());
                                    let result = match std::panic::catch_unwind(
                                        move || -> ::pyo3::PyResult<_> {
                                            let _py = *unwind_safe_py;
                                            {
                                                let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                let _ref = _cell.try_borrow()?;
                                                let _slf = &_ref;
                                                let _args = _py
                                                    .from_borrowed_ptr::<pyo3::types::PyTuple>(
                                                        _args,
                                                    );
                                                let _kwargs: Option<&pyo3::types::PyDict> =
                                                    _py.from_borrowed_ptr_or_opt(_kwargs);
                                                pyo3::callback::convert(_py, {
                                                    const PARAMS : & 'static [pyo3 :: derive_utils :: ParamDescription] = & [pyo3 :: derive_utils :: ParamDescription { name : "path" , is_optional : false , kw_only : false , }] ;
                                                    let mut output = [None; 1usize];
                                                    let mut _args = _args;
                                                    let mut _kwargs = _kwargs;
                                                    let (_args, _kwargs) =
                                                        pyo3::derive_utils::parse_fn_args(
                                                            Some(_LOCATION),
                                                            PARAMS,
                                                            _args,
                                                            _kwargs,
                                                            false,
                                                            true,
                                                            &mut output,
                                                        )?;
                                                    let arg0 = match output [0usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                    let arg1 = _kwargs;
                                                    EnsmallenGraph::dump_edges(_slf, arg0, arg1)
                                                })
                                            }
                                        },
                                    ) {
                                        Ok(result) => result,
                                        Err(e) => {
                                            if let Some(string) = e.downcast_ref::<String>() {
                                                Err(::pyo3::panic::PanicException::new_err((
                                                    string.clone(),
                                                )))
                                            } else if let Some(s) = e.downcast_ref::<&str>() {
                                                Err(::pyo3::panic::PanicException::new_err((
                                                    s.to_string(),
                                                )))
                                            } else {
                                                Err(::pyo3::panic::PanicException::new_err((
                                                    "panic from Rust code",
                                                )))
                                            }
                                        }
                                    };
                                    result.unwrap_or_else(|e| {
                                        e.restore(pool.python());
                                        ::pyo3::callback::callback_error()
                                    })
                                }
                            }
                            pyo3 :: class :: PyMethodDef :: cfunction_with_keywords ("dump_edges\u{0}" , __wrap , 0 , "dump_edges($self, path, *, verbose, separator, header, sources_column_number, sources_column, destinations_column_number, destinations_column, weights_column_number, weights_column, edge_types_column_number, edges_type_column, numeric_node_ids, directed)\n--\n\nWrite to disk the edges (and optionally the metadata) of the graph.\n\nParameters\n------------------------\npath: str,\n    Path where to save the edges and their metadata.\nverbose: bool = True,\n    Wether to show a loading bar while writing to file.\nseparator: str = \"\\t\",\n    What separator to use while writing out to file.\nheader: bool = True,\n    Wether to write out the header of the file.\nsources_column_number: int = 0,\n    The column number where to write out the .\nsources_column: str = \"subject\",\n    The name of the column where to write out the .\ndestinations_column_number: int = 1,\n    The column number where to write out the .\ndestinations_column: str = \"object\",\n    The name of the column where to write out the .\nedge_types_column_number: int = 2,\n    The column number where to write out the .\nedges_type_column: str = \"label\",\n    The name of the column where to write out the .\nweights_column_number: int = 3,\n    The column number where to write out the .\nweights_column: str = \"weight\",\n    The name of the column where to write out the .\nnumeric_node_ids: bool = False,\n    Wethever to save the internal numeric Ids instead of the string names.\ndirected: bool = False,\n    Wethever to save graph as directed or undirected.\n\nRaises\n------------------------\nTODO: update the set of exceptions\n\u{0}")
                        })],
                    ))
                }
            });
        }
        __init9027645084178861759
    };
}
mod from_csv {
    use super::*;
    use graph::{EdgeT, Graph, NodeT};
    impl EnsmallenGraph {
        /// Return graph loaded from given edge file and optionally node file.
        ///
        /// Parameters
        /// -------------------------------
        /// edge_path: String,
        ///     The path from where load the edge file.
        /// directed: bool,
        ///     Wethever to load the graph as directed or undirected.
        /// sources_column_number: int = 0,
        ///     The column number of the sources of the edges.
        ///     This value is overwritten by the source column value if one is provided.
        ///     If the edge file you are loading does not have a header, remember
        ///     to set the edge_header parameter to false.
        /// sources_column: str = None,
        ///     Name of the column to be loaded as source of the edges.
        /// destinations_column_number: int = 1,
        ///     The column number of the destinations of the edges.
        ///     This value is overwritten by the destination column value if one is provided.
        ///     If the edge file you are loading does not have a header, remember
        ///     to set the edge_header parameter to false.
        /// destinations_column: str = None,
        ///     Name of the column to be loaded as destination of the edges.
        /// edge_types_column_number: int = None,
        ///     The column number of the edge type of the edges.
        ///     This value is overwritten by the edge types column value if one is provided.
        ///     If the edge file you are loading does not have a header, remember
        ///     to set the edge_header parameter to false.
        /// edge_types_column: str = None,
        ///     Name of the column to be loaded as edge type of the edges.
        /// default_edge_type: str = None,
        ///     String representing the default edge type to use when the edge type
        ///     in the provided column is empty.
        /// weights_column_number: int = None,
        ///     The column number of the weight of the edges.
        ///     This value is overwritten by the weights column value if one is provided.
        ///     If the edge file you are loading does not have a header, remember
        ///     to set the edge_header parameter to false.
        /// weights_column: str = None,
        ///     Name of the column to be loaded as weight of the edges.
        /// default_weight: float = None,
        ///     String representing the default edge type to use when the edge type
        ///     in the provided column is empty.
        /// skip_self_loops: bool = False,
        ///     Wethever to skip self loops while loading the edge file.
        /// ignore_duplicated_edges: bool = True,
        ///     Wethever to skip duplicated edges while loading the edge file.
        ///     When NOT ignoring the duplicated edges, an exception with information
        ///     on the duplicated edge will be raised.
        ///     When ignoring the edge type while reading the file duplicated edges
        ///     in a multi-graph will be marked as duplicates.
        /// edge_header: bool = True,
        ///     Wethever to expect the first line of the edge file to be a header.
        /// edge_rows_to_skip: int = 0,
        ///     If the edge file has some descriptive text in the first few lines,
        ///     this is the parameter that allows you to skip it.
        /// edge_separator: str = "\t",
        ///     The expected separator for the edge file.
        /// node_path: str = None,
        ///     The path from where to load the node file.
        ///     If one is not provided, no node types will be loaded and the graph
        ///     might end-up with node IDs that are not aligned with other subgraphs
        ///     from the same edge file.
        /// nodes_column_number: int = None,
        ///     The column number of the node Ids.
        ///     This value is overwritten by the nodes column value if one is provided.
        ///     If the node file you are loading does not have a header, remember
        ///     to set the node_header parameter to false.
        /// nodes_column: str = None,
        ///     Name of the column to be loaded as node Ids.
        /// node_types_column_number: int = None,
        ///     The column number of the node type of the nodes.
        ///     This value is overwritten by the node types column value if one is provided.
        ///     If the node file you are loading does not have a header, remember
        ///     to set the node_header parameter to false.
        /// node_types_column: str = None,
        ///     Name of the column to be loaded as node types.
        /// default_node_type: str = None,
        ///     String representing the default node type to use when the node type
        ///     in the provided column is empty.
        /// ignore_duplicated_nodes: bool = True,
        ///     Wethever to skip duplicated nodes while loading the node file.
        ///     When NOT ignoring the duplicated nodes, an exception with information
        ///     on the duplicated node will be raised.
        /// node_header: bool = True,
        ///     Wethever to expect the first line of the node file to be a header.
        /// node_rows_to_skip: int = 0,
        ///     If the node file has some descriptive text in the first few lines,
        ///     this is the parameter that allows you to skip it.
        /// node_separator: str = "\t",
        ///      The expected separator for the node file.
        /// numeric_node_ids: bool = False,
        ///     Wether to load the Node Ids as numeric.
        /// numeric_node_type_ids: bool = False,
        ///     Wether to load the Node Type Ids as numeric.
        /// numeric_edge_type_ids: bool = False,
        ///     Wether to load the Edge Type Ids as numeric.
        /// name: str = "Graph",
        ///     The name of the graph to use.
        /// verbose: bool = True,
        ///     Wethever to load the files verbosely, showing a loading bar.
        ///
        /// Raises
        /// ------------------------
        /// ValueError,
        ///     TODO: Update the list of raised exceptions.
        ///
        /// Returns
        /// ------------------------
        /// The loaded graph.
        fn from_unsorted_csv(
            edge_path: String,
            directed: bool,
            py_kwargs: Option<&PyDict>,
        ) -> PyResult<EnsmallenGraph> {
            let (edges, nodes, name) = match build_csv_file_reader(edge_path, py_kwargs) {
                Ok(v) => Ok(v),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }?;
            Ok(EnsmallenGraph {
                graph: match Graph::from_unsorted_csv(edges, nodes, directed, name) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?,
            })
        }
        /// Return graph loaded from given edge file and optionally node file.
        ///
        /// Parameters
        /// -------------------------------
        /// edge_path: String,
        ///     The path from where load the edge file.
        /// directed: bool,
        ///     Wethever to load the graph as directed or undirected.
        /// sources_column_number: int = 0,
        ///     The column number of the sources of the edges.
        ///     This value is overwritten by the source column value if one is provided.
        ///     If the edge file you are loading does not have a header, remember
        ///     to set the edge_header parameter to false.
        /// sources_column: str = None,
        ///     Name of the column to be loaded as source of the edges.
        /// destinations_column_number: int = 1,
        ///     The column number of the destinations of the edges.
        ///     This value is overwritten by the destination column value if one is provided.
        ///     If the edge file you are loading does not have a header, remember
        ///     to set the edge_header parameter to false.
        /// destinations_column: str = None,
        ///     Name of the column to be loaded as destination of the edges.
        /// edge_types_column_number: int = None,
        ///     The column number of the edge type of the edges.
        ///     This value is overwritten by the edge types column value if one is provided.
        ///     If the edge file you are loading does not have a header, remember
        ///     to set the edge_header parameter to false.
        /// edge_types_column: str = None,
        ///     Name of the column to be loaded as edge type of the edges.
        /// default_edge_type: str = None,
        ///     String representing the default edge type to use when the edge type
        ///     in the provided column is empty.
        /// weights_column_number: int = None,
        ///     The column number of the weight of the edges.
        ///     This value is overwritten by the weights column value if one is provided.
        ///     If the edge file you are loading does not have a header, remember
        ///     to set the edge_header parameter to false.
        /// weights_column: str = None,
        ///     Name of the column to be loaded as weight of the edges.
        /// default_weight: float = None,
        ///     String representing the default edge type to use when the edge type
        ///     in the provided column is empty.
        /// skip_self_loops: bool = False,
        ///     Wethever to skip self loops while loading the edge file.
        /// ignore_duplicated_edges: bool = True,
        ///     Wethever to skip duplicated edges while loading the edge file.
        ///     When NOT ignoring the duplicated edges, an exception with information
        ///     on the duplicated edge will be raised.
        ///     When ignoring the edge type while reading the file duplicated edges
        ///     in a multi-graph will be marked as duplicates.
        /// edge_header: bool = True,
        ///     Wethever to expect the first line of the edge file to be a header.
        /// edge_rows_to_skip: int = 0,
        ///     If the edge file has some descriptive text in the first few lines,
        ///     this is the parameter that allows you to skip it.
        /// edge_separator: str = "\t",
        ///     The expected separator for the edge file.
        /// node_path: str = None,
        ///     The path from where to load the node file.
        ///     If one is not provided, no node types will be loaded and the graph
        ///     might end-up with node IDs that are not aligned with other subgraphs
        ///     from the same edge file.
        /// nodes_column_number: int = None,
        ///     The column number of the node Ids.
        ///     This value is overwritten by the nodes column value if one is provided.
        ///     If the node file you are loading does not have a header, remember
        ///     to set the node_header parameter to false.
        /// nodes_column: str = None,
        ///     Name of the column to be loaded as node Ids.
        /// node_types_column_number: int = None,
        ///     The column number of the node type of the nodes.
        ///     This value is overwritten by the node types column value if one is provided.
        ///     If the node file you are loading does not have a header, remember
        ///     to set the node_header parameter to false.
        /// node_types_column: str = None,
        ///     Name of the column to be loaded as node types.
        /// default_node_type: str = None,
        ///     String representing the default node type to use when the node type
        ///     in the provided column is empty.
        /// ignore_duplicated_nodes: bool = True,
        ///     Wethever to skip duplicated nodes while loading the node file.
        ///     When NOT ignoring the duplicated nodes, an exception with information
        ///     on the duplicated node will be raised.
        /// node_header: bool = True,
        ///     Wethever to expect the first line of the node file to be a header.
        /// node_rows_to_skip: int = 0,
        ///     If the node file has some descriptive text in the first few lines,
        ///     this is the parameter that allows you to skip it.
        /// node_separator: str = "\t",
        ///      The expected separator for the node file.
        /// numeric_node_ids: bool = False,
        ///     Wether to load the Node Ids as numeric.
        /// numeric_node_type_ids: bool = False,
        ///     Wether to load the Node Type Ids as numeric.
        /// numeric_edge_type_ids: bool = False,
        ///     Wether to load the Edge Type Ids as numeric.
        /// name: str = "Graph",
        ///     The name of the graph to use.
        /// verbose: bool = True,
        ///     Wethever to load the files verbosely, showing a loading bar.
        ///
        /// Raises
        /// ------------------------
        /// ValueError,
        ///     TODO: Update the list of raised exceptions.
        ///
        /// Returns
        /// ------------------------
        /// The loaded graph.
        fn from_sorted_csv(
            edge_path: String,
            directed: bool,
            nodes_number: NodeT,
            edges_number: EdgeT,
            py_kwargs: Option<&PyDict>,
        ) -> PyResult<EnsmallenGraph> {
            let (edges, nodes, name) = match build_csv_file_reader(edge_path, py_kwargs) {
                Ok(v) => Ok(v),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }?;
            Ok(EnsmallenGraph {
                graph: match Graph::from_sorted_csv(
                    edges,
                    nodes,
                    directed,
                    edges_number,
                    nodes_number,
                    name,
                ) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?,
            })
        }
    }
    #[used]
    #[allow(non_upper_case_globals)]
    #[link_section = ".init_array"]
    #[allow(non_upper_case_globals)]
    static __init5747138874892135718: extern "C" fn() = {
        #[link_section = ".text.startup"]
        extern "C" fn __init5747138874892135718() {
            pyo3::inventory::submit({
                {
                    type Inventory =
                        <EnsmallenGraph as pyo3::class::methods::HasMethodsInventory>::Methods;
                    <Inventory as pyo3::class::methods::PyMethodsInventory>::new(<[_]>::into_vec(
                        box [
                            pyo3::class::PyMethodDefType::Static({
                                #[allow(unused_mut)]
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                    _kwargs: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.from_unsorted_csv()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _args = _py
                                                        .from_borrowed_ptr::<pyo3::types::PyTuple>(
                                                            _args,
                                                        );
                                                    let _kwargs: Option<&pyo3::types::PyDict> =
                                                        _py.from_borrowed_ptr_or_opt(_kwargs);
                                                    pyo3::callback::convert(_py, {
                                                        const PARAMS : & 'static [pyo3 :: derive_utils :: ParamDescription] = & [pyo3 :: derive_utils :: ParamDescription { name : "edge_path" , is_optional : false , kw_only : false , } , pyo3 :: derive_utils :: ParamDescription { name : "directed" , is_optional : false , kw_only : false , }] ;
                                                        let mut output = [None; 2usize];
                                                        let mut _args = _args;
                                                        let mut _kwargs = _kwargs;
                                                        let (_args, _kwargs) =
                                                            pyo3::derive_utils::parse_fn_args(
                                                                Some(_LOCATION),
                                                                PARAMS,
                                                                _args,
                                                                _kwargs,
                                                                false,
                                                                true,
                                                                &mut output,
                                                            )?;
                                                        let arg0 = match output [0usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        let arg1 = match output [1usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        let arg2 = _kwargs;
                                                        EnsmallenGraph::from_unsorted_csv(
                                                            arg0, arg1, arg2,
                                                        )
                                                    })
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction_with_keywords ("from_unsorted_csv\u{0}" , __wrap , pyo3 :: ffi :: METH_STATIC , "from_unsorted_csv(edge_path, directed, *, sources_column_number, sources_column, destinations_column_number, destinations_column, edge_types_column_number, edge_types_column, default_edge_type, weights_column_number, weights_column, default_weight, skip_self_loops, ignore_duplicated_edges, edge_header, edge_rows_to_skip, edge_separator, node_path, nodes_column_number, nodes_column, node_types_column_number, node_types_column, default_node_type, ignore_duplicated_nodes, node_header, node_rows_to_skip, node_separator, numeric_node_ids, numeric_node_type_ids, numeric_edge_type_ids, name, verbose)\n--\n\nReturn graph loaded from given edge file and optionally node file.\n\nParameters\n-------------------------------\nedge_path: String,\n    The path from where load the edge file.\ndirected: bool,\n    Wethever to load the graph as directed or undirected.\nsources_column_number: int = 0,\n    The column number of the sources of the edges.\n    This value is overwritten by the source column value if one is provided.\n    If the edge file you are loading does not have a header, remember\n    to set the edge_header parameter to false.\nsources_column: str = None,\n    Name of the column to be loaded as source of the edges.\ndestinations_column_number: int = 1,\n    The column number of the destinations of the edges.\n    This value is overwritten by the destination column value if one is provided.\n    If the edge file you are loading does not have a header, remember\n    to set the edge_header parameter to false.\ndestinations_column: str = None,\n    Name of the column to be loaded as destination of the edges.\nedge_types_column_number: int = None,\n    The column number of the edge type of the edges.\n    This value is overwritten by the edge types column value if one is provided.\n    If the edge file you are loading does not have a header, remember\n    to set the edge_header parameter to false.\nedge_types_column: str = None,\n    Name of the column to be loaded as edge type of the edges.\ndefault_edge_type: str = None,\n    String representing the default edge type to use when the edge type\n    in the provided column is empty.\nweights_column_number: int = None,\n    The column number of the weight of the edges.\n    This value is overwritten by the weights column value if one is provided.\n    If the edge file you are loading does not have a header, remember\n    to set the edge_header parameter to false.\nweights_column: str = None,\n    Name of the column to be loaded as weight of the edges.\ndefault_weight: float = None,\n    String representing the default edge type to use when the edge type\n    in the provided column is empty.\nskip_self_loops: bool = False,\n    Wethever to skip self loops while loading the edge file.\nignore_duplicated_edges: bool = True,\n    Wethever to skip duplicated edges while loading the edge file.\n    When NOT ignoring the duplicated edges, an exception with information\n    on the duplicated edge will be raised.\n    When ignoring the edge type while reading the file duplicated edges\n    in a multi-graph will be marked as duplicates.\nedge_header: bool = True,\n    Wethever to expect the first line of the edge file to be a header.\nedge_rows_to_skip: int = 0,\n    If the edge file has some descriptive text in the first few lines,\n    this is the parameter that allows you to skip it.\nedge_separator: str = \"\\t\",\n    The expected separator for the edge file.\nnode_path: str = None,\n    The path from where to load the node file.\n    If one is not provided, no node types will be loaded and the graph\n    might end-up with node IDs that are not aligned with other subgraphs\n    from the same edge file.\nnodes_column_number: int = None,\n    The column number of the node Ids.\n    This value is overwritten by the nodes column value if one is provided.\n    If the node file you are loading does not have a header, remember\n    to set the node_header parameter to false.\nnodes_column: str = None,\n    Name of the column to be loaded as node Ids.\nnode_types_column_number: int = None,\n    The column number of the node type of the nodes.\n    This value is overwritten by the node types column value if one is provided.\n    If the node file you are loading does not have a header, remember\n    to set the node_header parameter to false.\nnode_types_column: str = None,\n    Name of the column to be loaded as node types.\ndefault_node_type: str = None,\n    String representing the default node type to use when the node type\n    in the provided column is empty.\nignore_duplicated_nodes: bool = True,\n    Wethever to skip duplicated nodes while loading the node file.\n    When NOT ignoring the duplicated nodes, an exception with information\n    on the duplicated node will be raised.\nnode_header: bool = True,\n    Wethever to expect the first line of the node file to be a header.\nnode_rows_to_skip: int = 0,\n    If the node file has some descriptive text in the first few lines,\n    this is the parameter that allows you to skip it.\nnode_separator: str = \"\\t\",\n     The expected separator for the node file.\nnumeric_node_ids: bool = False,\n    Wether to load the Node Ids as numeric.\nnumeric_node_type_ids: bool = False,\n    Wether to load the Node Type Ids as numeric.\nnumeric_edge_type_ids: bool = False,\n    Wether to load the Edge Type Ids as numeric.\nname: str = \"Graph\",\n    The name of the graph to use.\nverbose: bool = True,\n    Wethever to load the files verbosely, showing a loading bar.\n\nRaises\n------------------------\nValueError,\n    TODO: Update the list of raised exceptions.\n\nReturns\n------------------------\nThe loaded graph.\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Static({
                                #[allow(unused_mut)]
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                    _kwargs: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.from_sorted_csv()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _args = _py
                                                        .from_borrowed_ptr::<pyo3::types::PyTuple>(
                                                            _args,
                                                        );
                                                    let _kwargs: Option<&pyo3::types::PyDict> =
                                                        _py.from_borrowed_ptr_or_opt(_kwargs);
                                                    pyo3::callback::convert(_py, {
                                                        const PARAMS : & 'static [pyo3 :: derive_utils :: ParamDescription] = & [pyo3 :: derive_utils :: ParamDescription { name : "edge_path" , is_optional : false , kw_only : false , } , pyo3 :: derive_utils :: ParamDescription { name : "directed" , is_optional : false , kw_only : false , } , pyo3 :: derive_utils :: ParamDescription { name : "nodes_number" , is_optional : false , kw_only : false , } , pyo3 :: derive_utils :: ParamDescription { name : "edges_number" , is_optional : false , kw_only : false , }] ;
                                                        let mut output = [None; 4usize];
                                                        let mut _args = _args;
                                                        let mut _kwargs = _kwargs;
                                                        let (_args, _kwargs) =
                                                            pyo3::derive_utils::parse_fn_args(
                                                                Some(_LOCATION),
                                                                PARAMS,
                                                                _args,
                                                                _kwargs,
                                                                false,
                                                                true,
                                                                &mut output,
                                                            )?;
                                                        let arg0 = match output [0usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        let arg1 = match output [1usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        let arg2 = match output [2usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        let arg3 = match output [3usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        let arg4 = _kwargs;
                                                        EnsmallenGraph::from_sorted_csv(
                                                            arg0, arg1, arg2, arg3, arg4,
                                                        )
                                                    })
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction_with_keywords ("from_sorted_csv\u{0}" , __wrap , pyo3 :: ffi :: METH_STATIC , "from_sorted_csv(edge_path, directed, *, sources_column_number, sources_column, destinations_column_number, destinations_column, edge_types_column_number, edge_types_column, default_edge_type, weights_column_number, weights_column, default_weight, skip_self_loops, ignore_duplicated_edges, edge_header, edge_rows_to_skip, edge_separator, node_path, nodes_column_number, nodes_column, node_types_column_number, node_types_column, default_node_type, ignore_duplicated_nodes, node_header, node_rows_to_skip, node_separator, numeric_node_ids, numeric_node_type_ids, numeric_edge_type_ids, name, verbose)\n--\n\nReturn graph loaded from given edge file and optionally node file.\n\nParameters\n-------------------------------\nedge_path: String,\n    The path from where load the edge file.\ndirected: bool,\n    Wethever to load the graph as directed or undirected.\nsources_column_number: int = 0,\n    The column number of the sources of the edges.\n    This value is overwritten by the source column value if one is provided.\n    If the edge file you are loading does not have a header, remember\n    to set the edge_header parameter to false.\nsources_column: str = None,\n    Name of the column to be loaded as source of the edges.\ndestinations_column_number: int = 1,\n    The column number of the destinations of the edges.\n    This value is overwritten by the destination column value if one is provided.\n    If the edge file you are loading does not have a header, remember\n    to set the edge_header parameter to false.\ndestinations_column: str = None,\n    Name of the column to be loaded as destination of the edges.\nedge_types_column_number: int = None,\n    The column number of the edge type of the edges.\n    This value is overwritten by the edge types column value if one is provided.\n    If the edge file you are loading does not have a header, remember\n    to set the edge_header parameter to false.\nedge_types_column: str = None,\n    Name of the column to be loaded as edge type of the edges.\ndefault_edge_type: str = None,\n    String representing the default edge type to use when the edge type\n    in the provided column is empty.\nweights_column_number: int = None,\n    The column number of the weight of the edges.\n    This value is overwritten by the weights column value if one is provided.\n    If the edge file you are loading does not have a header, remember\n    to set the edge_header parameter to false.\nweights_column: str = None,\n    Name of the column to be loaded as weight of the edges.\ndefault_weight: float = None,\n    String representing the default edge type to use when the edge type\n    in the provided column is empty.\nskip_self_loops: bool = False,\n    Wethever to skip self loops while loading the edge file.\nignore_duplicated_edges: bool = True,\n    Wethever to skip duplicated edges while loading the edge file.\n    When NOT ignoring the duplicated edges, an exception with information\n    on the duplicated edge will be raised.\n    When ignoring the edge type while reading the file duplicated edges\n    in a multi-graph will be marked as duplicates.\nedge_header: bool = True,\n    Wethever to expect the first line of the edge file to be a header.\nedge_rows_to_skip: int = 0,\n    If the edge file has some descriptive text in the first few lines,\n    this is the parameter that allows you to skip it.\nedge_separator: str = \"\\t\",\n    The expected separator for the edge file.\nnode_path: str = None,\n    The path from where to load the node file.\n    If one is not provided, no node types will be loaded and the graph\n    might end-up with node IDs that are not aligned with other subgraphs\n    from the same edge file.\nnodes_column_number: int = None,\n    The column number of the node Ids.\n    This value is overwritten by the nodes column value if one is provided.\n    If the node file you are loading does not have a header, remember\n    to set the node_header parameter to false.\nnodes_column: str = None,\n    Name of the column to be loaded as node Ids.\nnode_types_column_number: int = None,\n    The column number of the node type of the nodes.\n    This value is overwritten by the node types column value if one is provided.\n    If the node file you are loading does not have a header, remember\n    to set the node_header parameter to false.\nnode_types_column: str = None,\n    Name of the column to be loaded as node types.\ndefault_node_type: str = None,\n    String representing the default node type to use when the node type\n    in the provided column is empty.\nignore_duplicated_nodes: bool = True,\n    Wethever to skip duplicated nodes while loading the node file.\n    When NOT ignoring the duplicated nodes, an exception with information\n    on the duplicated node will be raised.\nnode_header: bool = True,\n    Wethever to expect the first line of the node file to be a header.\nnode_rows_to_skip: int = 0,\n    If the node file has some descriptive text in the first few lines,\n    this is the parameter that allows you to skip it.\nnode_separator: str = \"\\t\",\n     The expected separator for the node file.\nnumeric_node_ids: bool = False,\n    Wether to load the Node Ids as numeric.\nnumeric_node_type_ids: bool = False,\n    Wether to load the Node Type Ids as numeric.\nnumeric_edge_type_ids: bool = False,\n    Wether to load the Edge Type Ids as numeric.\nname: str = \"Graph\",\n    The name of the graph to use.\nverbose: bool = True,\n    Wethever to load the files verbosely, showing a loading bar.\n\nRaises\n------------------------\nValueError,\n    TODO: Update the list of raised exceptions.\n\nReturns\n------------------------\nThe loaded graph.\u{0}")
                            }),
                        ],
                    ))
                }
            });
        }
        __init5747138874892135718
    };
}
mod getters {
    use super::*;
    use graph::{EdgeT, EdgeTypeT, NodeT, NodeTypeT, WeightT};
    use numpy::{PyArray, PyArray1};
    use std::collections::HashMap;
    impl EnsmallenGraph {
        /// Return the number of nodes in the graph.
        fn get_nodes_number(&self) -> NodeT {
            self.graph.get_nodes_number()
        }
        /// Return the nodes reverse mapping.
        ///
        /// Parameters
        /// -----------------
        /// k: int,
        ///      Number of central nodes to extract.
        fn get_top_k_central_nodes(&self, k: NodeT) -> Vec<NodeT> {
            self.graph.get_top_k_central_nodes(k)
        }
        /// Return the name of the graph.
        fn get_name(&self) -> String {
            self.graph.get_name()
        }
        /// Return the number of edges in the graph.
        fn get_edges_number(&self) -> EdgeT {
            self.graph.get_edges_number()
        }
        /// Return the number of undirected edges in the graph.
        fn get_undirected_edges_number(&self) -> EdgeT {
            self.graph.get_undirected_edges_number()
        }
        /// Return the number of edges types in the graph.
        ///
        /// This method will include, if found necessary by a missing value,
        /// also the default edge type in the count of total edge types.
        ///
        fn get_edge_types_number(&self) -> EdgeTypeT {
            self.graph.get_edge_types_number()
        }
        /// Return the number of edges in the graph.
        ///
        /// This method will include, if found necessary by a missing value,
        /// also the default node type in the count of total node types.
        ///
        fn get_node_types_number(&self) -> NodeTypeT {
            self.graph.get_node_types_number()
        }
        /// Return boolean representing if given node is a trap.
        ///
        /// A trap node is a node with no outbounds edges.
        ///
        /// Parameters
        /// ---------------------
        /// node: int,
        ///     Node ID to search if it's a trap.
        ///
        /// Returns
        /// ----------------------------
        /// Boolean representing if given node is a trap.
        ///
        fn is_node_trap(&self, node: NodeT) -> bool {
            self.graph.is_node_trap(node)
        }
        /// Return boolean representing singletons.
        ///
        /// Parameters
        /// ---------------------
        /// node_id: int,
        ///     Node ID to search if it's a singleton.
        ///
        /// Returns
        /// ----------------------------
        /// Boolean representing if given node is a singleton.
        ///
        fn is_singleton(&self, node_id: NodeT) -> bool {
            self.graph.is_singleton(node_id)
        }
        /// Return boolean representing singletons.
        ///
        /// Parameters
        /// ---------------------
        /// node_name: str,
        ///     Node name to search if it's a singleton.
        ///
        /// Returns
        /// ----------------------------
        /// Boolean representing if given node is a singleton.
        ///
        fn is_singleton_string(&self, node_name: &str) -> PyResult<bool> {
            match self.graph.is_singleton_string(node_name) {
                Ok(v) => Ok(v),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }
        }
        /// Return boolean representing if given edge is a trap.
        ///
        /// A trap edge is a edge with a destination node that is a trap node.
        ///
        /// Parameters
        /// ---------------------
        /// node: int,
        ///     Node ID to search if it's a trap.
        ///
        /// Returns
        /// ----------------------------
        /// Boolean representing if given edge is a trap.
        ///
        fn is_edge_trap(&self, edge: EdgeT) -> bool {
            self.graph.is_edge_trap(edge)
        }
        /// Return boolean representing if given edge exists in graph.
        ///
        /// Parameters
        /// ---------------------
        /// src: int,
        ///     Node ID to use as source of given edge.
        /// dst: int,
        ///     Node ID to use as destination of given edge.
        /// edge_type: Union[None, int],
        ///     Edge type ID. (By deafult is None).
        ///
        /// Returns
        /// ----------------------------
        /// Boolean representing if given edge exists in graph.
        ///
        fn has_edge(&self, src: NodeT, dst: NodeT, edge_type: Option<EdgeTypeT>) -> bool {
            self.graph.has_edge(src, dst, edge_type)
        }
        /// Return boolean representing if given edge exists in graph.
        ///
        /// Parameters
        /// ---------------------
        /// src: str,
        ///     Node name to use as source of given edge.
        /// dst: str,
        ///     Node name to use as destination of given edge.
        /// edge_type: Union[None, str],
        ///     Edge type name. (By deafult is None).
        ///
        /// Returns
        /// ----------------------------
        /// Boolean representing if given edge exists in graph.
        ///
        fn has_edge_string(&self, src: &str, dst: &str, edge_type: Option<String>) -> bool {
            self.graph.has_edge_string(&src, &dst, edge_type.as_ref())
        }
        /// Return boolean representing if given node exists in graph.
        ///
        /// Parameters
        /// ---------------------
        /// node_name: str,
        ///     Name of the node.
        /// node_type: str = None,
        ///     Optional node type of the node.
        ///
        /// Returns
        /// ----------------------------
        /// Boolean representing if given node exists in graph.
        ///
        fn has_node_string(&self, node_name: &str, node_type: Option<String>) -> bool {
            self.graph.has_node_string(node_name, node_type)
        }
        /// Return integer representing ID of the edge.
        ///
        /// Parameters
        /// ---------------------
        /// src: int,
        ///     Node ID to use as source of given edge.
        /// dst: int,
        ///     Node ID to use as destination of given edge.
        /// edge_type: Union[None, int],
        ///     Edge type ID. (By deafult is None).
        ///
        /// Returns
        /// ----------------------------
        /// Integer representing ID of the edge. It will return None when the edge does not exist.
        ///
        fn get_edge_id(
            &self,
            src: NodeT,
            dst: NodeT,
            edge_type: Option<EdgeTypeT>,
        ) -> Option<EdgeT> {
            self.graph.get_edge_id(src, dst, edge_type)
        }
        /// Return integer representing ID of the edge.
        ///
        /// Parameters
        /// ---------------------
        /// src: str,
        ///     Node name to use as source of given edge.
        /// dst: str,
        ///     Node name to use as destination of given edge.
        /// edge_type: Union[None, str],
        ///     Edge type name. (By deafult is None).
        ///
        /// Returns
        /// ----------------------------
        /// Integer representing ID of the edge. It will return None when the edge does not exist.
        ///
        fn get_edge_id_string(
            &self,
            src: &str,
            dst: &str,
            edge_type: Option<String>,
        ) -> Option<EdgeT> {
            self.graph.get_edge_id_string(src, dst, edge_type.as_ref())
        }
        /// Return mapping from instance not trap nodes to dense range of nodes.
        ///
        /// Returns
        /// ----------------------------
        /// Dict with mapping from not trap nodes to dense range of nodes.
        ///
        fn get_dense_node_mapping(&self) -> HashMap<NodeT, NodeT> {
            self.graph.get_dense_node_mapping()
        }
        /// Return the number of source nodes.
        ///
        /// Returns
        /// ----------------------------
        /// Number of the source nodes.
        ///
        fn get_source_nodes_number(&self) -> NodeT {
            self.graph.get_source_nodes_number()
        }
        /// Return vector of the non-unique source nodes.
        pub fn get_sources(&self) -> PyResult<Py<PyArray1<NodeT>>> {
            let gil = pyo3::Python::acquire_gil();
            Ok (match PyArray :: from_vec (gil . python () , self . graph . get_sources ()) . cast :: < NodeT > (false) { Ok (v) => Ok (v) , Err (_) => Err (PyErr :: new :: < exceptions :: ValueError , _ > ({ let res = :: alloc :: fmt :: format (:: core :: fmt :: Arguments :: new_v1 (& ["The given array cannot be casted to " , "."] , & match (& "NodeT" ,) { (arg0 ,) => [:: core :: fmt :: ArgumentV1 :: new (arg0 , :: core :: fmt :: Display :: fmt)] , })) ; res })) , } ? . to_owned ())
        }
        /// Return vector on the (non unique) destination nodes of the graph.
        pub fn get_destinations(&self) -> PyResult<Py<PyArray1<NodeT>>> {
            let gil = pyo3::Python::acquire_gil();
            Ok(
                match PyArray::from_vec(gil.python(), self.graph.get_destinations())
                    .cast::<NodeT>(false)
                {
                    Ok(v) => Ok(v),
                    Err(_) => Err(PyErr::new::<exceptions::ValueError, _>({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["The given array cannot be casted to ", "."],
                            &match (&"NodeT",) {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ));
                        res
                    })),
                }?
                .to_owned(),
            )
        }
        /// Return vector of the non-unique source nodes names.
        pub fn get_source_names(&self) -> Vec<String> {
            self.graph.get_source_names()
        }
        /// Return vector on the (non unique) destination nodes of the graph.
        pub fn get_destination_names(&self) -> Vec<String> {
            self.graph.get_destination_names()
        }
        /// Return vector of strings representing the node Ids reverse mapping.
        pub fn get_nodes_reverse_mapping(&self) -> Vec<String> {
            self.graph.get_nodes_reverse_mapping()
        }
        /// Return vector of node types.
        pub fn get_node_types(&self) -> PyResult<Py<PyArray1<NodeTypeT>>> {
            match match self.graph.get_node_types() {
                Some(values) => {
                    let gil = pyo3::Python::acquire_gil();
                    Ok(
                        match PyArray::from_vec(gil.python(), values).cast::<NodeTypeT>(false) {
                            Ok(v) => Ok(v),
                            Err(_) => Err(PyErr::new::<exceptions::ValueError, _>({
                                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                    &["The given array cannot be casted to ", "."],
                                    &match (&"NodeTypeT",) {
                                        (arg0,) => [::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        )],
                                    },
                                ));
                                res
                            })),
                        }?
                        .to_owned(),
                    )
                }
                None => Err("Graph does not have node types."),
            } {
                Ok(v) => Ok(v),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }
        }
        /// Return vector of edge types.
        pub fn get_edge_types(&self) -> PyResult<Py<PyArray1<EdgeTypeT>>> {
            match match self.graph.get_edge_types() {
                Some(values) => {
                    let gil = pyo3::Python::acquire_gil();
                    Ok(
                        match PyArray::from_vec(gil.python(), values).cast::<EdgeTypeT>(false) {
                            Ok(v) => Ok(v),
                            Err(_) => Err(PyErr::new::<exceptions::ValueError, _>({
                                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                    &["The given array cannot be casted to ", "."],
                                    &match (&"EdgeTypeT",) {
                                        (arg0,) => [::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        )],
                                    },
                                ));
                                res
                            })),
                        }?
                        .to_owned(),
                    )
                }
                None => Err("Graph does not have edge types."),
            } {
                Ok(v) => Ok(v),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }
        }
        /// Return vector of weights.
        pub fn get_weights(&self) -> PyResult<Py<PyArray1<WeightT>>> {
            match match self.graph.get_weights() {
                Some(values) => {
                    let gil = pyo3::Python::acquire_gil();
                    Ok(
                        match PyArray::from_vec(gil.python(), values).cast::<WeightT>(false) {
                            Ok(v) => Ok(v),
                            Err(_) => Err(PyErr::new::<exceptions::ValueError, _>({
                                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                    &["The given array cannot be casted to ", "."],
                                    &match (&"WeightT",) {
                                        (arg0,) => [::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        )],
                                    },
                                ));
                                res
                            })),
                        }?
                        .to_owned(),
                    )
                }
                None => Err("Graph does not have weights."),
            } {
                Ok(v) => Ok(v),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }
        }
        /// Return vector of node types_reverse_mapping.
        pub fn get_node_types_reverse_mapping(&self) -> Option<Vec<String>> {
            self.graph.get_node_types_reverse_mapping()
        }
        /// Return vector of edge types_reverse_mapping.
        pub fn get_edge_types_reverse_mapping(&self) -> Option<Vec<String>> {
            self.graph.get_edge_types_reverse_mapping()
        }
        /// Return dictionary of strings to Ids representing the ndoes mapping.
        pub fn get_nodes_mapping(&self) -> HashMap<String, NodeT> {
            self.graph.get_nodes_mapping()
        }
        /// Return the id of the edge type of the edge.
        ///
        /// Parameters
        /// ---------------------
        /// edge_id: int,
        ///     Numeric ID of the edge.
        ///
        /// Returns
        /// ---------------------
        /// Id of the edge type of the edge.
        fn get_edge_type(&self, edge_id: EdgeT) -> PyResult<EdgeTypeT> {
            match self.graph.get_edge_type(edge_id) {
                Ok(v) => Ok(v),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }
        }
        /// Return the id of the node type of the node.
        ///
        /// Parameters
        /// ---------------------
        /// node_id: int,
        ///     Numeric ID of the node.
        ///
        /// Returns
        /// ---------------------
        /// Id of the node type of the node.
        fn get_node_type(&self, node_id: NodeT) -> PyResult<NodeTypeT> {
            match self.graph.get_node_type(node_id) {
                Ok(v) => Ok(v),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }
        }
        /// Return the string name of the node.
        ///
        /// Parameters
        /// ---------------------
        /// node_id: int,
        ///     Numeric ID of the node.
        ///
        /// Returns
        /// ---------------------
        /// String name of the node.
        fn get_node_name(&self, node_id: NodeT) -> PyResult<String> {
            match self.graph.get_node_name(node_id) {
                Ok(v) => Ok(v),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }
        }
        /// Return the node id curresponding to given string name.
        ///
        /// Parameters
        /// ---------------------
        /// node_name: str,
        ///     String name of the node.
        ///
        /// Returns
        /// ---------------------
        /// Node ID.
        fn get_node_id(&self, node_name: &str) -> PyResult<NodeT> {
            match self.graph.get_node_id(node_name) {
                Ok(v) => Ok(v),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }
        }
        /// Return the count of how many time an edge type appears.
        fn get_edge_type_counts(&self) -> PyResult<HashMap<EdgeTypeT, usize>> {
            match self.graph.get_edge_type_counts() {
                Ok(v) => Ok(v),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }
        }
        /// Return the count of how many time an node type appears.
        fn get_node_type_counts(&self) -> PyResult<HashMap<EdgeTypeT, usize>> {
            match self.graph.get_node_type_counts() {
                Ok(v) => Ok(v),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }
        }
        /// Returns a boolean representing if the graph contains an edge that has
        /// source == destination.
        fn has_selfloops(&self) -> bool {
            self.graph.has_selfloops()
        }
        /// Returns true if the graph has weights.
        fn has_weights(&self) -> bool {
            self.graph.has_weights()
        }
        /// Returns true if the graph has node types.
        fn has_node_types(&self) -> bool {
            self.graph.has_node_types()
        }
        /// Returns true if the graph has edge types.
        fn has_edge_types(&self) -> bool {
            self.graph.has_edge_types()
        }
    }
    #[used]
    #[allow(non_upper_case_globals)]
    #[link_section = ".init_array"]
    #[allow(non_upper_case_globals)]
    static __init7134285972105153340: extern "C" fn() = {
        #[link_section = ".text.startup"]
        extern "C" fn __init7134285972105153340() {
            pyo3::inventory::submit({
                {
                    type Inventory =
                        <EnsmallenGraph as pyo3::class::methods::HasMethodsInventory>::Methods;
                    <Inventory as pyo3::class::methods::PyMethodsInventory>::new(<[_]>::into_vec(
                        box [
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.get_nodes_number()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    pyo3::callback::convert(
                                                        _py,
                                                        EnsmallenGraph::get_nodes_number(_slf),
                                                    )
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction ("get_nodes_number\u{0}" , __wrap , "get_nodes_number(self)\n--\n\nReturn the number of nodes in the graph.\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                    _kwargs: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.get_top_k_central_nodes()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    let _args = _py
                                                        .from_borrowed_ptr::<pyo3::types::PyTuple>(
                                                            _args,
                                                        );
                                                    let _kwargs: Option<&pyo3::types::PyDict> =
                                                        _py.from_borrowed_ptr_or_opt(_kwargs);
                                                    pyo3::callback::convert(_py, {
                                                        const PARAMS : & 'static [pyo3 :: derive_utils :: ParamDescription] = & [pyo3 :: derive_utils :: ParamDescription { name : "k" , is_optional : false , kw_only : false , }] ;
                                                        let mut output = [None; 1usize];
                                                        let mut _args = _args;
                                                        let mut _kwargs = _kwargs;
                                                        let (_args, _kwargs) =
                                                            pyo3::derive_utils::parse_fn_args(
                                                                Some(_LOCATION),
                                                                PARAMS,
                                                                _args,
                                                                _kwargs,
                                                                false,
                                                                false,
                                                                &mut output,
                                                            )?;
                                                        let arg0 = match output [0usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        EnsmallenGraph::get_top_k_central_nodes(
                                                            _slf, arg0,
                                                        )
                                                    })
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction_with_keywords ("get_top_k_central_nodes\u{0}" , __wrap , 0 , "get_top_k_central_nodes(self)\n--\n\nReturn the nodes reverse mapping.\n\nParameters\n-----------------\nk: int,\n     Number of central nodes to extract.\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str = "EnsmallenGraph.get_name()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    pyo3::callback::convert(
                                                        _py,
                                                        EnsmallenGraph::get_name(_slf),
                                                    )
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3::class::PyMethodDef::cfunction(
                                    "get_name\u{0}",
                                    __wrap,
                                    "get_name(self)\n--\n\nReturn the name of the graph.\u{0}",
                                )
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.get_edges_number()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    pyo3::callback::convert(
                                                        _py,
                                                        EnsmallenGraph::get_edges_number(_slf),
                                                    )
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction ("get_edges_number\u{0}" , __wrap , "get_edges_number(self)\n--\n\nReturn the number of edges in the graph.\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.get_undirected_edges_number()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    pyo3::callback::convert(
                                                        _py,
                                                        EnsmallenGraph::get_undirected_edges_number(
                                                            _slf,
                                                        ),
                                                    )
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction ("get_undirected_edges_number\u{0}" , __wrap , "get_undirected_edges_number(self)\n--\n\nReturn the number of undirected edges in the graph.\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.get_edge_types_number()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    pyo3::callback::convert(
                                                        _py,
                                                        EnsmallenGraph::get_edge_types_number(_slf),
                                                    )
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction ("get_edge_types_number\u{0}" , __wrap , "get_edge_types_number(self)\n--\n\nReturn the number of edges types in the graph.\n\nThis method will include, if found necessary by a missing value,\nalso the default edge type in the count of total edge types.\n\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.get_node_types_number()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    pyo3::callback::convert(
                                                        _py,
                                                        EnsmallenGraph::get_node_types_number(_slf),
                                                    )
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction ("get_node_types_number\u{0}" , __wrap , "get_node_types_number(self)\n--\n\nReturn the number of edges in the graph.\n\nThis method will include, if found necessary by a missing value,\nalso the default node type in the count of total node types.\n\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                    _kwargs: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str = "EnsmallenGraph.is_node_trap()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    let _args = _py
                                                        .from_borrowed_ptr::<pyo3::types::PyTuple>(
                                                            _args,
                                                        );
                                                    let _kwargs: Option<&pyo3::types::PyDict> =
                                                        _py.from_borrowed_ptr_or_opt(_kwargs);
                                                    pyo3::callback::convert(_py, {
                                                        const PARAMS : & 'static [pyo3 :: derive_utils :: ParamDescription] = & [pyo3 :: derive_utils :: ParamDescription { name : "node" , is_optional : false , kw_only : false , }] ;
                                                        let mut output = [None; 1usize];
                                                        let mut _args = _args;
                                                        let mut _kwargs = _kwargs;
                                                        let (_args, _kwargs) =
                                                            pyo3::derive_utils::parse_fn_args(
                                                                Some(_LOCATION),
                                                                PARAMS,
                                                                _args,
                                                                _kwargs,
                                                                false,
                                                                false,
                                                                &mut output,
                                                            )?;
                                                        let arg0 = match output [0usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        EnsmallenGraph::is_node_trap(_slf, arg0)
                                                    })
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction_with_keywords ("is_node_trap\u{0}" , __wrap , 0 , "is_node_trap($self, node)\n--\n\nReturn boolean representing if given node is a trap.\n\nA trap node is a node with no outbounds edges.\n\nParameters\n---------------------\nnode: int,\n    Node ID to search if it\'s a trap.\n\nReturns\n----------------------------\nBoolean representing if given node is a trap.\n\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                    _kwargs: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str = "EnsmallenGraph.is_singleton()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    let _args = _py
                                                        .from_borrowed_ptr::<pyo3::types::PyTuple>(
                                                            _args,
                                                        );
                                                    let _kwargs: Option<&pyo3::types::PyDict> =
                                                        _py.from_borrowed_ptr_or_opt(_kwargs);
                                                    pyo3::callback::convert(_py, {
                                                        const PARAMS : & 'static [pyo3 :: derive_utils :: ParamDescription] = & [pyo3 :: derive_utils :: ParamDescription { name : "node_id" , is_optional : false , kw_only : false , }] ;
                                                        let mut output = [None; 1usize];
                                                        let mut _args = _args;
                                                        let mut _kwargs = _kwargs;
                                                        let (_args, _kwargs) =
                                                            pyo3::derive_utils::parse_fn_args(
                                                                Some(_LOCATION),
                                                                PARAMS,
                                                                _args,
                                                                _kwargs,
                                                                false,
                                                                false,
                                                                &mut output,
                                                            )?;
                                                        let arg0 = match output [0usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        EnsmallenGraph::is_singleton(_slf, arg0)
                                                    })
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction_with_keywords ("is_singleton\u{0}" , __wrap , 0 , "is_singleton($self, node_id)\n--\n\nReturn boolean representing singletons.\n\nParameters\n---------------------\nnode_id: int,\n    Node ID to search if it\'s a singleton.\n\nReturns\n----------------------------\nBoolean representing if given node is a singleton.\n\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                    _kwargs: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.is_singleton_string()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    let _args = _py
                                                        .from_borrowed_ptr::<pyo3::types::PyTuple>(
                                                            _args,
                                                        );
                                                    let _kwargs: Option<&pyo3::types::PyDict> =
                                                        _py.from_borrowed_ptr_or_opt(_kwargs);
                                                    pyo3::callback::convert(_py, {
                                                        const PARAMS : & 'static [pyo3 :: derive_utils :: ParamDescription] = & [pyo3 :: derive_utils :: ParamDescription { name : "node_name" , is_optional : false , kw_only : false , }] ;
                                                        let mut output = [None; 1usize];
                                                        let mut _args = _args;
                                                        let mut _kwargs = _kwargs;
                                                        let (_args, _kwargs) =
                                                            pyo3::derive_utils::parse_fn_args(
                                                                Some(_LOCATION),
                                                                PARAMS,
                                                                _args,
                                                                _kwargs,
                                                                false,
                                                                false,
                                                                &mut output,
                                                            )?;
                                                        let _tmp : < & str as pyo3 :: derive_utils :: ExtractExt > :: Target = match output [0usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        let arg0 = &*_tmp;
                                                        EnsmallenGraph::is_singleton_string(
                                                            _slf, arg0,
                                                        )
                                                    })
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction_with_keywords ("is_singleton_string\u{0}" , __wrap , 0 , "is_singleton_string($self, node_name)\n--\n\nReturn boolean representing singletons.\n\nParameters\n---------------------\nnode_name: str,\n    Node name to search if it\'s a singleton.\n\nReturns\n----------------------------\nBoolean representing if given node is a singleton.\n\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                    _kwargs: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str = "EnsmallenGraph.is_edge_trap()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    let _args = _py
                                                        .from_borrowed_ptr::<pyo3::types::PyTuple>(
                                                            _args,
                                                        );
                                                    let _kwargs: Option<&pyo3::types::PyDict> =
                                                        _py.from_borrowed_ptr_or_opt(_kwargs);
                                                    pyo3::callback::convert(_py, {
                                                        const PARAMS : & 'static [pyo3 :: derive_utils :: ParamDescription] = & [pyo3 :: derive_utils :: ParamDescription { name : "edge" , is_optional : false , kw_only : false , }] ;
                                                        let mut output = [None; 1usize];
                                                        let mut _args = _args;
                                                        let mut _kwargs = _kwargs;
                                                        let (_args, _kwargs) =
                                                            pyo3::derive_utils::parse_fn_args(
                                                                Some(_LOCATION),
                                                                PARAMS,
                                                                _args,
                                                                _kwargs,
                                                                false,
                                                                false,
                                                                &mut output,
                                                            )?;
                                                        let arg0 = match output [0usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        EnsmallenGraph::is_edge_trap(_slf, arg0)
                                                    })
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction_with_keywords ("is_edge_trap\u{0}" , __wrap , 0 , "is_edge_trap($self, edge)\n--\n\nReturn boolean representing if given edge is a trap.\n\nA trap edge is a edge with a destination node that is a trap node.\n\nParameters\n---------------------\nnode: int,\n    Node ID to search if it\'s a trap.\n\nReturns\n----------------------------\nBoolean representing if given edge is a trap.\n\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                    _kwargs: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str = "EnsmallenGraph.has_edge()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    let _args = _py
                                                        .from_borrowed_ptr::<pyo3::types::PyTuple>(
                                                            _args,
                                                        );
                                                    let _kwargs: Option<&pyo3::types::PyDict> =
                                                        _py.from_borrowed_ptr_or_opt(_kwargs);
                                                    pyo3::callback::convert(_py, {
                                                        const PARAMS : & 'static [pyo3 :: derive_utils :: ParamDescription] = & [pyo3 :: derive_utils :: ParamDescription { name : "src" , is_optional : false , kw_only : false , } , pyo3 :: derive_utils :: ParamDescription { name : "dst" , is_optional : false , kw_only : false , } , pyo3 :: derive_utils :: ParamDescription { name : "edge_type" , is_optional : true , kw_only : false , }] ;
                                                        let mut output = [None; 3usize];
                                                        let mut _args = _args;
                                                        let mut _kwargs = _kwargs;
                                                        let (_args, _kwargs) =
                                                            pyo3::derive_utils::parse_fn_args(
                                                                Some(_LOCATION),
                                                                PARAMS,
                                                                _args,
                                                                _kwargs,
                                                                false,
                                                                false,
                                                                &mut output,
                                                            )?;
                                                        let arg0 = match output [0usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        let arg1 = match output [1usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        let arg2 = match output[2usize] {
                                                            Some(_obj) => _obj.extract()?,
                                                            None => None,
                                                        };
                                                        EnsmallenGraph::has_edge(
                                                            _slf, arg0, arg1, arg2,
                                                        )
                                                    })
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction_with_keywords ("has_edge\u{0}" , __wrap , 0 , "has_edge($self, src, dst, edge_type)\n--\n\nReturn boolean representing if given edge exists in graph.\n\nParameters\n---------------------\nsrc: int,\n    Node ID to use as source of given edge.\ndst: int,\n    Node ID to use as destination of given edge.\nedge_type: Union[None, int],\n    Edge type ID. (By deafult is None).\n\nReturns\n----------------------------\nBoolean representing if given edge exists in graph.\n\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                    _kwargs: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.has_edge_string()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    let _args = _py
                                                        .from_borrowed_ptr::<pyo3::types::PyTuple>(
                                                            _args,
                                                        );
                                                    let _kwargs: Option<&pyo3::types::PyDict> =
                                                        _py.from_borrowed_ptr_or_opt(_kwargs);
                                                    pyo3::callback::convert(_py, {
                                                        const PARAMS : & 'static [pyo3 :: derive_utils :: ParamDescription] = & [pyo3 :: derive_utils :: ParamDescription { name : "src" , is_optional : false , kw_only : false , } , pyo3 :: derive_utils :: ParamDescription { name : "dst" , is_optional : false , kw_only : false , } , pyo3 :: derive_utils :: ParamDescription { name : "edge_type" , is_optional : true , kw_only : false , }] ;
                                                        let mut output = [None; 3usize];
                                                        let mut _args = _args;
                                                        let mut _kwargs = _kwargs;
                                                        let (_args, _kwargs) =
                                                            pyo3::derive_utils::parse_fn_args(
                                                                Some(_LOCATION),
                                                                PARAMS,
                                                                _args,
                                                                _kwargs,
                                                                false,
                                                                false,
                                                                &mut output,
                                                            )?;
                                                        let _tmp : < & str as pyo3 :: derive_utils :: ExtractExt > :: Target = match output [0usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        let arg0 = &*_tmp;
                                                        let _tmp : < & str as pyo3 :: derive_utils :: ExtractExt > :: Target = match output [1usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        let arg1 = &*_tmp;
                                                        let arg2 = match output[2usize] {
                                                            Some(_obj) => _obj.extract()?,
                                                            None => None,
                                                        };
                                                        EnsmallenGraph::has_edge_string(
                                                            _slf, arg0, arg1, arg2,
                                                        )
                                                    })
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction_with_keywords ("has_edge_string\u{0}" , __wrap , 0 , "has_edge_string($self, src, dst, edge_type)\n--\n\nReturn boolean representing if given edge exists in graph.\n\nParameters\n---------------------\nsrc: str,\n    Node name to use as source of given edge.\ndst: str,\n    Node name to use as destination of given edge.\nedge_type: Union[None, str],\n    Edge type name. (By deafult is None).\n\nReturns\n----------------------------\nBoolean representing if given edge exists in graph.\n\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                    _kwargs: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.has_node_string()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    let _args = _py
                                                        .from_borrowed_ptr::<pyo3::types::PyTuple>(
                                                            _args,
                                                        );
                                                    let _kwargs: Option<&pyo3::types::PyDict> =
                                                        _py.from_borrowed_ptr_or_opt(_kwargs);
                                                    pyo3::callback::convert(_py, {
                                                        const PARAMS : & 'static [pyo3 :: derive_utils :: ParamDescription] = & [pyo3 :: derive_utils :: ParamDescription { name : "node_name" , is_optional : false , kw_only : false , } , pyo3 :: derive_utils :: ParamDescription { name : "node_type" , is_optional : true , kw_only : false , }] ;
                                                        let mut output = [None; 2usize];
                                                        let mut _args = _args;
                                                        let mut _kwargs = _kwargs;
                                                        let (_args, _kwargs) =
                                                            pyo3::derive_utils::parse_fn_args(
                                                                Some(_LOCATION),
                                                                PARAMS,
                                                                _args,
                                                                _kwargs,
                                                                false,
                                                                false,
                                                                &mut output,
                                                            )?;
                                                        let _tmp : < & str as pyo3 :: derive_utils :: ExtractExt > :: Target = match output [0usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        let arg0 = &*_tmp;
                                                        let arg1 = match output[1usize] {
                                                            Some(_obj) => _obj.extract()?,
                                                            None => None,
                                                        };
                                                        EnsmallenGraph::has_node_string(
                                                            _slf, arg0, arg1,
                                                        )
                                                    })
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction_with_keywords ("has_node_string\u{0}" , __wrap , 0 , "has_node_string($self, node_name, node_type)\n--\n\nReturn boolean representing if given node exists in graph.\n\nParameters\n---------------------\nnode_name: str,\n    Name of the node.\nnode_type: str = None,\n    Optional node type of the node.\n\nReturns\n----------------------------\nBoolean representing if given node exists in graph.\n\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                    _kwargs: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str = "EnsmallenGraph.get_edge_id()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    let _args = _py
                                                        .from_borrowed_ptr::<pyo3::types::PyTuple>(
                                                            _args,
                                                        );
                                                    let _kwargs: Option<&pyo3::types::PyDict> =
                                                        _py.from_borrowed_ptr_or_opt(_kwargs);
                                                    pyo3::callback::convert(_py, {
                                                        const PARAMS : & 'static [pyo3 :: derive_utils :: ParamDescription] = & [pyo3 :: derive_utils :: ParamDescription { name : "src" , is_optional : false , kw_only : false , } , pyo3 :: derive_utils :: ParamDescription { name : "dst" , is_optional : false , kw_only : false , } , pyo3 :: derive_utils :: ParamDescription { name : "edge_type" , is_optional : true , kw_only : false , }] ;
                                                        let mut output = [None; 3usize];
                                                        let mut _args = _args;
                                                        let mut _kwargs = _kwargs;
                                                        let (_args, _kwargs) =
                                                            pyo3::derive_utils::parse_fn_args(
                                                                Some(_LOCATION),
                                                                PARAMS,
                                                                _args,
                                                                _kwargs,
                                                                false,
                                                                false,
                                                                &mut output,
                                                            )?;
                                                        let arg0 = match output [0usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        let arg1 = match output [1usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        let arg2 = match output[2usize] {
                                                            Some(_obj) => _obj.extract()?,
                                                            None => None,
                                                        };
                                                        EnsmallenGraph::get_edge_id(
                                                            _slf, arg0, arg1, arg2,
                                                        )
                                                    })
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction_with_keywords ("get_edge_id\u{0}" , __wrap , 0 , "get_edge_id($self, src, dst, edge_type)\n--\n\nReturn integer representing ID of the edge.\n\nParameters\n---------------------\nsrc: int,\n    Node ID to use as source of given edge.\ndst: int,\n    Node ID to use as destination of given edge.\nedge_type: Union[None, int],\n    Edge type ID. (By deafult is None).\n\nReturns\n----------------------------\nInteger representing ID of the edge. It will return None when the edge does not exist.\n\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                    _kwargs: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.get_edge_id_string()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    let _args = _py
                                                        .from_borrowed_ptr::<pyo3::types::PyTuple>(
                                                            _args,
                                                        );
                                                    let _kwargs: Option<&pyo3::types::PyDict> =
                                                        _py.from_borrowed_ptr_or_opt(_kwargs);
                                                    pyo3::callback::convert(_py, {
                                                        const PARAMS : & 'static [pyo3 :: derive_utils :: ParamDescription] = & [pyo3 :: derive_utils :: ParamDescription { name : "src" , is_optional : false , kw_only : false , } , pyo3 :: derive_utils :: ParamDescription { name : "dst" , is_optional : false , kw_only : false , } , pyo3 :: derive_utils :: ParamDescription { name : "edge_type" , is_optional : true , kw_only : false , }] ;
                                                        let mut output = [None; 3usize];
                                                        let mut _args = _args;
                                                        let mut _kwargs = _kwargs;
                                                        let (_args, _kwargs) =
                                                            pyo3::derive_utils::parse_fn_args(
                                                                Some(_LOCATION),
                                                                PARAMS,
                                                                _args,
                                                                _kwargs,
                                                                false,
                                                                false,
                                                                &mut output,
                                                            )?;
                                                        let _tmp : < & str as pyo3 :: derive_utils :: ExtractExt > :: Target = match output [0usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        let arg0 = &*_tmp;
                                                        let _tmp : < & str as pyo3 :: derive_utils :: ExtractExt > :: Target = match output [1usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        let arg1 = &*_tmp;
                                                        let arg2 = match output[2usize] {
                                                            Some(_obj) => _obj.extract()?,
                                                            None => None,
                                                        };
                                                        EnsmallenGraph::get_edge_id_string(
                                                            _slf, arg0, arg1, arg2,
                                                        )
                                                    })
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction_with_keywords ("get_edge_id_string\u{0}" , __wrap , 0 , "get_edge_id_string($self, src, dst, edge_type)\n--\n\nReturn integer representing ID of the edge.\n\nParameters\n---------------------\nsrc: str,\n    Node name to use as source of given edge.\ndst: str,\n    Node name to use as destination of given edge.\nedge_type: Union[None, str],\n    Edge type name. (By deafult is None).\n\nReturns\n----------------------------\nInteger representing ID of the edge. It will return None when the edge does not exist.\n\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.get_dense_node_mapping()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    pyo3::callback::convert(
                                                        _py,
                                                        EnsmallenGraph::get_dense_node_mapping(
                                                            _slf,
                                                        ),
                                                    )
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction ("get_dense_node_mapping\u{0}" , __wrap , "get_dense_node_mapping($self)\n--\n\nReturn mapping from instance not trap nodes to dense range of nodes.\n\nReturns\n----------------------------\nDict with mapping from not trap nodes to dense range of nodes.\n\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.get_source_nodes_number()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    pyo3::callback::convert(
                                                        _py,
                                                        EnsmallenGraph::get_source_nodes_number(
                                                            _slf,
                                                        ),
                                                    )
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction ("get_source_nodes_number\u{0}" , __wrap , "get_source_nodes_number($self)\n--\n\nReturn the number of source nodes.\n\nReturns\n----------------------------\nNumber of the source nodes.\n\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str = "EnsmallenGraph.get_sources()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    pyo3::callback::convert(
                                                        _py,
                                                        EnsmallenGraph::get_sources(_slf),
                                                    )
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3::class::PyMethodDef::cfunction(
                                    "get_sources\u{0}",
                                    __wrap,
                                    "Return vector of the non-unique source nodes.\u{0}",
                                )
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.get_destinations()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    pyo3::callback::convert(
                                                        _py,
                                                        EnsmallenGraph::get_destinations(_slf),
                                                    )
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction ("get_destinations\u{0}" , __wrap , "Return vector on the (non unique) destination nodes of the graph.\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.get_source_names()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    pyo3::callback::convert(
                                                        _py,
                                                        EnsmallenGraph::get_source_names(_slf),
                                                    )
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3::class::PyMethodDef::cfunction(
                                    "get_source_names\u{0}",
                                    __wrap,
                                    "Return vector of the non-unique source nodes names.\u{0}",
                                )
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.get_destination_names()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    pyo3::callback::convert(
                                                        _py,
                                                        EnsmallenGraph::get_destination_names(_slf),
                                                    )
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction ("get_destination_names\u{0}" , __wrap , "Return vector on the (non unique) destination nodes of the graph.\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.get_nodes_reverse_mapping()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    pyo3::callback::convert(
                                                        _py,
                                                        EnsmallenGraph::get_nodes_reverse_mapping(
                                                            _slf,
                                                        ),
                                                    )
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction ("get_nodes_reverse_mapping\u{0}" , __wrap , "Return vector of strings representing the node Ids reverse mapping.\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.get_node_types()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    pyo3::callback::convert(
                                                        _py,
                                                        EnsmallenGraph::get_node_types(_slf),
                                                    )
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3::class::PyMethodDef::cfunction(
                                    "get_node_types\u{0}",
                                    __wrap,
                                    "Return vector of node types.\u{0}",
                                )
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.get_edge_types()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    pyo3::callback::convert(
                                                        _py,
                                                        EnsmallenGraph::get_edge_types(_slf),
                                                    )
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3::class::PyMethodDef::cfunction(
                                    "get_edge_types\u{0}",
                                    __wrap,
                                    "Return vector of edge types.\u{0}",
                                )
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str = "EnsmallenGraph.get_weights()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    pyo3::callback::convert(
                                                        _py,
                                                        EnsmallenGraph::get_weights(_slf),
                                                    )
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3::class::PyMethodDef::cfunction(
                                    "get_weights\u{0}",
                                    __wrap,
                                    "Return vector of weights.\u{0}",
                                )
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.get_node_types_reverse_mapping()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    pyo3 :: callback :: convert (_py , EnsmallenGraph :: get_node_types_reverse_mapping (_slf))
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3::class::PyMethodDef::cfunction(
                                    "get_node_types_reverse_mapping\u{0}",
                                    __wrap,
                                    "Return vector of node types_reverse_mapping.\u{0}",
                                )
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.get_edge_types_reverse_mapping()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    pyo3 :: callback :: convert (_py , EnsmallenGraph :: get_edge_types_reverse_mapping (_slf))
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3::class::PyMethodDef::cfunction(
                                    "get_edge_types_reverse_mapping\u{0}",
                                    __wrap,
                                    "Return vector of edge types_reverse_mapping.\u{0}",
                                )
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.get_nodes_mapping()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    pyo3::callback::convert(
                                                        _py,
                                                        EnsmallenGraph::get_nodes_mapping(_slf),
                                                    )
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction ("get_nodes_mapping\u{0}" , __wrap , "Return dictionary of strings to Ids representing the ndoes mapping.\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                    _kwargs: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.get_edge_type()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    let _args = _py
                                                        .from_borrowed_ptr::<pyo3::types::PyTuple>(
                                                            _args,
                                                        );
                                                    let _kwargs: Option<&pyo3::types::PyDict> =
                                                        _py.from_borrowed_ptr_or_opt(_kwargs);
                                                    pyo3::callback::convert(_py, {
                                                        const PARAMS : & 'static [pyo3 :: derive_utils :: ParamDescription] = & [pyo3 :: derive_utils :: ParamDescription { name : "edge_id" , is_optional : false , kw_only : false , }] ;
                                                        let mut output = [None; 1usize];
                                                        let mut _args = _args;
                                                        let mut _kwargs = _kwargs;
                                                        let (_args, _kwargs) =
                                                            pyo3::derive_utils::parse_fn_args(
                                                                Some(_LOCATION),
                                                                PARAMS,
                                                                _args,
                                                                _kwargs,
                                                                false,
                                                                false,
                                                                &mut output,
                                                            )?;
                                                        let arg0 = match output [0usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        EnsmallenGraph::get_edge_type(_slf, arg0)
                                                    })
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction_with_keywords ("get_edge_type\u{0}" , __wrap , 0 , "get_edge_type($self, edge_id)\n--\n\nReturn the id of the edge type of the edge.\n\nParameters\n---------------------\nedge_id: int,\n    Numeric ID of the edge.\n\nReturns\n---------------------\nId of the edge type of the edge.\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                    _kwargs: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.get_node_type()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    let _args = _py
                                                        .from_borrowed_ptr::<pyo3::types::PyTuple>(
                                                            _args,
                                                        );
                                                    let _kwargs: Option<&pyo3::types::PyDict> =
                                                        _py.from_borrowed_ptr_or_opt(_kwargs);
                                                    pyo3::callback::convert(_py, {
                                                        const PARAMS : & 'static [pyo3 :: derive_utils :: ParamDescription] = & [pyo3 :: derive_utils :: ParamDescription { name : "node_id" , is_optional : false , kw_only : false , }] ;
                                                        let mut output = [None; 1usize];
                                                        let mut _args = _args;
                                                        let mut _kwargs = _kwargs;
                                                        let (_args, _kwargs) =
                                                            pyo3::derive_utils::parse_fn_args(
                                                                Some(_LOCATION),
                                                                PARAMS,
                                                                _args,
                                                                _kwargs,
                                                                false,
                                                                false,
                                                                &mut output,
                                                            )?;
                                                        let arg0 = match output [0usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        EnsmallenGraph::get_node_type(_slf, arg0)
                                                    })
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction_with_keywords ("get_node_type\u{0}" , __wrap , 0 , "get_node_type($self, node_id)\n--\n\nReturn the id of the node type of the node.\n\nParameters\n---------------------\nnode_id: int,\n    Numeric ID of the node.\n\nReturns\n---------------------\nId of the node type of the node.\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                    _kwargs: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.get_node_name()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    let _args = _py
                                                        .from_borrowed_ptr::<pyo3::types::PyTuple>(
                                                            _args,
                                                        );
                                                    let _kwargs: Option<&pyo3::types::PyDict> =
                                                        _py.from_borrowed_ptr_or_opt(_kwargs);
                                                    pyo3::callback::convert(_py, {
                                                        const PARAMS : & 'static [pyo3 :: derive_utils :: ParamDescription] = & [pyo3 :: derive_utils :: ParamDescription { name : "node_id" , is_optional : false , kw_only : false , }] ;
                                                        let mut output = [None; 1usize];
                                                        let mut _args = _args;
                                                        let mut _kwargs = _kwargs;
                                                        let (_args, _kwargs) =
                                                            pyo3::derive_utils::parse_fn_args(
                                                                Some(_LOCATION),
                                                                PARAMS,
                                                                _args,
                                                                _kwargs,
                                                                false,
                                                                false,
                                                                &mut output,
                                                            )?;
                                                        let arg0 = match output [0usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        EnsmallenGraph::get_node_name(_slf, arg0)
                                                    })
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction_with_keywords ("get_node_name\u{0}" , __wrap , 0 , "get_node_name($self, node_id)\n--\n\nReturn the string name of the node.\n\nParameters\n---------------------\nnode_id: int,\n    Numeric ID of the node.\n\nReturns\n---------------------\nString name of the node.\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                    _kwargs: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str = "EnsmallenGraph.get_node_id()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    let _args = _py
                                                        .from_borrowed_ptr::<pyo3::types::PyTuple>(
                                                            _args,
                                                        );
                                                    let _kwargs: Option<&pyo3::types::PyDict> =
                                                        _py.from_borrowed_ptr_or_opt(_kwargs);
                                                    pyo3::callback::convert(_py, {
                                                        const PARAMS : & 'static [pyo3 :: derive_utils :: ParamDescription] = & [pyo3 :: derive_utils :: ParamDescription { name : "node_name" , is_optional : false , kw_only : false , }] ;
                                                        let mut output = [None; 1usize];
                                                        let mut _args = _args;
                                                        let mut _kwargs = _kwargs;
                                                        let (_args, _kwargs) =
                                                            pyo3::derive_utils::parse_fn_args(
                                                                Some(_LOCATION),
                                                                PARAMS,
                                                                _args,
                                                                _kwargs,
                                                                false,
                                                                false,
                                                                &mut output,
                                                            )?;
                                                        let _tmp : < & str as pyo3 :: derive_utils :: ExtractExt > :: Target = match output [0usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        let arg0 = &*_tmp;
                                                        EnsmallenGraph::get_node_id(_slf, arg0)
                                                    })
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction_with_keywords ("get_node_id\u{0}" , __wrap , 0 , "get_node_id($self, node_name)\n--\n\nReturn the node id curresponding to given string name.\n\nParameters\n---------------------\nnode_name: str,\n    String name of the node.\n\nReturns\n---------------------\nNode ID.\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.get_edge_type_counts()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    pyo3::callback::convert(
                                                        _py,
                                                        EnsmallenGraph::get_edge_type_counts(_slf),
                                                    )
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction ("get_edge_type_counts\u{0}" , __wrap , "get_edge_type_counts($self)\n--\n\nReturn the count of how many time an edge type appears.\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.get_node_type_counts()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    pyo3::callback::convert(
                                                        _py,
                                                        EnsmallenGraph::get_node_type_counts(_slf),
                                                    )
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction ("get_node_type_counts\u{0}" , __wrap , "get_node_type_counts($self)\n--\n\nReturn the count of how many time an node type appears.\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.has_selfloops()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    pyo3::callback::convert(
                                                        _py,
                                                        EnsmallenGraph::has_selfloops(_slf),
                                                    )
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction ("has_selfloops\u{0}" , __wrap , "has_selfloops(self)\n--\n\nReturns a boolean representing if the graph contains an edge that has\nsource == destination.\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str = "EnsmallenGraph.has_weights()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    pyo3::callback::convert(
                                                        _py,
                                                        EnsmallenGraph::has_weights(_slf),
                                                    )
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction ("has_weights\u{0}" , __wrap , "has_weights(self)\n--\n\nReturns true if the graph has weights.\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.has_node_types()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    pyo3::callback::convert(
                                                        _py,
                                                        EnsmallenGraph::has_node_types(_slf),
                                                    )
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction ("has_node_types\u{0}" , __wrap , "has_node_types(self)\n--\n\nReturns true if the graph has node types.\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.has_edge_types()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    pyo3::callback::convert(
                                                        _py,
                                                        EnsmallenGraph::has_edge_types(_slf),
                                                    )
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction ("has_edge_types\u{0}" , __wrap , "has_edge_types(self)\n--\n\nReturns true if the graph has edge types.\u{0}")
                            }),
                        ],
                    ))
                }
            });
        }
        __init7134285972105153340
    };
}
mod setters {
    use super::*;
    impl EnsmallenGraph {
        /// Set the name of the graph.
        fn set_name(&mut self, name: String) {
            self.graph.set_name(name)
        }
    }
    #[used]
    #[allow(non_upper_case_globals)]
    #[link_section = ".init_array"]
    #[allow(non_upper_case_globals)]
    static __init4712734044101901388: extern "C" fn() = {
        #[link_section = ".text.startup"]
        extern "C" fn __init4712734044101901388() {
            pyo3::inventory::submit({
                {
                    type Inventory =
                        <EnsmallenGraph as pyo3::class::methods::HasMethodsInventory>::Methods;
                    <Inventory as pyo3::class::methods::PyMethodsInventory>::new(<[_]>::into_vec(
                        box [pyo3::class::PyMethodDefType::Method({
                            unsafe extern "C" fn __wrap(
                                _slf: *mut pyo3::ffi::PyObject,
                                _args: *mut pyo3::ffi::PyObject,
                                _kwargs: *mut pyo3::ffi::PyObject,
                            ) -> *mut pyo3::ffi::PyObject {
                                const _LOCATION: &'static str = "EnsmallenGraph.set_name()";
                                {
                                    let pool = ::pyo3::GILPool::new();
                                    let unwind_safe_py =
                                        std::panic::AssertUnwindSafe(pool.python());
                                    let result = match std::panic::catch_unwind(
                                        move || -> ::pyo3::PyResult<_> {
                                            let _py = *unwind_safe_py;
                                            {
                                                let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                let mut _ref = _cell.try_borrow_mut()?;
                                                let _slf = &mut _ref;
                                                let _args = _py
                                                    .from_borrowed_ptr::<pyo3::types::PyTuple>(
                                                        _args,
                                                    );
                                                let _kwargs: Option<&pyo3::types::PyDict> =
                                                    _py.from_borrowed_ptr_or_opt(_kwargs);
                                                pyo3::callback::convert(_py, {
                                                    const PARAMS : & 'static [pyo3 :: derive_utils :: ParamDescription] = & [pyo3 :: derive_utils :: ParamDescription { name : "name" , is_optional : false , kw_only : false , }] ;
                                                    let mut output = [None; 1usize];
                                                    let mut _args = _args;
                                                    let mut _kwargs = _kwargs;
                                                    let (_args, _kwargs) =
                                                        pyo3::derive_utils::parse_fn_args(
                                                            Some(_LOCATION),
                                                            PARAMS,
                                                            _args,
                                                            _kwargs,
                                                            false,
                                                            false,
                                                            &mut output,
                                                        )?;
                                                    let arg0 = match output [0usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                    EnsmallenGraph::set_name(_slf, arg0)
                                                })
                                            }
                                        },
                                    ) {
                                        Ok(result) => result,
                                        Err(e) => {
                                            if let Some(string) = e.downcast_ref::<String>() {
                                                Err(::pyo3::panic::PanicException::new_err((
                                                    string.clone(),
                                                )))
                                            } else if let Some(s) = e.downcast_ref::<&str>() {
                                                Err(::pyo3::panic::PanicException::new_err((
                                                    s.to_string(),
                                                )))
                                            } else {
                                                Err(::pyo3::panic::PanicException::new_err((
                                                    "panic from Rust code",
                                                )))
                                            }
                                        }
                                    };
                                    result.unwrap_or_else(|e| {
                                        e.restore(pool.python());
                                        ::pyo3::callback::callback_error()
                                    })
                                }
                            }
                            pyo3::class::PyMethodDef::cfunction_with_keywords(
                                "set_name\u{0}",
                                __wrap,
                                0,
                                "set_name(self)\n--\n\nSet the name of the graph.\u{0}",
                            )
                        })],
                    ))
                }
            });
        }
        __init4712734044101901388
    };
}
mod filters {
    use super::*;
    use graph::WeightT;
    impl EnsmallenGraph {
        /// Return graph filtered by given filters.
        ///
        /// Parameters
        /// -------------
        /// `node_names`: List[str],
        ///     The node names to keep.
        /// `node_types`: List[str],
        ///     The node types to keep.
        /// `edge_types`: List[str],
        ///     The edge types to keep.
        /// `min_weight`: float,
        ///     Minimum weight to use to filter edges.
        /// `max_weight`: float,
        ///     Maximum weight to use to filter edges.
        /// `verbose`: bool,
        ///     Wether to show the loading bar.
        ///
        /// Returns
        /// -------------
        /// The filtered graph.
        pub fn filter(
            &self,
            nodes: Option<Vec<String>>,
            node_types: Option<Vec<String>>,
            edge_types: Option<Vec<String>>,
            min_weight: Option<WeightT>,
            max_weight: Option<WeightT>,
            verbose: Option<bool>,
        ) -> PyResult<EnsmallenGraph> {
            Ok(EnsmallenGraph {
                graph: match self.graph.filter(
                    nodes,
                    node_types,
                    edge_types,
                    min_weight,
                    max_weight,
                    verbose.unwrap_or(true),
                ) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?,
            })
        }
    }
    #[used]
    #[allow(non_upper_case_globals)]
    #[link_section = ".init_array"]
    #[allow(non_upper_case_globals)]
    static __init16423468343507468216: extern "C" fn() = {
        #[link_section = ".text.startup"]
        extern "C" fn __init16423468343507468216() {
            pyo3::inventory::submit({
                {
                    type Inventory =
                        <EnsmallenGraph as pyo3::class::methods::HasMethodsInventory>::Methods;
                    <Inventory as pyo3::class::methods::PyMethodsInventory>::new(<[_]>::into_vec(
                        box [pyo3::class::PyMethodDefType::Method({
                            unsafe extern "C" fn __wrap(
                                _slf: *mut pyo3::ffi::PyObject,
                                _args: *mut pyo3::ffi::PyObject,
                                _kwargs: *mut pyo3::ffi::PyObject,
                            ) -> *mut pyo3::ffi::PyObject {
                                const _LOCATION: &'static str = "EnsmallenGraph.filter()";
                                {
                                    let pool = ::pyo3::GILPool::new();
                                    let unwind_safe_py =
                                        std::panic::AssertUnwindSafe(pool.python());
                                    let result = match std::panic::catch_unwind(
                                        move || -> ::pyo3::PyResult<_> {
                                            let _py = *unwind_safe_py;
                                            {
                                                let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                let _ref = _cell.try_borrow()?;
                                                let _slf = &_ref;
                                                let _args = _py
                                                    .from_borrowed_ptr::<pyo3::types::PyTuple>(
                                                        _args,
                                                    );
                                                let _kwargs: Option<&pyo3::types::PyDict> =
                                                    _py.from_borrowed_ptr_or_opt(_kwargs);
                                                pyo3::callback::convert(_py, {
                                                    const PARAMS : & 'static [pyo3 :: derive_utils :: ParamDescription] = & [pyo3 :: derive_utils :: ParamDescription { name : "nodes" , is_optional : true , kw_only : false , } , pyo3 :: derive_utils :: ParamDescription { name : "node_types" , is_optional : true , kw_only : false , } , pyo3 :: derive_utils :: ParamDescription { name : "edge_types" , is_optional : true , kw_only : false , } , pyo3 :: derive_utils :: ParamDescription { name : "min_weight" , is_optional : true , kw_only : false , } , pyo3 :: derive_utils :: ParamDescription { name : "max_weight" , is_optional : true , kw_only : false , } , pyo3 :: derive_utils :: ParamDescription { name : "verbose" , is_optional : true , kw_only : false , }] ;
                                                    let mut output = [None; 6usize];
                                                    let mut _args = _args;
                                                    let mut _kwargs = _kwargs;
                                                    let (_args, _kwargs) =
                                                        pyo3::derive_utils::parse_fn_args(
                                                            Some(_LOCATION),
                                                            PARAMS,
                                                            _args,
                                                            _kwargs,
                                                            false,
                                                            false,
                                                            &mut output,
                                                        )?;
                                                    let arg0 = match output[0usize] {
                                                        Some(_obj) => _obj.extract()?,
                                                        None => None,
                                                    };
                                                    let arg1 = match output[1usize] {
                                                        Some(_obj) => _obj.extract()?,
                                                        None => None,
                                                    };
                                                    let arg2 = match output[2usize] {
                                                        Some(_obj) => _obj.extract()?,
                                                        None => None,
                                                    };
                                                    let arg3 = match output[3usize] {
                                                        Some(_obj) => _obj.extract()?,
                                                        None => None,
                                                    };
                                                    let arg4 = match output[4usize] {
                                                        Some(_obj) => _obj.extract()?,
                                                        None => None,
                                                    };
                                                    let arg5 = match output[5usize] {
                                                        Some(_obj) => _obj.extract()?,
                                                        None => None,
                                                    };
                                                    EnsmallenGraph::filter(
                                                        _slf, arg0, arg1, arg2, arg3, arg4, arg5,
                                                    )
                                                })
                                            }
                                        },
                                    ) {
                                        Ok(result) => result,
                                        Err(e) => {
                                            if let Some(string) = e.downcast_ref::<String>() {
                                                Err(::pyo3::panic::PanicException::new_err((
                                                    string.clone(),
                                                )))
                                            } else if let Some(s) = e.downcast_ref::<&str>() {
                                                Err(::pyo3::panic::PanicException::new_err((
                                                    s.to_string(),
                                                )))
                                            } else {
                                                Err(::pyo3::panic::PanicException::new_err((
                                                    "panic from Rust code",
                                                )))
                                            }
                                        }
                                    };
                                    result.unwrap_or_else(|e| {
                                        e.restore(pool.python());
                                        ::pyo3::callback::callback_error()
                                    })
                                }
                            }
                            pyo3 :: class :: PyMethodDef :: cfunction_with_keywords ("filter\u{0}" , __wrap , 0 , "filter($self, node_names, node_types, edge_types, min_weight, max_weight, verbose)\n--\n\nReturn graph filtered by given filters.\n\nParameters\n-------------\n`node_names`: List[str],\n    The node names to keep.\n`node_types`: List[str],\n    The node types to keep.\n`edge_types`: List[str],\n    The edge types to keep.\n`min_weight`: float,\n    Minimum weight to use to filter edges.\n`max_weight`: float,\n    Maximum weight to use to filter edges.\n`verbose`: bool,\n    Wether to show the loading bar.\n\nReturns\n-------------\nThe filtered graph.\u{0}")
                        })],
                    ))
                }
            });
        }
        __init16423468343507468216
    };
}
mod metrics {
    use super::*;
    use graph::{EdgeT, NodeT};
    use numpy::{PyArray, PyArray1};
    use std::collections::HashMap;
    impl EnsmallenGraph {
        /// Returns mean node degree of the graph.
        pub fn degrees_mean(&self) -> f64 {
            self.graph.degrees_mean()
        }
        /// Returns number of connected components in graph.
        ///
        /// Parameters
        /// ------------------------
        /// verbose: bool,
        ///     Wethever to display a loading bar while computing the spanning tree.
        ///
        /// Returns
        /// ------------------------
        /// Number of connected components.
        pub fn connected_components_number(&self, verbose: bool) -> NodeT {
            self.graph.connected_components_number(verbose).0
        }
        /// Returns number of self-loops.
        pub fn get_selfloops_number(&self) -> EdgeT {
            self.graph.get_self_loop_number()
        }
        /// Returns ratio of self-loops.
        pub fn get_selfloops_rate(&self) -> f64 {
            self.graph.get_self_loop_rate()
        }
        /// Returns median node degree of the graph.
        pub fn degrees_median(&self) -> NodeT {
            self.graph.degrees_median()
        }
        /// Returns mode node degree of the graph.
        pub fn degrees_mode(&self) -> NodeT {
            self.graph.degrees_mode()
        }
        /// Returns report relative to the graph metrics.
        ///
        /// The report includes a few useful metrics like:
        ///
        /// * degrees_median: the median degree of the nodes.
        /// * degrees_mean: the mean degree of the nodes.
        /// * degrees_mode: the mode degree of the nodes.
        /// * degrees_max: the max degree of the nodes.
        /// * degrees_min: the min degree of the nodes.
        /// * nodes_number: the number of nodes in the graph.
        /// * edges_number: the number of edges in the graph.
        /// * unique_node_types_number: the number of different node types in the graph.
        /// * unique_edge_types_number: the number of different edge types in the graph.
        /// * traps_rate: probability to end up in a trap when starting into any given node.
        /// * selfloops_rate: pecentage of edges that are selfloops.
        ///
        fn report(&self) -> HashMap<&str, String> {
            self.graph.report()
        }
        /// Return report on overlaps of the two graphs.
        ///
        /// Parameters
        /// -------------------
        /// other: &EnsmallenGraph,
        ///     Graph to compute the overlaps with.
        /// verbose: bool = True,
        ///     Wether to show loading bars.
        ///
        /// Returns
        /// -------------------
        /// Textual report.
        fn overlap_textual_report(
            &self,
            other: &EnsmallenGraph,
            verbose: Option<bool>,
        ) -> PyResult<String> {
            match self
                .graph
                .overlap_textual_report(&other.graph, verbose.unwrap_or(true))
            {
                Ok(v) => Ok(v),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }
        }
        /// Return the degree for the given node.
        ///
        /// Parameters
        /// ---------------------
        /// node: int,
        ///     Node ID to use to compute degrees product.
        ///
        /// Returns
        /// ----------------------------
        /// degrees product for the two given nodes.
        ///
        fn degree(&self, node: NodeT) -> NodeT {
            self.graph.get_node_degree(node)
        }
        /// Return all the degrees of the nodes graph.
        ///
        /// Returns
        /// ----------------------------
        /// Numpy array with all the degrees of the graph.
        ///
        fn degrees(&self) -> PyResult<Py<PyArray1<EdgeT>>> {
            let degrees = self.graph.get_node_degrees();
            let gil = pyo3::Python::acquire_gil();
            Ok(
                match PyArray::from_vec(gil.python(), degrees).cast::<EdgeT>(false) {
                    Ok(v) => Ok(v),
                    Err(_) => Err(PyErr::new::<exceptions::ValueError, _>({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["The given array cannot be casted to ", "."],
                            &match (&"EdgeT",) {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ));
                        res
                    })),
                }?
                .to_owned(),
            )
        }
        /// Return the Jaccard Index for the two given nodes.
        ///
        /// Parameters
        /// ---------------------
        /// one: int,
        ///     First node ID to use to compute Jaccard Index.
        /// two: int,
        ///     Second node ID to use to compute Jaccard Index.
        ///
        /// Returns
        /// ----------------------------
        /// Jaccard Index for the two given nodes.
        ///
        fn jaccard_index(&self, one: NodeT, two: NodeT) -> PyResult<f64> {
            match self.graph.jaccard_index(one, two) {
                Ok(v) => Ok(v),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }
        }
        /// Return the Adamic/Adar for the two given nodes.
        ///
        /// Parameters
        /// ---------------------
        /// one: int,
        ///     First node ID to use to compute Adamic/Adar.
        /// two: int,
        ///     Second node ID to use to compute Adamic/Adar.
        ///
        /// Returns
        /// ----------------------------
        /// Adamic/Adar for the two given nodes.
        ///
        fn adamic_adar_index(&self, one: NodeT, two: NodeT) -> PyResult<f64> {
            match self.graph.adamic_adar_index(one, two) {
                Ok(v) => Ok(v),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }
        }
        /// Return the Resource Allocation Index for the two given nodes.
        ///
        /// Parameters
        /// ---------------------
        /// one: int,
        ///     First node ID to use to compute Resource Allocation Index.
        /// two: int,
        ///     Second node ID to use to compute Resource Allocation Index.
        ///
        /// Returns
        /// ----------------------------
        /// Resource Allocation Index for the two given nodes.
        ///
        fn resource_allocation_index(&self, one: NodeT, two: NodeT) -> PyResult<f64> {
            match self.graph.resource_allocation_index(one, two) {
                Ok(v) => Ok(v),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }
        }
        /// Return the degrees product for the two given nodes.
        ///
        /// Parameters
        /// ---------------------
        /// one: int,
        ///     First node ID to use to compute degrees product.
        /// two: int,
        ///     Second node ID to use to compute degrees product.
        ///
        /// Returns
        /// ----------------------------
        /// degrees product for the two given nodes.
        ///
        fn degrees_product(&self, one: NodeT, two: NodeT) -> PyResult<usize> {
            match self.graph.degrees_product(one, two) {
                Ok(v) => Ok(v),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }
        }
        /// Return the traps rate of the graph.
        ///
        /// This feature is EXPERIMENTAL and still required proving.
        ///
        fn traps_rate(&self) -> f64 {
            self.graph.traps_rate()
        }
    }
    #[used]
    #[allow(non_upper_case_globals)]
    #[link_section = ".init_array"]
    #[allow(non_upper_case_globals)]
    static __init16690134668753049802: extern "C" fn() = {
        #[link_section = ".text.startup"]
        extern "C" fn __init16690134668753049802() {
            pyo3::inventory::submit({
                {
                    type Inventory =
                        <EnsmallenGraph as pyo3::class::methods::HasMethodsInventory>::Methods;
                    <Inventory as pyo3::class::methods::PyMethodsInventory>::new(<[_]>::into_vec(
                        box [
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str = "EnsmallenGraph.degrees_mean()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    pyo3::callback::convert(
                                                        _py,
                                                        EnsmallenGraph::degrees_mean(_slf),
                                                    )
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction ("degrees_mean\u{0}" , __wrap , "degrees_mean($self)\n--\n\nReturns mean node degree of the graph.\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                    _kwargs: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.connected_components_number()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    let _args = _py
                                                        .from_borrowed_ptr::<pyo3::types::PyTuple>(
                                                            _args,
                                                        );
                                                    let _kwargs: Option<&pyo3::types::PyDict> =
                                                        _py.from_borrowed_ptr_or_opt(_kwargs);
                                                    pyo3::callback::convert(_py, {
                                                        const PARAMS : & 'static [pyo3 :: derive_utils :: ParamDescription] = & [pyo3 :: derive_utils :: ParamDescription { name : "verbose" , is_optional : false , kw_only : false , }] ;
                                                        let mut output = [None; 1usize];
                                                        let mut _args = _args;
                                                        let mut _kwargs = _kwargs;
                                                        let (_args, _kwargs) =
                                                            pyo3::derive_utils::parse_fn_args(
                                                                Some(_LOCATION),
                                                                PARAMS,
                                                                _args,
                                                                _kwargs,
                                                                false,
                                                                false,
                                                                &mut output,
                                                            )?;
                                                        let arg0 = match output [0usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        EnsmallenGraph::connected_components_number(
                                                            _slf, arg0,
                                                        )
                                                    })
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction_with_keywords ("connected_components_number\u{0}" , __wrap , 0 , "connected_components_number($self, verbose)\n--\n\nReturns number of connected components in graph.\n\nParameters\n------------------------\nverbose: bool,\n    Wethever to display a loading bar while computing the spanning tree.\n\nReturns\n------------------------\nNumber of connected components.\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.get_selfloops_number()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    pyo3::callback::convert(
                                                        _py,
                                                        EnsmallenGraph::get_selfloops_number(_slf),
                                                    )
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction ("get_selfloops_number\u{0}" , __wrap , "get_selfloops_number($self)\n--\n\nReturns number of self-loops.\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.get_selfloops_rate()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    pyo3::callback::convert(
                                                        _py,
                                                        EnsmallenGraph::get_selfloops_rate(_slf),
                                                    )
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction ("get_selfloops_rate\u{0}" , __wrap , "get_selfloops_rate($self)\n--\n\nReturns ratio of self-loops.\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.degrees_median()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    pyo3::callback::convert(
                                                        _py,
                                                        EnsmallenGraph::degrees_median(_slf),
                                                    )
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction ("degrees_median\u{0}" , __wrap , "degrees_median($self)\n--\n\nReturns median node degree of the graph.\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str = "EnsmallenGraph.degrees_mode()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    pyo3::callback::convert(
                                                        _py,
                                                        EnsmallenGraph::degrees_mode(_slf),
                                                    )
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction ("degrees_mode\u{0}" , __wrap , "degrees_mode($self)\n--\n\nReturns mode node degree of the graph.\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str = "EnsmallenGraph.report()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    pyo3::callback::convert(
                                                        _py,
                                                        EnsmallenGraph::report(_slf),
                                                    )
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction ("report\u{0}" , __wrap , "report($self)\n--\n\nReturns report relative to the graph metrics.\n\nThe report includes a few useful metrics like:\n\n* degrees_median: the median degree of the nodes.\n* degrees_mean: the mean degree of the nodes.\n* degrees_mode: the mode degree of the nodes.\n* degrees_max: the max degree of the nodes.\n* degrees_min: the min degree of the nodes.\n* nodes_number: the number of nodes in the graph.\n* edges_number: the number of edges in the graph.\n* unique_node_types_number: the number of different node types in the graph.\n* unique_edge_types_number: the number of different edge types in the graph.\n* traps_rate: probability to end up in a trap when starting into any given node.\n* selfloops_rate: pecentage of edges that are selfloops.\n\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                    _kwargs: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.overlap_textual_report()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    let _args = _py
                                                        .from_borrowed_ptr::<pyo3::types::PyTuple>(
                                                            _args,
                                                        );
                                                    let _kwargs: Option<&pyo3::types::PyDict> =
                                                        _py.from_borrowed_ptr_or_opt(_kwargs);
                                                    pyo3::callback::convert(_py, {
                                                        const PARAMS : & 'static [pyo3 :: derive_utils :: ParamDescription] = & [pyo3 :: derive_utils :: ParamDescription { name : "other" , is_optional : false , kw_only : false , } , pyo3 :: derive_utils :: ParamDescription { name : "verbose" , is_optional : true , kw_only : false , }] ;
                                                        let mut output = [None; 2usize];
                                                        let mut _args = _args;
                                                        let mut _kwargs = _kwargs;
                                                        let (_args, _kwargs) =
                                                            pyo3::derive_utils::parse_fn_args(
                                                                Some(_LOCATION),
                                                                PARAMS,
                                                                _args,
                                                                _kwargs,
                                                                false,
                                                                false,
                                                                &mut output,
                                                            )?;
                                                        let _tmp : < & EnsmallenGraph as pyo3 :: derive_utils :: ExtractExt > :: Target = match output [0usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        let arg0 = &*_tmp;
                                                        let arg1 = match output[1usize] {
                                                            Some(_obj) => _obj.extract()?,
                                                            None => None,
                                                        };
                                                        EnsmallenGraph::overlap_textual_report(
                                                            _slf, arg0, arg1,
                                                        )
                                                    })
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction_with_keywords ("overlap_textual_report\u{0}" , __wrap , 0 , "Return report on overlaps of the two graphs.\n\nParameters\n-------------------\nother: &EnsmallenGraph,\n    Graph to compute the overlaps with.\nverbose: bool = True,\n    Wether to show loading bars.\n\nReturns\n-------------------\nTextual report.\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                    _kwargs: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str = "EnsmallenGraph.degree()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    let _args = _py
                                                        .from_borrowed_ptr::<pyo3::types::PyTuple>(
                                                            _args,
                                                        );
                                                    let _kwargs: Option<&pyo3::types::PyDict> =
                                                        _py.from_borrowed_ptr_or_opt(_kwargs);
                                                    pyo3::callback::convert(_py, {
                                                        const PARAMS : & 'static [pyo3 :: derive_utils :: ParamDescription] = & [pyo3 :: derive_utils :: ParamDescription { name : "node" , is_optional : false , kw_only : false , }] ;
                                                        let mut output = [None; 1usize];
                                                        let mut _args = _args;
                                                        let mut _kwargs = _kwargs;
                                                        let (_args, _kwargs) =
                                                            pyo3::derive_utils::parse_fn_args(
                                                                Some(_LOCATION),
                                                                PARAMS,
                                                                _args,
                                                                _kwargs,
                                                                false,
                                                                false,
                                                                &mut output,
                                                            )?;
                                                        let arg0 = match output [0usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        EnsmallenGraph::degree(_slf, arg0)
                                                    })
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction_with_keywords ("degree\u{0}" , __wrap , 0 , "degree($self, node)\n--\n\nReturn the degree for the given node.\n\nParameters\n---------------------\nnode: int,\n    Node ID to use to compute degrees product.\n\nReturns\n----------------------------\ndegrees product for the two given nodes.\n\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str = "EnsmallenGraph.degrees()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    pyo3::callback::convert(
                                                        _py,
                                                        EnsmallenGraph::degrees(_slf),
                                                    )
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction ("degrees\u{0}" , __wrap , "degrees($self)\n--\n\nReturn all the degrees of the nodes graph.\n\nReturns\n----------------------------\nNumpy array with all the degrees of the graph.\n\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                    _kwargs: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.jaccard_index()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    let _args = _py
                                                        .from_borrowed_ptr::<pyo3::types::PyTuple>(
                                                            _args,
                                                        );
                                                    let _kwargs: Option<&pyo3::types::PyDict> =
                                                        _py.from_borrowed_ptr_or_opt(_kwargs);
                                                    pyo3::callback::convert(_py, {
                                                        const PARAMS : & 'static [pyo3 :: derive_utils :: ParamDescription] = & [pyo3 :: derive_utils :: ParamDescription { name : "one" , is_optional : false , kw_only : false , } , pyo3 :: derive_utils :: ParamDescription { name : "two" , is_optional : false , kw_only : false , }] ;
                                                        let mut output = [None; 2usize];
                                                        let mut _args = _args;
                                                        let mut _kwargs = _kwargs;
                                                        let (_args, _kwargs) =
                                                            pyo3::derive_utils::parse_fn_args(
                                                                Some(_LOCATION),
                                                                PARAMS,
                                                                _args,
                                                                _kwargs,
                                                                false,
                                                                false,
                                                                &mut output,
                                                            )?;
                                                        let arg0 = match output [0usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        let arg1 = match output [1usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        EnsmallenGraph::jaccard_index(
                                                            _slf, arg0, arg1,
                                                        )
                                                    })
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction_with_keywords ("jaccard_index\u{0}" , __wrap , 0 , "jaccard_index($self, one, two)\n--\n\nReturn the Jaccard Index for the two given nodes.\n\nParameters\n---------------------\none: int,\n    First node ID to use to compute Jaccard Index.\ntwo: int,\n    Second node ID to use to compute Jaccard Index.\n\nReturns\n----------------------------\nJaccard Index for the two given nodes.\n\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                    _kwargs: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.adamic_adar_index()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    let _args = _py
                                                        .from_borrowed_ptr::<pyo3::types::PyTuple>(
                                                            _args,
                                                        );
                                                    let _kwargs: Option<&pyo3::types::PyDict> =
                                                        _py.from_borrowed_ptr_or_opt(_kwargs);
                                                    pyo3::callback::convert(_py, {
                                                        const PARAMS : & 'static [pyo3 :: derive_utils :: ParamDescription] = & [pyo3 :: derive_utils :: ParamDescription { name : "one" , is_optional : false , kw_only : false , } , pyo3 :: derive_utils :: ParamDescription { name : "two" , is_optional : false , kw_only : false , }] ;
                                                        let mut output = [None; 2usize];
                                                        let mut _args = _args;
                                                        let mut _kwargs = _kwargs;
                                                        let (_args, _kwargs) =
                                                            pyo3::derive_utils::parse_fn_args(
                                                                Some(_LOCATION),
                                                                PARAMS,
                                                                _args,
                                                                _kwargs,
                                                                false,
                                                                false,
                                                                &mut output,
                                                            )?;
                                                        let arg0 = match output [0usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        let arg1 = match output [1usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        EnsmallenGraph::adamic_adar_index(
                                                            _slf, arg0, arg1,
                                                        )
                                                    })
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction_with_keywords ("adamic_adar_index\u{0}" , __wrap , 0 , "adamic_adar_index($self, one, two)\n--\n\nReturn the Adamic/Adar for the two given nodes.\n\nParameters\n---------------------\none: int,\n    First node ID to use to compute Adamic/Adar.\ntwo: int,\n    Second node ID to use to compute Adamic/Adar.\n\nReturns\n----------------------------\nAdamic/Adar for the two given nodes.\n\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                    _kwargs: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.resource_allocation_index()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    let _args = _py
                                                        .from_borrowed_ptr::<pyo3::types::PyTuple>(
                                                            _args,
                                                        );
                                                    let _kwargs: Option<&pyo3::types::PyDict> =
                                                        _py.from_borrowed_ptr_or_opt(_kwargs);
                                                    pyo3::callback::convert(_py, {
                                                        const PARAMS : & 'static [pyo3 :: derive_utils :: ParamDescription] = & [pyo3 :: derive_utils :: ParamDescription { name : "one" , is_optional : false , kw_only : false , } , pyo3 :: derive_utils :: ParamDescription { name : "two" , is_optional : false , kw_only : false , }] ;
                                                        let mut output = [None; 2usize];
                                                        let mut _args = _args;
                                                        let mut _kwargs = _kwargs;
                                                        let (_args, _kwargs) =
                                                            pyo3::derive_utils::parse_fn_args(
                                                                Some(_LOCATION),
                                                                PARAMS,
                                                                _args,
                                                                _kwargs,
                                                                false,
                                                                false,
                                                                &mut output,
                                                            )?;
                                                        let arg0 = match output [0usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        let arg1 = match output [1usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        EnsmallenGraph::resource_allocation_index(
                                                            _slf, arg0, arg1,
                                                        )
                                                    })
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction_with_keywords ("resource_allocation_index\u{0}" , __wrap , 0 , "resource_allocation_index($self, one, two)\n--\n\nReturn the Resource Allocation Index for the two given nodes.\n\nParameters\n---------------------\none: int,\n    First node ID to use to compute Resource Allocation Index.\ntwo: int,\n    Second node ID to use to compute Resource Allocation Index.\n\nReturns\n----------------------------\nResource Allocation Index for the two given nodes.\n\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                    _kwargs: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.degrees_product()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    let _args = _py
                                                        .from_borrowed_ptr::<pyo3::types::PyTuple>(
                                                            _args,
                                                        );
                                                    let _kwargs: Option<&pyo3::types::PyDict> =
                                                        _py.from_borrowed_ptr_or_opt(_kwargs);
                                                    pyo3::callback::convert(_py, {
                                                        const PARAMS : & 'static [pyo3 :: derive_utils :: ParamDescription] = & [pyo3 :: derive_utils :: ParamDescription { name : "one" , is_optional : false , kw_only : false , } , pyo3 :: derive_utils :: ParamDescription { name : "two" , is_optional : false , kw_only : false , }] ;
                                                        let mut output = [None; 2usize];
                                                        let mut _args = _args;
                                                        let mut _kwargs = _kwargs;
                                                        let (_args, _kwargs) =
                                                            pyo3::derive_utils::parse_fn_args(
                                                                Some(_LOCATION),
                                                                PARAMS,
                                                                _args,
                                                                _kwargs,
                                                                false,
                                                                false,
                                                                &mut output,
                                                            )?;
                                                        let arg0 = match output [0usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        let arg1 = match output [1usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        EnsmallenGraph::degrees_product(
                                                            _slf, arg0, arg1,
                                                        )
                                                    })
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction_with_keywords ("degrees_product\u{0}" , __wrap , 0 , "degrees_product($self, one, two)\n--\n\nReturn the degrees product for the two given nodes.\n\nParameters\n---------------------\none: int,\n    First node ID to use to compute degrees product.\ntwo: int,\n    Second node ID to use to compute degrees product.\n\nReturns\n----------------------------\ndegrees product for the two given nodes.\n\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str = "EnsmallenGraph.traps_rate()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    pyo3::callback::convert(
                                                        _py,
                                                        EnsmallenGraph::traps_rate(_slf),
                                                    )
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction ("traps_rate\u{0}" , __wrap , "traps_rate(self)\n--\n\nReturn the traps rate of the graph.\n\nThis feature is EXPERIMENTAL and still required proving.\n\u{0}")
                            }),
                        ],
                    ))
                }
            });
        }
        __init16690134668753049802
    };
}
mod node_file_writer {
    use super::*;
    use graph::NodeFileWriter;
    impl EnsmallenGraph {
        /// Write to disk the nodes (and optionally the metadata) of the graph.
        ///
        /// Parameters
        /// ------------------------
        /// path: str,
        ///     Path where to save the nodes and their metadata.
        /// verbose: bool = True,
        ///     Wether to show a loading bar while writing to file.
        /// separator: str = "\t",
        ///     What separator to use while writing out to file.
        /// header: bool = True,
        ///     Wether to write out the header of the file.
        /// nodes_column_number: int = 0,
        ///     The column number where to write the nodes.
        /// nodes_column: str = "id",
        ///     The name of the column of the nodes.
        /// node_types_column_number: int = 1,
        ///     The column number where to write the node types.
        /// nodes_type_column: str = "category",
        ///     The name of the column of the node types.
        ///
        /// Raises
        /// ------------------------
        /// TODO: update the set of exceptions
        ///
        fn dump_nodes(&self, path: String, py_kwargs: Option<&PyDict>) -> PyResult<()> {
            let py = pyo3::Python::acquire_gil();
            let kwargs = match py_kwargs {
                Some(v) => v,
                None => PyDict::new(py.python()),
            };
            match validate_kwargs(
                kwargs,
                [
                    "verbose",
                    "separator",
                    "header",
                    "nodes_column_number",
                    "nodes_column",
                    "node_types_column_number",
                    "nodes_type_column",
                ]
                .iter()
                .map(|x| x.to_string())
                .collect(),
            ) {
                Ok(v) => Ok(v),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }?;
            let writer = NodeFileWriter::new(path)
                .set_verbose(match match kwargs.get_item("verbose") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<bool>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (&"verbose", &value.get_type().name(), &"bool") {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                } {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?)
                .set_separator(match match kwargs.get_item("separator") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<String>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (&"separator", &value.get_type().name(), &"String") {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                } {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?)
                .set_header(match match kwargs.get_item("header") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<bool>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (&"header", &value.get_type().name(), &"bool") {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                } {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?)
                .set_nodes_column_number(match match kwargs.get_item("nodes_column_number") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<usize>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (
                                            &"nodes_column_number",
                                            &value.get_type().name(),
                                            &"usize",
                                        ) {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                } {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?)
                .set_nodes_column(match match kwargs.get_item("nodes_column") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<String>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (
                                            &"nodes_column",
                                            &value.get_type().name(),
                                            &"String",
                                        ) {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                } {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?)
                .set_node_types_column_number(match match kwargs
                    .get_item("node_types_column_number")
                {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<usize>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (
                                            &"node_types_column_number",
                                            &value.get_type().name(),
                                            &"usize",
                                        ) {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                } {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?)
                .set_node_types_column(match match kwargs.get_item("nodes_type_column") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<String>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (
                                            &"nodes_type_column",
                                            &value.get_type().name(),
                                            &"String",
                                        ) {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                } {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?);
            match writer.dump(&self.graph) {
                Ok(v) => Ok(v),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }
        }
    }
    #[used]
    #[allow(non_upper_case_globals)]
    #[link_section = ".init_array"]
    #[allow(non_upper_case_globals)]
    static __init2632423851270273969: extern "C" fn() = {
        #[link_section = ".text.startup"]
        extern "C" fn __init2632423851270273969() {
            pyo3::inventory::submit({
                {
                    type Inventory =
                        <EnsmallenGraph as pyo3::class::methods::HasMethodsInventory>::Methods;
                    <Inventory as pyo3::class::methods::PyMethodsInventory>::new(<[_]>::into_vec(
                        box [pyo3::class::PyMethodDefType::Method({
                            unsafe extern "C" fn __wrap(
                                _slf: *mut pyo3::ffi::PyObject,
                                _args: *mut pyo3::ffi::PyObject,
                                _kwargs: *mut pyo3::ffi::PyObject,
                            ) -> *mut pyo3::ffi::PyObject {
                                const _LOCATION: &'static str = "EnsmallenGraph.dump_nodes()";
                                {
                                    let pool = ::pyo3::GILPool::new();
                                    let unwind_safe_py =
                                        std::panic::AssertUnwindSafe(pool.python());
                                    let result = match std::panic::catch_unwind(
                                        move || -> ::pyo3::PyResult<_> {
                                            let _py = *unwind_safe_py;
                                            {
                                                let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                let _ref = _cell.try_borrow()?;
                                                let _slf = &_ref;
                                                let _args = _py
                                                    .from_borrowed_ptr::<pyo3::types::PyTuple>(
                                                        _args,
                                                    );
                                                let _kwargs: Option<&pyo3::types::PyDict> =
                                                    _py.from_borrowed_ptr_or_opt(_kwargs);
                                                pyo3::callback::convert(_py, {
                                                    const PARAMS : & 'static [pyo3 :: derive_utils :: ParamDescription] = & [pyo3 :: derive_utils :: ParamDescription { name : "path" , is_optional : false , kw_only : false , }] ;
                                                    let mut output = [None; 1usize];
                                                    let mut _args = _args;
                                                    let mut _kwargs = _kwargs;
                                                    let (_args, _kwargs) =
                                                        pyo3::derive_utils::parse_fn_args(
                                                            Some(_LOCATION),
                                                            PARAMS,
                                                            _args,
                                                            _kwargs,
                                                            false,
                                                            true,
                                                            &mut output,
                                                        )?;
                                                    let arg0 = match output [0usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                    let arg1 = _kwargs;
                                                    EnsmallenGraph::dump_nodes(_slf, arg0, arg1)
                                                })
                                            }
                                        },
                                    ) {
                                        Ok(result) => result,
                                        Err(e) => {
                                            if let Some(string) = e.downcast_ref::<String>() {
                                                Err(::pyo3::panic::PanicException::new_err((
                                                    string.clone(),
                                                )))
                                            } else if let Some(s) = e.downcast_ref::<&str>() {
                                                Err(::pyo3::panic::PanicException::new_err((
                                                    s.to_string(),
                                                )))
                                            } else {
                                                Err(::pyo3::panic::PanicException::new_err((
                                                    "panic from Rust code",
                                                )))
                                            }
                                        }
                                    };
                                    result.unwrap_or_else(|e| {
                                        e.restore(pool.python());
                                        ::pyo3::callback::callback_error()
                                    })
                                }
                            }
                            pyo3 :: class :: PyMethodDef :: cfunction_with_keywords ("dump_nodes\u{0}" , __wrap , 0 , "dump_nodes($self, path, *, verbose, separator, header, nodes_column_number, nodes_column, node_types_column_number, nodes_type_column)\n--\n\nWrite to disk the nodes (and optionally the metadata) of the graph.\n\nParameters\n------------------------\npath: str,\n    Path where to save the nodes and their metadata.\nverbose: bool = True,\n    Wether to show a loading bar while writing to file.\nseparator: str = \"\\t\",\n    What separator to use while writing out to file.\nheader: bool = True,\n    Wether to write out the header of the file.\nnodes_column_number: int = 0,\n    The column number where to write the nodes.\nnodes_column: str = \"id\",\n    The name of the column of the nodes.\nnode_types_column_number: int = 1,\n    The column number where to write the node types.\nnodes_type_column: str = \"category\",\n    The name of the column of the node types.\n\nRaises\n------------------------\nTODO: update the set of exceptions\n\u{0}")
                        })],
                    ))
                }
            });
        }
        __init2632423851270273969
    };
}
mod preprocessing {
    use super::*;
    use rayon::prelude::*;
    use graph::{cooccurence_matrix as rust_cooccurence_matrix, word2vec as rust_word2vec, NodeT};
    use numpy::{PyArray, PyArray1};
    fn preprocessing(_py: Python, m: &PyModule) -> PyResult<()> {
        m.add_wrapped({
            &{
                #[allow(dead_code)]
                enum ProcMacroHack {
                    Value = ("[< __pyo3_get_function_ word2vec >]", 0).1,
                }
                {
                    __pyo3_get_function_word2vec
                }
            }
        })?;
        m.add_wrapped({
            &{
                #[allow(dead_code)]
                enum ProcMacroHack {
                    Value = ("[< __pyo3_get_function_ cooccurence_matrix >]", 0).1,
                }
                {
                    __pyo3_get_function_cooccurence_matrix
                }
            }
        })?;
        Ok(())
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    /// This autogenerated function is called by the python interpreter when importing
    /// the module.
    pub unsafe extern "C" fn PyInit_preprocessing() -> *mut pyo3::ffi::PyObject {
        use pyo3::derive_utils::ModuleDef;
        const NAME: &'static str = "preprocessing\u{0}";
        static MODULE_DEF: ModuleDef = unsafe { ModuleDef::new(NAME) };
        {
            {
                let pool = ::pyo3::GILPool::new();
                let unwind_safe_py = std::panic::AssertUnwindSafe(pool.python());
                let result = match std::panic::catch_unwind(move || -> ::pyo3::PyResult<_> {
                    let _py = *unwind_safe_py;
                    ::pyo3::callback::convert(_py, { MODULE_DEF.make_module("", preprocessing) })
                }) {
                    Ok(result) => result,
                    Err(e) => {
                        if let Some(string) = e.downcast_ref::<String>() {
                            Err(::pyo3::panic::PanicException::new_err((string.clone(),)))
                        } else if let Some(s) = e.downcast_ref::<&str>() {
                            Err(::pyo3::panic::PanicException::new_err((s.to_string(),)))
                        } else {
                            Err(::pyo3::panic::PanicException::new_err((
                                "panic from Rust code",
                            )))
                        }
                    }
                };
                result.unwrap_or_else(|e| {
                    e.restore(pool.python());
                    ::pyo3::callback::callback_error()
                })
            }
        }
    }
    /// Return training batches for Word2Vec models.
    ///
    /// The batch is composed of a tuple as the following:
    ///
    /// - (Contexts indices, central nodes indices): the tuple of nodes
    ///
    /// This does not provide any output value as the model uses NCE loss
    /// and basically the central nodes that are fed as inputs work as the
    /// outputs value.
    ///
    /// Arguments
    /// ---------
    ///
    /// sequences: List[List[int]],
    ///     the sequence of sequences of integers to preprocess.
    /// window_size: int,
    ///     Window size to consider for the sequences.
    ///
    fn word2vec(sequences: Vec<Vec<NodeT>>, window_size: usize) -> PyResult<(PyContexts, PyWords)> {
        let (contexts, words) = match rust_word2vec(sequences.into_par_iter(), window_size) {
            Ok(v) => Ok(v),
            Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
        }?;
        let gil = pyo3::Python::acquire_gil();
        Ok((
            match match PyArray::from_vec2(gil.python(), &contexts) {
                Ok(v) => Ok(v),
                Err(_) => Err(PyErr::new::<exceptions::ValueError, _>(
                    "The given value cannot be casted to a 2d numpy array.",
                )),
            }?
            .cast::<NodeT>(false)
            {
                Ok(v) => Ok(v),
                Err(_) => Err(PyErr::new::<exceptions::ValueError, _>({
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["The given 2d array cannot be casted to ", "."],
                        &match (&"NodeT",) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ));
                    res
                })),
            }?
            .to_owned(),
            match PyArray::from_vec(gil.python(), words).cast::<NodeT>(false) {
                Ok(v) => Ok(v),
                Err(_) => Err(PyErr::new::<exceptions::ValueError, _>({
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["The given array cannot be casted to ", "."],
                        &match (&"NodeT",) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ));
                    res
                })),
            }?
            .to_owned(),
        ))
    }
    unsafe extern "C" fn __pyo3_raw_word2vec(
        _slf: *mut pyo3::ffi::PyObject,
        _args: *mut pyo3::ffi::PyObject,
        _kwargs: *mut pyo3::ffi::PyObject,
    ) -> *mut pyo3::ffi::PyObject {
        const _LOCATION: &'static str = "word2vec()";
        {
            {
                let pool = ::pyo3::GILPool::new();
                let unwind_safe_py = std::panic::AssertUnwindSafe(pool.python());
                let result = match std::panic::catch_unwind(move || -> ::pyo3::PyResult<_> {
                    let _py = *unwind_safe_py;
                    ::pyo3::callback::convert(_py, {
                        let _args = _py.from_borrowed_ptr::<pyo3::types::PyTuple>(_args);
                        let _kwargs: Option<&pyo3::types::PyDict> =
                            _py.from_borrowed_ptr_or_opt(_kwargs);
                        {
                            const PARAMS: &'static [pyo3::derive_utils::ParamDescription] = &[
                                pyo3::derive_utils::ParamDescription {
                                    name: "sequences",
                                    is_optional: false,
                                    kw_only: false,
                                },
                                pyo3::derive_utils::ParamDescription {
                                    name: "window_size",
                                    is_optional: false,
                                    kw_only: false,
                                },
                            ];
                            let mut output = [None; 2usize];
                            let mut _args = _args;
                            let mut _kwargs = _kwargs;
                            let (_args, _kwargs) = pyo3::derive_utils::parse_fn_args(
                                Some(_LOCATION),
                                PARAMS,
                                _args,
                                _kwargs,
                                false,
                                true,
                                &mut output,
                            )?;
                            let arg0 = match output[0usize] {
                                Some(_obj) => _obj.extract()?,
                                None => ::std::rt::begin_panic(
                                    "Failed to extract required method argument",
                                ),
                            };
                            let arg1 = match output[1usize] {
                                Some(_obj) => _obj.extract()?,
                                None => ::std::rt::begin_panic(
                                    "Failed to extract required method argument",
                                ),
                            };
                            word2vec(arg0, arg1)
                        }
                    })
                }) {
                    Ok(result) => result,
                    Err(e) => {
                        if let Some(string) = e.downcast_ref::<String>() {
                            Err(::pyo3::panic::PanicException::new_err((string.clone(),)))
                        } else if let Some(s) = e.downcast_ref::<&str>() {
                            Err(::pyo3::panic::PanicException::new_err((s.to_string(),)))
                        } else {
                            Err(::pyo3::panic::PanicException::new_err((
                                "panic from Rust code",
                            )))
                        }
                    }
                };
                result.unwrap_or_else(|e| {
                    e.restore(pool.python());
                    ::pyo3::callback::callback_error()
                })
            }
        }
    }
    fn __pyo3_get_function_word2vec<'a>(
        args: impl Into<pyo3::derive_utils::PyFunctionArguments<'a>>,
    ) -> pyo3::PyResult<&'a pyo3::types::PyCFunction> {
        let name = "word2vec\u{0}";
        let name = std::ffi::CStr::from_bytes_with_nul(name.as_bytes()).unwrap();
        let doc = std :: ffi :: CStr :: from_bytes_with_nul (b"word2vec(sequences, window_size)\n--\n\nReturn training batches for Word2Vec models.\n\nThe batch is composed of a tuple as the following:\n\n- (Contexts indices, central nodes indices): the tuple of nodes\n\nThis does not provide any output value as the model uses NCE loss\nand basically the central nodes that are fed as inputs work as the\noutputs value.\n\nArguments\n---------\n\nsequences: List[List[int]],\n    the sequence of sequences of integers to preprocess.\nwindow_size: int,\n    Window size to consider for the sequences.\n\x00") . unwrap () ;
        pyo3::types::PyCFunction::internal_new(
            name,
            doc,
            pyo3::class::PyMethodType::PyCFunctionWithKeywords(__pyo3_raw_word2vec),
            pyo3::ffi::METH_VARARGS | pyo3::ffi::METH_KEYWORDS,
            args.into(),
        )
    }
    /// Return triple with CSR representation of cooccurrence matrix.
    ///
    /// The first vector has the sources, the second vector the destinations
    /// and the third one contains the min-max normalized frequencies.
    ///
    /// Arguments
    /// ---------
    ///
    /// sequences: List[List[int]],
    ///     the sequence of sequences of integers to preprocess.
    /// window_size: int = 4,
    ///     Window size to consider for the sequences.
    /// verbose: bool = False,
    ///     Wethever to show the progress bars.
    ///     The default behaviour is false.
    ///     
    fn cooccurence_matrix(
        sequences: Vec<Vec<NodeT>>,
        py_kwargs: Option<&PyDict>,
    ) -> PyResult<(PyWords, PyWords, PyFrequencies)> {
        let gil = pyo3::Python::acquire_gil();
        let kwargs = match py_kwargs {
            Some(v) => v,
            None => PyDict::new(gil.python()),
        };
        match validate_kwargs(
            kwargs,
            ["window_size", "verbose"]
                .iter()
                .map(|x| x.to_string())
                .collect(),
        ) {
            Ok(v) => Ok(v),
            Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
        }?;
        let (words, contexts, frequencies) = match rust_cooccurence_matrix(
            sequences.into_par_iter(),
            match match kwargs.get_item("window_size") {
                None => Ok(None),
                Some(value) => {
                    if value.get_type().name() == "NoneType" {
                        Ok(None)
                    } else {
                        match value.extract::<usize>() {
                            Ok(v) => Ok(Some(v)),
                            Err(_) => Err({
                                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                    &[
                                        "The value passed for ",
                                        " cannot be casted from ",
                                        " to ",
                                        ".",
                                    ],
                                    &match (&"window_size", &value.get_type().name(), &"usize") {
                                        (arg0, arg1, arg2) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg2,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ));
                                res
                            }),
                        }
                    }
                }
            } {
                Ok(v) => Ok(v),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }?
            .unwrap_or(3),
            sequences.len(),
            match match kwargs.get_item("verbose") {
                None => Ok(None),
                Some(value) => {
                    if value.get_type().name() == "NoneType" {
                        Ok(None)
                    } else {
                        match value.extract::<bool>() {
                            Ok(v) => Ok(Some(v)),
                            Err(_) => Err({
                                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                    &[
                                        "The value passed for ",
                                        " cannot be casted from ",
                                        " to ",
                                        ".",
                                    ],
                                    &match (&"verbose", &value.get_type().name(), &"bool") {
                                        (arg0, arg1, arg2) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg2,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ));
                                res
                            }),
                        }
                    }
                }
            } {
                Ok(v) => Ok(v),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }?
            .unwrap_or(true),
        ) {
            Ok(v) => Ok(v),
            Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
        }?;
        Ok((
            match PyArray::from_vec(gil.python(), words).cast::<NodeT>(false) {
                Ok(v) => Ok(v),
                Err(_) => Err(PyErr::new::<exceptions::ValueError, _>({
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["The given array cannot be casted to ", "."],
                        &match (&"NodeT",) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ));
                    res
                })),
            }?
            .to_owned(),
            match PyArray::from_vec(gil.python(), contexts).cast::<NodeT>(false) {
                Ok(v) => Ok(v),
                Err(_) => Err(PyErr::new::<exceptions::ValueError, _>({
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["The given array cannot be casted to ", "."],
                        &match (&"NodeT",) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ));
                    res
                })),
            }?
            .to_owned(),
            match PyArray::from_vec(gil.python(), frequencies).cast::<f64>(false) {
                Ok(v) => Ok(v),
                Err(_) => Err(PyErr::new::<exceptions::ValueError, _>({
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["The given array cannot be casted to ", "."],
                        &match (&"f64",) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ));
                    res
                })),
            }?
            .to_owned(),
        ))
    }
    unsafe extern "C" fn __pyo3_raw_cooccurence_matrix(
        _slf: *mut pyo3::ffi::PyObject,
        _args: *mut pyo3::ffi::PyObject,
        _kwargs: *mut pyo3::ffi::PyObject,
    ) -> *mut pyo3::ffi::PyObject {
        const _LOCATION: &'static str = "cooccurence_matrix()";
        {
            {
                let pool = ::pyo3::GILPool::new();
                let unwind_safe_py = std::panic::AssertUnwindSafe(pool.python());
                let result = match std::panic::catch_unwind(move || -> ::pyo3::PyResult<_> {
                    let _py = *unwind_safe_py;
                    ::pyo3::callback::convert(_py, {
                        let _args = _py.from_borrowed_ptr::<pyo3::types::PyTuple>(_args);
                        let _kwargs: Option<&pyo3::types::PyDict> =
                            _py.from_borrowed_ptr_or_opt(_kwargs);
                        {
                            const PARAMS: &'static [pyo3::derive_utils::ParamDescription] =
                                &[pyo3::derive_utils::ParamDescription {
                                    name: "sequences",
                                    is_optional: false,
                                    kw_only: false,
                                }];
                            let mut output = [None; 1usize];
                            let mut _args = _args;
                            let mut _kwargs = _kwargs;
                            let (_args, _kwargs) = pyo3::derive_utils::parse_fn_args(
                                Some(_LOCATION),
                                PARAMS,
                                _args,
                                _kwargs,
                                false,
                                true,
                                &mut output,
                            )?;
                            let arg0 = match output[0usize] {
                                Some(_obj) => _obj.extract()?,
                                None => ::std::rt::begin_panic(
                                    "Failed to extract required method argument",
                                ),
                            };
                            let arg1 = _kwargs;
                            cooccurence_matrix(arg0, arg1)
                        }
                    })
                }) {
                    Ok(result) => result,
                    Err(e) => {
                        if let Some(string) = e.downcast_ref::<String>() {
                            Err(::pyo3::panic::PanicException::new_err((string.clone(),)))
                        } else if let Some(s) = e.downcast_ref::<&str>() {
                            Err(::pyo3::panic::PanicException::new_err((s.to_string(),)))
                        } else {
                            Err(::pyo3::panic::PanicException::new_err((
                                "panic from Rust code",
                            )))
                        }
                    }
                };
                result.unwrap_or_else(|e| {
                    e.restore(pool.python());
                    ::pyo3::callback::callback_error()
                })
            }
        }
    }
    fn __pyo3_get_function_cooccurence_matrix<'a>(
        args: impl Into<pyo3::derive_utils::PyFunctionArguments<'a>>,
    ) -> pyo3::PyResult<&'a pyo3::types::PyCFunction> {
        let name = "cooccurence_matrix\u{0}";
        let name = std::ffi::CStr::from_bytes_with_nul(name.as_bytes()).unwrap();
        let doc = std :: ffi :: CStr :: from_bytes_with_nul (b"cooccurence_matrix(sequences, *, window_size, verbose)\n--\n\nReturn triple with CSR representation of cooccurrence matrix.\n\nThe first vector has the sources, the second vector the destinations\nand the third one contains the min-max normalized frequencies.\n\nArguments\n---------\n\nsequences: List[List[int]],\n    the sequence of sequences of integers to preprocess.\nwindow_size: int = 4,\n    Window size to consider for the sequences.\nverbose: bool = False,\n    Wethever to show the progress bars.\n    The default behaviour is false.\n    \x00") . unwrap () ;
        pyo3::types::PyCFunction::internal_new(
            name,
            doc,
            pyo3::class::PyMethodType::PyCFunctionWithKeywords(__pyo3_raw_cooccurence_matrix),
            pyo3::ffi::METH_VARARGS | pyo3::ffi::METH_KEYWORDS,
            args.into(),
        )
    }
    impl EnsmallenGraph {
        /// Return cooccurence matrix-based triples of words, contexts and frequencies.
        ///
        /// Parameters
        /// ---------------------
        /// length: int,
        ///     Maximal length of the random walk.
        ///     On graphs without traps, all walks have this length.
        /// window_size: int = 4,
        ///     Size of the window for local contexts.
        /// iterations: int = 1,
        ///     Number of cycles on the graphs to execute.
        /// min_length: int = 0,
        ///     Minimal length of the random walk. Will filter out smaller
        ///     random walks.
        /// return_weight: float = 1.0,
        ///     Weight on the probability of returning to node coming from
        ///     Having this higher tends the walks to be
        ///     more like a Breadth-First Search.
        ///     Having this very high  (> 2) makes search very local.
        ///     Equal to the inverse of p in the Node2Vec paper.
        /// explore_weight: float = 1.0,
        ///     Weight on the probability of visiting a neighbor node
        ///     to the one we're coming from in the random walk
        ///     Having this higher tends the walks to be
        ///     more like a Depth-First Search.
        ///     Having this very high makes search more outward.
        ///     Having this very low makes search very local.
        ///     Equal to the inverse of q in the Node2Vec paper.
        /// change_node_type_weight: float = 1.0,
        ///     Weight on the probability of visiting a neighbor node of a
        ///     different type than the previous node. This only applies to
        ///     colored graphs, otherwise it has no impact.
        /// change_edge_type_weight: float = 1.0,
        ///     Weight on the probability of visiting a neighbor edge of a
        ///     different type than the previous edge. This only applies to
        ///     multigraphs, otherwise it has no impact.
        /// dense_node_mapping: Dict[int, int],
        ///     Mapping to use for converting sparse walk space into a dense space.
        ///     This object can be created using the method available from graph
        ///     called `get_dense_node_mapping` that returns a mapping from
        ///     the non trap nodes (those from where a walk could start) and
        ///     maps these nodes into a dense range of values.
        /// random_state: int,
        ///     random_state to use to reproduce the walks.
        /// verbose: int = True,
        ///     Wethever to show or not the loading bar of the walks.
        ///
        /// Returns
        /// ----------------------------
        /// Triple with integer vectors of words and contexts and max-min normalized frequencies.
        ///
        fn cooccurence_matrix(
            &self,
            length: NodeT,
            py_kwargs: Option<&PyDict>,
        ) -> PyResult<(PyWords, PyWords, PyFrequencies)> {
            let gil = pyo3::Python::acquire_gil();
            let kwargs = match py_kwargs {
                Some(v) => v,
                None => PyDict::new(gil.python()),
            };
            match validate_kwargs(
                kwargs,
                build_walk_parameters_list(&["window_size", "verbose"]),
            ) {
                Ok(v) => Ok(v),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }?;
            let parameters = match self.build_walk_parameters(length, kwargs) {
                Ok(v) => Ok(v),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }?;
            let (words, contexts, frequencies) = match self.graph.cooccurence_matrix(
                &parameters,
                match match kwargs.get_item("window_size") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<usize>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (&"window_size", &value.get_type().name(), &"usize")
                                        {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                } {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?
                .unwrap_or(3),
                match match kwargs.get_item("verbose") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<bool>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (&"verbose", &value.get_type().name(), &"bool") {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                } {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?
                .unwrap_or(true),
            ) {
                Ok(v) => Ok(v),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }?;
            Ok((
                match PyArray::from_vec(gil.python(), words).cast::<NodeT>(false) {
                    Ok(v) => Ok(v),
                    Err(_) => Err(PyErr::new::<exceptions::ValueError, _>({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["The given array cannot be casted to ", "."],
                            &match (&"NodeT",) {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ));
                        res
                    })),
                }?
                .to_owned(),
                match PyArray::from_vec(gil.python(), contexts).cast::<NodeT>(false) {
                    Ok(v) => Ok(v),
                    Err(_) => Err(PyErr::new::<exceptions::ValueError, _>({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["The given array cannot be casted to ", "."],
                            &match (&"NodeT",) {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ));
                        res
                    })),
                }?
                .to_owned(),
                match PyArray::from_vec(gil.python(), frequencies).cast::<f64>(false) {
                    Ok(v) => Ok(v),
                    Err(_) => Err(PyErr::new::<exceptions::ValueError, _>({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["The given array cannot be casted to ", "."],
                            &match (&"f64",) {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ));
                        res
                    })),
                }?
                .to_owned(),
            ))
        }
        /// Return training batches for Node2Vec models.
        ///
        /// The batch is composed of a tuple as the following:
        ///
        /// - (Contexts indices, central nodes indices): the tuple of nodes
        ///
        /// This does not provide any output value as the model uses NCE loss
        /// and basically the central nodes that are fed as inputs work as the
        /// outputs value.
        ///
        /// Parameters
        /// ---------------------
        /// batch_size:
        ///     Number of walks to include within this batch.
        ///     Consider that the walks may be filtered by the given min_length.
        ///     In some pathological cases, this might leed to an empty batch.
        ///     These cases include graphs with particularly high number of traps.
        ///     Consider using the method graph.report() to verify if this might
        ///     apply to your use case.
        /// length: int,
        ///     Maximal length of the random walk.
        ///     On graphs without traps, all walks have this length.
        /// window_size: int,
        ///     Size of the window for local contexts.
        /// iterations: int = 1,
        ///     Number of iterations for each node.
        /// min_length: int = 0,
        ///     Minimal length of the random walk. Will filter out smaller
        ///     random walks.
        /// return_weight: float = 1.0,
        ///     Weight on the probability of returning to node coming from
        ///     Having this higher tends the walks to be
        ///     more like a Breadth-First Search.
        ///     Having this very high  (> 2) makes search very local.
        ///     Equal to the inverse of p in the Node2Vec paper.
        /// explore_weight: float = 1.0,
        ///     Weight on the probability of visiting a neighbor node
        ///     to the one we're coming from in the random walk
        ///     Having this higher tends the walks to be
        ///     more like a Depth-First Search.
        ///     Having this very high makes search more outward.
        ///     Having this very low makes search very local.
        ///     Equal to the inverse of q in the Node2Vec paper.
        /// change_node_type_weight: float = 1.0,
        ///     Weight on the probability of visiting a neighbor node of a
        ///     different type than the previous node. This only applies to
        ///     colored graphs, otherwise it has no impact.
        /// change_edge_type_weight: float = 1.0,
        ///     Weight on the probability of visiting a neighbor edge of a
        ///     different type than the previous edge. This only applies to
        ///     multigraphs, otherwise it has no impact.
        /// dense_node_mapping: Dict[int, int],
        ///     Mapping to use for converting sparse walk space into a dense space.
        ///     This object can be created using the method available from graph
        ///     called `get_dense_node_mapping` that returns a mapping from
        ///     the non trap nodes (those from where a walk could start) and
        ///     maps these nodes into a dense range of values.
        /// random_state: int,
        ///     random_state to use to reproduce the walks.
        ///
        /// Returns
        /// ----------------------------
        /// Tuple with vector of integer with contexts and words.
        fn node2vec(
            &self,
            batch_size: NodeT,
            length: NodeT,
            window_size: usize,
            py_kwargs: Option<&PyDict>,
        ) -> PyResult<(PyContexts, PyWords)> {
            let gil = pyo3::Python::acquire_gil();
            let kwargs = match py_kwargs {
                Some(v) => v,
                None => PyDict::new(gil.python()),
            };
            let parameters = match self.build_walk_parameters(length, kwargs) {
                Ok(v) => Ok(v),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }?;
            let (contexts, words) = match self.graph.node2vec(&parameters, batch_size, window_size)
            {
                Ok(v) => Ok(v),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }?;
            Ok((
                match match PyArray::from_vec2(gil.python(), &contexts) {
                    Ok(v) => Ok(v),
                    Err(_) => Err(PyErr::new::<exceptions::ValueError, _>(
                        "The given value cannot be casted to a 2d numpy array.",
                    )),
                }?
                .cast::<NodeT>(false)
                {
                    Ok(v) => Ok(v),
                    Err(_) => Err(PyErr::new::<exceptions::ValueError, _>({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["The given 2d array cannot be casted to ", "."],
                            &match (&"NodeT",) {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ));
                        res
                    })),
                }?
                .to_owned(),
                match PyArray::from_vec(gil.python(), words).cast::<NodeT>(false) {
                    Ok(v) => Ok(v),
                    Err(_) => Err(PyErr::new::<exceptions::ValueError, _>({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["The given array cannot be casted to ", "."],
                            &match (&"NodeT",) {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ));
                        res
                    })),
                }?
                .to_owned(),
            ))
        }
        /// Returns
        ///
        ///
        /// Parameters
        /// -----------------------------
        /// idx:int,
        ///     Index corresponding to batch to be rendered.
        /// batch_size: int = 2**10,
        ///     The batch size to use.
        /// negative_samples: float = 1.0,
        ///     Factor of negatives to use in every batch.
        ///     For example, with a batch size of 128 and negative_samples equal
        ///     to 1.0, there will be 64 positives and 64 negatives.
        /// graph_to_avoid: EnsmallenGraph = None,
        ///     Graph to avoid when generating the links.
        ///     This can be the validation component of the graph, for example.
        ///
        /// Returns
        /// -----------------------------
        /// Tuple containing training and validation graphs.
        ///
        fn link_prediction(
            &self,
            idx: u64,
            batch_size: usize,
            py_kwargs: Option<&PyDict>,
        ) -> PyResult<(PyContexts, Py<PyArray1<u8>>)> {
            let gil = pyo3::Python::acquire_gil();
            let kwargs = match py_kwargs {
                Some(v) => v,
                None => PyDict::new(gil.python()),
            };
            match validate_kwargs(
                kwargs,
                ["graph_to_avoid", "negative_samples"]
                    .iter()
                    .map(|x| x.to_string())
                    .collect(),
            ) {
                Ok(v) => Ok(v),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }?;
            let graph_to_avoid = match match kwargs.get_item("graph_to_avoid") {
                None => Ok(None),
                Some(value) => {
                    if value.get_type().name() == "NoneType" {
                        Ok(None)
                    } else {
                        match value.extract::<EnsmallenGraph>() {
                            Ok(v) => Ok(Some(v)),
                            Err(_) => Err({
                                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                    &[
                                        "The value passed for ",
                                        " cannot be casted from ",
                                        " to ",
                                        ".",
                                    ],
                                    &match (
                                        &"graph_to_avoid",
                                        &value.get_type().name(),
                                        &"EnsmallenGraph",
                                    ) {
                                        (arg0, arg1, arg2) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg2,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ));
                                res
                            }),
                        }
                    }
                }
            } {
                Ok(v) => Ok(v),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }?;
            let (edges, labels) = match self.graph.link_prediction(
                idx,
                batch_size,
                match match kwargs.get_item("negative_samples") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<f64>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (
                                            &"negative_samples",
                                            &value.get_type().name(),
                                            &"f64",
                                        ) {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                } {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?
                .unwrap_or(1.0),
                match &graph_to_avoid {
                    Some(g) => Some(&g.graph),
                    None => None,
                },
            ) {
                Ok(v) => Ok(v),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }?;
            Ok((
                match match PyArray::from_vec2(gil.python(), &edges) {
                    Ok(v) => Ok(v),
                    Err(_) => Err(PyErr::new::<exceptions::ValueError, _>(
                        "The given value cannot be casted to a 2d numpy array.",
                    )),
                }?
                .cast::<NodeT>(false)
                {
                    Ok(v) => Ok(v),
                    Err(_) => Err(PyErr::new::<exceptions::ValueError, _>({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["The given 2d array cannot be casted to ", "."],
                            &match (&"NodeT",) {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ));
                        res
                    })),
                }?
                .to_owned(),
                match PyArray::from_vec(gil.python(), labels).cast::<u8>(false) {
                    Ok(v) => Ok(v),
                    Err(_) => Err(PyErr::new::<exceptions::ValueError, _>({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["The given array cannot be casted to ", "."],
                            &match (&"u8",) {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ));
                        res
                    })),
                }?
                .to_owned(),
            ))
        }
    }
    #[used]
    #[allow(non_upper_case_globals)]
    #[link_section = ".init_array"]
    #[allow(non_upper_case_globals)]
    static __init6885849936538026204: extern "C" fn() = {
        #[link_section = ".text.startup"]
        extern "C" fn __init6885849936538026204() {
            pyo3::inventory::submit({
                {
                    type Inventory =
                        <EnsmallenGraph as pyo3::class::methods::HasMethodsInventory>::Methods;
                    <Inventory as pyo3::class::methods::PyMethodsInventory>::new(<[_]>::into_vec(
                        box [
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                    _kwargs: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.cooccurence_matrix()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    let _args = _py
                                                        .from_borrowed_ptr::<pyo3::types::PyTuple>(
                                                            _args,
                                                        );
                                                    let _kwargs: Option<&pyo3::types::PyDict> =
                                                        _py.from_borrowed_ptr_or_opt(_kwargs);
                                                    pyo3::callback::convert(_py, {
                                                        const PARAMS : & 'static [pyo3 :: derive_utils :: ParamDescription] = & [pyo3 :: derive_utils :: ParamDescription { name : "length" , is_optional : false , kw_only : false , }] ;
                                                        let mut output = [None; 1usize];
                                                        let mut _args = _args;
                                                        let mut _kwargs = _kwargs;
                                                        let (_args, _kwargs) =
                                                            pyo3::derive_utils::parse_fn_args(
                                                                Some(_LOCATION),
                                                                PARAMS,
                                                                _args,
                                                                _kwargs,
                                                                false,
                                                                true,
                                                                &mut output,
                                                            )?;
                                                        let arg0 = match output [0usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        let arg1 = _kwargs;
                                                        EnsmallenGraph::cooccurence_matrix(
                                                            _slf, arg0, arg1,
                                                        )
                                                    })
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction_with_keywords ("cooccurence_matrix\u{0}" , __wrap , 0 , "cooccurence_matrix($self, length, *, window_size, iterations, min_length, return_weight, explore_weight, change_edge_type_weight, change_node_type_weight, dense_node_mapping, random_state, verbose)\n--\n\nReturn cooccurence matrix-based triples of words, contexts and frequencies.\n\nParameters\n---------------------\nlength: int,\n    Maximal length of the random walk.\n    On graphs without traps, all walks have this length.\nwindow_size: int = 4,\n    Size of the window for local contexts.\niterations: int = 1,\n    Number of cycles on the graphs to execute.\nmin_length: int = 0,\n    Minimal length of the random walk. Will filter out smaller\n    random walks.\nreturn_weight: float = 1.0,\n    Weight on the probability of returning to node coming from\n    Having this higher tends the walks to be\n    more like a Breadth-First Search.\n    Having this very high  (> 2) makes search very local.\n    Equal to the inverse of p in the Node2Vec paper.\nexplore_weight: float = 1.0,\n    Weight on the probability of visiting a neighbor node\n    to the one we\'re coming from in the random walk\n    Having this higher tends the walks to be\n    more like a Depth-First Search.\n    Having this very high makes search more outward.\n    Having this very low makes search very local.\n    Equal to the inverse of q in the Node2Vec paper.\nchange_node_type_weight: float = 1.0,\n    Weight on the probability of visiting a neighbor node of a\n    different type than the previous node. This only applies to\n    colored graphs, otherwise it has no impact.\nchange_edge_type_weight: float = 1.0,\n    Weight on the probability of visiting a neighbor edge of a\n    different type than the previous edge. This only applies to\n    multigraphs, otherwise it has no impact.\ndense_node_mapping: Dict[int, int],\n    Mapping to use for converting sparse walk space into a dense space.\n    This object can be created using the method available from graph\n    called `get_dense_node_mapping` that returns a mapping from\n    the non trap nodes (those from where a walk could start) and\n    maps these nodes into a dense range of values.\nrandom_state: int,\n    random_state to use to reproduce the walks.\nverbose: int = True,\n    Wethever to show or not the loading bar of the walks.\n\nReturns\n----------------------------\nTriple with integer vectors of words and contexts and max-min normalized frequencies.\n\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                    _kwargs: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str = "EnsmallenGraph.node2vec()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    let _args = _py
                                                        .from_borrowed_ptr::<pyo3::types::PyTuple>(
                                                            _args,
                                                        );
                                                    let _kwargs: Option<&pyo3::types::PyDict> =
                                                        _py.from_borrowed_ptr_or_opt(_kwargs);
                                                    pyo3::callback::convert(_py, {
                                                        const PARAMS : & 'static [pyo3 :: derive_utils :: ParamDescription] = & [pyo3 :: derive_utils :: ParamDescription { name : "batch_size" , is_optional : false , kw_only : false , } , pyo3 :: derive_utils :: ParamDescription { name : "length" , is_optional : false , kw_only : false , } , pyo3 :: derive_utils :: ParamDescription { name : "window_size" , is_optional : false , kw_only : false , }] ;
                                                        let mut output = [None; 3usize];
                                                        let mut _args = _args;
                                                        let mut _kwargs = _kwargs;
                                                        let (_args, _kwargs) =
                                                            pyo3::derive_utils::parse_fn_args(
                                                                Some(_LOCATION),
                                                                PARAMS,
                                                                _args,
                                                                _kwargs,
                                                                false,
                                                                true,
                                                                &mut output,
                                                            )?;
                                                        let arg0 = match output [0usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        let arg1 = match output [1usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        let arg2 = match output [2usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        let arg3 = _kwargs;
                                                        EnsmallenGraph::node2vec(
                                                            _slf, arg0, arg1, arg2, arg3,
                                                        )
                                                    })
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction_with_keywords ("node2vec\u{0}" , __wrap , 0 , "node2vec($self, batch_size, length, window_size, *, iterations, min_length, return_weight, explore_weight, change_edge_type_weight, change_node_type_weight, dense_node_mapping, random_state)\n--\n\nReturn training batches for Node2Vec models.\n\nThe batch is composed of a tuple as the following:\n\n- (Contexts indices, central nodes indices): the tuple of nodes\n\nThis does not provide any output value as the model uses NCE loss\nand basically the central nodes that are fed as inputs work as the\noutputs value.\n\nParameters\n---------------------\nbatch_size:\n    Number of walks to include within this batch.\n    Consider that the walks may be filtered by the given min_length.\n    In some pathological cases, this might leed to an empty batch.\n    These cases include graphs with particularly high number of traps.\n    Consider using the method graph.report() to verify if this might\n    apply to your use case.\nlength: int,\n    Maximal length of the random walk.\n    On graphs without traps, all walks have this length.\nwindow_size: int,\n    Size of the window for local contexts.\niterations: int = 1,\n    Number of iterations for each node.\nmin_length: int = 0,\n    Minimal length of the random walk. Will filter out smaller\n    random walks.\nreturn_weight: float = 1.0,\n    Weight on the probability of returning to node coming from\n    Having this higher tends the walks to be\n    more like a Breadth-First Search.\n    Having this very high  (> 2) makes search very local.\n    Equal to the inverse of p in the Node2Vec paper.\nexplore_weight: float = 1.0,\n    Weight on the probability of visiting a neighbor node\n    to the one we\'re coming from in the random walk\n    Having this higher tends the walks to be\n    more like a Depth-First Search.\n    Having this very high makes search more outward.\n    Having this very low makes search very local.\n    Equal to the inverse of q in the Node2Vec paper.\nchange_node_type_weight: float = 1.0,\n    Weight on the probability of visiting a neighbor node of a\n    different type than the previous node. This only applies to\n    colored graphs, otherwise it has no impact.\nchange_edge_type_weight: float = 1.0,\n    Weight on the probability of visiting a neighbor edge of a\n    different type than the previous edge. This only applies to\n    multigraphs, otherwise it has no impact.\ndense_node_mapping: Dict[int, int],\n    Mapping to use for converting sparse walk space into a dense space.\n    This object can be created using the method available from graph\n    called `get_dense_node_mapping` that returns a mapping from\n    the non trap nodes (those from where a walk could start) and\n    maps these nodes into a dense range of values.\nrandom_state: int,\n    random_state to use to reproduce the walks.\n\nReturns\n----------------------------\nTuple with vector of integer with contexts and words.\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                    _kwargs: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.link_prediction()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    let _args = _py
                                                        .from_borrowed_ptr::<pyo3::types::PyTuple>(
                                                            _args,
                                                        );
                                                    let _kwargs: Option<&pyo3::types::PyDict> =
                                                        _py.from_borrowed_ptr_or_opt(_kwargs);
                                                    pyo3::callback::convert(_py, {
                                                        const PARAMS : & 'static [pyo3 :: derive_utils :: ParamDescription] = & [pyo3 :: derive_utils :: ParamDescription { name : "idx" , is_optional : false , kw_only : false , } , pyo3 :: derive_utils :: ParamDescription { name : "batch_size" , is_optional : false , kw_only : false , }] ;
                                                        let mut output = [None; 2usize];
                                                        let mut _args = _args;
                                                        let mut _kwargs = _kwargs;
                                                        let (_args, _kwargs) =
                                                            pyo3::derive_utils::parse_fn_args(
                                                                Some(_LOCATION),
                                                                PARAMS,
                                                                _args,
                                                                _kwargs,
                                                                false,
                                                                true,
                                                                &mut output,
                                                            )?;
                                                        let arg0 = match output [0usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        let arg1 = match output [1usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        let arg2 = _kwargs;
                                                        EnsmallenGraph::link_prediction(
                                                            _slf, arg0, arg1, arg2,
                                                        )
                                                    })
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction_with_keywords ("link_prediction\u{0}" , __wrap , 0 , "link_prediction($self, idx, batch_size, negative_samples, graph_to_avoid, avoid_self_loops)\n--\n\nReturns\n\n\nParameters\n-----------------------------\nidx:int,\n    Index corresponding to batch to be rendered.\nbatch_size: int = 2**10,\n    The batch size to use.\nnegative_samples: float = 1.0,\n    Factor of negatives to use in every batch.\n    For example, with a batch size of 128 and negative_samples equal\n    to 1.0, there will be 64 positives and 64 negatives.\ngraph_to_avoid: EnsmallenGraph = None,\n    Graph to avoid when generating the links.\n    This can be the validation component of the graph, for example.\n\nReturns\n-----------------------------\nTuple containing training and validation graphs.\n\u{0}")
                            }),
                        ],
                    ))
                }
            });
        }
        __init6885849936538026204
    };
}
mod tarjan {
    use super::*;
    use graph::{NodeT};
    use std::collections::HashSet;
    impl EnsmallenGraph {
        /// Returns list of sets of nodes of connected components.
        ///
        /// Raises
        /// ------------------------
        /// TODO: update the set of exceptions
        ///
        /// Returns
        /// ------------------------
        /// List of sets of connected components.
        ///
        fn strongly_connected_components(&self) -> Vec<HashSet<NodeT>> {
            self.graph.strongly_connected_components()
        }
    }
    #[used]
    #[allow(non_upper_case_globals)]
    #[link_section = ".init_array"]
    #[allow(non_upper_case_globals)]
    static __init7696087703123793752: extern "C" fn() = {
        #[link_section = ".text.startup"]
        extern "C" fn __init7696087703123793752() {
            pyo3::inventory::submit({
                {
                    type Inventory =
                        <EnsmallenGraph as pyo3::class::methods::HasMethodsInventory>::Methods;
                    <Inventory as pyo3::class::methods::PyMethodsInventory>::new(<[_]>::into_vec(
                        box [pyo3::class::PyMethodDefType::Method({
                            unsafe extern "C" fn __wrap(
                                _slf: *mut pyo3::ffi::PyObject,
                                _args: *mut pyo3::ffi::PyObject,
                            ) -> *mut pyo3::ffi::PyObject {
                                const _LOCATION: &'static str =
                                    "EnsmallenGraph.strongly_connected_components()";
                                {
                                    let pool = ::pyo3::GILPool::new();
                                    let unwind_safe_py =
                                        std::panic::AssertUnwindSafe(pool.python());
                                    let result = match std::panic::catch_unwind(
                                        move || -> ::pyo3::PyResult<_> {
                                            let _py = *unwind_safe_py;
                                            {
                                                let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                let _ref = _cell.try_borrow()?;
                                                let _slf = &_ref;
                                                pyo3::callback::convert(
                                                    _py,
                                                    EnsmallenGraph::strongly_connected_components(
                                                        _slf,
                                                    ),
                                                )
                                            }
                                        },
                                    ) {
                                        Ok(result) => result,
                                        Err(e) => {
                                            if let Some(string) = e.downcast_ref::<String>() {
                                                Err(::pyo3::panic::PanicException::new_err((
                                                    string.clone(),
                                                )))
                                            } else if let Some(s) = e.downcast_ref::<&str>() {
                                                Err(::pyo3::panic::PanicException::new_err((
                                                    s.to_string(),
                                                )))
                                            } else {
                                                Err(::pyo3::panic::PanicException::new_err((
                                                    "panic from Rust code",
                                                )))
                                            }
                                        }
                                    };
                                    result.unwrap_or_else(|e| {
                                        e.restore(pool.python());
                                        ::pyo3::callback::callback_error()
                                    })
                                }
                            }
                            pyo3 :: class :: PyMethodDef :: cfunction ("strongly_connected_components\u{0}" , __wrap , "strongly_connected_components($self)\n--\n\nReturns list of sets of nodes of connected components.\n\nRaises\n------------------------\nTODO: update the set of exceptions\n\nReturns\n------------------------\nList of sets of connected components.\n\u{0}")
                        })],
                    ))
                }
            });
        }
        __init7696087703123793752
    };
}
mod utilities {
    use super::*;
    use graph::{EdgeFileReader, NodeFileReader, NodeT, WalksParameters, WeightT};
    use std::collections::HashMap;
    pub(crate) fn build_csv_file_reader(
        edge_path: String,
        py_kwargs: Option<&PyDict>,
    ) -> Result<(EdgeFileReader, Option<NodeFileReader>, String), String> {
        let py = pyo3::Python::acquire_gil();
        let kwargs = match py_kwargs {
            Some(v) => v,
            None => PyDict::new(py.python()),
        };
        validate_kwargs(
            kwargs,
            [
                "sources_column_number",
                "sources_column",
                "destinations_column_number",
                "destinations_column",
                "edge_types_column_number",
                "edge_types_column",
                "default_edge_type",
                "weights_column_number",
                "weights_column",
                "default_weight",
                "skip_self_loops",
                "ignore_duplicated_edges",
                "edge_header",
                "edge_rows_to_skip",
                "edge_separator",
                "edge_max_rows_number",
                "node_path",
                "nodes_column_number",
                "nodes_column",
                "node_types_column_number",
                "node_types_column",
                "default_node_type",
                "ignore_duplicated_nodes",
                "node_header",
                "node_rows_to_skip",
                "node_separator",
                "node_max_rows_number",
                "verbose",
                "numeric_node_ids",
                "numeric_node_type_ids",
                "numeric_edge_type_ids",
                "name",
            ]
            .iter()
            .map(|x| x.to_string())
            .collect(),
        )?;
        let edges: EdgeFileReader = EdgeFileReader::new(edge_path)?
            .set_separator(match kwargs.get_item("edge_separator") {
                None => Ok(None),
                Some(value) => {
                    if value.get_type().name() == "NoneType" {
                        Ok(None)
                    } else {
                        match value.extract::<String>() {
                            Ok(v) => Ok(Some(v)),
                            Err(_) => Err({
                                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                    &[
                                        "The value passed for ",
                                        " cannot be casted from ",
                                        " to ",
                                        ".",
                                    ],
                                    &match (&"edge_separator", &value.get_type().name(), &"String")
                                    {
                                        (arg0, arg1, arg2) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg2,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ));
                                res
                            }),
                        }
                    }
                }
            }?)?
            .set_header(match kwargs.get_item("edge_header") {
                None => Ok(None),
                Some(value) => {
                    if value.get_type().name() == "NoneType" {
                        Ok(None)
                    } else {
                        match value.extract::<bool>() {
                            Ok(v) => Ok(Some(v)),
                            Err(_) => Err({
                                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                    &[
                                        "The value passed for ",
                                        " cannot be casted from ",
                                        " to ",
                                        ".",
                                    ],
                                    &match (&"edge_header", &value.get_type().name(), &"bool") {
                                        (arg0, arg1, arg2) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg2,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ));
                                res
                            }),
                        }
                    }
                }
            }?)
            .set_rows_to_skip(match kwargs.get_item("edge_rows_to_skip") {
                None => Ok(None),
                Some(value) => {
                    if value.get_type().name() == "NoneType" {
                        Ok(None)
                    } else {
                        match value.extract::<usize>() {
                            Ok(v) => Ok(Some(v)),
                            Err(_) => Err({
                                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                    &[
                                        "The value passed for ",
                                        " cannot be casted from ",
                                        " to ",
                                        ".",
                                    ],
                                    &match (
                                        &"edge_rows_to_skip",
                                        &value.get_type().name(),
                                        &"usize",
                                    ) {
                                        (arg0, arg1, arg2) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg2,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ));
                                res
                            }),
                        }
                    }
                }
            }?)
            .set_sources_column_number(match kwargs.get_item("sources_column_number") {
                None => Ok(None),
                Some(value) => {
                    if value.get_type().name() == "NoneType" {
                        Ok(None)
                    } else {
                        match value.extract::<usize>() {
                            Ok(v) => Ok(Some(v)),
                            Err(_) => Err({
                                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                    &[
                                        "The value passed for ",
                                        " cannot be casted from ",
                                        " to ",
                                        ".",
                                    ],
                                    &match (
                                        &"sources_column_number",
                                        &value.get_type().name(),
                                        &"usize",
                                    ) {
                                        (arg0, arg1, arg2) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg2,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ));
                                res
                            }),
                        }
                    }
                }
            }?)?
            .set_sources_column(match kwargs.get_item("sources_column") {
                None => Ok(None),
                Some(value) => {
                    if value.get_type().name() == "NoneType" {
                        Ok(None)
                    } else {
                        match value.extract::<String>() {
                            Ok(v) => Ok(Some(v)),
                            Err(_) => Err({
                                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                    &[
                                        "The value passed for ",
                                        " cannot be casted from ",
                                        " to ",
                                        ".",
                                    ],
                                    &match (&"sources_column", &value.get_type().name(), &"String")
                                    {
                                        (arg0, arg1, arg2) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg2,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ));
                                res
                            }),
                        }
                    }
                }
            }?)?
            .set_destinations_column_number(match kwargs.get_item("destinations_column_number") {
                None => Ok(None),
                Some(value) => {
                    if value.get_type().name() == "NoneType" {
                        Ok(None)
                    } else {
                        match value.extract::<usize>() {
                            Ok(v) => Ok(Some(v)),
                            Err(_) => Err({
                                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                    &[
                                        "The value passed for ",
                                        " cannot be casted from ",
                                        " to ",
                                        ".",
                                    ],
                                    &match (
                                        &"destinations_column_number",
                                        &value.get_type().name(),
                                        &"usize",
                                    ) {
                                        (arg0, arg1, arg2) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg2,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ));
                                res
                            }),
                        }
                    }
                }
            }?)?
            .set_destinations_column(match kwargs.get_item("destinations_column") {
                None => Ok(None),
                Some(value) => {
                    if value.get_type().name() == "NoneType" {
                        Ok(None)
                    } else {
                        match value.extract::<String>() {
                            Ok(v) => Ok(Some(v)),
                            Err(_) => Err({
                                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                    &[
                                        "The value passed for ",
                                        " cannot be casted from ",
                                        " to ",
                                        ".",
                                    ],
                                    &match (
                                        &"destinations_column",
                                        &value.get_type().name(),
                                        &"String",
                                    ) {
                                        (arg0, arg1, arg2) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg2,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ));
                                res
                            }),
                        }
                    }
                }
            }?)?
            .set_edge_types_column_number(match kwargs.get_item("edge_types_column_number") {
                None => Ok(None),
                Some(value) => {
                    if value.get_type().name() == "NoneType" {
                        Ok(None)
                    } else {
                        match value.extract::<usize>() {
                            Ok(v) => Ok(Some(v)),
                            Err(_) => Err({
                                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                    &[
                                        "The value passed for ",
                                        " cannot be casted from ",
                                        " to ",
                                        ".",
                                    ],
                                    &match (
                                        &"edge_types_column_number",
                                        &value.get_type().name(),
                                        &"usize",
                                    ) {
                                        (arg0, arg1, arg2) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg2,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ));
                                res
                            }),
                        }
                    }
                }
            }?)?
            .set_edge_types_column(match kwargs.get_item("edge_types_column") {
                None => Ok(None),
                Some(value) => {
                    if value.get_type().name() == "NoneType" {
                        Ok(None)
                    } else {
                        match value.extract::<String>() {
                            Ok(v) => Ok(Some(v)),
                            Err(_) => Err({
                                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                    &[
                                        "The value passed for ",
                                        " cannot be casted from ",
                                        " to ",
                                        ".",
                                    ],
                                    &match (
                                        &"edge_types_column",
                                        &value.get_type().name(),
                                        &"String",
                                    ) {
                                        (arg0, arg1, arg2) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg2,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ));
                                res
                            }),
                        }
                    }
                }
            }?)?
            .set_default_edge_type(match kwargs.get_item("default_edge_type") {
                None => Ok(None),
                Some(value) => {
                    if value.get_type().name() == "NoneType" {
                        Ok(None)
                    } else {
                        match value.extract::<String>() {
                            Ok(v) => Ok(Some(v)),
                            Err(_) => Err({
                                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                    &[
                                        "The value passed for ",
                                        " cannot be casted from ",
                                        " to ",
                                        ".",
                                    ],
                                    &match (
                                        &"default_edge_type",
                                        &value.get_type().name(),
                                        &"String",
                                    ) {
                                        (arg0, arg1, arg2) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg2,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ));
                                res
                            }),
                        }
                    }
                }
            }?)
            .set_weights_column_number(match kwargs.get_item("weights_column_number") {
                None => Ok(None),
                Some(value) => {
                    if value.get_type().name() == "NoneType" {
                        Ok(None)
                    } else {
                        match value.extract::<usize>() {
                            Ok(v) => Ok(Some(v)),
                            Err(_) => Err({
                                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                    &[
                                        "The value passed for ",
                                        " cannot be casted from ",
                                        " to ",
                                        ".",
                                    ],
                                    &match (
                                        &"weights_column_number",
                                        &value.get_type().name(),
                                        &"usize",
                                    ) {
                                        (arg0, arg1, arg2) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg2,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ));
                                res
                            }),
                        }
                    }
                }
            }?)?
            .set_weights_column(match kwargs.get_item("weights_column") {
                None => Ok(None),
                Some(value) => {
                    if value.get_type().name() == "NoneType" {
                        Ok(None)
                    } else {
                        match value.extract::<String>() {
                            Ok(v) => Ok(Some(v)),
                            Err(_) => Err({
                                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                    &[
                                        "The value passed for ",
                                        " cannot be casted from ",
                                        " to ",
                                        ".",
                                    ],
                                    &match (&"weights_column", &value.get_type().name(), &"String")
                                    {
                                        (arg0, arg1, arg2) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg2,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ));
                                res
                            }),
                        }
                    }
                }
            }?)?
            .set_default_weight(match kwargs.get_item("default_weight") {
                None => Ok(None),
                Some(value) => {
                    if value.get_type().name() == "NoneType" {
                        Ok(None)
                    } else {
                        match value.extract::<WeightT>() {
                            Ok(v) => Ok(Some(v)),
                            Err(_) => Err({
                                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                    &[
                                        "The value passed for ",
                                        " cannot be casted from ",
                                        " to ",
                                        ".",
                                    ],
                                    &match (&"default_weight", &value.get_type().name(), &"WeightT")
                                    {
                                        (arg0, arg1, arg2) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg2,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ));
                                res
                            }),
                        }
                    }
                }
            }?)
            .set_skip_self_loops(match kwargs.get_item("skip_self_loops") {
                None => Ok(None),
                Some(value) => {
                    if value.get_type().name() == "NoneType" {
                        Ok(None)
                    } else {
                        match value.extract::<bool>() {
                            Ok(v) => Ok(Some(v)),
                            Err(_) => Err({
                                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                    &[
                                        "The value passed for ",
                                        " cannot be casted from ",
                                        " to ",
                                        ".",
                                    ],
                                    &match (&"skip_self_loops", &value.get_type().name(), &"bool") {
                                        (arg0, arg1, arg2) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg2,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ));
                                res
                            }),
                        }
                    }
                }
            }?)
            .set_ignore_duplicates(match kwargs.get_item("ignore_duplicated_edges") {
                None => Ok(None),
                Some(value) => {
                    if value.get_type().name() == "NoneType" {
                        Ok(None)
                    } else {
                        match value.extract::<bool>() {
                            Ok(v) => Ok(Some(v)),
                            Err(_) => Err({
                                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                    &[
                                        "The value passed for ",
                                        " cannot be casted from ",
                                        " to ",
                                        ".",
                                    ],
                                    &match (
                                        &"ignore_duplicated_edges",
                                        &value.get_type().name(),
                                        &"bool",
                                    ) {
                                        (arg0, arg1, arg2) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg2,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ));
                                res
                            }),
                        }
                    }
                }
            }?)
            .set_verbose(match kwargs.get_item("verbose") {
                None => Ok(None),
                Some(value) => {
                    if value.get_type().name() == "NoneType" {
                        Ok(None)
                    } else {
                        match value.extract::<bool>() {
                            Ok(v) => Ok(Some(v)),
                            Err(_) => Err({
                                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                    &[
                                        "The value passed for ",
                                        " cannot be casted from ",
                                        " to ",
                                        ".",
                                    ],
                                    &match (&"verbose", &value.get_type().name(), &"bool") {
                                        (arg0, arg1, arg2) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg2,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ));
                                res
                            }),
                        }
                    }
                }
            }?)
            .set_numeric_node_ids(match kwargs.get_item("numeric_node_ids") {
                None => Ok(None),
                Some(value) => {
                    if value.get_type().name() == "NoneType" {
                        Ok(None)
                    } else {
                        match value.extract::<bool>() {
                            Ok(v) => Ok(Some(v)),
                            Err(_) => Err({
                                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                    &[
                                        "The value passed for ",
                                        " cannot be casted from ",
                                        " to ",
                                        ".",
                                    ],
                                    &match (&"numeric_node_ids", &value.get_type().name(), &"bool")
                                    {
                                        (arg0, arg1, arg2) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg2,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ));
                                res
                            }),
                        }
                    }
                }
            }?)
            .set_numeric_edge_type_ids(match kwargs.get_item("numeric_edge_type_ids") {
                None => Ok(None),
                Some(value) => {
                    if value.get_type().name() == "NoneType" {
                        Ok(None)
                    } else {
                        match value.extract::<bool>() {
                            Ok(v) => Ok(Some(v)),
                            Err(_) => Err({
                                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                    &[
                                        "The value passed for ",
                                        " cannot be casted from ",
                                        " to ",
                                        ".",
                                    ],
                                    &match (
                                        &"numeric_edge_type_ids",
                                        &value.get_type().name(),
                                        &"bool",
                                    ) {
                                        (arg0, arg1, arg2) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg2,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ));
                                res
                            }),
                        }
                    }
                }
            }?)
            .set_max_rows_number(match kwargs.get_item("edge_max_rows_number") {
                None => Ok(None),
                Some(value) => {
                    if value.get_type().name() == "NoneType" {
                        Ok(None)
                    } else {
                        match value.extract::<u64>() {
                            Ok(v) => Ok(Some(v)),
                            Err(_) => Err({
                                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                    &[
                                        "The value passed for ",
                                        " cannot be casted from ",
                                        " to ",
                                        ".",
                                    ],
                                    &match (
                                        &"edge_max_rows_number",
                                        &value.get_type().name(),
                                        &"u64",
                                    ) {
                                        (arg0, arg1, arg2) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg2,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ));
                                res
                            }),
                        }
                    }
                }
            }?);
        let nodes: Option<NodeFileReader> = match kwargs.get_item("node_path") {
            Some(_) => Some(
                NodeFileReader::new(
                    match kwargs.get_item("node_path") {
                        None => Ok(None),
                        Some(value) => {
                            if value.get_type().name() == "NoneType" {
                                Ok(None)
                            } else {
                                match value.extract::<String>() {
                                    Ok(v) => Ok(Some(v)),
                                    Err(_) => Err({
                                        let res =
                                            ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                                &[
                                                    "The value passed for ",
                                                    " cannot be casted from ",
                                                    " to ",
                                                    ".",
                                                ],
                                                &match (
                                                    &"node_path",
                                                    &value.get_type().name(),
                                                    &"String",
                                                ) {
                                                    (arg0, arg1, arg2) => [
                                                        ::core::fmt::ArgumentV1::new(
                                                            arg0,
                                                            ::core::fmt::Display::fmt,
                                                        ),
                                                        ::core::fmt::ArgumentV1::new(
                                                            arg1,
                                                            ::core::fmt::Display::fmt,
                                                        ),
                                                        ::core::fmt::ArgumentV1::new(
                                                            arg2,
                                                            ::core::fmt::Display::fmt,
                                                        ),
                                                    ],
                                                },
                                            ));
                                        res
                                    }),
                                }
                            }
                        }
                    }?
                    .unwrap(),
                )?
                .set_separator(match kwargs.get_item("node_separator") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<String>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (
                                            &"node_separator",
                                            &value.get_type().name(),
                                            &"String",
                                        ) {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                }?)?
                .set_header(match kwargs.get_item("edge_header") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<bool>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (&"edge_header", &value.get_type().name(), &"bool") {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                }?)
                .set_rows_to_skip(match kwargs.get_item("edge_rows_to_skip") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<usize>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (
                                            &"edge_rows_to_skip",
                                            &value.get_type().name(),
                                            &"usize",
                                        ) {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                }?)
                .set_nodes_column_number(match kwargs.get_item("nodes_column_number") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<usize>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (
                                            &"nodes_column_number",
                                            &value.get_type().name(),
                                            &"usize",
                                        ) {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                }?)?
                .set_nodes_column(match kwargs.get_item("nodes_column") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<String>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (
                                            &"nodes_column",
                                            &value.get_type().name(),
                                            &"String",
                                        ) {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                }?)?
                .set_node_types_column_number(match kwargs.get_item("node_types_column_number") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<usize>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (
                                            &"node_types_column_number",
                                            &value.get_type().name(),
                                            &"usize",
                                        ) {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                }?)?
                .set_node_types_column(match kwargs.get_item("node_types_column") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<String>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (
                                            &"node_types_column",
                                            &value.get_type().name(),
                                            &"String",
                                        ) {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                }?)?
                .set_default_node_type(match kwargs.get_item("default_node_type") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<String>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (
                                            &"default_node_type",
                                            &value.get_type().name(),
                                            &"String",
                                        ) {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                }?)
                .set_ignore_duplicates(match kwargs.get_item("ignore_duplicated_nodes") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<bool>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (
                                            &"ignore_duplicated_nodes",
                                            &value.get_type().name(),
                                            &"bool",
                                        ) {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                }?)
                .set_verbose(match kwargs.get_item("verbose") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<bool>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (&"verbose", &value.get_type().name(), &"bool") {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                }?)
                .set_numeric_node_ids(match kwargs.get_item("numeric_node_ids") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<bool>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (
                                            &"numeric_node_ids",
                                            &value.get_type().name(),
                                            &"bool",
                                        ) {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                }?)
                .set_numeric_node_type_ids(match kwargs.get_item("numeric_node_type_ids") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<bool>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (
                                            &"numeric_node_type_ids",
                                            &value.get_type().name(),
                                            &"bool",
                                        ) {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                }?)
                .set_max_rows_number(
                    match kwargs.get_item("node_max_rows_number") {
                        None => Ok(None),
                        Some(value) => {
                            if value.get_type().name() == "NoneType" {
                                Ok(None)
                            } else {
                                match value.extract::<u64>() {
                                    Ok(v) => Ok(Some(v)),
                                    Err(_) => Err({
                                        let res =
                                            ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                                &[
                                                    "The value passed for ",
                                                    " cannot be casted from ",
                                                    " to ",
                                                    ".",
                                                ],
                                                &match (
                                                    &"node_max_rows_number",
                                                    &value.get_type().name(),
                                                    &"u64",
                                                ) {
                                                    (arg0, arg1, arg2) => [
                                                        ::core::fmt::ArgumentV1::new(
                                                            arg0,
                                                            ::core::fmt::Display::fmt,
                                                        ),
                                                        ::core::fmt::ArgumentV1::new(
                                                            arg1,
                                                            ::core::fmt::Display::fmt,
                                                        ),
                                                        ::core::fmt::ArgumentV1::new(
                                                            arg2,
                                                            ::core::fmt::Display::fmt,
                                                        ),
                                                    ],
                                                },
                                            ));
                                        res
                                    }),
                                }
                            }
                        }
                    }?,
                ),
            ),
            None => None,
        };
        Ok((
            edges,
            nodes,
            match kwargs.get_item("name") {
                None => Ok(None),
                Some(value) => {
                    if value.get_type().name() == "NoneType" {
                        Ok(None)
                    } else {
                        match value.extract::<String>() {
                            Ok(v) => Ok(Some(v)),
                            Err(_) => Err({
                                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                    &[
                                        "The value passed for ",
                                        " cannot be casted from ",
                                        " to ",
                                        ".",
                                    ],
                                    &match (&"name", &value.get_type().name(), &"String") {
                                        (arg0, arg1, arg2) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg2,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ));
                                res
                            }),
                        }
                    }
                }
            }?
            .unwrap_or_else(|| "Graph".to_owned()),
        ))
    }
    impl EnsmallenGraph {
        pub(crate) fn build_walk_parameters(
            &self,
            length: NodeT,
            kwargs: &PyDict,
        ) -> Result<WalksParameters, String> {
            Ok(WalksParameters::new(length)?
                .set_change_edge_type_weight(match kwargs.get_item("change_edge_type_weight") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<WeightT>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (
                                            &"change_edge_type_weight",
                                            &value.get_type().name(),
                                            &"WeightT",
                                        ) {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                }?)?
                .set_change_node_type_weight(match kwargs.get_item("change_node_type_weight") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<WeightT>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (
                                            &"change_node_type_weight",
                                            &value.get_type().name(),
                                            &"WeightT",
                                        ) {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                }?)?
                .set_explore_weight(match kwargs.get_item("explore_weight") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<WeightT>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (
                                            &"explore_weight",
                                            &value.get_type().name(),
                                            &"WeightT",
                                        ) {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                }?)?
                .set_return_weight(match kwargs.get_item("return_weight") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<WeightT>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (
                                            &"return_weight",
                                            &value.get_type().name(),
                                            &"WeightT",
                                        ) {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                }?)?
                .set_random_state(match kwargs.get_item("random_state") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<usize>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (&"random_state", &value.get_type().name(), &"usize")
                                        {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                }?)
                .set_verbose(match kwargs.get_item("verbose") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<bool>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (&"verbose", &value.get_type().name(), &"bool") {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                }?)
                .set_iterations(match kwargs.get_item("iterations") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<NodeT>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (&"iterations", &value.get_type().name(), &"NodeT") {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                }?)?
                .set_min_length(match kwargs.get_item("min_length") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<NodeT>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (&"min_length", &value.get_type().name(), &"NodeT") {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                }?)?
                .set_dense_node_mapping(match kwargs.get_item("dense_node_mapping") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<HashMap<NodeT, NodeT>>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (
                                            &"dense_node_mapping",
                                            &value.get_type().name(),
                                            &"HashMap<NodeT, NodeT>",
                                        ) {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                }?))
        }
    }
}
pub(crate) use crate::preprocessing::*;
pub(crate) use crate::utilities::*;
mod types {
    use graph::{Graph, NodeT};
    use numpy::{PyArray1, PyArray2};
    use pyo3::prelude::*;
    pub(crate) struct EnsmallenGraph {
        pub(crate) graph: Graph,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for EnsmallenGraph {
        #[inline]
        fn clone(&self) -> EnsmallenGraph {
            match *self {
                EnsmallenGraph {
                    graph: ref __self_0_0,
                } => EnsmallenGraph {
                    graph: ::core::clone::Clone::clone(&(*__self_0_0)),
                },
            }
        }
    }
    unsafe impl pyo3::type_object::PyTypeInfo for EnsmallenGraph {
        type Type = EnsmallenGraph;
        type BaseType = pyo3::PyAny;
        type Layout = pyo3::PyCell<Self>;
        type BaseLayout = pyo3::pycell::PyCellBase<pyo3::PyAny>;
        type Initializer = pyo3::pyclass_init::PyClassInitializer<Self>;
        type AsRefTarget = pyo3::PyCell<Self>;
        const NAME: &'static str = "EnsmallenGraph";
        const MODULE: Option<&'static str> = None;
        const DESCRIPTION: &'static str = "\u{0}";
        const FLAGS: usize = 0 | 0;
        #[inline]
        fn type_object_raw(py: pyo3::Python) -> *mut pyo3::ffi::PyTypeObject {
            use pyo3::type_object::LazyStaticType;
            static TYPE_OBJECT: LazyStaticType = LazyStaticType::new();
            TYPE_OBJECT.get_or_init::<Self>(py)
        }
    }
    impl pyo3::PyClass for EnsmallenGraph {
        type Dict = pyo3::pyclass_slots::PyClassDummySlot;
        type WeakRef = pyo3::pyclass_slots::PyClassDummySlot;
        type BaseNativeType = pyo3::PyAny;
    }
    impl<'a> pyo3::derive_utils::ExtractExt<'a> for &'a EnsmallenGraph {
        type Target = pyo3::PyRef<'a, EnsmallenGraph>;
    }
    impl<'a> pyo3::derive_utils::ExtractExt<'a> for &'a mut EnsmallenGraph {
        type Target = pyo3::PyRefMut<'a, EnsmallenGraph>;
    }
    impl pyo3::pyclass::PyClassSend for EnsmallenGraph {
        type ThreadChecker = pyo3::pyclass::ThreadCheckerStub<EnsmallenGraph>;
    }
    impl pyo3::IntoPy<pyo3::PyObject> for EnsmallenGraph {
        fn into_py(self, py: pyo3::Python) -> pyo3::PyObject {
            pyo3::IntoPy::into_py(pyo3::Py::new(py, self).unwrap(), py)
        }
    }
    #[doc(hidden)]
    pub struct Pyo3MethodsInventoryForEnsmallenGraph {
        methods: Vec<pyo3::class::PyMethodDefType>,
    }
    impl pyo3::class::methods::PyMethodsInventory for Pyo3MethodsInventoryForEnsmallenGraph {
        fn new(methods: Vec<pyo3::class::PyMethodDefType>) -> Self {
            Self { methods }
        }
        fn get(&'static self) -> &'static [pyo3::class::PyMethodDefType] {
            &self.methods
        }
    }
    impl pyo3::class::methods::HasMethodsInventory for EnsmallenGraph {
        type Methods = Pyo3MethodsInventoryForEnsmallenGraph;
    }
    impl ::inventory::Collect for Pyo3MethodsInventoryForEnsmallenGraph {
        #[inline]
        fn registry() -> &'static ::inventory::Registry<Self> {
            static REGISTRY: ::inventory::Registry<Pyo3MethodsInventoryForEnsmallenGraph> =
                ::inventory::Registry::new();
            &REGISTRY
        }
    }
    impl pyo3::class::proto_methods::HasProtoRegistry for EnsmallenGraph {
        fn registry() -> &'static pyo3::class::proto_methods::PyProtoRegistry {
            static REGISTRY: pyo3::class::proto_methods::PyProtoRegistry =
                pyo3::class::proto_methods::PyProtoRegistry::new();
            &REGISTRY
        }
    }
    impl pyo3::pyclass::PyClassAlloc for EnsmallenGraph {}
    pub type PyContexts = Py<PyArray2<NodeT>>;
    pub type PyWords = Py<PyArray1<NodeT>>;
    pub type PyFrequencies = Py<PyArray1<f64>>;
}
pub(crate) use crate::types::*;
mod walks {
    use super::*;
    use numpy::{PyArray2};
    use graph::NodeT;
    impl EnsmallenGraph {
        /// Return random walks done on the graph using Rust.
        ///
        /// Parameters
        /// ---------------------
        /// length: int,
        ///     Maximal length of the random walk.
        ///     On graphs without traps, all walks have this length.
        /// quantity: int,
        ///     Number of nodes to sample.
        /// min_length: int = 0,
        ///     Minimal length of the random walk. Will filter out smaller
        ///     random walks.
        /// return_weight: float = 1.0,
        ///     Weight on the probability of returning to node coming from
        ///     Having this higher tends the walks to be
        ///     more like a Breadth-First Search.
        ///     Having this very high  (> 2) makes search very local.
        ///     Equal to the inverse of p in the Node2Vec paper.
        /// explore_weight: float = 1.0,
        ///     Weight on the probability of visiting a neighbor node
        ///     to the one we're coming from in the random walk
        ///     Having this higher tends the walks to be
        ///     more like a Depth-First Search.
        ///     Having this very high makes search more outward.
        ///     Having this very low makes search very local.
        ///     Equal to the inverse of q in the Node2Vec paper.
        /// change_edge_type_weight: float = 1.0,
        ///     Weight on the probability of visiting a neighbor node of a
        ///     different type than the previous node. This only applies to
        ///     colored graphs, otherwise it has no impact.
        /// change_node_type_weight: float = 1.0,
        ///     Weight on the probability of visiting a neighbor edge of a
        ///     different type than the previous edge. This only applies to
        ///     multigraphs, otherwise it has no impact.
        /// random_state: int = 42,
        ///     random_state to use to reproduce the walks.
        /// verbose: bool = False,
        ///     Wethever to show or not the loading bar of the walks.
        /// iterations: int = 1,
        ///     Number of cycles on the graphs to execute.
        /// dense_node_mapping: Dict[int, int] = None,
        ///     Mapping to use for converting sparse walk space into a dense space.
        ///     This object can be created using the method available from graph
        ///     called `get_dense_node_mapping` that returns a mapping from
        ///     the non trap nodes (those from where a walk could start) and
        ///     maps these nodes into a dense range of values.
        ///
        /// Raises
        /// ----------------------------
        /// TODO: Update raises
        ///
        /// Returns
        /// ----------------------------
        /// List of list of walks containing the numeric IDs of nodes.
        ///
        fn random_walks(
            &self,
            length: NodeT,
            quantity: NodeT,
            py_kwargs: Option<&PyDict>,
        ) -> PyResult<&PyArray2<NodeT>> {
            let py = pyo3::Python::acquire_gil();
            let kwargs = match py_kwargs {
                Some(v) => v,
                None => PyDict::new(py.python()),
            };
            match validate_kwargs(kwargs, build_walk_parameters_list(&[])) {
                Ok(v) => Ok(v),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }?;
            let parameters = match self.build_walk_parameters(length, kwargs) {
                Ok(v) => Ok(v),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }?;
            Ok(iter_to_nparray_2d(
                py.python(),
                length as usize,
                self.graph.get_nodes_number() as usize,
                match self.graph.random_walks_iter(quantity, &parameters) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?,
            ))
        }
        /// Return complete random walks done on the graph using Rust.
        ///
        /// Parameters
        /// ---------------------
        /// length: int,
        ///     Maximal length of the random walk.
        ///     On graphs without traps, all walks have this length.
        /// min_length: int = 0,
        ///     Minimal length of the random walk. Will filter out smaller
        ///     random walks.
        /// return_weight: float = 1.0,
        ///     Weight on the probability of returning to node coming from
        ///     Having this higher tends the walks to be
        ///     more like a Breadth-First Search.
        ///     Having this very high  (> 2) makes search very local.
        ///     Equal to the inverse of p in the Node2Vec paper.
        /// explore_weight: float = 1.0,
        ///     Weight on the probability of visiting a neighbor node
        ///     to the one we're coming from in the random walk
        ///     Having this higher tends the walks to be
        ///     more like a Depth-First Search.
        ///     Having this very high makes search more outward.
        ///     Having this very low makes search very local.
        ///     Equal to the inverse of q in the Node2Vec paper.
        /// change_edge_type_weight: float = 1.0,
        ///     Weight on the probability of visiting a neighbor node of a
        ///     different type than the previous node. This only applies to
        ///     colored graphs, otherwise it has no impact.
        /// change_node_type_weight: float = 1.0,
        ///     Weight on the probability of visiting a neighbor edge of a
        ///     different type than the previous edge. This only applies to
        ///     multigraphs, otherwise it has no impact.
        /// random_state: int = 42,
        ///     random_state to use to reproduce the walks.
        /// verbose: bool = False,
        ///     Wethever to show or not the loading bar of the walks.
        /// iterations: int = 1,
        ///     Number of cycles on the graphs to execute.
        /// dense_node_mapping: Dict[int, int] = None,
        ///     Mapping to use for converting sparse walk space into a dense space.
        ///     This object can be created using the method available from graph
        ///     called `get_dense_node_mapping` that returns a mapping from
        ///     the non trap nodes (those from where a walk could start) and
        ///     maps these nodes into a dense range of values.
        ///
        /// Raises
        /// ----------------------------
        /// TODO: Update raises
        ///
        /// Returns
        /// ----------------------------
        /// List of list of walks containing the numeric IDs of nodes.
        ///
        fn complete_walks(
            &self,
            length: NodeT,
            py_kwargs: Option<&PyDict>,
        ) -> PyResult<&'_ PyArray2<NodeT>> {
            let py = pyo3::Python::acquire_gil();
            let kwargs = match py_kwargs {
                Some(v) => v,
                None => PyDict::new(py.python()),
            };
            match validate_kwargs(kwargs, build_walk_parameters_list(&[])) {
                Ok(v) => Ok(v),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }?;
            let parameters = match self.build_walk_parameters(length, kwargs) {
                Ok(v) => Ok(v),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }?;
            Ok(iter_to_nparray_2d(
                py.python(),
                length as usize,
                self.graph.get_nodes_number() as usize,
                match self.graph.complete_walks_iter(&parameters) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?,
            ))
        }
    }
    #[used]
    #[allow(non_upper_case_globals)]
    #[link_section = ".init_array"]
    #[allow(non_upper_case_globals)]
    static __init2653520877317759370: extern "C" fn() = {
        #[link_section = ".text.startup"]
        extern "C" fn __init2653520877317759370() {
            pyo3::inventory::submit({
                {
                    type Inventory =
                        <EnsmallenGraph as pyo3::class::methods::HasMethodsInventory>::Methods;
                    <Inventory as pyo3::class::methods::PyMethodsInventory>::new(<[_]>::into_vec(
                        box [
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                    _kwargs: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str = "EnsmallenGraph.random_walks()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    let _args = _py
                                                        .from_borrowed_ptr::<pyo3::types::PyTuple>(
                                                            _args,
                                                        );
                                                    let _kwargs: Option<&pyo3::types::PyDict> =
                                                        _py.from_borrowed_ptr_or_opt(_kwargs);
                                                    pyo3::callback::convert(_py, {
                                                        const PARAMS : & 'static [pyo3 :: derive_utils :: ParamDescription] = & [pyo3 :: derive_utils :: ParamDescription { name : "length" , is_optional : false , kw_only : false , } , pyo3 :: derive_utils :: ParamDescription { name : "quantity" , is_optional : false , kw_only : false , }] ;
                                                        let mut output = [None; 2usize];
                                                        let mut _args = _args;
                                                        let mut _kwargs = _kwargs;
                                                        let (_args, _kwargs) =
                                                            pyo3::derive_utils::parse_fn_args(
                                                                Some(_LOCATION),
                                                                PARAMS,
                                                                _args,
                                                                _kwargs,
                                                                false,
                                                                true,
                                                                &mut output,
                                                            )?;
                                                        let arg0 = match output [0usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        let arg1 = match output [1usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        let arg2 = _kwargs;
                                                        EnsmallenGraph::random_walks(
                                                            _slf, arg0, arg1, arg2,
                                                        )
                                                    })
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction_with_keywords ("random_walks\u{0}" , __wrap , 0 , "random_walks($self, length, quantity, *, min_length, return_weight, explore_weight, change_edge_type_weight, change_node_type_weight, random_state, verbose, iterations, dense_node_mapping)\n--\n\nReturn random walks done on the graph using Rust.\n\nParameters\n---------------------\nlength: int,\n    Maximal length of the random walk.\n    On graphs without traps, all walks have this length.\nquantity: int,\n    Number of nodes to sample.\nmin_length: int = 0,\n    Minimal length of the random walk. Will filter out smaller\n    random walks.\nreturn_weight: float = 1.0,\n    Weight on the probability of returning to node coming from\n    Having this higher tends the walks to be\n    more like a Breadth-First Search.\n    Having this very high  (> 2) makes search very local.\n    Equal to the inverse of p in the Node2Vec paper.\nexplore_weight: float = 1.0,\n    Weight on the probability of visiting a neighbor node\n    to the one we\'re coming from in the random walk\n    Having this higher tends the walks to be\n    more like a Depth-First Search.\n    Having this very high makes search more outward.\n    Having this very low makes search very local.\n    Equal to the inverse of q in the Node2Vec paper.\nchange_edge_type_weight: float = 1.0,\n    Weight on the probability of visiting a neighbor node of a\n    different type than the previous node. This only applies to\n    colored graphs, otherwise it has no impact.\nchange_node_type_weight: float = 1.0,\n    Weight on the probability of visiting a neighbor edge of a\n    different type than the previous edge. This only applies to\n    multigraphs, otherwise it has no impact.\nrandom_state: int = 42,\n    random_state to use to reproduce the walks.\nverbose: bool = False,\n    Wethever to show or not the loading bar of the walks.\niterations: int = 1,\n    Number of cycles on the graphs to execute.\ndense_node_mapping: Dict[int, int] = None,\n    Mapping to use for converting sparse walk space into a dense space.\n    This object can be created using the method available from graph\n    called `get_dense_node_mapping` that returns a mapping from\n    the non trap nodes (those from where a walk could start) and\n    maps these nodes into a dense range of values.\n\nRaises\n----------------------------\nTODO: Update raises\n\nReturns\n----------------------------\nList of list of walks containing the numeric IDs of nodes.\n\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                    _kwargs: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.complete_walks()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    let _args = _py
                                                        .from_borrowed_ptr::<pyo3::types::PyTuple>(
                                                            _args,
                                                        );
                                                    let _kwargs: Option<&pyo3::types::PyDict> =
                                                        _py.from_borrowed_ptr_or_opt(_kwargs);
                                                    pyo3::callback::convert(_py, {
                                                        const PARAMS : & 'static [pyo3 :: derive_utils :: ParamDescription] = & [pyo3 :: derive_utils :: ParamDescription { name : "length" , is_optional : false , kw_only : false , }] ;
                                                        let mut output = [None; 1usize];
                                                        let mut _args = _args;
                                                        let mut _kwargs = _kwargs;
                                                        let (_args, _kwargs) =
                                                            pyo3::derive_utils::parse_fn_args(
                                                                Some(_LOCATION),
                                                                PARAMS,
                                                                _args,
                                                                _kwargs,
                                                                false,
                                                                true,
                                                                &mut output,
                                                            )?;
                                                        let arg0 = match output [0usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        let arg1 = _kwargs;
                                                        EnsmallenGraph::complete_walks(
                                                            _slf, arg0, arg1,
                                                        )
                                                    })
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction_with_keywords ("complete_walks\u{0}" , __wrap , 0 , "complete_walks($self, length, *, min_length, return_weight, explore_weight, change_edge_type_weight, change_node_type_weight, random_state, verbose, iterations, dense_node_mapping)\n--\n\nReturn complete random walks done on the graph using Rust.\n\nParameters\n---------------------\nlength: int,\n    Maximal length of the random walk.\n    On graphs without traps, all walks have this length.\nmin_length: int = 0,\n    Minimal length of the random walk. Will filter out smaller\n    random walks.\nreturn_weight: float = 1.0,\n    Weight on the probability of returning to node coming from\n    Having this higher tends the walks to be\n    more like a Breadth-First Search.\n    Having this very high  (> 2) makes search very local.\n    Equal to the inverse of p in the Node2Vec paper.\nexplore_weight: float = 1.0,\n    Weight on the probability of visiting a neighbor node\n    to the one we\'re coming from in the random walk\n    Having this higher tends the walks to be\n    more like a Depth-First Search.\n    Having this very high makes search more outward.\n    Having this very low makes search very local.\n    Equal to the inverse of q in the Node2Vec paper.\nchange_edge_type_weight: float = 1.0,\n    Weight on the probability of visiting a neighbor node of a\n    different type than the previous node. This only applies to\n    colored graphs, otherwise it has no impact.\nchange_node_type_weight: float = 1.0,\n    Weight on the probability of visiting a neighbor edge of a\n    different type than the previous edge. This only applies to\n    multigraphs, otherwise it has no impact.\nrandom_state: int = 42,\n    random_state to use to reproduce the walks.\nverbose: bool = False,\n    Wethever to show or not the loading bar of the walks.\niterations: int = 1,\n    Number of cycles on the graphs to execute.\ndense_node_mapping: Dict[int, int] = None,\n    Mapping to use for converting sparse walk space into a dense space.\n    This object can be created using the method available from graph\n    called `get_dense_node_mapping` that returns a mapping from\n    the non trap nodes (those from where a walk could start) and\n    maps these nodes into a dense range of values.\n\nRaises\n----------------------------\nTODO: Update raises\n\nReturns\n----------------------------\nList of list of walks containing the numeric IDs of nodes.\n\u{0}")
                            }),
                        ],
                    ))
                }
            });
        }
        __init2653520877317759370
    };
}
pub(crate) use crate::types::EnsmallenGraph;
mod modifiers {
    use super::*;
    impl EnsmallenGraph {
        /// Drop all edge types (if presents) and set all the edge to edge_type.
        ///
        /// Arguments
        /// ---------
        /// edge_type: str,
        ///     The edge type to assing to all the edges.
        pub fn set_all_edge_types(&self, edge_type: String) -> EnsmallenGraph {
            EnsmallenGraph {
                graph: self.graph.clone().set_all_edge_types(edge_type),
            }
        }
        /// Drop all node types (if presents) and set all the node to node_type.
        ///
        /// Arguments
        /// ---------
        /// node_type: str,
        ///     The node type to assing to all the nodes.
        pub fn set_all_node_types(&self, node_type: String) -> EnsmallenGraph {
            EnsmallenGraph {
                graph: self.graph.clone().set_all_node_types(node_type),
            }
        }
        /// Enable fast walk, using more memory.
        ///
        /// Arguments
        /// ------------------
        /// vector_destinations: bool,
        ///     wether to cache destinations into a vector for faster walks.
        /// vector_outbounds: bool,
        ///     wether to cache outbounds into a vector for faster walks.
        pub fn enable_fast_walk(&mut self, py_kwargs: Option<&PyDict>) -> PyResult<()> {
            let py = pyo3::Python::acquire_gil();
            let kwargs = match py_kwargs {
                Some(v) => v,
                None => PyDict::new(py.python()),
            };
            match validate_kwargs(
                kwargs,
                ["vector_destinations", "vector_outbounds"]
                    .iter()
                    .map(|x| x.to_string())
                    .collect(),
            ) {
                Ok(v) => Ok(v),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }?;
            self.graph.enable_fast_walk(
                match match kwargs.get_item("vector_destinations") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<bool>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (
                                            &"vector_destinations",
                                            &value.get_type().name(),
                                            &"bool",
                                        ) {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                } {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?
                .unwrap_or(true),
                match match kwargs.get_item("vector_outbounds") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<bool>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (
                                            &"vector_outbounds",
                                            &value.get_type().name(),
                                            &"bool",
                                        ) {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                } {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?
                .unwrap_or(true),
            );
            Ok(())
        }
        /// Disable fast walk, using less memory.
        pub fn disable_fast_walk(&mut self) {
            self.graph.disable_fast_walk()
        }
    }
    #[used]
    #[allow(non_upper_case_globals)]
    #[link_section = ".init_array"]
    #[allow(non_upper_case_globals)]
    static __init1665505439480811823: extern "C" fn() = {
        #[link_section = ".text.startup"]
        extern "C" fn __init1665505439480811823() {
            pyo3::inventory::submit({
                {
                    type Inventory =
                        <EnsmallenGraph as pyo3::class::methods::HasMethodsInventory>::Methods;
                    <Inventory as pyo3::class::methods::PyMethodsInventory>::new(<[_]>::into_vec(
                        box [
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                    _kwargs: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.set_all_edge_types()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    let _args = _py
                                                        .from_borrowed_ptr::<pyo3::types::PyTuple>(
                                                            _args,
                                                        );
                                                    let _kwargs: Option<&pyo3::types::PyDict> =
                                                        _py.from_borrowed_ptr_or_opt(_kwargs);
                                                    pyo3::callback::convert(_py, {
                                                        const PARAMS : & 'static [pyo3 :: derive_utils :: ParamDescription] = & [pyo3 :: derive_utils :: ParamDescription { name : "edge_type" , is_optional : false , kw_only : false , }] ;
                                                        let mut output = [None; 1usize];
                                                        let mut _args = _args;
                                                        let mut _kwargs = _kwargs;
                                                        let (_args, _kwargs) =
                                                            pyo3::derive_utils::parse_fn_args(
                                                                Some(_LOCATION),
                                                                PARAMS,
                                                                _args,
                                                                _kwargs,
                                                                false,
                                                                false,
                                                                &mut output,
                                                            )?;
                                                        let arg0 = match output [0usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        EnsmallenGraph::set_all_edge_types(
                                                            _slf, arg0,
                                                        )
                                                    })
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction_with_keywords ("set_all_edge_types\u{0}" , __wrap , 0 , "set_all_edge_types($self, edge_type)\n--\n\nDrop all edge types (if presents) and set all the edge to edge_type.\n\nArguments\n---------\nedge_type: str,\n    The edge type to assing to all the edges.\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                    _kwargs: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.set_all_node_types()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    let _args = _py
                                                        .from_borrowed_ptr::<pyo3::types::PyTuple>(
                                                            _args,
                                                        );
                                                    let _kwargs: Option<&pyo3::types::PyDict> =
                                                        _py.from_borrowed_ptr_or_opt(_kwargs);
                                                    pyo3::callback::convert(_py, {
                                                        const PARAMS : & 'static [pyo3 :: derive_utils :: ParamDescription] = & [pyo3 :: derive_utils :: ParamDescription { name : "node_type" , is_optional : false , kw_only : false , }] ;
                                                        let mut output = [None; 1usize];
                                                        let mut _args = _args;
                                                        let mut _kwargs = _kwargs;
                                                        let (_args, _kwargs) =
                                                            pyo3::derive_utils::parse_fn_args(
                                                                Some(_LOCATION),
                                                                PARAMS,
                                                                _args,
                                                                _kwargs,
                                                                false,
                                                                false,
                                                                &mut output,
                                                            )?;
                                                        let arg0 = match output [0usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        EnsmallenGraph::set_all_node_types(
                                                            _slf, arg0,
                                                        )
                                                    })
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction_with_keywords ("set_all_node_types\u{0}" , __wrap , 0 , "set_all_node_types($self, node_type)\n--\n\nDrop all node types (if presents) and set all the node to node_type.\n\nArguments\n---------\nnode_type: str,\n    The node type to assing to all the nodes.\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                    _kwargs: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.enable_fast_walk()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let mut _ref = _cell.try_borrow_mut()?;
                                                    let _slf = &mut _ref;
                                                    let _args = _py
                                                        .from_borrowed_ptr::<pyo3::types::PyTuple>(
                                                            _args,
                                                        );
                                                    let _kwargs: Option<&pyo3::types::PyDict> =
                                                        _py.from_borrowed_ptr_or_opt(_kwargs);
                                                    pyo3::callback::convert(_py, {
                                                        const PARAMS : & 'static [pyo3 :: derive_utils :: ParamDescription] = & [] ;
                                                        let mut output = [None; 0usize];
                                                        let mut _args = _args;
                                                        let mut _kwargs = _kwargs;
                                                        let (_args, _kwargs) =
                                                            pyo3::derive_utils::parse_fn_args(
                                                                Some(_LOCATION),
                                                                PARAMS,
                                                                _args,
                                                                _kwargs,
                                                                false,
                                                                true,
                                                                &mut output,
                                                            )?;
                                                        let arg0 = _kwargs;
                                                        EnsmallenGraph::enable_fast_walk(_slf, arg0)
                                                    })
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction_with_keywords ("enable_fast_walk\u{0}" , __wrap , 0 , "enable_fast_walk($self, *, vector_destinations, vector_outbounds)\n--\n\nEnable fast walk, using more memory.\n\nArguments\n------------------\nvector_destinations: bool,\n    wether to cache destinations into a vector for faster walks.\nvector_outbounds: bool,\n    wether to cache outbounds into a vector for faster walks.\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.disable_fast_walk()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let mut _ref = _cell.try_borrow_mut()?;
                                                    let _slf = &mut _ref;
                                                    pyo3::callback::convert(
                                                        _py,
                                                        EnsmallenGraph::disable_fast_walk(_slf),
                                                    )
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction ("disable_fast_walk\u{0}" , __wrap , "disable_fast_walk($self)\n--\n\nDisable fast walk, using less memory.\u{0}")
                            }),
                        ],
                    ))
                }
            });
        }
        __init1665505439480811823
    };
}
mod drop {
    use super::*;
    impl EnsmallenGraph {
        /// Returns new graph without edge types.
        ///
        /// Raises
        /// -----------------------------
        /// ValueError,
        ///     If the given graph does not have edge types.
        ///
        /// Returns
        /// -----------------------------
        /// Cloned graph without edge types.
        fn drop_edge_types(&self) -> PyResult<EnsmallenGraph> {
            Ok(EnsmallenGraph {
                graph: match self.graph.drop_edge_types() {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?,
            })
        }
        /// Returns new graph without singleton nodes.
        ///
        /// Note that this will change the internal nodes mapping.
        ///
        /// Parameters
        /// -----------------------------
        /// verbose: bool = True,
        ///     Wether to show a loading bar.
        ///
        /// Returns
        /// -----------------------------
        /// Cloned graph without singleton nodes.
        fn drop_singletons(&self, verbose: Option<bool>) -> PyResult<EnsmallenGraph> {
            Ok(EnsmallenGraph {
                graph: match self.graph.drop_singletons(verbose.unwrap_or(true)) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?,
            })
        }
        /// Returns new graph without weights.
        ///
        /// Raises
        /// -----------------------------
        /// ValueError,
        ///     If the given graph does not have weights.
        ///
        /// Returns
        /// -----------------------------
        /// Cloned graph without weights.
        fn drop_weights(&self) -> PyResult<EnsmallenGraph> {
            Ok(EnsmallenGraph {
                graph: match self.graph.drop_weights() {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?,
            })
        }
        /// Returns new graph without node types.
        ///
        /// Raises
        /// -----------------------------
        /// ValueError,
        ///     If the given graph does not have node types.
        ///
        /// Returns
        /// -----------------------------
        /// Cloned graph without node types.
        fn drop_node_types(&self) -> PyResult<EnsmallenGraph> {
            Ok(EnsmallenGraph {
                graph: match self.graph.drop_node_types() {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?,
            })
        }
        /// Drop all the components that are not connected to interesting
        /// nodes and edges.
        ///
        /// Parameters
        /// ----------
        /// node_names: List[str],
        ///     The name of the nodes of which components to keep
        /// node_types: List[str],
        ///     The types of the nodes of which components to keep
        /// edge_types: List[str],
        ///     The types of the edges of which components to keep
        ///
        fn drop_components(&self, py_kwargs: Option<&PyDict>) -> PyResult<EnsmallenGraph> {
            let py = pyo3::Python::acquire_gil();
            let kwargs = match py_kwargs {
                Some(v) => v,
                None => PyDict::new(py.python()),
            };
            match validate_kwargs(
                kwargs,
                build_walk_parameters_list(&["node_names", "node_types", "edge_types", "verbose"]),
            ) {
                Ok(v) => Ok(v),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }?;
            Ok(EnsmallenGraph {
                graph: match self.graph.drop_components(
                    match match kwargs.get_item("node_names") {
                        None => Ok(None),
                        Some(value) => {
                            if value.get_type().name() == "NoneType" {
                                Ok(None)
                            } else {
                                match value.extract::<Vec<String>>() {
                                    Ok(v) => Ok(Some(v)),
                                    Err(_) => Err({
                                        let res =
                                            ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                                &[
                                                    "The value passed for ",
                                                    " cannot be casted from ",
                                                    " to ",
                                                    ".",
                                                ],
                                                &match (
                                                    &"node_names",
                                                    &value.get_type().name(),
                                                    &"Vec<String>",
                                                ) {
                                                    (arg0, arg1, arg2) => [
                                                        ::core::fmt::ArgumentV1::new(
                                                            arg0,
                                                            ::core::fmt::Display::fmt,
                                                        ),
                                                        ::core::fmt::ArgumentV1::new(
                                                            arg1,
                                                            ::core::fmt::Display::fmt,
                                                        ),
                                                        ::core::fmt::ArgumentV1::new(
                                                            arg2,
                                                            ::core::fmt::Display::fmt,
                                                        ),
                                                    ],
                                                },
                                            ));
                                        res
                                    }),
                                }
                            }
                        }
                    } {
                        Ok(v) => Ok(v),
                        Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                    }?,
                    match match kwargs.get_item("node_types") {
                        None => Ok(None),
                        Some(value) => {
                            if value.get_type().name() == "NoneType" {
                                Ok(None)
                            } else {
                                match value.extract::<Vec<String>>() {
                                    Ok(v) => Ok(Some(v)),
                                    Err(_) => Err({
                                        let res =
                                            ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                                &[
                                                    "The value passed for ",
                                                    " cannot be casted from ",
                                                    " to ",
                                                    ".",
                                                ],
                                                &match (
                                                    &"node_types",
                                                    &value.get_type().name(),
                                                    &"Vec<String>",
                                                ) {
                                                    (arg0, arg1, arg2) => [
                                                        ::core::fmt::ArgumentV1::new(
                                                            arg0,
                                                            ::core::fmt::Display::fmt,
                                                        ),
                                                        ::core::fmt::ArgumentV1::new(
                                                            arg1,
                                                            ::core::fmt::Display::fmt,
                                                        ),
                                                        ::core::fmt::ArgumentV1::new(
                                                            arg2,
                                                            ::core::fmt::Display::fmt,
                                                        ),
                                                    ],
                                                },
                                            ));
                                        res
                                    }),
                                }
                            }
                        }
                    } {
                        Ok(v) => Ok(v),
                        Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                    }?,
                    match match kwargs.get_item("edge_types") {
                        None => Ok(None),
                        Some(value) => {
                            if value.get_type().name() == "NoneType" {
                                Ok(None)
                            } else {
                                match value.extract::<Vec<String>>() {
                                    Ok(v) => Ok(Some(v)),
                                    Err(_) => Err({
                                        let res =
                                            ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                                &[
                                                    "The value passed for ",
                                                    " cannot be casted from ",
                                                    " to ",
                                                    ".",
                                                ],
                                                &match (
                                                    &"edge_types",
                                                    &value.get_type().name(),
                                                    &"Vec<String>",
                                                ) {
                                                    (arg0, arg1, arg2) => [
                                                        ::core::fmt::ArgumentV1::new(
                                                            arg0,
                                                            ::core::fmt::Display::fmt,
                                                        ),
                                                        ::core::fmt::ArgumentV1::new(
                                                            arg1,
                                                            ::core::fmt::Display::fmt,
                                                        ),
                                                        ::core::fmt::ArgumentV1::new(
                                                            arg2,
                                                            ::core::fmt::Display::fmt,
                                                        ),
                                                    ],
                                                },
                                            ));
                                        res
                                    }),
                                }
                            }
                        }
                    } {
                        Ok(v) => Ok(v),
                        Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                    }?,
                    match match kwargs.get_item("verbose") {
                        None => Ok(None),
                        Some(value) => {
                            if value.get_type().name() == "NoneType" {
                                Ok(None)
                            } else {
                                match value.extract::<bool>() {
                                    Ok(v) => Ok(Some(v)),
                                    Err(_) => Err({
                                        let res =
                                            ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                                &[
                                                    "The value passed for ",
                                                    " cannot be casted from ",
                                                    " to ",
                                                    ".",
                                                ],
                                                &match (
                                                    &"verbose",
                                                    &value.get_type().name(),
                                                    &"bool",
                                                ) {
                                                    (arg0, arg1, arg2) => [
                                                        ::core::fmt::ArgumentV1::new(
                                                            arg0,
                                                            ::core::fmt::Display::fmt,
                                                        ),
                                                        ::core::fmt::ArgumentV1::new(
                                                            arg1,
                                                            ::core::fmt::Display::fmt,
                                                        ),
                                                        ::core::fmt::ArgumentV1::new(
                                                            arg2,
                                                            ::core::fmt::Display::fmt,
                                                        ),
                                                    ],
                                                },
                                            ));
                                        res
                                    }),
                                }
                            }
                        }
                    } {
                        Ok(v) => Ok(v),
                        Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                    }?
                    .unwrap_or(true),
                ) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?,
            })
        }
    }
    #[used]
    #[allow(non_upper_case_globals)]
    #[link_section = ".init_array"]
    #[allow(non_upper_case_globals)]
    static __init18178713379539461897: extern "C" fn() = {
        #[link_section = ".text.startup"]
        extern "C" fn __init18178713379539461897() {
            pyo3::inventory::submit({
                {
                    type Inventory =
                        <EnsmallenGraph as pyo3::class::methods::HasMethodsInventory>::Methods;
                    <Inventory as pyo3::class::methods::PyMethodsInventory>::new(<[_]>::into_vec(
                        box [
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.drop_edge_types()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    pyo3::callback::convert(
                                                        _py,
                                                        EnsmallenGraph::drop_edge_types(_slf),
                                                    )
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction ("drop_edge_types\u{0}" , __wrap , "drop_edge_types($self)\n--\n\nReturns new graph without edge types.\n\nRaises\n-----------------------------\nValueError,\n    If the given graph does not have edge types.\n\nReturns\n-----------------------------\nCloned graph without edge types.\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                    _kwargs: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.drop_singletons()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    let _args = _py
                                                        .from_borrowed_ptr::<pyo3::types::PyTuple>(
                                                            _args,
                                                        );
                                                    let _kwargs: Option<&pyo3::types::PyDict> =
                                                        _py.from_borrowed_ptr_or_opt(_kwargs);
                                                    pyo3::callback::convert(_py, {
                                                        const PARAMS : & 'static [pyo3 :: derive_utils :: ParamDescription] = & [pyo3 :: derive_utils :: ParamDescription { name : "verbose" , is_optional : true , kw_only : false , }] ;
                                                        let mut output = [None; 1usize];
                                                        let mut _args = _args;
                                                        let mut _kwargs = _kwargs;
                                                        let (_args, _kwargs) =
                                                            pyo3::derive_utils::parse_fn_args(
                                                                Some(_LOCATION),
                                                                PARAMS,
                                                                _args,
                                                                _kwargs,
                                                                false,
                                                                true,
                                                                &mut output,
                                                            )?;
                                                        let arg0 = match output[0usize] {
                                                            Some(_obj) => _obj.extract()?,
                                                            None => None,
                                                        };
                                                        EnsmallenGraph::drop_singletons(_slf, arg0)
                                                    })
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction_with_keywords ("drop_singletons\u{0}" , __wrap , 0 , "drop_singletons($self, verbose)\n--\n\nReturns new graph without singleton nodes.\n\nNote that this will change the internal nodes mapping.\n\nParameters\n-----------------------------\nverbose: bool = True,\n    Wether to show a loading bar.\n\nReturns\n-----------------------------\nCloned graph without singleton nodes.\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str = "EnsmallenGraph.drop_weights()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    pyo3::callback::convert(
                                                        _py,
                                                        EnsmallenGraph::drop_weights(_slf),
                                                    )
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction ("drop_weights\u{0}" , __wrap , "drop_weights($self)\n--\n\nReturns new graph without weights.\n\nRaises\n-----------------------------\nValueError,\n    If the given graph does not have weights.\n\nReturns\n-----------------------------\nCloned graph without weights.\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.drop_node_types()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    pyo3::callback::convert(
                                                        _py,
                                                        EnsmallenGraph::drop_node_types(_slf),
                                                    )
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction ("drop_node_types\u{0}" , __wrap , "drop_node_types($self)\n--\n\nReturns new graph without node types.\n\nRaises\n-----------------------------\nValueError,\n    If the given graph does not have node types.\n\nReturns\n-----------------------------\nCloned graph without node types.\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                    _kwargs: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.drop_components()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    let _args = _py
                                                        .from_borrowed_ptr::<pyo3::types::PyTuple>(
                                                            _args,
                                                        );
                                                    let _kwargs: Option<&pyo3::types::PyDict> =
                                                        _py.from_borrowed_ptr_or_opt(_kwargs);
                                                    pyo3::callback::convert(_py, {
                                                        const PARAMS : & 'static [pyo3 :: derive_utils :: ParamDescription] = & [] ;
                                                        let mut output = [None; 0usize];
                                                        let mut _args = _args;
                                                        let mut _kwargs = _kwargs;
                                                        let (_args, _kwargs) =
                                                            pyo3::derive_utils::parse_fn_args(
                                                                Some(_LOCATION),
                                                                PARAMS,
                                                                _args,
                                                                _kwargs,
                                                                false,
                                                                true,
                                                                &mut output,
                                                            )?;
                                                        let arg0 = _kwargs;
                                                        EnsmallenGraph::drop_components(_slf, arg0)
                                                    })
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction_with_keywords ("drop_components\u{0}" , __wrap , 0 , "drop_components($self, *, node_names, node_types, edge_types, verbose)\n--\n\nDrop all the components that are not connected to interesting\nnodes and edges.\n\nParameters\n----------\nnode_names: List[str],\n    The name of the nodes of which components to keep\nnode_types: List[str],\n    The types of the nodes of which components to keep\nedge_types: List[str],\n    The types of the edges of which components to keep\n\u{0}")
                            }),
                        ],
                    ))
                }
            });
        }
        __init18178713379539461897
    };
}
mod holdout {
    use super::*;
    use graph::{EdgeT, NodeT};
    impl EnsmallenGraph {
        /// Returns training and validation holdouts extracted from current graph.
        ///
        /// The holdouts is generated in such a way that the training set remains
        /// connected if the starting graph is connected by using a spanning tree.
        ///
        /// Parameters
        /// -----------------------------
        /// train_size: float,
        ///     The rate of edges to reserve for the training.
        /// random_state: int = 42,
        ///     The random_state to use to generate the holdout.
        /// edge_types: List[str] = None,
        ///     List of names of the edge types to put into the validation.
        /// include_all_edge_types: bool = False,
        ///     Wethever to include all the edges between two nodes.
        ///     This is only relevant in multi-graphs.
        /// verbose: bool = True,
        ///     Wethever to show the loading bar.
        ///
        /// Raises
        /// -----------------------------
        /// ValueError,
        ///     If the given train rate is not a real number between 0 and 1.
        ///
        /// Returns
        /// -----------------------------
        /// Tuple containing training and validation graphs.
        fn connected_holdout(
            &self,
            train_size: f64,
            py_kwargs: Option<&PyDict>,
        ) -> PyResult<(EnsmallenGraph, EnsmallenGraph)> {
            let py = pyo3::Python::acquire_gil();
            let kwargs = match py_kwargs {
                Some(v) => v,
                None => PyDict::new(py.python()),
            };
            match validate_kwargs(
                kwargs,
                build_walk_parameters_list(&[
                    "random_state",
                    "edge_types",
                    "include_all_edge_types",
                    "verbose",
                ]),
            ) {
                Ok(v) => Ok(v),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }?;
            let (g1, g2) = match self.graph.connected_holdout(
                match match kwargs.get_item("random_state") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<EdgeT>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (&"random_state", &value.get_type().name(), &"EdgeT")
                                        {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                } {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?
                .unwrap_or(42),
                train_size,
                match match kwargs.get_item("edge_types") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<Vec<String>>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (
                                            &"edge_types",
                                            &value.get_type().name(),
                                            &"Vec<String>",
                                        ) {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                } {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?,
                match match kwargs.get_item("include_all_edge_types") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<bool>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (
                                            &"include_all_edge_types",
                                            &value.get_type().name(),
                                            &"bool",
                                        ) {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                } {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?
                .unwrap_or(false),
                match match kwargs.get_item("verbose") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<bool>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (&"verbose", &value.get_type().name(), &"bool") {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                } {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?
                .unwrap_or(true),
            ) {
                Ok(v) => Ok(v),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }?;
            Ok((EnsmallenGraph { graph: g1 }, EnsmallenGraph { graph: g2 }))
        }
        /// Returns partial subgraph.
        ///
        /// This method creates a subset of the graph starting from a random node
        /// sampled using given random_state and includes all neighbouring nodes until
        /// the required number of nodes is reached. All the edges connecting any
        /// of the selected nodes are then inserted into this graph.
        ///
        /// Parameters
        /// -----------------------------
        /// nodes_number: int,
        ///     The number of edges to insert in the partial graph.
        /// random_state: int = 42,
        ///     The random_state to use to generate the partial graph.
        /// verbose: bool = True,
        ///     Wethever to show the loading bar.
        ///
        /// Raises
        /// -----------------------------
        /// TODO: Add the docstring for the raised exceptions.
        ///
        /// Returns
        /// -----------------------------
        /// Partial graph.
        fn random_subgraph(
            &self,
            nodes_number: NodeT,
            py_kwargs: Option<&PyDict>,
        ) -> PyResult<EnsmallenGraph> {
            let py = pyo3::Python::acquire_gil();
            let kwargs = match py_kwargs {
                Some(v) => v,
                None => PyDict::new(py.python()),
            };
            match validate_kwargs(
                kwargs,
                build_walk_parameters_list(&["random_state", "verbose"]),
            ) {
                Ok(v) => Ok(v),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }?;
            Ok(EnsmallenGraph {
                graph: match self.graph.random_subgraph(
                    match match kwargs.get_item("random_state") {
                        None => Ok(None),
                        Some(value) => {
                            if value.get_type().name() == "NoneType" {
                                Ok(None)
                            } else {
                                match value.extract::<usize>() {
                                    Ok(v) => Ok(Some(v)),
                                    Err(_) => Err({
                                        let res =
                                            ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                                &[
                                                    "The value passed for ",
                                                    " cannot be casted from ",
                                                    " to ",
                                                    ".",
                                                ],
                                                &match (
                                                    &"random_state",
                                                    &value.get_type().name(),
                                                    &"usize",
                                                ) {
                                                    (arg0, arg1, arg2) => [
                                                        ::core::fmt::ArgumentV1::new(
                                                            arg0,
                                                            ::core::fmt::Display::fmt,
                                                        ),
                                                        ::core::fmt::ArgumentV1::new(
                                                            arg1,
                                                            ::core::fmt::Display::fmt,
                                                        ),
                                                        ::core::fmt::ArgumentV1::new(
                                                            arg2,
                                                            ::core::fmt::Display::fmt,
                                                        ),
                                                    ],
                                                },
                                            ));
                                        res
                                    }),
                                }
                            }
                        }
                    } {
                        Ok(v) => Ok(v),
                        Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                    }?
                    .unwrap_or(42),
                    nodes_number,
                    match match kwargs.get_item("verbose") {
                        None => Ok(None),
                        Some(value) => {
                            if value.get_type().name() == "NoneType" {
                                Ok(None)
                            } else {
                                match value.extract::<bool>() {
                                    Ok(v) => Ok(Some(v)),
                                    Err(_) => Err({
                                        let res =
                                            ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                                &[
                                                    "The value passed for ",
                                                    " cannot be casted from ",
                                                    " to ",
                                                    ".",
                                                ],
                                                &match (
                                                    &"verbose",
                                                    &value.get_type().name(),
                                                    &"bool",
                                                ) {
                                                    (arg0, arg1, arg2) => [
                                                        ::core::fmt::ArgumentV1::new(
                                                            arg0,
                                                            ::core::fmt::Display::fmt,
                                                        ),
                                                        ::core::fmt::ArgumentV1::new(
                                                            arg1,
                                                            ::core::fmt::Display::fmt,
                                                        ),
                                                        ::core::fmt::ArgumentV1::new(
                                                            arg2,
                                                            ::core::fmt::Display::fmt,
                                                        ),
                                                    ],
                                                },
                                            ));
                                        res
                                    }),
                                }
                            }
                        }
                    } {
                        Ok(v) => Ok(v),
                        Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                    }?
                    .unwrap_or(true),
                ) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?,
            })
        }
        /// Returns training and validation holdouts extracted from current graph.
        ///
        /// The holdouts edges are randomly sampled and have no garanties that any
        /// particular graph structure is maintained.
        ///
        /// Parameters
        /// -----------------------------
        /// train_size: float,
        ///     The rate to reserve for the training.
        /// random_state: int = 42,
        ///     The random_state to make the holdout reproducible.
        /// include_all_edge_types: bool = True,
        ///     Wethever to include all the edges between two nodes.
        ///     This is only relevant in multi-graphs.
        /// edge_types: List[String] = None,
        ///     The edge types to be included in the validation.
        ///     If None (default value) is passed, any edge type can be in the validation set.
        ///     If a non None value is passed, the graph MUST be an heterogeneous graph
        ///     with multiple edge types, otherwise an exception will be raised.
        /// min_number_overlaps: int = None,
        ///     The minimum number of overlapping edges for an edge to be put into the validation set.
        ///     If the value passed is None (default value) any edge can be put into the validation set.
        ///     If a non None value is passed, the graph MUST be a multi-graph, otherwise an exception will be raised.
        /// verbose: bool = True,
        ///     Wethever to show the loading bar.
        ///
        /// Raises
        /// -----------------------------
        /// ValueError,
        ///     If the given train rate is invalid, for example less or equal to 0
        ///     or greater than one.
        /// ValueError,
        ///     If edge types are required but graph is not heterogeneous.
        /// ValueError,
        ///     If given edge types do not exist.
        /// ValueError,
        ///     If min number overlaps is given but graph is not a multigraph.
        ///
        /// Returns
        /// -----------------------------
        /// Tuple containing training and validation graphs.
        fn random_holdout(
            &self,
            train_size: f64,
            py_kwargs: Option<&PyDict>,
        ) -> PyResult<(EnsmallenGraph, EnsmallenGraph)> {
            let py = pyo3::Python::acquire_gil();
            let kwargs = match py_kwargs {
                Some(v) => v,
                None => PyDict::new(py.python()),
            };
            match validate_kwargs(
                kwargs,
                build_walk_parameters_list(&[
                    "random_state",
                    "include_all_edge_types",
                    "edge_types",
                    "min_number_overlaps",
                    "verbose",
                ]),
            ) {
                Ok(v) => Ok(v),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }?;
            let (g1, g2) = match self.graph.random_holdout(
                match match kwargs.get_item("random_state") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<EdgeT>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (&"random_state", &value.get_type().name(), &"EdgeT")
                                        {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                } {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?
                .unwrap_or(42),
                train_size,
                match match kwargs.get_item("include_all_edge_types") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<bool>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (
                                            &"include_all_edge_types",
                                            &value.get_type().name(),
                                            &"bool",
                                        ) {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                } {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?
                .unwrap_or(true),
                match match kwargs.get_item("edge_types") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<Vec<String>>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (
                                            &"edge_types",
                                            &value.get_type().name(),
                                            &"Vec<String>",
                                        ) {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                } {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?,
                match match kwargs.get_item("min_number_overlaps") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<EdgeT>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (
                                            &"min_number_overlaps",
                                            &value.get_type().name(),
                                            &"EdgeT",
                                        ) {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                } {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?,
                match match kwargs.get_item("verbose") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<bool>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (&"verbose", &value.get_type().name(), &"bool") {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                } {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?
                .unwrap_or(true),
            ) {
                Ok(v) => Ok(v),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }?;
            Ok((EnsmallenGraph { graph: g1 }, EnsmallenGraph { graph: g2 }))
        }
        /// Returns Graph with given amount of negative edges as positive edges.
        ///
        /// The graph generated may be used as a testing negatives partition to be
        /// fed into the argument "graph_to_avoid" of the link_prediction or the
        /// binary_skipgrams algorithm.
        ///
        ///
        /// Parameters
        /// -----------------------------
        /// negatives_number: int,
        ///     The number of negative edges to use.
        /// random_state: int = 42,
        ///     The random_state to use to generate the holdout.
        /// seed_graph: EnsmallenGraph = None,
        ///     The (optional) graph whose nodes are used as sources or destinations
        ///     of the generated negative edges.
        /// only_from_same_component: bool = True,
        ///     Wether to sample negative edges only from the same node component.
        ///     This avoids generating topologically impossible negative edges.
        /// verbose: bool = True,
        ///     Wethever to show the loading bar.
        ///     The loading bar will only be visible in console.
        ///
        /// Raises
        /// -----------------------------
        /// TODO: Add the docstring for the raised exceptions.
        ///
        /// Returns
        /// -----------------------------
        /// Graph containing given amount of edges missing in the original graph.
        fn sample_negatives(
            &self,
            negatives_number: EdgeT,
            py_kwargs: Option<&PyDict>,
        ) -> PyResult<EnsmallenGraph> {
            let py = pyo3::Python::acquire_gil();
            let kwargs = match py_kwargs {
                Some(v) => v,
                None => PyDict::new(py.python()),
            };
            match validate_kwargs(
                kwargs,
                build_walk_parameters_list(&[
                    "random_state",
                    "verbose",
                    "seed_graph",
                    "only_from_same_component",
                ]),
            ) {
                Ok(v) => Ok(v),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }?;
            let seed_graph = match match kwargs.get_item("seed_graph") {
                None => Ok(None),
                Some(value) => {
                    if value.get_type().name() == "NoneType" {
                        Ok(None)
                    } else {
                        match value.extract::<EnsmallenGraph>() {
                            Ok(v) => Ok(Some(v)),
                            Err(_) => Err({
                                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                    &[
                                        "The value passed for ",
                                        " cannot be casted from ",
                                        " to ",
                                        ".",
                                    ],
                                    &match (
                                        &"seed_graph",
                                        &value.get_type().name(),
                                        &"EnsmallenGraph",
                                    ) {
                                        (arg0, arg1, arg2) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg2,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ));
                                res
                            }),
                        }
                    }
                }
            } {
                Ok(v) => Ok(v),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }?;
            Ok(EnsmallenGraph {
                graph: match self.graph.sample_negatives(
                    match match kwargs.get_item("random_state") {
                        None => Ok(None),
                        Some(value) => {
                            if value.get_type().name() == "NoneType" {
                                Ok(None)
                            } else {
                                match value.extract::<EdgeT>() {
                                    Ok(v) => Ok(Some(v)),
                                    Err(_) => Err({
                                        let res =
                                            ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                                &[
                                                    "The value passed for ",
                                                    " cannot be casted from ",
                                                    " to ",
                                                    ".",
                                                ],
                                                &match (
                                                    &"random_state",
                                                    &value.get_type().name(),
                                                    &"EdgeT",
                                                ) {
                                                    (arg0, arg1, arg2) => [
                                                        ::core::fmt::ArgumentV1::new(
                                                            arg0,
                                                            ::core::fmt::Display::fmt,
                                                        ),
                                                        ::core::fmt::ArgumentV1::new(
                                                            arg1,
                                                            ::core::fmt::Display::fmt,
                                                        ),
                                                        ::core::fmt::ArgumentV1::new(
                                                            arg2,
                                                            ::core::fmt::Display::fmt,
                                                        ),
                                                    ],
                                                },
                                            ));
                                        res
                                    }),
                                }
                            }
                        }
                    } {
                        Ok(v) => Ok(v),
                        Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                    }?
                    .unwrap_or(42),
                    negatives_number,
                    match &seed_graph {
                        Some(sg) => Some(&sg.graph),
                        None => None,
                    },
                    match match kwargs.get_item("only_from_same_component") {
                        None => Ok(None),
                        Some(value) => {
                            if value.get_type().name() == "NoneType" {
                                Ok(None)
                            } else {
                                match value.extract::<bool>() {
                                    Ok(v) => Ok(Some(v)),
                                    Err(_) => Err({
                                        let res =
                                            ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                                &[
                                                    "The value passed for ",
                                                    " cannot be casted from ",
                                                    " to ",
                                                    ".",
                                                ],
                                                &match (
                                                    &"only_from_same_component",
                                                    &value.get_type().name(),
                                                    &"bool",
                                                ) {
                                                    (arg0, arg1, arg2) => [
                                                        ::core::fmt::ArgumentV1::new(
                                                            arg0,
                                                            ::core::fmt::Display::fmt,
                                                        ),
                                                        ::core::fmt::ArgumentV1::new(
                                                            arg1,
                                                            ::core::fmt::Display::fmt,
                                                        ),
                                                        ::core::fmt::ArgumentV1::new(
                                                            arg2,
                                                            ::core::fmt::Display::fmt,
                                                        ),
                                                    ],
                                                },
                                            ));
                                        res
                                    }),
                                }
                            }
                        }
                    } {
                        Ok(v) => Ok(v),
                        Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                    }?
                    .unwrap_or(true),
                    match match kwargs.get_item("verbose") {
                        None => Ok(None),
                        Some(value) => {
                            if value.get_type().name() == "NoneType" {
                                Ok(None)
                            } else {
                                match value.extract::<bool>() {
                                    Ok(v) => Ok(Some(v)),
                                    Err(_) => Err({
                                        let res =
                                            ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                                &[
                                                    "The value passed for ",
                                                    " cannot be casted from ",
                                                    " to ",
                                                    ".",
                                                ],
                                                &match (
                                                    &"verbose",
                                                    &value.get_type().name(),
                                                    &"bool",
                                                ) {
                                                    (arg0, arg1, arg2) => [
                                                        ::core::fmt::ArgumentV1::new(
                                                            arg0,
                                                            ::core::fmt::Display::fmt,
                                                        ),
                                                        ::core::fmt::ArgumentV1::new(
                                                            arg1,
                                                            ::core::fmt::Display::fmt,
                                                        ),
                                                        ::core::fmt::ArgumentV1::new(
                                                            arg2,
                                                            ::core::fmt::Display::fmt,
                                                        ),
                                                    ],
                                                },
                                            ));
                                        res
                                    }),
                                }
                            }
                        }
                    } {
                        Ok(v) => Ok(v),
                        Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                    }?
                    .unwrap_or(true),
                ) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?,
            })
        }
        /// Returns train and test graph following kfold validation scheme.
        ///
        /// The edges are splitted into k chunks. The k_index-th chunk is used to build
        /// the validation graph, all the other edges create the training graph.
        ///
        /// Parameters
        /// -----------------------------
        /// k: int,
        ///     The number of folds.
        /// k_index: int,
        ///     Which fold to use for the validation.
        /// edge_types: List[str] = None,
        ///     Edge types to be selected when computing the folds
        ///        (All the edge types not listed here will be always be used in the training set).
        /// random_state: int = 42,
        ///     The random_state (seed) to use for the holdout,
        /// verbose: bool = True,
        ///     Wethever to show the loading bar.
        ///
        /// Raises
        /// -----------------------------
        /// TODO: Add the docstring for the raised exceptions.
        ///
        /// Returns
        /// -----------------------------
        /// train, test graph.
        fn kfold(
            &self,
            k: EdgeT,
            k_index: u64,
            py_kwargs: Option<&PyDict>,
        ) -> PyResult<(EnsmallenGraph, EnsmallenGraph)> {
            let py = pyo3::Python::acquire_gil();
            let kwargs = match py_kwargs {
                Some(v) => v,
                None => PyDict::new(py.python()),
            };
            match validate_kwargs(
                kwargs,
                build_walk_parameters_list(&["edge_types", "random_state", "verbose"]),
            ) {
                Ok(v) => Ok(v),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }?;
            let (train, test) = match self.graph.kfold(
                k,
                k_index,
                match match kwargs.get_item("edge_types") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<Vec<String>>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (
                                            &"edge_types",
                                            &value.get_type().name(),
                                            &"Vec<String>",
                                        ) {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                } {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?,
                match match kwargs.get_item("random_state") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<u64>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (&"random_state", &value.get_type().name(), &"u64") {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                } {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?
                .unwrap_or(42),
                match match kwargs.get_item("verbose") {
                    None => Ok(None),
                    Some(value) => {
                        if value.get_type().name() == "NoneType" {
                            Ok(None)
                        } else {
                            match value.extract::<bool>() {
                                Ok(v) => Ok(Some(v)),
                                Err(_) => Err({
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &[
                                            "The value passed for ",
                                            " cannot be casted from ",
                                            " to ",
                                            ".",
                                        ],
                                        &match (&"verbose", &value.get_type().name(), &"bool") {
                                            (arg0, arg1, arg2) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ));
                                    res
                                }),
                            }
                        }
                    }
                } {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?
                .unwrap_or(true),
            ) {
                Ok(v) => Ok(v),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }?;
            Ok((
                EnsmallenGraph { graph: train },
                EnsmallenGraph { graph: test },
            ))
        }
    }
    #[used]
    #[allow(non_upper_case_globals)]
    #[link_section = ".init_array"]
    #[allow(non_upper_case_globals)]
    static __init2695162131542456377: extern "C" fn() = {
        #[link_section = ".text.startup"]
        extern "C" fn __init2695162131542456377() {
            pyo3::inventory::submit({
                {
                    type Inventory =
                        <EnsmallenGraph as pyo3::class::methods::HasMethodsInventory>::Methods;
                    <Inventory as pyo3::class::methods::PyMethodsInventory>::new(<[_]>::into_vec(
                        box [
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                    _kwargs: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.connected_holdout()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    let _args = _py
                                                        .from_borrowed_ptr::<pyo3::types::PyTuple>(
                                                            _args,
                                                        );
                                                    let _kwargs: Option<&pyo3::types::PyDict> =
                                                        _py.from_borrowed_ptr_or_opt(_kwargs);
                                                    pyo3::callback::convert(_py, {
                                                        const PARAMS : & 'static [pyo3 :: derive_utils :: ParamDescription] = & [pyo3 :: derive_utils :: ParamDescription { name : "train_size" , is_optional : false , kw_only : false , }] ;
                                                        let mut output = [None; 1usize];
                                                        let mut _args = _args;
                                                        let mut _kwargs = _kwargs;
                                                        let (_args, _kwargs) =
                                                            pyo3::derive_utils::parse_fn_args(
                                                                Some(_LOCATION),
                                                                PARAMS,
                                                                _args,
                                                                _kwargs,
                                                                false,
                                                                true,
                                                                &mut output,
                                                            )?;
                                                        let arg0 = match output [0usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        let arg1 = _kwargs;
                                                        EnsmallenGraph::connected_holdout(
                                                            _slf, arg0, arg1,
                                                        )
                                                    })
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction_with_keywords ("connected_holdout\u{0}" , __wrap , 0 , "connected_holdout($self, train_size, *, random_state, edge_types, include_all_edge_types, verbose)\n--\n\nReturns training and validation holdouts extracted from current graph.\n\nThe holdouts is generated in such a way that the training set remains\nconnected if the starting graph is connected by using a spanning tree.\n\nParameters\n-----------------------------\ntrain_size: float,\n    The rate of edges to reserve for the training.\nrandom_state: int = 42,\n    The random_state to use to generate the holdout.\nedge_types: List[str] = None,\n    List of names of the edge types to put into the validation.\ninclude_all_edge_types: bool = False,\n    Wethever to include all the edges between two nodes.\n    This is only relevant in multi-graphs.\nverbose: bool = True,\n    Wethever to show the loading bar.\n\nRaises\n-----------------------------\nValueError,\n    If the given train rate is not a real number between 0 and 1.\n\nReturns\n-----------------------------\nTuple containing training and validation graphs.\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                    _kwargs: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.random_subgraph()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    let _args = _py
                                                        .from_borrowed_ptr::<pyo3::types::PyTuple>(
                                                            _args,
                                                        );
                                                    let _kwargs: Option<&pyo3::types::PyDict> =
                                                        _py.from_borrowed_ptr_or_opt(_kwargs);
                                                    pyo3::callback::convert(_py, {
                                                        const PARAMS : & 'static [pyo3 :: derive_utils :: ParamDescription] = & [pyo3 :: derive_utils :: ParamDescription { name : "nodes_number" , is_optional : false , kw_only : false , }] ;
                                                        let mut output = [None; 1usize];
                                                        let mut _args = _args;
                                                        let mut _kwargs = _kwargs;
                                                        let (_args, _kwargs) =
                                                            pyo3::derive_utils::parse_fn_args(
                                                                Some(_LOCATION),
                                                                PARAMS,
                                                                _args,
                                                                _kwargs,
                                                                false,
                                                                true,
                                                                &mut output,
                                                            )?;
                                                        let arg0 = match output [0usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        let arg1 = _kwargs;
                                                        EnsmallenGraph::random_subgraph(
                                                            _slf, arg0, arg1,
                                                        )
                                                    })
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction_with_keywords ("random_subgraph\u{0}" , __wrap , 0 , "random_subgraph($self, nodes_number, *, random_state, verbose)\n--\n\nReturns partial subgraph.\n\nThis method creates a subset of the graph starting from a random node\nsampled using given random_state and includes all neighbouring nodes until\nthe required number of nodes is reached. All the edges connecting any\nof the selected nodes are then inserted into this graph.\n\nParameters\n-----------------------------\nnodes_number: int,\n    The number of edges to insert in the partial graph.\nrandom_state: int = 42,\n    The random_state to use to generate the partial graph.\nverbose: bool = True,\n    Wethever to show the loading bar.\n\nRaises\n-----------------------------\nTODO: Add the docstring for the raised exceptions.\n\nReturns\n-----------------------------\nPartial graph.\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                    _kwargs: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.random_holdout()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    let _args = _py
                                                        .from_borrowed_ptr::<pyo3::types::PyTuple>(
                                                            _args,
                                                        );
                                                    let _kwargs: Option<&pyo3::types::PyDict> =
                                                        _py.from_borrowed_ptr_or_opt(_kwargs);
                                                    pyo3::callback::convert(_py, {
                                                        const PARAMS : & 'static [pyo3 :: derive_utils :: ParamDescription] = & [pyo3 :: derive_utils :: ParamDescription { name : "train_size" , is_optional : false , kw_only : false , }] ;
                                                        let mut output = [None; 1usize];
                                                        let mut _args = _args;
                                                        let mut _kwargs = _kwargs;
                                                        let (_args, _kwargs) =
                                                            pyo3::derive_utils::parse_fn_args(
                                                                Some(_LOCATION),
                                                                PARAMS,
                                                                _args,
                                                                _kwargs,
                                                                false,
                                                                true,
                                                                &mut output,
                                                            )?;
                                                        let arg0 = match output [0usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        let arg1 = _kwargs;
                                                        EnsmallenGraph::random_holdout(
                                                            _slf, arg0, arg1,
                                                        )
                                                    })
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction_with_keywords ("random_holdout\u{0}" , __wrap , 0 , "random_holdout($self, train_size, *, random_state, include_all_edge_types, edge_types, min_number_overlaps, verbose)\n--\n\nReturns training and validation holdouts extracted from current graph.\n\nThe holdouts edges are randomly sampled and have no garanties that any\nparticular graph structure is maintained.\n\nParameters\n-----------------------------\ntrain_size: float,\n    The rate to reserve for the training.\nrandom_state: int = 42,\n    The random_state to make the holdout reproducible.\ninclude_all_edge_types: bool = True,\n    Wethever to include all the edges between two nodes.\n    This is only relevant in multi-graphs.\nedge_types: List[String] = None,\n    The edge types to be included in the validation.\n    If None (default value) is passed, any edge type can be in the validation set.\n    If a non None value is passed, the graph MUST be an heterogeneous graph\n    with multiple edge types, otherwise an exception will be raised.\nmin_number_overlaps: int = None,\n    The minimum number of overlapping edges for an edge to be put into the validation set.\n    If the value passed is None (default value) any edge can be put into the validation set.\n    If a non None value is passed, the graph MUST be a multi-graph, otherwise an exception will be raised.\nverbose: bool = True,\n    Wethever to show the loading bar.\n\nRaises\n-----------------------------\nValueError,\n    If the given train rate is invalid, for example less or equal to 0\n    or greater than one.\nValueError,\n    If edge types are required but graph is not heterogeneous.\nValueError,\n    If given edge types do not exist.\nValueError,\n    If min number overlaps is given but graph is not a multigraph.\n\nReturns\n-----------------------------\nTuple containing training and validation graphs.\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                    _kwargs: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str =
                                        "EnsmallenGraph.sample_negatives()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    let _args = _py
                                                        .from_borrowed_ptr::<pyo3::types::PyTuple>(
                                                            _args,
                                                        );
                                                    let _kwargs: Option<&pyo3::types::PyDict> =
                                                        _py.from_borrowed_ptr_or_opt(_kwargs);
                                                    pyo3::callback::convert(_py, {
                                                        const PARAMS : & 'static [pyo3 :: derive_utils :: ParamDescription] = & [pyo3 :: derive_utils :: ParamDescription { name : "negatives_number" , is_optional : false , kw_only : false , }] ;
                                                        let mut output = [None; 1usize];
                                                        let mut _args = _args;
                                                        let mut _kwargs = _kwargs;
                                                        let (_args, _kwargs) =
                                                            pyo3::derive_utils::parse_fn_args(
                                                                Some(_LOCATION),
                                                                PARAMS,
                                                                _args,
                                                                _kwargs,
                                                                false,
                                                                true,
                                                                &mut output,
                                                            )?;
                                                        let arg0 = match output [0usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        let arg1 = _kwargs;
                                                        EnsmallenGraph::sample_negatives(
                                                            _slf, arg0, arg1,
                                                        )
                                                    })
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction_with_keywords ("sample_negatives\u{0}" , __wrap , 0 , "sample_negatives($self, negatives_number, *, random_state, seed_graph, only_from_same_component, verbose)\n--\n\nReturns Graph with given amount of negative edges as positive edges.\n\nThe graph generated may be used as a testing negatives partition to be\nfed into the argument \"graph_to_avoid\" of the link_prediction or the\nbinary_skipgrams algorithm.\n\n\nParameters\n-----------------------------\nnegatives_number: int,\n    The number of negative edges to use.\nrandom_state: int = 42,\n    The random_state to use to generate the holdout.\nseed_graph: EnsmallenGraph = None,\n    The (optional) graph whose nodes are used as sources or destinations\n    of the generated negative edges.\nonly_from_same_component: bool = True,\n    Wether to sample negative edges only from the same node component.\n    This avoids generating topologically impossible negative edges.\nverbose: bool = True,\n    Wethever to show the loading bar.\n    The loading bar will only be visible in console.\n\nRaises\n-----------------------------\nTODO: Add the docstring for the raised exceptions.\n\nReturns\n-----------------------------\nGraph containing given amount of edges missing in the original graph.\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                    _kwargs: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str = "EnsmallenGraph.kfold()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    let _args = _py
                                                        .from_borrowed_ptr::<pyo3::types::PyTuple>(
                                                            _args,
                                                        );
                                                    let _kwargs: Option<&pyo3::types::PyDict> =
                                                        _py.from_borrowed_ptr_or_opt(_kwargs);
                                                    pyo3::callback::convert(_py, {
                                                        const PARAMS : & 'static [pyo3 :: derive_utils :: ParamDescription] = & [pyo3 :: derive_utils :: ParamDescription { name : "k" , is_optional : false , kw_only : false , } , pyo3 :: derive_utils :: ParamDescription { name : "k_index" , is_optional : false , kw_only : false , }] ;
                                                        let mut output = [None; 2usize];
                                                        let mut _args = _args;
                                                        let mut _kwargs = _kwargs;
                                                        let (_args, _kwargs) =
                                                            pyo3::derive_utils::parse_fn_args(
                                                                Some(_LOCATION),
                                                                PARAMS,
                                                                _args,
                                                                _kwargs,
                                                                false,
                                                                true,
                                                                &mut output,
                                                            )?;
                                                        let arg0 = match output [0usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        let arg1 = match output [1usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        let arg2 = _kwargs;
                                                        EnsmallenGraph::kfold(
                                                            _slf, arg0, arg1, arg2,
                                                        )
                                                    })
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction_with_keywords ("kfold\u{0}" , __wrap , 0 , "kfold($self, k, k_index, *, edge_types, random_state, verbose)\n--\n\nReturns train and test graph following kfold validation scheme.\n\nThe edges are splitted into k chunks. The k_index-th chunk is used to build\nthe validation graph, all the other edges create the training graph.\n\nParameters\n-----------------------------\nk: int,\n    The number of folds.\nk_index: int,\n    Which fold to use for the validation.\nedge_types: List[str] = None,\n    Edge types to be selected when computing the folds \n       (All the edge types not listed here will be always be used in the training set).\nrandom_state: int = 42,\n    The random_state (seed) to use for the holdout,\nverbose: bool = True,\n    Wethever to show the loading bar.\n\nRaises\n-----------------------------\nTODO: Add the docstring for the raised exceptions.\n\nReturns\n-----------------------------\ntrain, test graph.\u{0}")
                            }),
                        ],
                    ))
                }
            });
        }
        __init2695162131542456377
    };
}
mod operators {
    use super::*;
    use pyo3::class::basic::PyObjectProtocol;
    use pyo3::class::number::PyNumberProtocol;
    impl<'p> PyNumberProtocol<'p> for EnsmallenGraph {
        fn __or__(
            lhs: <EnsmallenGraph as pyo3::class::number::PyNumberOrProtocol<'p>>::Left,
            rhs: <EnsmallenGraph as pyo3::class::number::PyNumberOrProtocol<'p>>::Right,
        ) -> <EnsmallenGraph as pyo3::class::number::PyNumberOrProtocol<'p>>::Result {
            Ok(EnsmallenGraph {
                graph: match &lhs.graph | &rhs.graph {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?,
            })
        }
        fn __sub__(
            lhs: <EnsmallenGraph as pyo3::class::number::PyNumberSubProtocol<'p>>::Left,
            rhs: <EnsmallenGraph as pyo3::class::number::PyNumberSubProtocol<'p>>::Right,
        ) -> <EnsmallenGraph as pyo3::class::number::PyNumberSubProtocol<'p>>::Result {
            Ok(EnsmallenGraph {
                graph: match &lhs.graph - &rhs.graph {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?,
            })
        }
        fn __and__(
            lhs: <EnsmallenGraph as pyo3::class::number::PyNumberAndProtocol<'p>>::Left,
            rhs: <EnsmallenGraph as pyo3::class::number::PyNumberAndProtocol<'p>>::Right,
        ) -> <EnsmallenGraph as pyo3::class::number::PyNumberAndProtocol<'p>>::Result {
            Ok(EnsmallenGraph {
                graph: match &lhs.graph & &rhs.graph {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?,
            })
        }
        fn __xor__(
            lhs: <EnsmallenGraph as pyo3::class::number::PyNumberXorProtocol<'p>>::Left,
            rhs: <EnsmallenGraph as pyo3::class::number::PyNumberXorProtocol<'p>>::Right,
        ) -> <EnsmallenGraph as pyo3::class::number::PyNumberXorProtocol<'p>>::Result {
            Ok(EnsmallenGraph {
                graph: match &lhs.graph ^ &rhs.graph {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
                }?,
            })
        }
    }
    impl<'p> pyo3::class::number::PyNumberOrProtocol<'p> for EnsmallenGraph {
        type Left = EnsmallenGraph;
        type Right = EnsmallenGraph;
        type Result = PyResult<EnsmallenGraph>;
    }
    impl<'p> pyo3::class::number::PyNumberSubProtocol<'p> for EnsmallenGraph {
        type Left = EnsmallenGraph;
        type Right = EnsmallenGraph;
        type Result = PyResult<EnsmallenGraph>;
    }
    impl<'p> pyo3::class::number::PyNumberAndProtocol<'p> for EnsmallenGraph {
        type Left = EnsmallenGraph;
        type Right = EnsmallenGraph;
        type Result = PyResult<EnsmallenGraph>;
    }
    impl<'p> pyo3::class::number::PyNumberXorProtocol<'p> for EnsmallenGraph {
        type Left = EnsmallenGraph;
        type Right = EnsmallenGraph;
        type Result = PyResult<EnsmallenGraph>;
    }
    #[used]
    #[allow(non_upper_case_globals)]
    #[link_section = ".init_array"]
    #[allow(non_snake_case)]
    static __init_Number_8991739877547249111: extern "C" fn() = {
        #[link_section = ".text.startup"]
        extern "C" fn __init_Number_8991739877547249111() {
            let mut table = pyo3::ffi::PyNumberMethods::default();
            table.set_sub::<EnsmallenGraph>();
            table.set_and::<EnsmallenGraph>();
            table.set_xor::<EnsmallenGraph>();
            table.set_or::<EnsmallenGraph>();
            <EnsmallenGraph as pyo3::class::proto_methods::HasProtoRegistry>::registry()
                .set_number_methods(table);
        }
        __init_Number_8991739877547249111
    };
    impl<'p> PyObjectProtocol<'p> for EnsmallenGraph {
        fn __repr__(
            &'p self,
        ) -> <EnsmallenGraph as pyo3::class::basic::PyObjectReprProtocol<'p>>::Result {
            Ok(self.graph.textual_report())
        }
    }
    impl<'p> pyo3::class::basic::PyObjectReprProtocol<'p> for EnsmallenGraph {
        type Result = PyResult<String>;
    }
    #[used]
    #[allow(non_upper_case_globals)]
    #[link_section = ".init_array"]
    #[allow(non_snake_case)]
    static __init_Object_8991739877547249111: extern "C" fn() = {
        #[link_section = ".text.startup"]
        extern "C" fn __init_Object_8991739877547249111() {
            let mut table = pyo3::class::basic::PyObjectMethods::default();
            table.set_repr::<EnsmallenGraph>();
            <EnsmallenGraph as pyo3::class::proto_methods::HasProtoRegistry>::registry()
                .set_basic_methods(table);
        }
        __init_Object_8991739877547249111
    };
    impl EnsmallenGraph {
        /// Return true if given graph has any edge overlapping with current graph.
        ///
        /// Parameters
        /// ----------------------------
        /// graph: EnsmallenGraph,
        ///     The graph to check against.
        ///
        /// Returns
        /// ----------------------------
        /// Boolean representing if any overlapping edge was found.
        pub fn overlaps(&self, graph: &EnsmallenGraph) -> PyResult<bool> {
            match self.graph.overlaps(&graph.graph) {
                Ok(v) => Ok(v),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }
        }
        /// Return true if given graph edges are all contained within current graph.
        ///
        /// Parameters
        /// ----------------------------
        /// graph: EnsmallenGraph,
        ///     The graph to check against.
        ///
        /// Returns
        /// ----------------------------
        /// Boolean representing if graph contains completely the othe graph.
        pub fn contains(&self, graph: &EnsmallenGraph) -> PyResult<bool> {
            match self.graph.contains(&graph.graph) {
                Ok(v) => Ok(v),
                Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
            }
        }
    }
    #[used]
    #[allow(non_upper_case_globals)]
    #[link_section = ".init_array"]
    #[allow(non_upper_case_globals)]
    static __init8039546587418952619: extern "C" fn() = {
        #[link_section = ".text.startup"]
        extern "C" fn __init8039546587418952619() {
            pyo3::inventory::submit({
                {
                    type Inventory =
                        <EnsmallenGraph as pyo3::class::methods::HasMethodsInventory>::Methods;
                    <Inventory as pyo3::class::methods::PyMethodsInventory>::new(<[_]>::into_vec(
                        box [
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                    _kwargs: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str = "EnsmallenGraph.overlaps()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    let _args = _py
                                                        .from_borrowed_ptr::<pyo3::types::PyTuple>(
                                                            _args,
                                                        );
                                                    let _kwargs: Option<&pyo3::types::PyDict> =
                                                        _py.from_borrowed_ptr_or_opt(_kwargs);
                                                    pyo3::callback::convert(_py, {
                                                        const PARAMS : & 'static [pyo3 :: derive_utils :: ParamDescription] = & [pyo3 :: derive_utils :: ParamDescription { name : "graph" , is_optional : false , kw_only : false , }] ;
                                                        let mut output = [None; 1usize];
                                                        let mut _args = _args;
                                                        let mut _kwargs = _kwargs;
                                                        let (_args, _kwargs) =
                                                            pyo3::derive_utils::parse_fn_args(
                                                                Some(_LOCATION),
                                                                PARAMS,
                                                                _args,
                                                                _kwargs,
                                                                false,
                                                                false,
                                                                &mut output,
                                                            )?;
                                                        let _tmp : < & EnsmallenGraph as pyo3 :: derive_utils :: ExtractExt > :: Target = match output [0usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        let arg0 = &*_tmp;
                                                        EnsmallenGraph::overlaps(_slf, arg0)
                                                    })
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction_with_keywords ("overlaps\u{0}" , __wrap , 0 , "Return true if given graph has any edge overlapping with current graph.\n\nParameters\n----------------------------\ngraph: EnsmallenGraph,\n    The graph to check against.\n\nReturns\n----------------------------\nBoolean representing if any overlapping edge was found.\u{0}")
                            }),
                            pyo3::class::PyMethodDefType::Method({
                                unsafe extern "C" fn __wrap(
                                    _slf: *mut pyo3::ffi::PyObject,
                                    _args: *mut pyo3::ffi::PyObject,
                                    _kwargs: *mut pyo3::ffi::PyObject,
                                ) -> *mut pyo3::ffi::PyObject {
                                    const _LOCATION: &'static str = "EnsmallenGraph.contains()";
                                    {
                                        let pool = ::pyo3::GILPool::new();
                                        let unwind_safe_py =
                                            std::panic::AssertUnwindSafe(pool.python());
                                        let result = match std::panic::catch_unwind(
                                            move || -> ::pyo3::PyResult<_> {
                                                let _py = *unwind_safe_py;
                                                {
                                                    let _cell = _py . from_borrowed_ptr :: < pyo3 :: PyCell < EnsmallenGraph > > (_slf) ;
                                                    let _ref = _cell.try_borrow()?;
                                                    let _slf = &_ref;
                                                    let _args = _py
                                                        .from_borrowed_ptr::<pyo3::types::PyTuple>(
                                                            _args,
                                                        );
                                                    let _kwargs: Option<&pyo3::types::PyDict> =
                                                        _py.from_borrowed_ptr_or_opt(_kwargs);
                                                    pyo3::callback::convert(_py, {
                                                        const PARAMS : & 'static [pyo3 :: derive_utils :: ParamDescription] = & [pyo3 :: derive_utils :: ParamDescription { name : "graph" , is_optional : false , kw_only : false , }] ;
                                                        let mut output = [None; 1usize];
                                                        let mut _args = _args;
                                                        let mut _kwargs = _kwargs;
                                                        let (_args, _kwargs) =
                                                            pyo3::derive_utils::parse_fn_args(
                                                                Some(_LOCATION),
                                                                PARAMS,
                                                                _args,
                                                                _kwargs,
                                                                false,
                                                                false,
                                                                &mut output,
                                                            )?;
                                                        let _tmp : < & EnsmallenGraph as pyo3 :: derive_utils :: ExtractExt > :: Target = match output [0usize] { Some (_obj) => _obj . extract () ? , None => { :: std :: rt :: begin_panic ("Failed to extract required method argument") } } ;
                                                        let arg0 = &*_tmp;
                                                        EnsmallenGraph::contains(_slf, arg0)
                                                    })
                                                }
                                            },
                                        ) {
                                            Ok(result) => result,
                                            Err(e) => {
                                                if let Some(string) = e.downcast_ref::<String>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        string.clone(),
                                                    )))
                                                } else if let Some(s) = e.downcast_ref::<&str>() {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        s.to_string(),
                                                    )))
                                                } else {
                                                    Err(::pyo3::panic::PanicException::new_err((
                                                        "panic from Rust code",
                                                    )))
                                                }
                                            }
                                        };
                                        result.unwrap_or_else(|e| {
                                            e.restore(pool.python());
                                            ::pyo3::callback::callback_error()
                                        })
                                    }
                                }
                                pyo3 :: class :: PyMethodDef :: cfunction_with_keywords ("contains\u{0}" , __wrap , 0 , "Return true if given graph edges are all contained within current graph.\n\nParameters\n----------------------------\ngraph: EnsmallenGraph,\n    The graph to check against.\n\nReturns\n----------------------------\nBoolean representing if graph contains completely the othe graph.\u{0}")
                            }),
                        ],
                    ))
                }
            });
        }
        __init8039546587418952619
    };
}
fn ensmallen_graph(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<EnsmallenGraph>()?;
    m . add_wrapped ({ { # [allow (dead_code)] enum ProcMacroHack { Value = ("& | py | unsafe\n{ pyo3 :: PyObject :: from_owned_ptr(py, [< PyInit_ preprocessing >] ()) }" , 0) . 1 , } { & (| py | unsafe { pyo3 :: PyObject :: from_owned_ptr (py , PyInit_preprocessing ()) }) } } }) ? ;
    env_logger::init();
    Ok(())
}
#[no_mangle]
#[allow(non_snake_case)]
/// This autogenerated function is called by the python interpreter when importing
/// the module.
pub unsafe extern "C" fn PyInit_ensmallen_graph() -> *mut pyo3::ffi::PyObject {
    use pyo3::derive_utils::ModuleDef;
    const NAME: &'static str = "ensmallen_graph\u{0}";
    static MODULE_DEF: ModuleDef = unsafe { ModuleDef::new(NAME) };
    {
        {
            let pool = ::pyo3::GILPool::new();
            let unwind_safe_py = std::panic::AssertUnwindSafe(pool.python());
            let result = match std::panic::catch_unwind(move || -> ::pyo3::PyResult<_> {
                let _py = *unwind_safe_py;
                ::pyo3::callback::convert(_py, { MODULE_DEF.make_module("", ensmallen_graph) })
            }) {
                Ok(result) => result,
                Err(e) => {
                    if let Some(string) = e.downcast_ref::<String>() {
                        Err(::pyo3::panic::PanicException::new_err((string.clone(),)))
                    } else if let Some(s) = e.downcast_ref::<&str>() {
                        Err(::pyo3::panic::PanicException::new_err((s.to_string(),)))
                    } else {
                        Err(::pyo3::panic::PanicException::new_err((
                            "panic from Rust code",
                        )))
                    }
                }
            };
            result.unwrap_or_else(|e| {
                e.restore(pool.python());
                ::pyo3::callback::callback_error()
            })
        }
    }
}
