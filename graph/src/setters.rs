use super::*;
use itertools::Itertools;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};

impl Graph {
    /// Set the name of the graph.
    ///
    /// # Arguments
    ///
    /// * `name`: String - Name of the graph.
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Replace all edge types (if present) and set all the edge to edge_type.
    ///
    /// This happens INPLACE, that is edits the current graph instance.
    ///
    /// # Arguments
    /// * `edge_type`: S - The edge type to assing to all the edges.
    ///
    /// # Raises
    /// * If the graph does not have edges.
    /// * If the graph is a multigraph.
    pub fn set_inplace_all_edge_types<S: Into<String>>(&mut self, edge_type: S) -> Result<&Graph> {
        // If the graph does not have edges, it does not make sense to
        // try and set the edge types.
        self.must_have_edges()?;
        // Similarly, setting the edge types of a multigraph would make it
        // collapse to a homogeneous graph, and this operation is not supported
        // with the function set all edge types.
        self.must_not_be_multigraph().map_err(|_| {
            concat!(
                "The method set_all_edge_types does not support multigraphs because ",
                "setting the edge types of all edges to a single one in this type",
                "of graphs will cause a multigraph to collapse to an homogeneous ",
                "graph, leading to multiple undefined behaviours, such as loosing ",
                "the parallel edges that would collapse to one: which one should we keep?\n",
                "You can drop the parallell edges by calling the not INPLACE version ",
                "of this method.\n",
                "Consider that when using the remove method, you will still collapse ",
                "the multigraph to an homogeneous graph, and it will keep the FIRST edge ",
                "of any group of multigraph edges between two given nodes."
            )
            .to_string()
        })?;
        let vocabulary = Vocabulary::from_reverse_map(vec![edge_type.into()])?;
        let edge_types = EdgeTypeVocabulary::from_structs(
            vec![Some(0); self.get_directed_edges_number() as usize],
            vocabulary,
        );
        self.edge_types = Some(edge_types);
        Ok(self)
    }

    /// Replace all edge types (if present) and set all the edge to edge_type.
    ///
    /// This DOES NOT happen inplace, but created a new instance of the graph.
    ///
    /// # Arguments
    /// * `edge_type`: S - The edge type to assing to all the edges.
    pub fn set_all_edge_types<S: Into<String>>(&self, edge_type: S) -> Result<Graph> {
        let mut graph = self.drop_parallel_edges();
        graph.set_inplace_all_edge_types(edge_type)?;
        Ok(graph)
    }

    /// Replace all node types (if present) and set all the node to node_type.
    ///
    /// # Arguments
    /// * `node_type`: S - The node type to assing to all the nodes.
    pub fn set_inplace_all_node_types<S: Into<String>>(&mut self, node_type: S) -> Result<&Graph> {
        self.must_have_nodes()?;
        let vocabulary = Vocabulary::from_reverse_map(vec![node_type.into()])?;
        let node_types = NodeTypeVocabulary::from_structs(
            vec![Some(vec![0]); self.get_nodes_number() as usize],
            vocabulary,
        );
        self.node_types = Some(node_types);
        Ok(self)
    }

    /// Replace all node types (if present) and set all the node to node_type.
    ///
    /// This DOES NOT happen inplace, but created a new instance of the graph.
    ///
    /// # Arguments
    /// * `node_type`: S - The node type to assing to all the nodes.
    pub fn set_all_node_types<S: Into<String>>(&self, node_type: S) -> Result<Graph> {
        let mut graph = self.clone();
        graph.set_inplace_all_node_types(node_type)?;
        Ok(graph)
    }

