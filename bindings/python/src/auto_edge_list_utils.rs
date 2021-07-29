#[allow(unused_imports)]
use crate::types::EnsmallenGraph;
#[allow(unused_imports)]
use graph::{DumpGraph, Graph};
#[allow(unused_imports)]
use numpy::{PyArray, PyArray1, PyArray2};
#[allow(unused_imports)]
use pyo3::prelude::*;
#[allow(unused_imports)]
use pyo3::types::PyDict;
#[allow(unused_imports)]
use shared::*;
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet};
#[allow(unused_imports)]
use tags::*;

#[pyfunction]
#[automatically_generated_binding]
#[text_signature = "(original_edge_path, original_edge_list_separator, original_edge_list_header, original_sources_column, original_sources_column_number, original_destinations_column, original_destinations_column_number, original_edge_list_edge_type_column, original_edge_list_edge_type_column_number, original_weights_column, original_weights_column_number, target_edge_path, target_edge_list_separator, target_edge_list_header, target_sources_column_number, target_sources_column, target_destinations_column_number, target_destinations_column, target_edge_list_edge_type_column, target_edge_list_edge_type_column_number, target_weights_column, target_weights_column_number, comment_symbol, default_edge_type, default_weight, max_rows_number, rows_to_skip, edges_number, skip_edge_types_if_unavailable, skip_weights_if_unavailable, verbose, name)"]
/// Create a new undirected edge list from a given directed one by duplicating the undirected edges.
///
/// Parameters
/// ----------
/// original_edge_path: str,
///     The path from where to load the original edge list.
/// original_edge_list_separator: Optional[str],
///     Separator to use for the original edge list.
/// original_edge_list_header: Optional[bool],
///     Whether the original edge list has an header.
/// original_sources_column: Optional[str],
///     The column name to use to load the sources in the original edges list.
/// original_sources_column_number: Optional[int],
///     The column number to use to load the sources in the original edges list.
/// original_destinations_column: Optional[str],
///     The column name to use to load the destinations in the original edges list.
/// original_destinations_column_number: Optional[int],
///     The column number to use to load the destinations in the original edges list.
/// original_edge_list_edge_type_column: Optional[str],
///     The column name to use for the edge types in the original edges list.
/// original_edge_list_edge_type_column_number: Optional[int],
///     The column number to use for the edge types in the original edges list.
/// original_weights_column: Optional[str],
///     The column name to use for the weights in the original edges list.
/// original_weights_column_number: Optional[int],
///     The column number to use for the weights in the original edges list.
/// target_edge_path: str,
///     The path from where to load the target edge list. This must be different from the original edge list path.
/// target_edge_list_separator: Optional[str],
///     Separator to use for the target edge list. If None, the one provided from the original edge list will be used.
/// target_edge_list_header: Optional[bool],
///     Whether the target edge list has an header. If None, the one provided from the original edge list will be used.
/// target_sources_column: Optional[str],
///     The column name to use to load the sources in the target edges list. If None, the one provided from the original edge list will be used.
/// target_sources_column_number: Optional[int],
///     The column number to use to load the sources in the target edges list. If None, the one provided from the original edge list will be used.
/// target_destinations_column: Optional[str],
///     The column name to use to load the destinations in the target edges list. If None, the one provided from the original edge list will be used.
/// target_destinations_column_number: Optional[int],
///     The column number to use to load the destinations in the target edges list. If None, the one provided from the original edge list will be used.
/// target_edge_list_edge_type_column: Optional[str],
///     The column name to use for the edge types in the target edges list. If None, the one provided from the original edge list will be used.
/// target_edge_list_edge_type_column_number: Optional[int],
///     The column number to use for the edge types in the target edges list. If None, the one provided from the original edge list will be used.
/// target_weights_column: Optional[str],
///     The column name to use for the weights in the target edges list. If None, the one provided from the original edge list will be used.
/// target_weights_column_number: Optional[int],
///     The column number to use for the weights in the target edges list. If None, the one provided from the original edge list will be used.
/// comment_symbol: Optional[str],
///     The comment symbol to use within the original edge list.
/// default_edge_type: Optional[str],
///     The default edge type to use within the original edge list.
/// default_weight: Optional[float],
///     The default weight to use within the original edge list.
/// max_rows_number: Optional[int],
///     The amount of rows to load from the original edge list.
/// rows_to_skip: Optional[int],
///     The amount of rows to skip from the original edge list.
/// edges_number: Optional[int],
///     The expected number of edges. It will be used for the loading bar.
/// skip_edge_types_if_unavailable: Optional[bool],
///     Whether to automatically skip the edge types if they are not available.
/// skip_weights_if_unavailable: Optional[bool],
///     Whether to automatically skip the weights if they are not available.
/// verbose: Optional[bool],
///     Whether to show the loading bar while processing the file.
/// name: Optional[str],
///     The name of the graph to display in the loading bar.
///
///
/// Raises
/// -------
/// ValueError
///     If there are problems with opening the original or target file.
/// ValueError
///     If the original and target paths are identical.
///
pub fn convert_directed_edge_list_to_undirected(
    original_edge_path: &str,
    original_edge_list_separator: Option<String>,
    original_edge_list_header: Option<bool>,
    original_sources_column: Option<String>,
    original_sources_column_number: Option<usize>,
    original_destinations_column: Option<String>,
    original_destinations_column_number: Option<usize>,
    original_edge_list_edge_type_column: Option<String>,
    original_edge_list_edge_type_column_number: Option<usize>,
    original_weights_column: Option<String>,
    original_weights_column_number: Option<usize>,
    target_edge_path: &str,
    target_edge_list_separator: Option<String>,
    target_edge_list_header: Option<bool>,
    target_sources_column_number: Option<usize>,
    target_sources_column: Option<String>,
    target_destinations_column_number: Option<usize>,
    target_destinations_column: Option<String>,
    target_edge_list_edge_type_column: Option<String>,
    target_edge_list_edge_type_column_number: Option<usize>,
    target_weights_column: Option<String>,
    target_weights_column_number: Option<usize>,
    comment_symbol: Option<String>,
    default_edge_type: Option<String>,
    default_weight: Option<WeightT>,
    max_rows_number: Option<usize>,
    rows_to_skip: Option<usize>,
    edges_number: Option<usize>,
    skip_edge_types_if_unavailable: Option<bool>,
    skip_weights_if_unavailable: Option<bool>,
    verbose: Option<bool>,
    name: Option<String>,
) -> PyResult<EdgeT> {
    pe!(edge_list_utils::convert_directed_edge_list_to_undirected(
        original_edge_path,
        original_edge_list_separator,
        original_edge_list_header,
        original_sources_column,
        original_sources_column_number,
        original_destinations_column,
        original_destinations_column_number,
        original_edge_list_edge_type_column,
        original_edge_list_edge_type_column_number,
        original_weights_column,
        original_weights_column_number,
        target_edge_path,
        target_edge_list_separator,
        target_edge_list_header,
        target_sources_column_number,
        target_sources_column,
        target_destinations_column_number,
        target_destinations_column,
        target_edge_list_edge_type_column,
        target_edge_list_edge_type_column_number,
        target_weights_column,
        target_weights_column_number,
        comment_symbol,
        default_edge_type,
        default_weight,
        max_rows_number,
        rows_to_skip,
        edges_number,
        skip_edge_types_if_unavailable,
        skip_weights_if_unavailable,
        verbose,
        name
    ))
}

