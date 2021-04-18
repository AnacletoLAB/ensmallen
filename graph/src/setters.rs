use super::*;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;

impl Graph {
    /// Set the name of the graph.
    ///
    /// # Arguments
    ///
    /// * `name`: String - Name of the graph.
    pub fn set_name(&mut self, name: String) {
        self.invalidate_report();
        self.name = name;
    }

    /// Invalidate the cache for the textual report.
    /// This should be called as the first line of every methods that either get
    /// a mutable reference to self or get ownership of self.
    pub(crate) fn invalidate_report(&self) {
        *self.cached_report.write() = None;
    }

    /// Replace all edge types (if present) and set all the edge to edge_type.
    ///
    /// # Arguments
    /// * `edge_type`: S - The edge type to assing to all the edges.
    pub fn set_all_edge_types<S: Into<String>>(mut self, edge_type: S) -> Result<Graph, String> {
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
                "This is a strongly undefined behaviour that can be first handled with ",
                "the remove method, that can let you remove edge types.\n",
                "Consider that when using the remove method, you will still collapse ",
                "the multigraph to an homogeneous graph, and it will keep the FIRST edge ",
                "of any group of multigraph edges between two given nodes."
            )
            .to_string()
        })?;
        self.invalidate_report();
        let mut vocabulary = Vocabulary::default();
        vocabulary.insert(edge_type.into())?;
        vocabulary.build_reverse_mapping()?;
        let edge_types = EdgeTypeVocabulary::from_structs(
            vec![Some(0); self.get_directed_edges_number() as usize],
            vocabulary,
        );
        self.edge_types = Some(edge_types);
        Ok(self)
    }

    /// Replace all node types (if present) and set all the node to node_type.
    ///
    /// # Arguments
    /// * `node_type`: S - The node type to assing to all the nodes.
    pub fn set_all_node_types<S: Into<String>>(mut self, node_type: S) -> Result<Graph, String> {
        self.must_have_nodes()?;
        self.invalidate_report();
        let mut vocabulary = Vocabulary::default();
        vocabulary.insert(node_type.into())?;
        vocabulary.build_reverse_mapping()?;
        let node_types = NodeTypeVocabulary::from_structs(
            vec![Some(vec![0]); self.get_nodes_number() as usize],
            Some(vocabulary),
        );
        self.node_types = node_types;
        Ok(self)
    }

    /// Remove given node type ID from all nodes.
    ///
    /// If any given node remains with no node type, that node is labeled
    /// with node type None. Note that the modification happens inplace.
    ///
    /// # Arguments
    /// * `node_type_id`: NodeTypeT - The node type ID to remove.
    ///
    /// # Raises
    /// * If the graph does not have node types.
    /// * If the given node type ID does not exists in the graph.
    ///
    pub fn remove_inplace_node_type_id(mut self, node_type_id: NodeTypeT) -> Result<Graph, String> {
        self.must_have_node_types()?;
        self.validate_node_type_id(Some(node_type_id))?;
        if let Some(node_types) = self.node_types.as_mut() {
            node_types
                .ids
                .par_iter_mut()
                .filter(|maybe_node_type_ids| maybe_node_type_ids.is_some())
                .for_each(|node_type_ids| {
                    if let Some(ntis) = node_type_ids.as_mut() {
                        // We remove the given node type if one was given.
                        if let Some(pos) = ntis.iter().position(|x| *x == node_type_id) {
                            ntis.remove(pos);
                        }
                    }
                    // If after we have removed the node type the node does not have any
                    // node type anymore, we replace its empty vector with a None.
                    if node_type_ids.as_ref().map_or(false, |ntis| ntis.is_empty()) {
                        *node_type_ids = None;
                    }
                });
            node_types.counts[node_type_id as usize] = 0;
        }
        Ok(self)
    }

    /// Remove given edge type ID from all edges.
    ///
    /// # Arguments
    /// * `edge_type_id`: EdgeTypeT - The edge type ID to remove.
    ///
    /// # Raises
    /// *
    /// * If the graph does not have edge types.
    /// * If the given edge type ID does not exists in the graph.
    ///
    /// TODO!: add support for removal of edge types in the context of multigraphs when the user asks for removing an edge type.
    pub fn remove_inplace_edge_type_id(mut self, edge_type_id: EdgeTypeT) -> Result<Graph, String> {
        self.must_have_edge_types()?;
        self.validate_edge_type_id(Some(edge_type_id))?;
        if let Some(edge_types) = self.edge_types.as_mut() {
            edge_types
                .ids
                .par_iter_mut()
                .for_each(|maybe_edge_type_id| {
                    if maybe_edge_type_id
                        .as_ref()
                        .map_or(false, |et| *et == edge_type_id)
                    {
                        *maybe_edge_type_id = None;
                    }
                });
            edge_types.counts[edge_type_id as usize] = 0;
        }
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
    pub fn remove_inplace_node_type_name(self, node_type_name: &str) -> Result<Graph, String> {
        let node_type_id = self.get_node_type_id_from_node_type_name(node_type_name)?;
        self.remove_inplace_node_type_id(node_type_id)
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
    pub fn remove_node_type_id(&self, node_type_id: NodeTypeT) -> Result<Graph, String> {
        self.clone().remove_inplace_node_type_id(node_type_id)
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
    pub fn remove_node_type_name(&self, node_type_name: &str) -> Result<Graph, String> {
        self.clone().remove_inplace_node_type_name(node_type_name)
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
    pub fn remove_inplace_edge_type_name(self, edge_type_name: &str) -> Result<Graph, String> {
        let edge_type_id = self.get_edge_type_id_from_edge_type_name(Some(edge_type_name))?.unwrap();
        self.remove_inplace_edge_type_id(edge_type_id)
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
    pub fn remove_edge_type_id(&self, edge_type_id: EdgeTypeT) -> Result<Graph, String> {
        self.clone().remove_inplace_edge_type_id(edge_type_id)
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
    pub fn remove_edge_type_name(&self, edge_type_name: &str) -> Result<Graph, String> {
        self.clone().remove_inplace_edge_type_name(edge_type_name)
    }
}
