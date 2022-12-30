use super::*;
use indicatif::ParallelProgressIterator;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;
use std::sync::atomic::{AtomicU32, Ordering};

impl Graph {
    /// Returns number of triangles in the graph.
    ///
    /// # References
    /// This implementation is described in ["Faster Clustering Coefficient Using Vertex Covers"](https://ieeexplore.ieee.org/document/6693348).
    ///
    /// # Safety
    /// This method will raise a panic if called on an directed graph as those
    /// instances are not supported by this method.
    unsafe fn get_undirected_number_of_triangles(&self) -> EdgeT {
        // The current graph must be undirected.
        if self.is_directed() {
            panic!("This method cannot be called on directed graphs!");
        }

        // First, we compute the set of nodes composing a vertex cover set.
        // This vertex cover is NOT minimal, but is a 2-approximation.
        let vertex_cover = self
            .get_approximated_vertex_cover(
                Some("decreasing_node_degree"),
                Some(true),
                Some(true),
                None,
            )
            .unwrap();

        let vertex_cover_reference = vertex_cover.as_slice();

        // We start iterating over the nodes in the cover using rayon to parallelize the procedure.
        vertex_cover
            .par_iter()
            .enumerate()
            .filter_map(|(node_id, is_cover)| {
                if *is_cover {
                    Some(node_id as NodeT)
                } else {
                    None
                }
            })
            // For each node in the cover
            .flat_map(|node_id| {
                // We obtain the neighbours and collect them into a vector
                // We store them instead of using them in a stream because we will need
                // them multiple times below.
                let first_order_neighbours = self
                    .edges
                    .get_unchecked_neighbours_node_ids_from_src_node_id(node_id);

                first_order_neighbours
                    .par_iter()
                    .filter_map(move |&neighbour_node_id| {
                        if neighbour_node_id != node_id
                            && vertex_cover_reference[neighbour_node_id as usize]
                        {
                            Some((node_id, neighbour_node_id, first_order_neighbours))
                        } else {
                            None
                        }
                    })
            })
            .map(|(node_id, neighbour_node_id, first_order_neighbours)| {
                // We iterate over the neighbours
                // We compute the intersection of the neighbours.

                let mut first_neighbour_index = 0;
                let mut second_neighbour_index = 0;
                let mut partial_number_of_triangles: EdgeT = 0;

                let second_order_neighbours = self
                    .edges
                    .get_unchecked_neighbours_node_ids_from_src_node_id(neighbour_node_id);

                while first_neighbour_index < first_order_neighbours.len()
                    && second_neighbour_index < second_order_neighbours.len()
                {
                    /*
                    let first_order_neighbour = first_order_neighbours[first_neighbour_index];
                    // If this is a self-loop, we march on forward
                    if first_order_neighbour == neighbour_node_id
                        || first_order_neighbour == node_id
                    {
                        first_neighbour_index += 1;
                        continue;
                    }
                    // If this is not an intersection, we march forward
                    let second_order_neighbour = second_order_neighbours[second_neighbour_index];
                    if first_order_neighbour < second_order_neighbour {
                        first_neighbour_index += 1;
                        continue;
                    }
                    if first_order_neighbour > second_order_neighbour {
                        second_neighbour_index += 1;
                        continue;
                    }
                    // If we reach here, we are in an intersection.
                    first_neighbour_index += 1;
                    second_neighbour_index += 1;
                    // If the inner node is as well in the vertex cover
                    // we only count this as one, as we will encounter
                    // combinations of these nodes multiple times
                    // while iterating the vertex cover nodes
                    partial_number_of_triangles +=
                        if vertex_cover_reference[first_order_neighbour as usize] {
                            1
                        } else {
                            // Otherwise we won't encounter again this
                            // node and we need to count the triangles
                            // three times.
                            3
                        }; 
                    */

                    let first_order_neighbour = first_order_neighbours[first_neighbour_index];
                    let second_order_neighbour = second_order_neighbours[second_neighbour_index];

                    let is_tuple = first_order_neighbour == neighbour_node_id
                        || first_order_neighbour == node_id;

                    first_neighbour_index += (is_tuple || first_order_neighbour <= second_order_neighbour) as usize;
                    second_neighbour_index += (!is_tuple && first_order_neighbour >= second_order_neighbour) as usize;
                    
                    partial_number_of_triangles +=
                    if vertex_cover_reference[first_order_neighbour as usize] {
                        1
                    } else {
                        // Otherwise we won't encounter again this
                        // node and we need to count the triangles
                        // three times.
                        3
                    }; 
                }
                partial_number_of_triangles
            })
            .sum::<EdgeT>() / 2
    }

