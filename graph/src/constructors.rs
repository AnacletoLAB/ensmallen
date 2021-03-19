use super::*;
use elias_fano_rust::EliasFano;
use indicatif::ProgressIterator;
use bitvec::prelude::*;
use std::cmp::Ordering;
use rayon::prelude::ParallelSliceMut;
use std::collections::BTreeMap;
use itertools::Itertools;
use log::info;

type ParsedStringEdgesType = Result<
    (
        EliasFano,
        EliasFano,
        Vocabulary<NodeT>,
        Option<EdgeTypeVocabulary>,
        Vec<WeightT>,
        EdgeT,
        EdgeT,
        NodeT,
        NodeT,
        NodeT,
        u64,
        u8,
    ),
    String,
>;

#[macro_export]
/// Take a vector and make it a None if its empty, Some(vector) otherwise
macro_rules! optionify {
    ($val:expr) => {
        if $val.is_empty() {
            None
        } else {
            Some($val)
        }
    };
}

/// Returns iterator of nodes handling the node IDs.
pub(crate) fn parse_node_ids<'a, 'b>(
    nodes_iter: impl Iterator<Item = Result<(String, Option<Vec<String>>), String>> + 'a,
    ignore_duplicated_nodes: bool,
    nodes: &'b mut Vocabulary<NodeT>,
) -> impl Iterator<Item = Result<(NodeT, Option<Vec<String>>), String>> + 'a
where
    'b: 'a,
{
    nodes_iter.filter_map(move |row|{
        match row{
            Ok((node_name, node_type)) =>  {
                if node_name.is_empty() {
                    return Some(Err("Found an empty node name. Node names cannot be empty.".to_owned()))
                }
                if let Some(node_type_string) = &node_type{
                    if node_type_string.is_empty() {
                        return Some(Err("Found an empty node type name. Node type names cannot be empty.".to_owned()))
                    }
                }
                match nodes.get(&node_name){
                Some(_) => {
                    if ignore_duplicated_nodes {
                        None
                    } else {
                        Some(Err(format!(
                            concat!(
                                "\nFound duplicated nodes!\n",
                                "The node is {node_name}.\n",
                                "The node type of the row is {node_type:?}.\n",
                                "The library does not currently support multiple node types for a single node."
                            ),
                            node_name = node_name,
                            node_type = node_type
                        )))
                    }
                },
                None=>{
                    Some(match nodes.insert(node_name){
                        Ok(node_id) => Ok((node_id, node_type)),
                        Err(e) => Err(e)
                    })
                }
            }},
            Err(e) => Some(Err(e))
        }
    })
}

/// Returns iterator of nodes handling the node type IDs.
pub(crate) fn parse_node_type_ids<'a, 'b>(
    nodes_iter: impl Iterator<Item = Result<(NodeT, Option<Vec<String>>), String>> + 'a,
    node_types_vocabulary: &'b mut NodeTypeVocabulary,
) -> impl Iterator<Item = Result<(NodeT, Option<Vec<NodeTypeT>>), String>> + 'a
where
    'b: 'a,
{
    nodes_iter.map(move |row| match row {
        Ok((node_id, node_types)) => {
            Ok((node_id, node_types_vocabulary.insert_values(node_types)?))
        }
        Err(e) => Err(e),
    })
}

pub(crate) fn parse_edges_node_ids<'a, 'b>(
    edges_iterator: impl Iterator<Item = Result<StringQuadruple, String>> + 'a,
    nodes: &'b mut Vocabulary<NodeT>,
) -> impl Iterator<Item = Result<(NodeT, NodeT, Option<String>, Option<WeightT>), String>> + 'a
where
    'b: 'a,
{
    let empty_nodes_mapping = nodes.is_empty();
    edges_iterator.map(move |row: Result<StringQuadruple, String>| match row {
        Ok((src_name, dst_name, edge_type, weight)) => {
            for node_name in [src_name.clone(), dst_name.clone()].iter() {
                if node_name.is_empty() {
                    return Err("Found an empty node name. Node names cannot be empty.".to_owned());
                }
                if !nodes.contains_key(node_name) {
                    if empty_nodes_mapping {
                        nodes.insert(node_name.to_owned())?;
                    } else if !nodes.numeric_ids {
                        return Err(format!(
                            concat!(
                                "In the edge file was found the node {} ",
                                "which is not present in the given node file."
                            ),
                            node_name
                        ));
                    }
                }
            }
            if let Some(edge_type_string) = &edge_type{
                if edge_type_string.is_empty() {
                    return Err("Found an empty edge type name. Edge type names cannot be empty.".to_owned());
                }
            }
            Ok((
                match nodes.numeric_ids{
                    true => match src_name.parse::<NodeT>() {
                        Ok(val) => val,
                        Err(_) => {return Err(format!("The given source node ID `{}` is not numeric.", src_name));},
                    },
                    false => *nodes.get(&src_name).unwrap()
                },
                match nodes.numeric_ids{
                    true => match dst_name.parse::<NodeT>() {
                        Ok(val) => val,
                        Err(_) => {return Err(format!("The given destination node ID `{}` is not numeric.", dst_name));},
                    },
                    false => *nodes.get(&dst_name).unwrap()
                },
                edge_type,
                weight,
            ))
        }
        Err(e) => Err(e),
    })
}