    /// Remove given node type ID from all nodes.
    ///
    /// If any given node remains with no node type, that node is labeled
    /// with node type None. Note that the modification happens inplace.
    ///
    /// # Arguments
    /// * `node_type_id_to_remove`: NodeTypeT - The node type ID to remove.
    ///
    /// # Raises
    /// * If the graph does not have node types.
    /// * If the given node type ID does not exists in the graph.
    ///
    pub fn remove_inplace_node_type_ids(
        &mut self,
        node_type_ids_to_remove: Vec<NodeTypeT>,
    ) -> Result<&Graph> {
        self.must_have_node_types()?;

        // if the user passed no values, we won't modify the graph so we can
        // return ealry
        if node_type_ids_to_remove.is_empty() {
            return Ok(self);
        }
        // check that the values are in the range of node type ids
        self.validate_node_type_id(node_type_ids_to_remove.iter().max().cloned())?;

        // if there are duplicated nodes it's probably an error
        if node_type_ids_to_remove.len() != node_type_ids_to_remove.iter().unique().count() {
            return Err(
                "In the given vector of node type ids to remove there are duplicated values."
                    .to_string(),
            );
        }

        if let Some(node_types) = self.node_types.as_mut() {
            // compute the new node ids once the given ones are removed
            // we need this to keep a dense mapping.
            let new_node_type_ids =
                unsafe { node_types.unchecked_remove_values(node_type_ids_to_remove.clone()) };

            // Counter of how many new nodes have unknown type (aka how many nodes we removed)
            let new_unknown_nodes = AtomicU32::new(0);

            // Iter over each node and update its node
            node_types.ids.par_iter_mut().for_each(|node_type_ids| {
                if let Some(ntis) = node_type_ids.as_mut() {
                    // We remove the given node type if one was given.
                    for node_type_id_to_remove in &node_type_ids_to_remove {
                        if let Some(pos) = ntis.iter().position(|x| *x == *node_type_id_to_remove) {
                            ntis.remove(pos);
                        }
                    }

                    // node type anymore, we replace its empty vector with a None.
                    if ntis.is_empty() {
                        *node_type_ids = None;
                        new_unknown_nodes.fetch_add(1, Ordering::SeqCst);
                        return;
                    }

                    // densify the mapping
                    ntis.iter_mut().for_each(|node_type_id| {
                        if let Some(idx) = new_node_type_ids[*node_type_id as usize] {
                            *node_type_id = idx as NodeTypeT;
                        } else {
                            unreachable!("This should not happen");
                        }
                    }); // If after we have removed the node type the node does not have any
                }
            });

            node_types.unknown_count += new_unknown_nodes.load(Ordering::SeqCst);
        }
        Ok(self)
    }

    /// Remove singleton node types from all nodes.
    ///
    /// If any given node remains with no node type, that node is labeled
    /// with node type None. Note that the modification happens inplace.
    ///
    /// # Raises
    /// * If the graph does not have node types.
    ///
    pub fn remove_inplace_singleton_node_types(&mut self) -> Result<&mut Graph> {
        self.remove_inplace_node_type_ids(self.get_singleton_node_type_ids()?)?;
        Ok(self)
    }

    /// Remove given edge type ID from all edges.
    ///
    /// # Arguments
    /// * `edge_type_id`: EdgeTypeT - The edge type ID to remove.
    ///
    /// # Raises
    /// * If the graph is a multigraph.
    /// * If the graph does not have edge types.
    /// * If the given edge type ID does not exists in the graph.
    ///
    pub fn remove_inplace_edge_type_ids(
        &mut self,
        edge_type_ids_to_remove: Vec<EdgeTypeT>,
    ) -> Result<&mut Graph> {
        self.must_have_edge_types()?;

        self.must_not_be_multigraph().map_err(|_| {
            concat!(
                "The method remove_edge_type_id does not support multigraphs because ",
                "setting the edge types of all edges to a single one in this type",
                "of graphs will cause a multigraph to collapse to an homogeneous ",
                "graph, leading to multiple undefined behaviours, such as loosing ",
                "the parallel edges that would collapse to one: which one should we keep?\n",
                "You can drop the parallell edges by calling the not INPLACE version ",
                "of this method.\n",
                "Consider that when using the remove method, you will still collapse ",
                "the multigraph to an homogeneous graph, and it will keep the FIRST edge ",
                "of any group of multigraph edges between two given nodes."
            )
            .to_string()
        })?;

        // if the user passed no values, we won't modify the graph so we can
        // return ealry
        if edge_type_ids_to_remove.is_empty() {
            return Ok(self);
        }

        // check that the values are in the range of edge type ids
        self.validate_edge_type_id(edge_type_ids_to_remove.iter().max().cloned())?;

        // if there are duplicated edges it's probably an error
        if edge_type_ids_to_remove.len() != edge_type_ids_to_remove.iter().unique().count() {
            return Err(
                "In the given vector of edge type ids to remove there are duplicated values."
                    .to_string(),
            );
        }

        if let Some(edge_types) = self.edge_types.as_mut() {
            // compute the new edge ids once the given ones are removed
            // we need this to keep a dense mapping.
            let new_edge_type_ids =
                unsafe { edge_types.unchecked_remove_values(edge_type_ids_to_remove) };

            let new_unknown_edges = AtomicU64::new(0);
            edge_types
                .ids
                .par_iter_mut()
                .for_each(|maybe_edge_type_id| {
                    *maybe_edge_type_id = maybe_edge_type_id.and_then(|x| {
                        new_edge_type_ids[x as usize].map_or_else(
                            || {
                                new_unknown_edges.fetch_add(1, Ordering::SeqCst);
                                None
                            },
                            |x| Some(x as EdgeTypeT),
                        )
                    });
                });
            edge_types.unknown_count += new_unknown_edges.load(Ordering::SeqCst);
        }
        Ok(self)
    }