#[pyfunction]
#[automatically_generated_binding]
#[text_signature = "(original_csv_path, original_csv_separator, original_csv_header, target_csv_path, target_csv_separator, target_csv_header, target_csv_ids_column, target_csv_ids_column_number, comment_symbol, max_rows_number, rows_to_skip, lines_number, verbose)"]
/// Create a new CSV with the lines number added to it.
///
/// Parameters
/// ----------
/// original_csv_path: str,
///     The path from where to load the original CSV.
/// original_csv_separator: Optional[str],
///     Separator to use for the original CSV.
/// original_csv_header: Optional[bool],
///     Whether the original CSV has an header.
/// target_csv_path: str,
///     The path from where to load the target CSV. This cannot be the same as the original CSV.
/// target_csv_separator: Optional[str],
///     Separator to use for the target CSV. If None, the one provided from the original CSV will be used.
/// target_csv_header: Optional[bool],
///     Whether the target CSV has an header. If None, the one provided from the original CSV will be used.
/// target_csv_ids_column: Optional[str],
///     The column name to use for the ids in the target list.
/// target_csv_ids_column_number: Optional[int],
///     The column number to use for the ids in the target list.
/// comment_symbol: Optional[str],
///     The comment symbol to use within the original CSV.
/// max_rows_number: Optional[int],
///     The amount of rows to load from the original CSV.
/// rows_to_skip: Optional[int],
///     The amount of rows to skip from the original CSV.
/// verbose: Optional[bool],
///     Whether to show the loading bar while processing the file.
///
///
/// Raises
/// -------
/// ValueError
///     If there are problems with opening the original or target file.
/// ValueError
///     If the original and target paths are identical.
///
pub fn add_numeric_id_to_csv(
    original_csv_path: &str,
    original_csv_separator: Option<String>,
    original_csv_header: Option<bool>,
    target_csv_path: &str,
    target_csv_separator: Option<String>,
    target_csv_header: Option<bool>,
    target_csv_ids_column: Option<String>,
    target_csv_ids_column_number: Option<usize>,
    comment_symbol: Option<String>,
    max_rows_number: Option<usize>,
    rows_to_skip: Option<usize>,
    lines_number: Option<usize>,
    verbose: Option<bool>,
) -> PyResult<usize> {
    pe!(edge_list_utils::add_numeric_id_to_csv(
        original_csv_path,
        original_csv_separator,
        original_csv_header,
        target_csv_path,
        target_csv_separator,
        target_csv_header,
        target_csv_ids_column,
        target_csv_ids_column_number,
        comment_symbol,
        max_rows_number,
        rows_to_skip,
        lines_number,
        verbose
    ))
}

