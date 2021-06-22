use indicatif::ProgressIterator;
use rand::prelude::SliceRandom;
use rand::prelude::SmallRng;
use rand::SeedableRng;
use rayon::iter::ParallelBridge;
use vec_rand::sorted_unique_sub_sampling;
use vec_rand::xorshift::xorshift;

use super::*;

/// Return number of edges and iterator over edge list of a clique.
///
/// # Arguments
/// * `minimum_node_id`: NodeT - The minimum node ID for the range of the clique.
/// * `maximum_node_id`: NodeT - The maximum node ID for the range of the clique.
/// * `include_selfloops`: bool - Whether to include selfloops in the clique.
/// * `edge_type`: Option<EdgeTypeT> - Edge type for the edges in the chain.
/// * `weight`: Option<WeightT> - Edge weights for the edges in the chain.
///
/// # Safety
/// If the minimum node ID is higher than the maximum node ID the method will cause a panic
/// though to subtraction.
unsafe fn get_clique_edges_iterator_unchecked(
    minimum_node_id: NodeT,
    maximum_node_id: NodeT,
    include_selfloops: bool,
    edge_type: Option<EdgeTypeT>,
    weight: Option<WeightT>,
) -> (
    EdgeT,
    impl Iterator<Item = Result<(NodeT, NodeT, Option<EdgeTypeT>, Option<WeightT>), String>>,
) {
    let total_nodes = maximum_node_id - minimum_node_id;
    let total_edges = if total_nodes == 0 {
        0
    } else {
        total_nodes as EdgeT
            * if include_selfloops {
                total_nodes
            } else {
                total_nodes - 1
            } as EdgeT
    };
    (
        total_edges,
        (minimum_node_id..maximum_node_id)
            .map(move |src_node_id| {
                (minimum_node_id..maximum_node_id)
                    .filter(|&dst_node_id| include_selfloops || src_node_id != dst_node_id)
                    .map(|dst_node_id| Ok((src_node_id, dst_node_id, edge_type, weight)))
                    .collect::<Vec<_>>()
            })
            .flatten(),
    )
}

/// Return number of edges and iterator over edge list of a chain.
///
/// # Arguments
/// * `minimum_node_id`: NodeT - The minimum node ID for the range of the chain.
/// * `maximum_node_id`: NodeT - The maximum node ID for the range of the chain.
/// * `include_selfloops`: bool - Whether to include selfloops in the chain.
/// * `edge_type`: Option<EdgeTypeT> - Edge type for the edges in the chain.
/// * `weight`: Option<WeightT> - Edge weights for the edges in the chain.
///
/// # Safety
/// If the minimum node ID is higher than the maximum node ID the method will cause a panic
/// though to subtraction.
unsafe fn get_chain_edges_iterator_unchecked(
    minimum_node_id: NodeT,
    maximum_node_id: NodeT,
    include_selfloops: bool,
    edge_type: Option<EdgeTypeT>,
    weight: Option<WeightT>,
) -> (
    EdgeT,
    impl Iterator<Item = Result<(NodeT, NodeT, Option<EdgeTypeT>, Option<WeightT>), String>>,
) {
    let total_nodes = maximum_node_id - minimum_node_id;
    let total_edges = if total_nodes == 0 {
        0
    } else {
        ((total_nodes - 1) as EdgeT) * 2 + if include_selfloops { total_nodes } else { 0 } as EdgeT
    };
    (
        total_edges,
        (minimum_node_id..maximum_node_id)
            .map(move |src_node_id| {
                let contextual_minimum = if src_node_id == minimum_node_id {
                    minimum_node_id
                } else {
                    src_node_id - 1
                };
                let contextual_maximum = if src_node_id == maximum_node_id {
                    maximum_node_id
                } else {
                    src_node_id + 1
                };
                (contextual_minimum..contextual_maximum)
                    .filter(|&dst_node_id| include_selfloops || src_node_id != dst_node_id)
                    .map(|dst_node_id| Ok((src_node_id, dst_node_id, edge_type, weight)))
                    .collect::<Vec<_>>()
            })
            .flatten(),
    )
}

