use super::*;

/// # Boolean Queries
/// The naming convection for unchecked methods follows:
/// - `is_X_by_Y`
/// - `has_X_by_Y`
impl Graph {
    /// Returns boolean representing if given node is a singleton.
    ///
    /// The following works for traps and singletons.
    /// TODO: THIS IS SOMETHING TO BE GENERALIZED FOR DIRECTED GRAPHS.
    ///
    /// # Arguments
    ///
    /// `node_id`: NodeT - The node to be checked for.
    pub fn is_singleton_by_node_id(&self, node_id: NodeT) -> Result<bool, String> {
        Ok(self.has_singletons() && self.get_node_degree_by_node_id(node_id)? == 0)
    }

    /// Returns boolean representing if given node is a singleton with self-loops.
    ///
    /// # Arguments
    ///
    /// `node_id`: NodeT - The node to be checked for.
    pub fn is_singleton_with_self_loops_by_node_id(&self, node_id: NodeT) -> bool {
        self.has_singleton_nodes_with_self_loops_number()
            && self.iter_node_neighbours_ids(node_id).all(|dst| dst == node_id)
    }
    
    /// Returns boolean representing if given node is a singleton.
    ///
    /// # Arguments
    /// `node_name`: &str - The node name to be checked for.
    pub fn is_singleton_by_node_name(&self, node_name: &str) -> Result<bool, String> {
        self.is_singleton_by_node_id(self.get_node_id_by_node_name(node_name)?)
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
    /// assert!(graph.has_node_by_node_name(node_name));
    /// assert!(!graph.has_node_by_node_name(unexistent_node_name));
    /// ```
    pub fn has_node_by_node_name(&self, node_name: &str) -> bool {
        self.get_node_id_by_node_name(node_name).is_ok()
    }

    // TODO: add docstring and example!
    pub fn has_edge_by_node_ids(&self, src: NodeT, dst: NodeT) -> bool {
        self.get_edge_id_by_node_ids(src, dst).is_ok()
    }

    /// Returns boolean representing if edge passing between given nodes exists.
    ///
    /// # Arguments
    ///
    /// * src: NodeT - The source node of the edge.
    /// * dst: NodeT - The destination node of the edge.
    /// * edge_type: Option<EdgeTypeT> - The (optional) edge type.
    ///
    /// TODO: add example!
    pub fn has_edge_with_type_by_node_ids(&self, src: NodeT, dst: NodeT, edge_type: Option<EdgeTypeT>) -> bool {
        self.get_edge_id_with_type_by_node_ids(src, dst, edge_type).is_ok()
    }

    /// Returns boolean representing if given node is a trap.
    ///
    /// # Arguments
    ///
    /// * `node` - Integer ID of the node, if this is bigger that the number of nodes it will panic.
    ///
    pub fn is_node_trap_by_node_id(&self, node: NodeT) -> Result<bool, String> {
        Ok(self.get_node_degree_by_node_id(node)? == 0)
    }

    /// Returns whether the given node name and node type name exist in current graph.
    ///
    /// # Arguments
    ///
    /// * node_name: String - The node name.
    /// * node_type_name: String - The node type name.
    ///
    pub fn has_node_with_type_by_node_name(&self, node_name: &str, node_type_name: Option<Vec<String>>) -> bool {
        match self.get_node_id_by_node_name(node_name) {
            Err(_) => false,
            Ok(node_id) => {
                let our_node_types = self.get_node_type_name_by_node_id(node_id);
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

    /// Returns boolean representing if edge passing between given nodes exists.
    ///
    /// # Arguments
    ///
    /// * src: String - The source node name of the edge.
    /// * dst: String - The destination node name of the edge.
    /// * edge_type: Option<String> - The (optional) edge type name.
    ///
    pub fn has_edge_with_type_by_node_names(
        &self,
        src_name: &str,
        dst_name: &str,
        edge_type_name: Option<&String>,
    ) -> bool {
        self.get_edge_id_with_type_by_node_names(src_name, dst_name, edge_type_name)
            .is_ok()
    }

    // TODO: add docstring and example!
    pub fn has_edge_by_node_names(
        &self,
        src_name: &str,
        dst_name: &str
    ) -> bool {
        self.get_edge_id_by_node_names(src_name, dst_name).is_ok()
    }
}