#[pyfunction]
#[automatically_generated_binding]
#[text_signature = "(path, separator, header, sources_column, sources_column_number, destinations_column, destinations_column_number, comment_symbol, max_rows_number, rows_to_skip, edges_number, load_edge_list_in_parallel, verbose, name)"]
/// Return whether there are selfloops in the edge list.
///
/// Parameters
/// ----------
/// path: str,
///     The path from where to load the edge list.
/// separator: Optional[str],
///     The separator for the rows in the edge list.
/// header: Optional[bool],
///     Whether the edge list has an header.
/// sources_column: Optional[str],
///     The column name to use for the source nodes.
/// sources_column_number: Optional[int],
///     The column number to use for the source nodes.
/// destinations_column: Optional[str],
///     The column name to use for the destination nodes.
/// destinations_column_number: Optional[int],
///     The column number to use for the destination nodes.
/// comment_symbol: Optional[str],
///     The comment symbol to use for the lines to skip.
/// max_rows_number: Optional[int],
///     The number of rows to read at most. Note that this parameter is ignored when reading in parallel.
/// rows_to_skip: Optional[int],
///     Number of rows to skip in the edge list.
/// edges_number: Optional[int],
///     Number of edges in the edge list.
/// load_edge_list_in_parallel: Optional[bool],
///     Whether to execute the task in parallel or sequential. Generally, parallel is preferable.
/// verbose: Optional[bool],
///     Whether to show the loading bar while processing the file.
/// name: Optional[str],
///     The name of the graph to display in the loading bar.
///
pub fn are_there_selfloops_in_edge_list(
    path: &str,
    separator: Option<String>,
    header: Option<bool>,
    sources_column: Option<String>,
    sources_column_number: Option<usize>,
    destinations_column: Option<String>,
    destinations_column_number: Option<usize>,
    comment_symbol: Option<String>,
    max_rows_number: Option<usize>,
    rows_to_skip: Option<usize>,
    edges_number: Option<EdgeT>,
    load_edge_list_in_parallel: Option<bool>,
    verbose: Option<bool>,
    name: Option<String>,
) -> PyResult<bool> {
    pe!(edge_list_utils::are_there_selfloops_in_edge_list(
        path,
        separator,
        header,
        sources_column,
        sources_column_number,
        destinations_column,
        destinations_column_number,
        comment_symbol,
        max_rows_number,
        rows_to_skip,
        edges_number,
        load_edge_list_in_parallel,
        verbose,
        name
    ))
}