    /// Returns number of triangles in the graph without taking into account the weights.
    ///
    /// This is a naive implementation and is considerably less efficient
    /// than Bader's version in the case of undirected graphs.
    ///
    /// # Safety
    /// This method will raise a panic if called on an undirected graph becase
    /// there is a more efficient one for these cases.
    /// There is a method that automatically dispatches the more efficient method
    /// according to the instance.
    unsafe fn get_naive_number_of_triangles(&self) -> EdgeT {
        if !self.is_directed() {
            panic!("This method should not be called on undirected graphs! Use the efficient one!");
        }
        // We start iterating over the nodes using rayon to parallelize the procedure.
        let number_of_triangles: EdgeT = self
            .par_iter_node_ids()
            // For each node in the cover
            .map(|node_id| {
                // We obtain the neighbours and collect them into a vector
                // We store them instead of using them in a stream because we will need
                // them multiple times below.
                let neighbours = self
                    .iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                    .filter(|&neighbour_node_id| node_id != neighbour_node_id)
                    .collect::<Vec<NodeT>>();
                // We iterate over the neighbours
                neighbours
                    .par_iter()
                    // If the neighbour is a selfloop
                    // we return 0 new triangles.
                    .map(|&neighbour_node_id| {
                        // We compute the intersection of the neighbours.
                        iter_set::intersection(
                            neighbours.iter().cloned(),
                            self.iter_unchecked_neighbour_node_ids_from_source_node_id(
                                neighbour_node_id,
                            ),
                        )
                        .filter(|&inner_neighbour_id| inner_neighbour_id != neighbour_node_id)
                        .count() as EdgeT
                    })
                    .sum::<EdgeT>()
            })
            .sum::<EdgeT>();
        number_of_triangles
    }

    /// Returns total number of triangles ignoring the weights.
    ///
    /// The method dispatches the fastest method according to the current
    /// graph instance. Specifically:
    /// - For directed graphs it will use the naive algorithm.
    /// - For undirected graphs it will use Bader's version.
    ///
    pub fn get_number_of_triangles(&self) -> EdgeT {
        if self.is_directed() {
            unsafe { self.get_naive_number_of_triangles() }
        } else {
            unsafe { self.get_undirected_number_of_triangles() }
        }
    }

    /// Returns total number of triads in the graph without taking into account weights.
    pub fn get_number_of_triads(&self) -> EdgeT {
        self.par_iter_node_degrees()
            .map(|degree| (degree as EdgeT) * (degree.saturating_sub(1) as EdgeT))
            .sum()
    }

    /// Returns total number of triads in the weighted graph.
    pub fn get_number_of_weighted_triads(&self) -> Result<f64> {
        Ok(self
            .par_iter_weighted_node_degrees()?
            .map(|degree| {
                if degree > 1.0 {
                    degree * (degree - 1.0)
                } else {
                    0.0
                }
            })
            .sum())
    }

    /// Returns transitivity of the graph without taking into account weights.
    ///
    /// # Arguments
    /// * `low_centrality`: Option<usize> - The threshold over which to switch to parallel matryoshka. By default 50.
    /// * `verbose`: Option<bool> - Whether to show a loading bar.
    pub fn get_transitivity(&self) -> f64 {
        self.get_number_of_triangles() as f64 / self.get_number_of_triads() as f64
    }

