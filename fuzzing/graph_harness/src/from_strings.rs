use super::*;
use arbitrary::Arbitrary;
use graph::ItersWrapper;
use rayon::iter::IntoParallelIterator;

#[derive(Arbitrary, Debug, Clone)]
pub struct FromStringsParameters {
    pub number_of_node_types: Option<u8>,
    pub numeric_node_type_ids: Option<bool>,
    pub minimum_node_type_id: Option<NodeTypeT>,
    pub has_node_types: bool,
    pub number_of_nodes: Option<u8>,
    pub numeric_node_ids: bool,
    pub numeric_node_list_node_type_ids: bool,
    pub minimum_node_id: Option<NodeT>,
    pub number_of_edge_types: Option<u8>,
    pub numeric_edge_type_ids: Option<bool>,
    pub minimum_edge_type_id: Option<EdgeTypeT>,
    pub has_edge_types: bool,
    pub has_edge_weights: bool,
    pub directed: bool,
    pub complete: Option<bool>,
    pub sorted: Option<bool>,
    pub number_of_edges: Option<u8>,
    pub numeric_edge_list_node_ids: Option<bool>,
    pub numeric_edge_list_edge_type_ids: Option<bool>,
    pub skip_node_types_if_unavailable: Option<bool>,
    pub skip_edge_types_if_unavailable: Option<bool>,
    pub name: String,

    pub edges_iterator_is_parallel: bool,
    pub edge_types_iterator_is_parallel: bool,
    pub nodes_iterator_is_parallel: bool,
    pub node_types_iterator_is_parallel: bool,

    pub edges_iterator: Option<Vec<Result<(usize, StringQuadruple)>>>,
    pub edge_types_iterator: Option<Vec<Result<(usize, String)>>>,
    pub nodes_iterator: Option<Vec<Result<(usize, (String, Option<Vec<String>>))>>>,
    pub node_types_iterator: Option<Vec<Result<(usize, String)>>>,
}

macro_rules! to_iter_wrapper {
    ($is_parallel:expr, $array:expr) => {{
        $array.map(|value| {
            if $is_parallel {
                ItersWrapper::Parallel(value.into_par_iter())
            } else {
                ItersWrapper::Sequential(value.into_iter())
            }
        })
    }};
}

fn truncate(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}

pub fn build_graph_from_strings_harness(data: FromStringsParameters) -> Result<()> {
    let data_copy = data.clone();
    std::panic::set_hook(Box::new(move |info| {
        handle_panics_from_strings(info, data_copy.clone());
    }));

    let FromStringsParameters {
        number_of_node_types,
        numeric_node_type_ids,
        minimum_node_type_id,
        has_node_types,
        number_of_nodes,
        numeric_node_ids,
        numeric_node_list_node_type_ids,
        minimum_node_id,
        number_of_edge_types,
        numeric_edge_type_ids,
        minimum_edge_type_id,
        has_edge_types,
        has_edge_weights,
        directed,
        complete,
        sorted,
        number_of_edges,
        numeric_edge_list_node_ids,
        numeric_edge_list_edge_type_ids,
        skip_node_types_if_unavailable,
        skip_edge_types_if_unavailable,
        name,
        edges_iterator_is_parallel,
        edge_types_iterator_is_parallel,
        nodes_iterator_is_parallel,
        node_types_iterator_is_parallel,
        edges_iterator,
        edge_types_iterator,
        nodes_iterator,
        node_types_iterator,
    } = data;

    let mut graph = graph::build_graph_from_strings(
        to_iter_wrapper!(
            node_types_iterator_is_parallel,
            node_types_iterator.map(|node_types_iterator| node_types_iterator
                .into_iter()
                .map(|line| match line {
                    Ok((line_number, type_name)) => {
                        Ok((line_number, truncate(type_name.as_str(), 3).to_string()))
                    }
                    Err(e) => Err(e),
                })
                .collect::<Vec<_>>())
        ),
        number_of_node_types.map(|x| x as NodeTypeT),
        numeric_node_type_ids,
        minimum_node_type_id,
        has_node_types,
        None,
        to_iter_wrapper!(
            nodes_iterator_is_parallel,
            nodes_iterator.map(|nodes_iterator| nodes_iterator
                .into_iter()
                .map(|line| match line {
                    Ok((line_number, (node_name, node_types))) => {
                        Ok((
                            line_number,
                            (
                                truncate(node_name.as_str(), 3).to_string(),
                                node_types.map(|node_types| {
                                    node_types
                                        .into_iter()
                                        .map(|node_type| {
                                            truncate(node_type.as_str(), 3).to_string()
                                        })
                                        .collect::<Vec<_>>()
                                }),
                            ),
                        ))
                    }
                    Err(e) => Err(e),
                })
                .collect::<Vec<_>>())
        ),
        number_of_nodes.map(|x| x as NodeT),
        false,
        numeric_node_ids,
        numeric_node_list_node_type_ids,
        minimum_node_id,
        to_iter_wrapper!(
            edge_types_iterator_is_parallel,
            edge_types_iterator.map(|edge_types_iterator| edge_types_iterator
                .into_iter()
                .map(|line| match line {
                    Ok((line_number, type_name)) => {
                        Ok((line_number, truncate(type_name.as_str(), 3).to_string()))
                    }
                    Err(e) => Err(e),
                })
                .collect::<Vec<_>>())
        ),
        number_of_edge_types.map(|x| x as EdgeTypeT),
        numeric_edge_type_ids,
        minimum_edge_type_id,
        has_edge_types,
        None,
        to_iter_wrapper!(
            edges_iterator_is_parallel,
            edges_iterator.map(|edges_iterator| edges_iterator
                .into_iter()
                .map(|line| match line {
                    Ok((line_number, (src, dst, edge_type, weight))) => {
                        Ok((
                            line_number,
                            (
                                truncate(src.as_str(), 3).to_string(),
                                truncate(dst.as_str(), 3).to_string(),
                                edge_type
                                    .map(|edge_type| truncate(edge_type.as_str(), 3).to_string()),
                                weight,
                            ),
                        ))
                    }
                    Err(e) => Err(e),
                })
                .collect::<Vec<_>>())
        ),
        has_edge_weights,
        directed,
        Some(false),
        complete,
        Some(true),
        sorted,
        number_of_edges.map(|x| x as EdgeT),
        numeric_edge_list_node_ids,
        numeric_edge_list_edge_type_ids,
        skip_node_types_if_unavailable,
        skip_edge_types_if_unavailable,
        true,
        true,
        name,
    )?;

    // We ignore this error because we execute only the fuzzing to find
    // the panic situations that are NOT just errors, but unhandled errors.
    let _ = graph::test_utilities::default_test_suite(&mut graph, None);

    Ok(())
}

/// This function takes the data used for the current fuzz case and dump it.
/// this is needed for the automatic generation of unit tests from fuzzing.
fn handle_panics_from_strings(info: &std::panic::PanicInfo, data: FromStringsParameters) {
    // Find the root of the repository
    let path = get_folder();
    dump_panic_info(format!("{}/panic.csv", path), info);
    // Dump the informations
    std::fs::write(format!("{}/data.txt", &path), format!("{:#4?}", &data))
        .expect("Cannot write the edge file");
    dump_backtrace(&path);
}
