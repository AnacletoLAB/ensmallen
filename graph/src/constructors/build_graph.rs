use super::*;
use rayon::{iter::Empty, prelude::*};

/// Return new graph object built from string iterators.
///
/// # Arguments
/// `node_types_iterator`: Option<impl ParallelIterator<Item = Result<(usize, String)>>> - Iterator over the provided node types list.
/// `node_types_number`: Option<NodeTypeT> - The node types number, if known. It makes loading them faster.
/// `numeric_node_type_ids`: Option<bool> - Whether the provided node types are to be loaded as numeric.
/// `minimum_node_type_id`: Option<NodeTypeT> - The minimum node type ID, if they are numeric.
/// `has_node_types`: bool - Whether the graph is expected to have node types.
/// `nodes_iterator`: Option<impl ParallelIterator<Item = Result<(usize, (String, Option<Vec<String>>))>>> - Iterator over the provided node list.
/// `nodes_number`: Option<NodeT> - The number of nodes in the the graph, if known. It makes loading them faster.
/// `node_list_is_correct`: bool - Whether the node list is correct and checks can be skipped.
/// `numeric_node_ids`: bool - Whether to load the node IDs as numeric.
/// `numeric_node_list_node_type_ids`: bool - Whether to load the node type IDs as numeric.
/// `minimum_node_ids`: Option<NodeT> - The minimum node ID, if they are numeric.
/// `edge_types_iterator`: Option<impl ParallelIterator<Item = Result<(usize, String)>>> - Iterator over the provided edge type list.
/// `edge_types_number`: Option<EdgeTypeT> - The edge types number, if known. It makes loading them faster.
/// `numeric_edge_type_ids`: Option<bool> - Whether the provided edge type IDs are to be loaded as numeric.
/// `minimum_edge_type_id`: Option<EdgeTypeT> - The minimum edge type ID, if the are numeric.
/// `has_edge_types`: bool - Whether the graph has edge types.
/// `edges_iterator`: Option<impl ParallelIterator<Item = Result<(usize, (String, String, Option<String>, WeightT))>>,> - Iterator over the provided edge list.
/// `has_edge_weights`: bool - Whether the graph has edge weights.
/// `directed`: bool - Whether the graph is meant to be loaded as directed or undirected.
/// `edge_list_is_correct`: Option<bool> - Whether the edge list is correct and checks can be skipped.
/// `complete`: Option<bool> - Whether the edge list is complete, i.e. fully defined for undirected graphs in both directions.
/// `duplicates`: Option<bool> - Whether there may be duplicated edges in the graph.
/// `sorted`: Option<bool> - Whether the provided edge list is sorted.
/// `edges_number`: Option<EdgeT> - The number of edges in the graph, if known.
/// `numeric_edge_list_node_ids`: Option<bool> - Whether the provided node IDs in the edge list are to be loaded as numeric.
/// `numeric_edge_list_edge_type_ids`: Option<bool> - Whether the provided edge type IDs in the edge list are to be loaded as numeric.
/// `name: S - The name of the graph.
///
pub(crate) fn build_graph_from_strings<S: Into<String>>(
    node_types_iterator: Option<impl ParallelIterator<Item = Result<(usize, String)>>>,
    node_types_number: Option<NodeTypeT>,
    numeric_node_type_ids: Option<bool>,
    minimum_node_type_id: Option<NodeTypeT>,
    has_node_types: bool,
    nodes_iterator: Option<
        impl ParallelIterator<Item = Result<(usize, (String, Option<Vec<String>>))>>,
    >,
    nodes_number: Option<NodeT>,
    node_list_is_correct: bool,
    numeric_node_ids: bool,
    numeric_node_list_node_type_ids: bool,
    minimum_node_ids: Option<NodeT>,
    edge_types_iterator: Option<impl ParallelIterator<Item = Result<(usize, String)>>>,
    edge_types_number: Option<EdgeTypeT>,
    numeric_edge_type_ids: Option<bool>,
    minimum_edge_type_id: Option<EdgeTypeT>,
    has_edge_types: bool,
    edges_iterator: Option<
        impl ParallelIterator<Item = Result<(usize, (String, String, Option<String>, WeightT))>>,
    >,
    has_edge_weights: bool,
    directed: bool,
    correct: Option<bool>,
    complete: Option<bool>,
    duplicates: Option<bool>,
    sorted: Option<bool>,
    edges_number: Option<EdgeT>,
    numeric_edge_list_node_ids: Option<bool>,
    numeric_edge_list_edge_type_ids: Option<bool>,
    name: S,
) -> Result<Graph> {
    let node_types_vocabulary = parse_types(
        node_types_iterator,
        node_types_number,
        numeric_node_type_ids,
        minimum_node_type_id,
        has_node_types,
    )?;
    let (nodes, node_types) = parse_nodes(
        nodes_iterator,
        nodes_number,
        node_types_vocabulary,
        node_list_is_correct,
        numeric_node_ids,
        numeric_node_list_node_type_ids,
        minimum_node_ids,
    )?;
    let edge_types_vocabulary = parse_types(
        edge_types_iterator,
        edge_types_number,
        numeric_edge_type_ids,
        minimum_edge_type_id,
        has_edge_types,
    )?;
    let (nodes, edges, edge_types, weights) = parse_string_edges(
        edges_iterator,
        nodes,
        edge_types_vocabulary,
        has_edge_types,
        has_edge_weights,
        directed,
        correct,
        complete,
        duplicates,
        sorted,
        edges_number,
        numeric_edge_list_node_ids,
        numeric_edge_list_edge_type_ids,
    )?;
    Ok(Graph::new(
        directed, nodes, node_types, edges, edge_types, weights, name,
    ))
}

