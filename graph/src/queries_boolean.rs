use super::*;

/// # Boolean Queries
/// The naming convention for unchecked methods follows:
/// * `is_(.+?)_from_(.+)`
/// * `has_(.+?)_from_(.+)`
/// * `is_unchecked_(.+?)_from_(.+)`
/// * `has_unchecked_(.+?)_from_(.+)`
impl Graph {
    /// Returns boolean representing if given node is a singleton.
    ///
    /// # Arguments
    ///
    /// * `node_id`: NodeT - The node to be checked for.
    pub fn is_unchecked_singleton_from_node_id(&self, node_id: NodeT) -> bool {
        self.not_singleton_nodes
            .as_ref()
            .map_or(true, |nsns| !nsns[node_id as usize])
    }

    /// Returns boolean representing if given node is a singleton.
    ///
    /// # Arguments
    ///
    /// * `node_id`: NodeT - The node to be checked for.
    pub fn is_singleton_from_node_id(&self, node_id: NodeT) -> Result<bool, String> {
        self.validate_node_id(node_id)
            .map(|node_id| self.is_unchecked_singleton_from_node_id(node_id))
    }

    /// Returns boolean representing if given node is a singleton with self-loops.
    ///
    /// # Arguments
    ///
    /// * `node_id`: NodeT - The node to be checked for.
    pub fn is_singleton_with_selfloops_from_node_id(&self, node_id: NodeT) -> bool {
        self.singleton_nodes_with_selfloops
            .as_ref()
            .map_or(false, |snsls| snsls.contains(node_id))
    }

    /// Returns boolean representing if given node is a singleton.
    ///
    /// # Arguments
    /// `node_name`: &str - The node name to be checked for.
    pub fn is_singleton_from_node_name(&self, node_name: &str) -> Result<bool, String> {
        Ok(self.is_unchecked_singleton_from_node_id(self.get_node_id_from_node_name(node_name)?))
    }

    /// Returns whether the graph has the given node name.
    ///
    /// # Arguments
    ///
    /// * `node_name`: &str - Name of the node.
    ///
    /// # Examples
    /// To check if a node appears in the graph you can use:
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// let node_name = "ENSP00000000233";
    /// let unexistent_node_name = "I_do_not_exist!";
    /// assert!(graph.has_node_from_node_name(node_name));
    /// assert!(!graph.has_node_from_node_name(unexistent_node_name));
    /// ```
    pub fn has_node_from_node_name(&self, node_name: &str) -> bool {
        self.get_node_id_from_node_name(node_name).is_ok()
    }

    /// Returns whether edge passing between given node ids exists.
    ///
    /// # Arguments
    ///
    /// * `src`: NodeT - Source node id.
    /// * `dst`: NodeT - Destination node id.
    ///
    /// # Examples
    /// To check if an edge appears in the graph you can use:
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(false, true, true, true, false, false).unwrap();
    /// assert!(graph.has_edge_from_node_ids(0, 1));
    /// assert!(!graph.has_edge_from_node_ids(0, 4565));
    /// ```
    pub fn has_edge_from_node_ids(&self, src: NodeT, dst: NodeT) -> bool {
        self.get_edge_id_from_node_ids(src, dst).is_ok()
    }

    /// Returns whether edge with the given type passing between given nodes exists.
    ///
    /// # Arguments
    ///
    /// * `src`: NodeT - The source node of the edge.
    /// * `dst`: NodeT - The destination node of the edge.
    /// * `edge_type`: Option<EdgeTypeT> - The (optional) edge type.
    ///
    /// # Examples
    /// To check if an edge with given type appears in the graph you can use:
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(false, true, true, true, false, false).unwrap();
    /// assert!(graph.has_edge_from_node_ids_and_edge_type_id(0, 1, Some(0)));
    /// assert!(!graph.has_edge_from_node_ids_and_edge_type_id(0, 1, Some(1)));
    /// ```
    pub fn has_edge_from_node_ids_and_edge_type_id(
        &self,
        src: NodeT,
        dst: NodeT,
        edge_type: Option<EdgeTypeT>,
    ) -> bool {
        self.get_edge_id_from_node_ids_and_edge_type_id(src, dst, edge_type)
            .is_ok()
    }

