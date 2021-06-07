use super::*;
use vec_rand::sorted_unique_sub_sampling;

/// # Walk Queries
/// These are the queries that are used mainly in the random walk.
impl Graph {
    /// Returns quadruple with minimum and maximum edge ids, destination nodes and its indices if subsampling was required.
    ///
    /// If max neighbours was provided the subsampling is done by using the
    /// sorted unique sub-sampling (SUSS) algorithm.
    ///
    /// # Arguments
    /// * `max_neighbours`: Option<NodeT> - Optional maximum number of neighbours to consider.
    /// * `random_state`: u64 - The random state to use for the sampling if the maximum neighbours are asked.
    /// * `source_node_id`: NodeT - The source node ID to extract edge IDs and destination node IDs.
    pub(crate) unsafe fn get_unchecked_edges_and_destinations_from_source_node_id(
        &self,
        max_neighbours: Option<NodeT>,
        random_state: u64,
        source_node_id: NodeT,
    ) -> (EdgeT, EdgeT, Option<Vec<NodeT>>, Option<Vec<u64>>) {
        // We retrieve the range of edge ids, the minimum and maximum value.
        let (min_edge_id, max_edge_id) =
            self.get_unchecked_minmax_edge_ids_from_source_node_id(source_node_id);

        // We check if subsampling is enabled and if so, if it makes sense:
        // that is, if the range of neighbours (max_edge_id-min_edge_id) is smaller
        // than the required sub-sampling we do not use it as it would be useless.
        if let Some(indices) = max_neighbours.and_then(|mn| {
            sorted_unique_sub_sampling(min_edge_id, max_edge_id, mn as u64, random_state).ok()
        }) {
            let destinations: Vec<NodeT> = indices
                .iter()
                .map(|edge_id| self.get_unchecked_destination_node_id_from_edge_id(*edge_id))
                .collect();
            return (min_edge_id, max_edge_id, Some(destinations), Some(indices));
        }

        // If the destinations are stored explicitly because the time-memory tradeoff is enabled we are done.
        if self.destinations.is_some() {
            return (min_edge_id, max_edge_id, None, None);
        }

        // Finally if we are using the cache without sub-sampling
        let destinations = Some(
            self.iter_unchecked_neighbour_node_ids_from_source_node_id(source_node_id)
                .collect(),
        );
        (min_edge_id, max_edge_id, destinations, None)
    }

    /// Returns slice of destinations corresponding to given minmax edge ID and node.
    ///
    /// # Arguments
    /// * `min_edge_id`: EdgeT - Minimum edge ID for the slice.
    /// * `max_edge_id`: EdgeT - Maximum edge ID for the slice.
    /// * `source_node_id`: NodeT - The source node ID.
    /// * `destinations`: &'a Option<Vec<NodeT>> - The optional destinations slice that may have been provided when working with subsampling.
    pub(crate) fn get_destinations_slice<'a>(
        &'a self,
        min_edge_id: EdgeT,
        max_edge_id: EdgeT,
        destinations: &'a Option<Vec<NodeT>>,
    ) -> &'a [NodeT] {
        match (&self.destinations, destinations) {
            (_, Some(dsts)) => &dsts.as_slice(),
            (Some(dsts), None) => &dsts[min_edge_id as usize..max_edge_id as usize],
            _ => unreachable!(
                "It is not possible to have both destinations and cached destinations at once."
            ),
        }
    }
}
