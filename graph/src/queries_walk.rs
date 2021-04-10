use super::*;
use vec_rand::sorted_unique_sub_sampling;

/// # Walk Queries
/// These are the queries that are used mainly in the random walk.
impl Graph {

    pub(crate) fn get_node_edges_and_destinations(
        &self,
        max_neighbours: Option<NodeT>,
        random_state: u64,
        node: NodeT,
    ) -> (EdgeT, EdgeT, Option<Vec<NodeT>>, Option<Vec<u64>>) {
        // We retrieve the range of edge ids, the minimum and maximum value.
        let (min_edge_id, max_edge_id) = self.get_minmax_edge_ids_by_source_node_id(node);

        // We check if subsampling is enabled and if so, if it makes sense:
        // that is, if the range of neighbours (max_edge_id-min_edge_id) is smaller
        // than the required sub-sampling we do not use it as it would be useless.
        if let Some(indices) = max_neighbours.and_then(|mn| {
            sorted_unique_sub_sampling(min_edge_id, max_edge_id, mn as u64, random_state).ok()
        }) {
            let destinations: Vec<NodeT> = match self
                .cached_destinations
                .as_ref()
                .and_then(|cds| cds.get(&node))
            {
                Some(dsts) => {
                    indices
                        .iter()
                        .map(|edge_id| dsts[(*edge_id - min_edge_id) as usize])
                        .collect()
                },
                None => {
                    indices
                        .iter()
                        .map(|edge_id| self.get_destination_node_id_by_edge_id(*edge_id).unwrap())
                        .collect()
                },
            };
            return (min_edge_id, max_edge_id, Some(destinations), Some(indices));
        }

        // If the destinations are stored explicitly because the time-memory tradeoff is enabled we are done.
        if self.destinations.is_some() {
            return (min_edge_id, max_edge_id, None, None);
        }

        // Finally if we are using the cache without sub-sampling
        let destinations = match self
            .cached_destinations
            .as_ref()
            .map_or(false, |cds| cds.contains_key(&node))
        {
            true => None,
            false => Some(
                self.edges.iter_in_range(
                    self.encode_edge(node, 0)..self.encode_edge(node + 1, 0)
                ).map(|edge| {
                    self.decode_edge(edge).1
                }).collect()
            ),
        };
        (min_edge_id, max_edge_id, destinations, None)
    }
    
    /// TODO:! add doc
    pub(crate) fn get_destinations_slice<'a>(
        &'a self,
        min_edge_id: EdgeT,
        max_edge_id: EdgeT,
        node: NodeT,
        destinations: &'a Option<Vec<NodeT>>,
    ) -> &'a [NodeT] {
        match (&self.destinations, &self.cached_destinations, destinations) {
            (_, _, Some(dsts)) => &dsts.as_slice(),
            (Some(dsts), None, None) => &dsts[min_edge_id as usize..max_edge_id as usize],
            (None, Some(dsts), None) => dsts.get(&node).unwrap(),
            _ => unreachable!(
                "It is not possible to have both destinations and cached destinations at once."
            ),
        }
    }
}
