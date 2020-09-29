use super::*;
use elias_fano_rust::EliasFano;
use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;

macro_rules! optionify {
    ($val:expr) => {
        if $val.is_empty() {
            None
        } else {
            Some($val)
        }
    };
}

/// Read node file and returns graph builder data structures.
///
/// Specifically, the returned objects are:
/// * nodes_mapping: an hashmap from the node name to the node id.
/// * node_reverse_mapping: vector of node names.
/// * node_types_mapping: an hashmap from node types names to the node type ids.
/// * node_types_reverse_mapping: vector of the node types names.
/// * node_types: vector of the numeric node types ids.
pub(crate) fn parse_nodes(
    nodes_iter: impl Iterator<Item = Result<(String, Option<String>), String>>,
    ignore_duplicated_nodes: bool,
) -> Result<(Vocabulary<NodeT>, Option<VocabularyVec<NodeTypeT>>), String> {
    let mut nodes: Vocabulary<NodeT> = Vocabulary::default();
    let mut node_types: VocabularyVec<NodeTypeT> = VocabularyVec::default();

    let mut has_type = None;

    for values in nodes_iter {
        let (node_name, node_type) = values?;

        // clean way to save if the first edge has edge type and weights
        has_type = has_type.or_else(|| Some(node_type.is_some()));

        // check consistency
        if let Some(ht) = &has_type {
            if node_type.is_some() != *ht {
                return Err(format!(
                    concat!(
                        "The node {} {:?} has node type inconsistent with all the nodes before.\n",
                        "Either all nodes have node types or none have it."
                    ),
                    node_name, node_type
                ));
            }
        }

        // if the node is already mapped => duplicated line
        if nodes.contains_key(&node_name) {
            if ignore_duplicated_nodes {
                continue;
            }
            return Err(format!(
                concat!(
                    "\nFound duplicated nodes!\n",
                    "The node is {node_name}.\n",
                    "The node type of the row is {node_type:?}.\n",
                    "The library does not currently support multiple node types for a single node."
                ),
                node_name = node_name,
                node_type = node_type
            ));
        }
        nodes.insert(node_name);
        if let Some(ndt) = node_type {
            node_types.insert(ndt);
        }
    }

    Ok((nodes, optionify!(node_types)))
}

