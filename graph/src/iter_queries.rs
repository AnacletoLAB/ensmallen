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
        self.edges.iter_unchecked_edge_ids_from_source_node_id(src)
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
            .as_ref()
            .map(|weights| {
                weights[self.iter_unchecked_edge_ids_from_source_node_id(source_node_id)]
                    .iter()
                    .cloned()
            })
            .unwrap()
    }

    /// Returns iterator over the edge types that have given node ID as source.
    ///
    /// This method assumes that the given source node ID exists in the graph.
    /// Additionally it assumes that the graph has edge types.
    /// If either one of the above assumptions are not true, it will panic.
    ///
    /// # Arguments
    /// * `source_node_id`: NodeT - The source node whose weights are to be returned.
    ///
    /// # Safety
    /// If the given node ID does not exist in the graph the method will panic.
    pub unsafe fn iter_unchecked_edge_type_ids_from_source_node_id(
        &self,
        source_node_id: NodeT,
    ) -> impl Iterator<Item = Option<EdgeTypeT>> + '_ {
        self.edge_types
            .as_ref()
            .as_ref()
            .map(|edge_types| {
                edge_types.ids[self.iter_unchecked_edge_ids_from_source_node_id(source_node_id)]
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

    /// Returns iterator over the edge type IDs corresponding to the given edge ID.
    ///
    /// # Arguments
    /// * `src`: NodeT - Source node of the edge.
    /// * `dst`: NodeT -  Destination node of the edge.
    ///
    /// # Safety
    /// If any the given node IDs does not exist in the graph the method will panic.
    pub unsafe fn iter_unchecked_edge_type_ids_from_node_ids(
        &self,
        src: NodeT,
        dst: NodeT,
    ) -> impl Iterator<Item = Option<EdgeTypeT>> + '_ {
        self.iter_unchecked_edge_ids_from_node_ids(src, dst)
            .map(move |edge_id| self.get_unchecked_edge_type_id_from_edge_id(edge_id))
    }

    /// Returns iterator over the edge type IDs corresponding to the given edge ID.
    ///
    /// # Arguments
    /// * `edge_id`: EdgeT - The edge to query for.
    ///
    /// # Safety
    /// If any the given node IDs does not exist in the graph the method will panic.
    pub unsafe fn iter_unchecked_edge_type_ids_from_edge_id(
        &self,
        edge_id: EdgeT,
    ) -> impl Iterator<Item = Option<EdgeTypeT>> + '_ {
        let (src, dst) = self.get_unchecked_node_ids_from_edge_id(edge_id);
        self.iter_unchecked_edge_type_ids_from_node_ids(src, dst)
    }

    #[inline(always)]
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
    ) -> impl Iterator<Item = NodeT> + Send + '_ {
        self.edges
            .iter_unchecked_neighbour_node_ids_from_source_node_id(src)
    }

    #[inline(always)]
    /// Return iterator over edge type ids of the edges connected to the given source node id.
    ///
    /// # Arguments
    /// * `src`: NodeT - The node whose neighbours are to be retrieved.
    ///
    /// # Safety
    /// The behaviour is mot define and may lead to panic when the source node ID
    /// is not present in the graph or the graph does not have edge type ids.
    pub unsafe fn iter_unchecked_edge_type_id_from_source_node_id(
        &self,
        src: NodeT,
    ) -> impl Iterator<Item = Option<EdgeTypeT>> + '_ {
        self.iter_unchecked_edge_ids_from_source_node_id(src)
            .map(move |edge_id| self.get_unchecked_edge_type_id_from_edge_id(edge_id as EdgeT))
    }

    /// Return iterator over edge type ids of the edges connected to the given source node id.
    ///
    /// # Arguments
    /// * `src`: NodeT - The node whose neighbours are to be retrieved.
    ///
    /// # Raises
    /// * If the graph does not have edge types.
    /// * If the given source node ID does not exist in the graph.
    pub fn iter_edge_type_id_from_source_node_id(
        &self,
        src: NodeT,
    ) -> Result<impl Iterator<Item = Option<EdgeTypeT>> + '_> {
        self.validate_node_id(src)?;
        self.must_have_edge_types()?;
        Ok(unsafe { self.iter_unchecked_edge_type_id_from_source_node_id(src) })
    }

    /// Return iterator over NodeT of unique destinations of the given node src, excluding selfloops.
    ///
    /// # Arguments
    /// * `src`: NodeT - The node whose neighbours are to be retrieved.
    ///
    /// # Safety
    /// If the given node ID does not exist in the graph the method will panic.
    pub unsafe fn iter_unchecked_unique_neighbour_node_ids_from_source_node_id(
        &self,
        src: NodeT,
    ) -> impl Iterator<Item = NodeT> + Send + '_ {
        self.iter_unchecked_neighbour_node_ids_from_source_node_id(src)
            .scan(src, move |previous_node, dst| {
                Some(if src == dst {
                    None
                } else if *previous_node == dst {
                    None
                } else {
                    *previous_node = dst;
                    Some(dst)
                })
            })
            .filter_map(|value| value)
    }

    #[inline(always)]
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
        self.edges
            .par_iter_unchecked_neighbour_node_ids_from_source_node_id(src)
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
    pub fn iter_multigraph_edge_ids_from_node_ids(
        &self,
        src: NodeT,
        dst: NodeT,
    ) -> Result<impl Iterator<Item = EdgeT>> {
        let (min_edge_id, max_edge_id) = self.get_minmax_edge_ids_from_node_ids(src, dst)?;
        Ok(min_edge_id..max_edge_id)
    }

    /// Returns iterator over edge node IDs with given edge type.
    ///
    /// # Arguments
    /// * `edge_type_id`: Option<EdgeTypeT> - Edge type ID to extract.
    /// * `directed`: bool - Whether to iterate the edge list as directed or undirected.
    ///
    /// # Raises
    /// * If there are no edge types in the graph.
    /// * If the given edge type ID does not exist in the graph.
    pub fn iter_edge_node_ids_from_edge_type_id(
        &self,
        edge_type_id: Option<EdgeTypeT>,
        directed: bool,
    ) -> Result<impl Iterator<Item = (NodeT, NodeT)> + '_> {
        self.validate_edge_type_id(edge_type_id)
            .map(|edge_type_id| {
                self.iter_edge_node_ids_and_edge_type_id(directed)
                    .filter_map(move |(_, src, dst, this_edge_type_id)| {
                        if this_edge_type_id == edge_type_id {
                            Some((src, dst))
                        } else {
                            None
                        }
                    })
            })
    }

    /// Returns iterator over directed edge node IDs with given edge type.
    ///
    /// # Arguments
    /// * `edge_type_id`: Option<EdgeTypeT> - Edge type ID to extract.
    ///
    /// # Raises
    /// * If there are no edge types in the graph.
    /// * If the given edge type ID does not exist in the graph.
    pub fn iter_directed_edge_node_ids_from_edge_type_id(
        &self,
        edge_type_id: Option<EdgeTypeT>,
    ) -> Result<impl Iterator<Item = (NodeT, NodeT)> + '_> {
        self.validate_edge_type_id(edge_type_id)
            .map(|edge_type_id| {
                self.iter_directed_edge_node_ids_and_edge_type_id()
                    .filter_map(move |(_, src, dst, this_edge_type_id)| {
                        if this_edge_type_id == edge_type_id {
                            Some((src, dst))
                        } else {
                            None
                        }
                    })
            })
    }

    /// Returns parallel iterator over directed edge node IDs with given edge type.
    ///
    /// # Arguments
    /// * `edge_type_id`: Option<EdgeTypeT> - Edge type ID to extract.
    ///
    /// # Raises
    /// * If there are no edge types in the graph.
    /// * If the given edge type ID does not exist in the graph.
    pub fn par_iter_directed_edge_node_ids_from_edge_type_id(
        &self,
        edge_type_id: Option<EdgeTypeT>,
    ) -> Result<impl ParallelIterator<Item = (NodeT, NodeT)> + '_> {
        self.validate_edge_type_id(edge_type_id)
            .map(|edge_type_id| {
                self.par_iter_directed_edge_node_ids_and_edge_type_id()
                    .filter_map(move |(_, src, dst, this_edge_type_id)| {
                        if this_edge_type_id == edge_type_id {
                            Some((src, dst))
                        } else {
                            None
                        }
                    })
            })
    }

    /// Returns iterator over edge node IDs with given edge type name.
    ///
    /// # Arguments
    /// * `edge_type_name`: Option<&str> - Edge type name to extract.
    /// * `directed`: bool - Whether to iterate the edge list as directed or undirected.
    ///
    /// # Raises
    /// * If there are no edge types in the graph.
    /// * If the given edge type name does not exist in the graph.
    pub fn iter_edge_node_ids_from_edge_type_name(
        &self,
        edge_type_name: Option<&str>,
        directed: bool,
    ) -> Result<impl Iterator<Item = (NodeT, NodeT)> + '_> {
        self.get_edge_type_id_from_edge_type_name(edge_type_name)
            .map(|edge_type_id| {
                self.iter_edge_node_ids_and_edge_type_id(directed)
                    .filter_map(move |(_, src, dst, this_edge_type_id)| {
                        if this_edge_type_id == edge_type_id {
                            Some((src, dst))
                        } else {
                            None
                        }
                    })
            })
    }

    /// Returns iterator over directed edge node IDs with given edge type name.
    ///
    /// # Arguments
    /// * `edge_type_name`: Option<EdgeTypeT> - Edge type name to extract.
    ///
    /// # Raises
    /// * If there are no edge types in the graph.
    /// * If the given edge type name does not exist in the graph.
    pub fn iter_directed_edge_node_ids_from_edge_type_name(
        &self,
        edge_type_name: Option<&str>,
    ) -> Result<impl Iterator<Item = (NodeT, NodeT)> + '_> {
        self.get_edge_type_id_from_edge_type_name(edge_type_name)
            .map(|edge_type_id| {
                self.iter_directed_edge_node_ids_and_edge_type_id()
                    .filter_map(move |(_, src, dst, this_edge_type_id)| {
                        if this_edge_type_id == edge_type_id {
                            Some((src, dst))
                        } else {
                            None
                        }
                    })
            })
    }

    /// Returns parallel iterator over directed edge node IDs with given edge type name.
    ///
    /// # Arguments
    /// * `edge_type_name`: Option<EdgeTypeT> - Edge type names to extract.
    ///
    /// # Raises
    /// * If there are no edge types in the graph.
    /// * If the given edge type names does not exist in the graph.
    pub fn par_iter_directed_edge_node_ids_from_edge_type_name(
        &self,
        edge_type_name: Option<&str>,
    ) -> Result<impl ParallelIterator<Item = (NodeT, NodeT)> + '_> {
        self.get_edge_type_id_from_edge_type_name(edge_type_name)
            .map(|edge_type_id| {
                self.par_iter_directed_edge_node_ids_and_edge_type_id()
                    .filter_map(move |(_, src, dst, this_edge_type_id)| {
                        if this_edge_type_id == edge_type_id {
                            Some((src, dst))
                        } else {
                            None
                        }
                    })
            })
    }

    /// Returns parallel iterator over directed edge node names with given edge type id.
    ///
    /// # Arguments
    /// * `edge_type_id`: Option<EdgeTypeT> - Edge type id to extract.
    ///
    /// # Raises
    /// * If there are no edge types in the graph.
    /// * If the given edge type ids does not exist in the graph.
    pub fn par_iter_directed_edge_node_names_from_edge_type_id(
        &self,
        edge_type_id: Option<EdgeTypeT>,
    ) -> Result<impl ParallelIterator<Item = (String, String)> + '_> {
        self.par_iter_directed_edge_node_ids_from_edge_type_id(edge_type_id)
            .map(|iter| {
                iter.map(move |(src, dst)| unsafe {
                    (
                        self.get_unchecked_node_name_from_node_id(src),
                        self.get_unchecked_node_name_from_node_id(dst),
                    )
                })
            })
    }

    /// Returns parallel iterator over directed edge node names with given edge type name.
    ///
    /// # Arguments
    /// * `edge_type_name`: Option<&str> - Edge type name to extract.
    ///
    /// # Raises
    /// * If there are no edge types in the graph.
    /// * If the given edge type names does not exist in the graph.
    pub fn par_iter_directed_edge_node_names_from_edge_type_name(
        &self,
        edge_type_name: Option<&str>,
    ) -> Result<impl ParallelIterator<Item = (String, String)> + '_> {
        self.par_iter_directed_edge_node_ids_from_edge_type_name(edge_type_name)
            .map(|iter| {
                iter.map(move |(src, dst)| unsafe {
                    (
                        self.get_unchecked_node_name_from_node_id(src),
                        self.get_unchecked_node_name_from_node_id(dst),
                    )
                })
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
    ) -> Result<impl Iterator<Item = (NodeT, Option<&[NodeTypeT]>)> + '_> {
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
    ) -> Result<impl Iterator<Item = (NodeT, String, Option<&[NodeTypeT]>, Option<Vec<String>>)> + '_>
    {
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

    /// Returns parallel iterator over directed edge IDs with given edge type.
    ///
    /// # Arguments
    /// * `edge_type_id`: Option<EdgeTypeT> - Edge type ID to extract.
    ///
    /// # Raises
    /// * If there are no edge types in the graph.
    /// * If the given edge type ID does not exist in the graph.
    pub fn par_iter_directed_edge_ids_from_edge_type_id(
        &self,
        edge_type_id: Option<EdgeTypeT>,
    ) -> Result<impl ParallelIterator<Item = EdgeT> + '_> {
        self.validate_edge_type_id(edge_type_id)
            .map(|edge_type_id| {
                self.par_iter_directed_edge_type_ids()
                    .unwrap()
                    .enumerate()
                    .filter_map(move |(edge_id, this_edge_type_id)| {
                        if this_edge_type_id == edge_type_id {
                            Some(edge_id as EdgeT)
                        } else {
                            None
                        }
                    })
            })
    }

    /// Returns parallel iterator over directed edge IDs with given edge type name.
    ///
    /// # Arguments
    /// * `edge_type_name`: Option<EdgeTypeT> - Edge type names to extract.
    ///
    /// # Raises
    /// * If there are no edge types in the graph.
    /// * If the given edge type names does not exist in the graph.
    pub fn par_iter_directed_edge_ids_from_edge_type_name(
        &self,
        edge_type_name: Option<&str>,
    ) -> Result<impl ParallelIterator<Item = EdgeT> + '_> {
        self.par_iter_directed_edge_ids_from_edge_type_id(
            self.get_edge_type_id_from_edge_type_name(edge_type_name)?,
        )
    }

    /// Returns parallel iterator over directed edge node names with given node name prefixes
    ///
    /// # Arguments
    /// * `src_node_name_prefixes`: Option<Vec<&str>> - Prefixes of the source node names.
    /// * `dst_node_name_prefixes`: Option<Vec<&str>> - Prefixes of the source node names.
    ///
    pub fn par_iter_directed_edge_node_names_from_node_curie_prefixes<'a>(
        &'a self,
        src_node_name_prefixes: Option<Vec<&'a str>>,
        dst_node_name_prefixes: Option<Vec<&'a str>>,
    ) -> impl ParallelIterator<Item = (String, String)> + 'a {
        self.par_iter_directed_edges()
            .filter_map(move |(_, _, src_node_name, _, dst_node_name)| {
                if src_node_name_prefixes
                    .as_ref()
                    .map_or(true, |src_node_name_prefixes| {
                        src_node_name_prefixes.iter().any(|src_node_name_prefix| {
                            src_node_name.starts_with(src_node_name_prefix)
                        })
                    })
                    && dst_node_name_prefixes
                        .as_ref()
                        .map_or(true, |dst_node_name_prefixes| {
                            dst_node_name_prefixes.iter().any(|dst_node_name_prefix| {
                                dst_node_name.starts_with(dst_node_name_prefix)
                            })
                        })
                {
                    Some((src_node_name, dst_node_name))
                } else {
                    None
                }
            })
    }

    /// Returns parallel iterator over directed edge node IDs with given node name prefixes
    ///
    /// # Arguments
    /// * `src_node_name_prefixes`: Option<Vec<&str>> - Prefixes of the source node names.
    /// * `dst_node_name_prefixes`: Option<Vec<&str>> - Prefixes of the source node names.
    ///
    pub fn par_iter_directed_edge_node_ids_from_node_curie_prefixes<'a>(
        &'a self,
        src_node_name_prefixes: Option<Vec<&'a str>>,
        dst_node_name_prefixes: Option<Vec<&'a str>>,
    ) -> impl ParallelIterator<Item = (NodeT, NodeT)> + 'a {
        self.par_iter_directed_edges().filter_map(
            move |(_, src, src_node_name, dst, dst_node_name)| {
                if src_node_name_prefixes
                    .as_ref()
                    .map_or(true, |src_node_name_prefixes| {
                        src_node_name_prefixes.iter().any(|src_node_name_prefix| {
                            src_node_name.starts_with(src_node_name_prefix)
                        })
                    })
                    && dst_node_name_prefixes
                        .as_ref()
                        .map_or(true, |dst_node_name_prefixes| {
                            dst_node_name_prefixes.iter().any(|dst_node_name_prefix| {
                                dst_node_name.starts_with(dst_node_name_prefix)
                            })
                        })
                {
                    Some((src, dst))
                } else {
                    None
                }
            },
        )
    }

    /// Returns parallel iterator over directed edge IDs with given node name prefixes
    ///
    /// # Arguments
    /// * `src_node_name_prefixes`: Option<Vec<&str>> - Prefixes of the source node names.
    /// * `dst_node_name_prefixes`: Option<Vec<&str>> - Prefixes of the source node names.
    ///
    pub fn par_iter_directed_edge_ids_from_node_curie_prefixes<'a>(
        &'a self,
        src_node_name_prefixes: Option<Vec<&'a str>>,
        dst_node_name_prefixes: Option<Vec<&'a str>>,
    ) -> impl ParallelIterator<Item = EdgeT> + 'a {
        self.par_iter_directed_edges().filter_map(
            move |(edge_id, _, src_node_name, _, dst_node_name)| {
                if src_node_name_prefixes
                    .as_ref()
                    .map_or(true, |src_node_name_prefixes| {
                        src_node_name_prefixes.iter().any(|src_node_name_prefix| {
                            src_node_name.starts_with(src_node_name_prefix)
                        })
                    })
                    && dst_node_name_prefixes
                        .as_ref()
                        .map_or(true, |dst_node_name_prefixes| {
                            dst_node_name_prefixes.iter().any(|dst_node_name_prefix| {
                                dst_node_name.starts_with(dst_node_name_prefix)
                            })
                        })
                {
                    Some(edge_id)
                } else {
                    None
                }
            },
        )
    }

    /// Returns parallel iterator over node IDs with given curie prefixes
    ///
    /// # Arguments
    /// * `curie_prefixes`: &[&str] - Prefix of the node names.
    pub fn par_iter_node_ids_from_node_curie_prefixes<'a>(
        &'a self,
        curie_prefixes: &'a [&'a str],
    ) -> impl ParallelIterator<Item = NodeT> + 'a {
        self.par_iter_node_ids()
            .zip(self.par_iter_node_names())
            .filter_map(move |(node_id, node_name)| {
                if curie_prefixes
                    .iter()
                    .any(|curie_prefix| node_name.starts_with(curie_prefix))
                {
                    Some(node_id)
                } else {
                    None
                }
            })
    }

    /// Returns iterator over node IDs with given curie prefixes
    ///
    /// # Arguments
    /// * `curie_prefixes`: &[&str] - Prefix of the node names.
    pub fn iter_node_ids_from_node_curie_prefixes<'a>(
        &'a self,
        curie_prefixes: &'a [&'a str],
    ) -> impl Iterator<Item = NodeT> + 'a {
        self.iter_node_ids()
            .zip(self.iter_node_names())
            .filter_map(move |(node_id, node_name)| {
                if curie_prefixes
                    .iter()
                    .any(|curie_prefix| node_name.starts_with(curie_prefix))
                {
                    Some(node_id)
                } else {
                    None
                }
            })
    }

    /// Returns parallel iterator over node names with given curie prefixes
    ///
    /// # Arguments
    /// * `curie_prefixes`: &[&str] - Prefixes of node names.
    pub fn par_iter_node_names_from_node_curie_prefixes<'a>(
        &'a self,
        curie_prefixes: &'a [&'a str],
    ) -> impl ParallelIterator<Item = String> + 'a {
        self.par_iter_node_names().filter_map(move |node_name| {
            if curie_prefixes
                .iter()
                .any(|curie_prefix| node_name.starts_with(curie_prefix))
            {
                Some(node_name)
            } else {
                None
            }
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

    /// Return parallel iterator on the unweighted non-zero node degrees of the given subgraph.
    ///
    /// # Implementation details
    /// The non-zero aspect of this iterator refers to the degrees of the subgraph, which
    /// will generally be lower than the original graph and may include a considerable
    /// amount of disconnected nodes, which we want to ignore in this use case.
    /// Do note that zero degrees will be returned when the node have degree zero also
    /// in the original graph.
    ///
    /// # Arguments
    /// * `subgraph`: &Graph - The subgraph whose node degrees are to be retrieved.
    ///
    pub fn par_iter_non_zero_subgraph_node_degrees<'a>(
        &'a self,
        subgraph: &'a Graph,
    ) -> Result<impl ParallelIterator<Item = NodeT> + 'a> {
        self.must_share_node_vocabulary(subgraph)?;
        Ok(subgraph
            .par_iter_node_ids()
            .filter_map(move |node_id| unsafe {
                let degree = self.get_unchecked_node_degree_from_node_id(node_id);
                if degree == 0 {
                    Some(degree)
                } else {
                    if subgraph.get_unchecked_node_degree_from_node_id(node_id) > 0 {
                        Some(degree)
                    } else {
                        None
                    }
                }
            }))
    }
}
