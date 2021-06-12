use super::*;
use bitvec::prelude::*;
use elias_fano_rust::EliasFano;
use indicatif::ProgressIterator;
use itertools::Itertools;
use log::info;
use num_traits::Zero;
use rayon::iter::ParallelIterator;
use rayon::prelude::ParallelSliceMut;
use roaring::RoaringBitmap;
use std::cmp::Ordering;
use std::collections::BTreeMap;

type ParsedStringEdgesType = Result<
    (
        EliasFano,
        Option<EliasFano>,
        Vocabulary<NodeT>,
        Option<EdgeTypeVocabulary>,
        Option<Vec<WeightT>>,
        Option<WeightT>,
        Option<WeightT>,
        EdgeT,
        EdgeT,
        NodeT,
        NodeT,
        NodeT,
        u64,
        u8,
        Option<BitVec<Lsb0, u8>>,
        Option<RoaringBitmap>,
        NodeT,
        NodeT,
        Option<f64>,
        Option<f64>,
        Option<f64>,
        Option<NodeT>,
        bool,
        bool,
    ),
    String,
>;

/// Returns result representing if the given combination of numeric node ids and edge node ids is valid.
///
/// The error message given within the method should contain all informations
/// relative to the logic of this check.
///
/// # Arguments
/// * `has_nodes_list`: bool - Whether the graph has a node list.
/// * `numeric_node_ids`: bool - Whether the node IDs should be loaded as integers.
/// * `numeric_edge_node_ids`: bool - Whether the edge node IDs should be loaded as integers.
///
/// # Raises
/// * If the given combination of numeric nodes is not feaseable.
fn check_numeric_ids_compatibility(
    has_nodes_list: bool,
    numeric_node_ids: bool,
    numeric_edge_node_ids: bool,
) -> Result<(), String> {
    if has_nodes_list && numeric_node_ids && !numeric_edge_node_ids {
        return Err(concat!(
            "You are trying to load a numeric node list and a non numeric edge list.\n",
            "This is a problem because an edge composed of two nodes (e.g. \"2, 8\") is ",
            "not necessarily mapped internally to the same node ids of the node list.\n",
            "Possibily you want to also enable the parameter for the numeric edge node ids."
        )
        .to_string());
    }
    Ok(())
}

/// Returns iterator of nodes handling the node IDs.
///
/// # Arguments
/// nodes_iter: impl Iterator<Item = Result<(String, Option<Vec<String>>), String>> + 'a,
///     Iterator over the node list.
/// ignore_duplicated_nodes: bool,
///     Whether to just ignore the duplicated node types.
/// node_list_is_correct: bool,
///     Parameter to pinky promise that the node list is correct.
///     If you provide a broken node list to this method while promising
///     that the node list is correct, be prepared to deal with the fallout.
///     This parameter is mainly meant to be used internally when creating
///     graphs that CANNOT BE BROKEN by design. If you use this parameter
///     from any of the bindings, be SURE that the node list is actually
///     correct.
///     We assume that any provided node list is broken until disproved.
/// nodes: &'b mut Vocabulary<NodeT>,
///     Vocabulary of the nodes to be populated.
pub(crate) fn parse_node_ids<'a>(
    nodes_iter: impl Iterator<Item = Result<(String, Option<Vec<String>>), String>> + 'a,
    ignore_duplicated_nodes: bool,
    node_list_is_correct: bool,
    nodes: &'a mut Vocabulary<NodeT>,
) -> Box<dyn Iterator<Item = Result<(NodeT, Option<Vec<String>>), String>> + 'a> {
    // If the user is telling us that the node list is **surely correct**,
    // we can skip a significant amount of checks and therefore create
    // a simpler iterator.
    if node_list_is_correct {
        Box::new(
            nodes_iter.map_ok(move |(node_name, node_type)| {
                (nodes.unchecked_insert(node_name), node_type)
            }),
        )
    } else {
        Box::new(nodes_iter.filter_map(move |row| {
            row.map_or_else(|err| Some(Err(err)),  |(node_name, node_type)| {
                nodes.insert(node_name.as_str()).map_or_else(|err| Some(Err(err)), |(node_id, already_present_in_vocabulary)|{
                    if already_present_in_vocabulary{
                        if ignore_duplicated_nodes {
                            None
                        } else {
                            Some(Err(format!(
                                concat!(
                                    "The node {node_name} appears multiple times in the node list.\n",
                                    "The node type of the row is {node_type:?}.\n",
                                    "The library does not currently support multiple node types for a single node."
                                ),
                                node_name = node_name,
                                node_type = node_type
                            )))
                        }
                    } else {
                        Some(Ok((node_id, node_type)))
                    }
                })
            })
        }))
    }
}

/// Returns iterator of nodes handling the node type IDs.
///
/// # Arguments
/// nodes_iter: impl Iterator<Item = Result<(NodeT, Option<Vec<String>>), String>> + 'a,
///     Iterator over the node list.
/// node_list_is_correct: bool,
///     Parameter to pinky promise that the node list is correct.
///     If you provide a broken node list to this method while promising
///     that the node list is correct, be prepared to deal with the fallout.
///     This parameter is mainly meant to be used internally when creating
///     graphs that CANNOT BE BROKEN by design. If you use this parameter
///     from any of the bindings, be SURE that the node list is actually
///     correct.
///     We assume that any provided node list is broken until disproved.
/// node_types_vocabulary: &'b mut NodeTypeVocabulary,
///     Node types vocabulary to be populated.
pub(crate) fn parse_node_type_ids<'a>(
    nodes_iter: impl Iterator<Item = Result<(NodeT, Option<Vec<String>>), String>> + 'a,
    node_list_is_correct: bool,
    node_types_vocabulary: &'a mut NodeTypeVocabulary,
) -> Box<dyn Iterator<Item = Result<(NodeT, Option<Vec<NodeTypeT>>), String>> + 'a> {
    if node_list_is_correct {
        Box::new(nodes_iter.map_ok(move |(node_id, node_type_names)| {
            (
                node_id,
                node_types_vocabulary.unchecked_insert_values(node_type_names),
            )
        }))
    } else {
        Box::new(nodes_iter.map(move |row| {
            row.and_then(|(node_id, node_type_names)| {
                Ok((
                    node_id,
                    node_types_vocabulary.insert_values(node_type_names)?,
                ))
            })
        }))
    }
}

/// Returns modified iterator, adding what is need to digest edge node names into edge node IDs.
/// edges_iterator: impl Iterator<Item = Result<StringQuadruple, String>> + 'a,
///     Iterator over the edge node names list.
/// edge_list_is_correct: bool,
///     Parameter to pinky promise that the edge list is correct.
///     If you provide a broken edge list to this method while promising
///     that the edge list is correct, be prepared to deal with the fallout.
///     This parameter is mainly meant to be used internally when creating
///     graphs that CANNOT BE BROKEN by design. If you use this parameter
///     from any of the bindings, be SURE that the edge list is actually
///     correct.
///     We assume that any provided edge list is broken until disproved.
/// nodes: &'b mut Vocabulary<NodeT>,
///     Vocabulary of the nodes to be populated.
pub(crate) fn parse_edges_node_ids<'a>(
    edges_iterator: impl Iterator<Item = Result<StringQuadruple, String>> + 'a,
    edge_list_is_correct: bool,
    nodes: &'a mut Vocabulary<NodeT>,
) -> Box<dyn Iterator<Item = Result<(NodeT, NodeT, Option<String>, Option<WeightT>), String>> + 'a>
{
    let empty_nodes_mapping = nodes.is_empty();
    // If the user is telling us that the edge list is **surely correct**,
    // we can skip a significant amount of checks and therefore create
    // a simpler iterator.
    if edge_list_is_correct {
        Box::new(
            edges_iterator.map_ok(move |(src_name, dst_name, edge_type, weight)| {
                (
                    nodes.unchecked_insert(src_name),
                    nodes.unchecked_insert(dst_name),
                    edge_type,
                    weight,
                )
            }),
        )
    } else {
        Box::new(
            edges_iterator.map(move |row: Result<StringQuadruple, String>| {
                row.and_then(|(src_name, dst_name, edge_type, weight)| {
                    let (source_node_id, source_was_present) = nodes.insert(src_name.as_str())?;
                    let (destination_node_id, destination_was_present) =
                        nodes.insert(dst_name.as_str())?;
                    if !empty_nodes_mapping && (!source_was_present || !destination_was_present) {
                        Err(format!(
                            concat!(
                                "In the edge list was found the edge ({} => {}) ",
                                "containing nodes that do not appear in the given node list."
                            ),
                            src_name, dst_name
                        ))
                    } else {
                        Ok((source_node_id, destination_node_id, edge_type, weight))
                    }
                })
            }),
        )
    }
}