    /// Returns number of triangles for all nodes in the graph.
    ///
    /// # Arguments
    /// * `normalize`: Option<bool> - Whether to normalize the number of triangles.
    /// * `low_centrality`: Option<usize> - The threshold over which to switch to parallel matryoshka. By default 50.
    /// * `verbose`: Option<bool> - Whether to show a loading bar.
    ///
    /// # References
    /// This implementation is described in ["Faster Clustering Coefficient Using Vertex Covers"](https://ieeexplore.ieee.org/document/6693348).
    ///
    /// # Safety
    /// This method does not support directed graphs and will raise a panic.
    /// It should automatically dispatched the naive version for these cases.
    unsafe fn get_undirected_number_of_triangles_per_node(
        &self,
        normalize: Option<bool>,
        low_centrality: Option<usize>,
        verbose: Option<bool>,
    ) -> Vec<NodeT> {
        if self.is_directed() {
            panic!("This method does not work for directed graphs!");
        }
        let normalize = normalize.unwrap_or(true);
        let low_centrality = low_centrality.unwrap_or(50);
        let node_triangles_number = self
            .iter_node_ids()
            .map(|_| AtomicU32::new(0))
            .collect::<Vec<_>>();
        let verbose = verbose.unwrap_or(true);
        let vertex_cover = self
            .get_approximated_vertex_cover(None, None, None, None)
            .unwrap();
        let cover_size = vertex_cover
            .par_iter()
            .filter(|&&is_cover| is_cover)
            .count();
        let pb = get_loading_bar(
            verbose,
            "Computing number of triangles per node",
            cover_size,
        );
        // We start iterating over the nodes in the cover using rayon to parallelize the procedure.
        vertex_cover
            .par_iter()
            .enumerate()
            .filter_map(|(node_id, is_cover)| {
                if *is_cover {
                    Some(node_id as NodeT)
                } else {
                    None
                }
            })
            .progress_with(pb)
            // For each node in the cover
            .for_each(|node_id| {
                // We obtain the neighbours and collect them into a vector
                // We store them instead of using them in a stream because we will need
                // them multiple times below.
                let neighbours = self
                    .iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                    .filter(|&neighbour_node_id| node_id != neighbour_node_id)
                    .collect::<Vec<NodeT>>();
                // We iterate over the neighbours
                // TODO! find a way to do this without duplicating the code!
                if neighbours.len() < low_centrality {
                    neighbours.iter().for_each(|&neighbour_node_id| {
                        // If the neighbour either is a selfloop
                        // or is not present in the vertex cover
                        // we return 0 new triangles.
                        if vertex_cover[neighbour_node_id as usize] {
                            // We compute the intersection of the neighbours.
                            iter_set::intersection(
                                neighbours.iter().cloned(),
                                self.iter_unchecked_neighbour_node_ids_from_source_node_id(
                                    neighbour_node_id,
                                ),
                            )
                            .filter(|&inner_node_id| inner_node_id != neighbour_node_id)
                            .into_iter()
                            .for_each(|inner_node_id| {
                                // If the inner node is as well in the vertex cover
                                // we only count this as one, as we will encounter
                                // combinations of these nodes multiple times
                                // while iterating the vertex cover nodes
                                node_triangles_number[node_id as usize]
                                    .fetch_add(1, Ordering::Relaxed);
                                if !vertex_cover[inner_node_id as usize] {
                                    // Otherwise we won't encounter again this
                                    // node and we need to count the triangles
                                    // three times.
                                    node_triangles_number[neighbour_node_id as usize]
                                        .fetch_add(1, Ordering::Relaxed);
                                    node_triangles_number[inner_node_id as usize]
                                        .fetch_add(1, Ordering::Relaxed);
                                }
                            });
                        }
                    });
                } else {
                    neighbours.par_iter().for_each(|&neighbour_node_id| {
                        // If the neighbour either is a selfloop
                        // or is not present in the vertex cover
                        // we return 0 new triangles.
                        if vertex_cover[neighbour_node_id as usize] {
                            // We compute the intersection of the neighbours.
                            iter_set::intersection(
                                neighbours.iter().cloned(),
                                self.iter_unchecked_neighbour_node_ids_from_source_node_id(
                                    neighbour_node_id,
                                ),
                            )
                            .filter(|&inner_node_id| inner_node_id != neighbour_node_id)
                            .into_iter()
                            .for_each(|inner_node_id| {
                                // If the inner node is as well in the vertex cover
                                // we only count this as one, as we will encounter
                                // combinations of these nodes multiple times
                                // while iterating the vertex cover nodes
                                node_triangles_number[node_id as usize]
                                    .fetch_add(1, Ordering::Relaxed);
                                if !vertex_cover[inner_node_id as usize] {
                                    // Otherwise we won't encounter again this
                                    // node and we need to count the triangles
                                    // three times.
                                    node_triangles_number[neighbour_node_id as usize]
                                        .fetch_add(1, Ordering::Relaxed);
                                    node_triangles_number[inner_node_id as usize]
                                        .fetch_add(1, Ordering::Relaxed);
                                }
                            });
                        }
                    });
                }
            });
        let mut node_triangles_number =
            std::mem::transmute::<Vec<AtomicU32>, Vec<NodeT>>(node_triangles_number);
        if normalize {
            node_triangles_number
                .par_iter_mut()
                .for_each(|triangles_number| {
                    *triangles_number /= 2;
                });
        }
        node_triangles_number
    }

