use super::*;
use rayon::prelude::*;
use std::sync::Arc;

#[manual_binding]
/// Return new graph object built from string iterators.
///
/// # Arguments
/// * `node_types_iterator`: Option<impl ParallelIterator<Item = Result<(usize, String)>>> - Iterator over the provided node types list.
/// * `number_of_node_types`: Option<NodeTypeT> - The node types number, if known. It makes loading them faster.
/// * `numeric_node_type_ids`: Option<bool> - Whether the provided node types are to be loaded as numeric.
/// * `minimum_node_type_id`: Option<NodeTypeT> - The minimum node type ID, if they are numeric.
/// * `has_node_types`: bool - Whether the graph is expected to have node types.
/// * `nodes_iterator`: Option<impl ParallelIterator<Item = Result<(usize, (String, Option<Vec<String>>))>>> - Iterator over the provided node list.
/// * `number_of_nodes`: Option<NodeT> - The number of nodes in the the graph, if known. It makes loading them faster.
/// * `node_list_is_correct`: bool - Whether the node list is correct and checks can be skipped.
/// * `numeric_node_ids`: bool - Whether to load the node IDs as numeric.
/// * `numeric_node_list_node_type_ids`: bool - Whether to load the node type IDs as numeric.
/// * `minimum_node_ids`: Option<NodeT> - The minimum node ID, if they are numeric.
/// * `edge_types_iterator`: Option<impl ParallelIterator<Item = Result<(usize, String)>>> - Iterator over the provided edge type list.
/// * `number_of_edge_types`: Option<EdgeTypeT> - The edge types number, if known. It makes loading them faster.
/// * `numeric_edge_type_ids`: Option<bool> - Whether the provided edge type IDs are to be loaded as numeric.
/// * `minimum_edge_type_id`: Option<EdgeTypeT> - The minimum edge type ID, if the are numeric.
/// * `has_edge_types`: bool - Whether the graph has edge types.
/// * `edges_iterator`: Option<impl ParallelIterator<Item = Result<(usize, (String, String, Option<String>, WeightT))>>,> - Iterator over the provided edge list.
/// * `has_edge_weights`: bool - Whether the graph has edge weights.
/// * `directed`: bool - Whether the graph is meant to be loaded as directed or undirected.
/// * `edge_list_is_correct`: Option<bool> - Whether the edge list is correct and checks can be skipped.
/// * `complete`: Option<bool> - Whether the edge list is complete, i.e. fully defined for undirected graphs in both directions.
/// * `duplicates`: Option<bool> - Whether there may be duplicated edges in the graph.
/// * `sorted`: Option<bool> - Whether the provided edge list is sorted.
/// * `number_of_edges`: Option<EdgeT> - The number of edges in the graph, if known.
/// * `numeric_edge_list_node_ids`: Option<bool> - Whether the provided node IDs in the edge list are to be loaded as numeric.
/// * `numeric_edge_list_edge_type_ids`: Option<bool> - Whether the provided edge type IDs in the edge list are to be loaded as numeric.
/// * `may_have_singletons`: bool - Whether the graph may contain singletons.
/// * `may_have_singleton_with_selfloops`: bool - Whether the graph may contain singleton with selfloops.
/// * `name: S - The name of the graph.
///
pub fn build_graph_from_strings<S: Into<String>>(
    node_types_iterator: Option<
        ItersWrapper<
            Result<(usize, String)>,
            impl Iterator<Item = Result<(usize, String)>>,
            impl ParallelIterator<Item = Result<(usize, String)>>,
        >,
    >,
    number_of_node_types: Option<NodeTypeT>,
    numeric_node_type_ids: Option<bool>,
    minimum_node_type_id: Option<NodeTypeT>,
    has_node_types: bool,
    node_types_list_is_correct: Option<bool>,
    nodes_iterator: Option<
        ItersWrapper<
            Result<(usize, (String, Option<Vec<String>>))>,
            impl Iterator<Item = Result<(usize, (String, Option<Vec<String>>))>>,
            impl ParallelIterator<Item = Result<(usize, (String, Option<Vec<String>>))>>,
        >,
    >,
    mut number_of_nodes: Option<NodeT>,
    node_list_is_correct: bool,
    numeric_node_ids: bool,
    numeric_node_list_node_type_ids: bool,
    minimum_node_ids: Option<NodeT>,
    edge_types_iterator: Option<
        ItersWrapper<
            Result<(usize, String)>,
            impl Iterator<Item = Result<(usize, String)>>,
            impl ParallelIterator<Item = Result<(usize, String)>>,
        >,
    >,
    number_of_edge_types: Option<EdgeTypeT>,
    numeric_edge_type_ids: Option<bool>,
    minimum_edge_type_id: Option<EdgeTypeT>,
    has_edge_types: bool,
    edge_types_list_is_correct: Option<bool>,
    edges_iterator: Option<
        ItersWrapper<
            Result<(usize, StringQuadruple)>,
            impl Iterator<Item = Result<(usize, StringQuadruple)>>,
            impl ParallelIterator<Item = Result<(usize, StringQuadruple)>>,
        >,
    >,
    has_edge_weights: bool,
    directed: bool,
    correct: Option<bool>,
    complete: Option<bool>,
    duplicates: Option<bool>,
    sorted: Option<bool>,
    number_of_edges: Option<EdgeT>,
    numeric_edge_list_node_ids: Option<bool>,
    numeric_edge_list_edge_type_ids: Option<bool>,
    skip_node_types_if_unavailable: Option<bool>,
    skip_edge_types_if_unavailable: Option<bool>,
    mut may_have_singletons: bool,
    may_have_singleton_with_selfloops: bool,
    name: S,
) -> Result<Graph> {
    // If the user has requested to use the advanced feature of the
    // numeric edge list node IDs, and also requires the node list to be
    // loaded in parallel without knowing the number of nodes, then we
    // cannot load the graph. This is because the parallel loader of the
    // node list is not an IndexedParallelIterator, and thus we cannot
    // collect the nodes in the correct place in the nodes vector.
    // If we know the number of nodes in advance, we can pre-allocate a vector
    // of the correct size, and populate it in parallel ensuring that the
    // nodes are placed in the correct position. Without this information,
    // we would be forced to collect all of the nodes in a single vector,
    // and then sort them by their ID, which is not feasible for large graphs.
    if numeric_edge_list_node_ids.unwrap_or(false)
        && nodes_iterator
            .as_ref()
            .map_or(false, |iter| iter.is_parallel())
        && number_of_nodes.is_none()
    {
        // We compute the number of nodes, so we can actually provide to the user
        // what parameter to provide to the graph builder method when calling
        // it in the future. We cannot actually use this value to proceed to load the
        // graph as, in most real large graphs, procesing the entire node list solely
        // to count the number of nodes is not feasible and adds a significant overhead.

        let number_of_nodes = nodes_iterator.unwrap().count();

        // And after we have computed the number of nodes, we can return a rich
        // and informative error to the user, explaining why we cannot proceed to load the graph
        // and what they can do to fix the issue. We need to put in the error:
        //
        // - The reason for the error.
        // - The name of the graph.
        // - The number of nodes.

        return Err(
            format!(
                concat!(
                    "You have a selected a bad configuration of the graph builder for graph {}.\n",
                    "You have requested to load the node list in parallel while also using the numeric edge list node IDs feature. ",
                    "You have NOT provided the number of nodes in the node list.\n",
                    "Since we are loading the node list in parallel, and the iterator cannot be made indexed, the order ",
                    "of the nodes may change. If we know in advance the number of nodes, we can pre-allocate a vector ",
                    "of the correct size, and populate it in parallel ensuring that the nodes are placed in the correct position. ",
                    "This would allow for the mapping between the node list and the edge list to be maintained in the case of ",
                    "numeric node IDs. Without this information, we would be forced to collect all of the nodes in a single vector, ",
                    "and then sort them by their ID, which is not feasible for large graphs as it adds a significant overhead.\n",
                    "To fix this issue, you can add the number of nodes in the node list to the graph builder, by providing the ",
                    "parameter `number_of_nodes = {}` (we have just computed this value for you). ",
                ),
                name.into(),
                number_of_nodes,
            )
        );
    }

    // We need to do an analogous check for the parametrization provided for the node types.

    if numeric_node_list_node_type_ids
        && node_types_iterator
            .as_ref()
            .map_or(false, |iter| iter.is_parallel())
        && number_of_node_types.is_none()
    {
        // We compute the number of nodes, so we can actually provide to the user
        // what parameter to provide to the graph builder method when calling
        // it in the future. We cannot actually use this value to proceed to load the
        // graph as, in most real large graphs, procesing the entire node list solely
        // to count the number of nodes is not feasible and adds a significant overhead.

        let number_of_node_types = node_types_iterator.unwrap().count();

        // And after we have computed the number of nodes, we can return a rich
        // and informative error to the user, explaining why we cannot proceed to load the graph
        // and what they can do to fix the issue. We need to put in the error:
        //
        // - The reason for the error.
        // - The name of the graph.
        // - The number of node types

        return Err(
            format!(
                concat!(
                    "You have a selected a bad configuration of the graph builder for graph {}.\n",
                    "You have requested to load the node type list in parallel while also using the numeric node list node type IDs feature. ",
                    "You have NOT provided the number of node types in the node type list.\n",
                    "Since we are loading the node type list in parallel, and the iterator cannot be made indexed, the order ",
                    "of the node types may change. If we know in advance the number of node types, we can pre-allocate a vector ",
                    "of the correct size, and populate it in parallel ensuring that the node types are placed in the correct position. ",
                    "This would allow for the mapping between the node type list and the node list to be maintained in the case of ",
                    "numeric node type IDs. Without this information, we would be forced to collect all of the node types in a single vector, ",
                    "and then sort them by their ID, which is not feasible for large graphs as it adds a significant overhead.\n",
                    "To fix this issue, you can add the number of node types in the node type list to the graph builder, by providing the ",
                    "parameter `number_of_node_types = {}` (we have just computed this value for you). ",
                ),
                name.into(),
                number_of_node_types,
            )
        );
    }

    // We need to do an analogous check for the parametrization provided for the edge types.

    if numeric_edge_list_edge_type_ids.unwrap_or(false)
        && edge_types_iterator
            .as_ref()
            .map_or(false, |iter| iter.is_parallel())
        && number_of_edge_types.is_none()
    {
        // We compute the number of edge types, so we can actually provide to the user
        // what parameter to provide to the graph builder method when calling
        // it in the future. We cannot actually use this value to proceed to load the
        // graph as, in most real large graphs, procesing the entire edge type list solely
        // to count the number of edge types is not feasible and adds a significant overhead.

        let number_of_edge_types = edge_types_iterator.unwrap().count();

        // And after we have computed the number of edge types, we can return a rich
        // and informative error to the user, explaining why we cannot proceed to load the graph
        // and what they can do to fix the issue. We need to put in the error:
        //
        // - The reason for the error.
        // - The name of the graph.
        // - The number of edge types.

        return Err(
            format!(
                concat!(
                    "You have a selected a bad configuration of the graph builder for graph {}.\n",
                    "You have requested to load the edge type list in parallel while also using the numeric edge list edge type IDs feature. ",
                    "You have NOT provided the number of edge types in the edge type list.\n",
                    "Since we are loading the edge type list in parallel, and the iterator cannot be made indexed, the order ",
                    "of the edge types may change. If we know in advance the number of edge types, we can pre-allocate a vector ",
                    "of the correct size, and populate it in parallel ensuring that the edge types are placed in the correct position. ",
                    "This would allow for the mapping between the edge type list and the edge list to be maintained in the case of ",
                    "numeric edge type IDs. Without this information, we would be forced to collect all of the edge types in a single vector, ",
                    "and then sort them by their ID, which is not feasible for large graphs as it adds a significant overhead.\n",
                    "To fix this issue, you can add the number of edge types in the edge type list to the graph builder, by providing the ",
                    "parameter `number_of_edge_types = {}` (we have just computed this value for you). ",
                ),
                name.into(),
                number_of_edge_types,
            )
        );
    }

    let node_types_vocabulary = parse_types(
        node_types_iterator,
        number_of_node_types,
        numeric_node_type_ids,
        minimum_node_type_id,
        has_node_types,
        node_types_list_is_correct,
    )?;

    let nodes_iterator_was_provided = nodes_iterator.is_some();
    let (nodes, node_types) = parse_nodes(
        nodes_iterator,
        number_of_nodes,
        node_types_vocabulary,
        node_list_is_correct,
        numeric_node_ids,
        numeric_node_list_node_type_ids,
        minimum_node_ids,
        skip_node_types_if_unavailable,
    )?;

    // If the number of nodes was not known
    // and a nodes iterator was provided, we can fill the gap.
    if number_of_nodes.is_none() && nodes_iterator_was_provided {
        number_of_nodes.replace(nodes.len() as NodeT);
    }

    // If the iterator of the nodes was NOT provided,
    // then there cannot be singleton nodes becase
    // any node will be loaded from the
    // edge list. This only applies to the case when
    // also the nodes are not Numeric, otherwise there
    // may be singletons implicitly in the range.
    may_have_singletons &= nodes_iterator_was_provided || nodes.is_numeric();

    let edge_types_vocabulary = parse_types(
        edge_types_iterator,
        number_of_edge_types,
        numeric_edge_type_ids,
        minimum_edge_type_id,
        has_edge_types,
        edge_types_list_is_correct,
    )?;

    let (nodes, edges, edge_types, weights, has_selfloops) = parse_string_edges(
        edges_iterator,
        nodes,
        edge_types_vocabulary,
        has_edge_weights,
        directed,
        correct,
        complete,
        duplicates,
        sorted,
        number_of_nodes,
        number_of_edges,
        numeric_edge_list_node_ids,
        numeric_edge_list_edge_type_ids,
        skip_edge_types_if_unavailable,
    )?;

    Ok(Graph::new(
        directed,
        Arc::new(nodes),
        Arc::new(node_types),
        Arc::new(edges),
        Arc::new(edge_types),
        Arc::new(weights),
        may_have_singletons,
        may_have_singleton_with_selfloops && has_selfloops,
        name,
    ))
}