/// Returns iterator of edges handling the edge type IDs.
///
/// # Arguments
/// edges_iter: impl Iterator<Item = Result<(NodeT, NodeT, Option<String>, Option<WeightT>), String>> + 'a,
///     Iterator over the edge node names list.
/// edge_list_is_correct: bool,
///     Parameter to pinky promise that the edge list is correct.
///     If you provide a broken edge list to this method while promising
///     that the edge list is correct, be prepared to deal with the fallout.
///     This parameter is mainly meant to be used internally when creating
///     graphs that CANNOT BE BROKEN by design. If you use this parameter
///     from any of the bindings, be SURE that the edge list is actually
///     correct.
///     We assume that any provided edge list is broken until disproved.
/// edge_types: &'b mut Vocabulary<EdgeTypeT>,
///     Vocabulary of the edge types to be populated.
pub(crate) fn parse_edge_type_ids_vocabulary<'a>(
    edges_iter: impl Iterator<Item = Result<(NodeT, NodeT, Option<String>, Option<WeightT>), String>>
        + 'a,
    edge_list_is_correct: bool,
    edge_types: &'a mut Vocabulary<EdgeTypeT>,
) -> Box<dyn Iterator<Item = Result<Quadruple, String>> + 'a> {
    if edge_list_is_correct {
        Box::new(edges_iter.map_ok(move |(src, dst, edge_type, weight)| {
            (
                src,
                dst,
                edge_type.map(|et| edge_types.unchecked_insert(et)),
                weight,
            )
        }))
    } else {
        Box::new(edges_iter.map(move |row| {
            row.and_then(|(src, dst, edge_type, weight)| {
                Ok((
                    src,
                    dst,
                    edge_type.map_or_else(
                        || Ok::<_, String>(None),
                        |et| Ok(Some(edge_types.insert(et)?.0)),
                    )?,
                    weight,
                ))
            })
        }))
    }
}

/// TODO: I think this method can be made better!
pub(crate) fn parse_sorted_edges<'a>(
    edges_iter: impl Iterator<Item = Result<Quadruple, String>> + 'a,
    directed: bool,
    directed_edge_list: bool,
) -> Box<dyn Iterator<Item = Result<Quadruple, String>> + 'a> {
    if directed || directed_edge_list {
        return Box::new(edges_iter);
    }
    let mut sorting_tmp: BTreeMap<Triple, Option<WeightT>> = BTreeMap::new();
    Box::new(edges_iter
        .map(Some)
        .chain(vec![None])
        .flat_map(move |maybe_row| match maybe_row {
            Some(row) => {
                let mut results: Vec<Result<Quadruple, String>> = Vec::with_capacity(1);
                let result = match row {
                    Ok((src, dst, edge_type, weight)) => {
                        if !directed && src < dst {
                            sorting_tmp.insert((dst, src, edge_type), weight);
                        }
                        while !sorting_tmp.is_empty()
                            && *sorting_tmp.first_key_value().unwrap().0 < (src, dst, edge_type)
                        {
                            let ((smaller_src, smaller_dst, smaller_edge_type), smaller_weight) =
                                sorting_tmp.pop_first().unwrap();
                            results.push(Ok((
                                smaller_src,
                                smaller_dst,
                                smaller_edge_type,
                                smaller_weight,
                            )));
                        }
                        Ok((src, dst, edge_type, weight))
                    }
                    Err(e) => Err(e),
                };
                results.push(result);
                results
            }
            None => sorting_tmp
                .iter()
                .map(|((src, dst, edge_type), weight)| Ok((*src, *dst, *edge_type, *weight)))
                .collect::<Vec<_>>(),
        }))
}

pub(crate) fn parse_unsorted_quadruples(
    mut edges: Vec<Quadruple>,
    verbose: bool,
) -> (usize, impl Iterator<Item = Result<Quadruple, String>>) {
    info!("Sorting edges.");
    edges.par_sort_by(|(src1, dst1, edt1, _), (src2, dst2, edt2, _)| {
        (*src1, *dst1, *edt1).cmp(&(*src2, *dst2, *edt2))
    });

    println!("{:?}", edges);

    let edges_number = edges.len();
    let pb = get_loading_bar(verbose, "Building sorted graph", edges_number);

    (
        edges_number,
        edges.into_iter().progress_with(pb).map(Result::Ok),
    )
}

/// TODO: LUCA: I Think this method can be made better
pub(crate) fn parse_string_unsorted_edges<'a>(
    edges_iter: impl Iterator<Item = Result<StringQuadruple, String>>,
    mut nodes: Vocabulary<NodeT>,
    directed: bool,
    directed_edge_list: bool,
    edge_list_is_correct: bool,
    has_edge_types: bool,
    verbose: bool,
    numeric_edge_type_ids: bool,
) -> Result<
    (
        usize,
        impl Iterator<Item = Result<Quadruple, String>> + 'a,
        Vocabulary<NodeT>,
        Option<Vocabulary<EdgeTypeT>>,
    ),
    String,
> {
    let mut edge_types_vocabulary = if has_edge_types {
        Some(Vocabulary::default().set_numeric_ids(numeric_edge_type_ids))
    } else {
        None
    };
    let (edges_number, edges_iter) = {
        let edges_iter = parse_edges_node_ids(edges_iter, edge_list_is_correct, &mut nodes);
        let edges_iter: Box<dyn Iterator<Item = Result<Quadruple, String>>> =
            if let Some(ets) = &mut edge_types_vocabulary {
                Box::new(parse_edge_type_ids_vocabulary(
                    edges_iter,
                    edge_list_is_correct,
                    ets,
                ))
            } else {
                Box::new(edges_iter.map_ok(|(src, dst, _, weight)| (src, dst, None, weight)))
            };
        let edge_quadruples: Vec<Quadruple> = edges_iter
            .flat_map(|tuple| match tuple {
                Ok((src, dst, edt, weight)) => {
                    if !directed && src != dst && !directed_edge_list {
                        vec![Ok((src, dst, edt, weight)), Ok((dst, src, edt, weight))]
                    } else {
                        vec![Ok((src, dst, edt, weight))]
                    }
                }
                Err(e) => vec![Err(e)],
            })
            .collect::<Result<Vec<Quadruple>, String>>()?;

        parse_unsorted_quadruples(edge_quadruples, verbose)
    };
    info!("Building nodes reverse mapping.");
    nodes.build_reverse_mapping()?;
    if let Some(ets) = &mut edge_types_vocabulary {
        info!("Building edge types reverse mapping.");
        ets.build_reverse_mapping()?;
    }
    Ok((edges_number, edges_iter, nodes, edge_types_vocabulary))
}

/// Returns informations necessary to build a new EnsmallenGraph object.
///
/// # Arguments
/// edges_iter: impl Iterator<Item = Result<Quadruple, String>> - Iterator over edge informations.
/// edges_number: usize - Number of the edges in the graph. This information is needed both for evaluation of the graph properties and building elias-fano.
/// nodes_number: NodeT - Number of the nodes in the graph. This information is needed both for evaluation of the graph properties and building elias-fano.
/// ignore_duplicated_edges: bool - Whether to ignore duplicated edges while reading the graph.
/// has_edge_weights: bool - Whether the graph has weights.
/// has_edge_types: bool - Whether the graph has edge types.
/// might_contain_invalid_weights: bool - Whether we need to validate the weights.
/// might_contain_singletons: bool - Whether we need to expect singleton nodes. If the graph does not have singletons, we can build it faster.
/// might_contain_singletons_with_selfloops: bool - Whether we need to expect singleton nodes with selfloops. If the graph does not have singletons with selfloops, we can build it faster.
/// might_contain_trap_nodes: bool - Whether we need to expect trap nodes. If the graph does not have trap nodes we can build it faster.
/// directed: bool - Whether the graph is directed.
/// edge_list_is_correct: bool - Whether the edge list is correct and therefore we can skip validating it.
///
/// # Returned informations
/// The returned informations include:
/// - TODO! list.
pub(crate) fn build_edges(
    edges_iter: impl Iterator<Item = Result<Quadruple, String>>,
    edges_number: usize,
    nodes_number: NodeT,
    ignore_duplicated_edges: bool,
    has_edge_weights: bool,
    has_edge_types: bool,
    might_contain_invalid_weights: bool,
    might_contain_singletons: bool,
    might_contain_singletons_with_selfloops: bool,
    might_contain_trap_nodes: bool,
    directed: bool,
    edge_list_is_correct: bool,
) -> Result<
    (
        EliasFano,
        Option<EliasFano>,
        Option<Vec<Option<EdgeTypeT>>>,
        Option<Vec<WeightT>>,
        Option<WeightT>,
        Option<WeightT>,
        EdgeT,
        EdgeT,
        NodeT,
        NodeT,
        NodeT,
        u8,
        u64,
        Option<BitVec<Lsb0, u8>>,
        Option<RoaringBitmap>,
        NodeT,
        NodeT,
        Option<f64>,
        Option<f64>,
        Option<f64>,
        Option<NodeT>,
        bool,
        bool,
    ),
    String,