/// Returns iterator of edges handling the edge type IDs.
pub(crate) fn parse_edge_type_ids_vocabulary<'a, 'b>(
    edges_iter: impl Iterator<Item = Result<(NodeT, NodeT, Option<String>, Option<WeightT>), String>>
        + 'a,
    edge_types: &'b mut Vocabulary<EdgeTypeT>,
) -> impl Iterator<Item = Result<Quadruple, String>> + 'a
    where 'b: 'a,{
    edges_iter.map(move |row| match row {
        Ok((src, dst, edge_type, weight)) => {
            let edge_type_id = match edge_type {
                Some(et) => Some(edge_types.insert(et)?),
                None => None,
            };
            Ok((src, dst, edge_type_id, weight))
        }
        Err(e) => Err(e),
    })
}

/// Returns iterator of edges handling the edge type IDs.
pub(crate) fn parse_edge_type_ids<'a>(
    edges_iter: impl Iterator<Item = Result<Quadruple, String>> + 'a,
    edge_types: &'a mut Vec<Option<EdgeTypeT>>,
) -> impl Iterator<Item = Result<Quadruple, String>> + 'a {
    edges_iter.map(move |row| {
        match row {
            Ok((src, dst, edge_type, weight)) => {
                edge_types.push(edge_type);
                Ok((
                    src,
                    dst,
                    edge_type,
                    weight
                ))
            },
            Err(e) => Err(e)
        }
        
    })
}

/// Returns iterator of edges handling the weights.
pub(crate) fn parse_weights<'a>(
    edges_iter: impl Iterator<Item = Result<Quadruple, String>> + 'a,
    weights: &'a mut Vec<WeightT>,
) -> impl Iterator<Item = Result<Quadruple, String>> + 'a {
    edges_iter.map(move |row| match row {
        Ok((src, dst, edge_type, weight)) => {
            let parsed_weight = match weight {
                Some(w) => {
                    validate_weight(w)?;
                    weights.push(w);
                    Some(w)
                }
                None => None,
            };
            Ok((src, dst, edge_type, parsed_weight))
        }
        Err(e) => Err(e),
    })
}

pub(crate) fn parse_sorted_edges<'a>(
    edges_iter: impl Iterator<Item = Result<Quadruple, String>> + 'a,
    directed: bool,
    directed_edge_list: bool
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
    ignore_duplicated_edges: bool,
    verbose: bool,
) -> (EdgeT, impl Iterator<Item = Result<Quadruple, String>>) {
    let pb = get_loading_bar(verbose, "Building sorted graph", edges.len());

    info!("Sorting edges.");
    edges.par_sort_by(|(src1, dst1, edt1, _), (src2, dst2, edt2, _)|{
        (*src2, *dst2, *edt2).cmp(&(*src1, *dst1, *edt1))
    });

    if ignore_duplicated_edges{
        info!("Removing duplicated edges.");
        edges.dedup_by(|(src1, dst1, edt1, _), (src2, dst2, edt2, _)|{
            (src1, dst1, edt1) == (src2, dst2, edt2)
        });
    }

    let edges_number = edges.len() as EdgeT;

    (
        edges_number,
        (0..edges_number).progress_with(pb).map(move |_| { Ok(edges.pop().unwrap())}),
    )
}

