use std::cell::SyncUnsafeCell;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};

use super::*;
use hyperloglog_rs::prelude::*;
use num_traits::Zero;
use rayon::prelude::*;

impl Graph {
    #[inline(always)]
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
    /// # Implementation Details on Work Stealing
    /// Work stealing is a technique used in parallel computing to distribute and balance
    /// the workload among a group of threads or processes. In a work-stealing system,
    /// each thread maintains its own deque of tasks, and when a thread finishes its own tasks,
    /// it may randomly or deterministically steal tasks from the deque of another thread in the system.
    ///
    /// The idea behind work stealing is to avoid load imbalance among the threads.
    /// Load imbalance can occur when one thread is assigned more work than others,
    /// causing it to be idle while the other threads are busy. Work stealing helps
    /// to mitigate this problem by allowing threads to share their workloads,
    /// ensuring that all threads stay busy and the overall performance of the system is maximized.
    /// Work stealing also improves cache utilization by keeping the data local to the thread that needs it,
    /// reducing the amount of cache invalidation and contention.
    ///
    /// Work stealing threads are commonly used in many programming frameworks,
    /// such as the Rayon library in Rust, to create parallel code that
    /// automatically balances the workload among threads.
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
    /// * `BITS` - The number of bits used to store each element in the HyperLogLog data structure. This
    ///  determines the memory usage of the algorithm.
    ///
    /// # Constraints
    /// * `PRECISION` must be a compile-time constant.
    /// * `BITS` must be a compile-time constant.
    /// * `PRECISION` must be less than or equal to the maximum precision supported by the HyperLogLog
    ///   data structure. Values below 4 or higher than 16 are not supported.
    /// * BITS must be either 5 or 6 - other values are functionally possible but discouraged by
    /// empirical evidence.
    ///
    /// # Returns
    /// A vector containing the computed centrality scores for each node in the graph.
    ///
    fn hyperball<const PRECISION: usize, const BITS: usize>(
        &self,
        counters_ops: fn(&mut f32, f32, f32, usize),
    ) -> Result<Vec<f32>>
    where
        [(); ceil(1 << PRECISION, 32 / BITS)]:,
        [(); 1 << PRECISION]:,
    {
        // Create a mutable vector called centralities to store the centrality values for each node.
        // The size of the vector is set to the number of nodes in the graph using the get_number_of_nodes() method of self.
        let mut centralities = vec![0.0; self.get_number_of_nodes() as usize];
        // Create a SyncUnsafeCell to share centralities between threads.
        let shared_centralities = SyncUnsafeCell::new(&mut centralities);

        // Call the get_thread_pool() function to get the number of available CPUs and a thread pool to run computations on.
        let (cpu_number, pool) = get_thread_pool()?;

        // Create a closure called get_task that takes in three arguments:
        // - node_counters: a slice of AtomicU32 counters for each thread.
        // - thread_id: the ID of the current thread.
        // - number_of_nodes: the total number of nodes in the graph.
        // The closure iterates over the node_counters slice and finds a new task for the current thread to work on.
        // If a new task is found, the closure returns its node ID, otherwise it returns None.
        let get_task =
            |node_counters: &[AtomicU32], thread_id: usize, number_of_nodes: u32| -> Option<u32> {
                for i in
                    (thread_id..(thread_id + node_counters.len())).map(|i| i % node_counters.len())
                {
                    // We get the last value in this field
                    let previous = node_counters[i].fetch_add(1, Ordering::SeqCst);
                    // And we check whether this value is still within the range
                    // associated to this particular thread bucket
                    // Specifically we observe that for the last thread, we need to consider
                    // the few nodes we may loose due to the integer division.
                    if previous < number_of_nodes / node_counters.len() as u32 * (i as u32 + 1)
                        || i == node_counters.len() - 1 && previous < number_of_nodes
                    {
                        // If so, we have found a new task, hurray!
                        return Some(previous);
                    }
                }
                // Otherwise the tasks of this particular run are finished and we are done.
                None
            };

        // Create two Atomic types to track convergence and the current iteration.
        let convergence_flag: AtomicBool = AtomicBool::from(false);
        let current_iteration: AtomicUsize = AtomicUsize::from(1);

        // Get the number of nodes in the graph.
        let number_of_nodes = self.get_number_of_nodes();

        // Create a vector of AtomicU32 counters, one for each thread.
        // The initial value for each counter is u32::MAX.
        let node_counters: Vec<AtomicU32> =
            unsafe { core::mem::transmute(vec![u32::MAX; cpu_number]) };

        // Create a closure called reset_counters that takes in two arguments:
        // - node_counters: a slice of AtomicU32 counters for each thread.
        // - number_of_nodes: the total number of nodes in the graph.
        // The closure resets the value of each AtomicU32 counter in node_counters to an appropriate value for the next iteration.
        let reset_counters = |node_counters: &[AtomicU32], number_of_nodes: u32| {
            node_counters.iter().enumerate().for_each(|(i, counter)| {
                counter.store(
                    number_of_nodes / node_counters.len() as u32 * i as u32,
                    Ordering::Relaxed,
                );
            });
        };

        // Call the reset_counters closure to reset the counters for the next iteration.
        reset_counters(&node_counters, number_of_nodes);

        // Create a vector of AtomicBool convergence_flags, one for each thread.
        let convergence_flags: Vec<AtomicBool> =
            unsafe { core::mem::transmute(vec![false; cpu_number]) };

        // Create HyperLogLog counters for all nodes in the graph
        let mut counters: Vec<HyperLogLog<PRECISION, BITS>> = self
            .par_iter_node_ids()
            .map(|node_id| node_id.into())
            .collect::<Vec<_>>();

        // Create copies of the counters to keep track of the previous iteration's state
        let mut previous_counters = counters.clone();

        // Create a SyncUnsafeCell to share the counters between threads.
        let shared_counters = SyncUnsafeCell::new(&mut counters);
        let shared_previous_counters = SyncUnsafeCell::new(&mut previous_counters);

        // Use a Rayon thread pool to distribute the work across multiple threads
        pool.scope(|s| {
            // We spawn a thread for each bucket
            (0..node_counters.len()).for_each(|_| {
                s.spawn(|_| {
                    // We get the thread id
                    let thread_id = rayon::current_thread_index().expect(concat!(
                        "current_thread_id not called ",
                        "from a rayon thread. ",
                        "This should not be possible because ",
                        "this is in a Rayon Thread Pool."
                    ));
                    // Until we have not reached convergence
                    'outer: while !convergence_flag.load(Ordering::Relaxed) {
                        // We reset the convergence flag for this thread
                        let mut convergence = true;
                        // Get the next node to process from the task queue associated with the current thread
                        // or the other threads in a work-stealing fashion.
                        while let Some(node_id) =
                            get_task(&node_counters, thread_id, self.get_number_of_nodes())
                        {
                            // We get the current iteration
                            // Note we cannot retrieve this only once outside the loop
                            // as data races may happen between the while loop condition
                            // failure and the current iteration retrieval.
                            let iteration = current_iteration.load(Ordering::Relaxed);

                            // And, according to the current iteration, we get the counters
                            let (primary_counters, secondary_counters) = if iteration % 2 == 0 {
                                (&shared_counters, &shared_previous_counters)
                            } else {
                                (&shared_previous_counters, &shared_counters)
                            };

                            // We get the current node counters
                            let (counter, previous_counter) = (
                                unsafe { &mut (*primary_counters.get())[node_id as usize] },
                                unsafe { &(*secondary_counters.get())[node_id as usize] },
                            );

                            // Iterate through each neighbor of the current node
                            let new_counter = unsafe {
                                self.iter_unchecked_neighbour_node_ids_from_source_node_id(
                                    node_id as NodeT,
                                )
                            }
                            .map(|dst| unsafe { &(*secondary_counters.get())[dst as usize] })
                            .union()
                                | previous_counter;

                            // We check whether the counter has converged
                            // and we update the convergence flag accordingly
                            convergence &= previous_counter == &new_counter;

                            // Update the centrality value for the current node
                            counters_ops(
                                unsafe { &mut (*shared_centralities.get())[node_id as usize] },
                                new_counter.estimate_cardinality(),
                                previous_counter.estimate_cardinality(),
                                iteration,
                            );

                            // We update the counter for the current node
                            *counter = new_counter;
                        }

                        // We update the convergence flag for this thread
                        convergence_flags[thread_id].store(convergence, Ordering::Relaxed);

                        // We have finished all the tasks for this particular thread
                        // so we need to reset the counters for the next iteration

                        // Only the master thread, i.e. the one with id `0` will
                        // reset the counters for the next iteration

                        if thread_id == 0 {
                            // We check whether we have reached convergence
                            // by checking whether all the convergence flags are true
                            if convergence_flags
                                .iter()
                                .all(|convergence_flag| convergence_flag.load(Ordering::Relaxed))
                            {
                                // If so, we set the convergence flag to true
                                convergence_flag.store(true, Ordering::Relaxed);
                                // And we break the loop
                                break 'outer;
                            }

                            // Otherwise we increase the iteration number
                            current_iteration.fetch_add(1, Ordering::Relaxed);

                            // We reset the counters for the next iteration
                            reset_counters(&node_counters, number_of_nodes);
                        }
                    }
                });
            });
        });