> {
    // Before starting to build the actual graph we execute
    // a sanity check of the arguments provided, as some may get complex
    // in some situations.

    // If the graph has a single edge and is directed, surely it must
    // contain trap nodes. The only exception is when the edge in question
    // is a self-loop, and it has been correctly parametrized that the
    // graph may contain singleton nodes with self-loops.
    if !might_contain_trap_nodes
        && !might_contain_singletons_with_selfloops
        && directed
        && edges_number == 1
    {
        panic!(concat!(
            "It has been specified that the graph cannot have trap ",
            "nodes, but is a directed graph with a single edge.\n",
            "Therefore, it must have a trap node and the parametrization ",
            "provided is mistaken.\n",
            "This often is caused by a misparametrization of a builder ",
            "method that was not expected to be able to build this case."
        ));
    }

    info!("Started building of EliasFano edges data structure.");
    let node_bits = get_node_bits(nodes_number);
    let node_bit_mask = (1 << node_bits) - 1;

    // Before checking any edge, we assume that the node IDs
    // are sorted by the outbound node degree.
    // The first flag will be used to check if it is in a decreasing order.
    let mut nodes_are_sorted_by_decreasing_outbound_node_degree: bool = true;
    // The first flag will be used to check if it is in a increasing order.
    let mut nodes_are_sorted_by_increasing_outbound_node_degree: bool = true;
    let mut edges: EliasFano =
        EliasFano::new(encode_max_edge(nodes_number, node_bits), edges_number)?;

    // The graph might still contain duplicated edges, therefore the provided edges
    // number is a maximal value.
    let mut edge_type_ids: Option<Vec<Option<EdgeTypeT>>> = if has_edge_types {
        Some(Vec::with_capacity(edges_number))
    } else {
        None
    };

    let (
        mut weights,
        mut min_edge_weight,
        mut max_edge_weight,
        mut min_weighted_node_degree,
        mut max_weighted_node_degree,
        mut total_weights,
        mut weighted_singleton_nodes_number,
        mut current_weighted_node_degree,
    ): (
        Option<Vec<WeightT>>,
        Option<WeightT>,
        Option<WeightT>,
        Option<f64>,
        Option<f64>,
        Option<f64>,
        Option<NodeT>,
        Option<f64>,
    ) = if has_edge_weights {
        (
            Some(Vec::with_capacity(edges_number)),
            Some(WeightT::INFINITY),
            Some(WeightT::NEG_INFINITY),
            Some(f64::INFINITY),
            Some(f64::NEG_INFINITY),
            Some(0.0),
            Some(0),
            Some(0.0),
        )
    } else {
        (None, None, None, None, None, None, None, None)
    };

    // The unique sources variable is equal to the set of nodes of the graph when
    // there are no singletons and the graph is undirected. Otherwise, if there is
    // a singleton node, that must not appear in this set.
    // We will use this set during the random walks and other graph algorithms
    // in order to obtain the nth source node. For this reason we cannot
    // use a bitvec here, since we need to execute an unchecked select when the
    // object is not equal to the set of the nodes to remap the nth source node
    // to the nth unique source node, excluding the eventual; singleton nodes.
    // Similarly, in directed graphs, only a subset of the nodes might be a
    // source as there might be traps.
    // In the case of directed graphs, we have additionally the might have trap nodes
    // parameter which allows to specify whether the graph is known to contain
    // trap nodes. The parameter only makes sense in directed graphs.
    // Since we expect that the average use case (when we arew not dealing with pathological graphs)
    // the following set should be relatively dense, when we know that the set of unique
    // sources will be needed but it will be equal to the nodes with edges set, we compute it
    // afterwards. This is because it is known that an Elias Fano data structure
    // uses more than twice the memory required by a bitvec to memorize a set of
    // dense values.
    let mut unique_sources: Option<EliasFano> =
        if directed && (might_contain_trap_nodes || might_contain_singletons) {
            Some(EliasFano::new(nodes_number as u64, nodes_number as usize)?)
        } else {
            None
        };
    // When the graph is either undirected or directed without trap nodes, the unique sources set and the
    // nodes with edges set are equal one another.
    // We need to compute the following set when it is not trivial, that is when
    // either the graph is undirected and there are no singletons or alternatively
    // when the graph is directed and there are neither trap nodes nor singletons.
    // Additionally, since we need this support data structure when computing the
    // number of singletons with selfloops, we need to create it also when it has
    // been specified that there might be singletons with selfloops.
    let mut connected_nodes: Option<_> = if (might_contain_singletons
        || might_contain_singletons_with_selfloops)
        && nodes_number > 0
    {
        Some(bitvec![Lsb0, u8; 0; nodes_number as usize])
    } else {
        None
    };

    // Last source inserted
    let mut last_src: NodeT = 0;
    let mut last_dst: NodeT = 0;
    let mut min_node_degree: NodeT = NodeT::MAX;
    let mut max_node_degree: NodeT = 0;
    let mut current_node_degree: NodeT = 0;
    let mut previous_node_degree: NodeT = 0;
    let mut last_edge_type: Option<EdgeTypeT> = None;
    let mut unique_edges_number: EdgeT = 0;
    let mut unique_selfloop_number: NodeT = 0;
    let mut selfloop_number: EdgeT = 0;
    let mut forward_undirected_edges_counter: EdgeT = 0;
    let mut backward_undirected_edges_counter: EdgeT = 0;
    let mut has_detect_singletons_or_trap_nodes: bool = false;
    let mut duplicated_edges_number: usize = 0;
    let mut connected_nodes_number: NodeT =
        if might_contain_singletons || might_contain_singletons_with_selfloops {
            0
        } else {
            nodes_number
        };
    // This bitvec should be really sparse ON SANE GRAPHS
    // so we use a roaring bitvec to save memory.
    let mut singleton_nodes_with_selfloops = if might_contain_singletons_with_selfloops {
        Some(RoaringBitmap::new())
    } else {
        None
    };

    let mut first = true;
    for value in edges_iter {
        let (src, dst, edge_type, weight) = value?;
        let different_src = last_src != src || first;
        let different_dst = last_dst != dst || first;
        let selfloop = src == dst;
        let different_edge_type = last_edge_type != edge_type || first;
        if !(different_src || different_dst || different_edge_type) {
            if ignore_duplicated_edges {
                duplicated_edges_number += 1;
                continue;
            } else {
                return Err("A duplicated edge was found while building the graph.".to_owned());
            }
        }

        // If this is the first source node and it is not zero,
        // there must be singletons before it, hence nodes with node
        // degree zero. Therefore, the node IDs cannot be sorted by
        // decreating outbound node degree.
        if first && src > 0 {
            nodes_are_sorted_by_decreasing_outbound_node_degree = false;
            has_detect_singletons_or_trap_nodes = true;
        }

        match (
            &mut weights,
            &mut min_edge_weight,
            &mut max_edge_weight,
            &mut total_weights,
            weight,
        ) {
            (Some(ws), Some(min_w), Some(max_w), Some(total_weights), Some(w)) => {
                if might_contain_invalid_weights {
                    // If a zero weight was found we filter out this edge
                    if w.is_zero() {
                        return Err("Provided weight is zero.".to_string());
                    }
                    if w.is_infinite() {
                        return Err("Provided weight is infinite.".to_string());
                    }
                    if w.is_nan() {
                        return Err("Provided weight is NaN.".to_string());
                    }
                }
                *min_w = (*min_w).min(w);
                *max_w = (*max_w).max(w);
                *total_weights += w as f64;
                ws.push(w);
                Ok(())
            }
            (None, _, _, _, Some(_)) => Err(concat!(
                "A non-None weight was provided but no weights are expected ",
                "because the has_edge_weights flag has been set to false."
            )),
            (Some(_), _, _, _, None) => Err(concat!(
                "A None weight was found.\n",
                "This might mean you have either provided a None weight to the edge list or ",
                "you may have an empty weight in your edge list file.\n",
                "If you intend to load this edge list WITHOUT weights, do not provide the ",
                "edge weights colum or column number.\n",
                "If you intend to load this edge with its weight, add a default weight."
            )),
            _ => Ok(()),
        }?;

        if let Some(ets) = &mut edge_type_ids {
            ets.push(edge_type);
        }

        if !directed && !edge_list_is_correct {
            match src.cmp(&dst) {
                Ordering::Greater => {
                    // We retrieve the edge id of the forward edge, the one going from
                    // dst to src.
                    let maybe_edge_id = edges.rank(encode_edge(dst, src, node_bits));
                    // Now we need to find, starting from edge id, if the edge types are given,
                    // the correct edge id: if we are in a multi-graph the edge may be the same
                    // but have multiple edge types and hence be reported multiple times.
                    let maybe_edge_id = maybe_edge_id.and_then(|min_edge_id| {
                        edge_type_ids.as_ref().map_or(Some(min_edge_id), |ets| {
                            (min_edge_id
                                ..edges.unchecked_rank(encode_edge(dst, src + 1, node_bits)))
                                .find(|edge_id| ets[*edge_id as usize] == edge_type)
                        })
                    });
                    if maybe_edge_id.is_none() {
                        return Err(concat!(
                            "You are trying to load an undirected ",
                            "graph using the directed edge list ",
                            "parameter that requires for ALL edges to ",
                            "be fully defined in both directions.\n",
                            "The edge list you have provided does not ",
                            "provide the edges in both directions.",
                        )
                        .to_string());
                    }
                    // Finally now we need to check if the weights of the two edges, if given
                    // are actually equal.
                    // For the time being we do not allow for undirected graphs to have
                    // asymmetrical weights.
                    let has_unbalanced_undirected_edge = maybe_edge_id.map_or(true, |edge_id| {
                        weights.as_ref().map_or(false, |ws| {
                            (ws[edge_id as usize] - weight.unwrap()).abs() >= f32::EPSILON
                        })
                    });
                    if has_unbalanced_undirected_edge {
                        return Err(concat!(
                            "You are trying to load an undirected ",
                            "graph using the directed edge list method ",
                            "with different weights in the different ",
                            "directions. To do so you need to load ",
                            "the graph as directed."
                        )
                        .to_string());
                    }
                    backward_undirected_edges_counter += 1
                }
                Ordering::Less => forward_undirected_edges_counter += 1,
                Ordering::Equal => {}
            }
        }
        last_edge_type = edge_type;
        edges.unchecked_push(encode_edge(src, dst, node_bits));
        if selfloop {
            selfloop_number += 1;
        }
        // If either the source node or the destination node in the
        // edge list has changed (keep in mind that the edge list
        // at this point is sorted)
        if different_src || different_dst {
            if let Some(nwe) = &mut connected_nodes {
                for node in &[src, dst] {
                    unsafe {
                        let mut ptr = nwe.get_unchecked_mut(*node as usize);
                        if !*ptr {
                            *ptr = true;
                            if !selfloop || singleton_nodes_with_selfloops.is_none() {
                                connected_nodes_number += 1;
                            } else {
                                if let Some(bitmap) = &mut singleton_nodes_with_selfloops {
                                    bitmap.insert(*node);
                                }
                                break;
                            }
                        } else if !selfloop
                            && singleton_nodes_with_selfloops
                                .as_mut()
                                .map_or(false, |bitmap| bitmap.remove(*node))
                        {
                            connected_nodes_number += 1;
                        }
                    }
                }
            }
            unique_edges_number += 1;
            if selfloop {
                unique_selfloop_number += 1;
            }
            // If the src has changed we need to update multiple things,
            // including the set of unique source nodes and the
            // minimum and maximum node degrees.
            if different_src {
                if let Some(us) = &mut unique_sources {
                    us.unchecked_push(src as u64);
                }
                // If it is not the first edge
                if !first {
                    // We update the minimum node degree
                    min_node_degree = min_node_degree.min(current_node_degree);
                    // And the maximum node degree
                    max_node_degree = max_node_degree.max(current_node_degree);
                    // Check if the node IDs are provided sorted by decreasing
                    // outbound node degree.
                    if previous_node_degree != 0 {
                        if previous_node_degree < current_node_degree {
                            nodes_are_sorted_by_decreasing_outbound_node_degree = false;
                        } else if previous_node_degree > current_node_degree {
                            nodes_are_sorted_by_increasing_outbound_node_degree = false;
                        }
                    }
                    // If there are more than one node skipped when changing
                    // the source node, since we are parsing a sorted edge list,
                    // that node must surely be a trap or a singleton.
                    // Either way, the node list is not sorted.
                    if src - last_src > 1 {
                        nodes_are_sorted_by_decreasing_outbound_node_degree = false;
                        nodes_are_sorted_by_increasing_outbound_node_degree = false;
                        has_detect_singletons_or_trap_nodes = true;
                    }
                    // Update the previous node degree
                    previous_node_degree = current_node_degree;
                    // And reset the current node degree to 0.
                    current_node_degree = 0;
                    // We update the weighted node degrees if the weights are provided
                    if let (
                        Some(min_weighted_node_degree),
                        Some(max_weighted_node_degree),
                        Some(weighted_singleton_nodes_number),
                        Some(current_weighted_node_degree),
                    ) = (
                        &mut min_weighted_node_degree,
                        &mut max_weighted_node_degree,
                        &mut weighted_singleton_nodes_number,
                        &mut current_weighted_node_degree,
                    ) {
                        *min_weighted_node_degree =
                            (*min_weighted_node_degree).min(*current_weighted_node_degree);
                        *max_weighted_node_degree =
                            (*max_weighted_node_degree).max(*current_weighted_node_degree);
                        if *current_weighted_node_degree == 0.0 {
                            *weighted_singleton_nodes_number += 1;
                        }
                        *current_weighted_node_degree = 0.0;
                    }
                }
            }
        }
        // We increase the current source node ID degree.
        current_node_degree += 1;
        // We increase the current source node ID weighted degree.
        if let Some(cwnd) = &mut current_weighted_node_degree {
            *cwnd += weight.unwrap() as f64;
        }

        last_src = src;
        last_dst = dst;
        first = false;
    }

    if forward_undirected_edges_counter != backward_undirected_edges_counter {
        return Err(concat!(
            "You are trying to load an undirected graph ",
            "from a directed edge list but the edge list is not ",
            "complete."
        )
        .to_owned());
    }

    // We need to update the minimum and maximum node degrees
    // for the last edge.

    // We update the minimum node degree
    if current_node_degree > 0 {
        min_node_degree = min_node_degree.min(current_node_degree);
    }
    // And the maximum node degree
    max_node_degree = max_node_degree.max(current_node_degree);

    // We check if the last node is sorted.
    if previous_node_degree != 0 {
        if previous_node_degree < current_node_degree {
            nodes_are_sorted_by_decreasing_outbound_node_degree = false;
        } else if previous_node_degree > current_node_degree {
            nodes_are_sorted_by_increasing_outbound_node_degree = false;
        }
    }

    // We do not do an equality check because it gets very hard (computationally)
    // to compute exactly how many edges a graph has, expecially when taking into
    // consideration cases including numerical instability, like during the
    // computation of Laplacian transformations of the given input graph.
    if edges.len() > edges_number - duplicated_edges_number {
        panic!(
            concat!(
                "The provided number of edges {} does not match the number of edges {} obtained after ",
                "building the Elias-Fano data structure."
            ),
            edges_number - duplicated_edges_number,
            edges.len()
        );
    }

    // If this is the last source node and it is not equal to the number
    // of nodes in the graph minus one
    // there must be singletons after it, hence nodes with node
    // degree zero. Therefore, the node IDs cannot be sorted by
    // increasing outbound node degree.
    // The only case where this is not the case, is when all the nodes
    // in the graph are singletons, and the graph does not have any edge.
    if !edges.is_empty() && last_src != nodes_number - 1 {
        nodes_are_sorted_by_increasing_outbound_node_degree = false;
        has_detect_singletons_or_trap_nodes = true;
    }

    if has_detect_singletons_or_trap_nodes
        && !(might_contain_singletons || might_contain_trap_nodes)
    {
        panic!(
            concat!(
                "It has been specified that within the graph we are currently trying to build ",
                "there are no singletons nor trap nodes, but nodes with outbound node degree zero ",
                "where found.\n",
                "The graph is {}.\n",
                "It is likely that this is caused by some constructor ",
                "misparametrization of either the might_contain_singletons parameter or the ",
                "might_contain_trap_nodes parameter.\n",
                "The last source node ID of the graph is {} and the nodes number is {}.\n",
                "The graph contains {} edges."
            ),
            if directed { "directed" } else { "undirected" },
            last_src,
            nodes_number,
            edges.len()
        );
    }

    // We update the minimum weighted node degree
    if let (
        Some(cwnd),
        Some(min_weighted_node_degree),
        Some(max_weighted_node_degree),
        Some(weighted_singleton_nodes_number),
    ) = (
        current_weighted_node_degree,
        &mut min_weighted_node_degree,
        &mut max_weighted_node_degree,
        &mut weighted_singleton_nodes_number,
    ) {
        if current_node_degree > 0 {
            *min_weighted_node_degree = (*min_weighted_node_degree).min(cwnd);
            if cwnd == 0.0 {
                *weighted_singleton_nodes_number += 1;
            }
        }
        // And the maximum weighted node degree
        *max_weighted_node_degree = (*max_weighted_node_degree).max(cwnd);
    }

    if !edges.is_empty() && max_node_degree == 0 {
        panic!("When the graph has at least an edge the maximum node degree cannot be zero.")
    }

    if let Some(ws) = &weights {
        if edges.len() != ws.len() {
            panic!(
                "The number of weights {} does not match the number of edges {}.",
                ws.len(),
                edges.len()
            );
        }
        if ws.is_empty() {
            weights = None;
            min_edge_weight = None;
            max_edge_weight = None;
            min_weighted_node_degree = None;
            max_weighted_node_degree = None;
            total_weights = None;
            weighted_singleton_nodes_number = None;
        }
    }

    if let Some(ets) = &edge_type_ids {
        if edges.len() != ets.len() {
            panic!(
                "The number of edge types {} does not match the number of edges {}.",
                ets.len(),
                edges.len()
            );
        }

        if ets.is_empty() {
            edge_type_ids = None;
        }
    }

    if connected_nodes_number > nodes_number {
        panic!(
            "There is an error in the constructor, the not singleton  node number '{}' is bigger than node number '{}'",
            connected_nodes_number, nodes_number
        );
    }

    let singleton_nodes_with_selfloops_number = singleton_nodes_with_selfloops
        .as_ref()
        .map_or(0, |bitmap| bitmap.len() as NodeT);

    // While on internal methods nodes_number is always exact, the user may
    // provide a wrong value for nodes_number when loading a sorted csv.
    // If this happens, it might cause a slow down in the walk and other
    // currently unforseen consequences.
    if unique_sources
        .as_ref()
        .map_or(false, |us| us.len() as NodeT == nodes_number)
    {
        unique_sources = None;
    }

    // When we have computed the nodes with edges set but we have left None
    // the unique sources elias fano, this is done to avoid using extra memory
    // for no reason. We need to create the elias fano object starting from the
    // nodes with edges now to normalize the returned values.
    if might_contain_singletons
        && unique_sources.is_none()
        && nodes_number != connected_nodes_number + singleton_nodes_with_selfloops_number
    {
        unique_sources = connected_nodes
            .as_ref()
            .map_or(Ok::<_, String>(None), |nsns| {
                Ok(Some(EliasFano::from_iter(
                    nsns.iter_ones().into_iter().map(|x| x as u64),
                    nodes_number as u64,
                    connected_nodes_number as usize
                        + singleton_nodes_with_selfloops_number as usize,
                )?))
            })?;
    }

    if !directed
        && unique_sources
            .as_ref()
            .map_or(false, |x| connected_nodes_number > x.len() as NodeT)
    {
        panic!(
            "There is an error in the constructor, the not singleton node number '{}' is bigger than the len of unique sources which is '{}'",
            connected_nodes_number, unique_sources.unwrap().len()
        );
    }

    if edges_number == 0 && nodes_number > 0 {
        assert!(unique_sources.is_some());
    }

    // If the singleton_nodes_with_selfloops bitmap if empty, we return a None instead.
    if singleton_nodes_with_selfloops
        .as_ref()
        .map_or(false, |bitmap| bitmap.is_empty())
    {
        singleton_nodes_with_selfloops = None;
    }

    assert_eq!(
        singleton_nodes_with_selfloops_number,
        singleton_nodes_with_selfloops
            .as_ref()
            .map_or(0, |x| x.len() as NodeT)
    );
    // If we have found singleton nodes, information that when there are
    // singleton nodes we know only after parsing both the node list and the
    // edge list, we need to update the min node degree to 0.
    if nodes_number > connected_nodes_number + singleton_nodes_with_selfloops_number
        // this check is needed only if the graph is directed to verify if it 
        // has trap nodes, when there are trapnodes the number of unique sources
        // is less than the number of nodes
        || unique_sources
            .as_ref()
            .map_or(false, |us| (us.len() as NodeT) < nodes_number)
    {
        min_node_degree = 0;
        min_weighted_node_degree = min_weighted_node_degree.map(|val| val.min(0.0));
        max_weighted_node_degree = max_weighted_node_degree.map(|val| val.max(0.0));
    }

    if !directed
        && !edges.is_empty()
        && nodes_number == connected_nodes_number + singleton_nodes_with_selfloops_number
        && min_node_degree == 0
    {
        panic!(concat!(
            "When the graph is undirected and has at least an edge ",
            "and there are no singletons, ",
            "the minimum node degree cannot be zero."
        ));
    }

    if edges.len() < (nodes_number / 2) as usize && min_node_degree != 0 {
        panic!(
            concat!(
                "When the graph has less than N/2 edges, ",
                "it must contain singletons.\n",
                "This error is likely caused by an improper use of the ",
                "`might_contain_singletons` parameters, which was passed as ",
                "{}."
            ),
            might_contain_singletons
        );
    }

    Ok((
        edges,
        unique_sources,
        edge_type_ids,
        weights,
        min_edge_weight,
        max_edge_weight,
        unique_edges_number,
        selfloop_number,
        unique_selfloop_number,
        connected_nodes_number,
        singleton_nodes_with_selfloops_number,
        node_bits,
        node_bit_mask,
        connected_nodes,
        singleton_nodes_with_selfloops,
        min_node_degree,
        max_node_degree,
        min_weighted_node_degree,
        max_weighted_node_degree,
        total_weights,
        weighted_singleton_nodes_number,
        nodes_are_sorted_by_decreasing_outbound_node_degree,
        nodes_are_sorted_by_increasing_outbound_node_degree,
    ))
}

