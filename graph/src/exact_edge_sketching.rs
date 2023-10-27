use super::*;
use std::collections::HashSet;

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
    /// * The overlap matrix, which is a bidimensional vector of counters, namely u32, which is meant
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
    /// * The left subtraction vector, which is a flat vector of counters, namely u32, which is meant to have
    ///   in each position i the cardinality of the difference between the i-th hyper-sphere of neighbours of
    ///   the source node and the largest hyper-sphere of neighbours of the destination node, minus the cardinality
    ///   of all other intersections between the to to the i-th hyper-sphere of neighbours of the source node and the
    ///   largest hyper-sphere of neighbours of the destination node.
    ///   For instance, in the case of two hops, in position 0 we have the cardinality of the difference between
    ///   the neighbours of the source node and the 2-hop hypersphere of neighbours of the destination node.
    ///   In position 1 we have the cardinality of the difference between the 2-hop hypersphere of neighbours of
    ///   the source node and the 2-hop hypersphere of neighbours of the destination node, minus the value we have
    ///   in position 0.
    /// * The right subtraction vector, which is a flat vector of counters, namely u32, which is meant to have
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
    pub fn get_exact_edge_sketching_from_edge_node_ids(
        &self,
        src: NodeT,
        dst: NodeT,
        number_of_hops: Option<u8>,
    ) -> Result<(Vec<Vec<u32>>, Vec<u32>, Vec<u32>)> {
        self.validate_node_id(src)?;
        self.validate_node_id(dst)?;
        let number_of_hops = number_of_hops.unwrap_or(2);
        if number_of_hops == 0 {
            return Err(format!(
                concat!(
                    "The number of hops must be greater than 0, ",
                    "but it is {}."
                ),
                number_of_hops
            ));
        }

        let mut src_neighbour_hyperspheres = vec![HashSet::new(); number_of_hops as usize];
        let mut dst_neighbour_hyperspheres = vec![HashSet::new(); number_of_hops as usize];

        for (node, hypersphere) in [
            (src, &mut src_neighbour_hyperspheres[0]),
            (dst, &mut dst_neighbour_hyperspheres[0]),
        ] {
            // We insert the nodes themselves in the hyperspheres.
            hypersphere.insert(node);
            // First, we populate the hyperspheres of neighbours.
            for neighbour in
                unsafe { self.iter_unchecked_neighbour_node_ids_from_source_node_id(node) }
            {
                hypersphere.insert(neighbour);
            }
        }

        // Then, we populate the hyperspheres of neighbours up to the given number of hops.
        for i in 1..number_of_hops {
            for (node, hypersphere) in [
                (src, &mut src_neighbour_hyperspheres[i as usize]),
                (dst, &mut dst_neighbour_hyperspheres[i as usize]),
            ] {
                for neighbour in
                    unsafe { self.iter_unchecked_neighbour_node_ids_from_source_node_id(node) }
                {
                    hypersphere.insert(neighbour);
                }
            }
        }

        // At this point, we need to merge the hyperspheres of neighbours, so that
        // the second hop hypersphere contains the first hop hypersphere, the third
        // hop hypersphere contains the second and first hop hyperspheres, and so on.
        for i in 1..number_of_hops {
            src_neighbour_hyperspheres[i as usize] = src_neighbour_hyperspheres[i as usize]
                .union(&src_neighbour_hyperspheres[(i - 1) as usize])
                .cloned()
                .collect();
            dst_neighbour_hyperspheres[i as usize] = dst_neighbour_hyperspheres[i as usize]
                .union(&dst_neighbour_hyperspheres[(i - 1) as usize])
                .cloned()
                .collect();
        }

        // Now, we can compute the overlap matrix.
        let mut overlap_matrix = vec![vec![0; number_of_hops as usize]; number_of_hops as usize];

        for i in 0..number_of_hops {
            for j in 0..number_of_hops {
                overlap_matrix[i as usize][j as usize] = src_neighbour_hyperspheres[i as usize]
                    .intersection(&dst_neighbour_hyperspheres[j as usize])
                    .count() as u32;
                if i > 0 {
                    overlap_matrix[i as usize][j as usize] -=
                        overlap_matrix[(i - 1) as usize][j as usize];
                }
                if j > 0 {
                    overlap_matrix[i as usize][j as usize] -=
                        overlap_matrix[i as usize][(j - 1) as usize];
                }
                if i > 0 && j > 0 {
                    overlap_matrix[i as usize][j as usize] +=
                        overlap_matrix[(i - 1) as usize][(j - 1) as usize];
                }
            }
        }

        // Now, we can compute the left subtraction vector.
        let mut left_subtraction_vector = vec![0; number_of_hops as usize];
        let mut right_subtraction_vector = vec![0; number_of_hops as usize];

        for subtraction_vector in [&mut left_subtraction_vector, &mut right_subtraction_vector] {
            let mut comulative_cardinality = 0;
            for i in 0..number_of_hops {
                left_subtraction_vector[i as usize] = src_neighbour_hyperspheres[i as usize]
                    .difference(&dst_neighbour_hyperspheres[(number_of_hops - 1) as usize])
                    .count() as u32
                    - comulative_cardinality;
                comulative_cardinality += left_subtraction_vector[i as usize];
            }
        }

        Ok((
            overlap_matrix,
            left_subtraction_vector,
            right_subtraction_vector,
        ))
    }
}
