use super::*;
use itertools::Itertools;
use log::info;
use num_traits::{Pow, Zero};
use rayon::prelude::*;

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
    /// * `first_phase_minimum_improvement`: Option<f64> - The minimum improvement to warrant another first phase iteration. By default, `0.00001` (not zero because of numerical instability).
    /// * `default_weight`: Option<WeightT> - The default weight to use if the graph is not weighted. By default, one.
    /// * `patience`: Option<usize> - How many iterations of the first phase to wait for before stopping. By default, `5`.
    ///
    /// # Raises
    /// * If the `default_weight` has been provided but the graph is already weighted.
    /// * If the `default_weight` has an invalid value, i.e. zero, NaN or infinity.
    /// * If the `recursion_minimum_improvement` has an invalid value, i.e. NaN or infinity.
    /// * If the `first_phase_minimum_improvement` has an invalid value, i.e. NaN or infinity.
    ///
    /// # References
    /// The implementation follows what described in [Blondel et al paper](https://iopscience.iop.org/article/10.1088/1742-5468/2008/10/P10008/pdf?casa_token=YoBiFS-4w5EAAAAA:BaHtIrzOvzMsQol_XR7wFGqZWun5_GDn2O86KU9x5bVUN859DGred8dgV7iqxKmjrLOCTR62uccXUQ)
    /// and mainly the [Directed Louvain: maximizing modularity in directed networks](https://hal.archives-ouvertes.fr/hal-01231784/document)
    /// by Nicolas Dugué and Anthony Perez.
    pub fn louvain_community_detection(
        &self,
        recursion_minimum_improvement: Option<f64>,
        first_phase_minimum_improvement: Option<f64>,
        default_weight: Option<WeightT>,
        patience: Option<usize>,
    ) -> Result<Vec<Vec<usize>>> {
        if default_weight.is_some() && self.has_edge_weights() {
            return Err(concat!(
                "It does not make sense to provide the default weight when ",
                "the graph is already weighted."
            )
            .to_string());
        }
        let recursion_minimum_improvement: f64 = recursion_minimum_improvement.unwrap_or(0.0);
        let first_phase_minimum_improvement: f64 =
            first_phase_minimum_improvement.unwrap_or(0.00001);
        let default_weight: WeightT = default_weight.unwrap_or(1.0);
        let patience: usize = patience.unwrap_or(5);
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
        // We initialize the communities as the ids of the nodes.
        let mut communities = (0..self.get_nodes_number() as usize).collect::<Vec<usize>>();
        // Vector of the weights of the edges contained within each community.
        let mut communities_weights: Vec<f64> = self
            .par_iter_node_ids()
            .map(|node_id| unsafe {
                if let Ok(edge_id) = self.get_edge_id_from_node_ids(node_id, node_id) {
                    self.get_unchecked_edge_weight_from_edge_id(edge_id)
                        .unwrap_or(default_weight) as f64
                } else {
                    0.0
                }
            })
            .collect();
        // We initialize the community vectors.
        let mut node_ids_per_community: Vec<Vec<NodeT>> = self
            .par_iter_node_ids()
            .map(|node_id| vec![node_id])
            .collect();
        // We compute the weighted node outdegrees
        let weighted_node_outdegrees: Vec<f64> =
            self.get_weighted_node_degrees().unwrap_or_else(|_| {
                self.get_node_degrees()
                    .into_par_iter()
                    .map(|outdegree| (default_weight as f64) * outdegree as f64)
                    .collect::<Vec<_>>()
            });
        // We initialize the weighted communities outdegrees as the weighted node outdegrees
        let mut weighted_community_outdegrees = weighted_node_outdegrees.clone();
        // If the graph is directed we also need to compute the indegrees,
        // otherwise we can use the outdegrees.
        let weighted_node_indegrees = if self.is_directed() {
            Some(self.get_weighted_node_indegrees().unwrap_or_else(|_| {
                self.get_node_indegrees()
                    .into_par_iter()
                    .map(|indegree| (default_weight as f64) * indegree as f64)
                    .collect::<Vec<_>>()
            }))
        } else {
            None
        };
        // We initialize the weighted communities indegrees as the weighted node indegrees
        let mut weighted_community_indegrees = weighted_node_indegrees.clone();
        // Total edge weights, i.e. the weights of all the edges in the graph.
        let total_edge_weights: f64 = self
            .get_total_edge_weights()
            // If the graph does not start as a weighted graph, we use the default weight
            // that was provided by the user.
            .unwrap_or_else(|_| {
                (default_weight as f64) * (self.get_directed_edges_number() as f64)
            });
        let total_edge_weights_squared = total_edge_weights.pow(2);

        // Define method to compute in stream the total edge weight from
        // a given node to a given node community
        let get_node_to_community_weighted_degree =
            |src: NodeT,
             community_id: usize,
             communities: &[usize],
             community_node_ids: &[NodeT]| unsafe {
                if self.get_unchecked_node_degree_from_node_id(src) as usize
                    > community_node_ids.len()
                {
                    community_node_ids
                        .iter()
                        .filter(|&&dst| src != dst)
                        .map(|&dst| {
                            self.iter_unchecked_edge_ids_from_node_ids(src, dst)
                                .map(|edge_id| {
                                    self.get_unchecked_edge_weight_from_edge_id(edge_id)
                                        .unwrap_or(default_weight)
                                        as f64
                                })
                                .sum::<f64>()
                        })
                        .sum::<f64>()
                } else {
                    self.iter_unchecked_neighbour_node_ids_from_source_node_id(src)
                        .filter(|&dst| src != dst && communities[dst as usize] == community_id)
                        .map(|dst| {
                            self.iter_unchecked_edge_ids_from_node_ids(src, dst)
                                .map(|edge_id| {
                                    self.get_unchecked_edge_weight_from_edge_id(edge_id)
                                        .unwrap_or(default_weight)
                                        as f64
                                })
                                .sum::<f64>()
                        })
                        .sum::<f64>()
                }
            };
        // Define method to compute in stream the total weighted indegree
        // of a given node, including exclusively edges coming from a given community
        let get_community_to_node_weighted_degree = |dst: NodeT, community_node_ids: &[NodeT]| unsafe {
            community_node_ids
                .iter()
                .filter(|&&src| src != dst)
                .map(|&src| {
                    self.iter_unchecked_edge_ids_from_node_ids(src, dst)
                        .map(|edge_id| {
                            self.get_unchecked_edge_weight_from_edge_id(edge_id)
                                .unwrap_or(default_weight) as f64
                        })
                        .sum::<f64>()
                })
                .sum::<f64>()
        };
        // Define method to compute the modularity variation of adding a node to a given community.
        let compute_modularity_change =
            |node_id: NodeT,
             community_id: usize,
             communities: &[usize],
             community_node_ids: &[NodeT],
             outdegree: f64,
             indegree: f64,
             community_outdegree: f64,
             community_indegree: f64| {
                // We compute the node to community weighted degree
                let node_to_community_weighted_degree: f64 = get_node_to_community_weighted_degree(
                    node_id,
                    community_id,
                    communities,
                    community_node_ids,
                );
                // We actually compute the modularity variation
                let modularity_variation: f64 = node_to_community_weighted_degree
                    / total_edge_weights
                    - (outdegree * community_indegree + indegree * community_outdegree)
                        / total_edge_weights_squared;
                // We retrieve the computed modularity variations and the node to community weighted degree
                (node_to_community_weighted_degree, modularity_variation)
            };
        // The overall recursion is regulated by the total
        // change of modularity of the iteration.
        let mut total_modularity_change: f64 = 0.0;
        info!("Started Louvian phase one loop.");
        let mut loops_number: usize = 0;
        let mut patience_counter: usize = 0;
        // Execute the first phase until convergence
        loop {
            info!("Started Louvian phase one loop #{}.", loops_number);
            loops_number += 1;
            let mut total_change_per_iter: f64 = 0.0;
            self.iter_node_ids().for_each(|src| {
                // We retrieve the current node component.
                let current_node_community = communities[src as usize];
                // We retrieve the current node indegree and outdegree.
                let outdegree = weighted_node_outdegrees[src as usize];
                let indegree = weighted_node_indegrees
                    .as_ref()
                    .map_or(outdegree, |weighted_node_indegrees| {
                        weighted_node_indegrees[src as usize]
                    });

                // We search for the best community to add this node to.
                let result =
                    unsafe { self.iter_unchecked_neighbour_node_ids_from_source_node_id(src) }
                        .map(|dst| communities[dst as usize])
                        .filter(|&neighbour_community_id| {
                            neighbour_community_id != current_node_community
                        })
                        .unique()
                        .collect::<Vec<_>>()
                        .into_par_iter()
                        .map(|neighbour_community_id| {
                            // Similarly, we retrieve the current community indegree and outdegree.
                            let community_outdegree =
                                weighted_community_outdegrees[neighbour_community_id as usize];
                            let community_indegree = weighted_community_indegrees.as_ref().map_or(
                                community_outdegree,
                                |weighted_node_indegrees| {
                                    weighted_node_indegrees[neighbour_community_id as usize]
                                },
                            );
                            (
                                neighbour_community_id,
                                compute_modularity_change(
                                    src,
                                    neighbour_community_id,
                                    &communities,
                                    &node_ids_per_community[neighbour_community_id],
                                    outdegree,
                                    indegree,
                                    community_outdegree,
                                    community_indegree,
                                ),
                            )
                        })
                        .max_by(
                            |(_, (_, modularity_variation1)), (_, (_, modularity_variation2))| {
                                // These values cannot ever be NaNs, so this comparison
                                // can be unwrapped without the worry of causing a panic.
                                modularity_variation1
                                    .partial_cmp(modularity_variation2)
                                    .unwrap()
                            },
                        );

                if let Some((
                    neighbour_community_id,
                    (node_to_community_weighted_degree, adding_node_modularity_variation),
                )) = result
                {
                    // Similarly, we retrieve the current community indegree and outdegree.
                    let community_outdegree =
                        weighted_community_outdegrees[current_node_community as usize];
                    let community_indegree = weighted_community_indegrees.as_ref().map_or(
                        community_outdegree,
                        |weighted_node_indegrees| {
                            weighted_node_indegrees[current_node_community as usize]
                        },
                    );
                    // If we have found at least a new candidate
                    // community to move this node towards, we
                    // can compute the modularity change derived from moving
                    // this node outside from its current component.
                    let (
                        node_to_previous_community_weighted_outdegree,
                        removing_node_modularity_variation,
                    ) = compute_modularity_change(
                        src,
                        current_node_community,
                        &communities,
                        &node_ids_per_community[current_node_community],
                        outdegree,
                        indegree,
                        community_outdegree,
                        community_indegree,
                    );
                    // Compute the total modularity variation.
                    let modularity_variation =
                        adding_node_modularity_variation - removing_node_modularity_variation;
                    // If this is improving the current bound
                    if modularity_variation > 0.0 {
                        // When we need to change the community of a node, a lot of things
                        // need to happen:
                        //
                        // - We need to update the previous community weight
                        //      - Subtract the edges from the `src` node to the other nodes in the community
                        //      - Subtract the edges from the nodes in the community to the `src` node
                        //          - This second step in an undirected graph simply means to subtract again the
                        //            value computed at the previous step, while in a directed graph it requires
                        //            to compute the instar of the src edge and filter for the source nodes that
                        //            come from the `current_node_community` community.
                        // - We need to update the previous community weighted outdegree
                        //      - Subtract the weighted outdegree of the `src` node.
                        //      - Add the total edge weights of the edges from the `src` node to any of the edges of the community (precomputed in previous step)
                        // - If this is a directed graph, we need to update the previous community weighted indegree
                        //      - Subtract the weighted indegree of the `src` node.
                        //      - Add the total edge weights of the edges from the `src` node to any of the edges of the community (precomputed in previous step)
                        // - We need to update the new community weight
                        //      - Add the node to community weighted degree, precomputed when searching for best community.
                        //      - Add the community to the node weighted degree
                        //          - This second step in an undirected graph simply means to add again the
                        //            value computed at the previous step, while in a directed graph it requires
                        //            to compute the instar of the src edge and filter for the source nodes that
                        //            come from the `neighbour_community_id` community.
                        // - We need to update the new community weighted outdegree
                        //      - Subtract the node to the community weighted degree, computed at the previous step.
                        //      - Add the node outdegree, subtracting the edge weights from edges in the community.
                        //          - This second step, when in an undirected graph, simply requires to subtract to the
                        //            outdegree of the `src` the node to the community weighted degree, while in a
                        //            directed graph it requires to compute the indegree filtering nodes from the
                        //            comunity `neighbour_community_id`.
                        // - If this is a directed graph, we need to update the new community weighted indegree
                        //      - Subtract the node to the community weighted degree, computed at the previous step.
                        //      - Add the node indegree, subtracting the edge weights from edges in the community.
                        //          - This second step, when in an undirected graph, simply requires to subtract to the
                        //            indegree of the `src` the node to the community weighted degree, while in a
                        //            directed graph it requires to compute the indegree filtering nodes from the
                        //            comunity `neighbour_community_id`.
                        // - Lastly, we need to change the community of the node.

                        // Assign to the community of the best neighbor
                        // since this improves modularity
                        total_change_per_iter += modularity_variation;

                        // #############################################
                        // Updating the previous community values.
                        // #############################################
                        //
                        // Updating the previous community weights
                        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                        // If this is a directed graph
                        let previous_community_to_node_weighted_outdegree = if self.is_directed() {
                            // We need to compute the indegree from the nodes
                            // in the community to this node.
                            get_community_to_node_weighted_degree(
                                src,
                                &node_ids_per_community[current_node_community],
                            )
                        } else {
                            // Else if the graph is undirected, simply use again
                            // the previously weighted outdegree.
                            node_to_previous_community_weighted_outdegree
                        };
                        // We compute the previous community edge weight variation
                        let previous_community_edge_weight_variation =
                            node_to_previous_community_weighted_outdegree
                                + previous_community_to_node_weighted_outdegree;
                        // We subtract the two values from the community weight.
                        communities_weights[current_node_community as usize] -=
                            previous_community_edge_weight_variation;
                        //
                        if let Some(weighted_community_indegrees) =
                            &mut weighted_community_indegrees
                        {
                            // Updating the previous community weighted indegree
                            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                            // Now we proceed to update the indegree of the current community
                            // of the `src` node.
                            // We need to remove from the community indegree the `src` indegree,
                            // to which we need to remove the edges from the community to the node.
                            weighted_community_indegrees[current_node_community as usize] -=
                                indegree - previous_community_edge_weight_variation;
                        }

                        weighted_community_outdegrees[current_node_community as usize] -=
                            outdegree - previous_community_edge_weight_variation;

                        // #############################################
                        // Updating the new community values.
                        // #############################################
                        //
                        // Updating the new community weight
                        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                        // We need to add the edge weights of the edges from
                        // the `src` node to the nodes in the community `neighbour_community_id`, value which
                        // is precomputed in the variable `node_to_community_weighted_degree`,
                        // and then also add the edge weights of the edges from
                        // the nodes in the community `neighbour_community_id`, which in an undirected
                        // graph is equal to the previous value `node_to_community_weighted_degree`,
                        // while in a directed graph requires to compute the indegree of the nodes
                        // from the community `neighbour_community_id` to the node `src`.
                        //
                        // To compute this second edge weight, therefore, we start by
                        // checking whether the graph is directed.
                        let new_community_to_node_weighted_degree = if self.is_directed() {
                            // We need to compute the indegree from the nodes
                            // in the community to this node.
                            get_community_to_node_weighted_degree(
                                src,
                                &node_ids_per_community[neighbour_community_id],
                            )
                        } else {
                            // As aforementioned, if the graph is undirected
                            // then the communty to node weighted degree is equal
                            // to the precomputed value.
                            node_to_community_weighted_degree
                        };
                        // We compute the new community edge weight variation
                        let new_community_edge_weight_variation = node_to_community_weighted_degree
                            + new_community_to_node_weighted_degree;
                        // We add the two values from the community weight.
                        communities_weights[neighbour_community_id as usize] +=
                            new_community_edge_weight_variation;
                        //
                        if let Some(weighted_community_indegrees) =
                            &mut weighted_community_indegrees
                        {
                            // Updating the new community indegree
                            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                            // Now we proceed to update the new community indegree.
                            // This means subtracting the `node_to_community_weighted_degree`
                            // to the indegree of the community, which previously was summed to it,
                            // and add the indegree of the node.
                            // Additionally, we also need to subtract the `new_community_to_node_weighted_degree`
                            // to the indegree to avoid adding the edges that are now part of the community.
                            weighted_community_indegrees[neighbour_community_id as usize] +=
                                indegree - new_community_edge_weight_variation;
                        }

                        weighted_community_outdegrees[neighbour_community_id as usize] -=
                            outdegree - new_community_edge_weight_variation;

                        // Finally, we update the community of this node.
                        let index = node_ids_per_community[current_node_community as usize]
                            .iter()
                            .position(|&node| node == src)
                            .unwrap();
                        node_ids_per_community[current_node_community as usize].remove(index);
                        node_ids_per_community[neighbour_community_id as usize].push(src);
                        communities[src as usize] = neighbour_community_id;
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
        let mut reverse_communities_remapping = vec![INDEX_NOT_PRESENT; communities.len()];
        let mut communities_number = 0;
        // And we compactify.
        communities.iter_mut().for_each(|community| {
            if communities_remapping[*community as usize] == INDEX_NOT_PRESENT {
                communities_remapping[*community as usize] = communities_number;
                reverse_communities_remapping[communities_number as usize] = *community;
                communities_number += 1;
            }
            *community = communities_remapping[*community as usize];
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
                                communities_weights
                                    [reverse_communities_remapping[src_community as usize] as usize]
                            } else {
                                node_ids_per_community
                                    [reverse_communities_remapping[src_community as usize] as usize]
                                    .iter()
                                    .map(|&node_id| {
                                        get_node_to_community_weighted_degree(
                                            node_id,
                                            reverse_communities_remapping[dst_community as usize],
                                            &communities,
                                            &node_ids_per_community[reverse_communities_remapping
                                                [dst_community as usize]],
                                        )
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
        let mut all_communities: Vec<Vec<usize>> = vec![communities];
        // Recursion step.
        all_communities.extend(
            graph
                .louvain_community_detection(
                    Some(recursion_minimum_improvement),
                    Some(first_phase_minimum_improvement),
                    None,
                    Some(patience),
                )
                .unwrap(),
        );
        info!("Returning computed communities.");
        // Return the obtained graph.
        Ok(all_communities)
    }

    /// Returns the modularity of the graph from the given memberships.
    ///
    /// # Arguments
    /// * `node_community_memberships`: &[NodeT], The memberships assigned to each node of the graph.
    /// * `default_weight`: Option<WeightT> - The default weight to use if the graph is not weighted. By default, one.
    ///
    /// # Raises
    /// * If the `default_weight` has been provided but the graph is already weighted.
    /// * If the `default_weight` has an invalid value, i.e. zero, NaN or infinity.
    /// * If the number of provided memberships does not match the number of nodes of the graph.
    /// * If the memberships are not a dense range from 0 to `max(memberships)`.
    ///
    /// # References
    /// The formula implementation is as defined from [Community structure in directed networks](https://arxiv.org/pdf/0709.4500.pdf),
    /// by E. A. Leicht and M. E. J. Newman.
    pub fn get_modularity_from_node_community_memberships(
        &self,
        node_community_memberships: &[NodeT],
        default_weight: Option<WeightT>,
    ) -> Result<f64> {
        // If the current graph instance has no edges, surely the
        // modularity is zero.
        if !self.has_edges() {
            return Ok(0.0);
        }
        // Otherwise we check if the provided node colors are compatible with the current
        // graph instance.
        if node_community_memberships.len() != self.get_nodes_number() as usize {
            return Err(format!(
                concat!(
                    "The graph contains {} nodes, while the provided node community memberships ",
                    "where {}. You need to provide the node community memberships for each node."
                ),
                node_community_memberships.len(),
                self.get_nodes_number()
            ));
        }
        // If the graph has edge weights and a default weight has
        // been provided that value will not be considered.
        if default_weight.is_some() && self.has_edge_weights() {
            return Err(concat!(
                "It does not make sense to provide the default weight when ",
                "the graph is already weighted."
            )
            .to_string());
        }
        // Handle the defaults if the value has not been provided.
        let default_weight = default_weight.unwrap_or(1.0);
        // We check that the default weight parameter has a valida parameter.
        if default_weight.is_zero() || default_weight.is_nan() || default_weight.is_infinite() {
            return Err(concat!(
                "The provided parameter `default_weight` is an illegal value, i.e. ",
                "either zero, NaN or infinity."
            )
            .to_string());
        }
        // We compute the edge weights
        // Total edge weights, i.e. the weights of all the edges in the graph.
        let total_edge_weights: f64 = self
            .get_total_edge_weights()
            // If the graph does not start as a weighted graph, we use the default weight
            // that was provided by the user.
            .unwrap_or_else(|_| {
                (default_weight as f64) * (self.get_directed_edges_number() as f64)
            });
        // Compute the weighted node outdegrees
        let weighted_node_outdegrees: Vec<f64> =
            self.get_weighted_node_degrees().unwrap_or_else(|_| {
                self.par_iter_node_degrees()
                    .map(|degree| (default_weight as f64) * degree as f64)
                    .collect::<Vec<_>>()
            });
        // Compute the weighted node indegrees
        let weighted_node_indegrees = if self.is_directed() {
            Some(self.get_weighted_node_indegrees().unwrap_or_else(|_| {
                self.get_node_indegrees()
                    .into_par_iter()
                    .map(|indegree| (default_weight as f64) * indegree as f64)
                    .collect::<Vec<_>>()
            }))
        } else {
            None
        };

        let compute_modularity = |src, dst, edge_weight| {
            edge_weight as f64
                - weighted_node_indegrees.as_ref().map_or_else(
                    || weighted_node_outdegrees[src as usize],
                    |weighted_node_indegrees| weighted_node_indegrees[src as usize],
                ) * weighted_node_outdegrees[dst as usize]
                    / total_edge_weights
        };

        let have_same_community = |src: &NodeT, dst: &NodeT| -> bool {
            node_community_memberships[*src as usize] == node_community_memberships[*dst as usize]
        };

        // Then we actually compute the modularity.
        Ok(if self.has_edge_weights() {
            self.par_iter_directed_edge_node_ids()
                .zip(self.par_iter_edge_weights().unwrap())
                .filter(|((_, src, dst), _)| have_same_community(src, dst))
                .map(|((_, src, dst), edge_weight)| compute_modularity(src, dst, edge_weight))
                .sum::<f64>()
        } else {
            self.par_iter_directed_edge_node_ids()
                .filter(|(_, src, dst)| have_same_community(src, dst))
                .map(|(_, src, dst)| compute_modularity(src, dst, default_weight))
                .sum::<f64>()
        } / total_edge_weights)
    }
}
