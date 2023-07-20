use crate::constructors::build_graph_from_integers;

use super::*;
use atomic_float::AtomicF32;
use counter::Counter;
use indicatif::ParallelProgressIterator;
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;
use roaring::{RoaringBitmap, RoaringTreemap};
use std::cell::SyncUnsafeCell;
use std::collections::HashSet;
use vec_rand::cumsum_f32::cumsum_f32;
use vec_rand::sample_f32_from_cumsum;
use vec_rand::xorshift::xorshift as rand_u64;

/// Returns roaring tree map with the validation indices.
///
/// # Arguments
/// `k`: T - The number of folds.
/// `k_index`: T - The index of the current fold.
/// `mut indices`: Vec<u64> - The indices to sub-sample.
/// `random_state`: u64 - The random state for the kfold.
///
/// # Raises
/// * If the requested number of k-folds is higher than the number of elements.
/// * If the number of folds requested is one or zero.
/// * If the requested fold index is higher than the number of folds.
fn kfold<T: Copy + Eq + std::hash::Hash>(
    k: usize,
    k_index: usize,
    indices: &mut [T],
    random_state: u64,
) -> Result<&[T]> {
    if k > indices.len() {
        return Err(format!(
            concat!(
                "Cannot create a number of k-fold `{}` greater ",
                "than the number of available elements `{}`.\n",
                "This may be caused by an impossible stratified ",
                "k-fold."
            ),
            k,
            indices.len()
        ));
    }
    if k <= 1 {
        return Err(String::from(
            "Cannot do a k-fold with only one or zero folds.",
        ));
    }
    if k_index >= k {
        return Err(String::from(
            "The index of the k-fold must be strictly less than the number of folds.",
        ));
    }

    // if the graph has 8 edges and k = 3
    // we want the chunks sized to be:
    // 3, 3, 2

    // if the graph has 4 edges and k = 3
    // we want the chunks sized to be:
    // 2, 1, 1

    // shuffle the indices
    let mut rng = SmallRng::seed_from_u64(splitmix64(random_state) as EdgeT);
    indices.shuffle(&mut rng);

    // Get the k_index-th chunk
    let chunk_size = indices.len() as f64 / k as f64;
    let start = (k_index as f64 * chunk_size).ceil() as usize;
    let end = std::cmp::min(
        indices.len(),
        (((k_index + 1) as f64) * chunk_size).ceil() as usize,
    );
    // Return the chunk as a RoaringTreeMap
    Ok(&indices[start..end])
}