/// Return number of edges and iterator over edge list of a circle.
///
/// # Arguments
/// * `minimum_node_id`: NodeT - The minimum node ID for the range of the circle.
/// * `maximum_node_id`: NodeT - The maximum node ID for the range of the circle.
/// * `include_selfloops`: bool - Whether to include selfloops in the circle.
/// * `edge_type`: Option<EdgeTypeT> - Edge type for the edges in the circle.
/// * `weight`: Option<WeightT> - Edge weights for the edges in the circle.
///
/// # Safety
/// If the minimum node ID is higher than the maximum node ID the method will cause a panic
/// though to subtraction.
unsafe fn get_circle_edges_iterator(
    minimum_node_id: NodeT,
    maximum_node_id: NodeT,
    include_selfloops: bool,
    edge_type: Option<EdgeTypeT>,
    weight: Option<WeightT>,
) -> (
    EdgeT,
    impl Iterator<Item = Result<(NodeT, NodeT, Option<EdgeTypeT>, Option<WeightT>), String>>,
) {
    let total_nodes = maximum_node_id - minimum_node_id;
    let total_edges = if total_nodes == 0 {
        0
    } else {
        ((total_nodes - 1) as EdgeT) * 2 + if include_selfloops { total_nodes } else { 0 } as EdgeT
    };
    (
        total_edges,
        (minimum_node_id..maximum_node_id)
            .map(move |src_node_id| {
                let contextual_minimum = if src_node_id == minimum_node_id {
                    minimum_node_id
                } else {
                    src_node_id - 1
                };
                let contextual_maximum = if src_node_id == maximum_node_id - 1 {
                    maximum_node_id - 1
                } else {
                    src_node_id + 1
                };
                let mut result = (contextual_minimum..contextual_maximum)
                    .filter(|&dst_node_id| include_selfloops || src_node_id != dst_node_id)
                    .map(|dst_node_id| Ok((src_node_id, dst_node_id, edge_type, weight)))
                    .collect::<Vec<_>>();
                // In order close the circle
                // we connected the first node to the last
                // if we did not already do within this cell
                if src_node_id == minimum_node_id && contextual_maximum != maximum_node_id - 1 {
                    result.push(Ok((src_node_id, maximum_node_id - 1, edge_type, weight)));
                // And the last to the first
                // if we did not already do within this cell
                } else if src_node_id == maximum_node_id - 1
                    && contextual_minimum != minimum_node_id
                {
                    result.push(Ok((src_node_id, minimum_node_id, edge_type, weight)));
                }
                result
            })
            .flatten(),
    )
}

/// Return number of edges and iterator over edge list of a random tree.
///
/// # Arguments
/// * `random_state`: u64 - The random state to use to reproduce the sampling.
/// * `minimum_node_id`: NodeT - The minimum node ID for the range of the tree.
/// * `maximum_node_id`: NodeT - The maximum node ID for the range of the tree.
/// * `include_selfloops`: bool - Whether to include selfloops in the chain.
/// * `minimum_node_sampling`: NodeT - The minimum amount of nodes to sample per node.
/// * `maximum_node_sampling`: NodeT - The maximum amount of nodes to sample per node.
/// * `edge_type`: Option<EdgeTypeT> - Edge type for the edges in the tree.
/// * `weight`: Option<WeightT> - Edge weights for the edges in the tree.
///
/// # Safety
/// If the minimum node ID is higher than the maximum node ID the method will cause a panic
/// though to subtraction.
unsafe fn get_random_connected_graph_edges_iterator(
    mut random_state: u64,
    minimum_node_id: NodeT,
    maximum_node_id: NodeT,
    include_selfloops: bool,
    minimum_node_sampling: NodeT,
    maximum_node_sampling: NodeT,
    edge_type: Option<EdgeTypeT>,
    weight: Option<WeightT>,
) -> impl Iterator<Item = Result<(NodeT, NodeT, Option<EdgeTypeT>, Option<WeightT>), String>> {
    let total_nodes = maximum_node_id - minimum_node_id;
    let mut node_ids = (minimum_node_id..maximum_node_id).collect::<Vec<_>>();
    random_state = splitmix64(random_state);
    let mut rng = SmallRng::seed_from_u64(random_state);
    node_ids.shuffle(&mut rng);
    (0..total_nodes)
        .map(move |current_position| {
            random_state = xorshift(random_state);
            let quantity = current_position.min(
                minimum_node_sampling
                    + random_state as NodeT % (maximum_node_sampling - minimum_node_sampling),
            );
            let dst_node_id = node_ids[current_position as usize];
            let mut result = sorted_unique_sub_sampling(
                0,
                current_position as u64,
                quantity as u64,
                random_state,
            )
            .unwrap()
            .into_iter()
            .map(|src_position| {
                let src_node_id = node_ids[src_position as usize];

                vec![
                    Ok((src_node_id, dst_node_id, edge_type, weight)),
                    Ok((dst_node_id, src_node_id, edge_type, weight)),
                ]
            })
            .flatten()
            .collect::<Vec<_>>();
            if include_selfloops {
                result.push(Ok((dst_node_id, dst_node_id, edge_type, weight)));
            }
            result
        })
        .flatten()
}