pub(crate) fn parse_integer_unsorted_edges<'a>(
    edges_iter: impl Iterator<Item = Result<(NodeT, NodeT, Option<NodeTypeT>, Option<WeightT>), String>>,
    directed: bool,
    directed_edge_list: bool,
    ignore_duplicated_edges: bool,
    verbose: bool,
) -> Result<(EdgeT, impl Iterator<Item = Result<Quadruple, String>> + 'a), String> {
    let edge_quadruples:Vec<Quadruple> = edges_iter.flat_map(|tuple|{
        match tuple{
            Ok((src, dst, edt, weight)) => if !directed && src != dst && !directed_edge_list {
                vec![Ok((src, dst, edt, weight)), Ok((dst, src, edt, weight))]
            } else {
                vec![Ok((src, dst, edt, weight))]
            },
            Err(e) => vec![Err(e)]
        }
    }).collect::<Result<Vec<Quadruple>, String>>()?;

    Ok(parse_unsorted_quadruples(edge_quadruples, ignore_duplicated_edges, verbose))
}

pub(crate) fn parse_string_unsorted_edges<'a>(
    // This parameter does not NEED a lifetime because it does NOT survive the function call
    edges_iter: impl Iterator<Item = Result<StringQuadruple, String>>,
    mut nodes: Vocabulary<NodeT>,
    directed: bool,
    directed_edge_list: bool,
    verbose: bool,
    numeric_node_ids: bool,
    numeric_edge_types_ids: bool,
    ignore_duplicated_edges: bool
) -> Result<(EdgeT, impl Iterator<Item = Result<Quadruple, String>> + 'a, Vocabulary<NodeT>, Vocabulary<EdgeTypeT>), String>  {
    let mut edge_types_vocabulary = Vocabulary::default().set_numeric_ids(numeric_edge_types_ids);
    nodes = nodes.set_numeric_ids(numeric_node_ids);
    let (edges_number, edges_iter) = { 
            let edge_quadruples:Vec<Quadruple> = parse_edge_type_ids_vocabulary(
                parse_edges_node_ids(edges_iter, &mut nodes),
                &mut edge_types_vocabulary,
            ).flat_map(|tuple|{
                match tuple{
                    Ok((src, dst, edt, weight)) => if !directed && src != dst && !directed_edge_list {
                        vec![Ok((src, dst, edt, weight)), Ok((dst, src, edt, weight))]
                    } else {
                        vec![Ok((src, dst, edt, weight))]
                    },
                    Err(e) => vec![Err(e)]
                }
            }).collect::<Result<Vec<Quadruple>, String>>()?;
        
            parse_unsorted_quadruples(edge_quadruples, ignore_duplicated_edges, verbose)
    };
    info!("Building nodes reverse mapping.");
    nodes.build_reverse_mapping()?;
    info!("Building edge types reverse mapping.");
    edge_types_vocabulary.build_reverse_mapping()?;

    Ok((edges_number, edges_iter, nodes, edge_types_vocabulary))
}

