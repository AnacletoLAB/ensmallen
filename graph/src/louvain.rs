use super::*;
use log::info;
use num_traits::Zero;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

impl Graph {
    #[no_numpy_binding]
    /// Returns vector of vectors of communities for each layer of hierarchy.
    ///
    /// # Implementative details
    /// The following implementative choices have been made while developing this implementation
    /// of the Louvian algorithm:
    ///
    /// ## Structure of the result
    /// The result of this algorithm is a vector of vectors of integers.
    /// In any given i-th vector, there are the ids of the communities for the i-th layer.
    /// For instance, at the first layer, you have a vector of length `number of nodes` and
    /// dense numeric IDs which vary from 0 to the maximum number of communities identified
    /// at the first layer, `first layer communities number`. The second vector has as length
    /// the number of communities identified at the previous layer and as values numeric IDs
    /// varying from 0 to the maximum number of communities identified at the second
    /// layer, and so on and so forth.
    ///
    /// # Arguments
    /// * `recursion_minimum_improvement`: Option<f64> - The minimum improvement to warrant another resursion round. By default, zero.
    /// * `first_phase_minimum_improvement`: Option<f64> - The minimum improvement to warrant another first phase iteration. By default, zero.
    /// * `default_weight`: Option<WeightT> - The default weight to use if the graph is not weighted. By default, one.
    ///
    /// # Raises
    /// * If the `default_weight` has been provided but the graph is already weighted.
    /// * If the `default_weight` has an invalid value, i.e. zero, NaN or infinity.
    /// * If the `recursion_minimum_improvement` has an invalid value, i.e. NaN or infinity.
    /// * If the `first_phase_minimum_improvement` has an invalid value, i.e. NaN or infinity.
    ///
    /// # References
    /// [Blondel et al paper](https://iopscience.iop.org/article/10.1088/1742-5468/2008/10/P10008/pdf?casa_token=YoBiFS-4w5EAAAAA:BaHtIrzOvzMsQol_XR7wFGqZWun5_GDn2O86KU9x5bVUN859DGred8dgV7iqxKmjrLOCTR62uccXUQ)
    pub fn louvain_community_detection(
        &self,
        recursion_minimum_improvement: Option<f64>,
        first_phase_minimum_improvement: Option<f64>,
        default_weight: Option<WeightT>,
    ) -> Result<Vec<Vec<NodeT>>> {
        if default_weight.is_some() && self.has_edge_weights() {
            return Err(concat!(
                "It does not make sense to provide the default weight when ",
                "the graph is already weighted."
            )
            .to_string());
        }
        let mut communities: Vec<NodeT> = self.get_node_ids();
        let recursion_minimum_improvement: f64 = recursion_minimum_improvement.unwrap_or(0.0);
        let first_phase_minimum_improvement: f64 = first_phase_minimum_improvement.unwrap_or(0.0);
        let default_weight: WeightT = default_weight.unwrap_or(1.0);
        if recursion_minimum_improvement.is_nan() || recursion_minimum_improvement.is_infinite() {
            return Err(concat!(
                "The provided parameter `recursion_minimum_improvement` is an illegal value, i.e. ",
                "either NaN or infinity."
            )
            .to_string());
        }
        if first_phase_minimum_improvement.is_nan() || first_phase_minimum_improvement.is_infinite()
        {
            return Err(concat!(
                "The provided parameter `first_phase_minimum_improvement` is an illegal value, i.e. ",
                "either NaN or infinity."
            ).to_string());
        }
        if default_weight.is_zero() || default_weight.is_nan() || default_weight.is_infinite() {
            return Err(concat!(
                "The provided parameter `default_weight` is an illegal value, i.e. ",
                "either zero, NaN or infinity."
            )
            .to_string());
        }
        // Vector of the weights of the edges contained within each community.
        let mut communities_weights: Vec<f64> = vec![0.0; self.get_nodes_number() as usize];
        // Vector of the weighted indegrees of the communities, which when the procedure begins
        // is equal to the weighted indegree.
        let node_indegrees: Vec<f64> = self.get_weighted_node_indegrees().unwrap_or_else(|_| {
            self.get_node_indegrees()
                .into_iter()
                .map(|indegree| indegree as f64)
                .collect::<Vec<_>>()
        });
        let mut communities_indegrees = node_indegrees.clone();
        // Total edge weights, i.e. the weights of all the edges in the graph.
        let total_edge_weights: f64 = self
            .get_total_edge_weights()
            // If the graph does not start as a weighted graph, we use the default weight
            // that was provided by the user.
            .unwrap_or_else(|_| {
                (default_weight as f64) * (self.get_directed_edges_number() as f64)
            });

        // Define method to compute in stream the total edge weight from
        // a given node to a given node community
        let get_node_to_community_weighted_outdegree = if self.has_edge_weights() {
            |graph: &Graph, node_id: NodeT, community_id: NodeT, communities: &[NodeT]| unsafe {
                graph
                    .iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                    .zip(graph.iter_unchecked_edge_weights_from_source_node_id(node_id))
                    .filter_map(|(dst, weight)| {
                        if communities[dst as usize] == community_id {
                            Some(weight as f64)
                        } else {
                            None
                        }
                    })
                    .sum::<f64>()
            }
        } else {
            |graph: &Graph, node_id: NodeT, community_id: NodeT, communities: &[NodeT]| unsafe {
                graph
                    .iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                    .filter(|&dst| communities[dst as usize] == community_id)
                    .count() as f64
            }
        };
        // Define method to compute in stream the total weighted indegree
        // of a given node, excluding edges coming from a given community
        let get_node_to_community_weighted_indegree = if self.has_edge_weights() {
            |graph: &Graph, node_id: NodeT, community_id: NodeT, communities: &[NodeT]| unsafe {
                graph
                    .iter_unchecked_neighbour_node_ids_from_destination_node_id(node_id)
                    .zip(graph.iter_unchecked_edge_weights_from_destination_node_id(node_id))
                    .filter_map(|(dst, weight)| {
                        if communities[dst as usize] == community_id {
                            Some(weight as f64)
                        } else {
                            None
                        }
                    })
                    .sum::<f64>()
            }
        } else {
            |graph: &Graph, node_id: NodeT, community_id: NodeT, communities: &[NodeT]| unsafe {
                graph
                    .iter_unchecked_neighbour_node_ids_from_destination_node_id(node_id)
                    .filter(|&dst| communities[dst as usize] == community_id)
                    .count() as f64
            }
        };
        // The overall recursion is regulated by the total
        // change of modularity of the iteration.
        let mut total_modularity_change: f64 = 0.0;
        info!("Started Louvian phase one loop.");
        let mut loops_number: usize = 0;
        // Execute the first phase until convergence
        loop {
            info!("Started Louvian phase one loop #{}.", loops_number);
            loops_number += 1;
            let mut total_change_per_iter: f64 = 0.0;
            self.iter_node_ids().for_each(|src| {
                // We get the best neighbour.
                let node_community = communities[src as usize];
                let result =
                    unsafe { self.iter_unchecked_neighbour_node_ids_from_source_node_id(src) }
                        .map(|dst| communities[dst as usize])
                        .filter(|&neighbour_community_id| node_community != neighbour_community_id)
                        .map(|neighbour_community_id| {
                            let node_to_community_weighted_degree =
                                get_node_to_community_weighted_outdegree(
                                    self,
                                    src,
                                    neighbour_community_id,
                                    &communities,
                                );
                            let modularity_variation = total_edge_weights
                                * node_to_community_weighted_degree
                                - node_indegrees[src as usize]
                                    * communities_indegrees[neighbour_community_id as usize];
                            (
                                neighbour_community_id,
                                node_to_community_weighted_degree,
                                modularity_variation,
                            )
                        })
                        .max_by(
                            |(_, _, modularity_variation1), (_, _, modularity_variation2)| {
                                // These values cannot ever be NaNs, so this comparison
                                // can be unwrapped without the worry of causing a panic.
                                modularity_variation1
                                    .partial_cmp(modularity_variation2)
                                    .unwrap()
                            },
                        );

                if let Some((
                    neighbour_community_id,
                    node_to_community_weighted_degree,
                    modularity_variation,
                )) = result
                {
                    // If this is improving the current bound
                    if modularity_variation > 0.0 {
                        // Assign to the community of the best neighbor
                        // since this improves modularity
                        total_change_per_iter += modularity_variation;
                        // Add the total edge weight from the considered source node
                        // to the new community.
                        communities_weights[neighbour_community_id as usize] +=
                            node_to_community_weighted_degree;
                        // Remove the edge weights from this node to the the previous community.
                        communities_weights[node_community as usize] -=
                            get_node_to_community_weighted_outdegree(
                                self,
                                src,
                                node_community,
                                &communities,
                            );
                        // Update the indegree of the community, that is
                        // we need to remove the edge weights added with this new node
                        // and add the indegree of the node
                        communities_indegrees[neighbour_community_id as usize] -=
                            node_to_community_weighted_degree;
                        // Removing the eventual edge weights
                        // of the edges inbound to this node that are already part of this community.
                        communities_indegrees[neighbour_community_id as usize] +=
                            get_node_to_community_weighted_indegree(
                                self,
                                src,
                                node_community,
                                &communities,
                            );
                        // We update the community of this node.
                        communities[src as usize] = neighbour_community_id;
                    }
                }
            });

            total_modularity_change += total_change_per_iter;
            info!(
                "The modularity change in loop #{} is {}.",
                loops_number, total_change_per_iter
            );
            if total_change_per_iter < first_phase_minimum_improvement {
                break;
            }
        }

        info!(
            "The total modularity change is {}.",
            total_modularity_change
        );
        if total_modularity_change <= recursion_minimum_improvement {
            return Ok(vec![]);
        }

        info!("Started remapping of communities to dense range.");
        // Compactify the communities node IDs.
        let mut communities_remapping = vec![NOT_PRESENT; self.get_nodes_number() as usize];
        // We create the vector of vectors of the nodes per each community
        // that will be needed for the construction of the edge list.
        let mut node_ids_per_community: Vec<Vec<NodeT>> = Vec::new();
        // We create the remapped community weights
        let mut remapped_communities_weights = Vec::new();
        // And we compactify.
        communities
            .iter_mut()
            .zip(self.iter_node_ids())
            .for_each(|(community, node_id)| {
                if communities_remapping[*community as usize] == NOT_PRESENT {
                    communities_remapping[*community as usize] =
                        node_ids_per_community.len() as NodeT;
                    remapped_communities_weights.push(communities_weights[*community as usize]);
                    node_ids_per_community.push(Vec::new());
                }
                *community = communities_remapping[*community as usize];
                node_ids_per_community[*community as usize].push(node_id);
            });
        // Get the number of communities
        let communities_number = node_ids_per_community.len() as NodeT;
        info!("Creating graph for the next recursive iteration.");
        // Create the new graph and re-iterate the procedure.
        let graph = build_graph_from_integers(
            Some(
                (0..communities_number)
                    .into_par_iter()
                    .flat_map(|src_community| {
                        (0..communities_number)
                            .flat_map(|dst_community| {
                                // If this is an undirected graph, we can
                                // compute only the upper triangolar adjacency matrix
                                // and avoid computing twice the edge weight.
                                if !self.is_directed() && dst_community > src_community {
                                    return vec![];
                                }
                                let edge_weight = if dst_community == src_community {
                                    remapped_communities_weights[src_community as usize]
                                } else {
                                    node_ids_per_community[src_community as usize]
                                        .iter()
                                        .map(|&node_id| {
                                            get_node_to_community_weighted_outdegree(
                                                self,
                                                node_id,
                                                dst_community,
                                                &communities,
                                            )
                                        })
                                        .sum::<f64>()
                                } as WeightT;
                                if edge_weight.is_zero() {
                                    vec![]
                                } else if self.is_directed() || src_community == dst_community {
                                    vec![(0, (src_community, dst_community, None, edge_weight))]
                                } else {
                                    vec![
                                        (0, (src_community, dst_community, None, edge_weight)),
                                        (0, (dst_community, src_community, None, edge_weight)),
                                    ]
                                }
                            })
                            .collect::<Vec<_>>()
                    }),
            ),
            Vocabulary::from_range(0..communities_number),
            None,
            None,
            true,
            self.is_directed(),
            Some(true),
            Some(false),
            Some(false),
            None,
            true,
            true,
            self.get_name(),
        )
        .unwrap();
        // Append the obtained community to the result vector.
        let mut all_communities: Vec<Vec<NodeT>> = vec![communities];
        // Recursion step.
        all_communities.extend(
            graph
                .louvain_community_detection(
                    Some(recursion_minimum_improvement),
                    Some(first_phase_minimum_improvement),
                    None,
                )
                .unwrap(),
        );
        info!("Returning computed communities.");
        // Return the obtained graph.
        Ok(all_communities)
    }
}