/// Return number of edges and iterator over edge list of a random tree.
///
/// # Arguments
/// * `random_state`: u64 - The random state to use to reproduce the sampling.
/// * `minimum_node_id`: NodeT - The minimum node ID for the range of the tree.
/// * `maximum_node_id`: NodeT - The maximum node ID for the range of the tree.
/// * `include_selfloops`: bool - Whether to include selfloops in the chain.
/// * `edge_type`: Option<EdgeTypeT> - Edge type for the edges in the tree.
/// * `weight`: Option<WeightT> - Edge weights for the edges in the tree.
///
/// # Safety
/// If the minimum node ID is higher than the maximum node ID the method will cause a panic
/// though to subtraction.
unsafe fn get_random_spanning_tree_edges_iterator(
    random_state: u64,
    minimum_node_id: NodeT,
    maximum_node_id: NodeT,
    include_selfloops: bool,
    edge_type: Option<EdgeTypeT>,
    weight: Option<WeightT>,
) -> impl Iterator<Item = Result<(NodeT, NodeT, Option<EdgeTypeT>, Option<WeightT>), String>> {
    let total_nodes = maximum_node_id - minimum_node_id;
    let total_edges = if total_nodes == 0 {
        0
    } else {
        (total_nodes - 1) * 2
    };
    get_random_connected_graph_edges_iterator(
        random_state,
        minimum_node_id,
        maximum_node_id,
        include_selfloops,
        1,
        1,
        edge_type,
        weight,
    )
}

/// # Methods to generate random graphs
impl Graph {
    /// Creates new random connected graph with given sizes and types.
    ///
    /// # Arguments
    /// * `random_state`: u64 - The random state to use to reproduce the sampling.
    /// * `minimum_node_id`: NodeT - The minimum node ID for the connected graph.
    /// * `minimum_node_sampling`: NodeT - The minimum amount of nodes to sample per node.
    /// * `maximum_node_sampling`: NodeT - The maximum amount of nodes to sample per node.
    /// * `nodes_number`: Option<NodeT> - Number of nodes in the chain. By default 10.
    /// * `include_selfloops`: Option<bool> - Whether to include selfloops.
    /// * `node_type`: Option<&str> - The node type to use for the chain. By default 'chain'.
    /// * `edge_type`: Option<&str> - The node type to use for the chain. By default 'chain'.
    /// * `weight`: Option<WeightT> - The weight to use for the edges in the chain. By default None.
    /// * `directed`: Option<bool> - Whether the graph is to built as directed. By default false.
    /// * `name`: Option<&str> - Name of the graph. By default 'Chain'.
    /// * `verbose`: Option<bool> - Whether to show a loading bar while building the edge list.
    ///
    pub fn generate_random_connected_graph(
        random_state: Option<u64>,
        minimum_node_id: Option<NodeT>,
        minimum_node_sampling: Option<NodeT>,
        maximum_node_sampling: Option<NodeT>,
        nodes_number: Option<NodeT>,
        include_selfloops: Option<bool>,
        node_type: Option<&str>,
        edge_type: Option<&str>,
        weight: Option<WeightT>,
        directed: Option<bool>,
        name: Option<&str>,
        verbose: Option<bool>,
    ) -> Result<Graph, String> {
        let random_state = random_state.unwrap_or(42);
        let nodes_number = nodes_number.unwrap_or(10);
        let minimum_node_sampling = minimum_node_sampling.unwrap_or(1);
        let maximum_node_sampling = maximum_node_sampling.unwrap_or(5);
        let minimum_node_id = minimum_node_id.unwrap_or(0);
        let include_selfloops = include_selfloops.unwrap_or(false);
        let directed = directed.unwrap_or(false);
        let node_type = node_type.unwrap_or("connected");
        let mut node_types_vocabulary: Vocabulary<NodeTypeT> = Vocabulary::with_capacity(1);
        let node_type = unsafe { node_types_vocabulary.unchecked_insert(node_type.to_string()) };
        // TODO! replace with method that handles properly homogeneous node types!
        let node_type_id = Some(vec![node_type]);
        let node_type_ids = (0..nodes_number).map(|_| node_type_id).collect::<Vec<_>>();
        let node_types = NodeTypeVocabulary::from_structs(node_type_ids, node_types_vocabulary);

        let edge_type = edge_type.unwrap_or("connected");
        let mut edge_types_vocabulary: Vocabulary<EdgeTypeT> = Vocabulary::with_capacity(1);
        let edge_type = unsafe { edge_types_vocabulary.unchecked_insert(edge_type.to_string()) };
        let nodes = Vocabulary::from_range(minimum_node_id..(minimum_node_id + nodes_number));
        let name = name.unwrap_or("Connected");
        let verbose = verbose.unwrap_or(true);

        // Get the generator the chain in the middle of the two cliques
        let edges_iterator = unsafe {
            get_random_connected_graph_edges_iterator(
                random_state,
                0,
                nodes_number,
                include_selfloops,
                minimum_node_sampling,
                maximum_node_sampling,
                Some(edge_type),
                weight,
            )
        };

        Graph::from_integer_unsorted(
            // TODO! After having parallelized the constructor
            // refactor the methods to generate random graphs
            edges_iterator.par_bridge(),
            nodes,
            Some(node_types),
            Some(edge_types_vocabulary),
            directed,
            // TODO! Add `S` INTO for the string name
            name.to_string(),
            false,
            true,
            // It is enough to check if any of the edge weights provided
            // is not None, as we check beforehand that either all of them
            // are None or none are.
            weight.is_some(),
            weight.is_some(),
            // This graph contains singletons only if the total number of
            // nodes that are requested is one and no edges are requested.
            nodes_number == 1 && !include_selfloops,
            // This graph contains singletons with selfloops only if the total number of
            // nodes that are requested is one and selfloops are requested.
            nodes_number == 1 && include_selfloops,
            false,
            verbose,
        )
    }

