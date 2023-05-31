use num_traits::Zero;
use rand::prelude::SliceRandom;
use rand::prelude::SmallRng;
use rand::SeedableRng;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
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

/// Return numeric representation of the requested node ID from coordinates.
///
/// # Arguments
/// `x`: NodeT - The first coordinate of the given Node ID.
/// `y`: NodeT - The second coordinate of the given Node ID.
/// `maximal_dimension_size`: NodeT - The maximal dimension size of the second coordinate of the node.
///
fn compose_node_id(x: NodeT, y: NodeT, maximal_dimension_size: NodeT) -> NodeT {
    x * maximal_dimension_size + y
}

/// Return coordinates of the provided node ID in 2D space.
///
/// # Arguments
/// `node_id`: NodeT - The node ID to decode.
/// `maximal_dimension_size`: NodeT - The maximal dimension size of the second coordinate of the node.
///
fn decompose_node_id(node_id: NodeT, maximal_dimension_size: NodeT) -> (NodeT, NodeT) {
    (
        node_id / maximal_dimension_size,
        node_id % maximal_dimension_size,
    )
}

/// Returns node ID from the provided multi-dimensional coordinate.
///
/// # Arguments
/// `coordinates`: &[NodeT] - The coordinates of the node to decode.
/// `maximal_dimension_sizes`: &[NodeT] - The maximal coordinate sizes.
///
fn multidimensional_compose_node_id(
    coordinates: &[NodeT],
    maximal_dimension_sizes: &[NodeT],
) -> NodeT {
    coordinates
        .iter()
        .cloned()
        .zip(maximal_dimension_sizes.iter().cloned())
        .rev()
        .fold(0, |node_id, (coordinate, maximal_dimension_size)| {
            compose_node_id(node_id, coordinate, maximal_dimension_size)
        })
}

/// Returns coordinates in nD space for the provided node IDs.
///
/// # Arguments
/// `node_id`: NodeT - The node ID to decode.
/// `maximal_dimension_sizes`: &[NodeT] - The maximal coordinate sizes.
fn multidimensional_decompose_node_id(
    node_id: NodeT,
    maximal_dimension_sizes: &[NodeT],
) -> Vec<NodeT> {
    maximal_dimension_sizes
        .iter()
        .cloned()
        .scan(node_id, |node_id, maximal_dimension_size| {
            let (this_node_id, coordinate) = decompose_node_id(*node_id, maximal_dimension_size);
            *node_id = this_node_id;
            Some(coordinate)
        })
        .collect()
}

/// Return number of edges and iterator over edge list of an hyper-dimensional lattice with square cell.
///
/// # Implementative details
/// Please do note that the edge IDs are NOT produced in correct order so the graph will be sorted.
///
/// # Arguments
/// * `sides`: &'a [NodeT] - Sides of the hyper-dimensional lattice with square cell.
/// * `minimum_node_id`: NodeT - The minimum node ID for the range of the wheel.
/// * `weight`: Option<WeightT> - Edge weights for the edges in the wheel.
unsafe fn get_squared_lattice_edges_iterator<'a>(
    sides: &'a [NodeT],
    minimum_node_id: NodeT,
    weight: WeightT,
) -> (
    NodeT,
    EdgeT,
    impl ParallelIterator<Item = (usize, (NodeT, NodeT, Option<EdgeTypeT>, WeightT))> + 'a,
) {
    let dimensions = sides.len();
    let number_of_nodes = sides.iter().cloned().reduce(|a, b| a * b).unwrap();
    let number_of_edges = sides
        .iter()
        .cloned()
        .enumerate()
        .map(|(dimension_number, maximal_dimension_size)| {
            (maximal_dimension_size as EdgeT + 1)
                * sides
                    .iter()
                    .cloned()
                    .enumerate()
                    .filter_map(|(inner_dimension_number, maximal_dimension_size)| {
                        if inner_dimension_number == dimension_number {
                            None
                        } else {
                            Some(maximal_dimension_size as EdgeT)
                        }
                    })
                    .reduce(|a, b| a * b)
                    .unwrap()
        })
        .sum::<EdgeT>();
    (
        number_of_nodes,
        number_of_edges,
        (minimum_node_id..(minimum_node_id + number_of_nodes))
            .into_par_iter()
            .flat_map(move |src_node_id| {
                let mut coordinates = multidimensional_decompose_node_id(src_node_id, sides);
                let destinations_and_edge_types: Vec<(NodeT, EdgeTypeT)> = (0..dimensions)
                    .zip(sides.iter().cloned())
                    .flat_map(|(i, maximal_dimension_size)| {
                        let mut neighbours = Vec::new();
                        let coordinate = coordinates[i];
                        if coordinate > 0 {
                            coordinates[i] -= 1;
                            neighbours.push((
                                multidimensional_compose_node_id(&coordinates, sides),
                                i as EdgeTypeT,
                            ));
                            coordinates[i] += 1;
                        }
                        if coordinate != maximal_dimension_size - 1 {
                            coordinates[i] += 1;
                            neighbours.push((
                                multidimensional_compose_node_id(&coordinates, sides),
                                i as EdgeTypeT,
                            ));
                            coordinates[i] -= 1;
                        }
                        neighbours.into_iter()
                    })
                    .collect::<Vec<(NodeT, EdgeTypeT)>>();
                destinations_and_edge_types
                    .into_par_iter()
                    .map(move |(dst_node_id, edge_type)| {
                        (0, (src_node_id, dst_node_id, Some(edge_type), weight))
                    })
            }),
    )
}