fn parse_nodes(
    nodes_iterator: Option<impl Iterator<Item = Result<(String, Option<Vec<String>>), String>>>,
    ignore_duplicated_nodes: bool,
    node_list_is_correct: bool,
    numeric_node_ids: bool,
    numeric_node_types_ids: bool,
    numeric_edge_node_ids: bool,
    has_node_types: bool,
) -> Result<(Vocabulary<NodeT>, Option<NodeTypeVocabulary>), String> {
    let mut nodes = Vocabulary::default()
        .set_numeric_ids(numeric_node_ids || numeric_edge_node_ids && nodes_iterator.is_none());

    let node_types = if let Some(ni) = nodes_iterator {
        // TODO: the following can likely be dealt with in a better way.
        let node_iterator = parse_node_ids(
            ni,
            ignore_duplicated_nodes,
            node_list_is_correct,
            &mut nodes,
        );
        // In the case there is a node types we need to add its proper iterator.
        if has_node_types {
            let mut node_types =
                NodeTypeVocabulary::default().set_numeric_ids(numeric_node_types_ids);
            for row in parse_node_type_ids(node_iterator, node_list_is_correct, &mut node_types) {
                row?;
            }
            node_types.build_reverse_mapping()?;
            node_types.build_counts();

            if node_types.is_empty() {
                Ok(None)
            } else {
                Ok::<_, String>(Some(node_types))
            }
        } else {
            for row in node_iterator {
                row?;
            }
            Ok::<_, String>(None)
        }?
    } else {
        None
    };

    Ok((nodes, node_types))
}