/// Return new graph object built from string iterators.
///
/// # Arguments
/// `has_node_types`: bool - Whether the graph is expected to have node types.
/// `nodes_iterator`: Option<impl ParallelIterator<Item = Result<(usize, (String, Option<Vec<String>>))>>> - Iterator over the provided node list.
/// `nodes_number`: Option<NodeT> - The number of nodes in the the graph, if known. It makes loading them faster.
/// `node_list_is_correct`: bool - Whether the node list is correct and checks can be skipped.
/// `numeric_node_ids`: bool - Whether to load the node IDs as numeric.
/// `numeric_node_list_node_type_ids`: bool - Whether to load the node type IDs as numeric.
/// `minimum_node_ids`: Option<NodeT> - The minimum node ID, if they are numeric.
/// `has_edge_types`: bool - Whether the graph has edge types.
/// `edges_iterator`: Option<impl ParallelIterator<Item = Result<(usize, (String, String, Option<String>, WeightT))>>,> - Iterator over the provided edge list.
/// `has_edge_weights`: bool - Whether the graph has edge weights.
/// `directed`: bool - Whether the graph is meant to be loaded as directed or undirected.
/// `edge_list_is_correct`: Option<bool> - Whether the edge list is correct and checks can be skipped.
/// `complete`: Option<bool> - Whether the edge list is complete, i.e. fully defined for undirected graphs in both directions.
/// `duplicates`: Option<bool> - Whether there may be duplicated edges in the graph.
/// `sorted`: Option<bool> - Whether the provided edge list is sorted.
/// `edges_number`: Option<EdgeT> - The number of edges in the graph, if known.
/// `numeric_edge_list_node_ids`: Option<bool> - Whether the provided node IDs in the edge list are to be loaded as numeric.
/// `numeric_edge_list_edge_type_ids`: Option<bool> - Whether the provided edge type IDs in the edge list are to be loaded as numeric.
/// `name: S - The name of the graph.
///
pub(crate) fn build_graph_from_strings_without_type_iterators<S: Into<String>>(
    has_node_types: bool,
    nodes_iterator: Option<
        impl ParallelIterator<Item = Result<(usize, (String, Option<Vec<String>>))>>,
    >,
    nodes_number: Option<NodeT>,
    node_list_is_correct: bool,
    numeric_node_ids: bool,
    numeric_node_list_node_type_ids: bool,
    minimum_node_ids: Option<NodeT>,
    has_edge_types: bool,
    edges_iterator: Option<
        impl ParallelIterator<Item = Result<(usize, (String, String, Option<String>, WeightT))>>,
    >,
    has_edge_weights: bool,
    directed: bool,
    correct: Option<bool>,
    complete: Option<bool>,
    duplicates: Option<bool>,
    sorted: Option<bool>,
    edges_number: Option<EdgeT>,
    numeric_edge_list_node_ids: Option<bool>,
    numeric_edge_list_edge_type_ids: Option<bool>,
    name: S,
) -> Result<Graph> {
    build_graph_from_strings(
        None::<Empty<_>>,
        None,
        None,
        None,
        has_node_types,
        nodes_iterator,
        nodes_number,
        node_list_is_correct,
        numeric_node_ids,
        numeric_node_list_node_type_ids,
        minimum_node_ids,
        None::<Empty<_>>,
        None,
        None,
        None,
        has_edge_types,
        edges_iterator,
        has_edge_weights,
        directed,
        correct,
        complete,
        duplicates,
        sorted,
        edges_number,
        numeric_edge_list_node_ids,
        numeric_edge_list_edge_type_ids,
        name,
    )
}

/// Return new graph object built from integer iterators.
///
/// # Arguments
/// `edges_iterator`: Option<Vec<impl ParallelIterator<Item = Result<(usize, (NodeT, NodeT, Option<EdgeTypeT>, WeightT))>>,>,> - Iterator over the provided numeric edge list.
/// `nodes`: Vocabulary<NodeT> - The node vocabulary.
/// `node_types`: Option<NodeTypeVocabulary> - The node types vocabulary, if they exist in this graph.
/// `edge_types_vocabulary`: Option<Vocabulary<EdgeTypeT>> - The edge types vocabulary, if they exist in this graph.
/// `has_edge_weights`: bool - Whether this graph has edge weights.
/// `directed`: bool - Whether the graph is meant to be loaded as directed or undirected.
/// `complete`: Option<bool> - Whether the edge list is complete, i.e. fully defined for undirected graphs in both directions.
/// `duplicates`: Option<bool> - Whether there may be duplicated edges in the graph.
/// `sorted`: Option<bool> - Whether the provided edge list is sorted.
/// `edges_number`: Option<EdgeT> - The number of edges in the graph, if known.
/// `name`: S - The name of the graph.
pub(crate) fn build_graph_from_integers<S: Into<String>>(
    edges_iterator: Option<
        impl ParallelIterator<Item = (usize, (NodeT, NodeT, Option<EdgeTypeT>, WeightT))>,
    >,
    nodes: Vocabulary<NodeT>,
    node_types: Option<NodeTypeVocabulary>,
    edge_types_vocabulary: Option<Vocabulary<EdgeTypeT>>,
    has_edge_weights: bool,
    directed: bool,
    complete: Option<bool>,
    duplicates: Option<bool>,
    sorted: Option<bool>,
    edges_number: Option<EdgeT>,
    name: S,
) -> Result<Graph> {
    let (edges, edge_types, weights) = parse_integer_edges(
        edges_iterator,
        nodes.len() as NodeT,
        edge_types_vocabulary,
        has_edge_weights,
        directed,
        complete,
        duplicates,
        sorted,
        edges_number,
    )?;
    Ok(Graph::new(
        directed, nodes, node_types, edges, edge_types, weights, name,
    ))
}