pub(crate) fn build_edges(
    edges_iter: impl Iterator<Item = Result<Quadruple, String>>,
    edges_number: EdgeT,
    nodes_number: NodeT,
    ignore_duplicated_edges: bool,
    directed: bool,
    directed_edge_list: bool
) -> Result<(EliasFano, EliasFano, EdgeT, EdgeT, NodeT, NodeT, NodeT, u8, u64), String> {
    info!("Started building of EliasFano edges data structure.");
    let node_bits = get_node_bits(nodes_number);
    let node_bit_mask = (1 << node_bits) - 1;
    let mut edges: EliasFano = EliasFano::new(
        encode_max_edge(nodes_number, node_bits),
        edges_number as usize,
    )?;
    // TODO: the following data structure could be better to be a bitvector.
    // This is because universe == number of elements
    let mut unique_sources: EliasFano = EliasFano::new(nodes_number as u64, nodes_number as usize)?;
    // Last source inserted
    let mut last_src: NodeT = 0;
    let mut last_dst: NodeT = 0;
    let mut last_edge_type: Option<EdgeTypeT> = None;
    let mut unique_edges_number: EdgeT = 0;
    let mut unique_self_loop_number: NodeT = 0;
    let mut self_loop_number: EdgeT = 0;
    let mut forward_undirected_edges_counter: EdgeT = 0;
    let mut backward_undirected_edges_counter: EdgeT = 0;
    let mut nodes_with_edges = bitvec![Msb0, u8; 0; nodes_number as usize];
    let mut not_singleton_node_number: NodeT = 0;
    let mut singleton_nodes_with_self_loops = bitvec![Msb0, u8; 0; nodes_number as usize];
    let mut singleton_nodes_with_self_loops_number: NodeT = 0;
    let mut first = true;

    for value in edges_iter {
        let (src, dst, edge_type, _) = value?;
        let different_src = last_src != src || first;
        let different_dst = last_dst != dst || first;
        let self_loop = src == dst;
        let different_edge_type = last_edge_type != edge_type || first;
        if !(different_src || different_dst || different_edge_type){
            if ignore_duplicated_edges {
                continue;
            } else {
                return Err("A duplicated edge was found while building the graph.".to_owned());
            }
        }
        if  !directed && directed_edge_list{
            match src.cmp(&dst) {
                Ordering::Greater => {forward_undirected_edges_counter += 1},
                Ordering::Less => {backward_undirected_edges_counter += 1},
                Ordering::Equal => {}
            }
        }
        last_edge_type = edge_type;
        edges.unchecked_push(encode_edge(src, dst, node_bits));
        if self_loop {
            self_loop_number += 1;
        }
        if different_src || different_dst {
            for node in &[src, dst]{
                if !nodes_with_edges[*node as usize]{
                    nodes_with_edges.set(*node as usize, true);
                    if !self_loop{
                        not_singleton_node_number+=1;
                    } else {
                        singleton_nodes_with_self_loops.set(*node as usize, true);
                        singleton_nodes_with_self_loops_number+= 1;
                    }
                } else if !self_loop && singleton_nodes_with_self_loops[*node as usize]{
                    singleton_nodes_with_self_loops.set(*node as usize, false);
                    singleton_nodes_with_self_loops_number-= 1;
                    not_singleton_node_number+=1;
                }
            }
            unique_edges_number += 1;
            if self_loop {
                unique_self_loop_number += 1;
            }
            if different_src {
                unique_sources.unchecked_push(src as u64);
                last_src = src;
            }
            if different_dst {
                last_dst = dst;
            }
        }
        if first {
            first = false;
        }
    }

    if forward_undirected_edges_counter != backward_undirected_edges_counter {
        return Err(concat!(
            "You are trying to load an undirected graph ",
            "from a directed edge list but the edge list is not ",
            "complete."
        ).to_owned());
    }

    Ok((
        edges,
        unique_sources,
        unique_edges_number,
        self_loop_number,
        unique_self_loop_number,
        not_singleton_node_number,
        singleton_nodes_with_self_loops_number,
        node_bits,
        node_bit_mask,
    ))
}

fn parse_nodes(
    nodes_iterator: Option<impl Iterator<Item = Result<(String, Option<Vec<String>>), String>>>,
    ignore_duplicated_nodes: bool,
    numeric_node_ids: bool,
    numeric_node_types_ids: bool,
    has_node_types: bool
) -> Result<(Vocabulary<NodeT>, NodeTypeVocabulary), String> {
    let mut nodes = Vocabulary::default().set_numeric_ids(numeric_node_ids);
    let mut node_types = NodeTypeVocabulary::default().set_numeric_ids(numeric_node_types_ids);

    if let Some(ni) = nodes_iterator {
        // TODO: the following can likely be dealt with in a better way.
        let node_iterator = parse_node_ids(ni, ignore_duplicated_nodes, &mut nodes);
        // In the case there is a node types we need to add its proper iterator.
        let node_iterator: Box<dyn Iterator<Item=Result<(NodeT, Option<Vec<NodeTypeT>>), String>>> = match has_node_types{
            true => Box::new(parse_node_type_ids(
                node_iterator,
                &mut node_types,
            )),
            false => Box::new(node_iterator.map_ok(|(node_id, _)| (node_id, None)))
        };
        for row in node_iterator{
            row?;
        }
        node_types.build_reverse_mapping()?;
        node_types.build_counts();
    }

    Ok((nodes, node_types))
}

