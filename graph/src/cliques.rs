use super::*;
use indicatif::ProgressIterator;
use log::info;
use rayon::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Hash, Clone, Debug, PartialEq)]
pub struct Clique {
    graph: Graph,
    node_ids: Vec<NodeT>,
}

use std::string::ToString;
impl ToString for Clique {
    fn to_string(&self) -> String {
        let show_node_type = if self.graph.has_node_types() {
            unsafe {
                !self
                    .graph
                    .has_unchecked_isomorphic_node_types_from_node_ids(self.node_ids.as_ref())
            }
        } else {
            false
        };
        format!(
            concat!(
                "<p>",
                "Clique containing {number_of_nodes} nodes. ",
                "Specifically, the nodes involved in the clique are: {nodes}.",
                "{node_types_counts}",
                "{edge_types_counts}",
                "</p>",
            ),
            number_of_nodes = to_human_readable_high_integer(self.len() as usize),
            nodes = unsafe {
                get_unchecked_formatted_list(
                    &self
                        .node_ids
                        .iter()
                        .map(|&node_id| {
                            self.graph.get_unchecked_succinct_node_description(
                                node_id,
                                self.len(),
                                show_node_type,
                            )
                        })
                        .collect::<Vec<String>>(),
                    Some(5),
                )
            },
            node_types_counts = unsafe {
                self.graph
                    .get_unchecked_node_type_id_counts_hashmap_from_node_ids(self.node_ids.as_ref())
                    .map_or_else(
                        |_| "".to_string(),
                        |count| {
                            if count.is_empty() {
                                "".to_string()
                            } else {
                                format!(
                                    " Its nodes have {}.",
                                    self.graph
                                        .get_unchecked_node_types_description_from_count(count)
                                )
                            }
                        },
                    )
            },
            edge_types_counts = unsafe {
                self.graph
                    .get_unchecked_edge_type_id_counts_hashmap_from_node_ids(self.node_ids.as_ref())
                    .map_or_else(
                        |_| "".to_string(),
                        |count| {
                            if count.is_empty() {
                                "".to_string()
                            } else {
                                format!(
                                    " Its edges have {}.",
                                    self.graph
                                        .get_unchecked_edge_types_description_from_count(count)
                                )
                            }
                        },
                    )
            }
        )
    }
}

impl PartialOrd for Clique {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.len().cmp(&other.len()))
    }
}

impl Clique {
    pub(crate) fn from_node_ids(graph: &Graph, node_ids: Vec<NodeT>) -> Clique {
        Clique {
            graph: graph.clone(),
            node_ids,
        }
    }

    /// Return length of the Clique.
    pub fn len(&self) -> NodeT {
        self.node_ids.len() as NodeT
    }

    /// Return the node IDs of the nodes composing the clique.
    pub fn get_node_ids(&self) -> Vec<NodeT> {
        self.node_ids.clone()
    }

    /// Return the node names of the nodes composing the Clique.
    pub fn par_iter_node_names(&self) -> impl IndexedParallelIterator<Item = String> + '_ {
        self.node_ids.par_iter().map(move |&node_id| unsafe {
            self.graph.get_unchecked_node_name_from_node_id(node_id)
        })
    }

    /// Return the node names of the nodes composing the Clique.
    pub fn get_node_names(&self) -> Vec<String> {
        self.par_iter_node_names().collect()
    }
}

impl Graph {
    /// Returns parallel iterator over a subset of the graph cliques.
    ///
    /// # Arguments
    /// `minimum_degree`: Option<NodeT> - The optional minimum degree, by default 10.
    /// `minimum_clique_size`: Option<NodeT> - The optional minimum clique size, by default 10.
    /// `clique_per_node`: Option<usize> - Maximum number of clique to find for each node.
    /// `verbose`: Option<bool> - Whether to show a loading bar. By default, True.
    ///
    /// # Raises
    /// * If the current graph is directed.
    pub fn iter_approximated_cliques(
        &self,
        minimum_degree: Option<NodeT>,
        minimum_clique_size: Option<NodeT>,
        clique_per_node: Option<usize>,
        verbose: Option<bool>,
    ) -> Result<impl Iterator<Item = Clique> + '_> {
        self.must_be_undirected()?;
        // First of all we set the minimum degree, which if None were provided is set to 10.
        let minimum_degree = minimum_degree.unwrap_or(10);
        // First of all we set the minimum clique size, which if None were provided is set to 10.
        let minimum_clique_size = minimum_clique_size.unwrap_or(10);
        // The number of clique per node to compute, which by default is 1.
        let clique_per_node = clique_per_node.unwrap_or(1);
        // Whether to show the loading bar while computing cliques.
        let verbose = verbose.unwrap_or(true);
        // We create a vector with the initial node degrees of the graph, wrapped into atomic.
        let mut node_degrees: Vec<AtomicU32> =
            Vec::with_capacity(self.get_number_of_nodes() as usize);
        self.par_iter_node_degrees()
            .map(|degree| AtomicU32::new(degree))
            .collect_into_vec(&mut node_degrees);