/// # Holdouts.
impl Graph {
    /// Returns node-wise filter to generate a subsampled graph.
    ///
    /// # Arguments
    /// * `minimum_node_degree`: Option<NodeT> - The minimum node degree of either the source or destination node to be sampled. By default 0.
    /// * `maximum_node_degree`: Option<NodeT> - The maximum node degree of either the source or destination node to be sampled. By default, the number of nodes.
    /// * `node_types_names: Option<Vec<String>> - Node type names of the nodes to be samples as sources. If a node has any of the provided node types, it can be sampled as a source node.
    /// * `edge_types_names`: Option<Vec<String>> - Edge type names of the nodes to be samples as sources. If a node has any of the provided edge types, it can be sampled as a source node.
    /// * `nodes_prefixes`: Option<Vec<String>> - Prefixes of the nodes names to be samples as sources. If a node starts with any of the provided prefixes, it can be sampled as a source node.
    /// * `support`: Option<&Graph> - Parent graph of this subgraph, defining the `true` topology of the graph. Node degrees and connected components are sampled from this support graph when provided. Useful when sampling negative edges for a test graph. In this latter case, the support graph should be the training graph.
    fn get_graph_sampling_node_filter<'a>(
        &'a self,
        minimum_node_degree: Option<NodeT>,
        maximum_node_degree: Option<NodeT>,
        node_types_names: Option<Vec<String>>,
        edge_types_names: Option<Vec<String>>,
        nodes_prefixes: Option<Vec<String>>,
        support: &'a Graph,
    ) -> Result<impl Fn(NodeT) -> bool + '_> {
        let minimum_node_degree = minimum_node_degree.unwrap_or(0);
        let maximum_node_degree = maximum_node_degree.unwrap_or(NodeT::MAX);
        let allowed_node_types_ids = if let Some(node_types_names) = node_types_names {
            if node_types_names.is_empty() {
                return Err("The provided vector `node_types_names` is empty!".to_string());
            }
            Some(
                node_types_names
                    .into_iter()
                    .map(|node_type_name| {
                        self.get_node_type_id_from_node_type_name(&node_type_name)
                    })
                    .collect::<Result<Vec<NodeTypeT>>>()?,
            )
        } else {
            None
        };

        let edge_types_ids = if let Some(edge_types_names) = edge_types_names {
            if edge_types_names.is_empty() {
                return Err("The provided vector `edge_types_names` is empty!".to_string());
            }
            Some(
                edge_types_names
                    .into_iter()
                    .map(|edge_type_name| {
                        self.get_edge_type_id_from_edge_type_name(Some(&edge_type_name))
                            .map(|et| et.unwrap())
                    })
                    .collect::<Result<Vec<EdgeTypeT>>>()?,
            )
        } else {
            None
        };

        Ok(move |node_id: NodeT| {
            // If the user has provided a set of the node types to sample, we check
            // whether the current node has a node type among those to keep.
            if let Some(allowed_node_types_ids) = &allowed_node_types_ids {
                // We retrieve the node node types.
                if let Some(node_type) =
                    unsafe { self.get_unchecked_node_type_ids_from_node_id(node_id) }
                {
                    // If none of the node types allowed appear in the current node,
                    // we discard it.
                    if allowed_node_types_ids
                        .iter()
                        .all(|allowed_node_type_id| !node_type.contains(allowed_node_type_id))
                    {
                        return false;
                    }
                } else {
                    return false;
                }
            }

            if let Some(nodes_prefixes) = &nodes_prefixes {
                let node_name = unsafe { self.get_unchecked_node_name_from_node_id(node_id) };
                if !nodes_prefixes
                    .iter()
                    .any(|prefix| node_name.starts_with(prefix))
                {
                    return false;
                }
            }

            if let Some(edge_types_ids) = &edge_types_ids {
                if !edge_types_ids.iter().copied().any(|edge_type_id| unsafe {
                    self.has_unchecked_edge_from_node_id_and_edge_type_id(
                        node_id,
                        Some(edge_type_id),
                    )
                }) {
                    return false;
                }
            }

            let node_degree = unsafe { support.get_unchecked_node_degree_from_node_id(node_id) };
            if (node_degree < minimum_node_degree) && (node_degree > maximum_node_degree) {
                return false;
            }

            true
        })
    }

    /// Returns edge-wise filter to generate a subsampled graph.
    ///
    /// # Arguments
    /// * `sample_only_edges_with_heterogeneous_node_types`: Option<bool> - Whether to sample edges only with source and destination nodes that have different node types.
    fn get_graph_sampling_edge_filter<'a>(
        &'a self,
        sample_only_edges_with_heterogeneous_node_types: Option<bool>,
    ) -> Result<impl Fn(NodeT, NodeT) -> bool + '_> {
        let sample_only_edges_with_heterogeneous_node_types =
            sample_only_edges_with_heterogeneous_node_types.unwrap_or(false);

        if sample_only_edges_with_heterogeneous_node_types && !self.has_node_types() {
            return Err(concat!(
            "The parameter `sample_only_edges_with_heterogeneous_node_types` was provided with value `true` ",
            "but the current graph instance does not contain any node type. ",
            "If you expected to have node types within this graph, maybe you have either dropped them ",
            "with a wrong filter operation or use the wrong parametrization to load the graph."
        ).to_string());
        }

        if sample_only_edges_with_heterogeneous_node_types
            && self.has_exclusively_homogeneous_node_types().unwrap()
        {
            return Err(concat!(
                "The parameter `sample_only_edges_with_heterogeneous_node_types` was provided with value `true` ",
                "but the current graph instance has exclusively homogeneous node types, that is all the nodes have ",
                "the same node type. ",
                "If you expected to have heterogeneous node types within this graph, maybe you have either dropped them ",
                "with a wrong filter operation or use the wrong parametrization to load the graph."
            ).to_string());
        }

        Ok(move |src: NodeT, dst: NodeT| {
            if sample_only_edges_with_heterogeneous_node_types
                && unsafe {
                    self.get_unchecked_node_type_ids_from_node_id(src)
                        == self.get_unchecked_node_type_ids_from_node_id(dst)
                }
            {
                return false;
            }

            true
        })
    }

    /// Returns Graph with given amount of negative edges as positive edges.
    ///
    /// The graph generated may be used as a testing negatives partition to be
    /// fed into the argument "graph_to_avoid" of the link_prediction or the
    /// skipgrams algorithm.
    ///
    /// # Arguments
    /// * `number_of_negative_samples`: EdgeT - Number of negatives edges to include.
    /// * `random_state`: Option<EdgeT> - random_state to use to reproduce negative edge set.
    /// * `only_from_same_component`: Option<bool> - Whether to sample negative edges only from nodes that are from the same component.
    /// * `sample_only_edges_with_heterogeneous_node_types`: Option<bool> - Whether to sample negative edges only with source and destination nodes that have different node types.
    /// * `minimum_node_degree`: Option<NodeT> - The minimum node degree of either the source or destination node to be sampled. By default 0.
    /// * `maximum_node_degree`: Option<NodeT> - The maximum node degree of either the source or destination node to be sampled. By default, the number of nodes.
    /// * `source_node_types_names: Option<Vec<String>> - Node type names of the nodes to be samples as sources. If a node has any of the provided node types, it can be sampled as a source node.
    /// * `destination_node_types_names`: Option<Vec<String>> - Node type names of the nodes to be samples as destinations. If a node has any of the provided node types, it can be sampled as a destination node.
    /// * `source_edge_types_names`: Option<Vec<String>> - Edge type names of the nodes to be samples as sources. If a node has any of the provided edge types, it can be sampled as a source node.
    /// * `destination_edge_types_names`: Option<Vec<String>> - Edge type names of the nodes to be samples as destinations. If a node has any of the provided edge types, it can be sampled as a destination node.
    /// * `source_nodes_prefixes`: Option<Vec<String>> - Prefixes of the nodes names to be samples as sources. If a node starts with any of the provided prefixes, it can be sampled as a source node.
    /// * `destination_nodes_prefixes`: Option<Vec<String>> - Prefixes of the nodes names to be samples as destinations. If a node starts with any of the provided prefixes, it can be sampled as a destinations node.
    /// * `graph_to_avoid`: Option<&Graph> - Compatible graph whose edges are not to be sampled.
    /// * `support`: Option<&Graph> - Parent graph of this subgraph, defining the `true` topology of the graph. Node degrees and connected components are sampled from this support graph when provided. Useful when sampling negative edges for a test graph. In this latter case, the support graph should be the training graph.
    /// * `use_scale_free_distribution`: Option<bool> - Whether to sample the nodes using scale_free distribution. By default True. Not using this may cause significant biases.
    /// * `sample_edge_types`: Option<bool> - Whether to sample edge types, following the edge type counts distribution. By default it is true only when the current graph instance has edge types.
    /// * `number_of_sampling_attempts`: Option<usize> - Number of times to attempt to sample edges before giving up.
    ///
    /// # Raises
    /// * If the `sample_only_edges_with_heterogeneous_node_types` argument is provided as true, but the graph does not have node types.
    pub fn sample_negative_graph(
        &self,
        number_of_negative_samples: EdgeT,
        random_state: Option<EdgeT>,
        only_from_same_component: Option<bool>,
        sample_only_edges_with_heterogeneous_node_types: Option<bool>,
        minimum_node_degree: Option<NodeT>,
        maximum_node_degree: Option<NodeT>,
        source_node_types_names: Option<Vec<String>>,
        destination_node_types_names: Option<Vec<String>>,
        source_edge_types_names: Option<Vec<String>>,
        destination_edge_types_names: Option<Vec<String>>,
        source_nodes_prefixes: Option<Vec<String>>,
        destination_nodes_prefixes: Option<Vec<String>>,
        graph_to_avoid: Option<&Graph>,
        support: Option<&Graph>,
        use_scale_free_distribution: Option<bool>,
        sample_edge_types: Option<bool>,
        number_of_sampling_attempts: Option<usize>,
    ) -> Result<Graph> {
        let number_of_sampling_attempts = number_of_sampling_attempts.unwrap_or(100_000);
        if number_of_negative_samples == 0 {
            return Err(String::from(
                "The number of negative samples cannot be zero.",
            ));
        }

        if let Some(graph_to_avoid) = graph_to_avoid.as_ref() {
            self.must_share_node_vocabulary(graph_to_avoid)?;
        }

        if let Some(support) = support.as_ref() {
            self.must_share_node_vocabulary(support)?;
        }

        let sample_edge_types = sample_edge_types.unwrap_or(self.has_edge_types());

        if sample_edge_types {
            self.must_have_edge_types()?;
        }

        let support = support.unwrap_or(&self);

        let source_node_filter = self.get_graph_sampling_node_filter(
            minimum_node_degree,
            maximum_node_degree,
            source_node_types_names,
            source_edge_types_names,
            source_nodes_prefixes,
            support,
        )?;

        let destination_node_filter = self.get_graph_sampling_node_filter(
            minimum_node_degree,
            maximum_node_degree,
            destination_node_types_names,
            destination_edge_types_names,
            destination_nodes_prefixes,
            support,
        )?;

        let use_scale_free_distribution = use_scale_free_distribution.unwrap_or(true);
        let only_from_same_component = only_from_same_component.unwrap_or(false);
        let mut random_state = random_state.unwrap_or(0xbadf00d);

        let number_of_filtered_source_nodes = self
            .par_iter_node_ids()
            .filter(|node_id| source_node_filter(*node_id))
            .count();

        // If more than 80% of the source node ids is filtered out, we need to collect
        // them into a vector in order to make it feaseable to sample from them without
        // risking functional deadlocks. The limit is set at 80/100 as it seems a sensible
        // tradeoff: it means that without collecting the source node ids, we will expect
        // to have to discard 4 out of 5 sampled nodes.
        //
        // The reason for not collecting the filtered nodes in all cases, is that in the
        // case of a graph with a large number of nodes, collecting them may be
        // computationally expensive to prohibitive - I am looking at you WikiData.
        let source_node_ids: Option<Vec<u32>> = if number_of_filtered_source_nodes
            < (self.get_number_of_nodes() as f64 * 0.2) as usize
        {
            Some(
                self.par_iter_node_ids()
                    .filter(|node_id| source_node_filter(*node_id))
                    .collect::<Vec<NodeT>>(),
            )
        } else {
            None
        };

        // Secondarily, we may have to sample the source node IDs using the scale free distribution
        // of the OUTBOUND node ids. This parameter may be requested in order to sample negative
        // edges that are more similar to the positive edges, and less trivial to predict. For
        // instance, suppose your graph is a star: in this case, if the negative edges were not
        // sampled using the scale free distribution but a uniform, it would be unlikely for any
        // of them to start from the center node. On the other hand, if the negative edges are
        // sampled using the scale free distribution, it is more likely that some of them will
        // start from the center node, making much less trivial to predict whether an edge is
        // real or not.
        //
        // When we are not applying any filter, and thefore the number_of_filtered_source_nodes
        // is equal to the number of nodes, we can rely on the existing outbound node degrees
        // from the CSR data structure and we do not need to re-allocate again the source node
        // degrees, which would be a waste of memory.
        let source_node_degrees_cumsum = if use_scale_free_distribution {
            // We use a vector of f32 instead of EdgeT (u64) so to use half of the memory,
            // and we do not need to be exact.
            if let Some(source_node_ids) = source_node_ids.as_ref() {
                let mut node_degrees = source_node_ids
                    .par_iter()
                    .copied()
                    .map(|src| unsafe {
                        support.get_unchecked_node_degree_from_node_id(src) as f32
                    })
                    .collect::<Vec<f32>>();
                cumsum_f32(&mut node_degrees);
                Some(node_degrees)
            } else {
                None
            }
        } else {
            None
        };

        // If more than 80% of the destination node ids is filtered out, we need to collect
        // them into a vector in order to make it feaseable to sample from them without
        // risking functional deadlocks. The limit is set at 80/100 as it seems a sensible
        // tradeoff: it means that without collecting the destination node ids, we will expect
        // to have to discard 4 out of 5 sampled nodes.

        let number_of_filtered_destination_nodes = self
            .par_iter_node_ids()
            .filter(|node_id| destination_node_filter(*node_id))
            .count();

        let destination_node_ids: Option<Vec<u32>> = if number_of_filtered_destination_nodes
            < (self.get_number_of_nodes() as f64 * 0.2) as usize
        {
            Some(
                self.par_iter_node_ids()
                    .filter(|node_id| destination_node_filter(*node_id))
                    .collect::<Vec<NodeT>>(),
            )
        } else {
            None
        };

        // Secondarily, we may have to sample the destination node IDs using the scale free distribution
        // of the INBOUND node ids. This parameter may be requested in order to sample negative
        // edges that are more similar to the positive edges, and less trivial to predict. For
        // instance, suppose your graph is a star: in this case, if the negative edges were not
        // sampled using the scale free distribution but a uniform, it would be unlikely for any
        // of them to end in the center node. On the other hand, if the negative edges are
        // sampled using the scale free distribution, it is more likely that some of them will
        // end in the center node, making much less trivial to predict whether an edge is
        // real or not.

        // Now, when we are sampling across ALL destination nodes, we can rely on the existing
        // destinations vector from the CSR, as sampling uniformely an element from that vector
        // is equal to sample a destination node using the scale free distribution of the
        // inbound node degrees. However, when we are sampling only from a very reduced subset
        // of destination nodes, we need to re-allocate the destination node degrees vector
        // and use it to sample the destination nodes using the scale free distribution.

        // The complexity here derives from the need to sample the destination nodes using the
        // INBOUND node degree, which in directed graph is not trivial. While in an undirected
        // graph OUTBOUND and INBOUND node degrees are the same, in a directed graph they are
        // not. To compute the INBOUND node degrees of the destination nodes, we allocate a vector
        // of AtomicF32, and we iterate over the edges in parallel. For each edge, we execute a
        // binary search to find the destination node id in the destination node ids vector, and
        // we increment the corresponding AtomicF32. Of course, all of this is solely necessary
        // if the graph we are currently processing is directed.
        // After this is done, we can transmute the AtomicF32 vector into a f32 vector, and
        // execute the comulative sum on it.

        let destination_node_degrees_cumsum = if use_scale_free_distribution {
            // We use a vector of f32 instead of EdgeT (u64) so to use half of the memory,
            // and we do not need to be exact.
            if let Some(destination_node_ids) = destination_node_ids.as_ref() {
                let mut inbound_node_degrees = if self.is_directed() {
                    let inbound_node_degrees =
                        unsafe {
                            std::mem::transmute::<Vec<f32>, Vec<AtomicF32>>(
                                vec![0.0; destination_node_ids.len()],
                            )
                        };
                    support
                        .par_iter_directed_edge_node_ids()
                        .for_each(|(_, _, dst)| {
                            if let Ok(index) = destination_node_ids.binary_search(&dst) {
                                inbound_node_degrees[index]
                                    .fetch_add(1.0, core::sync::atomic::Ordering::Relaxed);
                            }
                        });
                    unsafe { std::mem::transmute::<Vec<AtomicF32>, Vec<f32>>(inbound_node_degrees) }
                } else {
                    destination_node_ids
                        .par_iter()
                        .copied()
                        .map(|dst| unsafe {
                            support.get_unchecked_node_degree_from_node_id(dst) as f32
                        })
                        .collect::<Vec<f32>>()
                };
                cumsum_f32(&mut inbound_node_degrees);
                Some(inbound_node_degrees)
            } else {
                None
            }
        } else {
            None
        };

        let edge_wise_filter =
            self.get_graph_sampling_edge_filter(sample_only_edges_with_heterogeneous_node_types)?;

        // In a complete directed graph allowing selfloops with N nodes there are N^2
        // edges. In a complete directed graph without selfloops there are N*(N-1) edges.
        // We can rewrite the first formula as (N*(N-1)) + N.
        //
        // In a complete undirected graph allowing selfloops with N nodes there are
        // (N*(N-1))/2 + N edges.

        // Here we use unique edges number because on a multigraph the negative
        // edges cannot have an edge type.
        let number_of_nodes = self.get_number_of_nodes() as EdgeT;

        // whether to sample negative edges only from the same connected component.
        let (node_components, mut complete_number_of_edges) = if only_from_same_component {
            let node_components = support.get_node_connected_component_ids(Some(false));
            let complete_number_of_edges: EdgeT = Counter::init(node_components.clone())
                .into_iter()
                .map(|(_, number_of_nodes): (_, &usize)| {
                    let mut edge_number = (*number_of_nodes * (*number_of_nodes - 1)) as EdgeT;
                    if !self.is_directed() {
                        edge_number /= 2;
                    }
                    edge_number
                })
                .sum();
            (Some(node_components), complete_number_of_edges)
        } else {
            let mut edge_number = number_of_nodes * (number_of_nodes - 1);
            if !self.is_directed() {
                edge_number /= 2;
            }
            (None, edge_number)
        };

        // Here we compute the number of edges that a complete graph would have if it had the same number of nodes
        // of the current graph. Moreover, the complete graph will have selfloops IFF the current graph has at
        // least one of them.
        if self.has_selfloops() {
            complete_number_of_edges += number_of_nodes;
        }

        // Now we compute the maximum number of negative edges that we can actually generate
        let max_negative_edges = complete_number_of_edges - self.get_number_of_unique_edges();

        // We check that the number of requested negative edges is compatible with the
        // current graph instance.
        if number_of_negative_samples > max_negative_edges {
            return Err(format!(
                concat!(
                    "The requested negatives number {} is more than the ",
                    "number of negative edges that exist in the graph ({})."
                ),
                number_of_negative_samples, max_negative_edges
            ));
        }

        let mut negative_edges_hashset: HashSet<(NodeT, NodeT)> =
            HashSet::with_capacity(number_of_negative_samples as usize);
        let mut sampling_round: usize = 0;
        let mut last_size = 0;

        let sample_source_node = |random_state: EdgeT| -> NodeT {
            if let (Some(source_node_ids), Some(source_node_degrees_cumsum)) = (
                source_node_ids.as_ref(),
                source_node_degrees_cumsum.as_ref(),
            ) {
                source_node_ids
                    [sample_f32_from_cumsum(&source_node_degrees_cumsum, random_state) as usize]
            } else if use_scale_free_distribution {
                support.get_random_outbounds_scale_free_node(random_state)
            } else {
                support.get_random_node(random_state)
            }
        };

        let sample_destination_node = |random_state: EdgeT| -> NodeT {
            if let (Some(destination_node_ids), Some(destination_node_degrees_cumsum)) = (
                destination_node_ids.as_ref(),
                destination_node_degrees_cumsum.as_ref(),
            ) {
                destination_node_ids[sample_f32_from_cumsum(
                    &destination_node_degrees_cumsum,
                    random_state,
                ) as usize]
            } else if use_scale_free_distribution {
                support.get_random_inbounds_scale_free_node(random_state)
            } else {
                support.get_random_node(random_state)
            }
        };

        // randomly extract negative edges until we have the choosen number
        while negative_edges_hashset.len() < number_of_negative_samples as usize {
            // generate two random_states for reproducibility porpouses
            random_state = splitmix64(random_state as u64) as EdgeT;
            let src_random_state = rand_u64(random_state);
            random_state = splitmix64(random_state as u64) as EdgeT;
            let dst_random_state = rand_u64(random_state);

            let sampling_filter_map = |mut src, mut dst| {
                if !self.is_directed() && src > dst {
                    let tmp = dst;
                    dst = src;
                    src = tmp;
                }

                if !self.has_selfloops() && src == dst {
                    return None;
                }

                if !source_node_filter(src) || !destination_node_filter(dst) {
                    return None;
                }

                if !edge_wise_filter(src, dst) {
                    return None;
                }

                if let Some(graph_to_avoid) = &graph_to_avoid {
                    if graph_to_avoid.has_edge_from_node_ids(src, dst) {
                        return None;
                    }
                }

                if let Some(ncs) = &node_components {
                    if ncs[src as usize] != ncs[dst as usize] {
                        return None;
                    }
                }

                if self.has_edge_from_node_ids(src, dst) {
                    return None;
                }

                if negative_edges_hashset.contains(&(src, dst)) {
                    return None;
                }

                Some((src, dst))
            };

            // generate the random edge-sources
            let sampled_edge_ids = (0..number_of_negative_samples)
                .into_par_iter()
                .filter_map(|i| {
                    let src = sample_source_node(
                        src_random_state.wrapping_add(src_random_state + i as u64),
                    );
                    let dst = sample_destination_node(
                        dst_random_state.wrapping_add(dst_random_state + i as u64),
                    );
                    sampling_filter_map(src, dst)
                })
                .collect::<Vec<(NodeT, NodeT)>>();

            for edge_id in sampled_edge_ids.iter() {
                if negative_edges_hashset.len() >= number_of_negative_samples as usize {
                    break;
                }
                negative_edges_hashset.insert(*edge_id);
            }

            if sampling_round > number_of_sampling_attempts {
                return Err(format!(concat!(
                    "Using the provided filters on the current graph instance it ",
                    "was not possible to sample a new negative edge after {number_of_sampling_attempts} sampling ",
                    "rounds."
                ), number_of_sampling_attempts=number_of_sampling_attempts));
            }

            if negative_edges_hashset.len() > last_size {
                last_size = negative_edges_hashset.len();
                sampling_round = 0;
            } else {
                sampling_round += 1;
            }
        }

        build_graph_from_integers(
            Some(
                negative_edges_hashset
                    .into_par_iter()
                    .map(|(src, dst)| unsafe {
                        (
                            0,
                            (
                                src,
                                dst,
                                if sample_edge_types {
                                    self.get_unchecked_random_scale_free_edge_type(
                                        random_state
                                            .wrapping_mul(src as u64 + 1)
                                            .wrapping_mul(dst as u64 + 2),
                                    )
                                } else {
                                    None
                                },
                                WeightT::NAN,
                            ),
                        )
                    }),
            ),
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types
                .as_ref()
                .as_ref()
                .map(|ets| ets.vocabulary.clone()),
            false,
            self.is_directed(),
            Some(false),
            Some(false),
            Some(false),
            None,
            true,
            self.has_selfloops(),
            format!("Negative {}", self.get_name()),
        )
    }

    /// Returns Graph with given amount of subsampled edges.
    ///
    /// # Arguments
    /// * `number_of_samples`: usize - Number of edges to include.
    /// * `random_state`: Option<EdgeT> - random_state to use to reproduce negative edge set.
    /// * `sample_only_edges_with_heterogeneous_node_types`: Option<bool> - Whether to sample negative edges only with source and destination nodes that have different node types.
    /// * `minimum_node_degree`: Option<NodeT> - The minimum node degree of either the source or destination node to be sampled. By default 0.
    /// * `maximum_node_degree`: Option<NodeT> - The maximum node degree of either the source or destination node to be sampled. By default, the number of nodes.
    /// * `source_node_types_names: Option<Vec<String>> - Node type names of the nodes to be samples as sources. If a node has any of the provided node types, it can be sampled as a source node.
    /// * `destination_node_types_names`: Option<Vec<String>> - Node type names of the nodes to be samples as destinations. If a node has any of the provided node types, it can be sampled as a destination node.
    /// * `source_edge_types_names`: Option<Vec<String>> - Edge type names of the nodes to be samples as sources. If a node has any of the provided edge types, it can be sampled as a source node.
    /// * `destination_edge_types_names`: Option<Vec<String>> - Edge type names of the nodes to be samples as destinations. If a node has any of the provided edge types, it can be sampled as a destination node.
    /// * `source_nodes_prefixes`: Option<Vec<String>> - Prefixes of the nodes names to be samples as sources. If a node starts with any of the provided prefixes, it can be sampled as a source node.
    /// * `destination_nodes_prefixes`: Option<Vec<String>> - Prefixes of the nodes names to be samples as destinations. If a node starts with any of the provided prefixes, it can be sampled as a destinations node.
    /// * `edge_type_names`: Option<&[Option<&str>]> - Edge type names of the edges to sample. Only edges with ANY of these edge types will be kept.
    /// * `support`: Option<&Graph> - Parent graph of this subgraph, defining the `true` topology of the graph. Node degrees are sampled from this support graph when provided. Useful when sampling positive edges for a test graph. In this latter case, the support graph should be the training graph.
    /// * `number_of_sampling_attempts`: Option<usize> - Number of times to attempt to sample edges before giving up.
    ///
    /// # Raises
    /// * If the `sample_only_edges_with_heterogeneous_node_types` argument is provided as true, but the graph does not have node types.
    pub fn sample_positive_graph(
        &self,
        number_of_samples: usize,
        random_state: Option<EdgeT>,
        sample_only_edges_with_heterogeneous_node_types: Option<bool>,
        minimum_node_degree: Option<NodeT>,
        maximum_node_degree: Option<NodeT>,
        source_node_types_names: Option<Vec<String>>,
        destination_node_types_names: Option<Vec<String>>,
        source_edge_types_names: Option<Vec<String>>,
        destination_edge_types_names: Option<Vec<String>>,
        source_nodes_prefixes: Option<Vec<String>>,
        destination_nodes_prefixes: Option<Vec<String>>,
        edge_type_names: Option<&[Option<&str>]>,
        support: Option<&Graph>,
        number_of_sampling_attempts: Option<usize>,
    ) -> Result<Graph> {
        let number_of_sampling_attempts = number_of_sampling_attempts.unwrap_or(100_000);
        if number_of_samples == 0 {
            return Err(String::from("The number of samples cannot be zero."));
        }

        if let Some(support) = support.as_ref() {
            self.must_share_node_vocabulary(support)?;
        }

        let support = support.unwrap_or(&self);
        let mut random_state = splitmix64(random_state.unwrap_or(42));

        let source_node_filter = self.get_graph_sampling_node_filter(
            minimum_node_degree,
            maximum_node_degree,
            source_node_types_names,
            source_edge_types_names,
            source_nodes_prefixes,
            support,
        )?;

        let destination_node_filter = self.get_graph_sampling_node_filter(
            minimum_node_degree,
            maximum_node_degree,
            destination_node_types_names,
            destination_edge_types_names,
            destination_nodes_prefixes,
            support,
        )?;

        let edge_wise_filter =
            self.get_graph_sampling_edge_filter(sample_only_edges_with_heterogeneous_node_types)?;

        let edge_type_ids = if let Some(edge_type_names) = edge_type_names {
            Some(self.get_edge_type_ids_from_edge_type_names(edge_type_names)?)
        } else {
            None
        };

        let mut edges_hashset = HashSet::with_capacity(number_of_samples as usize);
        let mut sampling_round: usize = 0;

        // randomly extract negative edges until we have the choosen number
        while edges_hashset.len() < number_of_samples as usize {
            // generate two random_states for reproducibility porpouses
            random_state = splitmix64(random_state as u64) as EdgeT;

            sampling_round += 1;

            let sampling_filter_map = |edge_id| {
                let (src, dst) = unsafe { self.get_unchecked_node_ids_from_edge_id(edge_id) };

                if !self.is_directed() && src > dst {
                    return None;
                }

                if !(source_node_filter(src)
                    && destination_node_filter(dst)
                    && (self.is_directed()
                        || source_node_filter(dst) && destination_node_filter(src)))
                {
                    return None;
                }

                if edge_type_ids.as_ref().map_or(false, |edge_type_ids| {
                    let edge_type_id =
                        unsafe { self.get_unchecked_edge_type_id_from_edge_id(edge_id) };
                    !edge_type_ids.iter().any(|this_edge_type_id| {
                        match (this_edge_type_id, edge_type_id) {
                            (None, None) => true,
                            (Some(e1), Some(e2)) => *e1 == e2,
                            _ => false,
                        }
                    })
                }) {
                    return None;
                }

                if !edge_wise_filter(src, dst) {
                    return None;
                }

                if edges_hashset.contains(&edge_id) {
                    return None;
                }

                Some(edge_id)
            };

            // generate the random edge-sources
            let sampled_edge_ids = self
                .par_iter_random_uniform_edge_ids(number_of_samples as usize, random_state)
                .filter_map(|edge_id| sampling_filter_map(edge_id))
                .collect::<Vec<EdgeT>>();

            // If we have found at least one edge, we can reset the sampling round counter.
            if !sampled_edge_ids.is_empty() {
                sampling_round = 0;
            }

            for edge_id in sampled_edge_ids.iter() {
                if edges_hashset.len() >= number_of_samples as usize {
                    break;
                }
                edges_hashset.insert(*edge_id);
            }

            if sampling_round > number_of_sampling_attempts {
                return Err(format!(
                    concat!(
                        "Using the provided filters on the current graph instance it ",
                        "was not possible to sample a new positive edge after {number_of_sampling_attempts} sampling ",
                        "rounds. We have found {number_of_edges} edges out of {number_of_samples} samples."
                    ),
                    number_of_sampling_attempts=number_of_sampling_attempts,
                    number_of_edges=edges_hashset.len(),
                    number_of_samples=number_of_samples
                ));
            }
        }

        build_graph_from_integers(
            Some(edges_hashset.into_par_iter().map(|edge_id| unsafe {
                let (src, dst) = self.get_unchecked_node_ids_from_edge_id(edge_id);
                (
                    0,
                    (
                        src,
                        dst,
                        self.get_unchecked_edge_type_id_from_edge_id(edge_id),
                        self.get_unchecked_edge_weight_from_edge_id(edge_id)
                            .unwrap_or(f32::NAN),
                    ),
                )
            })),
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types
                .as_ref()
                .as_ref()
                .map(|ets| ets.vocabulary.clone()),
            self.has_edge_weights(),
            self.is_directed(),
            Some(false),
            Some(false),
            Some(false),
            None,
            true,
            self.has_selfloops(),
            format!("Subsampled {}", self.get_name()),
        )
    }

    /// Compute the training and validation elements number from the training rate
    ///
    /// # Raises
    /// * If the training size is either greater than one or negative.
    /// * If the graph instance has only one edge.
    /// * If the resulting training edges number is 0.
    /// * If the resulting validation edges number is 0.
    fn get_holdouts_elements_number(
        &self,
        train_size: f64,
        total_elements: usize,
    ) -> Result<(usize, usize)> {
        if train_size <= 0.0 || train_size >= 1.0 {
            return Err(String::from("Train rate must be strictly between 0 and 1."));
        }
        if self.directed && self.get_number_of_directed_edges() == 1
            || !self.directed && self.get_number_of_directed_edges() == 2
        {
            return Err(String::from(
                "The current graph instance has only one edge. You cannot build an holdout with one edge.",
            ));
        }
        let train_elements_number = (total_elements as f64 * train_size) as usize;
        let valid_elements_number = total_elements - train_elements_number;

        if train_elements_number == 0 || train_elements_number >= total_elements {
            return Err(String::from(
                "The training set has 0 elements! Change the training rate.",
            ));
        }
        if valid_elements_number == 0 {
            return Err(String::from(
                "The validation set has 0 elements! Change the training rate.",
            ));
        }

        Ok((train_elements_number, valid_elements_number))
    }

    /// Returns training and validation graph.
    ///
    /// # Arguments
    /// * `random_state`: Option<EdgeT> - The random state to reproduce the holdout.
    /// * `validation_number_of_edges`: EdgeT - The number of edges to reserve for the validation graph.
    /// * `include_all_edge_types`: bool - Whether to include all the edge types in the graph, if the graph is a multigraph.
    /// * `user_condition_for_validation_edges`: impl Fn(EdgeT, NodeT, NodeT, Option<EdgeTypeT>) -> bool - The function to use to put edges in validation set.
    /// * `verbose`: Option<bool> - Whether to show the loading bar or not.
    /// * `train_graph_might_contain_singletons`: bool - Whether it is known that the resulting training graph may have singletons.
    /// * `train_graph_might_contain_singletons_with_selfloops`: bool - Whether it is known that the resulting training graph may have singletons with selfloops.
    ///
    /// # Raises
    /// * If the sampled validation edges are not enough for the required validation edges number.
    fn get_edge_holdout(
        &self,
        random_state: Option<EdgeT>,
        validation_number_of_edges: EdgeT,
        include_all_edge_types: bool,
        user_condition_for_validation_edges: impl Fn(EdgeT, NodeT, NodeT, Option<EdgeTypeT>) -> bool,
        verbose: Option<bool>,
    ) -> Result<(Graph, Graph)> {
        let verbose = verbose.unwrap_or(false);
        let random_state = random_state.unwrap_or(0xbadf00d);
        let validation_edges_pb = get_loading_bar(
            verbose,
            "Picking validation edges",
            validation_number_of_edges as usize,
        );

        // generate and shuffle the indices of the edges
        let mut rng = SmallRng::seed_from_u64(splitmix64(random_state as u64) as EdgeT);
        let mut edge_indices: Vec<EdgeT> = (0..self.get_number_of_directed_edges()).collect();
        edge_indices.shuffle(&mut rng);

        let mut valid_edges_bitmap = RoaringTreemap::new();
        let mut last_length = 0;

        for (edge_id, (src, dst, edge_type)) in edge_indices.into_iter().map(|edge_id| {
            (edge_id, unsafe {
                self.get_unchecked_node_ids_and_edge_type_id_from_edge_id(edge_id)
            })
        }) {
            // If the graph is undirected and we have extracted an edge that is a
            // simmetric one, we can skip this iteration.
            if !self.directed && src > dst {
                continue;
            }

            // We stop adding edges when we have reached the minimum amount.
            if user_condition_for_validation_edges(edge_id, src, dst, edge_type) {
                // Compute the forward edge ids that are required.
                valid_edges_bitmap.extend(self.compute_edge_ids_vector(
                    edge_id,
                    src,
                    dst,
                    include_all_edge_types,
                ));

                // If the graph is undirected
                if !self.directed {
                    // we compute also the backward edge ids that are required.
                    valid_edges_bitmap.extend(self.compute_edge_ids_vector(
                        unsafe {
                            self.get_unchecked_edge_id_from_node_ids_and_edge_type_id(
                                dst, src, edge_type,
                            )
                        },
                        dst,
                        src,
                        include_all_edge_types,
                    ));
                }
                validation_edges_pb.inc(valid_edges_bitmap.len() - last_length);
                last_length = valid_edges_bitmap.len();
            }

            // We stop the iteration when we found all the edges.
            if valid_edges_bitmap.len() >= validation_number_of_edges {
                break;
            }
        }

        if valid_edges_bitmap.len() < validation_number_of_edges {
            let actual_validation_number_of_edges = valid_edges_bitmap.len();
            return Err(format!(
                concat!(
                    "With the given configuration for the holdout, it is not possible to ",
                    "generate a validation set composed of {validation_number_of_edges} edges from the current graph.\n",
                    "The validation set can be composed of at most {actual_validation_number_of_edges} edges.\n"
                ),
                validation_number_of_edges=validation_number_of_edges,
                actual_validation_number_of_edges=actual_validation_number_of_edges,
            ));
        }
        let validation_edge_ids = (0..self.get_number_of_directed_edges())
            .into_par_iter()
            .filter(|edge_id| valid_edges_bitmap.contains(*edge_id))
            .collect::<Vec<_>>();

        let train_edge_ids = (0..self.get_number_of_directed_edges())
            .into_par_iter()
            .filter(|edge_id| !valid_edges_bitmap.contains(*edge_id))
            .collect::<Vec<_>>();

        let train_number_of_edges = train_edge_ids.len();
        let validation_number_of_edges = validation_edge_ids.len();

        Ok((
            build_graph_from_integers(
                Some(
                    train_edge_ids
                        .into_par_iter()
                        .enumerate()
                        .map(|(i, edge_id)| unsafe {
                            let (src, dst, edge_type, weight) = self
                            .get_unchecked_node_ids_and_edge_type_id_and_edge_weight_from_edge_id(
                                edge_id,
                            );
                            (i, (src, dst, edge_type, weight.unwrap_or(WeightT::NAN)))
                        }),
                ),
                self.nodes.clone(),
                self.node_types.clone(),
                self.edge_types
                    .as_ref()
                    .as_ref()
                    .map(|ets| ets.vocabulary.clone()),
                self.has_edge_weights(),
                self.is_directed(),
                Some(true),
                Some(false),
                Some(true),
                Some(train_number_of_edges as EdgeT),
                true,
                self.has_selfloops(),
                format!("{} train", self.get_name()),
            )?,
            build_graph_from_integers(
                Some(
                    validation_edge_ids
                        .into_par_iter()
                        .enumerate()
                        .map(|(i, edge_id)| unsafe {
                            let (src, dst, edge_type, weight) = self
                            .get_unchecked_node_ids_and_edge_type_id_and_edge_weight_from_edge_id(
                                edge_id,
                            );
                            (i, (src, dst, edge_type, weight.unwrap_or(WeightT::NAN)))
                        }),
                ),
                self.nodes.clone(),
                self.node_types.clone(),
                self.edge_types
                    .as_ref()
                    .as_ref()
                    .map(|ets| ets.vocabulary.clone()),
                self.has_edge_weights(),
                self.is_directed(),
                Some(true),
                Some(false),
                Some(true),
                Some(validation_number_of_edges as EdgeT),
                true,
                self.has_selfloops(),
                format!("{} test", self.get_name()),
            )?,
        ))
    }

    /// Returns holdout for training ML algorithms on the graph structure.
    ///
    /// The holdouts returned are a tuple of graphs. The first one, which
    /// is the training graph, is garanteed to have the same number of
    /// graph components as the initial graph. The second graph is the graph
    /// meant for testing or validation of the algorithm, and has no garantee
    /// to be connected. It will have at most (1-train_size) edges,
    /// as the bound of connectivity which is required for the training graph
    /// may lead to more edges being left into the training partition.
    ///
    /// In the option where a list of edge types has been provided, these
    /// edge types will be those put into the validation set.
    ///
    /// # Arguments
    ///
    /// * `train_size`: f64 - Rate target to reserve for training.
    /// * `random_state`: Option<EdgeT> - The random_state to use for the holdout,
    /// * `edge_types`: Option<&[Option<&str>]> - Edge types to be selected for in the validation set.
    /// * `include_all_edge_types`: Option<bool> - Whether to include all the edges between two nodes.
    /// * `minimum_node_degree`: Option<NodeT> - The minimum node degree of either the source or destination node to be sampled. By default 0.
    /// * `maximum_node_degree`: Option<NodeT> - The maximum node degree of either the source or destination node to be sampled. By default, the number of nodes.
    /// * `verbose`: Option<bool> - Whether to show the loading bar.
    ///
    /// # Raises
    /// * If the edge types have been specified but the graph does not have edge types.
    /// * If the required training size is not a real value between 0 and 1.
    /// * If the current graph does not allow for the creation of a spanning tree for the requested training size.
    pub fn connected_holdout(
        &self,
        train_size: f64,
        random_state: Option<EdgeT>,
        edge_types: Option<&[Option<&str>]>,
        include_all_edge_types: Option<bool>,
        minimum_node_degree: Option<NodeT>,
        maximum_node_degree: Option<NodeT>,
        verbose: Option<bool>,
    ) -> Result<(Graph, Graph)> {
        let include_all_edge_types = include_all_edge_types.unwrap_or(false);
        // If the user has requested to restrict the connected holdout to a
        // limited set of edge types, the graph must have edge types.
        if edge_types.is_some() {
            self.must_have_edge_types()?;
        }
        if train_size <= 0.0 || train_size >= 1.0 {
            return Err(String::from("Train rate must be strictly between 0 and 1."));
        }

        let edge_type_ids = edge_types.clone().map_or(Ok::<_, String>(None), |ets| {
            Ok(Some(
                self.get_edge_type_ids_from_edge_type_names(ets)?
                    .into_iter()
                    .collect::<HashSet<Option<EdgeTypeT>>>(),
            ))
        })?;

        let tree = self
            .random_spanning_arborescence_kruskal(random_state, edge_type_ids.clone(), verbose)
            .0;

        let edge_factor = if self.is_directed() { 1 } else { 2 };

        // We need to check if the connected holdout can actually be built with
        // the additional constraint of the edge types.
        let validation_number_of_edges = if let Some(etis) = &edge_type_ids {
            let selected_number_of_edges: EdgeT = etis
                .iter()
                .map(|et| unsafe { self.get_unchecked_edge_count_from_edge_type_id(*et) } as EdgeT)
                .sum();
            if selected_number_of_edges == 0 {
                return Err(format!(
                    concat!(
                        "The provided list of edge type(s) ({}) do exist in the current graph ",
                        "edge types dictionary, but they do not have any edge assigned ",
                        "to them, and therefore would create an empty validation set."
                    ),
                    edge_types
                        .unwrap()
                        .iter()
                        .cloned()
                        .filter_map(|e| e)
                        .collect::<Vec<&str>>()
                        .join(", ")
                ));
            }
            (selected_number_of_edges as f64 * (1.0 - train_size)) as EdgeT
        } else {
            (self.get_number_of_directed_edges() as f64 * (1.0 - train_size)) as EdgeT
        };
        let train_number_of_edges =
            self.get_number_of_directed_edges() - validation_number_of_edges;

        if tree.len() * edge_factor > train_number_of_edges as usize {
            return Err(format!(
                concat!(
                    "The given spanning tree of the graph contains {} edges ",
                    "that is more than the required training edges number {}.\n",
                    "This makes impossible to create a validation set using ",
                    "{} edges.\nIf possible, you should increase the ",
                    "train_size parameter which is currently equal to ",
                    "{}.\nThe deny map, by itself, is requiring at least ",
                    "a train rate of {}."
                ),
                tree.len() * edge_factor,
                train_number_of_edges,
                validation_number_of_edges,
                train_size,
                (tree.len() * edge_factor) as f64 / self.get_number_of_directed_edges() as f64
            ));
        }

        self.get_edge_holdout(
            random_state,
            validation_number_of_edges,
            include_all_edge_types,
            |_, src, dst, edge_type| {
                let is_in_tree = tree.contains(&(src, dst));
                unsafe {
                    if let Some(minimum_node_degree) = &minimum_node_degree {
                        if self.get_unchecked_node_degree_from_node_id(src) < *minimum_node_degree
                            || self.get_unchecked_node_degree_from_node_id(dst)
                                < *minimum_node_degree
                        {
                            return false;
                        }
                    }

                    if let Some(maximum_node_degree) = &maximum_node_degree {
                        if self.get_unchecked_node_degree_from_node_id(src) > *maximum_node_degree
                            || self.get_unchecked_node_degree_from_node_id(dst)
                                > *maximum_node_degree
                        {
                            return false;
                        }
                    }
                }
                let singleton_selfloop =
                    unsafe { self.is_unchecked_singleton_with_selfloops_from_node_id(src) };
                let correct_edge_type = edge_type_ids
                    .as_ref()
                    .map_or(true, |etis| etis.contains(&edge_type));
                // The tree must not contain the provided edge ID
                // And this is not a self-loop edge with degree 1
                // And the edge type of the edge ID is within the provided edge type
                !is_in_tree && !singleton_selfloop && correct_edge_type
            },
            verbose,
        )
    }

    /// Returns random holdout for training ML algorithms on the graph edges.
    ///
    /// The holdouts returned are a tuple of graphs. In neither holdouts the
    /// graph connectivity is necessarily preserved. To maintain that, use
    /// the method `connected_holdout`.
    ///
    /// # Arguments
    ///
    /// * `train_size`: f64 - rate target to reserve for training
    /// * `random_state`: Option<EdgeT> - The random_state to use for the holdout,
    /// * `include_all_edge_types`: Option<bool> - Whether to include all the edges between two nodes.
    /// * `edge_types`: Option<&[Option<&str>]> - The edges to include in validation set.
    /// * `min_number_overlaps`: Option<EdgeT> - The minimum number of overlaps to include the edge into the validation set.
    /// * `verbose`: Option<bool> - Whether to show the loading bar.
    ///
    /// # Raises
    /// * If the edge types have been specified but the graph does not have edge types.
    /// * If the minimum number of overlaps have been specified but the graph is not a multigraph.
    /// * If one or more of the given edge type names is not present in the graph.
    pub fn random_holdout(
        &self,
        train_size: f64,
        random_state: Option<EdgeT>,
        include_all_edge_types: Option<bool>,
        edge_types: Option<&[Option<&str>]>,
        min_number_overlaps: Option<EdgeT>,
        verbose: Option<bool>,
    ) -> Result<(Graph, Graph)> {
        let include_all_edge_types = include_all_edge_types.unwrap_or(false);
        // If the user has requested to restrict the connected holdout to a
        // limited set of edge types, the graph must have edge types.
        if edge_types.is_some() {
            self.must_have_edge_types()?;
        }
        let total_number_of_edges = if include_all_edge_types {
            self.get_number_of_unique_edges()
        } else {
            self.get_number_of_directed_edges()
        };

        let (_, validation_number_of_edges) =
            self.get_holdouts_elements_number(train_size, total_number_of_edges as usize)?;
        let edge_type_ids = edge_types.map_or(Ok::<_, String>(None), |ets| {
            Ok(Some(
                self.get_edge_type_ids_from_edge_type_names(ets)?
                    .into_iter()
                    .collect::<HashSet<Option<EdgeTypeT>>>(),
            ))
        })?;
        if min_number_overlaps.is_some() {
            self.must_be_multigraph()?;
        }
        self.get_edge_holdout(
            random_state,
            validation_number_of_edges as EdgeT,
            include_all_edge_types,
            |_, src, dst, edge_type| {
                // If a list of edge types was provided and the edge type
                // of the current edge is not within the provided list,
                // we skip the current edge.
                if !edge_type_ids
                    .as_ref()
                    .map_or(true, |etis| etis.contains(&edge_type))
                {
                    return false;
                }
                // If a minimum number of overlaps was provided and the current
                // edge has not the required minimum amount of overlaps.
                if let Some(mno) = min_number_overlaps {
                    if self.get_unchecked_edge_degree_from_node_ids(src, dst) < mno {
                        return false;
                    }
                }
                // Otherwise we accept the provided edge for the validation set
                true
            },
            verbose,
        )
    }

    /// Returns node-label holdout indices for training ML algorithms on the graph node labels.
    ///
    /// # Arguments
    /// * `train_size`: f64 - rate target to reserve for training,
    /// * `use_stratification`: Option<bool> - Whether to use node-label stratification,
    /// * `random_state`: Option<EdgeT> - The random_state to use for the holdout,
    ///
    /// # Example
    /// This example create an 80-20 split of the nodes in the graph
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    ///   let (train, test) = graph.get_node_label_holdout_indices(0.8, Some(true), None).unwrap();
    /// ```
    ///
    /// # Raises
    /// * If the graph does not have node types.
    /// * If stratification is requested but the graph has a single node type.
    /// * If stratification is requested but the graph has a multilabel node types.
    pub fn get_node_label_holdout_indices(
        &self,
        train_size: f64,
        use_stratification: Option<bool>,
        random_state: Option<EdgeT>,
    ) -> Result<(Vec<NodeT>, Vec<NodeT>)> {
        self.must_have_node_types()?;
        let random_state = random_state.unwrap_or(0xbadf00d);
        let use_stratification = use_stratification.unwrap_or(false);
        if use_stratification {
            if self.has_multilabel_node_types()? {
                return Err("It is impossible to create a stratified holdout when the graph has multi-label node types.".to_string());
            }
            if self.has_singleton_node_types()? {
                return Err("It is impossible to create a stratified holdout when the graph has node types with cardinality one.".to_string());
            }
        }

        if self.get_number_of_known_node_types()? < 2 {
            return Err("It is not possible to create a node label holdout when the number of nodes with known node type is less than two.".to_string());
        }

        // Compute the vectors with the indices of the nodes which node type matches
        // therefore the expected shape is:
        // (number_of_node_types, number of nodes of that node type)
        let node_sets: Vec<Vec<NodeT>> = self
            .node_types
            .as_ref()
            .as_ref()
            .map(|nts| {
                if use_stratification {
                    // Initialize the vectors for each node type
                    let mut node_sets: Vec<Vec<NodeT>> =
                        vec![Vec::new(); self.get_number_of_node_types().unwrap() as usize];
                    // itering over the indices and adding each node to the
                    // vector of the corresponding node type.
                    nts.ids.iter().enumerate().for_each(|(node_id, node_type)| {
                        // if the node has a node_type
                        if let Some(nt) = node_type {
                            // Get the index of the correct node type vector.
                            node_sets[nt[0] as usize].push(node_id as NodeT);
                        };
                    });
                    node_sets
                } else {
                    // just compute a vector with a single vector of the indices
                    //  of the nodes with node
                    vec![nts
                        .ids
                        .iter()
                        .enumerate()
                        .filter_map(|(node_id, node_type)| {
                            node_type.as_ref().map(|_| node_id as NodeT)
                        })
                        .collect()]
                }
            })
            .unwrap();

        // initialize the seed for a re-producible shuffle
        let mut rnd = SmallRng::seed_from_u64(splitmix64(random_state as u64));

        // Allocate the vectors for the nodes of each
        let mut train_node_indices: Vec<NodeT> =
            Vec::with_capacity(self.get_number_of_nodes() as usize);
        let mut test_node_indices: Vec<NodeT> =
            Vec::with_capacity(self.get_number_of_nodes() as usize);

        for mut node_set in node_sets {
            // Shuffle in a reproducible way the nodes of the current node_type
            node_set.shuffle(&mut rnd);
            // Compute how many of these nodes belongs to the training set
            let (train_size, _) = self.get_holdouts_elements_number(train_size, node_set.len())?;
            // Extend the node indices
            train_node_indices.extend(node_set[..train_size].iter().cloned());
            test_node_indices.extend(node_set[train_size..].into_iter());
        }

        Ok((train_node_indices, test_node_indices))
    }

    /// Returns node-label holdout indices for training ML algorithms on the graph node labels.
    ///
    /// # Arguments
    /// * `train_size`: f64 - rate target to reserve for training,
    /// * `use_stratification`: Option<bool> - Whether to use node-label stratification,
    /// * `random_state`: Option<EdgeT> - The random_state to use for the holdout,
    ///
    /// # Example
    /// This example create an 80-20 split of the nodes in the graph
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    ///   let (train, test) = graph.get_node_label_holdout_indices(0.8, Some(true), None).unwrap();
    /// ```
    ///
    /// # Raises
    /// * If the graph does not have node types.
    /// * If stratification is requested but the graph has a single node type.
    /// * If stratification is requested but the graph has a multilabel node types.
    pub fn get_node_label_holdout_labels(
        &self,
        train_size: f64,
        use_stratification: Option<bool>,
        random_state: Option<EdgeT>,
    ) -> Result<(Vec<Option<Vec<NodeTypeT>>>, Vec<Option<Vec<NodeTypeT>>>)> {
        // Retrieve the train and test node indices
        let (train_node_indices, test_node_indices) =
            self.get_node_label_holdout_indices(train_size, use_stratification, random_state)?;

        // Allocate the vectors for the nodes of each
        // For the training node types
        let mut train_node_types = vec![None; self.get_number_of_nodes() as usize];
        train_node_indices.into_iter().for_each(|node_id| unsafe {
            train_node_types[node_id as usize] = self
                .get_unchecked_node_type_ids_from_node_id(node_id)
                .map(|x| x.clone())
        });
        // For the test node types
        let mut test_node_types = vec![None; self.get_number_of_nodes() as usize];
        test_node_indices.into_iter().for_each(|node_id| unsafe {
            test_node_types[node_id as usize] = self
                .get_unchecked_node_type_ids_from_node_id(node_id)
                .map(|x| x.clone())
        });

        Ok((
            train_node_types
                .iter()
                .map(|x| x.map(|y| y.to_vec()))
                .collect::<Vec<_>>(),
            test_node_types
                .iter()
                .map(|x| x.map(|y| y.to_vec()))
                .collect::<Vec<_>>(),
        ))
    }

    /// Returns node-label holdout for training ML algorithms on the graph node labels.
    ///
    /// # Arguments
    /// * `train_size`: f64 - rate target to reserve for training,
    /// * `use_stratification`: Option<bool> - Whether to use node-label stratification,
    /// * `random_state`: Option<EdgeT> - The random_state to use for the holdout,
    ///
    /// # Example
    /// This example create an 80-20 split of the nodes in the graph
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    ///   let (train, test) = graph.get_node_label_holdout_graphs(0.8, Some(true), None).unwrap();
    /// ```
    ///
    /// # Raises
    /// * If the graph does not have node types.
    /// * If stratification is requested but the graph has a single node type.
    /// * If stratification is requested but the graph has a multilabel node types.
    pub fn get_node_label_holdout_graphs(
        &self,
        train_size: f64,
        use_stratification: Option<bool>,
        random_state: Option<EdgeT>,
    ) -> Result<(Graph, Graph)> {
        // Retrieve the node label holdouts indices.
        let (train_node_types, test_node_types) =
            self.get_node_label_holdout_labels(train_size, use_stratification, random_state)?;

        // Clone the current graph
        // here we could manually initialize the clones so that we don't waste
        // time and memory cloning the node_types which will be immediately
        // overwrite. We argue that this should not be impactfull so we prefer
        // to prioritze the simplicity of the code
        let mut train_graph = self.clone();
        let mut test_graph = self.clone();

        // Replace the node_types with the one computes above
        train_graph.node_types = Arc::new(NodeTypeVocabulary::from_option_structs(
            Some(train_node_types),
            self.node_types
                .as_ref()
                .as_ref()
                .map(|ntv| ntv.vocabulary.clone()),
        ));
        test_graph.node_types = Arc::new(NodeTypeVocabulary::from_option_structs(
            Some(test_node_types),
            self.node_types
                .as_ref()
                .as_ref()
                .map(|ntv| ntv.vocabulary.clone()),
        ));

        Ok((train_graph, test_graph))
    }

    /// Returns edge-label holdout for training ML algorithms on the graph edge labels.
    /// This is commonly used for edge type prediction tasks.
    ///
    /// This method returns two graphs, the train and the test one.
    /// The edges of the graph will be splitted in the train and test graphs according
    /// to the `train_size` argument.
    ///
    /// If stratification is enabled, the train and test will have the same ratios of
    /// edge types.
    ///
    /// # Arguments
    /// * `train_size`: f64 - rate target to reserve for training,
    /// * `use_stratification`: Option<bool> - Whether to use edge-label stratification,
    /// * `random_state`: Option<EdgeT> - The random_state to use for the holdout,
    ///
    /// # Example
    /// This example creates an 80-20 split of the edges mantaining the edge label ratios
    /// in train and test.
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    ///   let (train, test) = graph.get_edge_label_holdout_graphs(0.8, Some(true), None).unwrap();
    /// ```
    ///
    /// # Raises
    /// * If the graph is a multigraph.
    /// * If the graph does not have edge types.
    /// * If stratification is required but the graph has singleton edge types.
    pub fn get_edge_label_holdout_graphs(
        &self,
        train_size: f64,
        use_stratification: Option<bool>,
        random_state: Option<EdgeT>,
    ) -> Result<(Graph, Graph)> {
        self.must_not_be_multigraph()?;
        if self.get_number_of_known_edge_types()? < 2 {
            return Err("It is not possible to create a edge label holdout when the number of edges with known edge type is less than two.".to_string());
        }
        let use_stratification = use_stratification.unwrap_or(false);
        let random_state = random_state.unwrap_or(0xbadf00d);
        if use_stratification && self.has_singleton_edge_types()? {
            return Err("It is impossible to create a stratified holdout when the graph has edge types with cardinality one.".to_string());
        }

        // Compute the vectors with the indices of the edges which edge type matches
        // therefore the expected shape is:
        // (number_of_edge_types, number of edges of that edge type)
        let edge_sets: Vec<Vec<EdgeT>> = self
            .edge_types
            .as_ref()
            .as_ref()
            .map(|ets| {
                if use_stratification {
                    // Initialize the vectors for each edge type
                    let mut edge_sets: Vec<Vec<EdgeT>> =
                        vec![Vec::new(); self.get_number_of_edge_types().unwrap() as usize];
                    // itering over the indices and adding each edge to the
                    // vector of the corresponding edge type.
                    ets.ids
                        .iter()
                        .zip(self.iter_directed_edge_node_ids())
                        .for_each(|(edge_type, (edge_id, src, dst))| {
                            // If this is an undirected graph and the source is
                            // greater than the destination, i.e. this is an edge
                            // from the lower triangular matrix, we skip it so to
                            // avoid leaking information from the test set to the
                            // training set.
                            if !self.is_directed() && src > dst {
                                return;
                            }
                            // if the edge has a edge_type
                            if let Some(et) = edge_type {
                                // Get the index of the correct edge type vector.
                                edge_sets[*et as usize].push(edge_id as EdgeT);
                            };
                        });

                    Ok::<Vec<_>, String>(edge_sets)
                } else {
                    // just compute a vector with a single vector of the indices
                    // of the edges with edge types
                    Ok(vec![if self.is_directed() {
                        self.get_directed_edge_ids_with_known_edge_types()?
                    } else {
                        self.get_upper_triangular_edge_ids_with_known_edge_types()?
                    }])
                }
            })
            .transpose()?
            .unwrap();

        // initialize the seed for a re-producible shuffle
        let mut rnd = SmallRng::seed_from_u64(splitmix64(random_state as u64));

        // Allocate the vectors for the edges of each
        let mut train_edge_types = vec![None; self.get_number_of_directed_edges() as usize];
        let mut test_edge_types = vec![None; self.get_number_of_directed_edges() as usize];

        let train_edge_types_shared = SyncUnsafeCell::new(&mut train_edge_types);
        let test_edge_types_shared = SyncUnsafeCell::new(&mut test_edge_types);

        for mut edge_set in edge_sets {
            // Shuffle in a reproducible way the edges of the current edge_type
            edge_set.shuffle(&mut rnd);
            // Compute how many of these edges belongs to the training set
            let (train_size, _) = self.get_holdouts_elements_number(train_size, edge_set.len())?;

            let (train, test) = edge_set.split_at(train_size);

            // add the edges to the relative vectors
            train.par_iter().for_each(|edge_id| unsafe {
                (*train_edge_types_shared.get())[*edge_id as usize] =
                    self.get_unchecked_edge_type_id_from_edge_id(*edge_id);
                // If this is an undirected graph we also need to add the
                // corresponding edge from the lower triangular matrix.
                if !self.is_directed() {
                    let (src, dst) = self.get_unchecked_node_ids_from_edge_id(*edge_id);
                    let lower_triangular_edge_id =
                        self.get_unchecked_edge_id_from_node_ids(dst, src);
                    (*train_edge_types_shared.get())[lower_triangular_edge_id as usize] =
                        self.get_unchecked_edge_type_id_from_edge_id(lower_triangular_edge_id);
                }
            });
            test.par_iter().for_each(|edge_id| unsafe {
                (*test_edge_types_shared.get())[*edge_id as usize] =
                    self.get_unchecked_edge_type_id_from_edge_id(*edge_id);
                // If this is an undirected graph we also need to add the
                // corresponding edge from the lower triangular matrix.
                if !self.is_directed() {
                    let (src, dst) = self.get_unchecked_node_ids_from_edge_id(*edge_id);
                    let lower_triangular_edge_id =
                        self.get_unchecked_edge_id_from_node_ids(dst, src);
                    (*train_edge_types_shared.get())[lower_triangular_edge_id as usize] =
                        self.get_unchecked_edge_type_id_from_edge_id(lower_triangular_edge_id);
                }
            });
        }

        // Clone the current graph
        // here we could manually initialize the clones so that we don't waste
        // time and memory cloning the edge_types which will be immediately
        // overwrite. We argue that this should not be impactfull so we prefer
        // to prioritze the simplicity of the code
        let mut train_graph = self.clone();
        let mut test_graph = self.clone();

        // Replace the edge_types with the one computes above
        train_graph.edge_types = Arc::new(Some(EdgeTypeVocabulary::from_structs(
            train_edge_types,
            self.edge_types
                .as_ref()
                .as_ref()
                .map(|etv| etv.vocabulary.clone())
                .unwrap(),
        )));
        test_graph.edge_types = Arc::new(Some(EdgeTypeVocabulary::from_structs(
            test_edge_types,
            self.edge_types
                .as_ref()
                .as_ref()
                .map(|etv| etv.vocabulary.clone())
                .unwrap(),
        )));

        Ok((train_graph, test_graph))
    }

    /// Returns subgraph with given number of nodes.
    ///
    /// **This method creates a subset of the graph starting from a random node
    /// sampled using given random_state and includes all neighbouring nodes until
    /// the required number of nodes is reached**. All the edges connecting any
    /// of the selected nodes are then inserted into this graph.
    ///
    /// This is meant to execute distributed node embeddings.
    /// It may also sample singleton nodes.
    ///
    /// # Arguments
    /// * `number_of_nodes`: NodeT - Number of nodes to extract.
    /// * `random_state`: Option<usize> - Random random_state to use.
    /// * `verbose`: Option<bool> - Whether to show the loading bar.
    ///
    /// # Example
    /// this generates a random subgraph with 1000 nodes.
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    ///   let random_graph = graph.get_random_subgraph(1000, Some(0xbad5eed), Some(true)).unwrap();
    /// ```
    ///
    /// # Raises
    /// * If the requested number of nodes is one or less.
    /// * If the graph has less than the requested number of nodes.
    pub fn get_random_subgraph(
        &self,
        number_of_nodes: NodeT,
        random_state: Option<usize>,
        verbose: Option<bool>,
    ) -> Result<Graph> {
        if number_of_nodes <= 1 {
            return Err(String::from("Required nodes number must be more than 1."));
        }
        let verbose = verbose.unwrap_or(false);
        let random_state = random_state.unwrap_or(0xbadf00d);
        let connected_number_of_nodes = self.get_number_of_connected_nodes();
        if number_of_nodes > connected_number_of_nodes {
            return Err(format!(
                concat!(
                    "Required number of nodes ({}) is more than available ",
                    "number of nodes ({}) that have edges in current graph."
                ),
                number_of_nodes, connected_number_of_nodes
            ));
        }

        // Creating the loading bars
        let pb1 = get_loading_bar(verbose, "Sampling nodes subset", number_of_nodes as usize);
        let pb2 = get_loading_bar(
            verbose,
            "Computing subgraph edges",
            self.get_number_of_directed_edges() as usize,
        );

        // Creating the random number generator
        let mut rnd = SmallRng::seed_from_u64(splitmix64(random_state as u64) as u64);

        // Nodes indices
        let mut nodes: Vec<NodeT> = (0..self.get_number_of_nodes()).collect();

        // Shuffling the components using the given random_state.
        nodes.shuffle(&mut rnd);

        // Initializing stack and set of nodes
        let mut unique_nodes = RoaringBitmap::new();
        let mut stack: Vec<NodeT> = Vec::new();

        // We iterate on the components
        'outer: for node in nodes.iter() {
            // If the current node is a trap there is no need to continue with the current loop.
            if self.is_trap_node_from_node_id(*node).unwrap() {
                continue;
            }
            stack.push(*node);
            while !stack.is_empty() {
                let src = stack.pop().unwrap();
                for dst in
                    unsafe { self.iter_unchecked_neighbour_node_ids_from_source_node_id(src) }
                {
                    if !unique_nodes.contains(dst) && src != dst {
                        stack.push(dst);
                    }

                    unique_nodes.insert(*node);
                    unique_nodes.insert(dst);
                    pb1.inc(2);

                    // If we reach the desired number of unique nodes we can stop the iteration.
                    if unique_nodes.len() as NodeT >= number_of_nodes {
                        break 'outer;
                    }
                }
            }
        }

        pb1.finish();

        let selected_edge_ids = self
            .par_iter_directed_edge_node_ids_and_edge_type_id_and_edge_weight()
            .progress_with(pb2)
            .filter(|&(_, src, dst, _, _)| unique_nodes.contains(src) && unique_nodes.contains(dst))
            .map(|(edge_id, _, _, _, _)| edge_id)
            .collect::<Vec<_>>();

        let selected_number_of_edges = selected_edge_ids.len() as EdgeT;

        let pb3 = get_loading_bar(verbose, "Building subgraph", selected_edge_ids.len());

        build_graph_from_integers(
            Some(
                selected_edge_ids
                    .into_par_iter()
                    .enumerate()
                    .map(|(i, edge_id)| unsafe {
                        let (src, dst, edge_type, weight) = self
                            .get_unchecked_node_ids_and_edge_type_id_and_edge_weight_from_edge_id(
                                edge_id,
                            );
                        (i, (src, dst, edge_type, weight.unwrap_or(WeightT::NAN)))
                    })
                    .progress_with(pb3),
            ),
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types
                .as_ref()
                .as_ref()
                .map(|ets| ets.vocabulary.clone()),
            self.has_edge_weights(),
            self.is_directed(),
            Some(true),
            Some(false),
            Some(true),
            Some(selected_number_of_edges),
            true,
            self.has_selfloops(),
            format!("{} subgraph", self.get_name()),
        )
    }

    /// Returns node-label holdout for training ML algorithms on the graph node labels.
    ///
    /// # Arguments
    /// * `train_size`: f64 - rate target to reserve for training,
    /// * `use_stratification`: Option<bool> - Whether to use node-label stratification,
    /// * `random_state`: Option<EdgeT> - The random_state to use for the holdout,
    ///
    /// # Example
    /// This example create an 80-20 split of the nodes in the graph
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    ///   let (train, test) = graph.get_node_label_random_holdout(0.8, Some(true), None).unwrap();
    /// ```
    ///
    /// # Raises
    /// * If the graph does not have node types.
    /// * If stratification is requested but the graph has a single node type.
    /// * If stratification is requested but the graph has a multilabel node types.
    pub fn get_node_label_random_holdout(
        &self,
        train_size: f64,
        use_stratification: Option<bool>,
        random_state: Option<EdgeT>,
    ) -> Result<(Graph, Graph)> {
        self.must_have_node_types()?;
        let random_state = random_state.unwrap_or(0xbadf00d);
        let use_stratification = use_stratification.unwrap_or(false);
        if use_stratification {
            if self.has_multilabel_node_types()? {
                return Err("It is impossible to create a stratified holdout when the graph has multi-label node types.".to_string());
            }
            if self.has_singleton_node_types()? {
                return Err("It is impossible to create a stratified holdout when the graph has node types with cardinality one.".to_string());
            }
        }

        if self.get_number_of_known_node_types()? < 2 {
            return Err("It is not possible to create a node label holdout when the number of nodes with known node type is less than two.".to_string());
        }

        // Compute the vectors with the indices of the nodes which node type matches
        // therefore the expected shape is:
        // (number_of_node_types, number of nodes of that node type)
        let node_sets: Vec<Vec<NodeT>> = self
            .node_types
            .as_ref()
            .as_ref()
            .map(|nts| {
                if use_stratification {
                    // Initialize the vectors for each node type
                    let mut node_sets: Vec<Vec<NodeT>> =
                        vec![Vec::new(); self.get_number_of_node_types().unwrap() as usize];
                    // itering over the indices and adding each node to the
                    // vector of the corresponding node type.
                    nts.ids.iter().enumerate().for_each(|(node_id, node_type)| {
                        // if the node has a node_type
                        if let Some(nt) = node_type {
                            // Get the index of the correct node type vector.
                            node_sets[nt[0] as usize].push(node_id as NodeT);
                        };
                    });

                    node_sets
                } else {
                    // just compute a vector with a single vector of the indices
                    //  of the nodes with node
                    vec![nts
                        .ids
                        .iter()
                        .enumerate()
                        .filter_map(|(node_id, node_type)| {
                            node_type.as_ref().map(|_| node_id as NodeT)
                        })
                        .collect()]
                }
            })
            .unwrap();

        // initialize the seed for a re-producible shuffle
        let mut rnd = SmallRng::seed_from_u64(splitmix64(random_state as u64));

        // Allocate the vectors for the nodes of each
        let mut train_node_types: Vec<Option<Vec<NodeTypeT>>> =
            vec![None; self.get_number_of_nodes() as usize];
        let mut test_node_types: Vec<Option<Vec<NodeTypeT>>> =
            vec![None; self.get_number_of_nodes() as usize];

        for mut node_set in node_sets {
            // Shuffle in a reproducible way the nodes of the current node_type
            node_set.shuffle(&mut rnd);
            // Compute how many of these nodes belongs to the training set
            let (train_size, _) = self.get_holdouts_elements_number(train_size, node_set.len())?;
            // add the nodes to the relative vectors
            node_set[..train_size].iter().for_each(|node_id| unsafe {
                train_node_types[*node_id as usize] = self
                    .get_unchecked_node_type_ids_from_node_id(*node_id)
                    .map(|x| x.to_vec())
            });
            node_set[train_size..].iter().for_each(|node_id| unsafe {
                test_node_types[*node_id as usize] = self
                    .get_unchecked_node_type_ids_from_node_id(*node_id)
                    .map(|x| x.to_vec())
            });
        }

        // Clone the current graph
        // here we could manually initialize the clones so that we don't waste
        // time and memory cloning the node_types which will be immediately
        // overwrite. We argue that this should not be impactfull so we prefer
        // to prioritze the simplicity of the code
        let mut train_graph = self.clone();
        let mut test_graph = self.clone();

        // Replace the node_types with the one computes above
        train_graph.node_types = Arc::new(NodeTypeVocabulary::from_option_structs(
            Some(train_node_types),
            self.node_types
                .as_ref()
                .as_ref()
                .map(|ntv| ntv.vocabulary.clone()),
        ));
        test_graph.node_types = Arc::new(NodeTypeVocabulary::from_option_structs(
            Some(test_node_types),
            self.node_types
                .as_ref()
                .as_ref()
                .map(|ntv| ntv.vocabulary.clone()),
        ));

        Ok((train_graph, test_graph))
    }

    /// Returns node-label fold for training ML algorithms on the graph node labels.
    ///
    /// # Arguments
    /// * `k`: usize - The number of folds.
    /// * `k_index`: usize - Which fold to use for the validation.
    /// * `use_stratification`: Option<bool> - Whether to use node-label stratification,
    /// * `random_state`: Option<EdgeT> - The random_state to use for the holdout,
    ///
    /// # Example
    /// This example create an 80-20 split of the nodes in the graph
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    ///   let (train, test) = graph.get_node_label_random_holdout(0.8, Some(true), None).unwrap();
    /// ```
    ///
    /// # Raises
    /// * If the graph does not have node types.
    /// * If stratification is requested but the graph has a single node type.
    /// * If stratification is requested but the graph has a multilabel node types.
    pub fn get_node_label_kfold(
        &self,
        k: usize,
        k_index: usize,
        use_stratification: Option<bool>,
        random_state: Option<EdgeT>,
    ) -> Result<(Graph, Graph)> {
        self.must_have_node_types()?;
        let mut random_state = random_state.unwrap_or(0xbadf00d);
        let use_stratification = use_stratification.unwrap_or(false);
        if use_stratification {
            if self.has_multilabel_node_types()? {
                return Err("It is impossible to create a stratified holdout when the graph has multi-label node types.".to_string());
            }
            if self.has_singleton_node_types()? {
                return Err("It is impossible to create a stratified holdout when the graph has node types with cardinality one.".to_string());
            }
        }

        if self.get_number_of_known_node_types()? < 2 {
            return Err("It is not possible to create a node label holdout when the number of nodes with known node type is less than two.".to_string());
        }

        // Compute the vectors with the indices of the nodes which node type matches
        // therefore the expected shape is:
        // (number_of_node_types, number of nodes of that node type)
        let node_sets: Vec<Vec<NodeT>> = self
            .node_types
            .as_ref()
            .as_ref()
            .map(|nts| {
                if use_stratification {
                    // Initialize the vectors for each node type
                    let mut node_sets: Vec<Vec<NodeT>> =
                        vec![Vec::new(); self.get_number_of_node_types().unwrap() as usize];
                    // itering over the indices and adding each node to the
                    // vector of the corresponding node type.
                    nts.get_ids()
                        .iter()
                        .enumerate()
                        .for_each(|(node_id, node_type)| {
                            // if the node has a node_type
                            if let Some(nt) = node_type {
                                // Get the index of the correct node type vector.
                                node_sets[nt[0] as usize].push(node_id as _);
                            };
                        });

                    node_sets
                } else {
                    // just compute a vector with a single vector of the indices
                    //  of the nodes with node
                    vec![nts
                        .ids
                        .iter()
                        .enumerate()
                        .filter_map(|(node_id, node_type)| {
                            node_type.as_ref().map(|_| node_id as NodeT)
                        })
                        .collect()]
                }
            })
            .unwrap();

        // Allocate the vectors for the nodes of each
        let mut train_node_types: Vec<Option<Vec<NodeTypeT>>> = self
            .node_types
            .as_ref()
            .as_ref()
            .map(|nts| nts.get_ids().to_vec())
            .unwrap();

        let mut test_node_types: Vec<Option<Vec<NodeTypeT>>> =
            vec![None; self.get_number_of_nodes() as usize];

        for mut node_set in node_sets {
            random_state = splitmix64(random_state);
            // Shuffle in a reproducible way the nodes of the current node_type
            let validation_chunk = kfold(k, k_index, &mut node_set, random_state)?;
            // Iterate of node ids
            for test_node_id in validation_chunk {
                let node_type =
                    unsafe { self.get_unchecked_node_type_ids_from_node_id(*test_node_id) };
                if validation_chunk.contains(test_node_id) {
                    test_node_types[*test_node_id as usize] = node_type.map(|x| x.to_vec());
                    train_node_types[*test_node_id as usize] = None;
                }
            }
        }

        // Clone the current graph
        // here we could manually initialize the clones so that we don't waste
        // time and memory cloning the node_types which will be immediately
        // overwrite. We argue that this should not be impactfull so we prefer
        // to prioritze the simplicity of the code
        let mut train_graph = self.clone();
        let mut test_graph = self.clone();

        // Replace the node_types with the one computes above
        train_graph.node_types = Arc::new(NodeTypeVocabulary::from_option_structs(
            Some(train_node_types),
            self.node_types
                .as_ref()
                .as_ref()
                .map(|ntv| ntv.vocabulary.clone()),
        ));
        test_graph.node_types = Arc::new(NodeTypeVocabulary::from_option_structs(
            Some(test_node_types),
            self.node_types
                .as_ref()
                .as_ref()
                .map(|ntv| ntv.vocabulary.clone()),
        ));

        Ok((train_graph, test_graph))
    }

    /// Returns edge-label kfold for training ML algorithms on the graph edge labels.
    /// This is commonly used for edge type prediction tasks.
    ///
    /// This method returns two graphs, the train and the test one.
    /// The edges of the graph will be splitted in the train and test graphs according
    /// to the `train_size` argument.
    ///
    /// If stratification is enabled, the train and test will have the same ratios of
    /// edge types.
    ///
    /// # Arguments
    /// * `k`: usize - The number of folds.
    /// * `k_index`: usize - Which fold to use for the validation.
    /// * `use_stratification`: Option<bool> - Whether to use edge-label stratification,
    /// * `random_state`: Option<EdgeT> - The random_state to use for the holdout,
    ///
    /// # Example
    /// This example creates an 80-20 split of the edges mantaining the edge label ratios
    /// in train and test.
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    ///   let (train, test) = graph.get_edge_label_random_holdout(0.8, Some(true), None).unwrap();
    /// ```
    ///
    /// # Raises
    /// * If the graph does not have edge types.
    /// * If stratification is required but the graph has singleton edge types.
    pub fn get_edge_label_kfold(
        &self,
        k: usize,
        k_index: usize,
        use_stratification: Option<bool>,
        random_state: Option<EdgeT>,
    ) -> Result<(Graph, Graph)> {
        self.must_not_be_multigraph()?;
        if self.get_number_of_known_edge_types()? < 2 {
            return Err("It is not possible to create a edge label holdout when the number of edges with known edge type is less than two.".to_string());
        }
        let use_stratification = use_stratification.unwrap_or(false);
        let random_state = splitmix64(random_state.unwrap_or(0xbadf00d));
        if use_stratification && self.has_singleton_edge_types()? {
            return Err("It is impossible to create a stratified holdout when the graph has edge types with cardinality one.".to_string());
        }

        // Compute the vectors with the indices of the edges which edge type matches
        // therefore the expected shape is:
        // (number_of_edge_types, number of edges of that edge type)
        let edge_sets: Vec<Vec<EdgeT>> = self
            .edge_types
            .as_ref()
            .as_ref()
            .map(|ets| {
                if use_stratification {
                    // Initialize the vectors for each edge type
                    let mut edge_sets: Vec<Vec<EdgeT>> =
                        vec![Vec::new(); self.get_number_of_edge_types().unwrap() as usize];
                    // itering over the indices and adding each edge to the
                    // vector of the corresponding edge type.
                    ets.ids
                        .iter()
                        .zip(self.iter_directed_edge_node_ids())
                        .for_each(|(edge_type, (edge_id, src, dst))| {
                            // If this is an undirected graph and the source is
                            // greater than the destination, i.e. this is an edge
                            // from the lower triangular matrix, we skip it so to
                            // avoid leaking information from the test set to the
                            // training set.
                            if !self.is_directed() && src > dst {
                                return;
                            }
                            // if the edge has a edge_type
                            if let Some(et) = edge_type {
                                // Get the index of the correct edge type vector.
                                edge_sets[*et as usize].push(edge_id as EdgeT);
                            };
                        });

                    Ok::<Vec<_>, String>(edge_sets)
                } else {
                    // just compute a vector with a single vector of the indices
                    // of the edges with edge types
                    Ok(vec![if self.is_directed() {
                        self.get_directed_edge_ids_with_known_edge_types()?
                    } else {
                        self.get_upper_triangular_edge_ids_with_known_edge_types()?
                    }])
                }
            })
            .transpose()?
            .unwrap();

        // Allocate the vectors for the edges of each
        let mut train_edge_types = self
            .edge_types
            .as_ref()
            .as_ref()
            .map(|ets| ets.get_ids().to_vec())
            .unwrap();
        let mut test_edge_types = vec![None; self.get_number_of_directed_edges() as usize];

        let train_edge_types_shared = SyncUnsafeCell::new(&mut train_edge_types);
        let test_edge_types_shared = SyncUnsafeCell::new(&mut test_edge_types);

        edge_sets
            .into_par_iter()
            .enumerate()
            .map(|(i, mut edge_set)| {
                let random_state = splitmix64(random_state.wrapping_mul(i as u64 + 1));
                // Shuffle in a reproducible way the edges of the current edge_type
                let validation_chunk = kfold(k, k_index, &mut edge_set, random_state)?;
                // Iterate of edge ids
                for edge_id in validation_chunk {
                    let edge_type =
                        unsafe { self.get_unchecked_edge_type_id_from_edge_id(*edge_id) };
                    if validation_chunk.contains(edge_id) {
                        unsafe {
                            (*test_edge_types_shared.get())[*edge_id as usize] = edge_type;
                            (*train_edge_types_shared.get())[*edge_id as usize] = None;

                            // If this is an undirected graph we also need to add the
                            // corresponding edge from the lower triangular matrix.
                            if !self.is_directed() {
                                let (src, dst) = self.get_unchecked_node_ids_from_edge_id(*edge_id);
                                let lower_triangular_edge_id =
                                    self.get_unchecked_edge_id_from_node_ids(dst, src);
                                (*train_edge_types_shared.get())
                                    [lower_triangular_edge_id as usize] = self
                                    .get_unchecked_edge_type_id_from_edge_id(
                                        lower_triangular_edge_id,
                                    );
                            }
                        }
                    }
                }
                Ok(())
            })
            .collect::<Result<()>>()?;

        // Clone the current graph
        // here we could manually initialize the clones so that we don't waste
        // time and memory cloning the edge_types which will be immediately
        // overwrite. We argue that this should not be impactfull so we prefer
        // to prioritze the simplicity of the code
        let mut train_graph = self.clone();
        let mut test_graph = self.clone();

        // Replace the edge_types with the one computes above
        train_graph.edge_types = Arc::new(Some(EdgeTypeVocabulary::from_structs(
            train_edge_types,
            self.edge_types
                .as_ref()
                .as_ref()
                .map(|etv| etv.vocabulary.clone())
                .unwrap(),
        )));
        test_graph.edge_types = Arc::new(Some(EdgeTypeVocabulary::from_structs(
            test_edge_types,
            self.edge_types
                .as_ref()
                .as_ref()
                .map(|etv| etv.vocabulary.clone())
                .unwrap(),
        )));

        Ok((train_graph, test_graph))
    }

    /// Returns train and test graph following kfold validation scheme.
    ///
    /// The edges are splitted into k chunks. The k_index-th chunk is used to build
    /// the validation graph, all the other edges create the training graph.
    ///
    /// # Arguments
    /// * `k`: EdgeT - The number of folds.
    /// * `k_index`: u64 - Which fold to use for the validation.
    /// * `edge_types`: Option<&[Option<&str>]> - Edge types to be selected when computing the folds (All the edge types not listed here will be always be used in the training set).
    /// * `random_state`: Option<EdgeT> - The random_state (seed) to use for the holdout,
    /// * `verbose`: Option<bool> - Whether to show the loading bar.
    ///
    /// # Example
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// for i in 0..5 {
    ///     let (train, test) = graph.get_edge_prediction_kfold(5, i, None, Some(0xbad5eed), None).unwrap();
    ///     // Run the training
    /// }
    /// ```
    /// If We pass a vector of edge types, the K-fold will be executed only on the edges which match
    /// that type. All the other edges will always appear in the traning set.
    ///
    /// # Raises
    /// * If the number of requested k folds is one or zero.
    /// * If the given k fold index is greater than the number of k folds.
    /// * If edge types have been specified but it's an empty list.
    /// * If the number of k folds is higher than the number of edges in the graph.
    pub fn get_edge_prediction_kfold(
        &self,
        k: usize,
        k_index: usize,
        edge_types: Option<&[Option<&str>]>,
        random_state: Option<EdgeT>,
        verbose: Option<bool>,
    ) -> Result<(Graph, Graph)> {
        let random_state = random_state.unwrap_or(0xbadf00d);

        // If edge types is not None, to compute the chunks only use the edges
        // of the chosen edge_types
        let mut indices = if let Some(ets) = edge_types {
            if ets.is_empty() {
                return Err(String::from(
                    "Required edge types must be a non-empty list.",
                ));
            }

            let edge_type_ids = self
                .get_edge_type_ids_from_edge_type_names(ets)?
                .into_iter()
                .collect::<HashSet<Option<EdgeTypeT>>>();

            self.iter_edge_node_ids_and_edge_type_id(self.directed)
                .filter_map(|(edge_id, _, _, edge_type)| {
                    if !edge_type_ids.contains(&edge_type) {
                        return None;
                    }
                    Some(edge_id)
                })
                .collect::<Vec<EdgeT>>()
        } else {
            self.iter_edge_node_ids(self.directed)
                .map(|(edge_id, _, _)| edge_id)
                .collect::<Vec<EdgeT>>()
        };

        let chunk = kfold(k, k_index, &mut indices, random_state)?;

        // Create the two graphs
        self.get_edge_holdout(
            Some(random_state),
            chunk.len() as u64,
            false,
            |edge_id, _, _, _| chunk.contains(&edge_id),
            verbose,
        )
    }
}
