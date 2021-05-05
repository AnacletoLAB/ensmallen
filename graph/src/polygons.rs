use super::*;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

impl Graph {
    /// Returns number of triangles in the graph.
    ///
    /// # References
    /// This implementation is described in ["Faster Clustering Coefficient Using Vertex Covers"](https://ieeexplore.ieee.org/document/6693348).
    pub fn get_triangles_number(&self) -> EdgeT {
        // First, we compute the set of nodes composing a vertex cover set.
        // This vertex cover is NOT minimal, but is a 2-approximation.
        let vertex_cover_set = self.approximated_vertex_cover_set();
        // We start iterating over the nodes in the cover using rayon to parallelize the procedure.
        vertex_cover_set
            .par_iter()
            // For each node in the cover
            .map(|&node_id| {
                // We obtain the neighbours and collect them into a vector
                // We store them instead of using them in a stream because we will need
                // them multiple times below.
                let neighbours = self
                    .iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                    .collect::<Vec<NodeT>>();
                // We iterate over the neighbours
                neighbours
                    .iter()
                    .map(|&neighbour_node_id| {
                        // If the neighbour either is a selfloop
                        // or is not present in the vertex cover
                        // we return 0 new triangles.
                        if node_id == neighbour_node_id
                            || !vertex_cover_set.contains(&neighbour_node_id)
                        {
                            0
                        } else {
                            // We compute the intersection of the neighbours.
                            iter_set::intersection(
                                neighbours.iter().cloned(),
                                self.iter_unchecked_neighbour_node_ids_from_source_node_id(
                                    neighbour_node_id,
                                ),
                            )
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
                        }
                    })
                    .sum::<EdgeT>()
            })
            .sum::<EdgeT>()
    }
}