/// Return number of edges and iterator over edge list of a wheel.
///
/// # Implementative details
/// Please do note that the edge IDs are produced in correct order.
///
/// # Arguments
/// * `minimum_node_id`: NodeT - The minimum node ID for the range of the wheel.
/// * `maximum_node_id`: NodeT - The maximum node ID for the range of the wheel.
/// * `include_selfloops`: bool - Whether to include selfloops in the wheel.
/// * `edge_type`: Option<EdgeTypeT> - Edge type for the edges in the wheel.
/// * `weight`: Option<WeightT> - Edge weights for the edges in the wheel.
/// * `edge_id_offset`: usize - How many edges come before the first of this list.
///
/// # Safety
/// If the minimum node ID is higher than the maximum node ID the method will cause a panic
/// caused by subtraction.
unsafe fn get_wheel_edges_iterator(
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
    let first_node_of_circle = minimum_node_id + 1;
    let last_node_of_circle = maximum_node_id - 1;
    let total_edges = if total_nodes == 0 {
        0
    } else {
        (total_nodes - 1) * 4 + if include_selfloops { total_nodes } else { 0 }
    } as EdgeT;
    (
        total_edges,
        (minimum_node_id..maximum_node_id)
            .into_par_iter()
            .enumerate()
            .flat_map_iter(move |(i, src_node_id)| {
                let result: Box<
                    dyn Iterator<Item = (usize, (NodeT, NodeT, Option<EdgeTypeT>, WeightT))>,
                > = if i == 0 {
                    Box::new(
                        (minimum_node_id..maximum_node_id)
                            .filter(move |&dst_node_id| {
                                dst_node_id != src_node_id || include_selfloops
                            })
                            .enumerate()
                            .map(move |(j, dst_node_id)| {
                                (
                                    edge_id_offset + j,
                                    (src_node_id, dst_node_id, edge_type, weight),
                                )
                            }),
                    )
                } else {
                    Box::new({
                        let needs_previous_edge = src_node_id != first_node_of_circle;
                        let needs_following_edge = src_node_id != last_node_of_circle;
                        let needs_previous_closing_edge =
                            !needs_previous_edge && needs_following_edge;
                        let needs_following_closing_edge =
                            !needs_following_edge && needs_previous_edge;
                        let mut offsets = if include_selfloops {
                            total_nodes as usize + (i - 1) * 4
                        } else {
                            total_nodes as usize - 1 + (i - 1) * 3
                        };
                        // We initialize the edges vector with the edge to the center of the wheel
                        let mut edges = vec![(
                            edge_id_offset + offsets,
                            (src_node_id, minimum_node_id, edge_type, weight),
                        )];
                        offsets += 1;
                        // If this is the last node and we need to add the edge to the first node
                        // in order to close the circle part of the wheel
                        if needs_following_closing_edge {
                            edges.push((
                                edge_id_offset + offsets,
                                (src_node_id, first_node_of_circle, edge_type, weight),
                            ));
                            offsets += 1;
                        }
                        // Then we add, if necessary, the edge to the previous node.
                        if needs_previous_edge {
                            edges.push((
                                edge_id_offset + offsets,
                                (src_node_id, src_node_id - 1, edge_type, weight),
                            ));
                            offsets += 1;
                        }
                        // If self-loops are necessary, we add the edge to the node itself.
                        if include_selfloops {
                            edges.push((
                                edge_id_offset + offsets,
                                (src_node_id, src_node_id, edge_type, weight),
                            ));
                            offsets += 1;
                        }
                        // Then we add, if necessary, the edge to the following node.
                        if needs_following_edge {
                            edges.push((
                                edge_id_offset + offsets,
                                (src_node_id, src_node_id + 1, edge_type, weight),
                            ));
                            offsets += 1;
                        }
                        // If this is the first node and we need to add the edge to the last node
                        // in order to close the circle part of the wheel
                        if needs_previous_closing_edge {
                            edges.push((
                                edge_id_offset + offsets,
                                (src_node_id, last_node_of_circle, edge_type, weight),
                            ));
                        }
                        edges.into_iter()
                    })
                };
                result
            }),
    )
}