// TODO!: add docstring
pub(crate) fn parse_string_edges(
    edges_iter: impl Iterator<Item = Result<StringQuadruple, String>>,
    edges_number: usize,
    nodes_number: NodeT,
    directed: bool,
    mut nodes: Vocabulary<NodeT>,
    numeric_edge_type_ids: bool,
    directed_edge_list: bool,
    edge_list_is_correct: bool,
    ignore_duplicated_edges: bool,
    has_edge_types: bool,
    has_edge_weights: bool,
    might_contain_invalid_weights: bool,
    might_contain_singletons: bool,
    might_contain_singletons_with_selfloops: bool,
    might_contain_trap_nodes: bool,
) -> ParsedStringEdgesType {
    let mut edge_types_vocabulary: Vocabulary<EdgeTypeT> =
        Vocabulary::default().set_numeric_ids(numeric_edge_type_ids);

    // This is not equivalent to nodes_iterator.is_some() because the iterator
    // could also be empty, this is a corner-case that might happen when over-filtering
    // or fuzzing or loading an empty file with improper configurations.
    // There might be singletons if the user has told us that there might be singletons
    // and the node list is not empty. If the node list is empty, then it is not possible
    // to have singletons.
    let might_contain_singletons = !nodes.is_empty() && might_contain_singletons;
    // If the graph is undirected there cannot be trap nodes
    let might_contain_trap_nodes = directed && might_contain_trap_nodes;

    let edges_iter = parse_sorted_edges(
        parse_edge_type_ids_vocabulary(
            parse_edges_node_ids(edges_iter, edge_list_is_correct, &mut nodes),
            edge_list_is_correct,
            &mut edge_types_vocabulary,
        ),
        directed,
        directed_edge_list,
    );

    let (
        edges,
        unique_sources,
        edge_type_ids,
        weights,
        min_edge_weight,
        max_edge_weight,
        unique_edges_number,
        selfloop_number,
        unique_selfloop_number,
        connected_nodes_number,
        singleton_nodes_with_selfloops_number,
        node_bits,
        node_bit_mask,
        connected_nodes,
        singleton_nodes_with_selfloops,
        min_node_degree,
        max_node_degree,
        min_weighted_node_degree,
        max_weighted_node_degree,
        total_weights,
        weighted_singleton_nodes_number,
        nodes_are_sorted_by_decreasing_outbound_node_degree,
        nodes_are_sorted_by_increasing_outbound_node_degree,
    ) = build_edges(
        edges_iter,
        edges_number,
        nodes_number,
        ignore_duplicated_edges,
        has_edge_weights,
        has_edge_types,
        might_contain_invalid_weights,
        might_contain_singletons,
        might_contain_singletons_with_selfloops,
        might_contain_trap_nodes,
        directed,
        edge_list_is_correct,
    )?;

    nodes.build_reverse_mapping()?;
    edge_types_vocabulary.build_reverse_mapping()?;
    let edge_types =
        EdgeTypeVocabulary::from_option_structs(edge_type_ids, optionify!(edge_types_vocabulary));

    Ok((
        edges,
        unique_sources,
        nodes,
        edge_types,
        weights,
        min_edge_weight,
        max_edge_weight,
        unique_edges_number,
        selfloop_number,
        unique_selfloop_number,
        connected_nodes_number,
        singleton_nodes_with_selfloops_number,
        node_bit_mask,
        node_bits,
        connected_nodes,
        singleton_nodes_with_selfloops,
        min_node_degree,
        max_node_degree,
        min_weighted_node_degree,
        max_weighted_node_degree,
        total_weights,
        weighted_singleton_nodes_number,
        nodes_are_sorted_by_decreasing_outbound_node_degree,
        nodes_are_sorted_by_increasing_outbound_node_degree,
    ))
}

