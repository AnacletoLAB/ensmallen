use super::*;
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
    ///
    /// # References
    /// This implementation is described in ["Faster Clustering Coefficient Using Vertex Covers"](https://ieeexplore.ieee.org/document/6693348).
    pub fn get_number_of_triangles(&self, normalize: Option<bool>) -> EdgeT {
        let normalize = normalize.unwrap_or(true);
        // First, we compute the set of nodes composing a vertex cover set.
        // This vertex cover is NOT minimal, but is a 2-approximation.
        let vertex_cover_set = self.approximated_vertex_cover_set();
        // We start iterating over the nodes in the cover using rayon to parallelize the procedure.
        let mut number_of_triangles = vertex_cover_set
            .par_iter()
            // For each node in the cover
            .map(|&node_id| {
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
        if normalize {
            number_of_triangles /= 3 * if self.is_directed() { 1 } else { 2 }
        }
        number_of_triangles
    }

    /// Returns total number of triads in the graph.
    pub fn get_triads_number(&self) -> EdgeT {
        self.par_iter_unweighted_node_degrees()
            .map(|degree| (degree * degree.saturating_sub(1)) as EdgeT)
            .sum()
    }

    /// Returns transitivity of the graph.
    pub fn get_transitivity(&self) -> f64 {
        self.get_number_of_triangles(Some(false)) as f64 / self.get_triads_number() as f64
    }

    /// Returns number of triangles for all nodes in the graph.
    ///
    /// # Arguments
    /// * `normalize`: Option<bool> - Whether to normalize the number of triangles.
    ///
    /// # References
    /// This implementation is described in ["Faster Clustering Coefficient Using Vertex Covers"](https://ieeexplore.ieee.org/document/6693348).
    pub fn get_number_of_triangles_per_node(&self, normalize: Option<bool>) -> Vec<NodeT> {
        let normalize = normalize.unwrap_or(true);
        // First, we compute the set of nodes composing a vertex cover set.
        // This vertex cover is NOT minimal, but is a 2-approximation.
        let vertex_cover_set = self.approximated_vertex_cover_set();
        let node_triangles_number = self
            .iter_node_ids()
            .map(|_| AtomicU32::new(0))
            .collect::<Vec<_>>();
        // We start iterating over the nodes in the cover using rayon to parallelize the procedure.
        vertex_cover_set
            .par_iter()
            // For each node in the cover
            .for_each(|&node_id| {
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
            unsafe { std::mem::transmute::<Vec<AtomicU32>, Vec<NodeT>>(node_triangles_number) };
        if normalize {
            node_triangles_number
                .par_iter_mut()
                .for_each(|triangles_number| {
                    *triangles_number /= 2;
                });
        }
        node_triangles_number
    }

    /// Returns iterator over the clustering coefficients for all nodes in the graph.
    ///
    /// # References
    /// This implementation is described in ["Faster Clustering Coefficient Using Vertex Covers"](https://ieeexplore.ieee.org/document/6693348).
    pub fn iter_clustering_coefficient_per_node(
        &self,
    ) -> impl IndexedParallelIterator<Item = f64> + '_ {
        self.get_number_of_triangles_per_node(Some(false))
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
    /// # References
    /// This implementation is described in ["Faster Clustering Coefficient Using Vertex Covers"](https://ieeexplore.ieee.org/document/6693348).
    pub fn get_clustering_coefficient_per_node(&self) -> Vec<f64> {
        self.iter_clustering_coefficient_per_node().collect()
    }

    /// Returns the graph clustering coefficient.
    ///
    /// # References
    /// This implementation is described in ["Faster Clustering Coefficient Using Vertex Covers"](https://ieeexplore.ieee.org/document/6693348).
    pub fn get_clustering_coefficient(&self) -> f64 {
        self.iter_clustering_coefficient_per_node().sum()
    }

    /// Returns the graph average clustering coefficient.
    ///
    /// # References
    /// This implementation is described in ["Faster Clustering Coefficient Using Vertex Covers"](https://ieeexplore.ieee.org/document/6693348).
    pub fn get_average_clustering_coefficient(&self) -> f64 {
        self.get_clustering_coefficient() / self.get_nodes_number() as f64
    }
}