/// Return number of edges and iterator over edge list of a star.
///
/// # Implementative details
/// Please do note that the edge IDs are produced in correct order.
///
/// # Arguments
/// * `minimum_node_id`: NodeT - The minimum node ID for the range of the star.
/// * `maximum_node_id`: NodeT - The maximum node ID for the range of the star.
/// * `include_selfloops`: bool - Whether to include selfloops in the star.
/// * `edge_type`: Option<EdgeTypeT> - Edge type for the edges in the star.
/// * `weight`: Option<WeightT> - Edge weights for the edges in the star.
/// * `edge_id_offset`: usize - How many edges come before the first of this list.
///
/// # Safety
/// If the minimum node ID is higher than the maximum node ID the method will cause a panic
/// caused by subtraction.
unsafe fn get_star_edges_iterator(
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
        (total_nodes - 1) * 2 + if include_selfloops { total_nodes } else { 0 }
    } as EdgeT;
    (
        total_edges,
        (minimum_node_id..maximum_node_id)
            .into_par_iter()
            .enumerate()
            .flat_map_iter(move |(i, src_node_id)| {
                let result: Box<
                    dyn Iterator<Item = (usize, (NodeT, NodeT, Option<EdgeTypeT>, WeightT))>,
                > = if i == 0 {
                    Box::new(
                        (minimum_node_id..maximum_node_id)
                            .filter(move |&dst_node_id| {
                                dst_node_id != src_node_id || include_selfloops
                            })
                            .enumerate()
                            .map(move |(j, dst_node_id)| {
                                (
                                    edge_id_offset + j,
                                    (src_node_id, dst_node_id, edge_type, weight),
                                )
                            }),
                    )
                } else {
                    Box::new(
                        if include_selfloops {
                            vec![
                                (
                                    edge_id_offset + total_nodes as usize + (i - 1) * 2,
                                    (src_node_id, minimum_node_id, edge_type, weight),
                                ),
                                (
                                    edge_id_offset + total_nodes as usize + (i - 1) * 2 + 1,
                                    (src_node_id, src_node_id, edge_type, weight),
                                ),
                            ]
                        } else {
                            vec![(
                                edge_id_offset + total_nodes as usize + i - 2,
                                (src_node_id, minimum_node_id, edge_type, weight),
                            )]
                        }
                        .into_iter(),
                    )
                };
                result
            }),
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
    /// * `number_of_nodes`: Option<NodeT> - Number of nodes in the chain. By default 10.
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
        number_of_nodes: Option<NodeT>,
        include_selfloops: Option<bool>,
        node_type: Option<&str>,
        edge_type: Option<&str>,
        weight: Option<WeightT>,
        directed: Option<bool>,
        name: Option<&str>,
    ) -> Result<Graph> {
        let random_state = random_state.unwrap_or(42);
        let number_of_nodes = number_of_nodes.unwrap_or(10);
        let minimum_node_sampling = minimum_node_sampling.unwrap_or(1);
        let maximum_node_sampling = maximum_node_sampling.unwrap_or(5);
        let minimum_node_id = minimum_node_id.unwrap_or(0);
        let include_selfloops = include_selfloops.unwrap_or(false);
        let directed = directed.unwrap_or(false);
        let node_type = node_type.unwrap_or("connected");
        let node_types = NodeTypeVocabulary::from_structs(
            vec![Some(vec![0]); number_of_nodes as usize],
            Vocabulary::from_reverse_map(vec![node_type.to_owned()])?,
        );

        let edge_type = edge_type.unwrap_or("connected");
        let edge_types_vocabulary: Vocabulary<EdgeTypeT> =
            Vocabulary::from_reverse_map(vec![edge_type.to_owned()])?;
        let nodes = Vocabulary::from_range(minimum_node_id..(minimum_node_id + number_of_nodes));
        let name = name.unwrap_or("Connected");
        let has_edge_weights = weight.is_some();

        // Get the generator the chain in the middle of the two cliques
        let edges_iterator = unsafe {
            get_random_connected_graph_edges_iterator(
                random_state,
                0,
                number_of_nodes,
                include_selfloops,
                minimum_node_sampling,
                maximum_node_sampling,
                Some(0),
                weight.unwrap_or(WeightT::NAN),
            )
        };

        build_graph_from_integers(
            Some(edges_iterator),
            Arc::new(nodes),
            Arc::new(Some(node_types)),
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
    /// * `number_of_nodes`: Option<NodeT> - Number of nodes in the chain. By default 10.
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
        number_of_nodes: Option<NodeT>,
        include_selfloops: Option<bool>,
        node_type: Option<&str>,
        edge_type: Option<&str>,
        weight: Option<WeightT>,
        directed: Option<bool>,
        name: Option<&str>,
    ) -> Result<Graph> {
        let random_state = random_state.unwrap_or(42);
        let number_of_nodes = number_of_nodes.unwrap_or(10);
        let minimum_node_id = minimum_node_id.unwrap_or(0);
        let include_selfloops = include_selfloops.unwrap_or(false);
        let directed = directed.unwrap_or(false);
        let node_type = node_type.unwrap_or("tree");
        let node_types = NodeTypeVocabulary::from_structs(
            vec![Some(vec![0]); number_of_nodes as usize],
            Vocabulary::from_reverse_map(vec![node_type.to_owned()])?,
        );

        let edge_type = edge_type.unwrap_or("tree");
        let edge_types_vocabulary: Vocabulary<EdgeTypeT> =
            Vocabulary::from_reverse_map(vec![edge_type.to_owned()])?;
        let nodes = Vocabulary::from_range(minimum_node_id..(minimum_node_id + number_of_nodes));
        let name = name.unwrap_or("Tree");
        let has_edge_weights = weight.is_some();

        // Get the generator the chain in the middle of the two cliques
        let (number_of_edges, edges_iterator) = unsafe {
            get_random_spanning_tree_edges_iterator(
                random_state,
                0,
                number_of_nodes,
                include_selfloops,
                Some(0),
                weight.unwrap_or(WeightT::NAN),
            )
        };

        build_graph_from_integers(
            Some(edges_iterator),
            Arc::new(nodes),
            Arc::new(Some(node_types)),
            Some(edge_types_vocabulary),
            has_edge_weights,
            directed,
            Some(true),
            Some(false),
            Some(false),
            Some(number_of_edges),
            false,
            false,
            name.to_string(),
        )
    }

    /// Creates new star graph with given sizes and types.
    ///
    /// # Arguments
    /// * `minimum_node_id`: Option<NodeT> - Minimum node ID to start with. May be needed when circleing graphs. By default 0.
    /// * `number_of_nodes`: Option<NodeT> - Number of nodes in the star. By default 10.
    /// * `include_selfloops`: Option<bool> - Whether to include selfloops.
    /// * `node_type`: Option<&str> - The node type to use for the star. By default 'star'.
    /// * `edge_type`: Option<&str> - The node type to use for the star. By default 'star'.
    /// * `weight`: Option<WeightT> - The weight to use for the edges in the star. By default None.
    /// * `directed`: Option<bool> - Whether the graph is to built as directed. By default false.
    /// * `name`: Option<&str> - Name of the graph. By default 'Star'.
    ///
    pub fn generate_star_graph(
        minimum_node_id: Option<NodeT>,
        number_of_nodes: Option<NodeT>,
        include_selfloops: Option<bool>,
        node_type: Option<&str>,
        edge_type: Option<&str>,
        weight: Option<WeightT>,
        directed: Option<bool>,
        name: Option<&str>,
    ) -> Result<Graph> {
        let number_of_nodes = number_of_nodes.unwrap_or(10);
        let minimum_node_id = minimum_node_id.unwrap_or(0);
        let include_selfloops = include_selfloops.unwrap_or(false);
        let directed = directed.unwrap_or(false);
        let node_type = node_type.unwrap_or("star");
        let node_types = NodeTypeVocabulary::from_structs(
            vec![Some(vec![0]); number_of_nodes as usize],
            Vocabulary::from_reverse_map(vec![node_type.to_owned()])?,
        );

        let edge_type = edge_type.unwrap_or("star");
        let edge_types_vocabulary: Vocabulary<EdgeTypeT> =
            Vocabulary::from_reverse_map(vec![edge_type.to_owned()])?;
        let nodes = Vocabulary::from_range(minimum_node_id..(minimum_node_id + number_of_nodes));
        let name = name.unwrap_or("Star");
        let has_edge_weights = weight.is_some();

        // Get the generator the star in the middle of the two cliques
        let (number_of_edges, edges_iterator) = unsafe {
            get_star_edges_iterator(
                0,
                number_of_nodes,
                include_selfloops,
                Some(0),
                weight.unwrap_or(WeightT::NAN),
                0,
            )
        };

        build_graph_from_integers(
            Some(edges_iterator),
            Arc::new(nodes),
            Arc::new(Some(node_types)),
            Some(edge_types_vocabulary),
            has_edge_weights,
            directed,
            Some(true),
            Some(false),
            Some(true),
            Some(number_of_edges),
            false,
            false,
            name.to_string(),
        )
    }

    /// Creates new wheel graph with given sizes and types.
    ///
    /// # Arguments
    /// * `minimum_node_id`: Option<NodeT> - Minimum node ID to start with. May be needed when circleing graphs. By default 0.
    /// * `number_of_nodes`: Option<NodeT> - Number of nodes in the wheel. By default 10.
    /// * `include_selfloops`: Option<bool> - Whether to include selfloops.
    /// * `node_type`: Option<&str> - The node type to use for the wheel. By default 'wheel'.
    /// * `edge_type`: Option<&str> - The node type to use for the wheel. By default 'wheel'.
    /// * `weight`: Option<WeightT> - The weight to use for the edges in the wheel. By default None.
    /// * `directed`: Option<bool> - Whether the graph is to built as directed. By default false.
    /// * `name`: Option<&str> - Name of the graph. By default 'Wheel'.
    ///
    pub fn generate_wheel_graph(
        minimum_node_id: Option<NodeT>,
        number_of_nodes: Option<NodeT>,
        include_selfloops: Option<bool>,
        node_type: Option<&str>,
        edge_type: Option<&str>,
        weight: Option<WeightT>,
        directed: Option<bool>,
        name: Option<&str>,
    ) -> Result<Graph> {
        let number_of_nodes = number_of_nodes.unwrap_or(10);
        let minimum_node_id = minimum_node_id.unwrap_or(0);
        let include_selfloops = include_selfloops.unwrap_or(false);
        let directed = directed.unwrap_or(false);
        let node_type = node_type.unwrap_or("wheel");
        let node_types = NodeTypeVocabulary::from_structs(
            vec![Some(vec![0]); number_of_nodes as usize],
            Vocabulary::from_reverse_map(vec![node_type.to_owned()])?,
        );

        let edge_type = edge_type.unwrap_or("wheel");
        let edge_types_vocabulary: Vocabulary<EdgeTypeT> =
            Vocabulary::from_reverse_map(vec![edge_type.to_owned()])?;
        let nodes = Vocabulary::from_range(minimum_node_id..(minimum_node_id + number_of_nodes));
        let name = name.unwrap_or("Wheel");
        let has_edge_weights = weight.is_some();

        // Get the generator the wheel in the middle of the two cliques
        let (number_of_edges, edges_iterator) = unsafe {
            get_wheel_edges_iterator(
                0,
                number_of_nodes,
                include_selfloops,
                Some(0),
                weight.unwrap_or(WeightT::NAN),
                0,
            )
        };

        build_graph_from_integers(
            Some(edges_iterator),
            Arc::new(nodes),
            Arc::new(Some(node_types)),
            Some(edge_types_vocabulary),
            has_edge_weights,
            directed,
            Some(true),
            Some(false),
            Some(true),
            Some(number_of_edges),
            false,
            false,
            name.to_string(),
        )
    }

    /// Creates new circle graph with given sizes and types.
    ///
    /// # Arguments
    /// * `minimum_node_id`: Option<NodeT> - Minimum node ID to start with. May be needed when circleing graphs. By default 0.
    /// * `number_of_nodes`: Option<NodeT> - Number of nodes in the circle. By default 10.
    /// * `include_selfloops`: Option<bool> - Whether to include selfloops.
    /// * `node_type`: Option<&str> - The node type to use for the circle. By default 'circle'.
    /// * `edge_type`: Option<&str> - The node type to use for the circle. By default 'circle'.
    /// * `weight`: Option<WeightT> - The weight to use for the edges in the circle. By default None.
    /// * `directed`: Option<bool> - Whether the graph is to built as directed. By default false.
    /// * `name`: Option<&str> - Name of the graph. By default 'Circle'.
    ///
    pub fn generate_circle_graph(
        minimum_node_id: Option<NodeT>,
        number_of_nodes: Option<NodeT>,
        include_selfloops: Option<bool>,
        node_type: Option<&str>,
        edge_type: Option<&str>,
        weight: Option<WeightT>,
        directed: Option<bool>,
        name: Option<&str>,
    ) -> Result<Graph> {
        let number_of_nodes = number_of_nodes.unwrap_or(10);
        let minimum_node_id = minimum_node_id.unwrap_or(0);
        let include_selfloops = include_selfloops.unwrap_or(false);
        let directed = directed.unwrap_or(false);
        let node_type = node_type.unwrap_or("circle");
        let node_types = NodeTypeVocabulary::from_structs(
            vec![Some(vec![0]); number_of_nodes as usize],
            Vocabulary::from_reverse_map(vec![node_type.to_owned()])?,
        );

        let edge_type = edge_type.unwrap_or("circle");
        let edge_types_vocabulary: Vocabulary<EdgeTypeT> =
            Vocabulary::from_reverse_map(vec![edge_type.to_owned()])?;
        let nodes = Vocabulary::from_range(minimum_node_id..(minimum_node_id + number_of_nodes));
        let name = name.unwrap_or("Circle");
        let has_edge_weights = weight.is_some();

        // Get the generator the circle in the middle of the two cliques
        let (number_of_edges, edges_iterator) = unsafe {
            get_circle_edges_iterator(
                0,
                number_of_nodes,
                include_selfloops,
                Some(0),
                weight.unwrap_or(WeightT::NAN),
                0,
            )
        };

        build_graph_from_integers(
            Some(edges_iterator),
            Arc::new(nodes),
            Arc::new(Some(node_types)),
            Some(edge_types_vocabulary),
            has_edge_weights,
            directed,
            Some(true),
            Some(false),
            Some(true),
            Some(number_of_edges),
            false,
            false,
            name.to_string(),
        )
    }

    /// Creates new chain graph with given sizes and types.
    ///
    /// # Arguments
    /// * `minimum_node_id`: Option<NodeT> - Minimum node ID to start with. May be needed when chaining graphs. By default 0.
    /// * `number_of_nodes`: Option<NodeT> - Number of nodes in the chain. By default 10.
    /// * `include_selfloops`: Option<bool> - Whether to include selfloops.
    /// * `node_type`: Option<&str> - The node type to use for the chain. By default 'chain'.
    /// * `edge_type`: Option<&str> - The node type to use for the chain. By default 'chain'.
    /// * `weight`: Option<WeightT> - The weight to use for the edges in the chain. By default None.
    /// * `directed`: Option<bool> - Whether the graph is to built as directed. By default false.
    /// * `name`: Option<&str> - Name of the graph. By default 'Chain'.
    ///
    pub fn generate_chain_graph(
        minimum_node_id: Option<NodeT>,
        number_of_nodes: Option<NodeT>,
        include_selfloops: Option<bool>,
        node_type: Option<&str>,
        edge_type: Option<&str>,
        weight: Option<WeightT>,
        directed: Option<bool>,
        name: Option<&str>,
    ) -> Result<Graph> {
        let number_of_nodes = number_of_nodes.unwrap_or(10);
        let minimum_node_id = minimum_node_id.unwrap_or(0);
        let include_selfloops = include_selfloops.unwrap_or(false);
        let directed = directed.unwrap_or(false);
        let node_type = node_type.unwrap_or("chain");
        let node_types = NodeTypeVocabulary::from_structs(
            vec![Some(vec![0]); number_of_nodes as usize],
            Vocabulary::from_reverse_map(vec![node_type.to_owned()])?,
        );

        let edge_type = edge_type.unwrap_or("chain");
        let edge_types_vocabulary: Vocabulary<EdgeTypeT> =
            Vocabulary::from_reverse_map(vec![edge_type.to_owned()])?;
        let nodes = Vocabulary::from_range(minimum_node_id..(minimum_node_id + number_of_nodes));
        let name = name.unwrap_or("Chain");
        let has_edge_weights = weight.is_some();

        // Get the generator the chain in the middle of the two cliques
        let (number_of_edges, edges_iterator) = unsafe {
            get_chain_edges_iterator_unchecked(
                0,
                number_of_nodes,
                include_selfloops,
                Some(0),
                weight.unwrap_or(WeightT::NAN),
                0,
            )
        };

        build_graph_from_integers(
            Some(edges_iterator),
            Arc::new(nodes),
            Arc::new(Some(node_types)),
            Some(edge_types_vocabulary),
            has_edge_weights,
            directed,
            Some(true),
            Some(false),
            Some(true),
            Some(number_of_edges),
            false,
            false,
            name.to_string(),
        )
    }

    /// Creates new complete graph with given sizes and types.
    ///
    /// # Arguments
    /// * `minimum_node_id`: Option<NodeT> - Minimum node ID to start with. May be needed when combining graphs. By default 0.
    /// * `number_of_nodes`: Option<NodeT> - Number of nodes in the chain. By default 10.
    /// * `include_selfloops`: Option<bool> - Whether to include selfloops.
    /// * `node_type`: Option<&str> - The node type to use. By default 'complete'.
    /// * `edge_type`: Option<&str> - The node type to use. By default 'complete'.
    /// * `weight`: Option<WeightT> - The weight to use for the edges. By default None.
    /// * `directed`: Option<bool> - Whether the graph is to built as directed. By default false.
    /// * `name`: Option<&str> - Name of the graph. By default 'Complete'.
    ///
    pub fn generate_complete_graph(
        minimum_node_id: Option<NodeT>,
        number_of_nodes: Option<NodeT>,
        include_selfloops: Option<bool>,
        node_type: Option<&str>,
        edge_type: Option<&str>,
        weight: Option<WeightT>,
        directed: Option<bool>,
        name: Option<&str>,
    ) -> Result<Graph> {
        let number_of_nodes = number_of_nodes.unwrap_or(10);
        let minimum_node_id = minimum_node_id.unwrap_or(0);
        let include_selfloops = include_selfloops.unwrap_or(false);
        let directed = directed.unwrap_or(false);
        let node_type = node_type.unwrap_or("complete");
        let node_types = NodeTypeVocabulary::from_structs(
            vec![Some(vec![0]); number_of_nodes as usize],
            Vocabulary::from_reverse_map(vec![node_type.to_owned()])?,
        );

        let edge_type = edge_type.unwrap_or("complete");
        let edge_types_vocabulary: Vocabulary<EdgeTypeT> =
            Vocabulary::from_reverse_map(vec![edge_type.to_owned()])?;
        let nodes = Vocabulary::from_range(minimum_node_id..(minimum_node_id + number_of_nodes));
        let name = name.unwrap_or("Complete");
        let has_edge_weights = weight.is_some();

        // Get the generator the chain in the middle of the two cliques
        let (number_of_edges, edges_iterator) = unsafe {
            get_clique_edges_iterator_unchecked(
                0,
                number_of_nodes,
                include_selfloops,
                Some(0),
                weight.unwrap_or(WeightT::NAN),
                0,
            )
        };

        build_graph_from_integers(
            Some(edges_iterator),
            Arc::new(nodes),
            Arc::new(Some(node_types)),
            Some(edge_types_vocabulary),
            has_edge_weights,
            directed,
            Some(true),
            Some(false),
            Some(true),
            Some(number_of_edges),
            false,
            false,
            name.to_string(),
        )
    }

    /// Creates new barbell graph with given sizes and types.
    ///
    /// # Arguments
    /// * `minimum_node_id`: Option<NodeT> - Minimum node ID to start with. May be needed when chaining graphs. By default 0.
    /// * `left_clique_number_of_nodes`: Option<NodeT> - Number of nodes in the left clique. By default 10.
    /// * `right_clique_number_of_nodes`: Option<NodeT> -  Number of nodes in the right clique. By default equal to the left clique.
    /// * `chain_number_of_nodes`: Option<NodeT> - Number of nodes in the chain. By default 10.
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
        left_clique_number_of_nodes: Option<NodeT>,
        right_clique_number_of_nodes: Option<NodeT>,
        chain_number_of_nodes: Option<NodeT>,
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
        let left_clique_number_of_nodes = left_clique_number_of_nodes.unwrap_or(10);
        let chain_number_of_nodes = chain_number_of_nodes.unwrap_or(10);
        let include_selfloops = include_selfloops.unwrap_or(false);
        let directed = directed.unwrap_or(false);
        let right_clique_number_of_nodes =
            right_clique_number_of_nodes.unwrap_or(left_clique_number_of_nodes);
        let number_of_nodes =
            left_clique_number_of_nodes + chain_number_of_nodes + right_clique_number_of_nodes;
        let left_clique_node_type = left_clique_node_type.unwrap_or("left_clique");
        let right_clique_node_type = right_clique_node_type.unwrap_or("right_clique");
        let chain_node_type = chain_node_type.unwrap_or("chain");
        let node_types_vocabulary: Vocabulary<NodeTypeT> = Vocabulary::from_reverse_map(vec![
            left_clique_node_type.to_owned(),
            chain_node_type.to_owned(),
            right_clique_node_type.to_owned(),
        ])?;
        let mut node_type_ids: Vec<Option<Vec<NodeTypeT>>> = [
            left_clique_number_of_nodes,
            chain_number_of_nodes,
            right_clique_number_of_nodes,
        ]
        .iter()
        .enumerate()
        .flat_map(|(i, &subgraph_number_of_nodes)| {
            vec![Some(vec![i as NodeTypeT]); subgraph_number_of_nodes as usize]
        })
        .collect();
        node_type_ids[left_clique_number_of_nodes.saturating_sub(1) as usize]
            .as_mut()
            .map(|node_type_ids| node_type_ids.push(1));
        node_type_ids[(left_clique_number_of_nodes + chain_number_of_nodes).saturating_sub(1) as usize]
            .as_mut()
            .map(|node_type_ids| node_type_ids.push(2));

        let node_types = NodeTypeVocabulary::from_structs(node_type_ids, node_types_vocabulary);

        let left_clique_edge_type = left_clique_edge_type.unwrap_or("left_clique");
        let right_clique_edge_type = right_clique_edge_type.unwrap_or("right_clique");
        let chain_edge_type = chain_edge_type.unwrap_or("chain");
        let edge_types_vocabulary: Vocabulary<EdgeTypeT> = Vocabulary::from_reverse_map(vec![
            left_clique_edge_type.to_owned(),
            chain_edge_type.to_owned(),
            right_clique_edge_type.to_owned(),
        ])?;
        let nodes = Vocabulary::from_range(minimum_node_id..(minimum_node_id + number_of_nodes));
        let name = name.unwrap_or("Barbell");
        let has_edge_weights = left_clique_weight.is_some();

        // Get the generator for the left clique
        let (left_number_of_edges, left_clique_edges_iterator) = unsafe {
            get_clique_edges_iterator_unchecked(
                0,
                left_clique_number_of_nodes,
                include_selfloops,
                Some(0),
                left_clique_weight.unwrap_or(WeightT::NAN),
                0,
            )
        };
        // Get the generator the chain in the middle of the two cliques
        let (chain_number_of_edges, chain_edges_iterator) = unsafe {
            get_chain_edges_iterator_unchecked(
                left_clique_number_of_nodes.saturating_sub(1),
                left_clique_number_of_nodes
                    + chain_number_of_nodes
                    + if right_clique_number_of_nodes > 0 { 1 } else { 0 },
                include_selfloops,
                Some(1),
                chain_weight.unwrap_or(WeightT::NAN),
                left_number_of_edges as usize,
            )
        };
        // Get the generator for the right clique
        let (right_number_of_edges, right_clique_edges_iterator) = unsafe {
            get_clique_edges_iterator_unchecked(
                left_clique_number_of_nodes + chain_number_of_nodes,
                left_clique_number_of_nodes + chain_number_of_nodes + right_clique_number_of_nodes,
                include_selfloops,
                Some(2),
                right_clique_weight.unwrap_or(WeightT::NAN),
                (left_number_of_edges + chain_number_of_edges) as usize,
            )
        };

        let number_of_edges = left_number_of_edges + chain_number_of_edges + right_number_of_edges;

        let edges_iterator = left_clique_edges_iterator
            .chain(chain_edges_iterator)
            .chain(right_clique_edges_iterator);

        build_graph_from_integers(
            Some(edges_iterator),
            Arc::new(nodes),
            Arc::new(Some(node_types)),
            Some(edge_types_vocabulary),
            has_edge_weights,
            directed,
            Some(true),
            Some(false),
            Some(true),
            Some(number_of_edges),
            false,
            false,
            name.to_string(),
        )
    }

    /// Creates new lollipop graph with given sizes and types.
    ///
    /// # Arguments
    /// * `minimum_node_id`: Option<NodeT> - Minimum node ID to start with. May be needed when chaining graphs. By default 0.
    /// * `clique_number_of_nodes`: Option<NodeT> - Number of nodes in the left clique. By default 10.
    /// * `chain_number_of_nodes`: Option<NodeT> - Number of nodes in the chain. By default 10.
    /// * `include_selfloops`: Option<bool> - Whether to include selfloops.
    /// * `clique_node_type`: Option<&str> - The node type to use for the left clique. By default 'clique'.
    /// * `chain_node_type`: Option<&str> - The node type to use for the chain. By default 'chain'.
    /// * `clique_edge_type`: Option<&str> - The node type to use for the left clique. By default 'clique'.
    /// * `chain_edge_type`: Option<&str> - The node type to use for the chain. By default 'chain'.
    /// * `clique_weight`: Option<WeightT> - The weight to use for the edges in the left clique. By default None.
    /// * `chain_weight`: Option<WeightT> - The weight to use for the edges in the chain. By default None.
    /// * `directed`: Option<bool> - Whether the graph is to built as directed. By default false.
    /// * `name`: Option<&str> - Name of the graph. By default 'Lollipop'.
    ///
    /// # Raises
    /// * If the edge weights are provided only for a subset.
    pub fn generate_lollipop_graph(
        minimum_node_id: Option<NodeT>,
        clique_number_of_nodes: Option<NodeT>,
        chain_number_of_nodes: Option<NodeT>,
        include_selfloops: Option<bool>,
        clique_node_type: Option<&str>,
        chain_node_type: Option<&str>,
        clique_edge_type: Option<&str>,
        chain_edge_type: Option<&str>,
        clique_weight: Option<WeightT>,
        chain_weight: Option<WeightT>,
        directed: Option<bool>,
        name: Option<&str>,
    ) -> Result<Graph> {
        Graph::generate_barbell_graph(
            minimum_node_id,
            clique_number_of_nodes,
            Some(0),
            chain_number_of_nodes,
            include_selfloops,
            clique_node_type.or(Some("clique")),
            None,
            chain_node_type,
            clique_edge_type.or(Some("clique")),
            None,
            chain_edge_type,
            clique_weight,
            None,
            chain_weight,
            directed,
            name.or(Some("Lollipop")),
        )
    }

    /// Creates new squared lattice graph with given sizes and types.
    ///
    /// # Arguments
    /// * `sides`: &'a [NodeT] - Sides of the hyper-dimensional lattice with square cell.
    /// * `minimum_node_id`: Option<NodeT> - Minimum node ID to start with. May be needed when chaining graphs. By default 0.
    /// * `node_type`: Option<&str> - The node type to use for the squared lattice. By default 'squared_lattice'.
    /// * `weight`: Option<WeightT> - The weight to use for the edges in the left clique. By default None.
    /// * `directed`: Option<bool> - Whether the graph is to built as directed. By default false.
    /// * `name`: Option<&str> - Name of the graph. By default 'Lollipop'.
    ///
    /// # Raises
    /// * If the edge weights are provided only for a subset.
    pub fn generate_squared_lattice_graph(
        sides: &[NodeT],
        minimum_node_id: Option<NodeT>,
        node_type: Option<&str>,
        weight: Option<WeightT>,
        directed: Option<bool>,
        name: Option<&str>,
    ) -> Result<Graph> {
        if sides.is_empty() {
            return Err("The number of dimensions provided is zero.".to_string());
        }

        if sides.iter().any(|side| side.is_zero()) {
            return Err("One of the provided lattice sides is zero.".to_string());
        }
        let minimum_node_id = minimum_node_id.unwrap_or(0);
        let directed = directed.unwrap_or(false);
        let has_edge_weights = weight.is_some();

        // Get the generator the wheel in the middle of the two cliques
        let (number_of_nodes, number_of_edges, edges_iterator) = unsafe {
            get_squared_lattice_edges_iterator(
                sides,
                minimum_node_id,
                weight.unwrap_or(WeightT::NAN),
            )
        };

        let node_types = NodeTypeVocabulary::from_structs(
            vec![Some(vec![0]); number_of_nodes as usize],
            Vocabulary::from_reverse_map(vec![node_type.unwrap_or("squared_lattice").to_owned()])?,
        );
        let nodes = Vocabulary::from_range(minimum_node_id..(minimum_node_id + number_of_nodes));
        let edge_types_vocabulary: Vocabulary<EdgeTypeT> = Vocabulary::from_reverse_map(
            (0..sides.len())
                .map(|dimension| format!("Dimension_{}", dimension))
                .collect::<Vec<String>>(),
        )?;
        let name = name.unwrap_or("SquaredLattice");

        build_graph_from_integers(
            Some(edges_iterator),
            Arc::new(nodes),
            Arc::new(Some(node_types)),
            Some(edge_types_vocabulary),
            has_edge_weights,
            directed,
            Some(true),
            Some(false),
            Some(false),
            Some(number_of_edges),
            false,
            false,
            name.to_string(),
        )
    }
}