    /// Creates new random connected graph with given sizes and types.
    ///
    /// # Arguments
    /// * `random_state`: u64 - The random state to use to reproduce the sampling.
    /// * `minimum_node_id`: NodeT - The minimum node ID for the connected graph.
    /// * `minimum_node_sampling`: NodeT - The minimum amount of nodes to sample per node.
    /// * `maximum_node_sampling`: NodeT - The maximum amount of nodes to sample per node.
    /// * `nodes_number`: Option<NodeT> - Number of nodes in the chain. By default 10.
    /// * `include_selfloops`: Option<bool> - Whether to include selfloops.
    /// * `node_type`: Option<&str> - The node type to use for the chain. By default 'chain'.
    /// * `edge_type`: Option<&str> - The node type to use for the chain. By default 'chain'.
    /// * `weight`: Option<WeightT> - The weight to use for the edges in the chain. By default None.
    /// * `directed`: Option<bool> - Whether the graph is to built as directed. By default false.
    /// * `name`: Option<&str> - Name of the graph. By default 'Chain'.
    /// * `verbose`: Option<bool> - Whether to show a loading bar while building the edge list.
    ///
    pub fn generate_random_spanning_tree(
        random_state: Option<u64>,
        minimum_node_id: Option<NodeT>,
        nodes_number: Option<NodeT>,
        include_selfloops: Option<bool>,
        node_type: Option<&str>,
        edge_type: Option<&str>,
        weight: Option<WeightT>,
        directed: Option<bool>,
        name: Option<&str>,
        verbose: Option<bool>,
    ) -> Result<Graph, String> {
        let random_state = random_state.unwrap_or(42);
        let nodes_number = nodes_number.unwrap_or(10);
        let minimum_node_id = minimum_node_id.unwrap_or(0);
        let include_selfloops = include_selfloops.unwrap_or(false);
        let directed = directed.unwrap_or(false);
        let node_type = node_type.unwrap_or("connected");
        let mut node_types_vocabulary: Vocabulary<NodeTypeT> = Vocabulary::with_capacity(1);
        let node_type = unsafe { node_types_vocabulary.unchecked_insert(node_type.to_string()) };
        // TODO! replace with method that handles properly homogeneous node types!
        let node_type_id = Some(vec![node_type]);
        let node_type_ids = (0..nodes_number).map(|_| node_type_id).collect::<Vec<_>>();
        let node_types = NodeTypeVocabulary::from_structs(node_type_ids, node_types_vocabulary);

        let edge_type = edge_type.unwrap_or("connected");
        let mut edge_types_vocabulary: Vocabulary<EdgeTypeT> = Vocabulary::with_capacity(1);
        let edge_type = unsafe { edge_types_vocabulary.unchecked_insert(edge_type.to_string()) };
        let nodes = Vocabulary::from_range(minimum_node_id..(minimum_node_id + nodes_number));
        let name = name.unwrap_or("Connected");
        let verbose = verbose.unwrap_or(true);

        // Get the generator the chain in the middle of the two cliques
        let edges_iterator = unsafe {
            get_random_spanning_tree_edges_iterator(
                random_state,
                0,
                nodes_number,
                include_selfloops,
                Some(edge_type),
                weight,
            )
        };

        Graph::from_integer_unsorted(
            // TODO! After having parallelized the constructor
            // refactor the methods to generate random graphs
            edges_iterator.par_bridge(),
            nodes,
            Some(node_types),
            Some(edge_types_vocabulary),
            directed,
            // TODO! Add `S` INTO for the string name
            name.to_string(),
            false,
            true,
            // It is enough to check if any of the edge weights provided
            // is not None, as we check beforehand that either all of them
            // are None or none are.
            weight.is_some(),
            weight.is_some(),
            // This graph contains singletons only if the total number of
            // nodes that are requested is one and no edges are requested.
            nodes_number == 1 && !include_selfloops,
            // This graph contains singletons with selfloops only if the total number of
            // nodes that are requested is one and selfloops are requested.
            nodes_number == 1 && include_selfloops,
            false,
            verbose,
        )
    }