pub(crate) fn parse_string_edges(
    edges_iter: impl Iterator<Item = Result<StringQuadruple, String>>,
    edges_number: EdgeT,
    nodes_number: NodeT,
    directed: bool,
    mut nodes: Vocabulary<NodeT>,
    numeric_edge_node_ids: bool,
    numeric_edge_types_ids: bool,
    directed_edge_list: bool,
    ignore_duplicated_edges: bool,
    has_edge_types: bool,
    has_weights: bool
) -> ParsedStringEdgesType {
    let mut weights: Vec<WeightT> = Vec::new();
    let mut edge_types_vocabulary: Vocabulary<EdgeTypeT> = Vocabulary::default().set_numeric_ids(numeric_edge_types_ids);
    let mut edge_types_ids: Vec<Option<EdgeTypeT>> = Vec::new();
    nodes = nodes.set_numeric_ids(numeric_edge_node_ids);

    let wrapped_edges_iterator = parse_sorted_edges(
        parse_edge_type_ids_vocabulary(
            parse_edges_node_ids(edges_iter, &mut nodes),
            &mut edge_types_vocabulary,
        ),
        directed,
        directed_edge_list
    );

    let typed_edges_iter: Box<dyn Iterator<Item=Result<Quadruple, String>>> = match has_edge_types{
        true=> Box::new(parse_edge_type_ids(wrapped_edges_iterator, &mut edge_types_ids)),
        false => Box::new(wrapped_edges_iterator)
    };

    let weighted_edges_iter: Box<dyn Iterator<Item=Result<Quadruple, String>>> = match has_weights{
        true=> Box::new(parse_weights(typed_edges_iter, &mut weights)),
        false => Box::new(typed_edges_iter)
    };

    let (
        edges,
        unique_sources,
        unique_edges_number,
        self_loop_number,
        unique_self_loop_number,
        not_singleton_nodes_number,
        singleton_nodes_with_self_loops_number,
        node_bits,
        node_bit_mask,
    ) = build_edges(
        weighted_edges_iter,
        edges_number,
        nodes_number,
        ignore_duplicated_edges,
        directed,
        directed_edge_list
    )?;

    nodes.build_reverse_mapping()?;

    if !weights.is_empty() && edges.len() != weights.len() {
        return Err(format!(
            "The number of weights {} does not match the number of edges {}.",
            weights.len(),
            edges.len()
        ));
    }

    if !edge_types_ids.is_empty() && edges.len() != edge_types_ids.len() {
        return Err(format!(
            "The number of edge types {} does not match the number of edges {}.",
            edge_types_ids.len(),
            edges.len()
        ));
    }

    edge_types_vocabulary.build_reverse_mapping()?;
    let edge_types = EdgeTypeVocabulary::from_structs(edge_types_ids, optionify!(edge_types_vocabulary));

    Ok((
        edges,
        unique_sources,
        nodes,
        edge_types,
        weights,
        unique_edges_number,
        self_loop_number,
        unique_self_loop_number,
        not_singleton_nodes_number,
        singleton_nodes_with_self_loops_number,
        node_bit_mask,
        node_bits,
    ))
}

pub(crate) fn parse_integer_edges(
    edges_iter: impl Iterator<Item = Result<Quadruple, String>>,
    edges_number: EdgeT,
    nodes_number: NodeT,
    edge_types_vocabulary: Option<Vocabulary<EdgeTypeT>>,
    ignore_duplicated_edges: bool,
    directed: bool,
    directed_edge_list: bool,
    has_edge_types: bool,
    has_weights: bool
) -> Result<
    (
        EliasFano,
        EliasFano,
        Option<EdgeTypeVocabulary>,
        Vec<WeightT>,
        EdgeT,
        EdgeT,
        NodeT,
        NodeT,
        NodeT,
        u64,
        u8,
    ),
    String,
