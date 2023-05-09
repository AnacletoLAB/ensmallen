use super::*;
use hyperloglog_rs::prelude::*;
use num_traits::Zero;
use rayon::prelude::*;

impl Graph {
    /// Computes a provided centrality of the graph using the HyperBall algorithm.
    ///
    /// This implementation of the HyperBall algorithm computes the provided centrality measure for the
    /// given graph. The algorithm iteratively computes the distances between nodes using a probabilistic
    /// data structure called HyperLogLog, which can approximate the number of distinct elements in a set
    /// with high accuracy while using little memory. The HyperBall algorithm updates each node's counter
    /// by taking the union of its neighbours' counters, and then computes the provided centrality measure
    /// based on the number of distinct elements in each node's counter. The algorithm repeats this process
    /// until each node's counter reaches a steady state, at which point the algorithm terminates.
    ///
    /// # Arguments
    /// * `counters_ops` - A closure that takes four arguments: a mutable reference to a centrality score,
    ///   the current count of distinct elements in a node's counter, the previous count of distinct elements
    ///   in a node's counter, and the current iteration number. The closure is used to update each node's
    ///   centrality score based on its counter.
    ///
    /// # Type Parameters
    /// * `PRECISION` - The precision parameter for the HyperLogLog data structure. This determines the
    ///   accuracy of the distance approximations and the memory usage of the algorithm.
    ///
    /// # Constraints
    /// * `PRECISION` must be a compile-time constant.
    /// * `PRECISION` must be less than or equal to the maximum precision supported by the HyperLogLog
    ///   data structure.
    ///
    /// # Returns
    /// A vector containing the computed centrality scores for each node in the graph.
    ///
    fn hyperball<const PRECISION: usize>(
        &self,
        counters_ops: fn(&mut f32, f32, f32, usize),
    ) -> Vec<f32>
    where
        [(); ceil(1 << PRECISION, NUMBER_OF_REGISTERS_IN_WORD)]:,
        [(); 1 << PRECISION]:,
    {
        // Create counters for all nodes in the graph
        let mut counters: Vec<HyperLogLog<PRECISION>> = self
            .par_iter_node_ids()
            .map(|node_id| HyperLogLog::from(node_id))
            .collect::<Vec<_>>();

        // Create copies of the counters to keep track of the previous iteration's state
        let mut previous_counters = counters.clone();

        // Create a vector to store the centrality values for each node
        let mut centralities = vec![0.0; self.get_number_of_nodes() as usize];

        // Counter to track the number of iterations
        let mut iteration_number = 0;

        // Continue iterating until all counters converge
        loop {
            iteration_number += 1;
            if counters
                .par_iter_mut()
                .zip(previous_counters.par_iter())
                .zip(centralities.par_iter_mut())
                .enumerate()
                .all(|(node_id, ((counter, previous_counter), centrality))| {
                    *counter = previous_counter.clone();
                    // Iterate through each neighbor of the current node
                    unsafe {
                        self.iter_unchecked_neighbour_node_ids_from_source_node_id(node_id as NodeT)
                    }
                    .for_each(|dst| {
                        // Union the counter with the neighbor's counter from the previous iteration
                        counter.register_wise_max_inplace(&previous_counters[dst as usize]);
                    });

                    let new_count = counter.count_dispatched();
                    let last_count = previous_counter.count_dispatched();

                    if new_count < last_count + f32::EPSILON {
                        true
                    } else {
                        // Update the centrality value for the current node
                        counters_ops(centrality, new_count, last_count, iteration_number);
                        false
                    }
                })
            {
                break;
            }

            // Swap the current and previous counters
            core::mem::swap(&mut previous_counters, &mut counters);
        }
        centralities
    }

