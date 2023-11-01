use super::*;
use core::mem::MaybeUninit;
use hyperloglog_rs::prelude::*;
use std::collections::HashSet;
use rayon::prelude::*;

// Method to allocate an array of HashSets using maybe uninitialized memory,
// so to circumvent the fact that HashSet does not implement Copy.
fn allocate_array_of_hashsets<const N: usize>() -> [HashSet<NodeT>; N] {
    unsafe {
        let mut array: [HashSet<NodeT>; N] = MaybeUninit::uninit().assume_init();
        for i in 0..N {
            // We replace the previosly initialized value with an hashset
            // and we forget the previous value.
            std::mem::forget(std::mem::replace(&mut array[i], HashSet::new()));
        }
        array
    }
}

impl Graph {
    /// Get the exact edge sketching from a given edge.
    ///
    /// # Arguments
    /// * `src` - The source node of the edge.
    /// * `dst` - The destination node of the edge.
    /// * `include_selfloops` - Whether to include selfloops in the sketching.
    ///
    /// # Raises
    /// * If the source node does not exist.
    /// * If the destination node does not exist.
    /// * If the number of hops is not greater than 0.
    ///
    /// Implementative details
    /// ----------------------
    /// The exact edge sketching is a triple of features, and is the exact version of
    /// the approximated edge sketching based on HyperLogLog counters as described in
    /// the paper ["Graph Neural Networks for Link Prediction with Subgraph Sketching"](https://openreview.net/pdf?id=m1oqEOAozQU).
    ///
    /// While in the paper the authors use approximated counting data-structures, in this
    /// implementation we use exact counting data-structures, namely the `HashSet` data-structure.
    /// Of course, this is not scalable, but it is useful for testing purposes and to evaluate
    /// how well the approximated version performs.
    ///
    /// First, we compute the hyper-spheres of neighbours up to the given number of hops. This means
    /// that at the first hop we have the node plus all of its neighbours, at the second hop we have
    /// the node plus all of its neighbours plus all of the neighbours of the neighbours, and so on.
    ///
    /// Secondarily, we compute three features:
    /// * The overlap matrix, which is a bidimensional vector of counters, namely usize, which is meant
    ///   to have in each position i and j the intersection between the hyper-sphere of neighbours of
    ///   the source node at hop i and the hyper-sphere of neighbours of the destination node at hop j,
    ///   minus the intersection between the hyper-sphere of neighbours of the source node up to hop i and
    ///   the hyper-sphere of neighbours of the destination node up to hop j.
    ///   For instance, in the case of two hops, in position (0, 0) we have the cardinality of the intersection
    ///   between the neighbours of the source node and the neighbours of the destination node.
    ///   In position (1, 0) we have the cardinality of the intersection between the 2-hop hypersphere of
    ///   neighbours of the source node and the neighbours of the destination node, minus the value we have
    ///   in position (0, 0). In position (0, 1) we have the cardinality of the intersection between the
    ///   neighbours of the source node and the 2-hop hypersphere of neighbours of the destination node,
    ///   minus the value we have in position (0, 0). Finally, in position (1, 1) we have the cardinality
    ///   of the intersection between the 2-hop hypersphere of neighbours of the source node and the 2-hop
    ///   hypersphere of neighbours of the destination node, minus the values we have in positions (0, 0),
    ///   (1, 0) and (0, 1).
    /// * The left subtraction vector, which is a flat vector of counters, namely usize, which is meant to have
    ///   in each position i the cardinality of the difference between the i-th hyper-sphere of neighbours of
    ///   the source node and the largest hyper-sphere of neighbours of the destination node, minus the cardinality
    ///   of all other intersections between the to to the i-th hyper-sphere of neighbours of the source node and the
    ///   largest hyper-sphere of neighbours of the destination node.
    ///   For instance, in the case of two hops, in position 0 we have the cardinality of the difference between
    ///   the neighbours of the source node and the 2-hop hypersphere of neighbours of the destination node.
    ///   In position 1 we have the cardinality of the difference between the 2-hop hypersphere of neighbours of
    ///   the source node and the 2-hop hypersphere of neighbours of the destination node, minus the value we have
    ///   in position 0.
    /// * The right subtraction vector, which is a flat vector of counters, namely usize, which is meant to have
    ///   in each position i the cardinality of the difference between the i-th hyper-sphere of neighbours of
    ///   the destination node and the largest hyper-sphere of neighbours of the source node, minus the cardinality
    ///   of all other intersections between the to to the i-th hyper-sphere of neighbours of the destination node and the
    ///   largest hyper-sphere of neighbours of the source node.
    ///   For instance, in the case of two hops, in position 0 we have the cardinality of the difference between
    ///   the neighbours of the destination node and the 2-hop hypersphere of neighbours of the source node.
    ///   In position 1 we have the cardinality of the difference between the 2-hop hypersphere of neighbours of
    ///   the destination node and the 2-hop hypersphere of neighbours of the source node, minus the value we have
    ///   in position 0.
    ///
    fn get_exact_edge_sketching_from_edge_node_ids_with_constant<const N: usize>(
        &self,
        src: NodeT,
        dst: NodeT,
        include_selfloops: bool,
    ) -> ([[NodeT; N]; N], [NodeT; N], [NodeT; N]) {
        let mut src_neighbour_hypersphere = allocate_array_of_hashsets::<N>();
        let mut dst_neighbour_hypersphere = allocate_array_of_hashsets::<N>();

        for (node, hypersphere) in [
            (src, &mut src_neighbour_hypersphere[0]),
            (dst, &mut dst_neighbour_hypersphere[0]),
        ] {
            // We insert the nodes themselves in the hypersphere.
            if include_selfloops {
                hypersphere.insert(node as NodeT);
            }
            // First, we populate the hypersphere of neighbours.
            for neighbour in
                unsafe { self.iter_unchecked_neighbour_node_ids_from_source_node_id(node) }
            {
                hypersphere.insert(neighbour as NodeT);
            }
        }

        // Then, we populate the hypersphere of neighbours up to the given number of hops.
        for i in 1..N {
            for (previous_sphere, hypersphere) in [
                src_neighbour_hypersphere.as_mut().split_at_mut(i),
                dst_neighbour_hypersphere.as_mut().split_at_mut(i),
            ] {
                // We initialize the current hypersphere as a copy of the previous one.
                hypersphere[0] = previous_sphere[i - 1].clone();
                let previous_previous_sphere: Option<&HashSet<NodeT>> = if i > 1 {
                    Some(&previous_sphere[i - 2])
                } else {
                    None
                };
                // We want to iterate on the elements of the previous hypersphere.
                for node in &previous_sphere[i - 1] {
                    // We want to skip the nodes that were in the previous previous hypersphere,
                    // as the neighbours of the nodes contained therein were already inserted in
                    // the previous hypersphere.
                    if let Some(previous_previous_sphere) = previous_previous_sphere {
                        if previous_previous_sphere.contains(node) {
                            continue;
                        }
                    }
                    for neighbour in unsafe {
                        self.iter_unchecked_neighbour_node_ids_from_source_node_id(*node as u32)
                    } {
                        // And we insert the neighbours in the current hypersphere.
                        hypersphere[0].insert(neighbour as NodeT);
                    }
                }
            }
        }

        // Now, we can compute the overlap matrix.
        HashSet::overlap_and_differences_cardinality_matrices(
            &src_neighbour_hypersphere,
            &dst_neighbour_hypersphere,
        )
    }