> {
    let mut weights: Vec<WeightT> = Vec::new();
    let mut edge_types_ids: Vec<Option<EdgeTypeT>> = Vec::new();
    
    let edges_iter: Box<dyn Iterator<Item=Result<Quadruple, String>>> = match has_edge_types{
        true=> Box::new(parse_edge_type_ids(edges_iter, &mut edge_types_ids)),
        false => Box::new(edges_iter)
    };

    let edges_iter: Box<dyn Iterator<Item=Result<Quadruple, String>>> = match has_weights{
        true=> Box::new(parse_weights(edges_iter, &mut weights)),
        false => Box::new(edges_iter)
    };

    let (
        edges,
        unique_sources,
        unique_edges_number,
        self_loop_number,
        unique_self_loop_number,
        not_singleton_nodes_number,
        singleton_nodes_with_self_loops_number,
        node_bits,
        node_bit_mask,
    ) = build_edges(
        edges_iter,
        edges_number,
        nodes_number,
        ignore_duplicated_edges,
        directed,
        directed_edge_list
    )?;

    if !weights.is_empty() && edges.len() != weights.len() {
        return Err(format!(
            "The number of weights {} does not match the number of edges {}.",
            weights.len(),
            edges.len()
        ));
    }

    if !edge_types_ids.is_empty() && edges.len() != edge_types_ids.len() {
        return Err(format!(
            "The number of edge types {} does not match the number of edges {}.",
            edge_types_ids.len(),
            edges.len()
        ));
    }

    let edge_types = EdgeTypeVocabulary::from_structs(edge_types_ids, edge_types_vocabulary);

    Ok((
        edges,
        unique_sources,
        edge_types,
        weights,
        unique_edges_number,
        self_loop_number,
        unique_self_loop_number,
        not_singleton_nodes_number,
        singleton_nodes_with_self_loops_number,
        node_bit_mask,
        node_bits,
    ))
}

/// # Graph Constructors
impl Graph {

    pub(crate) fn build_graph<S: Into<String>>(
        edge_iter: impl Iterator<Item = Result<Quadruple, String>>,
        edges_number: EdgeT,
        nodes: Vocabulary<NodeT>,
        node_types: Option<NodeTypeVocabulary>,
        edge_types_vocabulary: Option<Vocabulary<EdgeTypeT>>,
        directed: bool,
        name: S,
        ignore_duplicated_edges: bool,
        has_edge_types: bool,
        has_weights: bool
    ) -> Result<Graph, String> {
        let (
            edges,
            unique_sources,
            edge_types,
            weights,
            unique_edges_number,
            self_loop_number,
            unique_self_loop_number,
            not_singleton_nodes_number,
            singleton_nodes_with_self_loops_number,
            node_bit_mask,
            node_bits,
        ) = parse_integer_edges(
            edge_iter,
            edges_number,
            nodes.len() as NodeT,
            edge_types_vocabulary,
            ignore_duplicated_edges,
            directed,
            true,
            has_edge_types,
            has_weights
        )?;

        Ok(Graph::new(
            directed,
            unique_self_loop_number,
            self_loop_number,
            not_singleton_nodes_number,
            singleton_nodes_with_self_loops_number,
            unique_edges_number,
            edges,
            unique_sources,
            nodes,
            node_bit_mask,
            node_bits,
            edge_types,
            name,
            optionify!(weights),
            node_types
        ))
    }

    /// Create new Graph object from unsorted source.
    ///
    /// # Arguments
    ///
    /// TODO: UPDATE THE DOCSTRING!
    /// 
    /// * edges_iterator: impl Iterator<Item = Result<StringQuadruple, String>>,
    ///     Iterator of the edges.
    /// * nodes_iterator: Option<impl Iterator<Item = Result<(String, Option<String>), String>>>,
    ///     Iterator of the nodes.
    /// * directed: bool,
    ///     Wether the graph should be directed or undirected.
    /// * ignore_duplicated_nodes: bool,
    ///     Wether to ignore duplicated nodes or to raise a proper exception.
    /// * ignore_duplicated_edges: bool,
    ///     Wether to ignore duplicated edges or to raise a proper exception.
    /// * skip_self_loops: bool,
    ///     Wether to skip self loops while reading the the edges iterator.
    pub fn from_string_unsorted<S: Into<String>>(
        edges_iterator: impl Iterator<Item = Result<StringQuadruple, String>>,
        nodes_iterator: Option<impl Iterator<Item = Result<(String, Option<Vec<String>>), String>>>,
        directed: bool,
        directed_edge_list: bool,
        name: S,
        ignore_duplicated_nodes: bool,
        ignore_duplicated_edges: bool,
        verbose: bool,
        numeric_edge_types_ids: bool,
        numeric_node_ids: bool,
        numeric_edge_node_ids: bool,
        numeric_node_types_ids: bool,
        has_node_types: bool,
        has_edge_types: bool,
        has_weights: bool
    ) -> Result<Graph, String> {
        let (nodes, node_types) = parse_nodes(
            nodes_iterator,
            ignore_duplicated_nodes,
            numeric_node_ids,
            numeric_node_types_ids,
            has_node_types
        )?;
        
        info!("Parse unsorted edges.");
        let (edges_number, edges_iterator, nodes, edge_types_vocabulary) =
            parse_string_unsorted_edges(
                edges_iterator,
                nodes,
                directed,
                directed_edge_list,
                verbose,
                numeric_edge_node_ids,
                numeric_edge_types_ids,
                ignore_duplicated_edges
            )?;

        Graph::build_graph(
            edges_iterator,
            edges_number,
            nodes,
            optionify!(node_types),
            optionify!(edge_types_vocabulary),
            directed,
            name,
            ignore_duplicated_edges,
            has_edge_types,
            has_weights
        )
    }

