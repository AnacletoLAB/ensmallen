use super::*;
use log::info;
use rayon::prelude::*;
use std::collections::HashSet;
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
                "Clique containing {nodes_number} nodes. ",
                "Specifically, the nodes involved in the clique are: {nodes}.",
                "{node_types_counts}",
                "{edge_types_counts}",
                "</p>",
            ),
            nodes_number = to_human_readable_high_integer(self.len() as usize),
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
                            format!(
                                " Its nodes have {}.",
                                self.graph
                                    .get_unchecked_node_types_description_from_count(count)
                            )
                        },
                    )
            },
            edge_types_counts = unsafe {
                self.graph
                    .get_unchecked_edge_type_id_counts_hashmap_from_node_ids(self.node_ids.as_ref())
                    .map_or_else(
                        |_| "".to_string(),
                        |count| {
                            format!(
                                " Its edges have {}.",
                                self.graph
                                    .get_unchecked_edge_types_description_from_count(count)
                            )
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
    /// Returns parallel iterator over the graph cliques with at least `minimum_degree` nodes.
    ///
    /// # Arguments
    /// `minimum_degree`: Option<NodeT> - The optional minimum degree, by default 5.
    pub fn par_iter_cliques(
        &self,
        minimum_degree: Option<NodeT>,
    ) -> Result<impl ParallelIterator<Item = Clique> + '_> {
        self.must_be_undirected()?;
        // First of all we set the minimum degree, which if None were provided is set to 5.
        let minimum_degree = minimum_degree.unwrap_or(5);
        // We create a vector with the initial node degrees of the graph, wrapped into atomic.
        info!("Create the node degrees vector.");
        let mut node_degrees: Vec<AtomicU32> = Vec::with_capacity(self.get_nodes_number() as usize);
        self.par_iter_node_degrees()
            .map(|degree| AtomicU32::new(degree))
            .collect_into_vec(&mut node_degrees);
        // Start to iterate over the node degrees vector
        // and in every iteration remove the nodes with degree smaller than
        // the provided amount. We iterate until no edit operation is
        // done.
        info!("Start iteration.");
        let mut current_iteration = 0;
        let mut total_removed_nodes = 0;
        loop {
            let currently_removed_nodes = self
                .par_iter_node_ids()
                .map(|node_id| {
                    // We retrieve the current node degree.
                    let degree = node_degrees[node_id as usize].load(Ordering::Relaxed);
                    // If the degree is zero, we have already dropped this node from the game.
                    if degree == 0 || degree == NODE_NOT_PRESENT {
                        return 0;
                    }
                    // If the degree is higher than the minimum degree
                    if degree > minimum_degree {
                        // If this node has neighbours 
                        if unsafe{self.iter_unchecked_unique_neighbour_node_ids_from_source_node_id(node_id)}.any(|neighbour_node_id|{
                            neighbour_node_id > node_id
                        }){
                            node_degrees[node_id as usize].store(NODE_NOT_PRESENT, Ordering::Relaxed);
                            return 1;
                        }
                        // If we are in a node that has, currently, degree higher or equal
                        // than the minimum degree, we check if is NOT inside a clique
                        // of size at least equal to the provided minimum degree.
                        let neighbours = unsafe {
                            self.iter_unchecked_unique_neighbour_node_ids_from_source_node_id(node_id)
                        }
                        .filter(|&dst| {
                            let degree = node_degrees[dst as usize].load(Ordering::Relaxed);
                            degree >= minimum_degree && degree != NODE_NOT_PRESENT
                        })
                        .collect::<HashSet<NodeT>>();

                        // To prove that this node is NOT inside a clique of the desired
                        // minimum size, we need to check its neighbours and find at
                        // least `neighbours.len() - not_shared_neighbours < minimum_degree`.
                        if (neighbours
                            .iter()
                            .filter(|&&neighbour_node_id| {
                                unsafe {
                                    self.iter_unchecked_unique_neighbour_node_ids_from_source_node_id(
                                        neighbour_node_id,
                                    )
                                }
                                .filter(|dst| {
                                    let degree = node_degrees[*dst as usize].load(Ordering::Relaxed);
                                    degree >= minimum_degree && degree != NODE_NOT_PRESENT && neighbours.contains(dst)
                                })
                                .take(minimum_degree as usize)
                                .count() as NodeT
                                    == minimum_degree
                            })
                            .take(minimum_degree as usize)
                            .count() as NodeT)
                            == minimum_degree
                        {
                            // We keep the node.
                            return 0;
                        }
                    }
                    node_degrees[node_id as usize].store(0, Ordering::Relaxed);
                    unsafe { self.iter_unchecked_unique_neighbour_node_ids_from_source_node_id(node_id) }
                        .filter(|&dst| {
                            node_degrees[dst as usize]
                                .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |mut degree| {
                                    if degree > 0 && degree != NODE_NOT_PRESENT {
                                        degree -= 1;
                                    }
                                    Some(degree)
                                })
                                .unwrap()
                                == 1
                        })
                        .count() as NodeT
                        + 1
                })
                .sum::<NodeT>();
            if currently_removed_nodes == 0 {
                break;
            }
            total_removed_nodes += currently_removed_nodes;
            info!(
                "Iteration #{current_iteration}: removed {currently_removed_nodes} more nodes, {percentage:.2}% out of total.",
                current_iteration=current_iteration,
                currently_removed_nodes=to_human_readable_high_integer(currently_removed_nodes as usize),
                percentage = total_removed_nodes as f64 / self.get_nodes_number() as f64 * 100.0
            );
            current_iteration += 1;
        }
        info!(
            "Searching cliques in the remaining {} nodes",
            to_human_readable_high_integer(
                (self.get_nodes_number() - total_removed_nodes) as usize
            )
        );
        // Convert the node degrees atomic vector into normal U32 values.
        let node_degrees =
            unsafe { std::mem::transmute::<Vec<AtomicU32>, Vec<NodeT>>(node_degrees) };
        // Actually compute and return cliques.
        Ok(self
            .par_iter_node_ids()
            .filter_map(move |node_id| {
                if node_degrees[node_id as usize] == 0 {
                    return None;
                }
                let neighbours = unsafe {
                    self.iter_unchecked_unique_neighbour_node_ids_from_source_node_id(node_id)
                }
                .filter(|&dst| node_degrees[dst as usize] > 0)
                .collect::<Vec<NodeT>>();
                // The cliques vectors starts out as the root node (implicit) and the first
                // neighbour of the root node.
                let mut cliques: Vec<Vec<NodeT>> = vec![vec![neighbours[0]]];
                // We iterate over the neighbours
                neighbours.into_iter().skip(1).for_each(|node_id| {
                    // If the current neighbour wont fit in any of the other cliques
                    // we need to prepare a vector where to store its current matches.
                    let mut possible_new_cliques = Vec::new();
                    // We start to iterate over the existing growing cliques.
                    let number_of_matches = cliques
                        .iter_mut()
                        .map(|clique| unsafe {
                            // We count the number of matches in the current clique.
                            let matches = iter_set::intersection(
                                clique.iter().cloned(),
                                self.iter_unchecked_unique_neighbour_node_ids_from_source_node_id(
                                    node_id,
                                )
                                .filter(|&dst| node_degrees[dst as usize] > 0),
                            )
                            .collect::<Vec<NodeT>>();
                            // If we have a perfect match we can add the current
                            // node to this clique. Note that we cannot stop
                            // at this point, as the node may be shared between
                            // multiple cliques.
                            if matches.len() == clique.len() {
                                clique.push(node_id);
                                clique.sort_unstable();
                                1
                            // Otherwise if the match is not perfect but we still
                            // have some matches we need to store these matches
                            // in the new clique we are growing for this node.
                            } else if matches.len() > 0 {
                                possible_new_cliques.push(matches);
                                0
                            } else {
                                0
                            }
                        })
                        .sum::<usize>();
                    // If the total number of matches is zero
                    if number_of_matches == 0 {
                        // We add the current node to the currently growing clique
                        if possible_new_cliques.is_empty() {
                            possible_new_cliques.push(Vec::new());
                        }
                        possible_new_cliques.iter_mut().for_each(|clique| {
                            clique.push(node_id);
                            clique.sort_unstable();
                        });
                        // and push the clique to the set of cliques.
                        cliques.extend(possible_new_cliques);
                    }
                });
                Some(
                    cliques
                        .into_iter()
                        .filter_map(|mut clique| {
                            if (clique.len() as NodeT) < minimum_degree {
                                None
                            } else {
                                clique.push(node_id);
                                Some(clique)
                            }
                        })
                        .collect::<Vec<Vec<NodeT>>>(),
                )
            })
            .flat_map(move |cliques| {
                cliques
                    .into_par_iter()
                    .map(move |clique| Clique::from_node_ids(self, clique))
            }))
    }

    /// Returns graph cliques with at least `minimum_degree` nodes.
    ///
    /// # Arguments
    /// `minimum_degree`: Option<NodeT> - The optional minimum degree, by default 5.
    pub fn get_cliques(&self, minimum_degree: Option<NodeT>) -> Result<Vec<Clique>> {
        Ok(self.par_iter_cliques(minimum_degree)?.collect())
    }

    /// Returns number of graph cliques with at least `minimum_degree` nodes.
    ///
    /// # Arguments
    /// `minimum_degree`: Option<NodeT> - The optional minimum degree, by default 5.
    pub fn get_cliques_number(&self, minimum_degree: Option<NodeT>) -> Result<usize> {
        Ok(self.par_iter_cliques(minimum_degree)?.count())
    }
}