    /// Remove singleton edge types from all edges.
    ///
    /// If any given edge remains with no edge type, that edge is labeled
    /// with edge type None. Note that the modification happens inplace.
    ///
    /// # Raises
    /// * If the graph does not have edge types.
    ///
    pub fn remove_inplace_singleton_edge_types(&mut self) -> Result<&mut Graph> {
        self.remove_inplace_edge_type_ids(self.get_singleton_edge_type_ids()?)?;
        Ok(self)
    }

    /// Remove given node type name from all nodes.
    ///
    /// If any given node remains with no node type, that node is labeled
    /// with node type None. Note that the modification happens inplace.
    ///
    /// # Arguments
    /// * `node_type_name`: &str - The node type ID to remove.
    ///
    /// # Raises
    /// * If the graph does not have node types.
    /// * If the given node type name does not exists in the graph.
    ///
    pub fn remove_inplace_node_type_name(&mut self, node_type_name: &str) -> Result<&Graph> {
        let node_type_id = self.get_node_type_id_from_node_type_name(node_type_name)?;
        self.remove_inplace_node_type_ids(vec![node_type_id])?;
        Ok(self)
    }

    /// Remove given node type ID from all nodes.
    ///
    /// If any given node remains with no node type, that node is labeled
    /// with node type None. Note that the modification DOES NOT happen inplace.
    ///
    /// # Arguments
    /// * `node_type_id`: NodeTypeT - The node type ID to remove.
    ///
    /// # Raises
    /// * If the graph does not have node types.
    /// * If the given node type ID does not exists in the graph.
    ///
    pub fn remove_node_type_id(&self, node_type_id: NodeTypeT) -> Result<Graph> {
        let mut graph = self.clone();
        graph.remove_inplace_node_type_ids(vec![node_type_id])?;
        Ok(graph)
    }

    /// Remove singleton node types from all nodes.
    ///
    /// If any given node remains with no node type, that node is labeled
    /// with node type None. Note that the modification DOES NOT happen inplace.
    ///
    /// # Raises
    /// * If the graph does not have node types.
    ///
    pub fn remove_singleton_node_types(&self) -> Result<Graph> {
        let mut graph = self.clone();
        graph.remove_inplace_singleton_node_types()?;
        Ok(graph)
    }

    /// Remove given node type name from all nodes.
    ///
    /// If any given node remains with no node type, that node is labeled
    /// with node type None. Note that the modification DOES NOT happen inplace.
    ///
    /// # Arguments
    /// * `node_type_name`: &str - The node type ID to remove.
    ///
    /// # Raises
    /// * If the graph does not have node types.
    /// * If the given node type name does not exists in the graph.
    ///
    pub fn remove_node_type_name(&self, node_type_name: &str) -> Result<Graph> {
        let mut graph = self.clone();
        graph.remove_inplace_node_type_name(node_type_name)?;
        Ok(graph)
    }

    /// Remove given edge type name from all edges.
    ///
    /// If any given edge remains with no edge type, that edge is labeled
    /// with edge type None. Note that the modification happens inplace.
    ///
    /// # Arguments
    /// * `edge_type_name`: &str - The edge type ID to remove.
    ///
    /// # Raises
    /// * If the graph does not have edge types.
    /// * If the given edge type name does not exists in the graph.
    ///
    pub fn remove_inplace_edge_type_name(&mut self, edge_type_name: &str) -> Result<&mut Graph> {
        let edge_type_id = self
            .get_edge_type_id_from_edge_type_name(Some(edge_type_name))?
            .unwrap();
        self.remove_inplace_edge_type_ids(vec![edge_type_id])
    }