/// TODO: add docstring
pub(crate) fn parse_integer_edges(
    edges_iter: impl Iterator<Item = Result<Quadruple, String>>,
    edges_number: usize,
    nodes_number: NodeT,
    edge_types_vocabulary: Option<Vocabulary<EdgeTypeT>>,
    ignore_duplicated_edges: bool,
    directed: bool,
    edge_list_is_correct: bool,
    has_edge_types: bool,
    has_edge_weights: bool,
    might_contain_invalid_weights: bool,
    might_contain_singletons: bool,
    might_contain_singletons_with_selfloops: bool,
    might_contain_trap_nodes: bool,
) -> Result<
    (
        EliasFano,
        Option<EliasFano>,
        Option<EdgeTypeVocabulary>,
        Option<Vec<WeightT>>,
        Option<WeightT>,
        Option<WeightT>,
        EdgeT,
        EdgeT,
        NodeT,
        NodeT,
        NodeT,
        u64,
        u8,
        Option<BitVec<Lsb0, u8>>,
        Option<RoaringBitmap>,
        NodeT,
        NodeT,
        Option<f64>,
        Option<f64>,
        Option<f64>,
        Option<NodeT>,
        bool,
        bool,
    ),
    String,
> {
    let (
        edges,
        unique_sources,
        edge_type_ids,
        weights,
        min_edge_weight,
        max_edge_weight,
        unique_edges_number,
        selfloop_number,
        unique_selfloop_number,
        connected_nodes_number,
        singleton_nodes_with_selfloops_number,
        node_bits,
        node_bit_mask,
        connected_nodes,
        singleton_nodes_with_selfloops,
        min_node_degree,
        max_node_degree,
        min_weighted_node_degree,
        max_weighted_node_degree,
        total_weights,
        weighted_singleton_nodes_number,
        nodes_are_sorted_by_decreasing_outbound_node_degree,
        nodes_are_sorted_by_increasing_outbound_node_degree,
    ) = build_edges(
        edges_iter,
        edges_number,
        nodes_number,
        ignore_duplicated_edges,
        has_edge_weights,
        has_edge_types,
        might_contain_invalid_weights,
        might_contain_singletons,
        might_contain_singletons_with_selfloops,
        might_contain_trap_nodes,
        directed,
        edge_list_is_correct,
    )?;

    let edge_types = EdgeTypeVocabulary::from_option_structs(edge_type_ids, edge_types_vocabulary);

    Ok((
        edges,
        unique_sources,
        edge_types,
        weights,
        min_edge_weight,
        max_edge_weight,
        unique_edges_number,
        selfloop_number,
        unique_selfloop_number,
        connected_nodes_number,
        singleton_nodes_with_selfloops_number,
        node_bit_mask,
        node_bits,
        connected_nodes,
        singleton_nodes_with_selfloops,
        min_node_degree,
        max_node_degree,
        min_weighted_node_degree,
        max_weighted_node_degree,
        total_weights,
        weighted_singleton_nodes_number,
        nodes_are_sorted_by_decreasing_outbound_node_degree,
        nodes_are_sorted_by_increasing_outbound_node_degree,
    ))
}

