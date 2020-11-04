use super::*;
use elias_fano_rust::EliasFano;
use indicatif::ProgressIterator;
use bitvec::prelude::*;
use rayon::prelude::ParallelSliceMut;
use std::collections::BTreeMap;

type ParsedStringEdgesType = Result<
    (
        EliasFano,
        EliasFano,
        Vocabulary<NodeT>,
        Option<VocabularyVec<EdgeTypeT, EdgeT>>,
        Vec<WeightT>,
        EdgeT,
        EdgeT,
        NodeT,
        NodeT,
        u64,
        u8,
    ),
    String,
>;

#[macro_export]
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
    nodes_iter: impl Iterator<Item = Result<(String, Option<String>), String>> + 'a,
    ignore_duplicated_nodes: bool,
    nodes: &'b mut Vocabulary<NodeT>,
) -> impl Iterator<Item = Result<(NodeT, Option<String>), String>> + 'a
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
    nodes_iter: impl Iterator<Item = Result<(NodeT, Option<String>), String>> + 'a,
    node_types: &'b mut VocabularyVec<NodeTypeT, NodeT>,
) -> impl Iterator<Item = Result<(NodeT, Option<NodeTypeT>), String>> + 'a
where
    'b: 'a,
{
    let mut has_node_types: Option<bool> = None;
    nodes_iter.map(move |row| match row {
        Ok((node_id, node_type)) => {
            if *has_node_types.get_or_insert(node_type.is_some()) != node_type.is_some(){
                return Err("The node types are not consistents. Either all node types are None, or all have valid values.".to_string());
            }
            let node_type_id = match node_type {
                Some(nt) => Some(node_types.insert(nt)?),
                None => None
            };
            Ok((node_id, node_type_id))
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
                    } else {
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
                *nodes.get(&src_name).unwrap(),
                *nodes.get(&dst_name).unwrap(),
                edge_type,
                weight,
            ))
        }
        Err(e) => Err(e),
    })
}