    /// Remove given edge type ID from all edges.
    ///
    /// If any given edge remains with no edge type, that edge is labeled
    /// with edge type None. Note that the modification DOES NOT happen inplace.
    ///
    /// # Arguments
    /// * `edge_type_id`: EdgeTypeT - The edge type ID to remove.
    ///
    /// # Raises
    /// * If the graph does not have edge types.
    /// * If the given edge type ID does not exists in the graph.
    ///
    pub fn remove_edge_type_id(&self, edge_type_id: EdgeTypeT) -> Result<Graph> {
        let mut graph = self.clone();
        graph.remove_inplace_edge_type_ids(vec![edge_type_id])?;
        Ok(graph)
    }

    /// Remove singleton edge types from all edges.
    ///
    /// If any given edge remains with no edge type, that edge is labeled
    /// with edge type None. Note that the modification DOES NOT happen inplace.
    ///
    /// # Raises
    /// * If the graph does not have edge types.
    ///
    pub fn remove_singleton_edge_types(&self) -> Result<Graph> {
        let mut graph = self.clone();
        graph.remove_inplace_singleton_edge_types()?;
        Ok(graph)
    }

    /// Remove given edge type name from all edges.
    ///
    /// If any given edge remains with no edge type, that edge is labeled
    /// with edge type None. Note that the modification DOES NOT happen inplace.
    ///
    /// # Arguments
    /// * `edge_type_name`: &str - The edge type ID to remove.
    ///
    /// # Raises
    /// * If the graph does not have edge types.
    /// * If the given edge type name does not exists in the graph.
    ///
    pub fn remove_edge_type_name(&self, edge_type_name: &str) -> Result<Graph> {
        let mut graph = self.clone();
        graph.remove_inplace_edge_type_name(edge_type_name)?;
        Ok(graph)
    }

    /// Remove node types from the graph.
    ///
    /// Note that the modification happens inplace.
    ///
    /// # Raises
    /// * If the graph does not have node types.
    ///
    pub fn remove_inplace_node_types(&mut self) -> Result<&Graph> {
        self.must_have_node_types()?;
        self.node_types = None;
        Ok(self)
    }

    /// Remove node types from the graph.
    ///
    /// Note that the modification does not happen inplace.
    ///
    /// # Raises
    /// * If the graph does not have node types.
    ///
    pub fn remove_node_types(&self) -> Result<Graph> {
        let mut graph = self.clone();
        graph.remove_inplace_node_types()?;
        Ok(graph)
    }

    /// Remove edge types from the graph.
    ///
    /// Note that the modification happens inplace.
    ///
    /// # Raises
    /// * If the graph does not have edge types.
    /// * If the graph is a multigraph.
    ///
    pub fn remove_inplace_edge_types(&mut self) -> Result<&Graph> {
        self.must_have_edge_types()?;
        self.must_not_be_multigraph()?;
        self.edge_types = None;
        Ok(self)
    }

    /// Remove edge types from the graph.
    ///
    /// Note that the modification does not happen inplace.
    ///
    /// # Raises
    /// * If the graph does not have edge types.
    ///
    pub fn remove_edge_types(&self) -> Result<Graph> {
        let mut graph = self.drop_parallel_edges();
        assert!(!graph.is_multigraph());
        graph.remove_inplace_edge_types()?;
        Ok(graph)
    }

    /// Remove edge weights from the graph.
    ///
    /// Note that the modification happens inplace.
    ///
    /// # Raises
    /// * If the graph does not have edge weights.
    ///
    pub fn remove_inplace_edge_weights(&mut self) -> Result<&Graph> {
        self.must_have_edge_weights()?;
        self.weights = None;
        Ok(self)
    }

    /// Remove edge weights from the graph.
    ///
    /// Note that the modification does not happen inplace.
    ///
    /// # Raises
    /// * If the graph does not have edge weights.
    ///
    pub fn remove_edge_weights(&self) -> Result<Graph> {
        let mut graph = self.clone();
        graph.remove_inplace_edge_weights()?;
        Ok(graph)
    }
}