    /// Get the exact edge sketching from a given edge.
    ///
    /// # Arguments
    /// * `subgraph` - The subgraph to consider.
    /// * `include_selfloops` - Whether to include selfloops in the sketching. By default, it is true.
    /// * `number_of_hops` - The number of hops to consider.
    ///
    pub fn get_exact_edge_sketching_from_graph(
        &self,
        subgraph: &Self,
        include_selfloops: Option<bool>,
        number_of_hops: Option<NodeT>,
    ) -> Result<(Vec<Vec<Vec<NodeT>>>, Vec<Vec<NodeT>>, Vec<Vec<NodeT>>)> {
        // The subgraph provided must be compatible with the current graph.
        self.must_share_node_vocabulary(subgraph)?;
        let number_of_edges: usize = self.get_number_of_directed_edges() as usize;
        let number_of_hops = number_of_hops.unwrap_or(2);
        let mut overlap_matrices: Vec<Vec<Vec<NodeT>>> =
            vec![vec![vec![0; number_of_hops as usize]; number_of_hops as usize]; number_of_edges];
        let mut left_subtraction_vectors: Vec<Vec<NodeT>> =
            vec![vec![0; number_of_hops as usize]; number_of_edges];
        let mut right_subtraction_vectors: Vec<Vec<NodeT>> =
            vec![vec![0; number_of_hops as usize]; number_of_edges];

        // We start to iterate in parallel using Rayon over the edges of the subgraph, zipped with the overlap matrices and the subtraction vectors.
        // For each edge, we compute the exact edge sketching and we store it in the corresponding position of the overlap matrices and the subtraction vectors.

        subgraph
            .par_iter_directed_edge_node_ids()
            .zip(overlap_matrices.par_iter_mut())
            .zip(left_subtraction_vectors.par_iter_mut())
            .zip(right_subtraction_vectors.par_iter_mut())
            .try_for_each(
                |((((_, src, dst), target_overlap_matrix), target_left_subtraction_vector),
                 target_right_subtraction_vector)| {
                    let (overlap_matrix, left_subtraction_vector, right_subtraction_vector) = self
                        .get_exact_edge_sketching_from_edge_node_ids(
                            src,
                            dst,
                            include_selfloops,
                            Some(number_of_hops),
                        )?;
                    
                    target_overlap_matrix.iter_mut().zip(overlap_matrix.iter()).for_each(|(target_row, row)| {
                        target_row.iter_mut().zip(row.iter()).for_each(|(target_value, value)| {
                            *target_value = *value;
                        })
                    });

                    target_left_subtraction_vector
                        .iter_mut()
                        .zip(left_subtraction_vector.iter())
                        .for_each(|(target_value, value)| {
                            *target_value = *value;
                        });

                    target_right_subtraction_vector
                        .iter_mut()
                        .zip(right_subtraction_vector.iter())
                        .for_each(|(target_value, value)| {
                            *target_value = *value;
                        });
                    
                    Ok::<(), String>(())
                },
            )?;

        Ok((
            overlap_matrices,
            left_subtraction_vectors,
            right_subtraction_vectors,
        ))
    }

