use super::*;
use rayon::iter::ParallelIterator;

impl Graph {
    /// Compute the maximum and minimum edge weight and cache it
    fn compute_max_and_min_edge_weight(&self) {
        let mut cache = unsafe { &mut (*self.cache.get()) };

        let (min, max) = match self.par_iter_edge_weights() {
            Ok(iter) => {
                let (min, max) = iter.map(|w| (w, w)).reduce(
                    || (WeightT::NAN, WeightT::NAN),
                    |(min_a, max_a), (min_b, max_b)| (min_a.min(min_b), max_a.max(max_b)),
                );
                (Ok(min), Ok(max))
            }
            Err(e) => (Err(e.clone()), Err(e)),
        };

        cache.min_edge_weight = Some(min);
        cache.max_edge_weight = Some(max);
    }

    cached_property!(get_mininum_edge_weight, Result<WeightT>, compute_max_and_min_edge_weight, min_edge_weight,
    /// Return the minimum weight, if graph has weights.
    ///
    /// # Example
    /// To get the minimum edge weight you can use:
    /// ```rust
    /// # let graph_with_weights = graph::test_utilities::load_ppi(false, false, true, true, false, false);
    /// # let graph_without_weights = graph::test_utilities::load_ppi(false, false, false, true, false, false);
    /// assert!(graph_with_weights.get_mininum_edge_weight().is_ok());
    /// assert!(graph_without_weights.get_mininum_edge_weight().is_err());
    /// println!("The graph minimum weight is {:?}.", graph_with_weights.get_mininum_edge_weight());
    /// ```
    ///
    /// # Raises
    /// * If the graph does not contain edge weights.
        );

    cached_property!(get_maximum_edge_weight, Result<WeightT>, compute_max_and_min_edge_weight, max_edge_weight,
    /// Return the maximum weight, if graph has weights.
    ///
    /// # Example
    /// To get the maximum edge weight you can use:
    /// ```rust
    /// # let graph_with_weights = graph::test_utilities::load_ppi(false, false, true, true, false, false);
    /// # let graph_without_weights = graph::test_utilities::load_ppi(false, false, false, true, false, false);
    /// assert!(graph_with_weights.get_maximum_edge_weight().is_ok());
    /// assert!(graph_without_weights.get_maximum_edge_weight().is_err());
    /// println!("The graph maximum weight is {:?}.", graph_with_weights.get_maximum_edge_weight());
    /// ```
    ///
    /// # Raises
    /// * If the graph does not contain edge weights.
        );
}