#[manual_binding]
/// Return new graph object built from string iterators.
///
/// # Arguments
/// * `has_node_types`: bool - Whether the graph is expected to have node types.
/// * `nodes_iterator`: Option<impl ParallelIterator<Item = Result<(usize, (String, Option<Vec<String>>))>>> - Iterator over the provided node list.
/// * `number_of_nodes`: Option<NodeT> - The number of nodes in the the graph, if known. It makes loading them faster.
/// * `node_list_is_correct`: bool - Whether the node list is correct and checks can be skipped.
/// * `numeric_node_ids`: bool - Whether to load the node IDs as numeric.
/// * `numeric_node_list_node_type_ids`: bool - Whether to load the node type IDs as numeric.
/// * `minimum_node_ids`: Option<NodeT> - The minimum node ID, if they are numeric.
/// * `has_edge_types`: bool - Whether the graph has edge types.
/// * `edges_iterator`: Option<impl ParallelIterator<Item = Result<(usize, (String, String, Option<String>, WeightT))>>,> - Iterator over the provided edge list.
/// * `has_edge_weights`: bool - Whether the graph has edge weights.
/// * `directed`: bool - Whether the graph is meant to be loaded as directed or undirected.
/// * `edge_list_is_correct`: Option<bool> - Whether the edge list is correct and checks can be skipped.
/// * `complete`: Option<bool> - Whether the edge list is complete, i.e. fully defined for undirected graphs in both directions.
/// * `duplicates`: Option<bool> - Whether there may be duplicated edges in the graph.
/// * `sorted`: Option<bool> - Whether the provided edge list is sorted.
/// * `number_of_edges`: Option<EdgeT> - The number of edges in the graph, if known.
/// * `numeric_edge_list_node_ids`: Option<bool> - Whether the provided node IDs in the edge list are to be loaded as numeric.
/// * `numeric_edge_list_edge_type_ids`: Option<bool> - Whether the provided edge type IDs in the edge list are to be loaded as numeric.
/// * `name: S - The name of the graph.
///
pub fn build_graph_from_strings_without_type_iterators<S: Into<String>>(
    has_node_types: bool,
    nodes_iterator: Option<
        ItersWrapper<
            Result<(usize, (String, Option<Vec<String>>))>,
            impl Iterator<Item = Result<(usize, (String, Option<Vec<String>>))>>,
            impl ParallelIterator<Item = Result<(usize, (String, Option<Vec<String>>))>>,
        >,
    >,
    number_of_nodes: Option<NodeT>,
    node_list_is_correct: bool,
    numeric_node_ids: bool,
    numeric_node_list_node_type_ids: bool,
    minimum_node_ids: Option<NodeT>,
    has_edge_types: bool,
    edges_iterator: Option<
        ItersWrapper<
            Result<(usize, StringQuadruple)>,
            impl Iterator<Item = Result<(usize, StringQuadruple)>>,
            impl ParallelIterator<Item = Result<(usize, StringQuadruple)>>,
        >,
    >,
    has_edge_weights: bool,
    directed: bool,
    correct: Option<bool>,
    complete: Option<bool>,
    duplicates: Option<bool>,
    sorted: Option<bool>,
    number_of_edges: Option<EdgeT>,
    numeric_edge_list_node_ids: Option<bool>,
    numeric_edge_list_edge_type_ids: Option<bool>,
    skip_node_types_if_unavailable: Option<bool>,
    skip_edge_types_if_unavailable: Option<bool>,
    may_have_singletons: bool,
    may_have_singleton_with_selfloops: bool,
    name: S,
) -> Result<Graph> {
    build_graph_from_strings(
        None::<ItersWrapper<_, std::iter::Empty<_>, rayon::iter::Empty<_>>>,
        None,
        None,
        None,
        has_node_types,
        None,
        nodes_iterator,
        number_of_nodes,
        node_list_is_correct,
        numeric_node_ids,
        numeric_node_list_node_type_ids,
        minimum_node_ids,
        None::<ItersWrapper<_, std::iter::Empty<_>, rayon::iter::Empty<_>>>,
        None,
        None,
        None,
        has_edge_types,
        None,
        edges_iterator,
        has_edge_weights,
        directed,
        correct,
        complete,
        duplicates,
        sorted,
        number_of_edges,
        numeric_edge_list_node_ids,
        numeric_edge_list_edge_type_ids,
        skip_node_types_if_unavailable,
        skip_edge_types_if_unavailable,
        may_have_singletons,
        may_have_singleton_with_selfloops,
        name,
    )
}