    /// Creates new circle graph with given sizes and types.
    ///
    /// # Arguments
    /// * `minimum_node_id`: Option<NodeT> - Minimum node ID to start with. May be needed when circleing graphs. By default 0.
    /// * `nodes_number`: Option<NodeT> - Number of nodes in the circle. By default 10.
    /// * `include_selfloops`: Option<bool> - Whether to include selfloops.
    /// * `node_type`: Option<&str> - The node type to use for the circle. By default 'circle'.
    /// * `edge_type`: Option<&str> - The node type to use for the circle. By default 'circle'.
    /// * `weight`: Option<WeightT> - The weight to use for the edges in the circle. By default None.
    /// * `directed`: Option<bool> - Whether the graph is to built as directed. By default false.
    /// * `name`: Option<&str> - Name of the graph. By default 'Circle'.
    /// * `verbose`: Option<bool> - Whether to show a loading bar while building the edge list.
    ///
    pub fn generate_circle_graph(
        minimum_node_id: Option<NodeT>,
        nodes_number: Option<NodeT>,
        include_selfloops: Option<bool>,
        node_type: Option<&str>,
        edge_type: Option<&str>,
        weight: Option<WeightT>,
        directed: Option<bool>,
        name: Option<&str>,
        verbose: Option<bool>,
    ) -> Result<Graph, String> {
        let nodes_number = nodes_number.unwrap_or(10);
        let minimum_node_id = minimum_node_id.unwrap_or(0);
        let include_selfloops = include_selfloops.unwrap_or(false);
        let directed = directed.unwrap_or(false);
        let node_type = node_type.unwrap_or("circle");
        let mut node_types_vocabulary: Vocabulary<NodeTypeT> = Vocabulary::with_capacity(1);
        let node_type = unsafe { node_types_vocabulary.unchecked_insert(node_type.to_string()) };
        // TODO! replace with method that handles properly homogeneous node types!
        let node_type_id = Some(vec![node_type]);
        let node_type_ids = (0..nodes_number).map(|_| node_type_id).collect::<Vec<_>>();
        let node_types = NodeTypeVocabulary::from_structs(node_type_ids, node_types_vocabulary);

        let edge_type = edge_type.unwrap_or("circle");
        let mut edge_types_vocabulary: Vocabulary<EdgeTypeT> = Vocabulary::with_capacity(1);
        let edge_type = unsafe { edge_types_vocabulary.unchecked_insert(edge_type.to_string()) };
        let nodes = Vocabulary::from_range(minimum_node_id..(minimum_node_id + nodes_number));
        let name = name.unwrap_or("Circle");
        let verbose = verbose.unwrap_or(true);

        // Get the generator the circle in the middle of the two cliques
        let (edges_number, edges_iterator) = unsafe {
            get_circle_edges_iterator(0, nodes_number, include_selfloops, Some(edge_type), weight)
        };

        let pb = get_loading_bar(verbose, "Building circle graph", edges_number as usize);

        Graph::from_integer_sorted(
            edges_iterator.progress_with(pb),
            edges_number as usize,
            nodes,
            Some(node_types),
            Some(edge_types_vocabulary),
            directed,
            true,
            name,
            false,
            true,
            // It is enough to check if any of the edge weights provided
            // is not None, as we check beforehand that either all of them
            // are None or none are.
            weight.is_some(),
            weight.is_some(),
            // This graph contains singletons only if the total number of
            // nodes that are requested is one and no edges are requested.
            nodes_number == 1 && edges_number == 0,
            // This graph contains singletons with selfloops only if the total number of
            // nodes that are requested is one and selfloops are requested.
            nodes_number == 1 && include_selfloops,
            false,
        )
    }

