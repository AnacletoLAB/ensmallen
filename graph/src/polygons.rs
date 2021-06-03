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
    /// # Arguments
    /// * `normalize`: Option<bool> - Whether to normalize the number of triangles.
    /// * `verbose`: Option<bool> - Whether to show a loading bar.
    ///
    /// # References
    /// This implementation is described in ["Faster Clustering Coefficient Using Vertex Covers"](https://ieeexplore.ieee.org/document/6693348).
    ///
    /// # Safety
    /// This method will raise a panic if called on an directed graph as those
    /// instances are not supported by this method.
    unsafe fn get_unweighted_undirected_number_of_triangles(
        &self,
        normalize: Option<bool>,
        verbose: Option<bool>,
    ) -> EdgeT {
        // The current graph must be undirected.
        if self.is_directed() {
            panic!("This method cannot be called on directed graphs!");
        }
        let verbose = verbose.unwrap_or(true);

        // By default we want to normalize the triangles number
        let normalize = normalize.unwrap_or(true);
        // First, we compute the set of nodes composing a vertex cover set.
        // This vertex cover is NOT minimal, but is a 2-approximation.
        let vertex_cover_set = self.approximated_vertex_cover_set();
        let pb = get_loading_bar(
            verbose,
            "Computing number of triangles",
            vertex_cover_set.len(),
        );
        // We start iterating over the nodes in the cover using rayon to parallelize the procedure.
        let mut number_of_triangles = vertex_cover_set
            .par_iter()
            .progress_with(pb)
            // For each node in the cover
            .map(|&node_id| unsafe {
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
                    // If the neighbour either is a selfloop
                    // or is not present in the vertex cover
                    // we return 0 new triangles.
                    .filter(|&neighbour_node_id| vertex_cover_set.contains(&neighbour_node_id))
                    .map(|&neighbour_node_id| {
                        // We compute the intersection of the neighbours.
                        iter_set::intersection(
                            neighbours.iter().cloned(),
                            self.iter_unchecked_neighbour_node_ids_from_source_node_id(
                                neighbour_node_id,
                            ),
                        )
                        .filter(|&inner_neighbour_id| inner_neighbour_id != neighbour_node_id)
                        .into_iter()
                        .map(|inner_node_id| {
                            // If the inner node is as well in the vertex cover
                            // we only count this as one, as we will encounter
                            // combinations of these nodes multiple times
                            // while iterating the vertex cover nodes
                            if vertex_cover_set.contains(&inner_node_id) {
                                1
                            } else {
                                // Otherwise we won't encounter again this
                                // node and we need to count the triangles
                                // three times.
                                3
                            }
                        })
                        .sum::<EdgeT>()
                    })
                    .sum::<EdgeT>()
            })
            .sum::<EdgeT>();
        assert!(number_of_triangles % 3 == 0);
        if normalize {
            number_of_triangles /= 3;
        }
        number_of_triangles
    }

    /// Returns number of triangles in the graph without taking into account the weights.
    ///
    /// This is a naive implementation and is considerably less efficient
    /// than Bader's version in the case of undirected graphs.
    ///
    /// # Arguments
    /// * `verbose`: Option<bool> - Whether to show a loading bar.
    ///
    /// # Safety
    /// This method will raise a panic if called on an undirected graph becase
    /// there is a more efficient one for these cases.
    /// There is a method that automatically dispatches the more efficient method
    /// according to the instance.
    unsafe fn get_unweighted_naive_number_of_triangles(&self, verbose: Option<bool>) -> EdgeT {
        if !self.is_directed() {
            panic!("This method should not be called on undirected graphs! Use the efficient one!");
        }
        let verbose = verbose.unwrap_or(true);
        let pb = get_loading_bar(
            verbose,
            "Computing number of triangles",
            self.get_nodes_number() as usize,
        );
        // We start iterating over the nodes using rayon to parallelize the procedure.
        let number_of_triangles: EdgeT = self
            .par_iter_node_ids()
            .progress_with(pb)
            // For each node in the cover
            .map(|node_id| unsafe {
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
    /// # Arguments
    /// * `normalize`: Option<bool> - Whether to normalize the number of triangles.
    /// * `verbose`: Option<bool> - Whether to show a loading bar.
    pub fn get_unweighted_number_of_triangles(
        &self,
        normalize: Option<bool>,
        verbose: Option<bool>,
    ) -> EdgeT {
        if self.is_directed() {
            unsafe { self.get_unweighted_naive_number_of_triangles(verbose) }
        } else {
            unsafe { self.get_unweighted_undirected_number_of_triangles(normalize, verbose) }
        }
    }

    /// Returns total number of triads in the graph without taking into account weights.
    pub fn get_unweighted_triads_number(&self) -> EdgeT {
        self.par_iter_unweighted_node_degrees()
            .map(|degree| (degree * degree.saturating_sub(1)) as EdgeT)
            .sum()
    }

    /// Returns total number of triads in the weighted graph.
    pub fn get_weighted_triads_number(&self) -> Result<f64, String> {
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
    /// * `verbose`: Option<bool> - Whether to show a loading bar.
    pub fn get_unweighted_transitivity(&self, verbose: Option<bool>) -> f64 {
        self.get_unweighted_number_of_triangles(Some(false), verbose) as f64
            / self.get_unweighted_triads_number() as f64
    }

    /// Returns number of triangles for all nodes in the graph.
    ///
    /// # Arguments
    /// * `normalize`: Option<bool> - Whether to normalize the number of triangles.
    /// * `verbose`: Option<bool> - Whether to show a loading bar.
    ///
    /// # References
    /// This implementation is described in ["Faster Clustering Coefficient Using Vertex Covers"](https://ieeexplore.ieee.org/document/6693348).
    ///
    /// # Safety
    /// This method does not support directed graphs and will raise a panic.
    /// It should automatically dispatched the naive version for these cases.
    unsafe fn get_unweighted_undirected_number_of_triangles_per_node(
        &self,
        normalize: Option<bool>,
        verbose: Option<bool>,
    ) -> Vec<NodeT> {
        if self.is_directed() {
            panic!("This method does not work for directed graphs!");
        }
        let normalize = normalize.unwrap_or(true);
        let node_triangles_number = self
            .iter_node_ids()
            .map(|_| AtomicU32::new(0))
            .collect::<Vec<_>>();
        let verbose = verbose.unwrap_or(true);
        let vertex_cover_set = self.approximated_vertex_cover_set();
        let pb = get_loading_bar(
            verbose,
            "Computing number of triangles per node",
            vertex_cover_set.len(),
        );
        // We start iterating over the nodes in the cover using rayon to parallelize the procedure.
        vertex_cover_set
            .par_iter()
            .progress_with(pb)
            // For each node in the cover
            .for_each(|&node_id| unsafe {
                // We obtain the neighbours and collect them into a vector
                // We store them instead of using them in a stream because we will need
                // them multiple times below.
                let neighbours = self
                    .iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                    .filter(|&neighbour_node_id| node_id != neighbour_node_id)
                    .collect::<Vec<NodeT>>();
                // We iterate over the neighbours
                neighbours.iter().for_each(|&neighbour_node_id| {
                    // If the neighbour either is a selfloop
                    // or is not present in the vertex cover
                    // we return 0 new triangles.
                    if vertex_cover_set.contains(&neighbour_node_id) {
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
                            node_triangles_number[node_id as usize].fetch_add(1, Ordering::Relaxed);
                            if !vertex_cover_set.contains(&inner_node_id) {
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
    ///
    /// # Safety
    /// This method will raise a panic if called on an directed graph becase
    /// there is a more efficient one for these cases.
    unsafe fn get_unweighted_naive_number_of_triangles_per_node(
        &self,
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
        let pb = get_loading_bar(
            verbose,
            "Computing number of triangles per node",
            self.get_nodes_number() as usize,
        );
        // We start iterating over the nodes using rayon to parallelize the procedure.
        self.par_iter_node_ids()
            .progress_with(pb)
            // For each node in the cover
            .for_each(|node_id| unsafe {
                // We obtain the neighbours and collect them into a vector
                // We store them instead of using them in a stream because we will need
                // them multiple times below.
                let neighbours = self
                    .iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                    .filter(|&neighbour_node_id| node_id != neighbour_node_id)
                    .collect::<Vec<NodeT>>();
                // We iterate over the neighbours
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
                            .filter(|&inner_neighbour_id| inner_neighbour_id != neighbour_node_id)
                            .count() as NodeT,
                            Ordering::Relaxed,
                        );
                    });
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
    /// * `verbose`: Option<bool> - Whether to show a loading bar.
    ///
    pub fn get_unweighted_number_of_triangles_per_node(
        &self,
        normalize: Option<bool>,
        verbose: Option<bool>,
    ) -> Vec<NodeT> {
        if self.is_directed() {
            unsafe { self.get_unweighted_naive_number_of_triangles_per_node(verbose) }
        } else {
            unsafe {
                self.get_unweighted_undirected_number_of_triangles_per_node(normalize, verbose)
            }
        }
    }

    /// Returns iterator over the clustering coefficients for all nodes in the graph.
    ///
    /// # Arguments
    /// * `verbose`: Option<bool> - Whether to show a loading bar.
    ///
    /// # References
    /// This implementation is described in ["Faster Clustering Coefficient Using Vertex Covers"](https://ieeexplore.ieee.org/document/6693348).
    pub fn iter_clustering_coefficient_per_node(
        &self,
        verbose: Option<bool>,
    ) -> impl IndexedParallelIterator<Item = f64> + '_ {
        self.get_unweighted_number_of_triangles_per_node(Some(false), verbose)
            .into_par_iter()
            .zip(self.par_iter_unweighted_node_degrees())
            .map(|(triangles_number, degree)| {
                if degree < 2 {
                    0.0
                } else {
                    triangles_number as f64 / (degree * (degree - 1)) as f64
                }
            })
    }

    /// Returns clustering coefficients for all nodes in the graph.
    ///
    /// # Arguments
    /// * `verbose`: Option<bool> - Whether to show a loading bar.
    ///
    /// # References
    /// This implementation is described in ["Faster Clustering Coefficient Using Vertex Covers"](https://ieeexplore.ieee.org/document/6693348).
    pub fn get_clustering_coefficient_per_node(&self, verbose: Option<bool>) -> Vec<f64> {
        self.iter_clustering_coefficient_per_node(verbose).collect()
    }

    /// Returns the graph clustering coefficient.
    ///
    /// # Arguments
    /// * `verbose`: Option<bool> - Whether to show a loading bar.
    ///
    /// # References
    /// This implementation is described in ["Faster Clustering Coefficient Using Vertex Covers"](https://ieeexplore.ieee.org/document/6693348).
    pub fn get_clustering_coefficient(&self, verbose: Option<bool>) -> f64 {
        self.iter_clustering_coefficient_per_node(verbose).sum()
    }

    /// Returns the graph average clustering coefficient.
    ///
    /// # Arguments
    /// * `verbose`: Option<bool> - Whether to show a loading bar.
    ///
    /// # References
    /// This implementation is described in ["Faster Clustering Coefficient Using Vertex Covers"](https://ieeexplore.ieee.org/document/6693348).
    pub fn get_average_clustering_coefficient(&self, verbose: Option<bool>) -> f64 {
        self.get_clustering_coefficient(verbose) / self.get_nodes_number() as f64
    }
}
