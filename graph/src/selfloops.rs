use crate::constructors::build_graph_from_integers;

use super::*;
use rayon::iter::ParallelIterator;

/// # Selfloops.
impl Graph {
    /// Returns new graph with added in missing self-loops with given edge type and weight.
    ///
    /// # Arguments
    /// `edge_type_name`: Option<&str> - The edge type to use for the selfloops.
    /// `weight`: Option<WeightT> - The weight to use for the new selfloops edges.
    ///
    /// # Raises
    /// * If the edge type for the new selfloops is provided but the graph does not have edge types.
    /// * If the edge weight for the new selfloops is provided but the graph does not have edge weights.
    /// * If the edge weight for the new selfloops is NOT provided but the graph does have edge weights.
    pub fn add_selfloops(
        &self,
        edge_type_name: Option<&str>,
        weight: Option<WeightT>,
    ) -> Result<Graph> {
        let edge_type_id = if edge_type_name.is_some() {
            self.get_edge_type_id_from_edge_type_name(edge_type_name)?
        } else {
            None
        };
        if weight.is_some() ^ self.has_edge_weights() {
            return Err(concat!(
                "The weight for the self-loops must be specified ",
                "only and exclusively if the graph has edge weights."
            )
            .to_string());
        }
        let weight = weight.unwrap_or(WeightT::NAN);
        let total_number_of_edges = self.get_number_of_directed_edges()
            - self.get_number_of_selfloops()
            + self.get_number_of_nodes() as EdgeT;

        build_graph_from_integers(
            Some(
                self.par_iter_directed_edge_node_ids_and_edge_type_id_and_edge_weight()
                    .map(|(_, src, dst, edge_type_id, weight)| {
                        (0, (src, dst, edge_type_id, weight.unwrap_or(WeightT::NAN)))
                    })
                    .chain(
                        self.par_iter_node_ids()
                            .filter(|&node_id| !self.has_selfloop_from_node_id(node_id))
                            .map(|node_id| (0, (node_id, node_id, edge_type_id, weight))),
                    ),
            ),
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types
                .as_ref()
                .as_ref()
                .map(|ets| ets.vocabulary.clone()),
            self.has_edge_weights(),
            self.is_directed(),
            Some(true),
            Some(false),
            Some(false),
            Some(total_number_of_edges),
            false,
            self.has_singleton_nodes_with_selfloops() || self.has_singleton_nodes(),
            self.get_name(),
        )
    }
}