    /// Creates new chain graph with given sizes and types.
    ///
    /// # Arguments
    /// * `minimum_node_id`: Option<NodeT> - Minimum node ID to start with. May be needed when chaining graphs. By default 0.
    /// * `nodes_number`: Option<NodeT> - Number of nodes in the chain. By default 10.
    /// * `include_selfloops`: Option<bool> - Whether to include selfloops.
    /// * `node_type`: Option<&str> - The node type to use for the chain. By default 'chain'.
    /// * `edge_type`: Option<&str> - The node type to use for the chain. By default 'chain'.
    /// * `weight`: Option<WeightT> - The weight to use for the edges in the chain. By default None.
    /// * `directed`: Option<bool> - Whether the graph is to built as directed. By default false.
    /// * `name`: Option<&str> - Name of the graph. By default 'Chain'.
    /// * `verbose`: Option<bool> - Whether to show a loading bar while building the edge list.
    ///
    pub fn generate_chain_graph(
        minimum_node_id: Option<NodeT>,
        nodes_number: Option<NodeT>,
        include_selfloops: Option<bool>,
        node_type: Option<&str>,
        edge_type: Option<&str>,
        weight: Option<WeightT>,
        directed: Option<bool>,
        name: Option<&str>,
        verbose: Option<bool>,
    ) -> Result<Graph, String> {
        let nodes_number = nodes_number.unwrap_or(10);
        let minimum_node_id = minimum_node_id.unwrap_or(0);
        let include_selfloops = include_selfloops.unwrap_or(false);
        let directed = directed.unwrap_or(false);
        let node_type = node_type.unwrap_or("chain");
        let mut node_types_vocabulary: Vocabulary<NodeTypeT> = Vocabulary::with_capacity(1);
        let node_type = unsafe { node_types_vocabulary.unchecked_insert(node_type.to_string()) };
        // TODO! replace with method that handles properly homogeneous node types!
        let node_type_id = Some(vec![node_type]);
        let node_type_ids = (0..nodes_number).map(|_| node_type_id).collect::<Vec<_>>();
        let node_types = NodeTypeVocabulary::from_structs(node_type_ids, node_types_vocabulary);

        let edge_type = edge_type.unwrap_or("chain");
        let mut edge_types_vocabulary: Vocabulary<EdgeTypeT> = Vocabulary::with_capacity(1);
        let edge_type = unsafe { edge_types_vocabulary.unchecked_insert(edge_type.to_string()) };
        let nodes = Vocabulary::from_range(minimum_node_id..(minimum_node_id + nodes_number));
        let name = name.unwrap_or("Chain");
        let verbose = verbose.unwrap_or(true);

        // Get the generator the chain in the middle of the two cliques
        let (edges_number, edges_iterator) = unsafe {
            get_chain_edges_iterator_unchecked(0, nodes_number, include_selfloops, Some(edge_type), weight)
        };

        let pb = get_loading_bar(verbose, "Building chain graph", edges_number as usize);

        Graph::from_integer_sorted(
            edges_iterator.progress_with(pb),
            edges_number as usize,
            nodes,
            Some(node_types),
            Some(edge_types_vocabulary),
            directed,
            true,
            name,
            false,
            true,
            // It is enough to check if any of the edge weights provided
            // is not None, as we check beforehand that either all of them
            // are None or none are.
            weight.is_some(),
            weight.is_some(),
            // This graph contains singletons only if the total number of
            // nodes that are requested is one and no edges are requested.
            nodes_number == 1 && edges_number == 0,
            // This graph contains singletons with selfloops only if the total number of
            // nodes that are requested is one and selfloops are requested.
            nodes_number == 1 && include_selfloops,
            false,
        )
    }

    /// Creates new complete graph with given sizes and types.
    ///
    /// # Arguments
    /// * `minimum_node_id`: Option<NodeT> - Minimum node ID to start with. May be needed when combining graphs. By default 0.
    /// * `nodes_number`: Option<NodeT> - Number of nodes in the chain. By default 10.
    /// * `include_selfloops`: Option<bool> - Whether to include selfloops.
    /// * `node_type`: Option<&str> - The node type to use. By default 'complete'.
    /// * `edge_type`: Option<&str> - The node type to use. By default 'complete'.
    /// * `weight`: Option<WeightT> - The weight to use for the edges. By default None.
    /// * `directed`: Option<bool> - Whether the graph is to built as directed. By default false.
    /// * `name`: Option<&str> - Name of the graph. By default 'Complete'.
    /// * `verbose`: Option<bool> - Whether to show a loading bar while building the edge list.
    ///
    pub fn generate_complete_graph(
        minimum_node_id: Option<NodeT>,
        nodes_number: Option<NodeT>,
        include_selfloops: Option<bool>,
        node_type: Option<&str>,
        edge_type: Option<&str>,
        weight: Option<WeightT>,
        directed: Option<bool>,
        name: Option<&str>,
        verbose: Option<bool>,
    ) -> Result<Graph, String> {
        let nodes_number = nodes_number.unwrap_or(10);
        let minimum_node_id = minimum_node_id.unwrap_or(0);
        let include_selfloops = include_selfloops.unwrap_or(false);
        let directed = directed.unwrap_or(false);
        let node_type = node_type.unwrap_or("complete");
        let mut node_types_vocabulary: Vocabulary<NodeTypeT> = Vocabulary::with_capacity(1);
        let node_type = unsafe { node_types_vocabulary.unchecked_insert(node_type.to_string()) };
        // TODO! replace with method that handles properly homogeneous node types!
        let node_type_id = Some(vec![node_type]);
        let node_type_ids = (0..nodes_number).map(|_| node_type_id).collect::<Vec<_>>();
        let node_types = NodeTypeVocabulary::from_structs(node_type_ids, node_types_vocabulary);

        let edge_type = edge_type.unwrap_or("complete");
        let mut edge_types_vocabulary: Vocabulary<EdgeTypeT> = Vocabulary::with_capacity(1);
        let edge_type = unsafe { edge_types_vocabulary.unchecked_insert(edge_type.to_string()) };
        let nodes = Vocabulary::from_range(minimum_node_id..(minimum_node_id + nodes_number));
        let name = name.unwrap_or("Complete");
        let verbose = verbose.unwrap_or(true);

        // Get the generator the chain in the middle of the two cliques
        let (edges_number, edges_iterator) = unsafe {
            get_clique_edges_iterator_unchecked(0, nodes_number, include_selfloops, Some(edge_type), weight)
        };

        let pb = get_loading_bar(verbose, "Building complete graph", edges_number as usize);

        Graph::from_integer_sorted(
            edges_iterator.progress_with(pb),
            edges_number as usize,
            nodes,
            Some(node_types),
            Some(edge_types_vocabulary),
            directed,
            true,
            name,
            false,
            true,
            // It is enough to check if any of the edge weights provided
            // is not None, as we check beforehand that either all of them
            // are None or none are.
            weight.is_some(),
            weight.is_some(),
            // This graph contains singletons only if the total number of
            // nodes that are requested is one and no edges are requested.
            nodes_number == 1 && edges_number == 0,
            // This graph contains singletons with selfloops only if the total number of
            // nodes that are requested is one and selfloops are requested.
            nodes_number == 1 && include_selfloops,
            false,
        )
    }

