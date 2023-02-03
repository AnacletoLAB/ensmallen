use std::sync::atomic::{AtomicBool, Ordering};

use super::*;
use rand::prelude::*;
use rayon::prelude::*;

impl Graph {
    /// Returns 2-approximated verted cover bitvec using greedy algorithm.
    ///
    /// # Arguments
    /// * `approach`: Option<&str> - The approach name to be used. By default, the edge list order is used.
    /// * `sequential`: Option<bool> - Whether to proceed sequantially or concurrently. By default, sequential.
    /// * `insert_only_source`: Option<bool> - Whether to insert only the source node or both source and destination.
    /// * `random_seed`: Option<u64> - The random seed to be used for the stocastic approaches.
    ///
    /// # Possible approaches
    /// There exist many possible approaches, which mostly differ by choosing whether
    /// to execute the process in parallel or sequentially, and how they sort the nodes.
    ///
    /// * `arbitrary` - Just use the order of nodes as they are loaded in the graph.
    /// * `decreasing_node_degree` - Sort the nodes by decreasing node degree.
    /// * `increasing_node_degree` - Sort the nodes by increasing node degree.
    /// * `random` - Shuffle the nodes using the provided random seed.
    ///
    /// # Implementative details
    /// We DO NOT provide a loading bar for this method because the loading bar
    /// iterative step is slower than the actual iteration.
    ///
    /// # References
    /// This implementation is described in ["A local-ratio theorem for approximating the weighted vertex cover problem"](http://www.cs.technion.ac.il/~reuven/PDF/vc_lr.pdf).
    ///
    pub fn get_vertex_cover(
        &self,
        approach: Option<&str>,
        sequential: Option<bool>,
        insert_only_source: Option<bool>,
        random_seed: Option<u64>,
    ) -> Result<Vec<bool>> {
        let sequential = sequential.unwrap_or(true);
        let insert_only_source = insert_only_source.unwrap_or(true);
        let approach = approach.unwrap_or("arbitrary");
        let random_seed = random_seed.unwrap_or(45647655);

        let mut vertex_cover: Vec<bool> = vec![false; self.get_number_of_nodes() as usize];

        let mut node_ids: Vec<NodeT> = self.get_node_ids();

        match approach {
            "arbitrary" => {}
            "decreasing_node_degree" => {
                node_ids.par_sort_unstable_by(|&a, &b| unsafe {
                    self.get_unchecked_node_degree_from_node_id(b)
                        .partial_cmp(&self.get_unchecked_node_degree_from_node_id(a))
                        .unwrap()
                });
            }
            "increasing_node_degree" => {
                node_ids.par_sort_unstable_by(|&a, &b| unsafe {
                    self.get_unchecked_node_degree_from_node_id(a)
                        .partial_cmp(&self.get_unchecked_node_degree_from_node_id(b))
                        .unwrap()
                });
            }
            "random" => {
                let mut rng = SmallRng::seed_from_u64(splitmix64(random_seed) as EdgeT);
                node_ids.shuffle(&mut rng);
            }
            approach => {
                return Err(format!(
                    concat!(
                        "You have provided as approach `{}`, but this is not supported. ",
                        "The supported approaches are:\n",
                        "1) `arbitrary`, where we use the nodes original order.\n",
                        "2) `decreasing_node_degree`, where we sort the nodes by decreasing node degree.\n",
                        "3) `increasing_node_degree`, where we sort the nodes by increasing node degree.\n",
                        "4) `random`, where shuffle the nodes at random, using the provided random seed.\n",
                        "If you intend to try out some other unavailable order, ",
                        "please do open an issue and pull request on GitHub."
                    ),
                    approach
                ));
            }
        };

        if sequential {
            // We iterate the node IDs from higher to lower
            node_ids.into_iter().for_each(|node_id| unsafe {
                if vertex_cover[node_id as usize] {
                    return;
                }
                for dst_node_id in
                    self.iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                {
                    if !vertex_cover[dst_node_id as usize] {
                        vertex_cover[node_id as usize] = true;
                        if !insert_only_source {
                            vertex_cover[dst_node_id as usize] = true;
                        }
                        return;
                    }
                }
            });
        } else {
            let atomic_vertex_cover: Vec<AtomicBool> = unsafe {
                std::mem::transmute::<Vec<bool>, Vec<AtomicBool>>(vec![
                    false;
                    self.get_number_of_nodes()
                        as usize
                ])
            };

            // We iterate the node IDs from higher to lower
            node_ids.into_par_iter().for_each(|node_id| unsafe {
                if atomic_vertex_cover[node_id as usize].load(Ordering::Relaxed) {
                    return;
                }
                for dst_node_id in
                    self.iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                {
                    if !atomic_vertex_cover[dst_node_id as usize].load(Ordering::Relaxed) {
                        atomic_vertex_cover[node_id as usize].store(true, Ordering::Relaxed);
                        if !insert_only_source {
                            atomic_vertex_cover[dst_node_id as usize]
                                .store(true, Ordering::Relaxed);
                        }
                        return;
                    }
                }
            });

            vertex_cover =
                unsafe { std::mem::transmute::<Vec<AtomicBool>, Vec<bool>>(atomic_vertex_cover) };
        }

        Ok(vertex_cover)
    }
}