    /// Dispatches the HyperBall algorithm with a given precision and counters operations function.
    /// The HyperBall algorithm is an approximation algorithm to compute node closeness centrality.
    ///
    /// # Arguments
    /// * `precision`: A `u8` indicating the desired precision for the algorithm. It must be in the
    /// range of 4 to 12 (inclusive), as these are the supported values for the precision.
    /// If None is provided, 6 is used by default.
    /// * `counters_ops`: A function that takes mutable references to four parameters of type `f32`.
    /// These parameters represent: (1) the centrality score, (2) the current count of the counter, (3) the previous count of the
    /// counter, and (4) the iteration number. This function is called for
    /// each node in each iteration of the algorithm, and it updates the centrality score.
    ///
    /// # Returns
    /// A `Result<Vec<f32>>` containing a vector with the approximated closeness centrality for each
    /// node in the graph. If the provided precision is not supported, an error message is returned.
    fn dispatch_hyperball(
        &self,
        precision: Option<u8>,
        counters_ops: fn(&mut f32, f32, f32, usize),
    ) -> Result<Vec<f32>> {
        Ok(match precision.unwrap_or(6) {
            4 => self.hyperball::<4>(counters_ops),
            5 => self.hyperball::<5>(counters_ops),
            6 => self.hyperball::<6>(counters_ops),
            7 => self.hyperball::<7>(counters_ops),
            8 => self.hyperball::<8>(counters_ops),
            9 => self.hyperball::<9>(counters_ops),
            10 => self.hyperball::<10>(counters_ops),
            11 => self.hyperball::<11>(counters_ops),
            12 => self.hyperball::<12>(counters_ops),
            _ => {
                return Err(format!(
                    concat!("The provided precision `{:?}` is not supported."),
                    precision
                ));
            }
        })
    }

    /// Returns an approximation of the closeness centrality for all nodes in the graph.
    ///
    /// This method applies the HyperBall algorithm to compute an approximation of the closeness
    /// centrality of each node. The precision parameter indicates the number of bits to use to
    /// represent the HyperLogLog registers. The higher the precision, the more accurate the
    /// results, but also the more memory required and the slower the algorithm.
    ///
    /// Closeness centrality is a metric that measures the importance of a node in a
    /// graph based on how close it is to all other nodes in the graph.
    /// This is determined by taking the reciprocal of the sum of the shortest
    /// path distances between a node and all other nodes in the graph.
    /// Closeness centrality is a measure of how quickly information can spread through a network,
    /// as nodes that are closer to other nodes can transmit information more efficiently.
    /// Nodes with higher closeness centrality are therefore considered more important
    /// in terms of their ability to communicate with other nodes in the network.
    /// However, closeness centrality is sensitive to disconnected nodes and may not
    /// provide a reliable measure of importance in graphs with multiple connected components.
    ///
    /// # Arguments
    /// * `precision`: Option<u8> - The number of bits to use to represent the HyperLogLog registers. By default 6.
    ///
    /// # Returns
    /// A vector of f32 values containing the approximated closeness centrality for each node.
    ///
    pub fn get_approximated_closeness_centrality(&self, precision: Option<u8>) -> Result<Vec<f32>> {
        let mut centralities = self.dispatch_hyperball(
            precision,
            |centrality: &mut f32, current_count, previous_count, iteration_number| {
                *centrality += iteration_number as f32 * (current_count - previous_count);
            },
        )?;

        centralities
            .par_iter_mut()
            .filter(|centrality| !centrality.is_zero())
            .for_each(|centrality| {
                *centrality = centrality.recip();
            });

        Ok(centralities)
    }

    /// Returns an approximation of the harmonic centrality for all nodes in the graph.
    ///
    /// This method applies the HyperBall algorithm to compute an approximation of the harmonic
    /// centrality of each node. The precision parameter indicates the number of bits to use to
    /// represent the HyperLogLog registers. The higher the precision, the more accurate the
    /// results, but also the more memory required and the slower the algorithm.
    ///
    /// Harmonic centrality is another metric that measures the importance of a node
    /// in a graph based on its ability to reach other nodes.
    /// It is defined as the sum of the harmonic mean of the distances between a node
    /// and all other nodes in the graph. The harmonic mean is used instead of the
    /// arithmetic mean as it gives greater weight to shorter distances.
    /// Harmonic centrality is also more robust to disconnected nodes than closeness centrality,
    /// as it assigns higher centrality scores to nodes that are closer to other nodes
    /// within their own connected component. However, harmonic centrality does not
    /// provide an accurate measure of importance in terms of communication efficiency,
    /// as it does not take into account the actual distances between nodes.
    /// It is therefore most useful for measuring the ability of a node to reach
    /// other nodes within a connected component.
    ///
    /// # Arguments
    /// * `precision`: Option<u8> - The number of bits to use to represent the HyperLogLog registers. By default 6.
    ///
    /// # Returns
    /// A vector of f32 values containing the approximated harmonic centrality for each node.
    ///
    pub fn get_approximated_harmonic_centrality(&self, precision: Option<u8>) -> Result<Vec<f32>> {
        self.dispatch_hyperball(
            precision,
            |centrality: &mut f32, current_count, previous_count, iteration_number| {
                *centrality += (iteration_number as f32).recip() * (current_count - previous_count);
            },
        )
    }
}