#[pyfunction]
#[automatically_generated_binding]
#[text_signature = "(original_edge_path, original_edge_list_separator, original_edge_list_header, original_edge_list_sources_column, original_edge_list_sources_column_number, original_edge_list_destinations_column, original_edge_list_destinations_column_number, original_edge_list_edge_type_column, original_edge_list_edge_type_column_number, original_edge_list_weights_column, original_edge_list_weights_column_number, target_edge_path, target_edge_list_separator, target_edge_list_header, target_edge_list_sources_column_number, target_edge_list_sources_column, target_edge_list_destinations_column_number, target_edge_list_destinations_column, target_edge_list_edge_type_column, target_edge_list_edge_type_column_number, target_edge_list_weights_column, target_edge_list_weights_column_number, comment_symbol, default_edge_type, default_weight, max_rows_number, rows_to_skip, edges_number, skip_edge_types_if_unavailable, skip_weights_if_unavailable, verbose, name)"]
/// Create a new edge list from a given one filtering duplicates.
///
/// Parameters
/// ----------
/// original_edge_path: str,
///     The path from where to load the original edge list.
/// original_edge_list_separator: Optional[str],
///     Separator to use for the original edge list.
/// original_edge_list_header: Optional[bool],
///     Whether the original edge list has an header.
/// original_edge_list_sources_column: Optional[str],
///     The column name to use to load the sources in the original edges list.
/// original_edge_list_sources_column_number: Optional[int],
///     The column number to use to load the sources in the original edges list.
/// original_edge_list_destinations_column: Optional[str],
///     The column name to use to load the destinations in the original edges list.
/// original_edge_list_destinations_column_number: Optional[int],
///     The column number to use to load the destinations in the original edges list.
/// original_edge_list_edge_type_column: Optional[str],
///     The column name to use for the edge types in the original edges list.
/// original_edge_list_edge_type_column_number: Optional[int],
///     The column number to use for the edge types in the original edges list.
/// original_edge_list_weights_column: Optional[str],
///     The column name to use for the weights in the original edges list.
/// original_edge_list_weights_column_number: Optional[int],
///     The column number to use for the weights in the original edges list.
/// target_edge_path: str,
///     The path from where to load the target edge list.
/// target_edge_list_separator: Optional[str],
///     Separator to use for the target edge list.
/// target_edge_list_header: Optional[bool],
///     Whether the target edge list has an header.
/// target_edge_list_sources_column: Optional[str],
///     The column name to use to load the sources in the target edges list.
/// target_edge_list_sources_column_number: Optional[int],
///     The column number to use to load the sources in the target edges list.
/// target_edge_list_destinations_column: Optional[str],
///     The column name to use to load the destinations in the target edges list.
/// target_edge_list_destinations_column_number: Optional[int],
///     The column number to use to load the destinations in the target edges list.
/// target_edge_list_edge_type_column: Optional[str],
///     The column name to use for the edge types in the target edges list.
/// target_edge_list_edge_type_column_number: Optional[int],
///     The column number to use for the edge types in the target edges list.
/// target_edge_list_weights_column: Optional[str],
///     The column name to use for the weights in the target edges list.
/// target_edge_list_weights_column_number: Optional[int],
///     The column number to use for the weights in the target edges list.
/// comment_symbol: Optional[str],
///     The comment symbol to use within the original edge list.
/// default_edge_type: Optional[str],
///     The default edge type to use within the original edge list.
/// default_weight: Optional[float],
///     The default weight to use within the original edge list.
/// max_rows_number: Optional[int],
///     The amount of rows to load from the original edge list.
/// rows_to_skip: Optional[int],
///     The amount of rows to skip from the original edge list.
/// edges_number: Optional[int],
///     The expected number of edges. It will be used for the loading bar.
/// skip_edge_types_if_unavailable: Optional[bool],
///     Whether to automatically skip the edge types if they are not available.
/// skip_weights_if_unavailable: Optional[bool],
///     Whether to automatically skip the weights if they are not available.
/// verbose: Optional[bool],
///     Whether to show the loading bar while processing the file.
/// name: Optional[str],
///     The name of the graph to display in the loading bar.
///
pub fn filter_duplicates_from_edge_list(
    original_edge_path: &str,
    original_edge_list_separator: Option<String>,
    original_edge_list_header: Option<bool>,
    original_edge_list_sources_column: Option<String>,
    original_edge_list_sources_column_number: Option<usize>,
    original_edge_list_destinations_column: Option<String>,
    original_edge_list_destinations_column_number: Option<usize>,
    original_edge_list_edge_type_column: Option<String>,
    original_edge_list_edge_type_column_number: Option<usize>,
    original_edge_list_weights_column: Option<String>,
    original_edge_list_weights_column_number: Option<usize>,
    target_edge_path: &str,
    target_edge_list_separator: Option<String>,
    target_edge_list_header: Option<bool>,
    target_edge_list_sources_column_number: Option<usize>,
    target_edge_list_sources_column: Option<String>,
    target_edge_list_destinations_column_number: Option<usize>,
    target_edge_list_destinations_column: Option<String>,
    target_edge_list_edge_type_column: Option<String>,
    target_edge_list_edge_type_column_number: Option<usize>,
    target_edge_list_weights_column: Option<String>,
    target_edge_list_weights_column_number: Option<usize>,
    comment_symbol: Option<String>,
    default_edge_type: Option<String>,
    default_weight: Option<WeightT>,
    max_rows_number: Option<usize>,
    rows_to_skip: Option<usize>,
    edges_number: Option<usize>,
    skip_edge_types_if_unavailable: Option<bool>,
    skip_weights_if_unavailable: Option<bool>,
    verbose: Option<bool>,
    name: Option<String>,
) -> PyResult<()> {
    pe!(edge_list_utils::filter_duplicates_from_edge_list(
        original_edge_path,
        original_edge_list_separator,
        original_edge_list_header,
        original_edge_list_sources_column,
        original_edge_list_sources_column_number,
        original_edge_list_destinations_column,
        original_edge_list_destinations_column_number,
        original_edge_list_edge_type_column,
        original_edge_list_edge_type_column_number,
        original_edge_list_weights_column,
        original_edge_list_weights_column_number,
        target_edge_path,
        target_edge_list_separator,
        target_edge_list_header,
        target_edge_list_sources_column_number,
        target_edge_list_sources_column,
        target_edge_list_destinations_column_number,
        target_edge_list_destinations_column,
        target_edge_list_edge_type_column,
        target_edge_list_edge_type_column_number,
        target_edge_list_weights_column,
        target_edge_list_weights_column_number,
        comment_symbol,
        default_edge_type,
        default_weight,
        max_rows_number,
        rows_to_skip,
        edges_number,
        skip_edge_types_if_unavailable,
        skip_weights_if_unavailable,
        verbose,
        name
    ))
}

