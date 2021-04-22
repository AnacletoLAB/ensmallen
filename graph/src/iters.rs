use super::*;
use rayon::prelude::*;

/// # Iterators
/// The naming convention for the iterators is:
/// If the method has the `par_` prefix then it should return a parallel iterator.
/// By default all the methods retruns both the ids and the name of the item and
/// if the method has the suffix `_ids` then it will returns **only** the ids.
/// Therefore, the naming convetions are:
/// * `iter_(.+)`
/// * `iter_unchecked_(.+)`
/// * `par_iter_(.+)`
/// * `par_iter_unchecked_(.+)`
impl Graph {
    /// Return iterator on the node IDs of the graph.
    pub fn iter_node_ids(&self) -> impl Iterator<Item = NodeT> + '_ {
        0..self.get_nodes_number()
    }

    /// Return iterator on the node names of the graph.
    pub fn iter_node_names(&self) -> impl Iterator<Item = String> + '_ {
        self.iter_node_ids()
            .map(move |node_id| self.get_unchecked_node_name_from_node_id(node_id))
    }

    /// Return iterator on the unique node type IDs of the graph.
    pub fn iter_unique_node_type_ids(
        &self,
    ) -> Result<impl Iterator<Item = NodeTypeT> + '_, String> {
        Ok(0..self.get_node_types_number()?)
    }

    /// Return iterator on the unique node type IDs counts of the graph.
    pub fn iter_node_type_counts(&self) -> Result<impl Iterator<Item = NodeT> + '_, String> {
        self.must_have_node_types()
            .map(|node_types| node_types.counts.iter().cloned())
    }

    /// Return iterator on the unique node type IDs and their counts of the graph.
    pub fn iter_unique_node_type_ids_and_counts(
        &self,
    ) -> Result<impl Iterator<Item = (NodeTypeT, NodeT)> + '_, String> {
        Ok(self
            .iter_unique_node_type_ids()?
            .zip(self.iter_node_type_counts()?))
    }

    /// Return iterator on the unique node type names of the graph.
    pub fn iter_unique_node_type_names(&self) -> Result<impl Iterator<Item = String> + '_, String> {
        self.must_have_node_types()
            .map(|node_types| node_types.vocabulary.reverse_map.iter().cloned())
    }

    /// Return iterator on the unique node type names and their counts of the graph.
    pub fn iter_unique_node_type_names_and_counts(
        &self,
    ) -> Result<impl Iterator<Item = (String, NodeT)> + '_, String> {
        Ok(self
            .iter_unique_node_type_names()?
            .zip(self.iter_node_type_counts()?))
    }

    /// Return iterator on the edge type IDs of the graph.
    pub fn iter_unique_edge_type_ids(
        &self,
    ) -> Result<impl Iterator<Item = EdgeTypeT> + '_, String> {
        Ok(0..self.get_edge_types_number()?)
    }

    /// Return iterator on the unique edge type IDs counts of the graph.
    pub fn iter_edge_type_counts(&self) -> Result<impl Iterator<Item = EdgeT> + '_, String> {
        self.must_have_edge_types()
            .map(|edge_types| edge_types.counts.iter().cloned())
    }

    /// Return iterator on the unique edge type IDs and their counts of the graph.
    pub fn iter_unique_edge_type_ids_and_counts(
        &self,
    ) -> Result<impl Iterator<Item = (EdgeTypeT, EdgeT)> + '_, String> {
        Ok(self
            .iter_unique_edge_type_ids()?
            .zip(self.iter_edge_type_counts()?))
    }

    /// Return iterator on the unique edge type names and their counts of the graph.
    pub fn iter_unique_edge_type_names_and_counts(
        &self,
    ) -> Result<impl Iterator<Item = (String, EdgeT)> + '_, String> {
        Ok(self
            .iter_unique_edge_type_names()?
            .zip(self.iter_edge_type_counts()?))
    }

    /// Return iterator on the unique edge type names of the graph.
    pub fn iter_unique_edge_type_names(&self) -> Result<impl Iterator<Item = String> + '_, String> {
        self.must_have_edge_types()
            .map(|edge_types| edge_types.vocabulary.reverse_map.iter().cloned())
    }

    /// Return iterator on the node of the graph.
    pub fn par_iter_node_ids(&self) -> impl ParallelIterator<Item = NodeT> + '_ {
        (0..self.get_nodes_number()).into_par_iter()
    }

    /// Return iterator on the node degrees of the graph.
    pub fn iter_node_degrees(&self) -> impl Iterator<Item = NodeT> + '_ {
        self.iter_node_ids()
            .map(move |node| self.get_unchecked_node_degree_from_node_id(node))
    }

    /// Return iterator on the node degrees of the graph.
    pub fn par_iter_node_degrees(&self) -> impl ParallelIterator<Item = NodeT> + '_ {
        self.par_iter_node_ids()
            .map(move |node| self.get_unchecked_node_degree_from_node_id(node))
    }

    /// Return iterator on the non-singleton nodes of the graph.
    ///
    /// Note that this includes also the singleton with self-loops and
    /// the trap nodes within this iterator. Only true singleton nodes,
    /// that is, nodes without any edge (both inbound and outbound) are
    /// included.
    ///
    /// Since the following requires to be boxed, we cannot create the
    /// parallel version of this iterator.
    ///
    pub fn iter_non_singleton_node_ids(&self) -> Box<dyn Iterator<Item = NodeT> + '_> {
        match self.not_singleton_nodes.as_ref() {
            Some(nsns) => Box::new(nsns.iter_ones().map(|node_id| node_id as NodeT)),
            _ => Box::new(self.iter_node_ids()),
        }
    }

    /// Return iterator on the singleton nodes IDs of the graph.
    pub fn iter_singleton_node_ids(&self) -> Box<dyn Iterator<Item = NodeT> + '_> {
        match self.not_singleton_nodes.as_ref() {
            Some(nsns) => Box::new(nsns.iter_zeros().map(|node_id| node_id as NodeT)),
            _ => Box::new(::std::iter::empty()),
        }
    }

    /// Return iterator on the singleton nodes names of the graph.
    pub fn iter_singleton_node_names(&self) -> impl Iterator<Item = String> + '_ {
        self.iter_singleton_node_ids()
            .map(move |node_id| self.get_unchecked_node_name_from_node_id(node_id))
    }

    /// Return iterator on the singleton with selfloops node IDs of the graph.
    pub fn iter_singleton_with_selfloops_node_ids(&self) -> Box<dyn Iterator<Item = NodeT> + '_> {
        match self.singleton_nodes_with_selfloops.as_ref() {
            Some(nsns) => Box::new(nsns.iter()),
            _ => Box::new(::std::iter::empty()),
        }
    }

    /// Return iterator on the singleton with selfloops node names of the graph.
    pub fn iter_singleton_with_selfloops_node_names(&self) -> impl Iterator<Item = String> + '_ {
        self.iter_singleton_with_selfloops_node_ids()
            .map(move |node_id| self.get_unchecked_node_name_from_node_id(node_id))
    }

    /// Return iterator on the singleton node type IDs of the graph.
    ///
    /// # Raises
    /// * If there are no node types in the graph.
    pub fn iter_singleton_node_type_ids(
        &self,
    ) -> Result<impl Iterator<Item = NodeTypeT> + '_, String> {
        self.iter_unique_node_type_ids_and_counts()
            .map(|iter_unique_node_type_ids_and_counts| {
                iter_unique_node_type_ids_and_counts.filter_map(|(node_type_id, count)| {
                    if count == 1 {
                        Some(node_type_id)
                    } else {
                        None
                    }
                })
            })
    }

    /// Return iterator on the singleton edge type IDs of the graph.
    ///
    /// # Raises
    /// * If there are no edge types in the graph.
    pub fn iter_singleton_edge_type_ids(
        &self,
    ) -> Result<impl Iterator<Item = EdgeTypeT> + '_, String> {
        self.iter_unique_edge_type_ids_and_counts()
            .map(|iter_unique_edge_type_ids_and_counts| {
                iter_unique_edge_type_ids_and_counts.filter_map(|(edge_type_id, count)| {
                    if count == 1 {
                        Some(edge_type_id)
                    } else {
                        None
                    }
                })
            })
    }

    /// Return iterator on the singleton node type names of the graph.
    ///
    /// # Raises
    /// * If there are no node types in the graph.
    pub fn iter_singleton_node_type_names(
        &self,
    ) -> Result<impl Iterator<Item = String> + '_, String> {
        self.iter_unique_node_type_names_and_counts().map(
            |iter_unique_node_type_names_and_counts| {
                iter_unique_node_type_names_and_counts.filter_map(|(node_type_id, count)| {
                    if count == 1 {
                        Some(node_type_id)
                    } else {
                        None
                    }
                })
            },
        )
    }

    /// Return iterator on the singleton edge type names of the graph.
    ///
    /// # Raises
    /// * If there are no edge types in the graph.
    pub fn iter_singleton_edge_type_names(
        &self,
    ) -> Result<impl Iterator<Item = String> + '_, String> {
        self.iter_unique_edge_type_names_and_counts().map(
            |iter_unique_edge_type_names_and_counts| {
                iter_unique_edge_type_names_and_counts.filter_map(|(edge_type_id, count)| {
                    if count == 1 {
                        Some(edge_type_id)
                    } else {
                        None
                    }
                })
            },
        )
    }

    /// Return iterator on the (non unique) source nodes of the graph.
    ///
    /// # Arguments
    /// * `directed`: bool - Whether to filter out the undirected edges.
    pub fn iter_source_node_ids(&self, directed: bool) -> impl Iterator<Item = NodeT> + '_ {
        self.iter_edge_ids(directed).map(move |(_, src, _)| src)
    }

    /// Return iterator on the edges' weights.
    ///
    /// # Example
    /// To get an iterator over the edges weights you can use:
    /// ```rust
    /// # let graph_with_weights = graph::test_utilities::load_ppi(false, false, true, true, false, false);
    /// # let graph_without_weights = graph::test_utilities::load_ppi(false, false, false, true, false, false);
    /// assert!(graph_with_weights.iter_edge_weights().is_ok());
    /// assert!(graph_without_weights.iter_edge_weights().is_err());
    /// println!("The graph weights are {:?}.", graph_with_weights.iter_edge_weights().unwrap().collect::<Vec<_>>());
    /// ```
    pub fn iter_edge_weights(&self) -> Result<impl Iterator<Item = WeightT> + '_, String> {
        self.must_have_edge_weights()?;
        Ok(self.weights.as_ref().map(|ws| ws.iter().cloned()).unwrap())
    }

    /// Return parallel iterator on the edges' weights.
    ///
    /// # Example
    /// To get an iterator over the edges weights you can use:
    /// ```rust
    /// # use rayon::iter::ParallelIterator;
    /// # let graph_with_weights = graph::test_utilities::load_ppi(false, false, true, true, false, false);
    /// # let graph_without_weights = graph::test_utilities::load_ppi(false, false, false, true, false, false);
    /// assert!(graph_with_weights.iter_edge_weights().is_ok());
    /// assert!(graph_without_weights.iter_edge_weights().is_err());
    /// println!("The graph weights are {:?}.", graph_with_weights.par_iter_edge_weights().unwrap().collect::<Vec<_>>());
    /// ```
    pub fn par_iter_edge_weights(
        &self,
    ) -> Result<impl ParallelIterator<Item = WeightT> + '_, String> {
        self.must_have_edge_weights()?;
        Ok(self
            .weights
            .as_ref()
            .map(|ws| ws.par_iter().cloned())
            .unwrap())
    }

    /// Return parallel iterator on the (non unique) source nodes of the graph.
    ///
    /// # Arguments
    /// * `directed`: bool - Whether to filter out the undirected edges.
    pub fn par_iter_source_node_ids(
        &self,
        directed: bool,
    ) -> impl ParallelIterator<Item = NodeT> + '_ {
        self.par_iter_edge_ids(directed).map(move |(_, src, _)| src)
    }

    /// Return iterator on the (non unique) destination nodes of the graph.
    ///
    /// # Arguments
    /// * `directed`: bool - Whether to filter out the undirected edges.
    pub fn iter_destination_node_ids(&self, directed: bool) -> impl Iterator<Item = NodeT> + '_ {
        self.iter_edge_ids(directed).map(move |(_, _, dst)| dst)
    }

    /// Return parallel iterator on the (non unique) destination nodes of the graph.
    ///
    /// # Arguments
    /// * `directed`: bool - Whether to filter out the undirected edges.
    pub fn par_iter_destination_node_ids(
        &self,
        directed: bool,
    ) -> impl ParallelIterator<Item = NodeT> + '_ {
        self.par_iter_edge_ids(directed).map(move |(_, _, dst)| dst)
    }

    /// Return iterator on the node IDs and ther node type IDs.
    pub fn iter_node_ids_and_node_type_ids(
        &self,
    ) -> impl Iterator<Item = (NodeT, Option<Vec<NodeTypeT>>)> + '_ {
        self.iter_node_ids().map(move |node_id| {
            (
                node_id,
                self.get_unchecked_node_type_id_from_node_id(node_id),
            )
        })
    }

    /// Return iterator on the node of the graph.
    pub fn par_iter_node_ids_and_node_type_ids(
        &self,
    ) -> impl ParallelIterator<Item = (NodeT, Option<Vec<NodeTypeT>>)> + '_ {
        self.par_iter_node_ids().map(move |node_id| {
            (
                node_id,
                self.get_unchecked_node_type_id_from_node_id(node_id),
            )
        })
    }

    /// Return iterator on the node of the graph as Strings.
    pub fn iter_node_names_and_node_type_names(
        &self,
    ) -> impl Iterator<Item = (NodeT, String, Option<Vec<NodeTypeT>>, Option<Vec<String>>)> + '_
    {
        self.iter_node_ids_and_node_type_ids()
            .map(move |(node_id, node_types)| {
                (
                    node_id,
                    self.get_unchecked_node_name_from_node_id(node_id),
                    node_types,
                    self.get_unchecked_node_type_names_from_node_id(node_id),
                )
            })
    }

    /// Return iterator on the edges of the graph.
    ///
    /// # Arguments
    /// * `directed`: bool - Whether to filter out the undirected edges.
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
    /// * `directed`: bool - Whether to filter out the undirected edges.
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
    /// * `directed`: bool - Whether to filter out the undirected edges.
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
    /// * `directed`: bool - Whether to filter out the undirected edges.
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
    /// * `directed`: bool - Whether to filter out the undirected edges.
    pub fn iter_edge_node_ids_and_edge_type_id(
        &self,
        directed: bool,
    ) -> impl Iterator<Item = (EdgeT, NodeT, NodeT, Option<EdgeTypeT>)> + '_ {
        self.iter_edge_ids(directed)
            .map(move |(edge_id, src, dst)| {
                (
                    edge_id,
                    src,
                    dst,
                    self.get_unchecked_edge_type_id_from_edge_id(edge_id),
                )
            })
    }

    /// Return iterator on the edges of the graph with the string name.
    ///
    /// # Arguments
    /// * `directed`: bool - Whether to filter out the undirected edges.
    pub fn iter_edge_node_names_and_edge_type_name(
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
                let edge_type_id = self.get_unchecked_edge_type_id_from_edge_id(edge_id);
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
    /// * `directed`: bool - Whether to filter out the undirected edges.
    pub fn par_iter_edge_node_names_and_edge_type_name(
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
                let edge_type_id = self.get_unchecked_edge_type_id_from_edge_id(edge_id);
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

    /// Return iterator on the edges of the graph.
    ///
    /// # Arguments
    /// * `directed`: bool - Whether to filter out the undirected edges.
    pub fn par_iter_edge_node_ids_and_edge_type_id(
        &self,
        directed: bool,
    ) -> impl ParallelIterator<Item = (EdgeT, NodeT, NodeT, Option<EdgeTypeT>)> + '_ {
        self.par_iter_edge_ids(directed)
            .map(move |(edge_id, src, dst)| {
                (
                    edge_id,
                    src,
                    dst,
                    self.get_unchecked_edge_type_id_from_edge_id(edge_id),
                )
            })
    }

    /// Return iterator on the edges of the graph with the string name.
    ///
    /// # Arguments
    /// * `directed`: bool - Whether to filter out the undirected edges.
    pub fn par_iter_edge_node_names_and_edge_type_name_and_edge_weight(
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
        self.par_iter_edge_node_names_and_edge_type_name(directed)
            .map(
                move |(edge_id, src, src_name, dst, dst_name, edge_type, edge_type_name)| {
                    (
                        edge_id,
                        src,
                        src_name,
                        dst,
                        dst_name,
                        edge_type,
                        edge_type_name,
                        self.get_unchecked_edge_weight_from_edge_id(edge_id),
                    )
                },
            )
    }

    /// Return iterator on the edges of the graph with the string name.
    ///
    /// # Arguments
    /// * `directed`: bool - Whether to filter out the undirected edges.
    pub fn iter_edge_node_names_and_edge_type_name_and_edge_weight(
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
        self.iter_edge_node_names_and_edge_type_name(directed).map(
            move |(edge_id, src, src_name, dst, dst_name, edge_type, edge_type_name)| {
                (
                    edge_id,
                    src,
                    src_name,
                    dst,
                    dst_name,
                    edge_type,
                    edge_type_name,
                    self.get_unchecked_edge_weight_from_edge_id(edge_id),
                )
            },
        )
    }

    /// Return iterator on the edges of the graph with the string name.
    ///
    /// # Arguments
    /// * `directed`: bool - Whether to filter out the undirected edges.
    pub fn par_iter_edge_node_ids_and_edge_type_id_and_edge_weight(
        &self,
        directed: bool,
    ) -> impl ParallelIterator<Item = (EdgeT, NodeT, NodeT, Option<EdgeTypeT>, Option<WeightT>)> + '_
    {
        self.par_iter_edge_node_ids_and_edge_type_id(directed).map(
            move |(edge_id, src, dst, edge_type)| {
                (
                    edge_id,
                    src,
                    dst,
                    edge_type,
                    self.get_unchecked_edge_weight_from_edge_id(edge_id),
                )
            },
        )
    }

    /// Return iterator on the edges of the graph with the string name.
    ///
    /// # Arguments
    /// * `directed`: bool - Whether to filter out the undirected edges.
    pub fn iter_edge_node_ids_and_edge_type_id_and_edge_weight(
        &self,
        directed: bool,
    ) -> impl Iterator<Item = (EdgeT, NodeT, NodeT, Option<EdgeTypeT>, Option<WeightT>)> + '_ {
        self.iter_edge_node_ids_and_edge_type_id(directed).map(
            move |(edge_id, src, dst, edge_type)| {
                (
                    edge_id,
                    src,
                    dst,
                    edge_type,
                    self.get_unchecked_edge_weight_from_edge_id(edge_id),
                )
            },
        )
    }

    /// Return iterator on the edges of the graph.
    ///
    /// # Arguments
    /// * `directed`: bool - Whether to filter out the undirected edges.
    pub fn iter_unique_edge_node_ids(
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

    /// Return iterator on the unique sources of the graph.
    pub fn iter_unique_source_node_ids(&self) -> Box<dyn Iterator<Item = NodeT> + '_> {
        if let Some(x) = &self.unique_sources {
            return Box::new(x.iter().map(|source| source as NodeT));
        }
        Box::new(self.iter_node_ids())
    }
}
