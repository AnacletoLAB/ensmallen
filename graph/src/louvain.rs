use super::*;
use log::info;
use num_traits::{Pow, Zero};
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rayon::prelude::*;

impl Graph {
    #[no_numpy_binding]
    /// Returns vector of vectors of communities for each layer of hierarchy minimizing undirected modularity.
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
    /// * `first_phase_minimum_improvement`: Option<f64> - The minimum improvement to warrant another first phase iteration. By default, `0.00001` (not zero because of numerical instability).
    /// * `patience`: Option<usize> - How many iterations of the first phase to wait for before stopping. By default, `5`.
    /// * `random_state`: Option<u64> - The random state to use to reproduce this modularity computation. By default, 42.
    ///
    /// # Raises
    /// * If the graph is not directed.
    /// * If the `recursion_minimum_improvement` has an invalid value, i.e. NaN or infinity.
    /// * If the `first_phase_minimum_improvement` has an invalid value, i.e. NaN or infinity.
    ///
    /// # References
    /// The implementation follows what described in [Blondel et al paper](https://iopscience.iop.org/article/10.1088/1742-5468/2008/10/P10008/pdf?casa_token=YoBiFS-4w5EAAAAA:BaHtIrzOvzMsQol_XR7wFGqZWun5_GDn2O86KU9x5bVUN859DGred8dgV7iqxKmjrLOCTR62uccXUQ)
    /// and mainly the [Directed Louvain: maximizing modularity in directed networks](https://hal.archives-ouvertes.fr/hal-01231784/document)
    /// by Nicolas Dugué and Anthony Perez.
    pub fn get_undirected_louvain_community_detection(
        &self,
        recursion_minimum_improvement: Option<f64>,
        first_phase_minimum_improvement: Option<f64>,
        patience: Option<usize>,
        random_state: Option<u64>,
    ) -> Result<Vec<Vec<usize>>> {
        self.must_be_undirected()?;
        let recursion_minimum_improvement: f64 = recursion_minimum_improvement.unwrap_or(0.0);
        let first_phase_minimum_improvement: f64 =
            first_phase_minimum_improvement.unwrap_or(0.00001);
        let patience: usize = patience.unwrap_or(5);
        let random_state = random_state.unwrap_or(42);
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
        // We need to collect the node ids into a vector because we will shuffle them in the main loop.
        let mut node_ids = (0..self.get_number_of_nodes() as usize).collect::<Vec<usize>>();
        // We initialize the communities as the ids of the nodes.
        let mut communities = node_ids.clone();
        // Vector of the weights of the edges contained within each community.
        // This, at the beginning, is equal to the weight of the nodes selfloops if present.
        // At the beginning, this also matches the weight contribution of a node to its own community.
        // Afterwards, the two vectors diverge.
        let mut communities_weights: Vec<f64> = if self.has_selfloops() {
            // If the graph has selfloops, we do the effort of creating the array of the weights
            // of the various communities.
            self.par_iter_node_ids()
                .map(|node_id| unsafe {
                    if let Ok(edge_id) = self.get_edge_id_from_node_ids(node_id, node_id) {
                        self.get_unchecked_edge_weight_from_edge_id(edge_id)
                            .unwrap_or(1.0) as f64
                    } else {
                        0.0
                    }
                })
                .collect()
        } else {
            // Alternatively, we know they are all zero.
            vec![0.0; self.get_number_of_nodes() as usize]
        };
        // We compute the weighted node degrees
        let weighted_node_degrees: Vec<f64> =
            self.get_weighted_node_degrees().unwrap_or_else(|_| {
                self.par_iter_node_degrees()
                    .map(|degree| degree as f64)
                    .collect::<Vec<_>>()
            });
        // We initialize the weighted communities outdegrees as the weighted node outdegrees
        let mut weighted_community_degrees = weighted_node_degrees.clone();
        // Total edge weights, i.e. the weights of all the edges in the graph.
        let total_edge_weights: f64 = self
            .get_total_edge_weights()
            // If the graph does not start as a weighted graph, we use the default weight
            // that was provided by the user.
            .unwrap_or_else(|_| self.get_number_of_directed_edges() as f64);
        // We need also the double of the total edge weights.
        let total_edge_weights_doubled = total_edge_weights * 2.0;
        // Since we also need it, we also compute the total edge weights squared.
        let total_edge_weights_squared_doubled = 2.0 * total_edge_weights.pow(2);

        // The overall recursion is regulated by the total
        // change of modularity of the iteration.
        let mut total_modularity_change: f64 = 0.0;
        info!("Started Louvian phase one loop.");
        let mut loops_number: usize = 0;
        let mut patience_counter: usize = 0;
        let mut rng = SmallRng::seed_from_u64(splitmix64(random_state) as EdgeT);
        // Execute the first phase until convergence
        loop {
            info!("Started Louvian phase one loop #{}.", loops_number);
            loops_number += 1;
            node_ids.shuffle(&mut rng);
            let mut total_change_per_iter: f64 = 0.0;
            node_ids.iter().cloned().for_each(|src| {
                // We retrieve the current node component.
                let current_node_community = communities[src];
                // Retrieve the current node weighted degree.
                let current_node_weighted_degree = weighted_node_degrees[src];
                // Create vector of communities indegrees
                let mut communities_indegrees = vec![0.0; self.get_number_of_nodes() as usize];
                // Populate neighbours communities indegrees and weights of the neighbours communities.
                let neighbours_weights_and_community_ids: Vec<(f64, usize)> =
                    if self.has_edge_weights() {
                        unsafe {
                            self.iter_unchecked_neighbour_node_ids_from_source_node_id(src as NodeT)
                        }
                        .zip(unsafe {
                            self.iter_unchecked_edge_weights_from_source_node_id(src as NodeT)
                        })
                        .filter(|(dst, _)| *dst != src as NodeT)
                        .map(|(dst, weight)| (communities[dst as usize], weight))
                        .map(|(neighbour_community_id, weight)| {
                            let neighbour_community_degree_adding_node = weighted_community_degrees
                                [neighbour_community_id]
                                + current_node_weighted_degree;
                            communities_indegrees[neighbour_community_id] += weight as f64;
                            (
                                neighbour_community_degree_adding_node,
                                neighbour_community_id,
                            )
                        })
                        .collect()
                    } else {
                        unsafe {
                            self.iter_unchecked_neighbour_node_ids_from_source_node_id(src as NodeT)
                        }
                        .filter(|&dst| dst != src as NodeT)
                        .map(|dst| communities[dst as usize])
                        .map(|neighbour_community_id| {
                            let neighbour_community_degree_adding_node = communities_weights
                                [neighbour_community_id]
                                + current_node_weighted_degree;
                            communities_indegrees[neighbour_community_id] += 1.0;
                            (
                                neighbour_community_degree_adding_node,
                                neighbour_community_id,
                            )
                        })
                        .collect()
                    };
                let best_community = neighbours_weights_and_community_ids
                    .into_par_iter()
                    .map(|(neighbour_community_degree_adding_node, community_id)| {
                        let adding_node_modularity_variation = communities_indegrees
                            [community_id as usize]
                            / total_edge_weights_doubled
                            - neighbour_community_degree_adding_node * current_node_weighted_degree
                                / total_edge_weights_squared_doubled;
                        (
                            neighbour_community_degree_adding_node,
                            community_id,
                            adding_node_modularity_variation,
                        )
                    })
                    .max_by(
                        |(_, _, one): &(f64, usize, f64), (_, _, two): &(f64, usize, f64)| {
                            one.partial_cmp(two).unwrap()
                        },
                    );

                // If we have found a better community we can check
                // if it actually improved the modularity.
                if let Some((
                    neighbour_community_degree_adding_node,
                    community_id,
                    adding_node_modularity_variation,
                )) = best_community
                {
                    // Compute the weight of the current component if the node where to be removed.
                    let current_component_degree_without_node = weighted_community_degrees
                        [current_node_community]
                        - current_node_weighted_degree;

                    let removing_node_modularity_variation = communities_indegrees
                        [current_node_community as usize]
                        / total_edge_weights_doubled
                        - current_component_degree_without_node * current_node_weighted_degree
                            / total_edge_weights_squared_doubled;

                    // Compute the total modularity variation.
                    let modularity_variation =
                        adding_node_modularity_variation - removing_node_modularity_variation;
                    // If this is improving the current bound
                    if modularity_variation > 0.0 {
                        total_change_per_iter += modularity_variation;
                        communities[src] = community_id;
                        weighted_community_degrees[current_node_community] =
                            current_component_degree_without_node;
                        communities_weights[current_node_community] -=
                            communities_indegrees[current_node_community as usize];
                        weighted_community_degrees[community_id] =
                            neighbour_community_degree_adding_node;
                        communities_weights[community_id] +=
                            communities_indegrees[community_id as usize];
                    }
                }
            });

            total_modularity_change += total_change_per_iter;
            info!(
                "The modularity change in loop #{} is {}.",
                loops_number, total_change_per_iter
            );
            if total_change_per_iter <= first_phase_minimum_improvement {
                patience_counter += 1;
                if patience_counter > patience || total_change_per_iter <= f64::EPSILON {
                    break;
                }
            } else {
                patience_counter = 0;
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
        let mut communities_remapping = vec![INDEX_NOT_PRESENT; communities.len()];
        let mut remapped_community_weights = Vec::with_capacity(communities.len());
        let mut node_ids_per_community = Vec::new();
        let mut communities_number = 0;
        // And we compactify.
        communities
            .iter_mut()
            .enumerate()
            .for_each(|(node_id, community)| {
                if communities_remapping[*community as usize] == INDEX_NOT_PRESENT {
                    communities_remapping[*community as usize] = communities_number;
                    remapped_community_weights.push(communities_weights[*community]);
                    node_ids_per_community.push(Vec::new());
                    communities_number += 1;
                }
                let remapped_community = communities_remapping[*community as usize];
                node_ids_per_community[remapped_community].push(node_id as NodeT);
                *community = remapped_community;
            });

        let communities_number = communities_number as NodeT;

        info!("Creating graph for the next recursive iteration.");
        // Create the new graph and re-iterate the procedure.
        let graph = build_graph_from_integers(
            Some(
                (0..communities_number)
                    .into_par_iter()
                    .flat_map_iter(move |src_community| {
                        (0..communities_number)
                            .map(move |dst_community| (src_community, dst_community))
                    })
                    // If this is an undirected graph, we can
                    // compute only the upper triangolar adjacency matrix
                    // and avoid computing twice the edge weight.
                    .filter(|&(src_community, dst_community)| {
                        self.is_directed() || dst_community <= src_community
                    })
                    .map(|(src_community, dst_community)| {
                        (
                            src_community,
                            dst_community,
                            if dst_community == src_community {
                                remapped_community_weights[src_community as usize]
                            } else {
                                let dst_community = dst_community as usize;
                                node_ids_per_community[src_community as usize]
                                    .iter()
                                    .cloned()
                                    .map(|src| unsafe {
                                        if self.has_edge_weights(){
                                            self.iter_unchecked_neighbour_node_ids_from_source_node_id(
                                                src,
                                            ).zip(self.iter_unchecked_edge_weights_from_source_node_id(src))
                                            .filter_map(|(dst, weight)|{
                                                if communities[dst as usize] == dst_community {
                                                    Some(weight as f64)
                                                } else {
                                                    None
                                                }
                                            }).sum::<f64>()
                                        } else {
                                            self.iter_unchecked_neighbour_node_ids_from_source_node_id(
                                                src as NodeT,
                                            )
                                            .filter(|&dst|{
                                                communities[dst as usize] == dst_community
                                            }).count() as f64
                                        }
                                    })
                                    .sum::<f64>()
                            } as WeightT,
                        )
                    })
                    .filter(|&(_, _, edge_weight)| !edge_weight.is_zero())
                    .flat_map(|(src_community, dst_community, edge_weight)| {
                        if self.is_directed() || src_community == dst_community {
                            vec![(0, (src_community, dst_community, None, edge_weight))]
                        } else {
                            vec![
                                (0, (src_community, dst_community, None, edge_weight)),
                                (0, (dst_community, src_community, None, edge_weight)),
                            ]
                        }
                    }),
            ),
            Arc::new(Vocabulary::from_range(0..communities_number, "Nodes".to_string())),
            Arc::new(None),
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
        let mut all_communities: Vec<Vec<usize>> = vec![communities];
        // Recursion step.
        all_communities.extend(
            graph
                .get_undirected_louvain_community_detection(
                    Some(recursion_minimum_improvement),
                    Some(first_phase_minimum_improvement),
                    Some(patience),
                    Some(random_state),
                )
                .unwrap(),
        );
        info!("Returning computed communities.");
        // Return the obtained graph.
        Ok(all_communities)
    }

    /// Validated the provided parameters to compute modularity.
    ///
    /// # Arguments
    /// * `node_community_memberships`: &[NodeT], The memberships assigned to each node of the graph.
    ///
    /// # Raises
    /// * If the number of provided memberships does not match the number of nodes of the graph.
    fn validate_modularity_parameters(&self, node_community_memberships: &[NodeT]) -> Result<()> {
        // Otherwise we check if the provided node colors are compatible with the current
        // graph instance.
        if node_community_memberships.len() != self.get_number_of_nodes() as usize {
            return Err(format!(
                concat!(
                    "The graph contains {} nodes, while the provided node community memberships ",
                    "where {}. You need to provide the node community memberships for each node."
                ),
                node_community_memberships.len(),
                self.get_number_of_nodes()
            ));
        }
        Ok(())
    }

    /// Returns the modularity of the graph from the given memberships based on provided lambdas.
    ///
    /// # Arguments
    /// * `node_community_memberships`: &[NodeT], The memberships assigned to each node of the graph.
    /// * `factor`: f64 - The factor to use for the normalization factor.
    /// * `indegrees`: &[f64] - The weighted indegrees of the graph nodes.
    /// * `outdegrees`: &[f64] - The weighted outdegrees of the graph nodes.
    ///
    /// # Raises
    /// * If the number of provided memberships does not match the number of nodes of the graph.
    ///
    /// # References
    /// The formula implementation is as defined from [Community structure in directed networks](https://arxiv.org/pdf/0709.4500.pdf),
    /// by E. A. Leicht and M. E. J. Newman.
    fn get_modularity_from_node_community_memberships(
        &self,
        node_community_memberships: &[NodeT],
        factor: f64,
        indegrees: &[f64],
        outdegrees: &[f64],
    ) -> Result<f64> {
        // If the current graph instance has no edges, surely the
        // modularity is zero.
        if !self.has_edges() {
            return Ok(0.0);
        }
        // Validate parameters and get default weight.
        self.validate_modularity_parameters(node_community_memberships)?;
        // We compute the edge weights
        // Total edge weights, i.e. the weights of all the edges in the graph.
        let total_edge_weights: f64 = self
            .get_total_edge_weights()
            // If the graph does not start as a weighted graph, we use the default weight
            // that was provided by the user.
            .unwrap_or_else(|_| self.get_number_of_directed_edges() as f64);
        // Lambda to set the logic of filtering nodes of the single node community
        // ony one time.
        let have_same_community = |src: &NodeT, dst: &NodeT| -> bool {
            node_community_memberships[*src as usize] == node_community_memberships[*dst as usize]
        };
        // Lmbda to compute the modularity.
        let compute_modularity = |src, dst, edge_weight| {
            edge_weight as f64
                - indegrees[src as usize] * outdegrees[dst as usize] / (factor * total_edge_weights)
        };
        // Then we actually compute the modularity.
        Ok(if self.has_edge_weights() {
            self.par_iter_directed_edge_node_ids()
                .zip(self.par_iter_directed_edge_weights().unwrap())
                .filter(|((_, src, dst), _)| have_same_community(src, dst))
                .map(|((_, src, dst), edge_weight)| {
                    compute_modularity(src, dst, edge_weight as f64)
                })
                .sum::<f64>()
        } else {
            self.par_iter_directed_edge_node_ids()
                .filter(|(_, src, dst)| have_same_community(src, dst))
                .map(|(_, src, dst)| compute_modularity(src, dst, 1.0))
                .sum::<f64>()
        } / (factor * total_edge_weights))
    }

    /// Returns the directed modularity of the graph from the given memberships.
    ///
    /// # Arguments
    /// * `node_community_memberships`: &[NodeT], The memberships assigned to each node of the graph.
    ///
    /// # Raises
    /// * If the number of provided memberships does not match the number of nodes of the graph.
    ///
    /// # References
    /// The formula implementation is as defined from [Community structure in directed networks](https://arxiv.org/pdf/0709.4500.pdf),
    /// by E. A. Leicht and M. E. J. Newman.
    pub fn get_directed_modularity_from_node_community_memberships(
        &self,
        node_community_memberships: &[NodeT],
    ) -> Result<f64> {
        // Compute the weighted node outdegrees
        let weighted_node_outdegrees: Vec<f64> =
            self.get_weighted_node_degrees().unwrap_or_else(|_| {
                self.par_iter_node_degrees()
                    .map(|degree| degree as f64)
                    .collect::<Vec<_>>()
            });
        // Compute the weighted node indegrees
        let weighted_node_indegrees = if self.is_directed() {
            Some(self.get_weighted_node_indegrees().unwrap_or_else(|_| {
                self.get_node_indegrees()
                    .into_par_iter()
                    .map(|indegree| indegree as f64)
                    .collect::<Vec<_>>()
            }))
        } else {
            None
        };

        self.get_modularity_from_node_community_memberships(
            node_community_memberships,
            1.0,
            weighted_node_indegrees
                .as_ref()
                .map_or(&weighted_node_outdegrees, |indegrees| &indegrees),
            &weighted_node_outdegrees,
        )
    }

    /// Returns the undirected modularity of the graph from the given memberships.
    ///
    /// # Arguments
    /// * `node_community_memberships`: &[NodeT], The memberships assigned to each node of the graph.
    ///
    /// # Raises
    /// * If the number of provided memberships does not match the number of nodes of the graph.
    ///
    /// # References
    /// The formula implementation is as defined from [Community structure in directed networks](https://arxiv.org/pdf/0709.4500.pdf),
    /// by E. A. Leicht and M. E. J. Newman.
    pub fn get_undirected_modularity_from_node_community_memberships(
        &self,
        node_community_memberships: &[NodeT],
    ) -> Result<f64> {
        self.must_be_undirected()?;
        // Compute the weighted node outdegrees
        let weighted_node_degrees: Vec<f64> =
            self.get_weighted_node_degrees().unwrap_or_else(|_| {
                self.par_iter_node_degrees()
                    .map(|degree| degree as f64)
                    .collect::<Vec<_>>()
            });

        self.get_modularity_from_node_community_memberships(
            node_community_memberships,
            2.0,
            &weighted_node_degrees,
            &weighted_node_degrees,
        )
    }
}