#[pyfunction]
#[automatically_generated_binding]
#[text_signature = "(original_edge_path, original_edge_list_separator, original_edge_list_header, original_sources_column, original_sources_column_number, original_destinations_column, original_destinations_column_number, original_edge_list_edge_type_column, original_edge_list_edge_type_column_number, original_weights_column, original_weights_column_number, target_edge_path, target_edge_list_separator, target_edge_list_header, target_sources_column, target_sources_column_number, target_destinations_column, target_destinations_column_number, target_edge_list_edge_type_column, target_edge_list_edge_type_column_number, target_weights_column, target_weights_column_number, comment_symbol, default_edge_type, default_weight, max_rows_number, rows_to_skip, edges_number, skip_edge_types_if_unavailable, skip_weights_if_unavailable, verbose, name)"]
/// Create a new directed edge list from a given undirected one by duplicating the undirected edges.
///
/// Parameters
/// ----------
/// original_edge_path: str,
///     The path from where to load the original edge list.
/// original_edge_list_separator: Optional[str],
///     Separator to use for the original edge list.
/// original_edge_list_header: Optional[bool],
///     Whether the original edge list has an header.
/// original_sources_column: Optional[str],
///     The column name to use to load the sources in the original edges list.
/// original_sources_column_number: Optional[int],
///     The column number to use to load the sources in the original edges list.
/// original_destinations_column: Optional[str],
///     The column name to use to load the destinations in the original edges list.
/// original_destinations_column_number: Optional[int],
///     The column number to use to load the destinations in the original edges list.
/// original_edge_list_edge_type_column: Optional[str],
///     The column name to use for the edge types in the original edges list.
/// original_edge_list_edge_type_column_number: Optional[int],
///     The column number to use for the edge types in the original edges list.
/// original_weights_column: Optional[str],
///     The column name to use for the weights in the original edges list.
/// original_weights_column_number: Optional[int],
///     The column number to use for the weights in the original edges list.
/// target_edge_path: str,
///     The path from where to load the target edge list. This must be different from the original edge list path.
/// target_edge_list_separator: Optional[str],
///     Separator to use for the target edge list. If None, the one provided from the original edge list will be used.
/// target_edge_list_header: Optional[bool],
///     Whether the target edge list has an header. If None, the one provided from the original edge list will be used.
/// target_sources_column: Optional[str],
///     The column name to use to load the sources in the target edges list. If None, the one provided from the original edge list will be used.
/// target_sources_column_number: Optional[int],
///     The column number to use to load the sources in the target edges list. If None, the one provided from the original edge list will be used.
/// target_destinations_column: Optional[str],
///     The column name to use to load the destinations in the target edges list. If None, the one provided from the original edge list will be used.
/// target_destinations_column_number: Optional[int],
///     The column number to use to load the destinations in the target edges list. If None, the one provided from the original edge list will be used.
/// target_edge_list_edge_type_column: Optional[str],
///     The column name to use for the edge types in the target edges list. If None, the one provided from the original edge list will be used.
/// target_edge_list_edge_type_column_number: Optional[int],
///     The column number to use for the edge types in the target edges list. If None, the one provided from the original edge list will be used.
/// target_weights_column: Optional[str],
///     The column name to use for the weights in the target edges list. If None, the one provided from the original edge list will be used.
/// target_weights_column_number: Optional[int],
///     The column number to use for the weights in the target edges list. If None, the one provided from the original edge list will be used.
/// comment_symbol: Optional[str],
///     The comment symbol to use within the original edge list.
/// default_edge_type: Optional[str],
///     The default edge type to use within the original edge list.
/// default_weight: Optional[float],
///     The default weight to use within the original edge list.
/// max_rows_number: Optional[int],
///     The amount of rows to load from the original edge list.
/// rows_to_skip: Optional[int],
///     The amount of rows to skip from the original edge list.
/// edges_number: Optional[int],
///     The expected number of edges. It will be used for the loading bar.
/// skip_edge_types_if_unavailable: Optional[bool],
///     Whether to automatically skip the edge types if they are not available.
/// skip_weights_if_unavailable: Optional[bool],
///     Whether to automatically skip the weights if they are not available.
/// verbose: Optional[bool],
///     Whether to show the loading bar while processing the file.
/// name: Optional[str],
///     The name of the graph to display in the loading bar.
///
pub fn convert_undirected_edge_list_to_directed(
    original_edge_path: &str,
    original_edge_list_separator: Option<String>,
    original_edge_list_header: Option<bool>,
    original_sources_column: Option<String>,
    original_sources_column_number: Option<usize>,
    original_destinations_column: Option<String>,
    original_destinations_column_number: Option<usize>,
    original_edge_list_edge_type_column: Option<String>,
    original_edge_list_edge_type_column_number: Option<usize>,
    original_weights_column: Option<String>,
    original_weights_column_number: Option<usize>,
    target_edge_path: &str,
    target_edge_list_separator: Option<String>,
    target_edge_list_header: Option<bool>,
    target_sources_column: Option<String>,
    target_sources_column_number: Option<usize>,
    target_destinations_column: Option<String>,
    target_destinations_column_number: Option<usize>,
    target_edge_list_edge_type_column: Option<String>,
    target_edge_list_edge_type_column_number: Option<usize>,
    target_weights_column: Option<String>,
    target_weights_column_number: Option<usize>,
    comment_symbol: Option<String>,
    default_edge_type: Option<String>,
    default_weight: Option<WeightT>,
    max_rows_number: Option<usize>,
    rows_to_skip: Option<usize>,
    edges_number: Option<usize>,
    skip_edge_types_if_unavailable: Option<bool>,
    skip_weights_if_unavailable: Option<bool>,
    verbose: Option<bool>,
    name: Option<String>,
) -> PyResult<EdgeT> {
    pe!(edge_list_utils::convert_undirected_edge_list_to_directed(
        original_edge_path,
        original_edge_list_separator,
        original_edge_list_header,
        original_sources_column,
        original_sources_column_number,
        original_destinations_column,
        original_destinations_column_number,
        original_edge_list_edge_type_column,
        original_edge_list_edge_type_column_number,
        original_weights_column,
        original_weights_column_number,
        target_edge_path,
        target_edge_list_separator,
        target_edge_list_header,
        target_sources_column,
        target_sources_column_number,
        target_destinations_column,
        target_destinations_column_number,
        target_edge_list_edge_type_column,
        target_edge_list_edge_type_column_number,
        target_weights_column,
        target_weights_column_number,
        comment_symbol,
        default_edge_type,
        default_weight,
        max_rows_number,
        rows_to_skip,
        edges_number,
        skip_edge_types_if_unavailable,
        skip_weights_if_unavailable,
        verbose,
        name
    ))
}

