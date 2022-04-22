use super::*;
use rayon::prelude::*;
use vec_rand::sample_uniform;

/// # Iterators
/// The naming convention for the iterators is:
/// If the method has the `par_` prefix then it should return a parallel iterator.
/// By default all the methods retruns both the ids and the name of the item and
/// if the method has the suffix `_ids` then it will returns **only** the ids.
/// Therefore, the naming convetions are:
/// * `/iter_(.+)/`
/// * `/iter_(.+)_unchecked/`
/// * `/par_iter_(.+)/`
/// * `/par_iter_(.+)_unchecked/`
impl Graph {
    /// Return iterator on the node IDs of the graph.
    pub fn iter_node_ids(&self) -> impl Iterator<Item = NodeT> + '_ {
        0..self.get_nodes_number()
    }

    /// Return parallel iterator on the node of the graph.
    pub fn par_iter_node_ids(&self) -> impl IndexedParallelIterator<Item = NodeT> + '_ {
        (0..self.get_nodes_number()).into_par_iter()
    }

    /// Return iterator on the node names of the graph.
    pub fn iter_node_names(&self) -> impl Iterator<Item = String> + '_ {
        self.iter_node_ids()
            .map(move |node_id| unsafe { self.get_unchecked_node_name_from_node_id(node_id) })
    }

    /// Return parallel iterator on the node names of the graph.
    pub fn par_iter_node_names(&self) -> impl IndexedParallelIterator<Item = String> + '_ {
        self.nodes.par_iter_keys()
    }

    /// Return iterator on the node urls of the graph.
    pub fn iter_node_urls(&self) -> impl Iterator<Item = Option<String>> + '_ {
        self.iter_node_names()
            .map(|node_name| get_node_source_url_from_node_name(&node_name).ok())
    }

    /// Return parallel iterator on the node urls of the graph.
    pub fn par_iter_node_urls(&self) -> impl IndexedParallelIterator<Item = Option<String>> + '_ {
        self.par_iter_node_names()
            .map(|node_name| get_node_source_url_from_node_name(&node_name).ok())
    }

    /// Return iterator on the node ontologies of the graph.
    pub fn iter_node_ontologies(&self) -> impl Iterator<Item = Option<String>> + '_ {
        self.iter_node_names()
            .map(move |node_name| unsafe { self.get_unchecked_ontology_from_node_name(&node_name) })
    }

    /// Return parallel iterator on the node ontologies of the graph.
    pub fn par_iter_node_ontologies(
        &self,
    ) -> impl IndexedParallelIterator<Item = Option<String>> + '_ {
        self.par_iter_node_names()
            .map(move |node_name| unsafe { self.get_unchecked_ontology_from_node_name(&node_name) })
    }

    /// Return iterator on the unique node type IDs of the graph.
    ///
    /// # Raises
    /// * If the graph does not contain node types.
    pub fn iter_unique_node_type_ids(&self) -> Result<impl Iterator<Item = NodeTypeT> + '_> {
        Ok(0..self.get_node_types_number()?)
    }

    /// Return parallel iterator on the unique node type IDs of the graph.
    ///
    /// # Raises
    /// * If the graph does not contain node types.
    pub fn par_iter_unique_node_type_ids(
        &self,
    ) -> Result<impl IndexedParallelIterator<Item = NodeTypeT> + '_> {
        Ok((0..self.get_node_types_number()?).into_par_iter())
    }

    /// Return iterator on the unique node type IDs counts of the graph.
    ///
    /// # Raises
    /// * If the graph does not contain node types.
    pub fn iter_node_type_counts(&self) -> Result<impl Iterator<Item = NodeT> + '_> {
        self.must_have_node_types()
            .map(|node_types| node_types.counts.iter().cloned())
    }

    /// Return iterator on the unique node type IDs and their counts of the graph.
    pub fn iter_unique_node_type_ids_and_counts(
        &self,
    ) -> Result<impl Iterator<Item = (NodeTypeT, NodeT)> + '_> {
        Ok(self
            .iter_unique_node_type_ids()?
            .zip(self.iter_node_type_counts()?))
    }

    /// Return iterator on the unique node type names of the graph.
    pub fn iter_unique_node_type_names(&self) -> Result<impl Iterator<Item = String> + '_> {
        self.must_have_node_types()
            .map(|node_types| node_types.vocabulary.iter_keys())
    }

    /// Return iterator on the unique node type names and their counts of the graph.
    pub fn iter_unique_node_type_names_and_counts(
        &self,
    ) -> Result<impl Iterator<Item = (String, NodeT)> + '_> {
        Ok(self
            .iter_unique_node_type_names()?
            .zip(self.iter_node_type_counts()?))
    }

    /// Return iterator on the edge type IDs of the graph.
    pub fn iter_unique_edge_type_ids(&self) -> Result<impl Iterator<Item = EdgeTypeT> + '_> {
        Ok(0..self.get_edge_types_number()?)
    }

    /// Return parallel iterator on the unique edge type IDs of the graph.
    ///
    /// # Raises
    /// * If the graph does not contain edge types.
    pub fn par_iter_unique_edge_type_ids(
        &self,
    ) -> Result<impl IndexedParallelIterator<Item = EdgeTypeT> + '_> {
        Ok((0..self.get_edge_types_number()?).into_par_iter())
    }

    /// Return iterator on the unique edge type IDs counts of the graph.
    pub fn iter_edge_type_counts(&self) -> Result<impl Iterator<Item = EdgeT> + '_> {
        self.must_have_edge_types()
            .map(|edge_types| edge_types.counts.iter().cloned())
    }

    /// Return iterator on the unique edge type IDs and their counts of the graph.
    pub fn iter_unique_edge_type_ids_and_counts(
        &self,
    ) -> Result<impl Iterator<Item = (EdgeTypeT, EdgeT)> + '_> {
        Ok(self
            .iter_unique_edge_type_ids()?
            .zip(self.iter_edge_type_counts()?))
    }

    /// Return iterator on the unique edge type names and their counts of the graph.
    pub fn iter_unique_edge_type_names_and_counts(
        &self,
    ) -> Result<impl Iterator<Item = (String, EdgeT)> + '_> {
        Ok(self
            .iter_unique_edge_type_names()?
            .zip(self.iter_edge_type_counts()?))
    }

    /// Return iterator on the unique edge type names of the graph.
    pub fn iter_unique_edge_type_names(&self) -> Result<impl Iterator<Item = String> + '_> {
        self.must_have_edge_types()
            .map(|edge_types| edge_types.vocabulary.iter_keys())
    }

    /// Return iterator on the unweighted node degrees of the graph.
    ///
    /// Note that with unweighted it is meant that if this graph instance
    /// has weights, the degree will not take them into consideration.
    pub fn iter_node_degrees(&self) -> impl Iterator<Item = NodeT> + '_ {
        self.iter_node_ids()
            .map(move |node| unsafe { self.get_unchecked_node_degree_from_node_id(node) })
    }

    /// Return parallel iterator on the unweighted node degrees of the graph.
    ///
    /// Note that with unweighted it is meant that if this graph instance
    /// has weights, the degree will not take them into consideration.
    pub fn par_iter_node_degrees(&self) -> impl IndexedParallelIterator<Item = NodeT> + '_ {
        self.par_iter_node_ids()
            .map(move |node_id| unsafe { self.get_unchecked_node_degree_from_node_id(node_id) })
    }

    /// Return iterator on the unweighted comulative node degrees of the graph.
    pub fn iter_comulative_node_degrees(&self) -> impl Iterator<Item = EdgeT> + '_ {
        self.iter_node_ids().map(move |node_id| unsafe {
            self.get_unchecked_comulative_node_degree_from_node_id(node_id)
        })
    }

    /// Return parallel iterator on the unweighted comulative node degrees of the graph.
    pub fn par_iter_comulative_node_degrees(
        &self,
    ) -> impl IndexedParallelIterator<Item = EdgeT> + '_ {
        self.par_iter_node_ids().map(move |node_id| unsafe {
            self.get_unchecked_comulative_node_degree_from_node_id(node_id)
        })
    }

    /// Return iterator on the unweighted reciprocal squared root node degrees of the graph.
    pub fn iter_reciprocal_sqrt_degrees(&self) -> impl Iterator<Item = WeightT> + '_ {
        self.iter_node_ids().map(move |node_id| unsafe {
            self.get_unchecked_reciprocal_sqrt_degree_from_node_id(node_id)
        })
    }

    /// Return parallel iterator on the unweighted reciprocal squared root node degrees of the graph.
    pub fn par_iter_reciprocal_sqrt_degrees(
        &self,
    ) -> impl IndexedParallelIterator<Item = WeightT> + '_ {
        self.par_iter_node_ids().map(move |node_id| unsafe {
            self.get_unchecked_reciprocal_sqrt_degree_from_node_id(node_id)
        })
    }

    /// Return iterator on the weighted node degrees of the graph.
    ///
    /// Note that with weighted it is meant that if this graph instance
    /// has weights, the degree will be weighted on the edge weight.
    ///
    /// Note that if one or more edges have a negative edge weight,
    /// the resulting node degree may be negative.
    /// This check is **NOT** done by this method, as in some situations
    /// this may be desired by the user.
    pub fn iter_weighted_node_degrees(&self) -> Result<impl Iterator<Item = f64> + '_> {
        self.must_have_edge_weights()?;
        Ok(self.iter_node_ids().map(move |node_id| unsafe {
            self.get_unchecked_weighted_node_degree_from_node_id(node_id)
        }))
    }

    /// Return iterator on the weighted node degrees of the graph.
    ///
    /// Note that with weighted it is meant that if this graph instance
    /// has weights, the degree will not take them into consideration.
    ///
    /// Note that if one or more edges have a negative edge weight,
    /// the resulting node degree may be negative.
    /// This check is **NOT** done by this method, as in some situations
    /// this may be desired by the user.
    pub fn par_iter_weighted_node_degrees(
        &self,
    ) -> Result<impl IndexedParallelIterator<Item = f64> + '_> {
        self.must_have_edge_weights()?;
        Ok(self.par_iter_node_ids().map(move |node_id| unsafe {
            self.get_unchecked_weighted_node_degree_from_node_id(node_id)
        }))
    }

    /// Return iterator on the non-singleton nodes of the graph.
    ///
    /// Note that this includes also the singleton with self-loops and
    /// the trap nodes within this iterator. Only true singleton nodes,
    /// that is, nodes without any edge (both inbound and outbound) are
    /// excluded.
    ///
    /// Since the following requires to be boxed, we cannot create the
    /// parallel version of this iterator.
    ///
    pub fn iter_connected_node_ids(&self) -> Box<dyn Iterator<Item = NodeT> + '_> {
        match self.connected_nodes.as_ref() {
            Some(nsns) => Box::new(nsns.iter_ones().map(|node_id| node_id as NodeT)),
            _ => Box::new(self.iter_node_ids()),
        }
    }

    /// Return iterator on the singleton nodes IDs of the graph.
    pub fn iter_singleton_node_ids(&self) -> Box<dyn Iterator<Item = NodeT> + '_> {
        match self.connected_nodes.as_ref() {
            Some(nsns) => Box::new(nsns.iter_zeros().map(|node_id| node_id as NodeT).filter(
                move |&node_id| unsafe {
                    self.get_unchecked_node_degree_from_node_id(node_id) == 0
                },
            )),
            _ => Box::new(::std::iter::empty()),
        }
    }

    /// Return iterator on the singleton nodes names of the graph.
    pub fn iter_singleton_node_names(&self) -> impl Iterator<Item = String> + '_ {
        self.iter_singleton_node_ids()
            .map(move |node_id| unsafe { self.get_unchecked_node_name_from_node_id(node_id) })
    }

    /// Return iterator on the singleton with selfloops node IDs of the graph.
    pub fn iter_singleton_nodes_with_selfloops_node_ids(&self) -> impl Iterator<Item = NodeT> + '_ {
        self.iter_node_ids().filter(move |&node_id| unsafe {
            self.is_unchecked_singleton_with_selfloops_from_node_id(node_id)
        })
    }

    /// Return parallell iterator on the singleton with selfloops node IDs of the graph.
    pub fn par_iter_singleton_nodes_with_selfloops_node_ids(
        &self,
    ) -> impl ParallelIterator<Item = NodeT> + '_ {
        self.par_iter_node_ids().filter(move |&node_id| unsafe {
            self.is_unchecked_singleton_with_selfloops_from_node_id(node_id)
        })
    }

    /// Return iterator on the singleton with selfloops node names of the graph.
    pub fn iter_singleton_nodes_with_selfloops_node_names(
        &self,
    ) -> impl Iterator<Item = String> + '_ {
        self.iter_singleton_nodes_with_selfloops_node_ids()
            .map(move |node_id| unsafe { self.get_unchecked_node_name_from_node_id(node_id) })
    }

    /// Return iterator on the singleton node type IDs of the graph.
    ///
    /// # Raises
    /// * If there are no node types in the graph.
    pub fn iter_singleton_node_type_ids(&self) -> Result<impl Iterator<Item = NodeTypeT> + '_> {
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
    pub fn iter_singleton_edge_type_ids(&self) -> Result<impl Iterator<Item = EdgeTypeT> + '_> {
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
    pub fn iter_singleton_node_type_names(&self) -> Result<impl Iterator<Item = String> + '_> {
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
    pub fn iter_singleton_edge_type_names(&self) -> Result<impl Iterator<Item = String> + '_> {
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

    /// Return iterator on the homogeneous node type ids.
    pub fn iter_homogeneous_node_type_ids(&self) -> Result<impl Iterator<Item = NodeTypeT> + '_> {
        self.must_have_node_types().map(move |node_types| {
            node_types
                .counts
                .iter()
                .enumerate()
                .filter(move |&(_, node_type_count)| *node_type_count == self.get_nodes_number())
                .map(|(node_type_id, _)| NodeTypeT::from_usize(node_type_id))
        })
    }

    /// Return iterator on the homogeneous node type names.
    pub fn iter_homogeneous_node_type_names(&self) -> Result<impl Iterator<Item = String> + '_> {
        Ok(self
            .iter_homogeneous_node_type_ids()?
            .map(move |node_type_id| {
                self.get_node_type_name_from_node_type_id(node_type_id)
                    .unwrap()
            }))
    }

    /// Return parallel iterator on the homogeneous node type ids.
    pub fn par_iter_homogeneous_node_type_ids(
        &self,
    ) -> Result<impl ParallelIterator<Item = NodeTypeT> + '_> {
        self.must_have_node_types().map(move |node_types| {
            node_types
                .counts
                .par_iter()
                .enumerate()
                .filter(move |&(_, node_type_count)| *node_type_count == self.get_nodes_number())
                .map(|(node_type_id, _)| NodeTypeT::from_usize(node_type_id))
        })
    }

    /// Return parallel iterator on the homogeneous node type names.
    pub fn par_iter_homogeneous_node_type_names(
        &self,
    ) -> Result<impl ParallelIterator<Item = String> + '_> {
        Ok(self
            .par_iter_homogeneous_node_type_ids()?
            .map(move |node_type_id| {
                self.get_node_type_name_from_node_type_id(node_type_id)
                    .unwrap()
            }))
    }

    /// Return iterator on the (non unique) source nodes of the graph.
    ///
    /// # Arguments
    /// * `directed`: bool - Whether to filter out the undirected edges.
    pub fn iter_source_node_ids(&self, directed: bool) -> impl Iterator<Item = NodeT> + '_ {
        self.iter_edge_node_ids(directed)
            .map(move |(_, src, _)| src)
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
    pub fn iter_edge_weights(&self) -> Result<impl Iterator<Item = WeightT> + '_> {
        self.must_have_edge_weights()?;
        Ok(self
            .weights
            .as_ref()
            .as_ref()
            .map(|ws| ws.iter().cloned())
            .unwrap())
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
    ) -> Result<impl IndexedParallelIterator<Item = WeightT> + '_> {
        self.must_have_edge_weights()?;
        Ok(self
            .weights
            .as_ref()
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
        self.par_iter_edge_node_ids(directed)
            .map(move |(_, src, _)| src)
    }

    /// Return parallel iterator on the (non unique) directed source nodes of the graph.
    pub fn par_iter_directed_source_node_ids(
        &self,
    ) -> impl IndexedParallelIterator<Item = NodeT> + '_ {
        self.par_iter_directed_edge_node_ids()
            .map(move |(_, src, _)| src)
    }

    /// Return iterator on the (non unique) destination nodes of the graph.
    ///
    /// # Arguments
    /// * `directed`: bool - Whether to filter out the undirected edges.
    pub fn iter_destination_node_ids(&self, directed: bool) -> impl Iterator<Item = NodeT> + '_ {
        self.iter_edge_node_ids(directed)
            .map(move |(_, _, dst)| dst)
    }

    /// Return parallel iterator on the (non unique) destination nodes of the graph.
    ///
    /// # Arguments
    /// * `directed`: bool - Whether to filter out the undirected edges.
    pub fn par_iter_destination_node_ids(
        &self,
        directed: bool,
    ) -> impl ParallelIterator<Item = NodeT> + '_ {
        self.par_iter_edge_node_ids(directed)
            .map(move |(_, _, dst)| dst)
    }

    /// Return iterator on random (non unique) source node IDs.
    ///
    /// # Arguments
    /// 'quantity': usize - Number of nodes to sample.
    /// 'random_state': u64 - Random state to use to sample the nodes.
    pub fn iter_random_source_node_ids(
        &self,
        quantity: usize,
        mut random_state: u64,
    ) -> impl Iterator<Item = NodeT> + '_ {
        let number_of_directed_edges = self.get_number_of_directed_edges();
        (0..quantity).map(move |_| unsafe {
            random_state = splitmix64(random_state);
            self.get_unchecked_node_ids_from_edge_id(sample_uniform(
                number_of_directed_edges,
                random_state,
            ) as EdgeT)
                .0
        })
    }

    /// Return parallel iterator on random (non unique) source node IDs.
    ///
    /// # Arguments
    /// 'quantity': usize - Number of nodes to sample.
    /// 'random_state': u64 - Random state to use to sample the nodes.
    pub fn par_iter_random_source_node_ids(
        &self,
        quantity: usize,
        random_state: u64,
    ) -> impl IndexedParallelIterator<Item = NodeT> + '_ {
        let number_of_directed_edges = self.get_number_of_directed_edges();
        (0..quantity).into_par_iter().map(move |i| unsafe {
            self.get_unchecked_node_ids_from_edge_id(sample_uniform(
                number_of_directed_edges,
                splitmix64(random_state + i as u64),
            ) as EdgeT)
                .0
        })
    }

    /// Return iterator on the (non unique) directed destination nodes of the graph.
    pub fn iter_directed_destination_node_ids(&self) -> impl Iterator<Item = NodeT> + '_ {
        self.iter_directed_edge_node_ids()
            .map(move |(_, _, dst)| dst)
    }

    /// Return parallel iterator on the (non unique) directed destination nodes of the graph.
    pub fn par_iter_directed_destination_node_ids(
        &self,
    ) -> impl IndexedParallelIterator<Item = NodeT> + '_ {
        self.par_iter_directed_edge_node_ids()
            .map(move |(_, _, dst)| dst)
    }

    /// Return iterator on the node IDs and ther node type IDs.
    pub fn iter_node_ids_and_node_type_ids(
        &self,
    ) -> impl Iterator<Item = (NodeT, Option<&Vec<NodeTypeT>>)> + '_ {
        self.iter_node_ids().map(move |node_id| unsafe {
            (
                node_id,
                self.get_unchecked_node_type_ids_from_node_id(node_id),
            )
        })
    }

    /// Return iterator on the node IDs and ther node type IDs.
    pub fn par_iter_node_ids_and_node_type_ids(
        &self,
    ) -> impl IndexedParallelIterator<Item = (NodeT, Option<&Vec<NodeTypeT>>)> + '_ {
        self.par_iter_node_ids().map(move |node_id| unsafe {
            (
                node_id,
                self.get_unchecked_node_type_ids_from_node_id(node_id),
            )
        })
    }

    /// Return iterator on the node type IDs.
    ///
    /// # Safety
    /// If the graph does not contain node types, this iterator will be an
    /// iterator over None values.
    pub unsafe fn iter_unchecked_node_type_ids(
        &self,
    ) -> impl Iterator<Item = Option<&Vec<NodeTypeT>>> + '_ {
        self.iter_node_ids()
            .map(move |node_id| self.get_unchecked_node_type_ids_from_node_id(node_id))
    }

    /// Return iterator on the one-hot encoded node type IDs.
    ///
    /// # Raises
    /// * If the graph does not contain node types.
    pub fn iter_one_hot_encoded_node_type_ids(
        &self,
    ) -> Result<impl Iterator<Item = Vec<bool>> + '_> {
        let node_types_number = self.get_node_types_number()?;
        Ok(unsafe {
            self.iter_unchecked_node_type_ids()
                .map(move |maybe_node_types| {
                    let mut dummies = vec![false; node_types_number as usize];
                    if let Some(node_types) = maybe_node_types {
                        node_types.iter().for_each(|&node_type| {
                            dummies[node_type as usize] = true;
                        });
                    }
                    dummies
                })
        })
    }

    /// Return iterator on the known one-hot encoded node type IDs.
    ///
    /// # Raises
    /// * If the graph does not contain node types.
    pub fn iter_one_hot_encoded_known_node_type_ids(
        &self,
    ) -> Result<impl Iterator<Item = Vec<bool>> + '_> {
        let node_types_number = self.get_node_types_number()?;
        Ok(unsafe {
            self.iter_unchecked_node_type_ids()
                .filter_map(move |maybe_node_types| {
                    if let Some(node_types) = maybe_node_types {
                        let mut dummies = vec![false; node_types_number as usize];
                        node_types.iter().for_each(|&node_type| {
                            dummies[node_type as usize] = true;
                        });
                        Some(dummies)
                    } else {
                        None
                    }
                })
        })
    }

    /// Return iterator on the node of the graph.
    ///
    /// # Safety
    /// If the graph does not contain node types, this iterator will be an
    /// iterator over None values as node types.
    pub unsafe fn par_iter_unchecked_node_ids_and_node_type_ids(
        &self,
    ) -> impl IndexedParallelIterator<Item = (NodeT, Option<&Vec<NodeTypeT>>)> + '_ {
        self.par_iter_node_ids().map(move |node_id| {
            (
                node_id,
                self.get_unchecked_node_type_ids_from_node_id(node_id),
            )
        })
    }

    /// Return iterator on the node of the graph as Strings.
    pub fn iter_node_names_and_node_type_names(
        &self,
    ) -> impl Iterator<Item = (NodeT, String, Option<&Vec<NodeTypeT>>, Option<Vec<String>>)> + '_
    {
        self.iter_node_ids_and_node_type_ids()
            .map(move |(node_id, node_types)| unsafe {
                (
                    node_id,
                    self.get_unchecked_node_name_from_node_id(node_id),
                    node_types,
                    self.get_unchecked_node_type_names_from_node_id(node_id),
                )
            })
    }

    /// Return parallell iterator on the node of the graph as Strings.
    pub fn par_iter_node_names_and_node_type_names(
        &self,
    ) -> impl IndexedParallelIterator<
        Item = (NodeT, String, Option<&Vec<NodeTypeT>>, Option<Vec<String>>),
    > + '_ {
        unsafe {
            self.par_iter_unchecked_node_ids_and_node_type_ids().map(
                move |(node_id, node_types)| {
                    (
                        node_id,
                        self.get_unchecked_node_name_from_node_id(node_id),
                        node_types,
                        self.get_unchecked_node_type_names_from_node_id(node_id),
                    )
                },
            )
        }
    }

    /// Return iterator on the edges of the graph.
    ///
    /// # Arguments
    /// * `directed`: bool - Whether to filter out the undirected edges.
    pub fn iter_edge_node_ids(
        &self,
        directed: bool,
    ) -> Box<dyn Iterator<Item = (EdgeT, NodeT, NodeT)> + '_> {
        if self.sources.is_some() && self.destinations.is_some() {
            return Box::new(
                (0..self.get_number_of_directed_edges()).filter_map(move |edge_id| {
                    let (src, dst) = unsafe { self.get_unchecked_node_ids_from_edge_id(edge_id) };
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

    /// Return iterator on the edges of the graph.
    pub fn iter_directed_edge_node_ids(
        &self,
    ) -> Box<dyn Iterator<Item = (EdgeT, NodeT, NodeT)> + Send + '_> {
        if self.sources.is_some() && self.destinations.is_some() {
            return Box::new(
                (0..self.get_number_of_directed_edges()).filter_map(move |edge_id| {
                    let (src, dst) = unsafe { self.get_unchecked_node_ids_from_edge_id(edge_id) };
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
        self.iter_edge_node_ids(directed)
            .map(move |(edge_id, src, dst)| unsafe {
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
    pub fn par_iter_edge_node_ids(
        &self,
        directed: bool,
    ) -> impl ParallelIterator<Item = (EdgeT, NodeT, NodeT)> + '_ {
        self.edges
            .par_iter()
            .enumerate()
            .filter_map(move |(edge_id, edge)| {
                let (src, dst) = self.decode_edge(edge);
                if !directed && src > dst {
                    return None;
                }
                Some((edge_id as EdgeT, src, dst))
            })
    }

    /// Return iterator on the directed edges of the graph.
    pub fn par_iter_directed_edge_node_ids(
        &self,
    ) -> impl IndexedParallelIterator<Item = (EdgeT, NodeT, NodeT)> + '_ {
        self.edges
            .par_iter()
            .enumerate()
            .map(move |(edge_id, edge)| {
                let (src, dst) = self.decode_edge(edge);
                (edge_id as EdgeT, src, dst)
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
        self.par_iter_edge_node_ids(directed)
            .map(move |(edge_id, src, dst)| unsafe {
                (
                    edge_id,
                    src,
                    self.get_unchecked_node_name_from_node_id(src),
                    dst,
                    self.get_unchecked_node_name_from_node_id(dst),
                )
            })
    }

    /// Return iterator on the edges of the graph with the string name.
    pub fn par_iter_directed_edges(
        &self,
    ) -> impl IndexedParallelIterator<Item = (EdgeT, NodeT, String, NodeT, String)> + '_ {
        self.par_iter_directed_edge_node_ids()
            .map(move |(edge_id, src, dst)| unsafe {
                (
                    edge_id,
                    src,
                    self.get_unchecked_node_name_from_node_id(src),
                    dst,
                    self.get_unchecked_node_name_from_node_id(dst),
                )
            })
    }

    /// Return iterator on the edges node IDs of the graph and their weights.
    ///
    /// # Raises
    /// * If the current graph instance does not contain edge weights.
    pub fn iter_edge_node_ids_and_edge_weight(
        &self,
    ) -> Result<impl Iterator<Item = (EdgeT, NodeT, NodeT, WeightT)> + '_> {
        Ok(self
            .iter_edge_node_ids(true)
            .zip(self.iter_edge_weights()?)
            .map(move |((edge_id, src, dst), weight)| (edge_id, src, dst, weight)))
    }

    /// Return parallel iterator on the edges node IDs of the graph and their weights.
    ///
    /// # Raises
    /// * If the current graph instance does not contain edge weights.
    pub fn par_iter_edge_node_ids_and_edge_weight(
        &self,
    ) -> Result<impl IndexedParallelIterator<Item = (EdgeT, NodeT, NodeT, WeightT)> + '_> {
        Ok(self
            .par_iter_directed_edge_node_ids()
            .zip(self.par_iter_edge_weights()?)
            .map(move |((edge_id, src, dst), weight)| (edge_id, src, dst, weight)))
    }

    /// Return iterator on the edge node IDs of the graph and their edge type ID
    ///
    /// # Arguments
    /// * `directed`: bool - Whether to filter out the undirected edges.
    pub fn iter_edge_node_ids_and_edge_type_id(
        &self,
        directed: bool,
    ) -> impl Iterator<Item = (EdgeT, NodeT, NodeT, Option<EdgeTypeT>)> + '_ {
        self.iter_edge_node_ids(directed)
            .map(move |(edge_id, src, dst)| unsafe {
                (
                    edge_id,
                    src,
                    dst,
                    self.get_unchecked_edge_type_id_from_edge_id(edge_id),
                )
            })
    }

    /// Return iterator on the directed edge node IDs of the graph and their edge type ID
    pub fn iter_directed_edge_node_ids_and_edge_type_id(
        &self,
    ) -> impl Iterator<Item = (EdgeT, NodeT, NodeT, Option<EdgeTypeT>)> + '_ {
        self.iter_directed_edge_node_ids()
            .map(move |(edge_id, src, dst)| unsafe {
                (
                    edge_id,
                    src,
                    dst,
                    self.get_unchecked_edge_type_id_from_edge_id(edge_id),
                )
            })
    }

    /// Return iterator on the one-hot encoded edge type IDs.
    ///
    /// # Raises
    /// * If the current graph instance does not contain edge types.
    pub fn iter_one_hot_encoded_edge_type_ids(
        &self,
    ) -> Result<impl Iterator<Item = Vec<bool>> + '_> {
        let edge_types_number = self.get_edge_types_number()?;
        Ok(self
            .get_edge_type_ids()?
            .into_iter()
            .map(move |maybe_edge_type| {
                let mut dummies = vec![false; edge_types_number as usize];
                if let Some(edge_type) = maybe_edge_type {
                    dummies[edge_type as usize] = true;
                }
                dummies
            }))
    }

    /// Return iterator on the one-hot encoded known edge type IDs.
    ///
    /// # Raises
    /// * If the current graph instance does not contain edge types.
    pub fn iter_one_hot_encoded_known_edge_type_ids(
        &self,
    ) -> Result<impl Iterator<Item = Vec<bool>> + '_> {
        let edge_types_number = self.get_edge_types_number()?;
        Ok(self
            .get_edge_type_ids()?
            .into_iter()
            .filter_map(move |maybe_edge_type| {
                if let Some(edge_type) = maybe_edge_type {
                    let mut dummies = vec![false; edge_types_number as usize];
                    dummies[edge_type as usize] = true;
                    Some(dummies)
                } else {
                    None
                }
            }))
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
            .map(move |(edge_id, src, src_name, dst, dst_name)| unsafe {
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
            .map(move |(edge_id, src, src_name, dst, dst_name)| unsafe {
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

    /// Return iterator on the directed edges of the graph with the ids and string name.
    /// The result is (edge_id, src, src_name, dst, dst_name, edge_type, edge_type_name)
    pub fn par_iter_directed_edge_node_names_and_edge_type_name(
        &self,
    ) -> impl IndexedParallelIterator<
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
        self.par_iter_directed_edges()
            .map(move |(edge_id, src, src_name, dst, dst_name)| unsafe {
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
        self.par_iter_edge_node_ids(directed)
            .map(move |(edge_id, src, dst)| unsafe {
                (
                    edge_id,
                    src,
                    dst,
                    self.get_unchecked_edge_type_id_from_edge_id(edge_id),
                )
            })
    }

    /// Return iterator on the directed edges of the graph.
    pub fn par_iter_directed_edge_node_ids_and_edge_type_id(
        &self,
    ) -> impl IndexedParallelIterator<Item = (EdgeT, NodeT, NodeT, Option<EdgeTypeT>)> + '_ {
        self.par_iter_directed_edge_node_ids()
            .map(move |(edge_id, src, dst)| unsafe {
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
                move |(edge_id, src, src_name, dst, dst_name, edge_type, edge_type_name)| unsafe {
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

    /// Return iterator on the directed edges of the graph with the string name.
    pub fn par_iter_directed_edge_node_names_and_edge_type_name_and_edge_weight(
        &self,
    ) -> impl IndexedParallelIterator<
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
        self.par_iter_directed_edge_node_names_and_edge_type_name()
            .map(
                move |(edge_id, src, src_name, dst, dst_name, edge_type, edge_type_name)| unsafe {
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
            move |(edge_id, src, src_name, dst, dst_name, edge_type, edge_type_name)| unsafe {
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
            move |(edge_id, src, dst, edge_type)| unsafe {
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

    /// Return iterator on the directed edges of the graph with the string name.
    pub fn par_iter_directed_edge_node_ids_and_edge_type_id_and_edge_weight(
        &self,
    ) -> impl IndexedParallelIterator<Item = (EdgeT, NodeT, NodeT, Option<EdgeTypeT>, Option<WeightT>)>
           + '_ {
        self.par_iter_directed_edge_node_ids_and_edge_type_id().map(
            move |(edge_id, src, dst, edge_type)| unsafe {
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

    /// Return iterator on the edges of the graph including node IDs, edge type and edge weight.
    ///
    /// # Arguments
    /// * `directed`: bool - Whether to filter out the undirected edges.
    pub fn iter_edge_node_ids_and_edge_type_id_and_edge_weight(
        &self,
        directed: bool,
    ) -> impl Iterator<Item = (EdgeT, NodeT, NodeT, Option<EdgeTypeT>, Option<WeightT>)> + '_ {
        self.iter_edge_node_ids_and_edge_type_id(directed).map(
            move |(edge_id, src, dst, edge_type)| unsafe {
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

    /// Return iterator on the directed edges of the graph including node IDs, edge type and edge weight.
    pub fn iter_directed_edge_node_ids_and_edge_type_id_and_edge_weight(
        &self,
    ) -> impl Iterator<Item = (EdgeT, NodeT, NodeT, Option<EdgeTypeT>, Option<WeightT>)> + '_ {
        self.iter_directed_edge_node_ids_and_edge_type_id().map(
            move |(edge_id, src, dst, edge_type)| unsafe {
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
            return Box::new((0..self.get_number_of_directed_edges()).filter_map(
                move |edge_id| unsafe {
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
                },
            ));
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
    pub fn iter_unique_source_node_ids(&self) -> impl Iterator<Item = NodeT> + '_ {
        self.iter_node_ids().filter(move |&node_id| unsafe {
            self.get_unchecked_node_degree_from_node_id(node_id) > 0
        })
    }

    /// Return parallell iterator on the unique sources of the graph.
    pub fn par_iter_unique_source_node_ids(&self) -> impl ParallelIterator<Item = NodeT> + '_ {
        self.par_iter_node_ids().filter(move |&node_id| unsafe {
            self.get_unchecked_node_degree_from_node_id(node_id) > 0
        })
    }

    /// Returns iterator over edge IDs of the edges with unknown edge types
    ///
    /// # Raises
    /// * If there are no edge types in the graph.
    pub fn iter_edge_ids_with_unknown_edge_types(
        &self,
    ) -> Result<impl Iterator<Item = EdgeT> + '_> {
        self.must_have_edge_types().map(|edge_types| {
            edge_types
                .ids
                .iter()
                .enumerate()
                .filter_map(|(edge_id, edge_type_id)| {
                    if edge_type_id.is_some() {
                        None
                    } else {
                        Some(edge_id as EdgeT)
                    }
                })
        })
    }

    /// Returns iterator over edge IDs of the edges with known edge types
    ///
    /// # Raises
    /// * If there are no edge types in the graph.
    pub fn iter_edge_ids_with_known_edge_types(&self) -> Result<impl Iterator<Item = EdgeT> + '_> {
        self.must_have_edge_types().map(|edge_types| {
            edge_types
                .ids
                .iter()
                .enumerate()
                .filter_map(|(edge_id, edge_type_id)| {
                    if edge_type_id.is_some() {
                        Some(edge_id as EdgeT)
                    } else {
                        None
                    }
                })
        })
    }

    /// Returns iterator over edge node IDs of the edges with unknown edge types
    ///
    /// # Arguments
    /// * `directed`: bool - Whether to load the edges as directed or undirected.
    ///
    /// # Raises
    /// * If there are no edge types in the graph.
    pub fn iter_edge_node_ids_with_unknown_edge_types(
        &self,
        directed: bool,
    ) -> Result<impl Iterator<Item = (NodeT, NodeT)> + '_> {
        self.must_have_edge_types()?;
        Ok(self
            .iter_edge_node_ids_and_edge_type_id(directed)
            .filter_map(|(_, src, dst, edge_type)| {
                if edge_type.is_none() {
                    Some((src, dst))
                } else {
                    None
                }
            }))
    }

    /// Returns iterator over edge node IDs of the edges with known edge types
    ///
    /// # Arguments
    /// * `directed`: bool - Whether to load the edges as directed or undirected.
    ///
    /// # Raises
    /// * If there are no edge types in the graph.
    pub fn iter_edge_node_ids_with_known_edge_types(
        &self,
        directed: bool,
    ) -> Result<impl Iterator<Item = (NodeT, NodeT)> + '_> {
        self.must_have_edge_types()?;
        Ok(self
            .iter_edge_node_ids_and_edge_type_id(directed)
            .filter_map(|(_, src, dst, edge_type)| {
                if edge_type.is_some() {
                    Some((src, dst))
                } else {
                    None
                }
            }))
    }

    /// Returns iterator over node IDs of the nodes with unknown node types
    ///
    /// # Raises
    /// * If there are no node types in the graph.
    pub fn iter_node_ids_with_unknown_node_types(
        &self,
    ) -> Result<impl Iterator<Item = NodeT> + '_> {
        self.must_have_node_types().map(|node_types| {
            node_types
                .ids
                .iter()
                .enumerate()
                .filter_map(|(node_id, node_type_id)| {
                    if node_type_id.is_some() {
                        None
                    } else {
                        Some(node_id as NodeT)
                    }
                })
        })
    }

    /// Returns iterator over node IDs of the nodes with given node type ID.
    ///
    /// # Argument
    /// * `node_type_id`: node_type_id - The node type ID to filter for.
    ///
    /// # Raises
    /// * If there are no node types in the graph.
    /// * If the given node type ID does not exist in the current graph instance.
    pub fn iter_node_ids_from_node_type_id(
        &self,
        node_type_id: NodeTypeT,
    ) -> Result<impl Iterator<Item = NodeT> + '_> {
        self.validate_node_type_id(Some(node_type_id))?;
        self.must_have_node_types().map(|node_types| {
            node_types
                .ids
                .iter()
                .enumerate()
                .filter_map(move |(node_id, node_type_ids)| {
                    if node_type_ids
                        .as_ref()
                        .map_or(false, |node_type_ids| node_type_ids.contains(&node_type_id))
                    {
                        Some(node_id as NodeT)
                    } else {
                        None
                    }
                })
        })
    }

    /// Returns iterator over node IDs of the nodes with given node type name.
    ///
    /// # Argument
    /// * `node_type_name`: &str - The node type name to filter for.
    ///
    /// # Raises
    /// * If there are no node types in the graph.
    /// * If the given node type name does not exist in the current graph instance.
    pub fn iter_node_ids_from_node_type_name(
        &self,
        node_type_name: &str,
    ) -> Result<impl Iterator<Item = NodeT> + '_> {
        self.iter_node_ids_from_node_type_id(
            self.get_node_type_id_from_node_type_name(node_type_name)?,
        )
    }

    /// Returns parallel iterator over node IDs of the nodes with given node type ID.
    ///
    /// # Argument
    /// * `node_type_id`: node_type_id - The node type ID to filter for.
    ///
    /// # Raises
    /// * If there are no node types in the graph.
    /// * If the given node type ID does not exist in the current graph instance.
    pub fn par_iter_node_ids_from_node_type_id(
        &self,
        node_type_id: NodeTypeT,
    ) -> Result<impl ParallelIterator<Item = NodeT> + '_> {
        self.validate_node_type_id(Some(node_type_id))?;
        self.must_have_node_types().map(|node_types| {
            node_types
                .ids
                .par_iter()
                .enumerate()
                .filter_map(move |(node_id, node_type_ids)| {
                    if node_type_ids
                        .as_ref()
                        .map_or(false, |node_type_ids| node_type_ids.contains(&node_type_id))
                    {
                        Some(node_id as NodeT)
                    } else {
                        None
                    }
                })
        })
    }

    /// Returns parallel iterator over node IDs of the nodes with given node type name.
    ///
    /// # Argument
    /// * `node_type_name`: &str - The node type name to filter for.
    ///
    /// # Raises
    /// * If there are no node types in the graph.
    /// * If the given node type name does not exist in the current graph instance.
    pub fn par_iter_node_ids_from_node_type_name(
        &self,
        node_type_name: &str,
    ) -> Result<impl ParallelIterator<Item = NodeT> + '_> {
        self.par_iter_node_ids_from_node_type_id(
            self.get_node_type_id_from_node_type_name(node_type_name)?,
        )
    }

    /// Returns iterator over node names of the nodes with given node type ID.
    ///
    /// # Argument
    /// * `node_type_id`: node_type_id - The node type ID to filter for.
    ///
    /// # Raises
    /// * If there are no node types in the graph.
    /// * If the given node type ID does not exist in the current graph instance.
    pub fn iter_node_names_from_node_type_id(
        &self,
        node_type_id: NodeTypeT,
    ) -> Result<impl Iterator<Item = String> + '_> {
        self.validate_node_type_id(Some(node_type_id))?;
        self.must_have_node_types().map(|node_types| {
            node_types
                .ids
                .iter()
                .enumerate()
                .filter_map(move |(node_id, node_type_ids)| unsafe {
                    if node_type_ids
                        .as_ref()
                        .map_or(false, |node_type_ids| node_type_ids.contains(&node_type_id))
                    {
                        Some(self.get_unchecked_node_name_from_node_id(node_id as NodeT))
                    } else {
                        None
                    }
                })
        })
    }

    /// Returns iterator over node names of the nodes with given node type name.
    ///
    /// # Argument
    /// * `node_type_name`: &str - The node type name to filter for.
    ///
    /// # Raises
    /// * If there are no node types in the graph.
    /// * If the given node type name does not exist in the current graph instance.
    pub fn iter_node_names_from_node_type_name(
        &self,
        node_type_name: &str,
    ) -> Result<impl Iterator<Item = String> + '_> {
        self.iter_node_names_from_node_type_id(
            self.get_node_type_id_from_node_type_name(node_type_name)?,
        )
    }

    /// Returns parallel iterator over node names of the nodes with given node type ID.
    ///
    /// # Argument
    /// * `node_type_id`: node_type_id - The node type ID to filter for.
    ///
    /// # Raises
    /// * If there are no node types in the graph.
    /// * If the given node type ID does not exist in the current graph instance.
    pub fn par_iter_node_names_from_node_type_id(
        &self,
        node_type_id: NodeTypeT,
    ) -> Result<impl ParallelIterator<Item = String> + '_> {
        self.validate_node_type_id(Some(node_type_id))?;
        self.must_have_node_types().map(|node_types| {
            node_types.ids.par_iter().enumerate().filter_map(
                move |(node_id, node_type_ids)| unsafe {
                    if node_type_ids
                        .as_ref()
                        .map_or(false, |node_type_ids| node_type_ids.contains(&node_type_id))
                    {
                        Some(self.get_unchecked_node_name_from_node_id(node_id as NodeT))
                    } else {
                        None
                    }
                },
            )
        })
    }

    /// Returns parallel iterator over node names of the nodes with given node type name.
    ///
    /// # Argument
    /// * `node_type_name`: &str - The node type name to filter for.
    ///
    /// # Raises
    /// * If there are no node types in the graph.
    /// * If the given node type name does not exist in the current graph instance.
    pub fn par_iter_node_names_from_node_type_name(
        &self,
        node_type_name: &str,
    ) -> Result<impl ParallelIterator<Item = String> + '_> {
        self.par_iter_node_names_from_node_type_id(
            self.get_node_type_id_from_node_type_name(node_type_name)?,
        )
    }

    /// Returns iterator over node IDs of the nodes with known node types
    ///
    /// # Raises
    /// * If there are no node types in the graph.
    pub fn iter_node_ids_with_known_node_types(&self) -> Result<impl Iterator<Item = NodeT> + '_> {
        self.must_have_node_types().map(|node_types| {
            node_types
                .ids
                .iter()
                .enumerate()
                .filter_map(|(node_id, node_type_id)| {
                    if node_type_id.is_some() {
                        Some(node_id as NodeT)
                    } else {
                        None
                    }
                })
        })
    }

    /// Returns iterator over edge node names of the edges with unknown edge types
    ///
    /// # Arguments
    /// * `directed`: bool - Whether to load the edges as directed or undirected.
    ///
    /// # Raises
    /// * If there are no edge types in the graph.
    pub fn iter_edge_node_names_with_unknown_edge_types(
        &self,
        directed: bool,
    ) -> Result<impl Iterator<Item = (String, String)> + '_> {
        self.iter_edge_node_ids_with_unknown_edge_types(directed)
            .map(|x| {
                x.map(move |(src, dst)| unsafe {
                    (
                        self.get_unchecked_node_name_from_node_id(src),
                        self.get_unchecked_node_name_from_node_id(dst),
                    )
                })
            })
    }

    /// Returns iterator over edge node names of the edges with known edge types
    ///
    /// # Arguments
    /// * `directed`: bool - Whether to load the edges as directed or undirected.
    ///
    /// # Raises
    /// * If there are no edge types in the graph.
    pub fn iter_edge_node_names_with_known_edge_types(
        &self,
        directed: bool,
    ) -> Result<impl Iterator<Item = (String, String)> + '_> {
        self.iter_edge_node_ids_with_known_edge_types(directed)
            .map(|x| {
                x.map(move |(src, dst)| unsafe {
                    (
                        self.get_unchecked_node_name_from_node_id(src),
                        self.get_unchecked_node_name_from_node_id(dst),
                    )
                })
            })
    }

    /// Returns iterator over node names of the nodes with unknown node types
    ///
    /// # Raises
    /// * If there are no node types in the graph.
    pub fn iter_node_names_with_unknown_node_types(
        &self,
    ) -> Result<impl Iterator<Item = String> + '_> {
        self.iter_node_ids_with_unknown_node_types().map(|x| {
            x.map(move |node_id| unsafe { self.get_unchecked_node_name_from_node_id(node_id) })
        })
    }

    /// Returns iterator over node names of the nodes with known node types
    ///
    /// # Raises
    /// * If there are no node types in the graph.
    pub fn iter_node_names_with_known_node_types(
        &self,
    ) -> Result<impl Iterator<Item = String> + '_> {
        self.iter_node_ids_with_known_node_types().map(|x| {
            x.map(move |node_id| unsafe { self.get_unchecked_node_name_from_node_id(node_id) })
        })
    }

    /// Returns parallel iterator over edge IDs of the edges with unknown edge types
    ///
    /// # Raises
    /// * If there are no edge types in the graph.
    pub fn par_iter_edge_ids_with_unknown_edge_types(
        &self,
    ) -> Result<impl ParallelIterator<Item = EdgeT> + '_> {
        self.must_have_edge_types().map(|edge_types| {
            edge_types
                .ids
                .par_iter()
                .enumerate()
                .filter_map(|(edge_id, edge_type_id)| {
                    if edge_type_id.is_some() {
                        None
                    } else {
                        Some(edge_id as EdgeT)
                    }
                })
        })
    }

    /// Returns parallel iterator over edge IDs of the edges with known edge types
    ///
    /// # Raises
    /// * If there are no edge types in the graph.
    pub fn par_iter_edge_ids_with_known_edge_types(
        &self,
    ) -> Result<impl ParallelIterator<Item = EdgeT> + '_> {
        self.must_have_edge_types().map(|edge_types| {
            edge_types
                .ids
                .par_iter()
                .enumerate()
                .filter_map(|(edge_id, edge_type_id)| {
                    if edge_type_id.is_some() {
                        Some(edge_id as EdgeT)
                    } else {
                        None
                    }
                })
        })
    }

    /// Returns parallel iterator over edge node IDs of the edges with unknown edge types
    ///
    /// # Arguments
    /// * `directed`: bool - Whether to load the edges as directed or undirected.
    ///
    /// # Raises
    /// * If there arIndexedParallele no edge types in the graph.
    pub fn par_iter_edge_node_ids_with_unknown_edge_types(
        &self,
        directed: bool,
    ) -> Result<impl ParallelIterator<Item = (NodeT, NodeT)> + '_> {
        self.must_have_edge_types()?;
        Ok(self
            .par_iter_edge_node_ids_and_edge_type_id(directed)
            .filter_map(|(_, src, dst, edge_type)| {
                if edge_type.is_none() {
                    Some((src, dst))
                } else {
                    None
                }
            }))
    }

    /// Returns parallel iterator over edge node IDs of the edges with known edge types
    ///
    /// # Arguments
    /// * `directed`: bool - Whether to load the edges as directed or undirected.
    ///
    /// # Raises
    /// * If there arIndexedParallele no edge types in the graph.
    pub fn par_iter_edge_node_ids_with_known_edge_types(
        &self,
        directed: bool,
    ) -> Result<impl ParallelIterator<Item = (NodeT, NodeT)> + '_> {
        self.must_have_edge_types()?;
        Ok(self
            .par_iter_edge_node_ids_and_edge_type_id(directed)
            .filter_map(|(_, src, dst, edge_type)| {
                if edge_type.is_some() {
                    Some((src, dst))
                } else {
                    None
                }
            }))
    }

    /// Returns parallel iterator over node IDs of the nodes with unknown node types
    ///
    /// # Raises
    /// * If there are no node types in the graph.
    pub fn par_iter_node_ids_with_unknown_node_types(
        &self,
    ) -> Result<impl ParallelIterator<Item = NodeT> + '_> {
        self.must_have_node_types().map(|node_types| {
            node_types
                .ids
                .par_iter()
                .enumerate()
                .filter_map(|(node_id, node_type_id)| {
                    if node_type_id.is_some() {
                        None
                    } else {
                        Some(node_id as NodeT)
                    }
                })
        })
    }

    /// Returns parallel iterator over node IDs of the nodes with known node types
    ///
    /// # Raises
    /// * If there are no node types in the graph.
    pub fn par_iter_node_ids_with_known_node_types(
        &self,
    ) -> Result<impl ParallelIterator<Item = NodeT> + '_> {
        self.must_have_node_types().map(|node_types| {
            node_types
                .ids
                .par_iter()
                .enumerate()
                .filter_map(|(node_id, node_type_id)| {
                    if node_type_id.is_some() {
                        Some(node_id as NodeT)
                    } else {
                        None
                    }
                })
        })
    }

    /// Returns parallel iterator over edge node names of the edges with unknown edge types
    ///
    /// # Arguments
    /// * `directed`: bool - Whether to load the edges as directed or undirected.
    ///
    /// # Raises
    /// * If there are no edge types in the graph.
    pub fn par_iter_edge_node_names_with_unknown_edge_types(
        &self,
        directed: bool,
    ) -> Result<impl ParallelIterator<Item = (String, String)> + '_> {
        self.par_iter_edge_node_ids_with_unknown_edge_types(directed)
            .map(|x| {
                x.map(move |(src, dst)| unsafe {
                    (
                        self.get_unchecked_node_name_from_node_id(src),
                        self.get_unchecked_node_name_from_node_id(dst),
                    )
                })
            })
    }

    /// Returns parallel iterator over edge node names of the edges with known edge types
    ///
    /// # Arguments
    /// * `directed`: bool - Whether to load the edges as directed or undirected.
    ///
    /// # Raises
    /// * If there are no edge types in the graph.
    pub fn par_iter_edge_node_names_with_known_edge_types(
        &self,
        directed: bool,
    ) -> Result<impl ParallelIterator<Item = (String, String)> + '_> {
        self.par_iter_edge_node_ids_with_known_edge_types(directed)
            .map(|x| {
                x.map(move |(src, dst)| unsafe {
                    (
                        self.get_unchecked_node_name_from_node_id(src),
                        self.get_unchecked_node_name_from_node_id(dst),
                    )
                })
            })
    }

    /// Returns parallel iterator over node names of the nodes with unknown node types
    ///
    /// # Raises
    /// * If there are no node types in the graph.
    pub fn par_iter_node_names_with_unknown_node_types(
        &self,
    ) -> Result<impl ParallelIterator<Item = String> + '_> {
        self.par_iter_node_ids_with_unknown_node_types().map(|x| {
            x.map(move |node_id| unsafe { self.get_unchecked_node_name_from_node_id(node_id) })
        })
    }

    /// Returns parallel iterator over node names of the nodes with known node types
    ///
    /// # Raises
    /// * If there are no node types in the graph.
    pub fn par_iter_node_names_with_known_node_types(
        &self,
    ) -> Result<impl ParallelIterator<Item = String> + '_> {
        self.par_iter_node_ids_with_known_node_types().map(|x| {
            x.map(move |node_id| unsafe { self.get_unchecked_node_name_from_node_id(node_id) })
        })
    }
}