/// Returns iterator of edges handling the edge type IDs.
pub(crate) fn parse_edge_type_ids_vocabulary<'a>(
    edges_iter: impl Iterator<Item = Result<(NodeT, NodeT, Option<String>, Option<WeightT>), String>>
        + 'a,
    edge_types: &'a mut Vocabulary<EdgeTypeT>,
) -> impl Iterator<Item = Result<Quadruple, String>> + 'a {
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
    edge_types: &'a mut Vec<EdgeTypeT>,
) -> impl Iterator<Item = Result<Quadruple, String>> + 'a {
    let mut has_edge_types: Option<bool> = None;
    edges_iter.map(move |row| {
        match row {
            Ok((src, dst, edge_type, weight)) => {
                if *has_edge_types.get_or_insert(edge_type.is_some()) != edge_type.is_some(){
                    return Err("The edge_types are not consistents. Either all edge_types are None, or all have valid values.".to_string());
                }
                Ok((
                    src,
                    dst,
                    edge_type.map(|nt| {
                        edge_types.push(nt);
                        nt
                    }),
                    weight,
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
    let mut has_weights: Option<bool> = None;
    edges_iter.map(move |row| match row {
        Ok((src, dst, edge_type, weight)) => {
            if *has_weights.get_or_insert(weight.is_some()) != weight.is_some(){
                return Err("The weights are not consistents. Either all weights are None, or all have valid values.".to_string());
            }
            
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
    sorting_tmp: &'a mut BTreeMap<Triple, Option<WeightT>>,
) -> impl Iterator<Item = Result<Quadruple, String>> + 'a {
    edges_iter
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
        })
}

pub(crate) fn parse_string_unsorted_edges<'a>(
    edges_iter: impl Iterator<Item = Result<StringQuadruple, String>>,
    mut nodes: Vocabulary<NodeT>,
    directed: bool,
    verbose: bool,
    numeric_edge_types_ids: bool,
    ignore_duplicated_edges: bool
) -> Result<
    (
        EdgeT,
        impl Iterator<Item = Result<Quadruple, String>> + 'a,
        Vocabulary<NodeT>,
        Vocabulary<EdgeTypeT>,
    ),
    String,
> {
    let mut edge_types_vocabulary = Vocabulary::new(numeric_edge_types_ids);

    let wrapped_edges_iterator = parse_edge_type_ids_vocabulary(
        parse_edges_node_ids(edges_iter, &mut nodes),
        &mut edge_types_vocabulary,
    );

    let mut edge_quadruples:Vec<Quadruple> = wrapped_edges_iterator.flat_map(|tuple|{
        match tuple{
            Ok((src, dst, edt, weight)) => if !directed && src != dst {
                vec![Ok((src, dst, edt, weight)), Ok((dst, src, edt, weight))]
            } else {
                vec![Ok((src, dst, edt, weight))]
            },
            Err(e) => vec![Err(e)]
        }
    }).collect::<Result<Vec<Quadruple>, String>>()?;

    edge_quadruples.par_sort_by(|(src1, dst1, edt1, weight1), (src2, dst2, edt2, weight2)|{
        match (src1, dst1, edt1).cmp(&(src2, dst2, edt2)){
            std::cmp::Ordering::Equal => match (weight1, weight2) {
                (Some(w1), Some(w2)) => {
                    if w1 < w2 {
                        std::cmp::Ordering::Greater
                    } else {
                        std::cmp::Ordering::Less
                    }
                },
                _ => std::cmp::Ordering::Equal
            },
            std::cmp::Ordering::Less => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Greater =>std::cmp::Ordering::Less
        }
    });

    let pb = get_loading_bar(verbose, "Sorting and building graph", edge_quadruples.len());
    let edges_number = edge_quadruples.len() as EdgeT;
    edge_types_vocabulary.build_reverse_mapping()?;
    nodes.build_reverse_mapping()?;
    
    let mut last_value: Option<Quadruple> = None;

    Ok((
        edges_number,
        (0..edges_number).progress_with(pb).filter_map(move |_| {
            let new_value = edge_quadruples.pop().unwrap();
            let (src, dst, edt, _) = new_value;
            match &mut last_value{
                Some(lv) => {
                    let (last_src, last_dst, last_edt, _) = *lv;
                    match (last_src, last_dst, last_edt)==(src, dst, edt) {
                    true => if ignore_duplicated_edges{
                        None
                    } else {
                        // TODO! Make this error more helpful!
                        Some(Err("Duplicated edges found!".to_owned()))
                    },
                    false => {
                        *lv = new_value;
                        Some(Ok(new_value))
                    }
                }},
                None => {
                    last_value = Some(new_value);
                    Some(Ok(new_value))
                }
            }
        }),
        nodes,
        edge_types_vocabulary,
    ))
}

pub(crate) fn parse_integer_unsorted_edges<'a>(
    edges_iter: impl Iterator<Item = Result<(NodeT, NodeT, Option<NodeTypeT>, Option<WeightT>), String>>,
    sorting_tmp: &'a mut BTreeMap<Triple, Option<WeightT>>,
    directed: bool,
    verbose: bool,
) -> Result<(EdgeT, impl Iterator<Item = Result<Quadruple, String>> + 'a), String> {
    for value in edges_iter {
        let (src, dst, edt, weight) = value?;
        sorting_tmp.insert((src, dst, edt), weight);
        if !directed && src != dst {
            sorting_tmp.insert((dst, src, edt), weight);
        }
    }

    let pb = get_loading_bar(verbose, "Sorting and building graph", sorting_tmp.len());
    let edges_number = sorting_tmp.len() as EdgeT;

    Ok((
        edges_number,
        (0..sorting_tmp.len()).progress_with(pb).map(move |_| {
            let ((src, dst, edge_type), weight) = sorting_tmp.pop_first().unwrap();
            Ok((src, dst, edge_type, weight))
        }),
    ))
}

pub(crate) fn build_edges(
    edges_iter: impl Iterator<Item = Result<Quadruple, String>>,
    edges_number: EdgeT,
    nodes_number: NodeT,
    ignore_duplicated_edges: bool,
) -> Result<(EliasFano, EliasFano, EdgeT, EdgeT, NodeT, NodeT, u8, u64), String> {
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
    // TODO: using roaring might be sub-optimal when the bitvec is dense.
    let mut non_singleton_nodes = bitvec![Msb0, u8; 0; nodes_number as usize];
    let mut non_singleton_nodes_number: NodeT = 0;
    let mut first = true;

    for value in edges_iter {
        let (src, dst, edge_type, _) = value?;
        let different_src = last_src != src || first;
        let different_dst = last_dst != dst || first;
        let different_edge_type = last_edge_type != edge_type || first;
        if !(different_src || different_dst || different_edge_type){
            if ignore_duplicated_edges {
                continue;
            } else {
                return Err("A duplicated edge was found while building the graph.".to_owned());
            }
        }
        last_edge_type = edge_type;
        edges.push(encode_edge(src, dst, node_bits)).unwrap();
        if src == dst {
            self_loop_number += 1;
        }
        if different_src || different_dst {
            if !non_singleton_nodes[src as usize]{
                non_singleton_nodes.set(src as usize, true);
                non_singleton_nodes_number+=1;
            }
            if !non_singleton_nodes[dst as usize]{
                non_singleton_nodes.set(dst as usize, true);
                non_singleton_nodes_number+=1;
            }
            unique_edges_number += 1;
            if src == dst {
                unique_self_loop_number += 1;
            }
            if different_src {
                unique_sources.push(src as u64).unwrap();
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

    Ok((
        edges,
        unique_sources,
        unique_edges_number,
        self_loop_number,
        unique_self_loop_number,
        non_singleton_nodes_number,
        node_bits,
        node_bit_mask,
    ))
}

fn parse_nodes(
    nodes_iterator: Option<impl Iterator<Item = Result<(String, Option<String>), String>>>,
    ignore_duplicated_nodes: bool,
    numeric_node_ids: bool,
    numeric_node_types_ids: bool,
) -> Result<(Vocabulary<NodeT>, VocabularyVec<NodeTypeT, NodeT>), String> {
    let mut nodes = Vocabulary::new(numeric_node_ids);
    let mut node_types = VocabularyVec::new(numeric_node_types_ids);

    if let Some(ni) = nodes_iterator {
        // TODO: the following can likely be dealt with in a better way.
        for row in parse_node_type_ids(
            parse_node_ids(ni, ignore_duplicated_nodes, &mut nodes),
            &mut node_types,
        ) {
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
    numeric_edge_types_ids: bool,
    ignore_duplicated_edges: bool,
) -> ParsedStringEdgesType {
    let mut weights: Vec<WeightT> = Vec::new();
    let mut edge_types_vocabulary: Vocabulary<EdgeTypeT> = Vocabulary::new(numeric_edge_types_ids);
    let mut edge_types_ids: Vec<EdgeTypeT> = Vec::new();
    let mut edge_sorting_tmp = BTreeMap::new();

    let wrapped_edges_iterator = parse_sorted_edges(
        parse_edge_type_ids_vocabulary(
            parse_edges_node_ids(edges_iter, &mut nodes),
            &mut edge_types_vocabulary,
        ),
        directed,
        &mut edge_sorting_tmp,
    );

    let typed_edges_iter = parse_edge_type_ids(wrapped_edges_iterator, &mut edge_types_ids);

    let weighted_edges_iter = parse_weights(typed_edges_iter, &mut weights);

    let (
        edges,
        unique_sources,
        unique_edges_number,
        self_loop_number,
        unique_self_loop_number,
        not_singleton_nodes_number,
        node_bits,
        node_bit_mask,
    ) = build_edges(
        weighted_edges_iter,
        edges_number,
        nodes_number,
        ignore_duplicated_edges,
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
    let edge_types = VocabularyVec::from_structs(edge_types_ids, optionify!(edge_types_vocabulary));

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
) -> Result<
    (
        EliasFano,
        EliasFano,
        Option<VocabularyVec<EdgeTypeT, EdgeT>>,
        Vec<WeightT>,
        EdgeT,
        EdgeT,
        NodeT,
        NodeT,
        u64,
        u8,
    ),
    String,
> {
    let mut weights: Vec<WeightT> = Vec::new();
    let mut edge_types_ids: Vec<EdgeTypeT> = Vec::new();

    let typed_edges_iter = parse_edge_type_ids(edges_iter, &mut edge_types_ids);

    let weighted_edges_iter = parse_weights(typed_edges_iter, &mut weights);

    let (
        edges,
        unique_sources,
        unique_edges_number,
        self_loop_number,
        unique_self_loop_number,
        not_singleton_nodes_number,
        node_bits,
        node_bit_mask,
    ) = build_edges(
        weighted_edges_iter,
        edges_number,
        nodes_number,
        ignore_duplicated_edges,
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

    let edge_types = VocabularyVec::from_structs(edge_types_ids, edge_types_vocabulary);

    Ok((
        edges,
        unique_sources,
        edge_types,
        weights,
        unique_edges_number,
        self_loop_number,
        unique_self_loop_number,
        not_singleton_nodes_number,
        node_bit_mask,
        node_bits,
    ))
}

/// # Graph Constructors
impl Graph {

    pub(crate) fn build_graph(
        edge_iter: impl Iterator<Item = Result<Quadruple, String>>,
        edges_number: EdgeT,
        nodes: Vocabulary<NodeT>,
        node_types: Option<VocabularyVec<NodeTypeT, NodeT>>,
        edge_types_vocabulary: Option<Vocabulary<EdgeTypeT>>,
        directed: bool,
        name: String,
        ignore_duplicated_edges: bool
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
            node_bit_mask,
            node_bits,
        ) = parse_integer_edges(
            edge_iter,
            edges_number,
            nodes.len() as NodeT,
            edge_types_vocabulary,
            ignore_duplicated_edges,
        )?;

        Ok(Graph {
            directed,
            unique_self_loop_number,
            self_loop_number,
            not_singleton_nodes_number,
            unique_edges_number,
            edges,
            unique_sources,
            nodes,
            node_bit_mask,
            node_bits,
            node_types,
            edge_types,
            name,
            weights:optionify!(weights),
            destinations: None,
            outbounds: None,
            cached_destinations: None
        })
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
    pub fn from_string_unsorted(
        edges_iterator: impl Iterator<Item = Result<StringQuadruple, String>>,
        nodes_iterator: Option<impl Iterator<Item = Result<(String, Option<String>), String>>>,
        directed: bool,
        name: String,
        ignore_duplicated_nodes: bool,
        ignore_duplicated_edges: bool,
        verbose: bool,
        numeric_edge_types_ids: bool,
        numeric_node_ids: bool,
        numeric_node_types_ids: bool
    ) -> Result<Graph, String> {
        let (nodes, node_types) = parse_nodes(
            nodes_iterator,
            ignore_duplicated_nodes,
            numeric_node_ids,
            numeric_node_types_ids,
        )?;

        let (edges_number, edges_iterator, nodes, edge_types_vocabulary) =
            parse_string_unsorted_edges(
                edges_iterator,
                nodes,
                directed,
                verbose,
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
            ignore_duplicated_edges
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
        node_types: Option<VocabularyVec<NodeTypeT, NodeT>>,
        edge_types_vocabulary: Option<Vocabulary<EdgeTypeT>>,
        directed: bool,
        name: String,
        ignore_duplicated_edges: bool,
        verbose: bool
    ) -> Result<Graph, String> {
        let mut edge_sorting_tmp = BTreeMap::new();

        let (edges_number, edges_iterator) =
            parse_integer_unsorted_edges(edges_iterator, &mut edge_sorting_tmp, directed, verbose)?;

        Graph::build_graph(
            edges_iterator,
            edges_number,
            nodes,
            node_types,
            edge_types_vocabulary,
            directed,
            name,
            ignore_duplicated_edges
        )
    }

    /// Create new Graph object from sorted sources.
    pub fn from_string_sorted(
        edges_iterator: impl Iterator<Item = Result<StringQuadruple, String>>,
        nodes_iterator: Option<impl Iterator<Item = Result<(String, Option<String>), String>>>,
        directed: bool,
        ignore_duplicated_nodes: bool,
        ignore_duplicated_edges: bool,
        edges_number: EdgeT,
        nodes_number: NodeT,
        numeric_edge_types_ids: bool,
        numeric_node_ids: bool,
        numeric_node_types_ids: bool,
        name: String,
    ) -> Result<Graph, String> {
        let (nodes, node_types) = parse_nodes(
            nodes_iterator,
            ignore_duplicated_nodes,
            numeric_node_ids,
            numeric_node_types_ids,
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
            node_bit_mask,
            node_bits,
        ) = parse_string_edges(
            edges_iterator,
            edges_number,
            nodes_number,
            directed,
            nodes,
            numeric_edge_types_ids,
            ignore_duplicated_edges,
        )?;

        Ok(Graph {
            directed,
            unique_self_loop_number,
            self_loop_number,
            not_singleton_nodes_number,
            unique_edges_number,
            edges,
            unique_sources,
            nodes,
            node_bit_mask,
            node_bits,
            edge_types,
            name,
            weights: optionify!(weights),
            node_types:optionify!(node_types),
            destinations: None,
            outbounds: None,
            cached_destinations: None
    })
    }
}
