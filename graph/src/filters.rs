use super::*;
use indicatif::ProgressIterator;

impl Graph {
    /// Return graph filtered by given weights range.
    ///
    /// # Arguments
    ///
    /// * node_names: Option<Vec<String>> - The node names to keep.
    /// * node_types: Option<Vec<String>> - The node types to keep.
    /// * edge_types: Option<Vec<String>> - The edge types to keep.
    /// * min_weight: Option<WeightT> - Minimum weight to use to filter edges.
    /// * max_weight: Option<WeightT> - Maximum weight to use to filter edges.
    /// * verbose: bool - Wether to show the loading bar.
    ///
    pub fn filter(
        &self,
        node_names: Option<Vec<String>>,
        node_types: Option<Vec<Option<String>>>,
        edge_types: Option<Vec<Option<String>>>,
        min_weight: Option<WeightT>,
        max_weight: Option<WeightT>,
        verbose: bool,
    ) -> Result<Graph, String> {
        if let (Some(min_w), Some(max_w)) = (min_weight, max_weight) {
            if min_w >= max_w {
                return Err(format!(
                    "The given minimum weight ({}) is greater or equal than the given maximum weight ({})!",
                    min_w, max_w
                ));
            }
        }

        let pb = get_loading_bar(
            verbose,
            format!("Building filtered {}", self.name).as_ref(),
            self.get_directed_edges_number() as usize,
        );

        let node_ids = self.get_filter_bitmap(node_names, node_types)?;
        let edge_types_ids = edge_types.map_or(Ok::<_, String>(None), |ets| Ok(Some(self.translate_edge_types(ets)?)));
        let edge_types_ids = edge_types_ids?;

        Graph::build_graph(
            self.get_edges_quadruples(true)
                .progress_with(pb)
                .filter_map(|(_, src, dst, edge_type, weight)| {
                    if let Some(nis) = &node_ids {
                        if !nis.contains(src) || !nis.contains(dst) {
                            return None;
                        }
                    }
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
                    if let Some(ets) =  &edge_types_ids {
                        if !ets.contains(&edge_type) {
                            return None;
                        }
                    }
                    Some(Ok((src, dst, edge_type, weight)))
                }),
            self.get_directed_edges_number(),
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types.as_ref().map(|ets| ets.vocabulary.clone()),
            self.directed,
            self.name.clone(),
            false,
            self.has_edge_types(),
            self.has_weights(),
        )
    }

    /// Return filtered iterator over NodeT of destinations of the given node src.
    ///
    /// # Arguments
    ///
    /// * src: NodeT - The source node.
    /// * node_names: Option<Vec<String>> - The node names to keep.
    /// * node_types: Option<Vec<String>> - The node types to keep.
    /// * edge_types: Option<Vec<String>> - The edge types to keep.
    /// * min_weight: Option<WeightT> - Minimum weight to use to filter edges.
    /// * max_weight: Option<WeightT> - Maximum weight to use to filter edges.
    /// * verbose: bool - Wether to show the loading bar.
    ///
    pub fn get_filtered_neighbours_range(
        &self,
        src: NodeT,
        node_names: Option<Vec<String>>,
        node_types: Option<Vec<Option<String>>>,
        edge_types: Option<Vec<Option<String>>>,
        min_weight: Option<WeightT>,
        max_weight: Option<WeightT>,
    ) -> Result<impl Iterator<Item = NodeT> + '_, String> {
        let node_ids = self.get_filter_bitmap(node_names, node_types)?;
        let edge_types_ids =
            edge_types.map_or(Ok::<_, String>(None), |ets| Ok(Some(self.translate_edge_types(ets)?)))?;
        Ok(self
            .get_unchecked_destinations_range(src)
            .filter_map(move |edge_id| {
                if let Some(ets) = &edge_types_ids {
                    if !ets.contains(& self.get_unchecked_edge_type(edge_id)) {
                        return None;
                    }
                }
                let weight = self.get_unchecked_edge_weight(edge_id);
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
                let dst = self.get_destination(edge_id);
                if let Some(nis) = &node_ids {
                    if !nis.contains(src) || !nis.contains(dst) {
                        return None;
                    }
                }
                Some(dst)
            }))
    }
}