    /// Returns number of triangles in the graph without taking into account the weights.
    ///
    /// This is a naive implementation and is considerably less efficient
    /// than Bader's version in the case of undirected graphs.
    ///
    /// # Arguments
    /// * `low_centrality`: Option<usize> - The threshold over which to switch to parallel matryoshka. By default 50.
    /// * `verbose`: Option<bool> - Whether to show a loading bar.
    ///
    /// # Safety
    /// This method will raise a panic if called on an directed graph becase
    /// there is a more efficient one for these cases.
    unsafe fn get_naive_number_of_triangles_per_node(
        &self,
        low_centrality: Option<usize>,
        verbose: Option<bool>,
    ) -> Vec<NodeT> {
        if !self.is_directed() {
            panic!("This method should not be called on directed graphs as there is a more efficient one!");
        }
        // Number of nodes per triangles
        let node_triangles_number = self
            .iter_node_ids()
            .map(|_| AtomicU32::new(0))
            .collect::<Vec<_>>();
        let verbose = verbose.unwrap_or(true);
        let low_centrality = low_centrality.unwrap_or(50);
        let pb = get_loading_bar(
            verbose,
            "Computing number of triangles per node",
            self.get_number_of_nodes() as usize,
        );
        // We start iterating over the nodes using rayon to parallelize the procedure.
        self.par_iter_node_ids()
            .progress_with(pb)
            // For each node in the cover
            .for_each(|node_id| {
                // We obtain the neighbours and collect them into a vector
                // We store them instead of using them in a stream because we will need
                // them multiple times below.
                let neighbours = self
                    .iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                    .filter(|&neighbour_node_id| node_id != neighbour_node_id)
                    .collect::<Vec<NodeT>>();
                // We iterate over the neighbours
                // TODO! find a way to do this without duplicating the code!
                if neighbours.len() < low_centrality {
                    neighbours
                        .iter()
                        // If the neighbour is a selfloop
                        // we return 0 new triangles.
                        .for_each(|&neighbour_node_id| {
                            // We compute the intersection of the neighbours.
                            node_triangles_number[node_id as usize].fetch_add(
                                iter_set::intersection(
                                    neighbours.iter().cloned(),
                                    self.iter_unchecked_neighbour_node_ids_from_source_node_id(
                                        neighbour_node_id,
                                    ),
                                )
                                .filter(|&inner_neighbour_id| {
                                    inner_neighbour_id != neighbour_node_id
                                })
                                .count() as NodeT,
                                Ordering::Relaxed,
                            );
                        });
                } else {
                    neighbours
                        .par_iter()
                        // If the neighbour is a selfloop
                        // we return 0 new triangles.
                        .for_each(|&neighbour_node_id| {
                            // We compute the intersection of the neighbours.
                            node_triangles_number[node_id as usize].fetch_add(
                                iter_set::intersection(
                                    neighbours.iter().cloned(),
                                    self.iter_unchecked_neighbour_node_ids_from_source_node_id(
                                        neighbour_node_id,
                                    ),
                                )
                                .filter(|&inner_neighbour_id| {
                                    inner_neighbour_id != neighbour_node_id
                                })
                                .count() as NodeT,
                                Ordering::Relaxed,
                            );
                        });
                }
            });
        std::mem::transmute::<Vec<AtomicU32>, Vec<NodeT>>(node_triangles_number)
    }

