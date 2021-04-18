use super::*;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::collections::HashMap;

impl Graph {
    /// Enable extra perks that buys you time as you accept to spend more memory.
    ///
    /// # Arguments
    /// * `vector_sources`: bool - Whether to cache sources into a vector for faster walks.
    /// * `vector_destinations`: bool - Whether to cache destinations into a vector for faster walks.
    /// * `vector_cumulative_node_degrees`: bool - Whether to cache cumulative_node_degrees into a vector for faster walks.
    /// * `cache_size`: Option<f64> - percentage of nodes destinations to cache. This cannot be used with the vector destinations.
    pub fn enable(
        &mut self,
        vector_sources: bool,
        vector_destinations: bool,
        vector_cumulative_node_degrees: bool,
        cache_size: Option<f64>,
    ) -> Result<(), String> {
        if (vector_destinations || self.destinations.is_some()) && (cache_size.is_some() || self.cached_destinations.is_some()){
            return Err(concat!(
                "It is not possible (nor would it make sense) to have both ",
                "partially cached destinations and completely cached vector ",
                "destinations at once.\n",
                "If you want to switch from one to the ",
                "other form of destinations cache remember to run the method ",
                "disable_all to disable all forms of time-memory tradeoffs.\n",
                "Once you have disabled again all trade-offs, you can ",
                "re-enable the any one you would like."
            ).to_string());
        }

        if vector_destinations {
            if self.destinations.is_none() {
                self.destinations = Some(self.get_destinations(true));
            }
        } else {
            self.destinations = None;
        }
        if vector_sources {
            if self.sources.is_none() {
                self.sources = Some(self.get_sources(true));
            }
        } else {
            self.sources = None;
        }
        if vector_cumulative_node_degrees {
            if self.cumulative_node_degrees.is_none() {
                self.cumulative_node_degrees = Some(self.get_cumulative_node_degrees());
            }
        } else {
            self.cumulative_node_degrees = None;
        }
        if let Some(cs) = cache_size {
            if cs <= 0.0 || cs >= 1.0 {
                return Err("Cache size must be between strictly 0 and 1, otherwise just enable the destinations vector.".to_owned());
            }
            let cached_nodes_number: NodeT = (self.get_nodes_number() as f64 * cs) as NodeT;
            if cached_nodes_number == 0 || cached_nodes_number == self.get_nodes_number() {
                return Err("Required cached nodes number cannot be 0 or all the nodes.".to_owned());
            }
            self.cached_destinations = Some(
                self.get_top_k_central_node_ids(cached_nodes_number)
                    .par_iter()
                    .map(|node_id| {
                        (
                            *node_id,
                            self.iter_unchecked_neighbour_node_ids_from_source_node_id(*node_id)
                                .collect::<Vec<NodeT>>(),
                        )
                    })
                    .collect::<HashMap<NodeT, Vec<NodeT>>>(),
            );
        } else {
            self.cached_destinations = None;
        }
        Ok(())
    }

    /// Disable all extra perks, reducing memory impact but incresing time requirements.
    pub fn disable_all(&mut self) {
        self.destinations = None;
        self.sources = None;
        self.cumulative_node_degrees = None;
        self.cached_destinations = None;
    }
}