    /// Creates new barbell graph with given sizes and types.
    ///
    /// # Arguments
    /// * `minimum_node_id`: Option<NodeT> - Minimum node ID to start with. May be needed when chaining graphs. By default 0.
    /// * `left_clique_nodes_number`: Option<NodeT> - Number of nodes in the left clique. By default 10.
    /// * `right_clique_nodes_number`: Option<NodeT> -  Number of nodes in the right clique. By default equal to the left clique.
    /// * `chain_nodes_number`: Option<NodeT> - Number of nodes in the chain. By default 10.
    /// * `include_selfloops`: Option<bool> - Whether to include selfloops.
    /// * `left_clique_node_type`: Option<&str> - The node type to use for the left clique. By default 'left_clique'.
    /// * `right_clique_node_type`: Option<&str> - The node type to use for the right clique. By default 'right_clique'.
    /// * `chain_node_type`: Option<&str> - The node type to use for the chain. By default 'chain'.
    /// * `left_clique_edge_type`: Option<&str> - The node type to use for the left clique. By default 'left_clique'.
    /// * `right_clique_edge_type`: Option<&str> - The node type to use for the right clique. By default 'right_clique'.
    /// * `chain_edge_type`: Option<&str> - The node type to use for the chain. By default 'chain'.
    /// * `left_clique_weight`: Option<WeightT> - The weight to use for the edges in the left clique. By default None.
    /// * `right_clique_weight`: Option<WeightT> - The weight to use for the edges in the right clique. By default None.
    /// * `chain_weight`: Option<WeightT> - The weight to use for the edges in the chain. By default None.
    /// * `directed`: Option<bool> - Whether the graph is to built as directed. By default false.
    /// * `name`: Option<&str> - Name of the graph. By default 'Barbell'.
    /// * `verbose`: Option<bool> - Whether to show a loading bar while building the edge list.
    ///
    /// # Raises
    /// * If the edge weights are provided only for a subset.
    pub fn generate_barbell_graph(
        minimum_node_id: Option<NodeT>,
        left_clique_nodes_number: Option<NodeT>,
        right_clique_nodes_number: Option<NodeT>,
        chain_nodes_number: Option<NodeT>,
        include_selfloops: Option<bool>,
        left_clique_node_type: Option<&str>,
        right_clique_node_type: Option<&str>,
        chain_node_type: Option<&str>,
        left_clique_edge_type: Option<&str>,
        right_clique_edge_type: Option<&str>,
        chain_edge_type: Option<&str>,
        left_clique_weight: Option<WeightT>,
        right_clique_weight: Option<WeightT>,
        chain_weight: Option<WeightT>,
        directed: Option<bool>,
        name: Option<&str>,
        verbose: Option<bool>,
    ) -> Result<Graph, String> {
        match (left_clique_weight, right_clique_weight, chain_weight) {
            (None, None, None) | (Some(_), Some(_), Some(_)) => Ok(()),
            _ => Err("The edge weights have been provided only for a subset of the graph sub-structures.".to_string())
        }?;
        let minimum_node_id = minimum_node_id.unwrap_or(0);
        let left_clique_nodes_number = left_clique_nodes_number.unwrap_or(10);
        let chain_nodes_number = chain_nodes_number.unwrap_or(10);
        let include_selfloops = include_selfloops.unwrap_or(false);
        let directed = directed.unwrap_or(false);
        let right_clique_nodes_number =
            right_clique_nodes_number.unwrap_or(left_clique_nodes_number);
        let nodes_number =
            left_clique_nodes_number + chain_nodes_number + right_clique_nodes_number;
        let left_clique_node_type = left_clique_node_type.unwrap_or("left_clique");
        let right_clique_node_type = right_clique_node_type.unwrap_or("right_clique");
        let chain_node_type = chain_node_type.unwrap_or("chain");
        let mut node_types_vocabulary: Vocabulary<NodeTypeT> = Vocabulary::with_capacity(3);
        let left_clique_node_type =
            unsafe { node_types_vocabulary.unchecked_insert(left_clique_node_type.to_string()) };
        let right_clique_node_type =
            unsafe { node_types_vocabulary.unchecked_insert(right_clique_node_type.to_string()) };
        let chain_node_type =
            unsafe { node_types_vocabulary.unchecked_insert(chain_node_type.to_string()) };
        let mut node_type_ids = vec![None; nodes_number as usize];
        // Set the node types of the nodes on the left clique
        if left_clique_nodes_number > 0 {
            let left_clique_node_type_ids = Some(vec![left_clique_node_type]);
            for i in 0..left_clique_nodes_number {
                node_type_ids[i as usize] = left_clique_node_type_ids.clone();
            }
        }
        // Set the node types of the nodes on the chain
        if chain_nodes_number > 0 {
            // Fix the welding points node types
            if left_clique_nodes_number > 0 {
                if let Some(&mut last_left_clique_node_types) =
                    node_type_ids[(left_clique_nodes_number - 1) as usize].as_mut()
                {
                    last_left_clique_node_types.push(chain_node_type);
                }
            }
            let chain_node_type_ids = Some(vec![chain_node_type]);
            for i in left_clique_nodes_number..(left_clique_nodes_number + chain_nodes_number) {
                node_type_ids[i as usize] = chain_node_type_ids.clone();
            }
        }
        // Set the node types of the nodes on the right clique
        if right_clique_nodes_number > 0 {
            let total_previous_nodes = left_clique_nodes_number + chain_nodes_number;
            // Fix the welding points node types
            if total_previous_nodes > 0 {
                if let Some(&mut last_left_clique_node_types) =
                    node_type_ids[(total_previous_nodes - 1) as usize].as_mut()
                {
                    last_left_clique_node_types.push(right_clique_node_type);
                }
            }
            let right_clique_node_type_ids = Some(vec![right_clique_node_type]);
            for i in total_previous_nodes..(total_previous_nodes + right_clique_nodes_number) {
                node_type_ids[i as usize] = right_clique_node_type_ids.clone();
            }
        }

        let node_types = NodeTypeVocabulary::from_structs(node_type_ids, node_types_vocabulary);

        let left_clique_edge_type = left_clique_edge_type.unwrap_or("left_clique");
        let right_clique_edge_type = right_clique_edge_type.unwrap_or("right_clique");
        let chain_edge_type = chain_edge_type.unwrap_or("chain");
        let mut edge_types_vocabulary: Vocabulary<EdgeTypeT> = Vocabulary::with_capacity(3);
        let left_clique_edge_type =
            unsafe { edge_types_vocabulary.unchecked_insert(left_clique_edge_type.to_string()) };
        let right_clique_edge_type =
            unsafe { edge_types_vocabulary.unchecked_insert(right_clique_edge_type.to_string()) };
        let chain_edge_type =
            unsafe { edge_types_vocabulary.unchecked_insert(chain_edge_type.to_string()) };
        let nodes = Vocabulary::from_range(minimum_node_id..(minimum_node_id + nodes_number));
        let name = name.unwrap_or("Barbell");
        let verbose = verbose.unwrap_or(true);

        // Get the generator for the left clique
        let (left_edges_number, left_clique_edges_iterator) = unsafe {
            get_clique_edges_iterator_unchecked(
                0,
                left_clique_nodes_number,
                include_selfloops,
                Some(left_clique_edge_type),
                left_clique_weight,
            )
        };
        // Get the generator the chain in the middle of the two cliques
        let (chain_edges_number, chain_edges_iterator) = unsafe {
            get_chain_edges_iterator_unchecked(
                left_clique_nodes_number,
                left_clique_nodes_number + chain_nodes_number,
                include_selfloops,
                Some(chain_edge_type),
                chain_weight,
            )
        };
        // Get the generator for the right clique
        let (right_edges_number, right_clique_edges_iterator) = unsafe {
            get_clique_edges_iterator_unchecked(
                left_clique_nodes_number + chain_nodes_number,
                left_clique_nodes_number + chain_nodes_number + right_clique_nodes_number,
                include_selfloops,
                Some(right_clique_edge_type),
                right_clique_weight,
            )
        };

        let edges_number = left_edges_number + chain_edges_number + right_edges_number;

        let pb = get_loading_bar(verbose, "Building barbell graph", edges_number as usize);

        let edges_iterator = left_clique_edges_iterator
            .chain(chain_edges_iterator)
            .chain(right_clique_edges_iterator)
            .progress_with(pb);

        Graph::from_integer_sorted(
            edges_iterator,
            edges_number as usize,
            nodes,
            Some(node_types),
            Some(edge_types_vocabulary),
            directed,
            true,
            name,
            false,
            true,
            // It is enough to check if any of the edge weights provided
            // is not None, as we check beforehand that either all of them
            // are None or none are.
            left_clique_weight.is_some(),
            left_clique_weight.is_some(),
            // This graph contains singletons only if the total number of
            // nodes that are requested is one and no edges are requested.
            nodes_number == 1 && edges_number == 0,
            // This graph contains singletons with selfloops only if the total number of
            // nodes that are requested is one and selfloops are requested.
            nodes_number == 1 && include_selfloops,
            false,
        )
    }
}