    /// Create new Graph object from unsorted source.
    ///
    /// # Arguments
    ///
    /// * edges_iterator: impl Iterator<Item = Result<StringQuadruple, String>>,
    ///     Iterator of the edges.
    /// * nodes_iterator: Option<impl Iterator<Item = Result<(String, Option<String>), String>>>,
    ///     Iterator of the nodes.
    /// * directed: bool,
    ///     Wether the graph should be directed or undirected.
    /// * ignore_duplicated_nodes: bool,
    ///     Wether to ignore duplicated nodes or to raise a proper exception.
    /// * ignore_duplicated_edges: bool,
    ///     Wether to ignore duplicated edges or to raise a proper exception.
    /// * skip_self_loops: bool,
    ///     Wether to skip self loops while reading the the edges iterator.
    pub fn from_integer_unsorted(
        edges_iterator: impl Iterator<
            Item = Result<(NodeT, NodeT, Option<NodeTypeT>, Option<WeightT>), String>,
        >,
        nodes: Vocabulary<NodeT>,
        node_types: Option<NodeTypeVocabulary>,
        edge_types_vocabulary: Option<Vocabulary<EdgeTypeT>>,
        directed: bool,
        directed_edge_list: bool,
        name: String,
        ignore_duplicated_edges: bool,
        has_edge_types: bool,
        has_weights: bool,
        verbose: bool
    ) -> Result<Graph, String> {
        let (edges_number, edges_iterator) =
            parse_integer_unsorted_edges(edges_iterator, directed, directed_edge_list, ignore_duplicated_edges, verbose)?;

        Graph::build_graph(
            edges_iterator,
            edges_number,
            nodes,
            node_types,
            edge_types_vocabulary,
            directed,
            name,
            ignore_duplicated_edges,
            has_edge_types,
            has_weights
        )
    }

    /// Create new Graph object from sorted sources.
    pub fn from_string_sorted<S: Into<String>>(
        edges_iterator: impl Iterator<Item = Result<StringQuadruple, String>>,
        nodes_iterator: Option<impl Iterator<Item = Result<(String, Option<Vec<String>>), String>>>,
        directed: bool,
        directed_edge_list: bool,
        ignore_duplicated_nodes: bool,
        ignore_duplicated_edges: bool,
        edges_number: EdgeT,
        nodes_number: NodeT,
        numeric_edge_types_ids: bool,
        numeric_node_ids: bool,
        numeric_edge_node_ids: bool,
        numeric_node_types_ids: bool,
        has_node_types: bool,
        has_edge_types: bool,
        has_weights: bool,
        name: S,
    ) -> Result<Graph, String> {
        let (nodes, node_types) = parse_nodes(
            nodes_iterator,
            ignore_duplicated_nodes,
            numeric_node_ids,
            numeric_node_types_ids,
            has_node_types
        )?;

        let (
            edges,
            unique_sources,
            nodes,
            edge_types,
            weights,
            unique_edges_number,
            self_loop_number,
            unique_self_loop_number,
            not_singleton_nodes_number,
            singleton_nodes_with_self_loops_number,
            node_bit_mask,
            node_bits,
        ) = parse_string_edges(
            edges_iterator,
            edges_number,
            nodes_number,
            directed,
            nodes,
            numeric_edge_node_ids,
            numeric_edge_types_ids,
            directed_edge_list,
            ignore_duplicated_edges,
            has_edge_types,
            has_weights
        )?;

        Ok(Graph::new(
            directed,
            unique_self_loop_number,
            self_loop_number,
            not_singleton_nodes_number,
            singleton_nodes_with_self_loops_number,
            unique_edges_number,
            edges,
            unique_sources,
            nodes,
            node_bit_mask,
            node_bits,
            edge_types,
            name,
            optionify!(weights),
            optionify!(node_types)
        ))
    }
}