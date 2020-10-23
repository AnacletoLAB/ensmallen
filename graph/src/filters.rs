use super::*;
use indicatif::ProgressIterator;
use roaring::RoaringBitmap;

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
    pub fn filter(
        &self,
        nodes: Option<Vec<String>>,
        _node_types: Option<Vec<String>>,
        _edge_types: Option<Vec<String>>,
        min_weight: Option<WeightT>,
        max_weight: Option<WeightT>,
        verbose: bool,
    ) -> Result<Graph, String> {
        let pb = get_loading_bar(
            verbose,
            format!("Building filtered {}", self.name).as_ref(),
            self.get_edges_number() as usize,
        );

        let mut node_ids = RoaringBitmap::new();
        if let Some(ns) = nodes {
            node_ids.extend(
                ns.iter()
                    .map(|node_name| self.get_node_id(node_name))
                    .collect::<Result<Vec<NodeT>, String>>()?,
            );
        }

        Graph::build_graph(
            self.get_edges_quadruples().progress_with(pb).filter_map(
                |(_, src, dst, edge_type, weight)| {
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
                },
            ),
            self.get_edges_number(),
            self.nodes.clone(),
            self.node_types.clone(),
            match &self.edge_types {
                Some(ets) => Some(ets.vocabulary.clone()),
                None => None,
            },
            self.directed,
            self.name.clone(),
            false,
        )
    }
}
