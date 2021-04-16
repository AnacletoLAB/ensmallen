use super::*;
use rayon::prelude::*;

/// # Iterators
/// * `iter_(.+?)_from_(.+)`
/// * `iter_unchecked_(.+?)_from_(.+)`
/// * `par_iter_(.+?)_from_(.+)`
/// * `par_iter_unchecked_(.+?)_from_(.+)`
impl Graph {
    /// Returns range of the edge ids of edges starting from the given source node.
    ///
    /// # Arguments
    ///
    /// * `src`: NodeT - Source node of the edge.
    ///
    pub fn iter_unchecked_edge_ids_from_source_node_id(
        &self,
        src: NodeT,
    ) -> std::ops::Range<usize> {
        let (min_edge_id, max_edge_id) =
            self.get_unchecked_minmax_edge_ids_from_source_node_id(src);
        min_edge_id as usize..max_edge_id as usize
    }

    /// Returns range of the edge ids of edges starting from the given source node.
    ///
    /// # Arguments
    ///
    /// * `src`: NodeT - Source node of the edge.
    ///
    pub fn par_iter_unchecked_edge_ids_from_source_node_id(
        &self,
        src: NodeT,
    ) -> impl ParallelIterator<Item = NodeT> + '_ {
        self.iter_unchecked_edge_ids_from_source_node_id(src)
            .into_par_iter()
            .map(|node_id| node_id as NodeT)
    }

    /// Returns range of multigraph minimum and maximum edge ids with same source and destination nodes and different edge type.
    ///
    /// # Arguments
    ///
    /// * `src`: NodeT - Source node of the edge.
    /// * `dst`: NodeT -  Destination node of the edge.
    ///
    pub fn iter_unchecked_edge_ids_from_node_ids(
        &self,
        src: NodeT,
        dst: NodeT,
    ) -> impl Iterator<Item = EdgeT> {
        let (min_edge_id, max_edge_id) = self.get_unchecked_minmax_edge_ids_from_node_ids(src, dst);
        min_edge_id..max_edge_id
    }

    /// Return iterator over NodeT of destinations of the given node src.
    ///
    /// # Arguments
    /// * `src`: NodeT - The node whose neighbours are to be retrieved.
    ///
    pub fn iter_unchecked_neighbour_node_ids_from_source_node_id(
        &self,
        src: NodeT,
    ) -> Box<dyn Iterator<Item = NodeT> + '_> {
        match &self.destinations {
            Some(dsts) => Box::new(
                dsts[self.iter_unchecked_edge_ids_from_source_node_id(src)]
                    .iter()
                    .cloned(),
            ),
            None => Box::new(
                self.edges
                    .iter_in_range(self.encode_edge(src, 0)..self.encode_edge(src + 1, 0))
                    .map(move |edge| self.decode_edge(edge).1),
            ),
        }
    }

    /// Return iterator over NodeT of destinations of the given node src.
    ///
    /// # Arguments
    /// * `src`: NodeT - The node whose neighbour names are to be retrieved.
    ///
    pub fn iter_unchecked_neighbour_node_names_from_source_node_id(
        &self,
        src: NodeT,
    ) -> impl Iterator<Item = String> + '_ {
        self.iter_unchecked_neighbour_node_ids_from_source_node_id(src)
            .map(move |dst| self.get_unchecked_node_name_from_node_id(dst))
    }

    /// Returns option of range of multigraph minimum and maximum edge ids with same source and destination nodes and different edge type.
    ///
    /// # Arguments
    ///
    /// * `src`: NodeT - Source node id of the edge.
    /// * `dst`: NodeT -  Destination node id of the edge.
    ///
    pub fn iter_edge_ids_from_node_ids(
        &self,
        src: NodeT,
        dst: NodeT,
    ) -> Result<impl Iterator<Item = EdgeT>, String> {
        let (min_edge_id, max_edge_id) = self.get_minmax_edge_ids_from_node_ids(src, dst)?;
        Ok(min_edge_id..max_edge_id)
    }
}