/// # Graph Constructors
impl Graph {
    pub(crate) fn from_integer_sorted<S: Into<String>>(
        edges_iter: impl Iterator<Item = Result<Quadruple, String>>,
        edges_number: usize,
        nodes: Vocabulary<NodeT>,
        node_types: Option<NodeTypeVocabulary>,
        edge_types_vocabulary: Option<Vocabulary<EdgeTypeT>>,
        directed: bool,
        edge_list_is_correct: bool,
        name: S,
        ignore_duplicated_edges: bool,
        has_edge_types: bool,
        has_edge_weights: bool,
        might_contain_invalid_weights: bool,
        might_contain_singletons: bool,
        might_contain_singletons_with_selfloops: bool,
        might_contain_trap_nodes: bool,
    ) -> Result<Graph, String> {
        let (
            edges,
            unique_sources,
            edge_types,
            weights,
            min_edge_weight,
            max_edge_weight,
            unique_edges_number,
            selfloop_number,
            unique_selfloop_number,
            connected_nodes_number,
            singleton_nodes_with_selfloops_number,
            node_bit_mask,
            node_bits,
            connected_nodes,
            singleton_nodes_with_selfloops,
            min_node_degree,
            max_node_degree,
            min_weighted_node_degree,
            max_weighted_node_degree,
            total_weights,
            weighted_singleton_nodes_number,
            nodes_are_sorted_by_decreasing_outbound_node_degree,
            nodes_are_sorted_by_increasing_outbound_node_degree,
        ) = parse_integer_edges(
            edges_iter,
            edges_number,
            nodes.len() as NodeT,
            edge_types_vocabulary,
            ignore_duplicated_edges,
            directed,
            edge_list_is_correct,
            has_edge_types,
            has_edge_weights,
            might_contain_invalid_weights,
            might_contain_singletons,
            might_contain_singletons_with_selfloops,
            might_contain_trap_nodes,
        )?;

        Ok(Graph::new(
            directed,
            unique_selfloop_number,
            selfloop_number,
            connected_nodes_number,
            singleton_nodes_with_selfloops_number,
            unique_edges_number,
            edges,
            unique_sources,
            nodes,
            node_bit_mask,
            node_bits,
            edge_types,
            name,
            weights,
            min_edge_weight,
            max_edge_weight,
            node_types,
            connected_nodes,
            singleton_nodes_with_selfloops,
            min_node_degree,
            max_node_degree,
            min_weighted_node_degree,
            max_weighted_node_degree,
            total_weights,
            weighted_singleton_nodes_number,
            nodes_are_sorted_by_decreasing_outbound_node_degree,
            nodes_are_sorted_by_increasing_outbound_node_degree,
        ))
    }

    /// Create new Graph object from unsorted source.
    ///
    /// # Arguments
    /// * `edges_iterator`: impl Iterator<Item = Result<StringQuadruple, String>> - Iterator on the edge list composed of strings.
    /// * `nodes_iterator`: Option<impl Iterator<Item = Result<(String, Option<Vec<String>>), String>>> - Option of an iterator on the node list composed of strings.
    /// * `directed`: bool - Whether the edge list should be loaded as directed or undirected.
    /// * `directed_edge_list`: bool - Whether the edge list is expressed as directed. If this is an undirected graph, we expect a directed edge list to contain edges in BOTH directions.
    /// * `name`: S - The name of the graph.
    /// * `ignore_duplicated_nodes`: bool - Whether to ignore and skip the detected duplicated node names or to raise an error.
    /// * `node_list_is_correct`: bool - Whether the user pinky promises that the node list is correct. This feature will lead to panics if used improperly by an over-optimistic user. Enable this flag only if you are sure you are correct.
    /// * `ignore_duplicated_edges`: bool - Whether to ignore and skip the detected duplicated edges or to raise an error.
    /// * `edge_list_is_correct`: bool - Whether the user pinky promises that the edge list is correct. This feature will lead to panics if used improperly by an over-optimistic user. Enable this flag only if you are sure you are correct.
    /// * `numeric_edge_type_ids`: bool - Whether the edge type IDs should be loaded as numeric, casting them to integers. The range of edge type IDs MUST be dense.
    /// * `numeric_node_ids`: bool - Whether the node IDs should be loaded as numeric, casting them to integers. The range of node IDs MUST be dense.
    /// * `numeric_edge_node_ids`: bool - Whether the edge node IDs should be loaded as numeric, casting them to integers. The range of edge node IDs MUST be dense.
    /// * `numeric_node_types_ids`: bool - Whether the node type IDs should be loaded as numeric, casting them to integers. The range of node type IDs MUST be dense.
    /// * `has_node_types`: bool - Whether the graph has node types.
    /// * `has_edge_types`: bool - Whether the graph has edge types.
    /// * `has_edge_weights`: bool - Whether the graph has edge weights.
    /// * `might_contain_singletons`: bool - Whether the graph is KNOWN to have or not singleton nodes. Beware that improper use of this flag might lead to panics. Enable this flag only if you are sure you are correct.
    /// * `might_contain_singletons_with_selfloops`: bool - Whether the graph is KNOWN to have or not singleton nodes with selfloops. Beware that improper use of this flag might lead to panics. Enable this flag only if you are sure you are correct.
    /// * `might_contain_trap_nodes`: bool - Whether the graph is KNOWN to have or not trap nodes. Beware that improper use of this flag might lead to panics. Enable this flag only if you are sure you are correct.
    /// * `verbose`: bool - Whether we should show loading bars while building the graph.
    pub fn from_string_unsorted<S: Into<String>>(
        edges_iterator: impl Iterator<Item = Result<StringQuadruple, String>>,
        nodes_iterator: Option<impl Iterator<Item = Result<(String, Option<Vec<String>>), String>>>,
        directed: bool,
        directed_edge_list: bool,
        name: S,
        ignore_duplicated_nodes: bool,
        node_list_is_correct: bool,
        ignore_duplicated_edges: bool,
        edge_list_is_correct: bool,
        numeric_edge_type_ids: bool,
        numeric_node_ids: bool,
        numeric_edge_node_ids: bool,
        numeric_node_types_ids: bool,
        has_node_types: bool,
        has_edge_types: bool,
        has_edge_weights: bool,
        might_contain_invalid_weights: bool,
        might_contain_singletons: bool,
        might_contain_singletons_with_selfloops: bool,
        might_contain_trap_nodes: bool,
        verbose: bool,
    ) -> Result<Graph, String> {
        check_numeric_ids_compatibility(
            nodes_iterator.is_some(),
            numeric_node_ids,
            numeric_edge_node_ids,
        )?;

        let (nodes, node_types) = parse_nodes(
            nodes_iterator,
            ignore_duplicated_nodes,
            node_list_is_correct,
            numeric_node_ids,
            numeric_node_types_ids,
            numeric_edge_node_ids,
            has_node_types,
        )?;

        // This is not equivalent to nodes_iterator.is_some() because the iterator
        // could also be empty, this is a corner-case that might happen when over-filtering
        // or fuzzing or loading an empty file with improper configurations.
        // There might be singletons if the user has told us that there might be singletons
        // and the node list is not empty. If the node list is empty, then it is not possible
        // to have singletons.
        let might_contain_singletons = !nodes.is_empty() && might_contain_singletons;
        // If the graph is undirected there cannot be trap nodes
        let might_contain_trap_nodes = directed && might_contain_trap_nodes;

        info!("Parse unsorted edges.");
        let (edges_number, edges_iterator, nodes, edge_types_vocabulary) =
            parse_string_unsorted_edges(
                edges_iterator,
                nodes,
                directed,
                directed_edge_list,
                edge_list_is_correct,
                has_edge_types,
                verbose,
                numeric_edge_type_ids,
            )?;

        Graph::from_integer_sorted(
            edges_iterator,
            edges_number,
            nodes,
            node_types,
            edge_types_vocabulary,
            directed,
            edge_list_is_correct || !directed_edge_list,
            name,
            ignore_duplicated_edges,
            has_edge_types,
            has_edge_weights,
            might_contain_invalid_weights,
            might_contain_singletons,
            might_contain_singletons_with_selfloops,
            might_contain_trap_nodes,
        )
    }

