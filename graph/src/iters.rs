use super::*;
use rayon::prelude::*;

/// # Iterators
/// The naming convention for the iterators is:
/// If the method has the `par_` prefix then it should return a parallel iterator.
/// By default all the methods retruns both the ids and the name of the item and
/// if the method has the suffix `_ids` then it will returns **only** the ids.
impl Graph {
    /// Returns range of the edge ids of edges starting from the given source node.
    ///
    /// # Arguments
    ///
    /// * `src` - Source node of the edge.
    ///
    pub(crate) fn iter_unchecked_edge_ids_from_source_node_id(
        &self,
        src: NodeT,
    ) -> std::ops::Range<usize> {
        let (min_edge_id, max_edge_id) = self.get_minmax_edge_ids_from_source_node_id(src);
        min_edge_id as usize..max_edge_id as usize
    }

    /// Return iterator on the node degrees of the graph.
    pub fn iter_node_degrees(&self) -> impl Iterator<Item = NodeT> + '_ {
        (0..self.get_nodes_number()).map(move |node| self.get_node_degree_from_node_id(node).unwrap())
    }

    /// Return iterator on the node degrees of the graph.
    pub fn par_iter_node_degrees(&self) -> impl ParallelIterator<Item = NodeT> + '_ {
        (0..self.get_nodes_number())
            .into_par_iter()
            .map(move |node| self.get_node_degree_from_node_id(node).unwrap())
    }

    /// Return iterator on the singleton nodes of the graph.
    pub fn iter_singleton_node_ids(&self) -> Box<dyn Iterator<Item = NodeT> + '_> {
        match self.not_singleton_nodes.as_ref() {
            Some(nsns) => Box::new(nsns.iter_zeros().map(|node_id| node_id as NodeT)),
            _ => Box::new(::std::iter::empty()),
        }
    }

    /// Return iterator on the singleton with selfloops nodes of the graph.
    pub fn iter_singleton_with_selfloops_node_ids(&self) -> Box<dyn Iterator<Item = NodeT> + '_> {
        match self.singleton_nodes_with_self_loops.as_ref() {
            Some(nsns) => Box::new(nsns.iter()),
            _ => Box::new(::std::iter::empty()),
        }
    }

    /// Return iterator over NodeT of destinations of the given node src.
    ///
    /// # Arguments
    /// * `src`: NodeT - The node whose neighbours are to be retrieved.
    ///
    pub(crate) fn iter_node_neighbours_ids(
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
    pub(crate) fn iter_node_neighbours(&self, src: NodeT) -> impl Iterator<Item = String> + '_ {
        self.iter_node_neighbours_ids(src)
            .map(move |dst| self.get_unchecked_node_name_from_node_id(dst))
    }

    /// Return iterator on the (non unique) source nodes of the graph.
    ///
    /// # Arguments
    /// * `directed`: bool, whether to filter out the undirected edges.
    pub fn iter_sources_ids(&self, directed: bool) -> impl Iterator<Item = NodeT> + '_ {
        self.iter_edge_ids(directed).map(move |(_, src, _)| src)
    }

    /// Return parallel iterator on the (non unique) source nodes of the graph.
    ///
    /// # Arguments
    /// * `directed`: bool, whether to filter out the undirected edges.
    pub fn par_iter_sources_ids(&self, directed: bool) -> impl ParallelIterator<Item = NodeT> + '_ {
        self.par_iter_edge_ids(directed).map(move |(_, src, _)| src)
    }

    /// Return iterator on the (non unique) destination nodes of the graph.
    ///
    /// # Arguments
    /// * `directed`: bool, whether to filter out the undirected edges.
    pub fn iter_destinations_ids(&self, directed: bool) -> impl Iterator<Item = NodeT> + '_ {
        self.iter_edge_ids(directed).map(move |(_, _, dst)| dst)
    }

    /// Return parallel iterator on the (non unique) destination nodes of the graph.
    ///
    /// # Arguments
    /// * `directed`: bool, whether to filter out the undirected edges.
    pub fn par_iter_destinations_ids(
        &self,
        directed: bool,
    ) -> impl ParallelIterator<Item = NodeT> + '_ {
        self.par_iter_edge_ids(directed).map(move |(_, _, dst)| dst)
    }

    /// Return iterator on the node of the graph.
    pub fn iter_node_ids(&self) -> impl Iterator<Item = (NodeT, Option<Vec<NodeTypeT>>)> + '_ {
        (0..self.get_nodes_number())
            .map(move |node_id| (node_id, self.get_unchecked_node_type_id_from_node_id(node_id)))
    }

    /// Return iterator on the node of the graph as Strings.
    pub fn iter_nodes(
        &self,
    ) -> impl Iterator<Item = (NodeT, String, Option<Vec<NodeTypeT>>, Option<Vec<String>>)> + '_
    {
        self.iter_node_ids().map(move |(node_id, node_types)| {
            (
                node_id,
                self.nodes.unchecked_translate(node_id),
                node_types,
                self.get_node_type_name_from_node_id(node_id).unwrap_or(None),
            )
        })
    }

    /// Return iterator on the edges of the graph.
    ///
    /// # Arguments
    /// * `directed`: bool, whether to filter out the undirected edges.
    pub fn iter_edge_ids(
        &self,
        directed: bool,
    ) -> Box<dyn Iterator<Item = (EdgeT, NodeT, NodeT)> + '_> {
        if self.sources.is_some() && self.destinations.is_some() {
            return Box::new(
                (0..self.get_directed_edges_number()).filter_map(move |edge_id| {
                    let (src, dst) = self.get_unchecked_node_ids_from_edge_id(edge_id);
                    if !directed && src > dst {
                        return None;
                    }
                    Some((edge_id, src, dst))
                }),
            );
        }
        Box::new(
            self.edges
                .iter()
                .enumerate()
                .filter_map(move |(edge_id, edge)| {
                    let (src, dst) = self.decode_edge(edge);
                    if !directed && src > dst {
                        return None;
                    }
                    Some((edge_id as EdgeT, src, dst))
                }),
        )
    }
    /// Return iterator on the edges of the graph with the string name.
    ///
    /// # Arguments
    /// * `directed`: bool, whether to filter out the undirected edges.
    pub fn iter_edges(
        &self,
        directed: bool,
    ) -> impl Iterator<Item = (EdgeT, NodeT, String, NodeT, String)> + '_ {
        self.iter_edge_ids(directed)
            .map(move |(edge_id, src, dst)| {
                (
                    edge_id,
                    src,
                    self.get_unchecked_node_name_from_node_id(src),
                    dst,
                    self.get_unchecked_node_name_from_node_id(dst),
                )
            })
    }

    /// Return iterator on the edges of the graph.
    ///
    /// # Arguments
    /// * `directed`: bool, whether to filter out the undirected edges.
    pub fn par_iter_edge_ids(
        &self,
        directed: bool,
    ) -> impl ParallelIterator<Item = (EdgeT, NodeT, NodeT)> + '_ {
        self.edges
            .par_enumerate()
            .filter_map(move |(edge_id, edge)| {
                let (src, dst) = self.decode_edge(edge);
                if !directed && src > dst {
                    return None;
                }
                Some((edge_id as EdgeT, src, dst))
            })
    }

    /// Return iterator on the edges of the graph with the string name.
    ///
    /// # Arguments
    /// * `directed`: bool, whether to filter out the undirected edges.
    pub fn par_iter_edges(
        &self,
        directed: bool,
    ) -> impl ParallelIterator<Item = (EdgeT, NodeT, String, NodeT, String)> + '_ {
        self.par_iter_edge_ids(directed)
            .map(move |(edge_id, src, dst)| {
                (
                    edge_id,
                    src,
                    self.get_unchecked_node_name_from_node_id(src),
                    dst,
                    self.get_unchecked_node_name_from_node_id(dst),
                )
            })
    }

    /// Return iterator on the edges of the graph.
    ///
    /// # Arguments
    /// * `directed`: bool, whether to filter out the undirected edges.
    pub fn iter_edges_with_type_ids(
        &self,
        directed: bool,
    ) -> impl Iterator<Item = (EdgeT, NodeT, NodeT, Option<EdgeTypeT>)> + '_ {
        self.iter_edge_ids(directed)
            .map(move |(edge_id, src, dst)| {
                (
                    edge_id,
                    src,
                    dst,
                    self.get_unchecked_edge_type_from_edge_id(edge_id),
                )
            })
    }

    /// Return iterator on the edges of the graph with the string name.
    ///
    /// # Arguments
    /// * `directed`: bool, whether to filter out the undirected edges.
    pub fn iter_edges_with_type(
        &self,
        directed: bool,
    ) -> impl Iterator<
        Item = (
            EdgeT,
            NodeT,
            String,
            NodeT,
            String,
            Option<EdgeTypeT>,
            Option<String>,
        ),
    > + '_ {
        self.iter_edges(directed)
            .map(move |(edge_id, src, src_name, dst, dst_name)| {
                let edge_type_id = self.get_unchecked_edge_type_from_edge_id(edge_id);
                (
                    edge_id,
                    src,
                    src_name,
                    dst,
                    dst_name,
                    edge_type_id,
                    self.get_unchecked_edge_type_name_from_edge_type_id(edge_type_id),
                )
            })
    }

    /// Return iterator on the edges of the graph with the ids and string name.
    /// The result is (edge_id, src, src_name, dst, dst_name, edge_type, edge_type_name)
    ///
    /// # Arguments
    /// * `directed`: bool, whether to filter out the undirected edges.
    pub fn par_iter_edge_with_type(
        &self,
        directed: bool,
    ) -> impl ParallelIterator<
        Item = (
            EdgeT,
            NodeT,
            String,
            NodeT,
            String,
            Option<EdgeTypeT>,
            Option<String>,
        ),
    > + '_ {
        self.par_iter_edges(directed)
            .map(move |(edge_id, src, src_name, dst, dst_name)| {
                let edge_type_id = self.get_unchecked_edge_type_from_edge_id(edge_id);
                (
                    edge_id,
                    src,
                    src_name,
                    dst,
                    dst_name,
                    edge_type_id,
                    self.get_unchecked_edge_type_name_from_edge_type_id(edge_type_id),
                )
            })
    }

    /// Return iterator on the edges of the graph with the string name.
    ///
    /// # Arguments
    /// * `directed`: bool, whether to filter out the undirected edges.
    pub fn par_iter_edge_with_type_ids(
        &self,
        directed: bool,
    ) -> impl ParallelIterator<Item = (EdgeT, NodeT, NodeT, Option<EdgeTypeT>)> + '_ {
        self.par_iter_edge_ids(directed)
            .map(move |(edge_id, src, dst)| {
                (
                    edge_id,
                    src,
                    dst,
                    self.get_unchecked_edge_type_from_edge_id(edge_id),
                )
            })
    }

    /// Return iterator on the edges of the graph with the string name.
    ///
    /// # Arguments
    /// * `directed`: bool, whether to filter out the undirected edges.
    pub fn par_iter_edge_with_type_and_weight(
        &self,
        directed: bool,
    ) -> impl ParallelIterator<
        Item = (
            EdgeT,
            NodeT,
            String,
            NodeT,
            String,
            Option<EdgeTypeT>,
            Option<String>,
            Option<WeightT>,
        ),
    > + '_ {
        self.par_iter_edge_with_type(directed).map(
            move |(edge_id, src, src_name, dst, dst_name, edge_type, edge_type_name)| {
                (
                    edge_id,
                    src,
                    src_name,
                    dst,
                    dst_name,
                    edge_type,
                    edge_type_name,
                    self.get_unchecked_weight_from_edge_id(edge_id),
                )
            },
        )
    }

    /// Return iterator on the edges of the graph with the string name.
    ///
    /// # Arguments
    /// * `directed`: bool, whether to filter out the undirected edges.
    pub fn iter_edge_with_type_and_weight(
        &self,
        directed: bool,
    ) -> impl Iterator<
        Item = (
            EdgeT,
            NodeT,
            String,
            NodeT,
            String,
            Option<EdgeTypeT>,
            Option<String>,
            Option<WeightT>,
        ),
    > + '_ {
        self.iter_edges_with_type(directed).map(
            move |(edge_id, src, src_name, dst, dst_name, edge_type, edge_type_name)| {
                (
                    edge_id,
                    src,
                    src_name,
                    dst,
                    dst_name,
                    edge_type,
                    edge_type_name,
                    self.get_unchecked_weight_from_edge_id(edge_id),
                )
            },
        )
    }

    /// Return iterator on the edges of the graph with the string name.
    ///
    /// # Arguments
    /// * `directed`: bool, whether to filter out the undirected edges.
    pub fn par_iter_edge_with_type_and_weight_ids(
        &self,
        directed: bool,
    ) -> impl ParallelIterator<Item = (EdgeT, NodeT, NodeT, Option<EdgeTypeT>, Option<WeightT>)> + '_
    {
        self.par_iter_edge_with_type_ids(directed)
            .map(move |(edge_id, src, dst, edge_type)| {
                (
                    edge_id,
                    src,
                    dst,
                    edge_type,
                    self.get_unchecked_weight_from_edge_id(edge_id),
                )
            })
    }

    /// Return iterator on the edges of the graph with the string name.
    ///
    /// # Arguments
    /// * `directed`: bool, whether to filter out the undirected edges.
    pub fn iter_edge_with_type_and_weight_ids(
        &self,
        directed: bool,
    ) -> impl Iterator<Item = (EdgeT, NodeT, NodeT, Option<EdgeTypeT>, Option<WeightT>)> + '_ {
        self.iter_edges_with_type_ids(directed)
            .map(move |(edge_id, src, dst, edge_type)| {
                (
                    edge_id,
                    src,
                    dst,
                    edge_type,
                    self.get_unchecked_weight_from_edge_id(edge_id),
                )
            })
    }

    /// Return iterator on the edges of the graph.
    ///
    /// # Arguments
    /// * `directed`: bool, whether to filter out the undirected edges.
    pub fn iter_unique_edges(
        &self,
        directed: bool,
    ) -> Box<dyn Iterator<Item = (NodeT, NodeT)> + '_> {
        if self.sources.is_some() && self.destinations.is_some() {
            return Box::new(
                (0..self.get_directed_edges_number()).filter_map(move |edge_id| {
                    let (src, dst) = self.get_unchecked_node_ids_from_edge_id(edge_id);
                    if edge_id > 0 {
                        let (last_src, last_dst) =
                            self.get_unchecked_node_ids_from_edge_id(edge_id - 1);
                        if last_src == src && last_dst == dst {
                            return None;
                        }
                    }
                    if !directed && src > dst {
                        return None;
                    }
                    Some((src, dst))
                }),
            );
        }
        Box::new(self.edges.iter_uniques().filter_map(move |edge| {
            let (src, dst) = self.decode_edge(edge);
            if !directed && src > dst {
                return None;
            }
            Some((src, dst))
        }))
    }

    /// Returns option of range of multigraph minimum and maximum edge ids with same source and destination nodes and different edge type.
    ///
    /// # Arguments
    ///
    /// * `src` - Source node id of the edge.
    /// * `dst` - Destination node id of the edge.
    ///
    pub fn iter_edge_ids_from_node_ids(
        &self,
        src: NodeT,
        dst: NodeT,
    ) -> Result<impl Iterator<Item = EdgeT>, String> {
        let (min_edge_id, max_edge_id) = self.get_minmax_edge_ids_from_node_ids(src, dst)?;
        Ok(min_edge_id..max_edge_id)
    }

    /// Return iterator on the unique sources of the graph.
    pub fn iter_unique_sources(&self) -> Box<dyn Iterator<Item = NodeT> + '_> {
        if let Some(x) = &self.unique_sources {
            return Box::new(x.iter().map(|source| source as NodeT));
        }
        Box::new(0..self.get_nodes_number())
    }
}