    /// Returns number of triangles in the graph without taking into account the weights.
    ///
    /// The method dispatches the fastest method according to the current
    /// graph instance. Specifically:
    /// - For directed graphs it will use the naive algorithm.
    /// - For undirected graphs it will use Bader's version.
    ///
    /// # Arguments
    /// * `normalize`: Option<bool> - Whether to normalize the number of triangles.
    /// * `low_centrality`: Option<usize> - The threshold over which to switch to parallel matryoshka. By default 50.
    /// * `verbose`: Option<bool> - Whether to show a loading bar.
    ///
    pub fn get_number_of_triangles_per_node(
        &self,
        normalize: Option<bool>,
        low_centrality: Option<usize>,
        verbose: Option<bool>,
    ) -> Vec<NodeT> {
        if self.is_directed() {
            unsafe { self.get_naive_number_of_triangles_per_node(low_centrality, verbose) }
        } else {
            unsafe {
                self.get_undirected_number_of_triangles_per_node(normalize, low_centrality, verbose)
            }
        }
    }

    /// Returns iterator over the clustering coefficients for all nodes in the graph.
    ///
    /// # Arguments
    /// * `low_centrality`: Option<usize> - The threshold over which to switch to parallel matryoshka. By default 50.
    /// * `verbose`: Option<bool> - Whether to show a loading bar.
    ///
    /// # References
    /// This implementation is described in ["Faster Clustering Coefficient Using Vertex Covers"](https://ieeexplore.ieee.org/document/6693348).
    pub fn par_iter_clustering_coefficient_per_node(
        &self,
        low_centrality: Option<usize>,
        verbose: Option<bool>,
    ) -> impl IndexedParallelIterator<Item = f64> + '_ {
        self.get_number_of_triangles_per_node(Some(false), low_centrality, verbose)
            .into_par_iter()
            .zip(self.par_iter_node_degrees())
            .map(|(triangles_number, degree)| {
                if degree <= 1 {
                    0.0
                } else {
                    triangles_number as f64 / ((degree as EdgeT) * (degree as EdgeT - 1)) as f64
                }
            })
    }

    /// Returns clustering coefficients for all nodes in the graph.
    ///
    /// # Arguments
    /// * `low_centrality`: Option<usize> - The threshold over which to switch to parallel matryoshka. By default 50.
    /// * `verbose`: Option<bool> - Whether to show a loading bar.
    ///
    /// # References
    /// This implementation is described in ["Faster Clustering Coefficient Using Vertex Covers"](https://ieeexplore.ieee.org/document/6693348).
    pub fn get_clustering_coefficient_per_node(
        &self,
        low_centrality: Option<usize>,
        verbose: Option<bool>,
    ) -> Vec<f64> {
        self.par_iter_clustering_coefficient_per_node(low_centrality, verbose)
            .collect()
    }

    /// Returns the graph clustering coefficient.
    ///
    /// # Arguments
    /// * `low_centrality`: Option<usize> - The threshold over which to switch to parallel matryoshka. By default 50.
    /// * `verbose`: Option<bool> - Whether to show a loading bar.
    ///
    /// # References
    /// This implementation is described in ["Faster Clustering Coefficient Using Vertex Covers"](https://ieeexplore.ieee.org/document/6693348).
    pub fn get_clustering_coefficient(
        &self,
        low_centrality: Option<usize>,
        verbose: Option<bool>,
    ) -> f64 {
        self.par_iter_clustering_coefficient_per_node(low_centrality, verbose)
            .sum()
    }

    /// Returns the graph average clustering coefficient.
    ///
    /// # Arguments
    /// * `low_centrality`: Option<usize> - The threshold over which to switch to parallel matryoshka. By default 50.
    /// * `verbose`: Option<bool> - Whether to show a loading bar.
    ///
    /// # References
    /// This implementation is described in ["Faster Clustering Coefficient Using Vertex Covers"](https://ieeexplore.ieee.org/document/6693348).
    pub fn get_average_clustering_coefficient(
        &self,
        low_centrality: Option<usize>,
        verbose: Option<bool>,
    ) -> f64 {
        self.get_clustering_coefficient(low_centrality, verbose) / self.get_number_of_nodes() as f64
    }
}
