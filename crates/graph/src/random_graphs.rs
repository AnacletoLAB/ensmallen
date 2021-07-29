use num_traits::Zero;
use rand::prelude::SliceRandom;
use rand::prelude::SmallRng;
use rand::SeedableRng;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use vec_rand::sorted_unique_sub_sampling;
use vec_rand::{splitmix64, xorshift::xorshift};

use crate::constructors::build_graph_from_integers;
use crate::graph::Graph;
use crate::vocabularies::*;
use shared::*;

/// Return number of edges and iterator over edge list of a clique.
///
/// # Arguments
/// * `minimum_node_id`: NodeT - The minimum node ID for the range of the clique.
/// * `maximum_node_id`: NodeT - The maximum node ID for the range of the clique.
/// * `include_selfloops`: bool - Whether to include selfloops in the clique.
/// * `edge_type`: Option<EdgeTypeT> - Edge type for the edges in the chain.
/// * `weight`: Option<WeightT> - Edge weights for the edges in the chain.
/// * `edge_id_offset`: usize - How many edges come before the first of this list.
///
/// # Safety
/// If the minimum node ID is higher than the maximum node ID the method will cause a panic
/// though to subtraction.
unsafe fn get_clique_edges_iterator_unchecked(
    minimum_node_id: NodeT,
    maximum_node_id: NodeT,
    include_selfloops: bool,
    edge_type: Option<EdgeTypeT>,
    weight: WeightT,
    edge_id_offset: usize,
) -> (
    EdgeT,
    impl ParallelIterator<Item = (usize, (NodeT, NodeT, Option<EdgeTypeT>, WeightT))>,
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
        ((minimum_node_id as usize)..(maximum_node_id as usize))
            .into_par_iter()
            .enumerate()
            .flat_map(move |(i, src_node_id)| {
                (minimum_node_id..maximum_node_id)
                    .filter(|&dst_node_id| include_selfloops || src_node_id as NodeT != dst_node_id)
                    .enumerate()
                    .map(|(j, dst_node_id)| {
                        (
                            edge_id_offset
                                + i * (total_nodes - (!include_selfloops) as NodeT) as usize
                                + j,
                            (src_node_id as NodeT, dst_node_id, edge_type, weight),
                        )
                    })
                    .collect::<Vec<_>>()
            }),
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
/// * `edge_id_offset`: usize - How many edges come before the first of this list.
///
/// # Safety
/// If the minimum node ID is higher than the maximum node ID the method will cause a panic
/// though to subtraction.
unsafe fn get_chain_edges_iterator_unchecked(
    minimum_node_id: NodeT,
    maximum_node_id: NodeT,
    include_selfloops: bool,
    edge_type: Option<EdgeTypeT>,
    weight: WeightT,
    edge_id_offset: usize,
) -> (
    EdgeT,
    impl ParallelIterator<Item = (usize, (NodeT, NodeT, Option<EdgeTypeT>, WeightT))>,
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
            .into_par_iter()
            .enumerate()
            .map(move |(i, src_node_id)| {
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
                let offset = edge_id_offset
                    + match i {
                        0 => 0,
                        _ => 1 + i * include_selfloops as usize + 2 * (i - 1),
                    };
                (contextual_minimum..=contextual_maximum)
                    .filter(|&dst_node_id| include_selfloops || src_node_id != dst_node_id)
                    .enumerate()
                    .map(|(j, dst_node_id)| {
                        (offset + j, (src_node_id, dst_node_id, edge_type, weight))
                    })
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
/// * `edge_id_offset`: usize - How many edges come before the first of this list.
///
/// # Safety
/// If the minimum node ID is higher than the maximum node ID the method will cause a panic
/// though to subtraction.
unsafe fn get_circle_edges_iterator(
    minimum_node_id: NodeT,
    maximum_node_id: NodeT,
    include_selfloops: bool,
    edge_type: Option<EdgeTypeT>,
    weight: WeightT,
    edge_id_offset: usize,
) -> (
    EdgeT,
    impl ParallelIterator<Item = (usize, (NodeT, NodeT, Option<EdgeTypeT>, WeightT))>,
) {
    let total_nodes = maximum_node_id - minimum_node_id;
    let total_edges = if total_nodes == 0 {
        0
    } else {
        total_nodes * 2 + if include_selfloops { total_nodes } else { 0 }
    } as EdgeT;
    (
        total_edges,
        (minimum_node_id..maximum_node_id)
            .into_par_iter()
            .enumerate()
            .map(move |(i, src_node_id)| {
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
                let mut offset = edge_id_offset + i * include_selfloops as usize + 2 * i;
                let has_to_close_circle =
                    src_node_id == maximum_node_id - 1 && contextual_minimum != minimum_node_id;
                if has_to_close_circle {
                    offset += 1;
                }
                let mut result = (contextual_minimum..=contextual_maximum)
                    .filter(|&dst_node_id| include_selfloops || src_node_id != dst_node_id)
                    .enumerate()
                    .map(|(j, dst_node_id)| {
                        (offset + j, (src_node_id, dst_node_id, edge_type, weight))
                    })
                    .collect::<Vec<_>>();
                // In order close the circle
                // we connected the first node to the last
                // if we did not already do within this cell
                if src_node_id == minimum_node_id && contextual_maximum != maximum_node_id - 1 {
                    result.push((
                        offset + result.len(),
                        (src_node_id, maximum_node_id - 1, edge_type, weight),
                    ));
                // And the last to the first
                // if we did not already do within this cell
                } else if has_to_close_circle {
                    result.push((
                        offset - 1,
                        (src_node_id, minimum_node_id, edge_type, weight),
                    ));
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
    weight: WeightT,
) -> impl ParallelIterator<Item = (usize, (NodeT, NodeT, Option<EdgeTypeT>, WeightT))> {
    let total_nodes = maximum_node_id - minimum_node_id;
    let mut node_ids = (minimum_node_id..maximum_node_id).collect::<Vec<_>>();
    random_state = splitmix64(random_state);
    let mut rng = SmallRng::seed_from_u64(random_state);
    node_ids.shuffle(&mut rng);
    (0..total_nodes as usize)
        .into_par_iter()
        .flat_map(move |current_position| {
            let random_state = xorshift(random_state + current_position as u64);
            let quantity = (current_position as NodeT).min(
                minimum_node_sampling
                    + match maximum_node_sampling - minimum_node_sampling {
                        0 => 0,
                        delta => random_state as NodeT % delta,
                    },
            );
            if quantity.is_zero() {
                return vec![];
            }
            let dst_node_id = node_ids[current_position];
            let mut result = sorted_unique_sub_sampling(
                0,
                current_position as u64,
                quantity as u64,
                random_state,
            )
            .unwrap()
            .into_iter()
            .flat_map(|src_position| {
                let src_node_id = node_ids[src_position as usize];
                // We use 0 because it is not possible
                // to know how many edges come before this one
                // and expecially the position where to put the
                // edge in the opposite direction.
                vec![
                    (0, (src_node_id, dst_node_id, edge_type, weight)),
                    (0, (dst_node_id, src_node_id, edge_type, weight)),
                ]
            })
            .collect::<Vec<_>>();
            if include_selfloops {
                result.push((0, (dst_node_id, dst_node_id, edge_type, weight)));
            }
            result
        })
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
    weight: WeightT,
) -> (
    EdgeT,
    impl ParallelIterator<Item = (usize, (NodeT, NodeT, Option<EdgeTypeT>, WeightT))>,
) {
    let total_nodes = maximum_node_id - minimum_node_id;
    let total_edges = if total_nodes == 0 {
        0
    } else {
        (total_nodes - 1) * 2
    } as EdgeT;
    (
        total_edges,
        get_random_connected_graph_edges_iterator(
            random_state,
            minimum_node_id,
            maximum_node_id,
            include_selfloops,
            1,
            1,
            edge_type,
            weight,
        ),
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
    ) -> Result<Graph> {
        let random_state = random_state.unwrap_or(42);
        let nodes_number = nodes_number.unwrap_or(10);
        let minimum_node_sampling = minimum_node_sampling.unwrap_or(1);
        let maximum_node_sampling = maximum_node_sampling.unwrap_or(5);
        let minimum_node_id = minimum_node_id.unwrap_or(0);
        let include_selfloops = include_selfloops.unwrap_or(false);
        let directed = directed.unwrap_or(false);
        let node_type = node_type.unwrap_or("connected");
        let node_types = NodeTypeVocabulary::from_structs(
            vec![Some(vec![0]); nodes_number as usize],
            Vocabulary::from_reverse_map(vec![node_type.to_owned()])?,
        );

        let edge_type = edge_type.unwrap_or("connected");
        let edge_types_vocabulary: Vocabulary<EdgeTypeT> =
            Vocabulary::from_reverse_map(vec![edge_type.to_owned()])?;
        let nodes = Vocabulary::from_range(minimum_node_id..(minimum_node_id + nodes_number));
        let name = name.unwrap_or("Connected");
        let has_edge_weights = weight.is_some();

        // Get the generator the chain in the middle of the two cliques
        let edges_iterator = unsafe {
            get_random_connected_graph_edges_iterator(
                random_state,
                0,
                nodes_number,
                include_selfloops,
                minimum_node_sampling,
                maximum_node_sampling,
                Some(0),
                weight.unwrap_or(WeightT::NAN),
            )
        };

        build_graph_from_integers(
            Some(edges_iterator),
            nodes,
            Some(node_types),
            Some(edge_types_vocabulary),
            has_edge_weights,
            directed,
            Some(true),
            Some(false),
            Some(false),
            None,
            false,
            false,
            name.to_string(),
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
    ) -> Result<Graph> {
        let random_state = random_state.unwrap_or(42);
        let nodes_number = nodes_number.unwrap_or(10);
        let minimum_node_id = minimum_node_id.unwrap_or(0);
        let include_selfloops = include_selfloops.unwrap_or(false);
        let directed = directed.unwrap_or(false);
        let node_type = node_type.unwrap_or("connected");
        let node_types = NodeTypeVocabulary::from_structs(
            vec![Some(vec![0]); nodes_number as usize],
            Vocabulary::from_reverse_map(vec![node_type.to_owned()])?,
        );

        let edge_type = edge_type.unwrap_or("connected");
        let edge_types_vocabulary: Vocabulary<EdgeTypeT> =
            Vocabulary::from_reverse_map(vec![edge_type.to_owned()])?;
        let nodes = Vocabulary::from_range(minimum_node_id..(minimum_node_id + nodes_number));
        let name = name.unwrap_or("Connected");
        let has_edge_weights = weight.is_some();

        // Get the generator the chain in the middle of the two cliques
        let (edges_number, edges_iterator) = unsafe {
            get_random_spanning_tree_edges_iterator(
                random_state,
                0,
                nodes_number,
                include_selfloops,
                Some(0),
                weight.unwrap_or(WeightT::NAN),
            )
        };

        build_graph_from_integers(
            Some(edges_iterator),
            nodes,
            Some(node_types),
            Some(edge_types_vocabulary),
            has_edge_weights,
            directed,
            Some(true),
            Some(false),
            Some(false),
            Some(edges_number),
            false,
            false,
            name.to_string(),
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
    ) -> Result<Graph> {
        let nodes_number = nodes_number.unwrap_or(10);
        let minimum_node_id = minimum_node_id.unwrap_or(0);
        let include_selfloops = include_selfloops.unwrap_or(false);
        let directed = directed.unwrap_or(false);
        let node_type = node_type.unwrap_or("circle");
        let node_types = NodeTypeVocabulary::from_structs(
            vec![Some(vec![0]); nodes_number as usize],
            Vocabulary::from_reverse_map(vec![node_type.to_owned()])?,
        );

        let edge_type = edge_type.unwrap_or("circle");
        let edge_types_vocabulary: Vocabulary<EdgeTypeT> =
            Vocabulary::from_reverse_map(vec![edge_type.to_owned()])?;
        let nodes = Vocabulary::from_range(minimum_node_id..(minimum_node_id + nodes_number));
        let name = name.unwrap_or("Circle");
        let has_edge_weights = weight.is_some();

        // Get the generator the circle in the middle of the two cliques
        let (edges_number, edges_iterator) = unsafe {
            get_circle_edges_iterator(
                0,
                nodes_number,
                include_selfloops,
                Some(0),
                weight.unwrap_or(WeightT::NAN),
                0,
            )
        };

        build_graph_from_integers(
            Some(edges_iterator),
            nodes,
            Some(node_types),
            Some(edge_types_vocabulary),
            has_edge_weights,
            directed,
            Some(true),
            Some(false),
            Some(true),
            Some(edges_number),
            false,
            false,
            name.to_string(),
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
    ) -> Result<Graph> {
        let nodes_number = nodes_number.unwrap_or(10);
        let minimum_node_id = minimum_node_id.unwrap_or(0);
        let include_selfloops = include_selfloops.unwrap_or(false);
        let directed = directed.unwrap_or(false);
        let node_type = node_type.unwrap_or("chain");
        let node_types = NodeTypeVocabulary::from_structs(
            vec![Some(vec![0]); nodes_number as usize],
            Vocabulary::from_reverse_map(vec![node_type.to_owned()])?,
        );

        let edge_type = edge_type.unwrap_or("chain");
        let edge_types_vocabulary: Vocabulary<EdgeTypeT> =
            Vocabulary::from_reverse_map(vec![edge_type.to_owned()])?;
        let nodes = Vocabulary::from_range(minimum_node_id..(minimum_node_id + nodes_number));
        let name = name.unwrap_or("Chain");
        let has_edge_weights = weight.is_some();

        // Get the generator the chain in the middle of the two cliques
        let (edges_number, edges_iterator) = unsafe {
            get_chain_edges_iterator_unchecked(
                0,
                nodes_number,
                include_selfloops,
                Some(0),
                weight.unwrap_or(WeightT::NAN),
                0,
            )
        };

        build_graph_from_integers(
            Some(edges_iterator),
            nodes,
            Some(node_types),
            Some(edge_types_vocabulary),
            has_edge_weights,
            directed,
            Some(true),
            Some(false),
            Some(true),
            Some(edges_number),
            false,
            false,
            name.to_string(),
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
    ) -> Result<Graph> {
        let nodes_number = nodes_number.unwrap_or(10);
        let minimum_node_id = minimum_node_id.unwrap_or(0);
        let include_selfloops = include_selfloops.unwrap_or(false);
        let directed = directed.unwrap_or(false);
        let node_type = node_type.unwrap_or("complete");
        let node_types = NodeTypeVocabulary::from_structs(
            vec![Some(vec![0]); nodes_number as usize],
            Vocabulary::from_reverse_map(vec![node_type.to_owned()])?,
        );

        let edge_type = edge_type.unwrap_or("complete");
        let edge_types_vocabulary: Vocabulary<EdgeTypeT> =
            Vocabulary::from_reverse_map(vec![edge_type.to_owned()])?;
        let nodes = Vocabulary::from_range(minimum_node_id..(minimum_node_id + nodes_number));
        let name = name.unwrap_or("Complete");
        let has_edge_weights = weight.is_some();

        // Get the generator the chain in the middle of the two cliques
        let (edges_number, edges_iterator) = unsafe {
            get_clique_edges_iterator_unchecked(
                0,
                nodes_number,
                include_selfloops,
                Some(0),
                weight.unwrap_or(WeightT::NAN),
                0,
            )
        };

        build_graph_from_integers(
            Some(edges_iterator),
            nodes,
            Some(node_types),
            Some(edge_types_vocabulary),
            has_edge_weights,
            directed,
            Some(true),
            Some(false),
            Some(true),
            Some(edges_number),
            false,
            false,
            name.to_string(),
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
    ) -> Result<Graph> {
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
        let node_types_vocabulary: Vocabulary<NodeTypeT> = Vocabulary::from_reverse_map(vec![
            left_clique_node_type.to_owned(),
            right_clique_node_type.to_owned(),
            chain_node_type.to_owned(),
        ])?;
        let mut node_type_ids: Vec<Option<Vec<NodeTypeT>>> = [
            left_clique_nodes_number,
            chain_nodes_number,
            right_clique_nodes_number,
        ]
        .iter()
        .enumerate()
        .flat_map(|(i, &subgraph_nodes_number)| {
            vec![Some(vec![i as NodeTypeT]); subgraph_nodes_number as usize]
        })
        .collect();
        node_type_ids[left_clique_nodes_number.saturating_sub(1) as usize]
            .as_mut()
            .map(|node_type_ids| node_type_ids.push(1));
        node_type_ids[(left_clique_nodes_number + chain_nodes_number).saturating_sub(1) as usize]
            .as_mut()
            .map(|node_type_ids| node_type_ids.push(2));

        let node_types = NodeTypeVocabulary::from_structs(node_type_ids, node_types_vocabulary);

        let left_clique_edge_type = left_clique_edge_type.unwrap_or("left_clique");
        let right_clique_edge_type = right_clique_edge_type.unwrap_or("right_clique");
        let chain_edge_type = chain_edge_type.unwrap_or("chain");
        let edge_types_vocabulary: Vocabulary<EdgeTypeT> = Vocabulary::from_reverse_map(vec![
            left_clique_edge_type.to_owned(),
            right_clique_edge_type.to_owned(),
            chain_edge_type.to_owned(),
        ])?;
        let nodes = Vocabulary::from_range(minimum_node_id..(minimum_node_id + nodes_number));
        let name = name.unwrap_or("Barbell");
        let has_edge_weights = left_clique_weight.is_some();

        // Get the generator for the left clique
        let (left_edges_number, left_clique_edges_iterator) = unsafe {
            get_clique_edges_iterator_unchecked(
                0,
                left_clique_nodes_number,
                include_selfloops,
                Some(0),
                left_clique_weight.unwrap_or(WeightT::NAN),
                0,
            )
        };
        // Get the generator the chain in the middle of the two cliques
        let (chain_edges_number, chain_edges_iterator) = unsafe {
            get_chain_edges_iterator_unchecked(
                left_clique_nodes_number.saturating_sub(1),
                left_clique_nodes_number + chain_nodes_number + 1,
                include_selfloops,
                Some(1),
                chain_weight.unwrap_or(WeightT::NAN),
                left_edges_number as usize,
            )
        };
        // Get the generator for the right clique
        let (right_edges_number, right_clique_edges_iterator) = unsafe {
            get_clique_edges_iterator_unchecked(
                left_clique_nodes_number + chain_nodes_number,
                left_clique_nodes_number + chain_nodes_number + right_clique_nodes_number,
                include_selfloops,
                Some(2),
                right_clique_weight.unwrap_or(WeightT::NAN),
                (left_edges_number + chain_edges_number) as usize,
            )
        };

        let edges_number = left_edges_number + chain_edges_number + right_edges_number;

        let edges_iterator = left_clique_edges_iterator
            .chain(chain_edges_iterator)
            .chain(right_clique_edges_iterator);

        build_graph_from_integers(
            Some(edges_iterator),
            nodes,
            Some(node_types),
            Some(edge_types_vocabulary),
            has_edge_weights,
            directed,
            Some(true),
            Some(false),
            Some(true),
            Some(edges_number),
            false,
            false,
            name.to_string(),
        )
    }
}
