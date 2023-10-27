use super::*;
use core::mem::MaybeUninit;
use hyperloglog_rs::prelude::*;
use std::collections::HashSet;

// Method to allocate an array of HashSets using maybe uninitialized memory,
// so to circumvent the fact that HashSet does not implement Copy.
fn allocate_array_of_hashsets<const N: usize>() -> [HashSet<usize>; N] {
    unsafe {
        let mut array: [HashSet<usize>; N] = MaybeUninit::uninit().assume_init();
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
    /// * `number_of_hops` - The number of hops to consider. By default, 2.
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
    ) -> ([[usize; N]; N], [usize; N], [usize; N]) {
        let mut src_neighbour_hypersphere = allocate_array_of_hashsets::<N>();
        let mut dst_neighbour_hypersphere = allocate_array_of_hashsets::<N>();

        for (node, hypersphere) in [
            (src, &mut src_neighbour_hypersphere[0]),
            (dst, &mut dst_neighbour_hypersphere[0]),
        ] {
            // We insert the nodes themselves in the hypersphere.
            hypersphere.insert(node as usize);
            // First, we populate the hypersphere of neighbours.
            for neighbour in
                unsafe { self.iter_unchecked_neighbour_node_ids_from_source_node_id(node) }
            {
                hypersphere.insert(neighbour as usize);
            }
        }

        // Then, we populate the hypersphere of neighbours up to the given number of hops.
        for i in 1..N {
            for (previous_sphere, hypersphere) in [
                src_neighbour_hypersphere.as_mut().split_at_mut(i),
                dst_neighbour_hypersphere.as_mut().split_at_mut(i),
            ] {
                for node in &previous_sphere[0] {
                    for neighbour in unsafe {
                        self.iter_unchecked_neighbour_node_ids_from_source_node_id(*node as u32)
                    } {
                        hypersphere[0].insert(neighbour as usize);
                    }
                }
            }
        }

        // At this point, we need to merge the hypersphere of neighbours, so that
        // the second hop hypersphere contains the first hop hypersphere, the third
        // hop hypersphere contains the second and first hop hypersphere, and so on.
        for i in 1..N {
            src_neighbour_hypersphere[i as usize] = src_neighbour_hypersphere[i as usize]
                .union(&src_neighbour_hypersphere[(i - 1) as usize])
                .cloned()
                .collect();
            dst_neighbour_hypersphere[i as usize] = dst_neighbour_hypersphere[i as usize]
                .union(&dst_neighbour_hypersphere[(i - 1) as usize])
                .cloned()
                .collect();
        }

        // Now, we can compute the overlap matrix.
        HashSet::overlap_and_differences_cardinality_matrices(
            &src_neighbour_hypersphere,
            &dst_neighbour_hypersphere,
        )
    }

    pub fn get_exact_edge_sketching_from_edge_node_ids(
        &self,
        src: NodeT,
        dst: NodeT,
        number_of_hops: Option<NodeT>,
    ) -> Result<(Vec<Vec<usize>>, Vec<usize>, Vec<usize>)> {
        self.validate_node_id(src)?;
        self.validate_node_id(dst)?;

        let number_of_hops = number_of_hops.unwrap_or(2);
        match number_of_hops {
            1 => {
                let (overlap_matrix, left_subtraction_vector, right_subtraction_vector) =
                    self.get_exact_edge_sketching_from_edge_node_ids_with_constant::<1>(src, dst);
                Ok((
                    vec![overlap_matrix[0].to_vec()],
                    vec![left_subtraction_vector[0]],
                    vec![right_subtraction_vector[0]],
                ))
            }
            2 => {
                let (overlap_matrix, left_subtraction_vector, right_subtraction_vector) =
                    self.get_exact_edge_sketching_from_edge_node_ids_with_constant::<2>(src, dst);
                Ok((
                    overlap_matrix.iter().map(|row| row.to_vec()).collect(),
                    left_subtraction_vector.to_vec(),
                    right_subtraction_vector.to_vec(),
                ))
            }
            3 => {
                let (overlap_matrix, left_subtraction_vector, right_subtraction_vector) =
                    self.get_exact_edge_sketching_from_edge_node_ids_with_constant::<3>(src, dst);
                Ok((
                    overlap_matrix.iter().map(|row| row.to_vec()).collect(),
                    left_subtraction_vector.to_vec(),
                    right_subtraction_vector.to_vec(),
                ))
            }
            4 => {
                let (overlap_matrix, left_subtraction_vector, right_subtraction_vector) =
                    self.get_exact_edge_sketching_from_edge_node_ids_with_constant::<4>(src, dst);
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