    /// Create new Graph object from unsorted source.
    ///
    /// # Arguments
    /// * `edges_iterator`: impl Iterator<Item = Result<(NodeT, NodeT, Option<NodeTypeT>, Option<WeightT>), String>> - Iterator over the egde node IDs.
    /// * `nodes`: Vocabulary<NodeT> - Vocabulary of the node IDs.
    /// * `node_types`: Option<NodeTypeVocabulary> - Option of the vocabulary of the node type IDs.
    /// * `edge_types_vocabulary`: Option<Vocabulary<EdgeTypeT>> - Option of the Vocabulary of the edge type IDs.
    /// * `directed`: bool - Whether to load the graph as directed or undirected.
    /// * `name`: String - Name of the graph.
    /// * `ignore_duplicated_edges`: bool - Whether to ignore and skip the detected duplicated edges or to raise an error.
    /// * `has_edge_types`: bool - Whether the graph has edge types.
    /// * `has_edge_weights`: bool - Whether the graph has edge weights.
    /// * `might_contain_singletons`: bool - Whether the graph is KNOWN to have or not singleton nodes. Beware that improper use of this flag might lead to panics. Enable this flag only if you are sure you are correct.
    /// * `might_contain_singletons_with_selfloops`: bool - Whether the graph is KNOWN to have or not singleton nodes with selfloops. Beware that improper use of this flag might lead to panics. Enable this flag only if you are sure you are correct.
    /// * `might_contain_trap_nodes`: bool - Whether the graph is KNOWN to have or not trap nodes. Beware that improper use of this flag might lead to panics. Enable this flag only if you are sure you are correct.
    /// * `verbose`: bool - Whether to show theloading bars while loading the graph.
    pub fn from_integer_unsorted(
        edges_iterator: impl ParallelIterator<
            Item = Result<(NodeT, NodeT, Option<NodeTypeT>, Option<WeightT>), String>,
        >,
        nodes: Vocabulary<NodeT>,
        node_types: Option<NodeTypeVocabulary>,
        edge_types_vocabulary: Option<Vocabulary<EdgeTypeT>>,
        directed: bool,
        name: String,
        ignore_duplicated_edges: bool,
        has_edge_types: bool,
        has_edge_weights: bool,
        might_contain_invalid_weights: bool,
        might_contain_singletons: bool,
        might_contain_singletons_with_selfloops: bool,
        might_contain_trap_nodes: bool,
        verbose: bool,
    ) -> Result<Graph, String> {
        let (edges_number, edges_iterator) =
            parse_unsorted_quadruples(edges_iterator.collect::<Result<Vec<_>, String>>()?, verbose);

        Graph::from_integer_sorted(
            edges_iterator,
            edges_number,
            nodes,
            node_types,
            edge_types_vocabulary,
            directed,
            true,
            name,
            ignore_duplicated_edges,
            has_edge_types,
            has_edge_weights,
            might_contain_invalid_weights,
            might_contain_singletons,
            might_contain_singletons_with_selfloops,
            might_contain_trap_nodes,
        )
    }

    /// Create new Graph object from sorted sources.
    ///
    /// # Arguments
    /// * `edges_iterator`: impl Iterator<Item = Result<StringQuadruple, String>> - Iterator on the edge list composed of strings.
    /// * `nodes_iterator`: Option<impl Iterator<Item = Result<(String, Option<Vec<String>>), String>>> - Option of an iterator on the node list composed of strings.
    /// * `directed`: bool - Whether the edge list should be loaded as directed or undirected.
    /// * `directed_edge_list`: bool - Whether the edge list is expressed as directed. If this is an undirected graph, we expect a directed edge list to contain edges in BOTH directions.
    /// * `name`: S - The name of the graph.
    /// * `ignore_duplicated_nodes`: bool - Whether to ignore and skip the detected duplicated node names or to raise an error.
    /// * `node_list_is_correct`: bool - Whether the user pinky promises that the node list is correct. This feature will lead to panics if used improperly by an over-optimistic user. Enable this flag only if you are sure you are correct.
    /// * `ignore_duplicated_edges`: bool - Whether to ignore and skip the detected duplicated edges or to raise an error.
    /// * `edge_list_is_correct`: bool - Whether the user pinky promises that the edge list is correct. This feature will lead to panics if used improperly by an over-optimistic user. Enable this flag only if you are sure you are correct.
    /// * `edges_number`: usize - Exact number of edges in the graph.
    /// * `nodes_number`: NodeT - Exact number of nodes in the graph.
    /// * `numeric_edge_type_ids`: bool - Whether the edge type IDs should be loaded as numeric, casting them to integers. The range of edge type IDs MUST be dense.
    /// * `numeric_node_ids`: bool - Whether the node IDs should be loaded as numeric, casting them to integers. The range of node IDs MUST be dense.
    /// * `numeric_edge_node_ids`: bool - Whether the edge node IDs should be loaded as numeric, casting them to integers. The range of edge node IDs MUST be dense.
    /// * `numeric_node_types_ids`: bool - Whether the node type IDs should be loaded as numeric, casting them to integers. The range of node type IDs MUST be dense.
    /// * `has_node_types`: bool - Whether the graph has node types.
    /// * `has_edge_types`: bool - Whether the graph has edge types.
    /// * `has_edge_weights`: bool - Whether the graph has edge weights.
    /// * `might_contain_invalid_weights`: bool - Whether the graph may contain or not invalid weights. Beware that improper use of this flag might lead to panics. Enable this flag only if you are sure you are correct.
    /// * `might_contain_singletons`: bool - Whether the graph may contain or not singleton nodes. Beware that improper use of this flag might lead to panics. Enable this flag only if you are sure you are correct.
    /// * `might_contain_singletons_with_selfloops`: bool - Whether the graph may contain or not singleton nodes with selfloops. Beware that improper use of this flag might lead to panics. Enable this flag only if you are sure you are correct.
    /// * `might_contain_trap_nodes`: bool - Whether the graph may contain or not trap nodes. Beware that improper use of this flag might lead to panics. Enable this flag only if you are sure you are correct.
    pub fn from_string_sorted<S: Into<String>>(
        edges_iterator: impl Iterator<Item = Result<StringQuadruple, String>>,
        nodes_iterator: Option<impl Iterator<Item = Result<(String, Option<Vec<String>>), String>>>,
        directed: bool,
        directed_edge_list: bool,
        name: S,
        ignore_duplicated_nodes: bool,
        node_list_is_correct: bool,
        ignore_duplicated_edges: bool,
        edge_list_is_correct: bool,
        edges_number: usize,
        mut nodes_number: NodeT,
        numeric_edge_type_ids: bool,
        numeric_node_ids: bool,
        numeric_edge_node_ids: bool,
        numeric_node_types_ids: bool,
        has_node_types: bool,
        has_edge_types: bool,
        has_edge_weights: bool,
        might_contain_invalid_weights: bool,
        might_contain_singletons: bool,
        might_contain_singletons_with_selfloops: bool,
        might_contain_trap_nodes: bool,
    ) -> Result<Graph, String> {
        check_numeric_ids_compatibility(
            nodes_iterator.is_some(),
            numeric_node_ids,
            numeric_edge_node_ids,
        )?;
        let (nodes, node_types) = parse_nodes(
            nodes_iterator,
            ignore_duplicated_nodes,
            node_list_is_correct,
            numeric_node_ids,
            numeric_node_types_ids,
            numeric_edge_node_ids,
            has_node_types,
        )?;

        if !nodes.is_empty() {
            nodes_number = nodes.len() as NodeT;
        }

        let (
            edges,
            unique_sources,
            nodes,
            edge_types,
            weights,
            min_edge_weight,
            max_edge_weight,
            unique_edges_number,
            selfloop_number,
            unique_selfloop_number,
            connected_nodes_number,
            singleton_nodes_with_selfloops_number,
            node_bit_mask,
            node_bits,
            connected_nodes,
            singleton_nodes_with_selfloops,
            min_node_degree,
            max_node_degree,
            min_weighted_node_degree,
            max_weighted_node_degree,
            total_weights,
            weighted_singleton_nodes_number,
            nodes_are_sorted_by_decreasing_outbound_node_degree,
            nodes_are_sorted_by_increasing_outbound_node_degree,
        ) = parse_string_edges(
            edges_iterator,
            edges_number,
            nodes_number,
            directed,
            nodes,
            numeric_edge_type_ids,
            directed_edge_list,
            edge_list_is_correct,
            ignore_duplicated_edges,
            has_edge_types,
            has_edge_weights,
            might_contain_invalid_weights,
            might_contain_singletons,
            might_contain_singletons_with_selfloops,
            might_contain_trap_nodes,
        )?;

        Ok(Graph::new(
            directed,
            unique_selfloop_number,
            selfloop_number,
            connected_nodes_number,
            singleton_nodes_with_selfloops_number,
            unique_edges_number,
            edges,
            unique_sources,
            nodes,
            node_bit_mask,
            node_bits,
            edge_types,
            name,
            weights,
            min_edge_weight,
            max_edge_weight,
            node_types,
            connected_nodes,
            singleton_nodes_with_selfloops,
            min_node_degree,
            max_node_degree,
            min_weighted_node_degree,
            max_weighted_node_degree,
            total_weights,
            weighted_singleton_nodes_number,
            nodes_are_sorted_by_decreasing_outbound_node_degree,
            nodes_are_sorted_by_increasing_outbound_node_degree,
        ))
    }
}