    /// Get the exact edge sketching from a given edge.
    ///
    /// # Arguments
    /// * `src` - The source node of the edge.
    /// * `dst` - The destination node of the edge.
    /// * `include_selfloops` - Whether to include selfloops in the sketching. By default, it is true.
    /// * `number_of_hops` - The number of hops to consider.
    ///
    pub fn get_exact_edge_sketching_from_edge_node_ids(
        &self,
        src: NodeT,
        dst: NodeT,
        include_selfloops: Option<bool>,
        number_of_hops: Option<NodeT>,
    ) -> Result<(Vec<Vec<NodeT>>, Vec<NodeT>, Vec<NodeT>)> {
        self.validate_node_id(src)?;
        self.validate_node_id(dst)?;

        let include_selfloops = include_selfloops.unwrap_or(true);

        let number_of_hops = number_of_hops.unwrap_or(2);
        match number_of_hops {
            1 => {
                let (overlap_matrix, left_subtraction_vector, right_subtraction_vector) = self
                    .get_exact_edge_sketching_from_edge_node_ids_with_constant::<1>(
                        src,
                        dst,
                        include_selfloops,
                    );
                Ok((
                    vec![overlap_matrix[0].to_vec()],
                    vec![left_subtraction_vector[0]],
                    vec![right_subtraction_vector[0]],
                ))
            }
            2 => {
                let (overlap_matrix, left_subtraction_vector, right_subtraction_vector) = self
                    .get_exact_edge_sketching_from_edge_node_ids_with_constant::<2>(
                        src,
                        dst,
                        include_selfloops,
                    );
                Ok((
                    overlap_matrix.iter().map(|row| row.to_vec()).collect(),
                    left_subtraction_vector.to_vec(),
                    right_subtraction_vector.to_vec(),
                ))
            }
            3 => {
                let (overlap_matrix, left_subtraction_vector, right_subtraction_vector) = self
                    .get_exact_edge_sketching_from_edge_node_ids_with_constant::<3>(
                        src,
                        dst,
                        include_selfloops,
                    );
                Ok((
                    overlap_matrix.iter().map(|row| row.to_vec()).collect(),
                    left_subtraction_vector.to_vec(),
                    right_subtraction_vector.to_vec(),
                ))
            }
            4 => {
                let (overlap_matrix, left_subtraction_vector, right_subtraction_vector) = self
                    .get_exact_edge_sketching_from_edge_node_ids_with_constant::<4>(
                        src,
                        dst,
                        include_selfloops,
                    );
                Ok((
                    overlap_matrix.iter().map(|row| row.to_vec()).collect(),
                    left_subtraction_vector.to_vec(),
                    right_subtraction_vector.to_vec(),
                ))
            }
            _ => {
                return Err(format!(
                    concat!("The number of hops must be less than 5, ", "but it is {}."),
                    number_of_hops
                ))
            }
        }
    }
}
