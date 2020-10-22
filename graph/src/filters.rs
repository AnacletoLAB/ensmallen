use super::*;

impl Graph {
    /// Return graph filtered by given weights range.
    ///
    /// # Arguments
    ///
    /// min_weight: Option<WeightT>,
    ///     Minimum weight to use to filter edges.
    /// max_weight: Option<WeightT>,
    ///     Maximum weight to use to filter edges.
    /// verbose: bool,
    ///     Wether to show the loading bar.
    ///
    pub fn filter_weights(
        &self,
        min_weight: Option<WeightT>,
        max_weight: Option<WeightT>,
        verbose: bool,
    ) -> Result<Graph, String> {
        Graph::from_integer_unsorted(
            self.get_edges_quadruples()
                .filter_map(|(_, src, dst, edge_type, weight)| {
                    if let (Some(_min), Some(w)) = (min_weight, weight) {
                        if _min > w {
                            return None;
                        }
                    }
                    if let (Some(_max), Some(w)) = (max_weight, weight) {
                        if w >= _max {
                            return None;
                        }
                    }
                    Some(Ok((src, dst, edge_type, weight)))
                }),
            self.nodes.clone(),
            self.node_types.clone(),
            match &self.edge_types {
                Some(ets) => Some(ets.vocabulary.clone()),
                None => None,
            },
            self.directed,
            format!(
                "({} - filtered by weights with min: {:?} and max: {:?}",
                self.name, min_weight, max_weight
            ),
            false,
            verbose,
        )
    }
}