    /// Returns boolean representing if given node is a trap.
    ///
    /// If the provided node_id is higher than the number of nodes in the graph,
    /// the method will panic.
    ///
    /// # Arguments
    ///
    /// * `node_id`: NodeT - Integer ID of the node, if this is bigger that the number of nodes it will panic.
    ///
    pub fn is_unchecked_trap_node_from_node_id(&self, node_id: NodeT) -> bool {
        self.get_unchecked_node_degree_from_node_id(node_id) == 0
            && self
                .not_singleton_nodes
                .as_ref()
                .map_or(true, |nsns| nsns[node_id as usize])
    }

    /// Returns boolean representing if given node is a trap.
    ///
    /// # Arguments
    ///
    /// * `node_id`: NodeT - Integer ID of the node, if this is bigger that the number of nodes it will panic.
    ///
    pub fn is_trap_node_from_node_id(&self, node_id: NodeT) -> Result<bool, String> {
        self.validate_node_id(node_id)
            .map(|node_id| self.is_unchecked_trap_node_from_node_id(node_id))
    }

    /// Returns whether the given node name and node type name exist in current graph.
    ///
    /// # Arguments
    ///
    /// * `node_name`: String - The node name.
    /// * `node_type_name`: String - The node type name.
    ///
    pub fn has_node_from_node_name_and_node_type_name(
        &self,
        node_name: &str,
        node_type_name: Option<Vec<String>>,
    ) -> bool {
        match self.get_node_id_from_node_name(node_name) {
            Err(_) => false,
            Ok(node_id) => {
                let our_node_types = self.get_node_type_name_from_node_id(node_id);
                match (our_node_types, node_type_name) {
                    (Err(_), None) => true,
                    (Ok(None), None) => true,
                    (Ok(Some(mut our_nts)), Some(mut other_nts)) => {
                        our_nts.sort();
                        other_nts.sort();
                        our_nts == other_nts
                    }
                    _ => false,
                }
            }
        }
    }

    /// Returns whether if edge passing between given nodes exists.
    ///
    /// # Arguments
    ///
    /// * `src`: String - The source node name of the edge.
    /// * `dst`: String - The destination node name of the edge.
    ///
    /// # Examples
    /// To check if an edge in the graph you can use:
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(false, true, true, true, false, false).unwrap();
    /// assert!(graph.has_edge_from_node_names("ENSP00000000233", "ENSP00000432568"));
    /// assert!(!graph.has_edge_from_node_names("ENSP00000000233", "NonExistent"));
    /// ```
    pub fn has_edge_from_node_names(&self, src_name: &str, dst_name: &str) -> bool {
        self.get_edge_id_from_node_names(src_name, dst_name).is_ok()
    }

    /// Returns whether if edge with type passing between given nodes exists.
    ///
    /// # Arguments
    ///
    /// * `src`: String - The source node name of the edge.
    /// * `dst`: String - The destination node name of the edge.
    /// * `edge_type`: Option<String> - The (optional) edge type name.
    ///
    /// # Examples
    /// To check if an edge with type in the graph you can use:
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(false, true, true, true, false, false).unwrap();
    /// let edge_type = "red".to_string();
    /// let unexistent_edge_type = "NonExistent".to_string();
    /// assert!(graph.has_edge_from_node_names_and_edge_type_name("ENSP00000000233", "ENSP00000432568", Some(&edge_type)));
    /// assert!(!graph.has_edge_from_node_names_and_edge_type_name("ENSP00000000233", "ENSP00000432568", Some(&unexistent_edge_type)));
    /// assert!(!graph.has_edge_from_node_names_and_edge_type_name("ENSP00000000233", "NonExistent", Some(&edge_type)));
    /// assert!(!graph.has_edge_from_node_names_and_edge_type_name("ENSP00000000233", "NonExistent", Some(&unexistent_edge_type)));
    /// ```
    pub fn has_edge_from_node_names_and_edge_type_name(
        &self,
        src_name: &str,
        dst_name: &str,
        edge_type_name: Option<&String>,
    ) -> bool {
        self.get_edge_id_from_node_names_and_edge_type_name(src_name, dst_name, edge_type_name)
            .is_ok()
    }
}