#[pyfunction]
#[automatically_generated_binding]
#[text_signature = "(path, separator, header, sources_column, sources_column_number, destinations_column, destinations_column_number, comment_symbol, max_rows_number, rows_to_skip, edges_number, load_edge_list_in_parallel, verbose, name)"]
/// Return number of selfloops in the given edge list.
///
/// Parameters
/// ----------
/// path: str,
///     The path from where to load the edge list.
/// separator: Optional[str],
///     The separator for the rows in the edge list.
/// header: Optional[bool],
///     Whether the edge list has an header.
/// sources_column: Optional[str],
///     The column name to use for the source nodes.
/// sources_column_number: Optional[int],
///     The column number to use for the source nodes.
/// destinations_column: Optional[str],
///     The column name to use for the destination nodes.
/// destinations_column_number: Optional[int],
///     The column number to use for the destination nodes.
/// comment_symbol: Optional[str],
///     The comment symbol to use for the lines to skip.
/// max_rows_number: Optional[int],
///     The number of rows to read at most. Note that this parameter is ignored when reading in parallel.
/// rows_to_skip: Optional[int],
///     Number of rows to skip in the edge list.
/// edges_number: Optional[int],
///     Number of edges in the edge list.
/// load_edge_list_in_parallel: Optional[bool],
///     Whether to execute the task in parallel or sequential. Generally, parallel is preferable.
/// verbose: Optional[bool],
///     Whether to show the loading bar while processing the file.
/// name: Optional[str],
///     The name of the graph to display in the loading bar.
///
pub fn is_numeric_edge_list(
    path: &str,
    separator: Option<String>,
    header: Option<bool>,
    sources_column: Option<String>,
    sources_column_number: Option<usize>,
    destinations_column: Option<String>,
    destinations_column_number: Option<usize>,
    comment_symbol: Option<String>,
    max_rows_number: Option<usize>,
    rows_to_skip: Option<usize>,
    edges_number: Option<EdgeT>,
    load_edge_list_in_parallel: Option<bool>,
    verbose: Option<bool>,
    name: Option<String>,
) -> PyResult<bool> {
    pe!(edge_list_utils::is_numeric_edge_list(
        path,
        separator,
        header,
        sources_column,
        sources_column_number,
        destinations_column,
        destinations_column_number,
        comment_symbol,
        max_rows_number,
        rows_to_skip,
        edges_number,
        load_edge_list_in_parallel,
        verbose,
        name
    ))
}