/// Read edge file and returns graph builder data structures.
///
/// # Arguments
///
/// * `nodes`: &mut Vocabulary<NodeT> - Vocabulary of the nodes of the graph.
/// * `directed`: bool - Wethever to load the graph as directed or not.
/// * `edges_iterator`: impl Iterator<Item = Result<(String, String, Option<String>, Option<WeightT>), String>> - Iterator from where to load the edges data.
/// * `skip_self_loops`: bool - Wethever to skip self-loops while parsing the iterator.
/// * `ignore_duplicated_edges`: bool - Wethever to ignore duplicated edges.
pub(crate) fn parse_edges(
    nodes: &mut Vocabulary<NodeT>,
    directed: bool,
    edges_iterator: impl Iterator<
        Item = Result<(String, String, Option<String>, Option<WeightT>), String>,
    >,
    skip_self_loops: bool,
    ignore_duplicated_edges: bool,
) -> Result<(GraphDictionary, Option<Vocabulary<EdgeTypeT>>), String> {
    // save if the node file was loaded or not
    let empty_nodes_mapping: bool = nodes.is_empty();
    // edges mappings
    let mut edge_types_vocabulary: Vocabulary<NodeTypeT> = Vocabulary::default();
    // helper structure
    let mut unique_edges_tree: GraphDictionary = GraphDictionary::new();

    let mut has_edges = None;
    let mut has_weights = None;

    for values in edges_iterator {
        let (source_node_name, destination_node_name, edge_type, edge_weight) = values?;
        // Check if we need to skip self-loops
        if skip_self_loops && source_node_name == destination_node_name {
            // If current edge is a self-loop and we need to skip them we skip.
            continue;
        }
        // check that the values of the weights are reasonable (IF PRESENT)
        if let Some(val) = &edge_weight {
            match val.is_finite() && *val > 0.0 {
                true => Ok(()),
                false => Err(format!(
                    "The weight {} is either infinite or NaN or Zero.",
                    val
                )),
            }?
        }
        // clean way to save if the first edge has edge type and weights
        has_edges = has_edges.or_else(|| Some(edge_type.is_some()));
        has_weights = has_weights.or_else(|| Some(edge_weight.is_some()));

        // check consistency
        if let Some(he) = &has_edges {
            if edge_type.is_some() != *he {
                return Err(format!(
                    concat!(
                        "The edge {} {} {:?} {:?} has edge type inconsistent with all the edges before.\n",
                        "Either all edges have edge types or none have it."
                    ),
                    source_node_name, destination_node_name, edge_type, edge_weight
                ));
            }
        }
        if let Some(hw) = &has_weights {
            if edge_weight.is_some() != *hw {
                return Err(format!(
                    concat!(
                        "The edge {} {} {:?} {:?} has weight inconsistent with all the edges before.\n",
                        "Either all edges have weights or none have it."
                    ),
                    source_node_name, destination_node_name, edge_type, edge_weight
                ));
            }
        }

        // Handle missing node IDs when no node file was provided
        for node_name in &[&source_node_name, &destination_node_name] {
            if !nodes.contains_key(node_name) {
                if empty_nodes_mapping {
                    nodes.insert(node_name.to_string());
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
        // Retrieve the node IDs
        let source_node_id = nodes.get(&source_node_name).unwrap();
        let destinations_node_id = nodes.get(&destination_node_name).unwrap();
        // Retrieve the edge type id if it was given.
        let edge_types_id = if let Some(et) = &edge_type {
            Some(edge_types_vocabulary.insert(et.to_string()))
        } else {
            None
        };

        // Get the metadata of the edge and if it's not present, add it
        let key = (*source_node_id, *destinations_node_id);
        if let Some(metadata) = unique_edges_tree.get_mut(&key) {
            let edge_is_duplicated = match metadata {
                Some(e) => e.contains_edge_type(edge_types_id),
                None => true,
            };
            if edge_is_duplicated {
                if ignore_duplicated_edges {
                    continue;
                }
                return Err(format!(
                    concat!(
                        "Found duplicated edges!\n",
                        "The source node is {source} and the destination node is {destination}.\n",
                        "The edge type of the row is {edge_type:?}.",
                    ),
                    source = source_node_name,
                    destination = destination_node_name,
                    edge_type = edge_type,
                ));
            }
        }

        unique_edges_tree.simple_extend(
            *source_node_id,
            *destinations_node_id,
            edge_types_id,
            edge_weight,
            directed,
        );
    }

    Ok((unique_edges_tree, optionify!(edge_types_vocabulary)))
}

pub(crate) fn build_graph(
    unique_edges_tree: &mut GraphDictionary,
    nodes: Vocabulary<NodeT>,
    node_types: Option<VocabularyVec<NodeTypeT>>,
    edge_types: Option<Vocabulary<EdgeTypeT>>,
    directed: bool,
) -> Graph {
    // structures to fill for the graph
    // outbounds is initialized as vector of values unique edges and with length equal to the number of nodes.
    let mut weights: Vec<WeightT> = Vec::new();
    let mut edge_types_vector: Vec<NodeTypeT> = Vec::new();
    let (max_src, max_dst);
    // We get the tree last value without borrowing it for too long.
    {
        let ((tmp_max_src, tmp_max_dst), _) = unique_edges_tree.last_key_value().unwrap();
        max_src = *tmp_max_src + 1;
        max_dst = *tmp_max_dst + 1;
    }
    let node_bits = get_node_bits(max!(max_src, max_dst));
    let node_bit_mask = (1 << node_bits) - 1;
    let mut edges: EliasFano = EliasFano::new(
        encode_edge(max_src, max_dst, node_bits) as u64,
        unique_edges_tree.len(),
    );

    let pb = ProgressBar::new(unique_edges_tree.len() as u64);
    pb.set_draw_delta(unique_edges_tree.len() as u64 / 100);
    pb.set_style(ProgressStyle::default_bar().template(
        "Building tree {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
    ));
    
    // now that the tree is built
    // we can iter on the edge in order (no further sorting required)
    // during the iteration we pop the minimum value each time
    while !unique_edges_tree.is_empty() {
        // we gradually destroy the tree while we fill the other structures
        // in this way we reduce the memory peak
        // the unwrap is guaranteed to succeed because we check if the tree is empty
        let ((src, dst), mut metadata) = unique_edges_tree.pop_first().unwrap();
        // Reverse the metadata of the edge into the graph vectors
        match &mut metadata {
            Some(m) => {
                m.for_each(|(weight, edge_type)| {
                    edges.unchecked_push(encode_edge(src, dst, node_bits));
                    if let Some(w) = weight {
                        weights.push(w);
                    }
                    if let Some(et) = edge_type {
                        edge_types_vector.push(et)
                    }
                });
            }
            None => {
                edges.unchecked_push(encode_edge(src, dst, node_bits));
            }
        }
        pb.inc(1);
    }
    pb.finish();

    let unique_sources: EliasFano = EliasFano::from_iter(
        edges
            .iter()
            .map(|edge| {
                let (src, _) = decode_edge(edge, node_bits, node_bit_mask);
                src as u64
            })
            .unique(),
        max_src as u64,
        max_src,
    )
    .unwrap();

    let mut graph = Graph {
        nodes,
        edges,
        node_types,
        directed,
        node_bits,
        node_bit_mask,
        unique_sources,
        has_traps: false,
        weights: optionify!(weights),
        edge_types: match edge_types {
            Some(et) => Some(VocabularyVec::<EdgeTypeT> {
                vocabulary: et,
                ids: edge_types_vector,
            }),
            None => None,
        },
    };

    graph.has_traps = !graph.get_trap_nodes().is_empty();

    graph
}

/// # Graph Constructors
impl Graph {
    /// Create new Graph object.
    ///
    /// # Arguments
    ///
    /// * edges_iterator: impl Iterator<Item = Result<(String, String, Option<String>, Option<WeightT>), String>>,
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
    pub fn new(
        edges_iterator: impl Iterator<
            Item = Result<(String, String, Option<String>, Option<WeightT>), String>,
        >,
        nodes_iterator: Option<impl Iterator<Item = Result<(String, Option<String>), String>>>,
        directed: bool,
        ignore_duplicated_nodes: bool,
        ignore_duplicated_edges: bool,
        skip_self_loops: bool,
    ) -> Result<Graph, String> {
        let (mut nodes, node_types) = if let Some(ni) = nodes_iterator {
            parse_nodes(ni, ignore_duplicated_nodes)?
        } else {
            (Vocabulary::default(), None)
        };

        let (mut unique_edges_tree, edge_types_vocabulary) = parse_edges(
            &mut nodes,
            directed,
            edges_iterator,
            skip_self_loops,
            ignore_duplicated_edges,
        )?;

        Ok(build_graph(
            &mut unique_edges_tree,
            nodes,
            node_types,
            edge_types_vocabulary,
            directed,
        ))
    }
}