        // We define the method we use to remove a node from the set of nodes
        // that might form a clique of at least `minimum_degree` nodes.
        // This just set the degree of the given node to 0 and decrease the degree
        // of each of its neighbours to reflect the removing of the node from
        // the graph.
        let update_node_degree = |node_id: NodeT| {
            node_degrees[node_id as usize].store(0, Ordering::Relaxed);
            unsafe { self.iter_unchecked_neighbour_node_ids_from_source_node_id(node_id) }
                .for_each(|dst| {
                    node_degrees[dst as usize]
                        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |degree| {
                            Some(degree.saturating_sub(1))
                        })
                        .unwrap();
                });
        };

        // We do a preliminary filtering round removing the nodes with degree
        // lower than the provided minimum amount as they cannot form a clique
        // with at least `minimum_degree` nodes.
        // This initial filtering is also done again afterwards, but
        // by doing this preliminarly with an iterator we can allocate
        // a much smaller vector of nodes of interest.
        info!("Preliminary degree-based filtering.");
        let mut node_ids = self
            .par_iter_node_ids()
            .filter_map(|node_id| {
                // We retrieve the current node degree.
                let degree = node_degrees[node_id as usize].load(Ordering::Relaxed);
                // If the degree is zero, we have already dropped this node from the game.
                if degree < minimum_degree {
                    if degree > 0 {
                        update_node_degree(node_id);
                    }
                    None
                } else {
                    Some(node_id)
                }
            })
            .collect::<Vec<NodeT>>();
        // Compute a small report.
        let removed_number_of_nodes = self.get_number_of_nodes() as usize - node_ids.len();
        info!(
            concat!(
                "The preliminary filtering has removed ",
                "{removed_number_of_nodes} nodes ({percentage:.2})."
            ),
            removed_number_of_nodes = to_human_readable_high_integer(removed_number_of_nodes),
            percentage = removed_number_of_nodes as f64 / self.get_number_of_nodes() as f64 * 100.0
        );
        // Start to iterate over the node degrees vector
        // and in every iteration remove the nodes with degree smaller than
        // the provided amount. We iterate until no edit operation is
        // done.
        info!("Start iterations.");
        let mut current_iteration = 0;
        let minimum_clique_size_minus_one = (minimum_clique_size - 1) as usize;
        loop {
            let previous_number_of_nodes = node_ids.len();
            node_ids = node_ids
                .into_par_iter()
                .filter_map(|node_id| {
                    // We retrieve the current node degree.
                    let degree = node_degrees[node_id as usize].load(Ordering::Relaxed);
                    // If the degree is smaller than the minimum degree we can
                    // filter it out.
                    if degree < minimum_degree {
                        update_node_degree(node_id);
                        return None;
                    }
                    // Compute the neighbours of the node that have a degree
                    // high enough to be compatible with being in a clique with
                    // at least `minimum_degree` nodes.
                    // here the degree must be at least `minimum_degree - 1`
                    // because each node must have an edge to all the other nodes
                    // thus it's the size of the clique minus the node itself
                    let neighbours = unsafe {
                        self.iter_unchecked_unique_neighbour_node_ids_from_source_node_id(node_id)
                    }
                    .filter(|&dst| {
                        node_degrees[dst as usize].load(Ordering::Relaxed)
                            >= (minimum_clique_size - 1) as NodeT
                    })
                    .collect::<Vec<NodeT>>();

                    // to be in a clique with `k` nodes, the node must have at
                    // least `k - 1` neighbours with at least degree `k - 1`.
                    // if this condition is not met, we can filter the node.
                    if neighbours.len() < minimum_clique_size_minus_one {
                        update_node_degree(node_id);
                        return None;
                    }

                    // To check that the node may be part of a clique of size at least `minimum_clique_size`,
                    // then at least `minimum_clique_size - 1` neighbouring nodes must have at least `minimum_clique_size - 1`
                    // neighbouring nodes in the set of nodes of the initial node.
                    if neighbours
                        .iter()
                        .filter(|&&neighbour_node_id| {
                            iter_set::intersection(neighbours.iter().cloned(), unsafe {
                                self.iter_unchecked_unique_neighbour_node_ids_from_source_node_id(
                                    neighbour_node_id,
                                )
                            })
                            .take(minimum_clique_size_minus_one)
                            .count()
                                == minimum_clique_size_minus_one
                        })
                        .take(minimum_clique_size_minus_one)
                        .count()
                        != minimum_clique_size_minus_one
                    {
                        update_node_degree(node_id);
                        return None;
                    }

                    // We keep the node.
                    Some(node_id)
                })
                .collect::<Vec<NodeT>>();
            // We check if we have to stop.
            let currently_removed_nodes = previous_number_of_nodes - node_ids.len();
            if currently_removed_nodes == 0 {
                break;
            }
            info!(
                "#{current_iteration}: removed {currently_removed_nodes} nodes, remaining {remaining_nodes} ({percentage:.2}%).",
                current_iteration=current_iteration,
                currently_removed_nodes=to_human_readable_high_integer(currently_removed_nodes),
                remaining_nodes=to_human_readable_high_integer(node_ids.len()),
                percentage = node_ids.len() as f64 / self.get_number_of_nodes() as f64 * 100.0
            );
            current_iteration += 1;
        }

        //===========================================
        // Start computation of clique roots.
        //===========================================
        // We convert the atomic degrees to non-atomic.
        let mut node_degrees =
            unsafe { std::mem::transmute::<Vec<AtomicU32>, Vec<NodeT>>(node_degrees) };
        let mut node_degrees_copy = node_degrees.clone();

        info!(
            "Computing clique roots for {} nodes.",
            to_human_readable_high_integer(node_ids.len())
        );
        // Finally, we compute the clique root set of the nodes
        // and we obtain the set of nodes from where cliques may
        // be computed.
        let mut clique_roots = Vec::new();
        while let Some((node_id, _)) = node_degrees_copy
            .par_iter()
            .cloned()
            .enumerate()
            .filter(|(_, degree)| *degree >= minimum_clique_size - 1)
            .min_by_key(|(_, degree)| *degree)
        {
            clique_roots.push(node_id as NodeT);
            let covered_nodes = unsafe {
                self.iter_unchecked_unique_neighbour_node_ids_from_source_node_id(node_id as NodeT)
            }
            .filter(|&neighbour_node_id| {
                node_degrees_copy[neighbour_node_id as usize] >= minimum_clique_size - 1
            })
            .collect::<Vec<NodeT>>();
            // We mark as covered the central node and all of its neighbours.
            node_degrees_copy[node_id] = 0;
            // Since the central node is covered, the degree of all of its
            // neighbours must be decreased by one.
            covered_nodes.iter().for_each(|&node_id| {
                node_degrees_copy[node_id as usize] -= 1;
            });
        }

        info!(
            "Found {} clique roots.",
            to_human_readable_high_integer(clique_roots.len())
        );

        // Create the progress bar.
        let pb = get_loading_bar(verbose, "Computing graph cliques", clique_roots.len());
        // Actually compute and return cliques.
        Ok(clique_roots
            .into_iter()
            .progress_with(pb)
            .filter_map(move |node_id| {
                // First of all we find the degree of this node.
                let mut node_degree = node_degrees[node_id as usize];
                // We compute the neighbours of this node.
                let mut neighbours = unsafe {
                    self.iter_unchecked_unique_neighbour_node_ids_from_source_node_id(node_id)
                }
                .filter(|&dst| node_degrees[dst as usize] >= node_degree)
                .collect::<Vec<NodeT>>();
                // We start to find the cliques.
                let mut cliques = Vec::new();
                loop {
                    let mut tentative_clique = vec![];
                    let mut clique_neighbours = neighbours.clone();
                    while let Some((best_neighbour_node_id, shared_neighbours)) = clique_neighbours
                        .par_iter()
                        .cloned()
                        .filter_map(|neighbour_node_id| {
                            let shared_neighbours = iter_set::intersection(
                                unsafe {
                                    self.iter_unchecked_unique_neighbour_node_ids_from_source_node_id(
                                        neighbour_node_id,
                                    )
                                },
                                clique_neighbours.iter().cloned(),
                            ).collect::<Vec<NodeT>>();
                            if shared_neighbours.is_empty() {
                                None
                            } else {
                                Some((neighbour_node_id, shared_neighbours))
                            }
                        })
                        .max_by_key(|(_, a)| a.len())
                    {
                        clique_neighbours =shared_neighbours;
                        tentative_clique.push(best_neighbour_node_id);
                    }
                    if tentative_clique.is_empty() {
                        break;
                    }

                    node_degree = node_degree.min(tentative_clique.len() as NodeT - 1);

                    tentative_clique.push(node_id);

                    // Reduce the size of the degree of the nodes in the clique
                    // by the number of nodes in the clique, except for themselves.
                    tentative_clique.iter().for_each(|&node_in_clique|{
                        node_degrees[node_in_clique as usize] -= node_degree;
                    });

                    if tentative_clique.len() < minimum_clique_size as usize {
                        break;
                    }

                    cliques.push(tentative_clique);
                    node_degree = node_degrees[node_id as usize];
                    if cliques.len() == clique_per_node || node_degree == 0 {
                        break;
                    }
                    // We remove from the node's neighbours
                    // the nodes that now have a smaller degree.
                    neighbours.retain(|&dst| {
                        node_degrees[dst as usize] >= node_degree
                    });
                }
                // Expand the isomorphic groups in the cliques.
                Some(
                    cliques
                        .into_iter()
                        .filter(|clique| clique.len() > minimum_degree as usize)
                        .collect::<Vec<Vec<NodeT>>>(),
                )
            })
            .flat_map(move |cliques| {
                cliques
                    .into_iter()
                    .map(move |clique| Clique::from_node_ids(self, clique))
            }))
    }

    /// Returns graph cliques with at least `minimum_degree` nodes.
    ///
    /// # Arguments
    /// `minimum_degree`: Option<NodeT> - The optional minimum degree, by default 10.
    /// `minimum_clique_size`: Option<NodeT> - The optional minimum clique size, by default 10.
    /// `clique_per_node`: Option<usize> - Maximum number of clique to find for each node.
    /// `verbose`: Option<bool> - Whether to show a loading bar. By default, True.
    ///
    /// # Raises
    /// * If the current graph is directed.
    pub fn get_approximated_cliques(
        &self,
        minimum_degree: Option<NodeT>,
        minimum_clique_size: Option<NodeT>,
        clique_per_node: Option<usize>,
        verbose: Option<bool>,
    ) -> Result<Vec<Clique>> {
        Ok(self
            .iter_approximated_cliques(
                minimum_degree,
                minimum_clique_size,
                clique_per_node,
                verbose,
            )?
            .collect())
    }

    /// Returns the maximum clique in the graph.
    ///
    /// # Raises
    /// * If the current graph is directed.
    pub fn get_max_clique(&self) -> Result<Clique> {
        let minimum_node_degree = self.get_minimum_node_degree()?;
        let mut threshold = self.get_maximum_node_degree()? / 10;
        let mut candidate_maximal_clique = loop {
            if let Some(clique) = self
                .iter_approximated_cliques(Some(threshold), Some(threshold), Some(1), Some(false))?
                .take(1)
                .collect::<Vec<Clique>>()
                .first()
            {
                break clique.to_owned();
            }
            threshold /= 2;
            if minimum_node_degree > threshold || threshold == 1 {
                return Err("Impossible to detect the maximum clique in this graph.".to_string());
            }
        };
        while let Some(clique) = self
            .iter_approximated_cliques(
                Some(candidate_maximal_clique.len()),
                Some(candidate_maximal_clique.len() + 1),
                Some(1),
                Some(false),
            )?
            .take(1)
            .collect::<Vec<Clique>>()
            .first()
        {
            candidate_maximal_clique = clique.to_owned();
        }
        Ok(candidate_maximal_clique)
    }

    /// Returns number of graph cliques with at least `minimum_degree` nodes.
    ///
    /// # Arguments
    /// `minimum_degree`: Option<NodeT> - The optional minimum degree, by default 10.
    /// `minimum_clique_size`: Option<NodeT> - The optional minimum clique size, by default 10.
    /// `clique_per_node`: Option<usize> - Maximum number of clique to find for each node.
    /// `verbose`: Option<bool> - Whether to show a loading bar. By default, True.
    ///
    /// # Raises
    /// * If the current graph is directed.
    pub fn get_approximated_number_of_cliques(
        &self,
        minimum_degree: Option<NodeT>,
        minimum_clique_size: Option<NodeT>,
        clique_per_node: Option<usize>,
        verbose: Option<bool>,
    ) -> Result<usize> {
        Ok(self
            .iter_approximated_cliques(
                minimum_degree,
                minimum_clique_size,
                clique_per_node,
                verbose,
            )?
            .count())
    }
}