#[pyfunction]
#[automatically_generated_binding]
#[text_signature = "(path, separator, header, sources_column, sources_column_number, destinations_column, destinations_column_number, comment_symbol, max_rows_number, rows_to_skip, edges_number, load_edge_list_in_parallel, verbose, name)"]
/// Return number of selfloops in the given edge list.
///
/// Parameters
/// ----------
/// path: str,
///     The path from where to load the edge list.
/// separator: Optional[str],
///     The separator for the rows in the edge list.
/// header: Optional[bool],
///     Whether the edge list has an header.
/// sources_column: Optional[str],
///     The column name to use for the source nodes.
/// sources_column_number: Optional[int],
///     The column number to use for the source nodes.
/// destinations_column: Optional[str],
///     The column name to use for the destination nodes.
/// destinations_column_number: Optional[int],
///     The column number to use for the destination nodes.
/// comment_symbol: Optional[str],
///     The comment symbol to use for the lines to skip.
/// max_rows_number: Optional[int],
///     The number of rows to read at most. Note that this parameter is ignored when reading in parallel.
/// rows_to_skip: Optional[int],
///     Number of rows to skip in the edge list.
/// edges_number: Optional[int],
///     Number of edges in the edge list.
/// load_edge_list_in_parallel: Optional[bool],
///     Whether to execute the task in parallel or sequential. Generally, parallel is preferable.
/// verbose: Optional[bool],
///     Whether to show the loading bar while processing the file.
/// name: Optional[str],
///     The name of the graph to display in the loading bar.
///
pub fn get_selfloops_number_from_edge_list(
    path: &str,
    separator: Option<String>,
    header: Option<bool>,
    sources_column: Option<String>,
    sources_column_number: Option<usize>,
    destinations_column: Option<String>,
    destinations_column_number: Option<usize>,
    comment_symbol: Option<String>,
    max_rows_number: Option<usize>,
    rows_to_skip: Option<usize>,
    edges_number: Option<EdgeT>,
    load_edge_list_in_parallel: Option<bool>,
    verbose: Option<bool>,
    name: Option<String>,
) -> PyResult<EdgeT> {
    pe!(edge_list_utils::get_selfloops_number_from_edge_list(
        path,
        separator,
        header,
        sources_column,
        sources_column_number,
        destinations_column,
        destinations_column_number,
        comment_symbol,
        max_rows_number,
        rows_to_skip,
        edges_number,
        load_edge_list_in_parallel,
        verbose,
        name
    ))
}

#[pyfunction]
#[automatically_generated_binding]
#[text_signature = "(path, separator, header, sources_column, sources_column_number, destinations_column, destinations_column_number, comment_symbol, max_rows_number, rows_to_skip, edges_number, load_edge_list_in_parallel, verbose, name)"]
/// Return minimum and maximum node number from given numeric edge list.
///
/// Parameters
/// ----------
/// path: str,
///     The path from where to load the edge list.
/// separator: Optional[str],
///     The separator for the rows in the edge list.
/// header: Optional[bool],
///     Whether the edge list has an header.
/// sources_column: Optional[str],
///     The column name to use for the source nodes.
/// sources_column_number: Optional[int],
///     The column number to use for the source nodes.
/// destinations_column: Optional[str],
///     The column name to use for the destination nodes.
/// destinations_column_number: Optional[int],
///     The column number to use for the destination nodes.
/// comment_symbol: Optional[str],
///     The comment symbol to use for the lines to skip.
/// max_rows_number: Optional[int],
///     The number of rows to read at most. Note that this parameter is ignored when reading in parallel.
/// rows_to_skip: Optional[int],
///     Number of rows to skip in the edge list.
/// edges_number: Optional[int],
///     Number of edges in the edge list.
/// load_edge_list_in_parallel: Optional[bool],
///     Whether to execute the task in parallel or sequential. Generally, parallel is preferable.
/// verbose: Optional[bool],
///     Whether to show the loading bar while processing the file.
/// name: Optional[str],
///     The name of the graph to display in the loading bar.
///
///
/// Raises
/// -------
/// ValueError
///     If there are problems with the edge list file.
/// ValueError
///     If the elements in the edge list are not numeric.
/// ValueError
///     If the edge list is empty.
///
pub fn get_minmax_node_from_numeric_edge_list(
    path: &str,
    separator: Option<String>,
    header: Option<bool>,
    sources_column: Option<String>,
    sources_column_number: Option<usize>,
    destinations_column: Option<String>,
    destinations_column_number: Option<usize>,
    comment_symbol: Option<String>,
    max_rows_number: Option<usize>,
    rows_to_skip: Option<usize>,
    edges_number: Option<EdgeT>,
    load_edge_list_in_parallel: Option<bool>,
    verbose: Option<bool>,
    name: Option<String>,
) -> PyResult<(EdgeT, EdgeT, EdgeT)> {
    pe!(edge_list_utils::get_minmax_node_from_numeric_edge_list(
        path,
        separator,
        header,
        sources_column,
        sources_column_number,
        destinations_column,
        destinations_column_number,
        comment_symbol,
        max_rows_number,
        rows_to_skip,
        edges_number,
        load_edge_list_in_parallel,
        verbose,
        name
    ))
}