        // Return the computed centralities
        Ok(centralities)
    }

    #[inline(always)]
    /// Dispatches the HyperBall algorithm with a given precision and counters operations function.
    /// The HyperBall algorithm is an approximation algorithm to compute node closeness centrality.
    ///
    /// # Arguments
    /// * `precision`: A `u8` indicating the desired precision for the algorithm. It must be in the
    /// range of 4 to 16 (inclusive), as these are the supported values for the precision.
    /// If None is provided, 6 is used by default.
    /// * `bits`: A `u8` indicating the number of bits to use for the HyperLogLog counters. It must be either 5 or 6,
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
        bits: Option<u8>,
        counters_ops: fn(&mut f32, f32, f32, usize),
    ) -> Result<Vec<f32>> {
        Ok(match (precision.unwrap_or(6), bits.unwrap_or(6)) {
            (4, 5) => self.hyperball::<4, 5>(counters_ops)?,
            (5, 5) => self.hyperball::<5, 5>(counters_ops)?,
            (6, 5) => self.hyperball::<6, 5>(counters_ops)?,
            (7, 5) => self.hyperball::<7, 5>(counters_ops)?,
            (8, 5) => self.hyperball::<8, 5>(counters_ops)?,
            (9, 5) => self.hyperball::<9, 5>(counters_ops)?,
            (10, 5) => self.hyperball::<10, 5>(counters_ops)?,
            (11, 5) => self.hyperball::<11, 5>(counters_ops)?,
            (12, 5) => self.hyperball::<12, 5>(counters_ops)?,
            (13, 5) => self.hyperball::<13, 5>(counters_ops)?,
            (14, 5) => self.hyperball::<14, 5>(counters_ops)?,
            (15, 5) => self.hyperball::<15, 5>(counters_ops)?,
            (16, 5) => self.hyperball::<16, 5>(counters_ops)?,
            (4, 6) => self.hyperball::<4, 6>(counters_ops)?,
            (5, 6) => self.hyperball::<5, 6>(counters_ops)?,
            (6, 6) => self.hyperball::<6, 6>(counters_ops)?,
            (7, 6) => self.hyperball::<7, 6>(counters_ops)?,
            (8, 6) => self.hyperball::<8, 6>(counters_ops)?,
            (9, 6) => self.hyperball::<9, 6>(counters_ops)?,
            (10, 6) => self.hyperball::<10, 6>(counters_ops)?,
            (11, 6) => self.hyperball::<11, 6>(counters_ops)?,
            (12, 6) => self.hyperball::<12, 6>(counters_ops)?,
            (13, 6) => self.hyperball::<13, 6>(counters_ops)?,
            (14, 6) => self.hyperball::<14, 6>(counters_ops)?,
            (15, 6) => self.hyperball::<15, 6>(counters_ops)?,
            (16, 6) => self.hyperball::<16, 6>(counters_ops)?,
            _ => {
                return Err(format!(
                    concat!(
                        "The provided precision `{:?}` and bits `{:?}` combo is not supported."
                    ),
                    precision, bits
                ));
            }
        })
    }

    /// Returns an approximation of the total distances centrality for all nodes in the graph.
    ///
    /// This method applies the HyperBall algorithm to compute an approximation of the total distances
    /// from each node. The precision parameter indicates the number of bits to use to
    /// represent the HyperLogLog registers. The higher the precision, the more accurate the
    /// results, but also the more memory required and the slower the algorithm.
    ///
    /// # Arguments
    /// * `precision`: Option<u8> - The number of bits to use to represent the HyperLogLog registers. By default 6.
    /// * `bits`: Option<u8> - The number of bits to use for the HyperLogLog counters. It must be either 5 or 6, and by default 6.
    ///
    /// # Returns
    /// A vector of f32 values containing the approximated total distances for each node.
    ///
    pub fn get_approximated_total_distances(
        &self,
        precision: Option<u8>,
        bits: Option<u8>,
    ) -> Result<Vec<f32>> {
        self.dispatch_hyperball(
            precision,
            bits,
            |centrality: &mut f32, current_count, previous_count, iteration_number| {
                *centrality += iteration_number as f32 * (current_count - previous_count);
            },
        )
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
    /// * `bits`: Option<u8> - The number of bits to use for the HyperLogLog counters. It must be either 5 or 6, and by default 6.
    ///
    /// # Returns
    /// A vector of f32 values containing the approximated closeness centrality for each node.
    ///
    pub fn get_approximated_closeness_centrality(
        &self,
        precision: Option<u8>,
        bits: Option<u8>,
    ) -> Result<Vec<f32>> {
        let mut total_distances = self.get_approximated_total_distances(precision, bits)?;

        total_distances
            .par_iter_mut()
            .filter(|total_distance| !total_distance.is_zero())
            .for_each(|total_distance| {
                *total_distance = total_distance.recip();
            });

        Ok(total_distances)
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
    /// * `bits`: Option<u8> - The number of bits to use for the HyperLogLog counters. It must be either 5 or 6, and by default 6.
    ///
    /// # Returns
    /// A vector of f32 values containing the approximated harmonic centrality for each node.
    ///
    pub fn get_approximated_harmonic_centrality(
        &self,
        precision: Option<u8>,
        bits: Option<u8>,
    ) -> Result<Vec<f32>> {
        self.dispatch_hyperball(
            precision,
            bits,
            |centrality: &mut f32, current_count, previous_count, iteration_number| {
                *centrality += (iteration_number as f32).recip() * (current_count - previous_count);
            },
        )
    }
}