/// Return new graph object built from string iterators.
///
/// # Arguments
/// `directed`: bool - Whether the graph is meant to be loaded as directed or undirected.
/// `name: S - The name of the graph.
///
pub fn build_empty_graph<S: Into<String>>(directed: bool, name: S) -> Result<Graph> {
    build_graph_from_strings_without_type_iterators(
        false,
        None::<ItersWrapper<_, std::iter::Empty<_>, rayon::iter::Empty<_>>>,
        None,
        false,
        false,
        false,
        None,
        false,
        None::<ItersWrapper<_, std::iter::Empty<_>, rayon::iter::Empty<_>>>,
        false,
        directed,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        false,
        false,
        name,
    )
}

#[manual_binding]
/// Return new graph object built from integer iterators.
///
/// # Arguments
/// * `edges_iterator`: Option<Vec<impl ParallelIterator<Item = Result<(usize, (NodeT, NodeT, Option<EdgeTypeT>, WeightT))>>,>,> - Iterator over the provided numeric edge list.
/// * `nodes`: Vocabulary<NodeT> - The node vocabulary.
/// * `node_types`: Option<NodeTypeVocabulary> - The node types vocabulary, if they exist in this graph.
/// * `edge_types_vocabulary`: Option<Vocabulary<EdgeTypeT>> - The edge types vocabulary, if they exist in this graph.
/// * `has_edge_weights`: bool - Whether this graph has edge weights.
/// * `directed`: bool - Whether the graph is meant to be loaded as directed or undirected.
/// * `complete`: Option<bool> - Whether the edge list is complete, i.e. fully defined for undirected graphs in both directions.
/// * `duplicates`: Option<bool> - Whether there may be duplicated edges in the graph.
/// * `sorted`: Option<bool> - Whether the provided edge list is sorted.
/// * `number_of_edges`: Option<EdgeT> - The number of edges in the graph, if known.
/// * `may_have_singletons`: bool - Whether the graph may contain singletons.
/// * `may_have_singleton_with_selfloops`: bool - Whether the graph may contain singleton with selfloops.
/// * `name`: S - The name of the graph.
pub fn build_graph_from_integers<S: Into<String>>(
    edges_iterator: Option<
        impl ParallelIterator<Item = (usize, (NodeT, NodeT, Option<EdgeTypeT>, WeightT))>,
    >,
    nodes: Arc<Vocabulary<NodeT>>,
    node_types: Arc<Option<NodeTypeVocabulary>>,
    edge_types_vocabulary: Option<Vocabulary<EdgeTypeT>>,
    has_edge_weights: bool,
    directed: bool,
    complete: Option<bool>,
    duplicates: Option<bool>,
    sorted: Option<bool>,
    number_of_edges: Option<EdgeT>,
    may_have_singletons: bool,
    may_have_singleton_with_selfloops: bool,
    name: S,
) -> Result<Graph> {
    let (edges, edge_types, weights, has_selfloops) = parse_integer_edges(
        edges_iterator,
        nodes.len() as NodeT,
        edge_types_vocabulary,
        has_edge_weights,
        directed,
        complete,
        duplicates,
        sorted,
        number_of_edges,
    )?;
    Ok(Graph::new(
        directed,
        nodes.clone(),
        node_types.clone(),
        Arc::new(edges),
        Arc::new(edge_types),
        Arc::new(weights),
        may_have_singletons,
        may_have_singleton_with_selfloops && has_selfloops,
        name,
    ))
}