#[pyfunction]
#[automatically_generated_binding]
#[text_signature = "(path, target_path, separator, header, sources_column, sources_column_number, destinations_column, destinations_column_number, edge_types_column, edge_types_column_number, rows_to_skip, skip_edge_types_if_unavailable)"]
/// Sort given numeric edge list in place using the sort command.
///
/// Parameters
/// ----------
/// path: str,
///     The path from where to load the edge list.
/// target_path: str,
///     The where to store the edge list.
/// separator: Optional[str],
///     The separator for the rows in the edge list.
/// header: Optional[bool],
///     Whether the edge list has an header.
/// sources_column: Optional[str],
///     The column name to use for the source nodes.
/// sources_column_number: Optional[int],
///     The column number to use for the source nodes.
/// destinations_column: Optional[str],
///     The column name to use for the destination nodes.
/// destinations_column_number: Optional[int],
///     The column number to use for the destination nodes.
/// edge_types_column: Optional[str],
///     The column name to use for the edge types.
/// edge_types_column_number: Optional[int],
///     The column number to use for the edge types.
/// rows_to_skip: Optional[int],
///     Number of rows to skip in the edge list.
/// skip_edge_types_if_unavailable: Optional[bool],
///     Whether to automatically skip the edge types if they are not available.
///
pub fn sort_numeric_edge_list(
    path: &str,
    target_path: &str,
    separator: Option<String>,
    header: Option<bool>,
    sources_column: Option<String>,
    sources_column_number: Option<usize>,
    destinations_column: Option<String>,
    destinations_column_number: Option<usize>,
    edge_types_column: Option<String>,
    edge_types_column_number: Option<usize>,
    rows_to_skip: Option<usize>,
    skip_edge_types_if_unavailable: Option<bool>,
) -> PyResult<()> {
    pe!(edge_list_utils::sort_numeric_edge_list(
        path,
        target_path,
        separator,
        header,
        sources_column,
        sources_column_number,
        destinations_column,
        destinations_column_number,
        edge_types_column,
        edge_types_column_number,
        rows_to_skip,
        skip_edge_types_if_unavailable
    ))
}

#[pyfunction]
#[automatically_generated_binding]
#[text_signature = "(path, separator, header, sources_column, sources_column_number, destinations_column, destinations_column_number, edge_types_column, edge_types_column_number, rows_to_skip, skip_edge_types_if_unavailable)"]
/// Sort given numeric edge list in place using the sort command.
///
/// Parameters
/// ----------
/// path: str,
///     The path from where to load the edge list.
/// separator: Optional[str],
///     The separator for the rows in the edge list.
/// header: Optional[bool],
///     Whether the edge list has an header.
/// sources_column: Optional[str],
///     The column name to use for the source nodes.
/// sources_column_number: Optional[int],
///     The column number to use for the source nodes.
/// destinations_column: Optional[str],
///     The column name to use for the destination nodes.
/// destinations_column_number: Optional[int],
///     The column number to use for the destination nodes.
/// edge_types_column: Optional[str],
///     The column name to use for the edge types.
/// edge_types_column_number: Optional[int],
///     The column number to use for the edge types.
/// rows_to_skip: Optional[int],
///     Number of rows to skip in the edge list.
/// skip_edge_types_if_unavailable: Optional[bool],
///     Whether to automatically skip the edge types if they are not available.
///
pub fn sort_numeric_edge_list_inplace(
    path: &str,
    separator: Option<String>,
    header: Option<bool>,
    sources_column: Option<String>,
    sources_column_number: Option<usize>,
    destinations_column: Option<String>,
    destinations_column_number: Option<usize>,
    edge_types_column: Option<String>,
    edge_types_column_number: Option<usize>,
    rows_to_skip: Option<usize>,
    skip_edge_types_if_unavailable: Option<bool>,
) -> PyResult<()> {
    pe!(edge_list_utils::sort_numeric_edge_list_inplace(
        path,
        separator,
        header,
        sources_column,
        sources_column_number,
        destinations_column,
        destinations_column_number,
        edge_types_column,
        edge_types_column_number,
        rows_to_skip,
        skip_edge_types_if_unavailable
    ))
}
