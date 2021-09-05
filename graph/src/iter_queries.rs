use super::*;
use rayon::prelude::*;

/// # Iterators
/// The methods must follow the following naming convenction
/// * `/iter_(.+?)_from_(.+)/`
/// * `/iter_(.+?)_from_(.+)_unchecked/`
/// * `/par_iter_(.+?)_from_(.+)/`
/// * `/par_iter_(.+?)_from_(.+)_unchecked/`
impl Graph {
    /// Returns range of the edge ids of edges starting from the given source node.
    ///
    /// # Arguments
    ///
    /// * `src`: NodeT - Source node of the edge.
    ///
    /// # Safety
    /// If the given node ID does not exist in the graph the method will panic.
    pub unsafe fn iter_unchecked_edge_ids_from_source_node_id(
        &self,
        src: NodeT,
    ) -> std::ops::Range<usize> {
        let (min_edge_id, max_edge_id) =
            self.get_unchecked_minmax_edge_ids_from_source_node_id(src);
        min_edge_id as usize..max_edge_id as usize
    }

    /// Returns range of the edge ids of edges inbound to the given destination node.
    ///
    /// # Arguments
    ///
    /// * `dst`: NodeT - Source node of the edge.
    ///
    /// # Safety
    /// If the given node ID does not exist in the graph the method will panic.
    ///
    /// TODO: refactor this to be faster for directed graphs when
    /// the support datastructure is implemented.
    pub unsafe fn iter_unchecked_edge_ids_from_destination_node_id(
        &self,
        dst: NodeT,
    ) -> Box<dyn Iterator<Item = EdgeT> + Send + '_> {
        if self.is_directed() {
            Box::new(self.iter_directed_edge_node_ids().filter_map(
                move |(edge_id, _, this_dst)| {
                    if this_dst == dst {
                        Some(edge_id)
                    } else {
                        None
                    }
                },
            ))
        } else {
            Box::new(
                self.iter_unchecked_edge_ids_from_source_node_id(dst)
                    .map(|edge_id| edge_id as EdgeT),
            )
        }
    }

    /// Returns iterator over the edge weights that have given node ID as source.
    ///
    /// This method assumes that the given source node ID exists in the graph.
    /// Additionally it assumes that the graph has weights.
    /// If either one of the above assumptions are not true, it will panic.
    ///
    /// # Arguments
    /// * `source_node_id`: NodeT - The source node whose weights are to be returned.
    ///
    /// # Safety
    /// If the given node ID does not exist in the graph the method will panic.
    pub unsafe fn iter_unchecked_edge_weights_from_source_node_id(
        &self,
        source_node_id: NodeT,
    ) -> impl Iterator<Item = WeightT> + '_ {
        self.weights
            .as_ref()
            .map(|weights| {
                weights[self.iter_unchecked_edge_ids_from_source_node_id(source_node_id)]
                    .iter()
                    .cloned()
            })
            .unwrap()
    }

    /// Returns iterator over the edge weights that have given node ID as destination.
    ///
    /// This method assumes that the given destination node ID exists in the graph.
    /// Additionally it assumes that the graph has weights.
    /// If either one of the above assumptions are not true, it will panic.
    ///
    /// # Arguments
    /// * `destination_node_id`: NodeT - The destination node whose weights are to be returned.
    ///
    /// # Safety
    /// If the given node ID does not exist in the graph the method will panic.
    pub unsafe fn iter_unchecked_edge_weights_from_destination_node_id(
        &self,
        destination_node_id: NodeT,
    ) -> impl Iterator<Item = WeightT> + '_ {
        self.weights
            .as_ref()
            .map(move |weights| {
                self.iter_unchecked_edge_ids_from_destination_node_id(destination_node_id)
                    .map(move |edge_id| weights[edge_id as usize])
            })
            .unwrap()
    }

    /// Returns range of the edge ids of edges starting from the given source node.
    ///
    /// # Arguments
    ///
    /// * `src`: NodeT - Source node of the edge.
    ///
    /// # Safety
    /// If the given node ID does not exist in the graph the method will panic.
    pub unsafe fn par_iter_unchecked_edge_ids_from_source_node_id(
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
    /// # Safety
    /// If any the given node IDs does not exist in the graph the method will panic.
    pub unsafe fn iter_unchecked_edge_ids_from_node_ids(
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
    /// # Safety
    /// If the given node ID does not exist in the graph the method will panic.
    pub unsafe fn iter_unchecked_neighbour_node_ids_from_source_node_id(
        &self,
        src: NodeT,
    ) -> Box<dyn Iterator<Item = NodeT> + Send + '_> {
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

    /// Return iterator over sources of the given destination node.
    ///
    /// # Arguments
    /// * `dst`: NodeT - The node whose neighbours are to be retrieved.
    ///
    /// # Safety
    /// If the given node ID does not exist in the graph the method will panic.
    ///
    /// TODO! make this faster for directed graphs!
    pub unsafe fn iter_unchecked_neighbour_node_ids_from_destination_node_id(
        &self,
        dst: NodeT,
    ) -> Box<dyn Iterator<Item = NodeT> + Send + '_> {
        if self.is_directed() {
            Box::new(
                self.iter_directed_edge_node_ids()
                    .filter_map(
                        move |(_, src, this_dst)| {
                            if this_dst == dst {
                                Some(src)
                            } else {
                                None
                            }
                        },
                    ),
            )
        } else {
            self.iter_unchecked_neighbour_node_ids_from_source_node_id(dst)
        }
    }

    /// Return parallel iterator over NodeT of destinations of the given node src.
    ///
    /// # Arguments
    /// * `src`: NodeT - The node whose neighbours are to be retrieved.
    ///
    /// # Safety
    /// If the given node ID does not exist in the graph the method will panic.
    /// For now, this iterator only works when the destinations are cached.
    pub unsafe fn par_iter_unchecked_neighbour_node_ids_from_source_node_id(
        &self,
        src: NodeT,
    ) -> impl IndexedParallelIterator<Item = NodeT> + Send + '_ {
        match &self.destinations {
            Some(dsts) => dsts[self.iter_unchecked_edge_ids_from_source_node_id(src)]
                .par_iter()
                .cloned(),
            None => panic!(concat!(
                "The parallel iteration of neighbours ",
                "without the cached destinations ",
                "is not currently supported."
            )),
        }
    }

    /// Return iterator over neighbours intersection.
    ///
    /// # Arguments
    /// * `first_src_node_id`: NodeT - The first node whose neighbours are to be retrieved.
    /// * `second_src_node_id`: NodeT - The second node whose neighbours are to be retrieved.
    ///
    /// # Safety
    /// If any of the given node ID does not exist in the graph the method will panic.
    pub unsafe fn iter_unchecked_neighbour_node_ids_intersection_from_source_node_ids(
        &self,
        first_src_node_id: NodeT,
        second_src_node_id: NodeT,
    ) -> impl Iterator<Item = NodeT> + Send + '_ {
        iter_set::intersection(
            self.iter_unchecked_neighbour_node_ids_from_source_node_id(first_src_node_id),
            self.iter_unchecked_neighbour_node_ids_from_source_node_id(second_src_node_id),
        )
    }

    /// Return iterator over neighbours union.
    ///
    /// # Arguments
    /// * `first_src_node_id`: NodeT - The first node whose neighbours are to be retrieved.
    /// * `second_src_node_id`: NodeT - The second node whose neighbours are to be retrieved.
    ///
    /// # Safety
    /// If any of the given node ID does not exist in the graph the method will panic.
    pub unsafe fn iter_unchecked_neighbour_node_ids_union_from_source_node_ids(
        &self,
        first_src_node_id: NodeT,
        second_src_node_id: NodeT,
    ) -> impl Iterator<Item = NodeT> + Send + '_ {
        iter_set::union(
            self.iter_unchecked_neighbour_node_ids_from_source_node_id(first_src_node_id),
            self.iter_unchecked_neighbour_node_ids_from_source_node_id(second_src_node_id),
        )
    }

    /// Return iterator over neighbours difference.
    ///
    /// # Arguments
    /// * `first_src_node_id`: NodeT - The first node whose neighbours are to be retrieved.
    /// * `second_src_node_id`: NodeT - The second node whose neighbours are to be retrieved.
    ///
    /// # Safety
    /// If any of the given node ID does not exist in the graph the method will panic.
    pub unsafe fn iter_unchecked_neighbour_node_ids_difference_from_source_node_ids(
        &self,
        first_src_node_id: NodeT,
        second_src_node_id: NodeT,
    ) -> impl Iterator<Item = NodeT> + Send + '_ {
        iter_set::difference(
            self.iter_unchecked_neighbour_node_ids_from_source_node_id(first_src_node_id),
            self.iter_unchecked_neighbour_node_ids_from_source_node_id(second_src_node_id),
        )
    }

    /// Return iterator over NodeT of destinations of the given node src.
    ///
    /// # Arguments
    /// * `src`: NodeT - The node whose neighbour names are to be retrieved.
    ///
    /// # Safety
    /// If the given node ID does not exist in the graph the method will panic.
    pub unsafe fn iter_unchecked_neighbour_node_names_from_source_node_id(
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
    ) -> Result<impl Iterator<Item = EdgeT>> {
        let (min_edge_id, max_edge_id) = self.get_minmax_edge_ids_from_node_ids(src, dst)?;
        Ok(min_edge_id..max_edge_id)
    }

    /// Returns iterator over edge IDs and their properties with given edge type.
    ///
    /// # Arguments
    /// * `edge_type_id`: Option<EdgeTypeT> - Edge type ID to extract.
    /// * `directed`: bool - Whether to iterate the edge list as directed or undirected.
    ///
    /// # Raises
    /// * If there are no edge types in the graph.
    /// * If the given edge type ID does not exist in the graph.
    pub fn iter_edge_node_ids_and_edge_type_id_from_edge_type_id(
        &self,
        edge_type_id: Option<EdgeTypeT>,
        directed: bool,
    ) -> Result<impl Iterator<Item = (EdgeT, NodeT, NodeT, Option<EdgeTypeT>)> + '_> {
        self.validate_edge_type_id(edge_type_id)
            .map(|edge_type_id| {
                self.iter_edge_node_ids_and_edge_type_id(directed)
                    .filter(move |(_, _, _, this_edge_type_id)| *this_edge_type_id == edge_type_id)
            })
    }

    /// Returns iterator over node IDs and their properties with given node type.
    ///
    /// # Arguments
    /// * `node_type_id`: Option<NodeTypeT> - node type ID to extract.
    ///
    /// # Raises
    /// * If there are no node types in the graph.
    /// * If the given node type ID does not exist in the graph.
    pub fn iter_node_ids_and_node_type_ids_from_node_type_id(
        &self,
        node_type_id: Option<NodeTypeT>,
    ) -> Result<impl Iterator<Item = (NodeT, Option<Vec<NodeTypeT>>)> + '_> {
        self.validate_node_type_id(node_type_id)
            .map(|node_type_id| {
                self.iter_node_ids_and_node_type_ids().filter(
                    move |(_, this_node_type_ids)| match (this_node_type_ids, &node_type_id) {
                        (Some(tntis), Some(nti)) => tntis.contains(nti),
                        (None, None) => true,
                        _ => false,
                    },
                )
            })
    }

    /// Returns iterator over node names and their properties with given node type.
    ///
    /// # Arguments
    /// * `node_type_id`: Option<NodeTypeT> - node type ID to extract.
    ///
    /// # Raises
    /// * If there are no node types in the graph.
    /// * If the given node type ID does not exist in the graph.
    pub fn iter_node_names_and_node_type_names_from_node_type_id(
        &self,
        node_type_id: Option<NodeTypeT>,
    ) -> Result<
        impl Iterator<Item = (NodeT, String, Option<Vec<NodeTypeT>>, Option<Vec<String>>)> + '_,
    > {
        self.validate_node_type_id(node_type_id)
            .map(|node_type_id| {
                self.iter_node_names_and_node_type_names().filter(
                    move |(_, _, this_node_type_ids, _)| match (this_node_type_ids, &node_type_id) {
                        (Some(tntis), Some(nti)) => tntis.contains(nti),
                        (None, None) => true,
                        _ => false,
                    },
                )
            })
    }

    /// Returns iterator over edge node names and their properties with given edge type.
    ///
    /// # Arguments
    /// * `edge_type_id`: Option<EdgeTypeT> - Edge type ID to extract.
    /// * `directed`: bool - Whether to iterate the edge list as directed or undirected.
    ///
    /// # Raises
    /// * If there are no edge types in the graph.
    /// * If the given edge type ID does not exist in the graph.
    pub fn iter_edge_node_names_and_edge_type_name_from_edge_type_id(
        &self,
        edge_type_id: Option<EdgeTypeT>,
        directed: bool,
    ) -> Result<
        impl Iterator<
                Item = (
                    EdgeT,
                    NodeT,
                    String,
                    NodeT,
                    String,
                    Option<EdgeTypeT>,
                    Option<String>,
                ),
            > + '_,
    > {
        self.validate_edge_type_id(edge_type_id)
            .map(|edge_type_id| {
                self.iter_edge_node_names_and_edge_type_name(directed)
                    .filter(move |(_, _, _, _, _, this_edge_type_id, _)| {
                        *this_edge_type_id == edge_type_id
                    })
            })
    }
}
