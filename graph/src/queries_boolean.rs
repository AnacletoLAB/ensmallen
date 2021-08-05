use super::*;

/// # Boolean Queries
/// The naming convention for unchecked methods follows:
/// * `/is_(.+?)_from_(.+)/`
/// * `/has_(.+?)_from_(.+)/`
/// * `/is_(.+?)_from_(.+)_unchecked/`
/// * `/has_(.+?)_from_(.+)_unchecked/`
impl Graph {
    /// Returns boolean representing if given node is not a singleton nor a singleton with selfloop.
    ///
    /// # Arguments
    /// * `node_id`: NodeT - The node to be checked for.
    ///
    /// # Safety
    /// If the given node ID does not exists in the graph this method will panic.
    pub unsafe fn is_unchecked_connected_from_node_id(&self, node_id: NodeT) -> bool {
        self.connected_nodes
            .as_ref()
            .map_or(true, |connected_nodes| connected_nodes[node_id as usize])
    }

    /// Returns boolean representing if given node is a singleton or a singleton with selfloop.
    ///
    /// # Arguments
    /// * `node_id`: NodeT - The node to be checked for.
    ///
    /// # Safety
    /// If the given node ID does not exists in the graph this method will panic.
    pub unsafe fn is_unchecked_disconnected_node_from_node_id(&self, node_id: NodeT) -> bool {
        !self.is_unchecked_connected_from_node_id(node_id)
    }

    /// Returns boolean representing if given node is a singleton.
    ///
    /// # Arguments
    /// * `node_id`: NodeT - The node to be checked for.
    ///
    /// # Safety
    /// If the given node ID does not exists in the graph this method will panic.
    pub unsafe fn is_unchecked_singleton_from_node_id(&self, node_id: NodeT) -> bool {
        // First we check the node degree: if it has a node degree greater than
        // zero, surely this node cannot be a singleton node.
        if self.get_unchecked_node_degree_from_node_id(node_id) > 0 {
            return false;
        }
        // If this is a directed graph, we need to distinguish trap nodes from singleton nodes
        if self.is_directed() {
            self.is_unchecked_disconnected_node_from_node_id(node_id)
        } else {
            // Otherwise in an undirected graph the nodes with zero degree are only
            // singleton nodes.
            true
        }
    }

    /// Returns boolean representing if given node is a singleton.
    ///
    /// # Arguments
    /// * `node_id`: NodeT - The node to be checked for.
    pub fn is_singleton_from_node_id(&self, node_id: NodeT) -> Result<bool> {
        self.validate_node_id(node_id)
            .map(|node_id| unsafe { self.is_unchecked_singleton_from_node_id(node_id) })
    }

    /// Returns boolean representing if given node is a singleton with self-loops.
    ///
    /// # Arguments
    /// * `node_id`: NodeT - The node to be checked for.
    pub unsafe fn is_unchecked_singleton_with_selfloops_from_node_id(
        &self,
        node_id: NodeT,
    ) -> bool {
        self.is_unchecked_disconnected_node_from_node_id(node_id)
            && self.get_unchecked_node_degree_from_node_id(node_id) > 0
            && self
                .iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                .all(|dst| node_id == dst)
    }

    /// Returns boolean representing if given node is a singleton with self-loops.
    ///
    /// # Arguments
    /// * `node_id`: NodeT - The node to be checked for.
    pub fn is_singleton_with_selfloops_from_node_id(&self, node_id: NodeT) -> Result<bool> {
        self.validate_node_id(node_id).map(|node_id| unsafe {
            self.is_unchecked_singleton_with_selfloops_from_node_id(node_id)
        })
    }

    /// Returns boolean representing if given node is a singleton.
    ///
    /// Nota that this method will raise a panic if caled with unproper
    /// parametrization.
    ///
    /// # Arguments
    /// * `node_name`: &str - The node name to be checked for.
    ///
    /// # Safety
    /// If the given node name does not exist in the graph this method will panic.
    pub unsafe fn is_unchecked_singleton_from_node_name(&self, node_name: &str) -> bool {
        self.is_unchecked_singleton_from_node_id(
            self.get_unchecked_node_id_from_node_name(node_name),
        )
    }

    /// Returns boolean representing if given node is a singleton.
    ///
    /// # Arguments
    /// * `node_name`: &str - The node name to be checked for.
    pub fn is_singleton_from_node_name(&self, node_name: &str) -> Result<bool> {
        Ok(unsafe {
            self.is_unchecked_singleton_from_node_id(self.get_node_id_from_node_name(node_name)?)
        })
    }

    /// Returns whether the graph has the given node name.
    ///
    /// # Arguments
    ///
    /// * `node_name`: &str - Name of the node.
    ///
    /// # Example
    /// To check if a node appears in the graph you can use:
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// let node_name = "ENSP00000000233";
    /// let unexistent_node_name = "I_do_not_exist!";
    /// assert!(graph.has_node_name(node_name));
    /// assert!(!graph.has_node_name(unexistent_node_name));
    /// ```
    pub fn has_node_name(&self, node_name: &str) -> bool {
        self.get_node_id_from_node_name(node_name).is_ok()
    }

    /// Returns whether the graph has the given node type id.
    ///
    /// # Arguments
    ///
    /// * `node_type_id`: NodeTypeT - id of the node.
    ///
    /// # Example
    /// To check if a node appears in the graph you can use:
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// let node_type_id = 0;
    /// let unexistent_node_type_id = 34567;
    /// assert!(graph.has_node_type_id(node_type_id));
    /// assert!(!graph.has_node_type_id(unexistent_node_type_id));
    /// ```
    pub fn has_node_type_id(&self, node_type_id: NodeTypeT) -> bool {
        self.validate_node_type_id(Some(node_type_id)).is_ok()
    }

    /// Returns whether the graph has the given node type name.
    ///
    /// # Arguments
    ///
    /// * `node_type_name`: &str - Name of the node.
    ///
    /// # Example
    /// To check if a node appears in the graph you can use:
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// let node_type_name = "biolink:Gene";
    /// let unexistent_node_type_name = "I_do_not_exist!";
    /// assert!(graph.has_node_type_name(node_type_name));
    /// assert!(!graph.has_node_type_name(unexistent_node_type_name));
    /// ```
    pub fn has_node_type_name(&self, node_type_name: &str) -> bool {
        self.get_node_type_id_from_node_type_name(node_type_name)
            .is_ok()
    }

    /// Returns whether the graph has the given edge type id.
    ///
    /// # Arguments
    ///
    /// * `edge_type_id`: EdgeTypeT - id of the edge.
    ///
    /// # Example
    /// To check if a edge appears in the graph you can use:
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// let edge_type_id = 0;
    /// let unexistent_edge_type_id = 567;
    /// assert!(graph.has_edge_type_id(edge_type_id));
    /// assert!(!graph.has_edge_type_id(unexistent_edge_type_id));
    /// ```
    pub fn has_edge_type_id(&self, edge_type_id: EdgeTypeT) -> bool {
        self.validate_edge_type_id(Some(edge_type_id)).is_ok()
    }

    /// Returns whether the graph has the given edge type name.
    ///
    /// # Arguments
    ///
    /// * `edge_type_name`: &str - Name of the edge.
    ///
    /// # Example
    /// To check if a edge appears in the graph you can use:
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// let edge_type_name = "red";
    /// let unexistent_edge_type_name = "I_do_not_exist!";
    /// assert!(graph.has_edge_type_name(edge_type_name));
    /// assert!(!graph.has_edge_type_name(unexistent_edge_type_name));
    /// ```
    pub fn has_edge_type_name(&self, edge_type_name: &str) -> bool {
        self.get_edge_type_id_from_edge_type_name(Some(edge_type_name))
            .is_ok()
    }

    /// Returns whether edge passing between given node ids exists.
    ///
    /// # Arguments
    ///
    /// * `src`: NodeT - Source node id.
    /// * `dst`: NodeT - Destination node id.
    ///
    /// # Example
    /// To check if an edge appears in the graph you can use:
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(false, true, true, true, false, false);
    /// assert!(graph.has_edge_from_node_ids(0, 1));
    /// assert!(!graph.has_edge_from_node_ids(0, 4565));
    /// ```
    pub fn has_edge_from_node_ids(&self, src: NodeT, dst: NodeT) -> bool {
        self.get_edge_id_from_node_ids(src, dst).is_ok()
    }

    /// Returns whether the given node ID has a selfloop.
    ///
    /// # Arguments
    /// * `node_id`: NodeT - Source node id.
    ///
    /// # Example
    /// To check if a selfloop appears in the graph you can use:
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(false, true, true, true, false, false);
    /// assert!(graph.has_selfloop_from_node_id(0));
    /// assert!(!graph.has_selfloop_from_node_id(4565));
    /// ```
    pub fn has_selfloop_from_node_id(&self, node_id: NodeT) -> bool {
        self.has_edge_from_node_ids(node_id, node_id)
    }

    /// Returns whether edge with the given type passing between given nodes exists.
    ///
    /// # Arguments
    ///
    /// * `src`: NodeT - The source node of the edge.
    /// * `dst`: NodeT - The destination node of the edge.
    /// * `edge_type`: Option<EdgeTypeT> - The (optional) edge type.
    ///
    /// # Example
    /// To check if an edge with given type appears in the graph you can use:
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(false, true, true, true, false, false);
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
    /// # Safety
    /// If the given node ID does not exists in the graph this method will panic.
    pub unsafe fn is_unchecked_trap_node_from_node_id(&self, node_id: NodeT) -> bool {
        self.connected_nodes
            .as_ref()
            .map_or(!self.has_singleton_nodes(), |connected_nodes| {
                connected_nodes[node_id as usize]
            })
            && self.get_unchecked_node_degree_from_node_id(node_id) == 0
    }

    /// Returns boolean representing if given node is a trap.
    ///
    /// # Arguments
    ///
    /// * `node_id`: NodeT - Integer ID of the node, if this is bigger that the number of nodes it will panic.
    ///
    pub fn is_trap_node_from_node_id(&self, node_id: NodeT) -> Result<bool> {
        self.validate_node_id(node_id)
            .map(|node_id| unsafe { self.is_unchecked_trap_node_from_node_id(node_id) })
    }

    /// Returns whether the given node name and node type name exist in current graph.
    ///
    /// # Arguments
    ///
    /// * `node_name`: &str - The node name.
    /// * `node_type_name`: Option<Vec<String>> - The node types name.
    ///
    pub fn has_node_name_and_node_type_name(
        &self,
        node_name: &str,
        node_type_name: Option<Vec<String>>,
    ) -> bool {
        match self.get_node_id_from_node_name(node_name) {
            Err(_) => false,
            Ok(node_id) => {
                let our_node_types = self.get_node_type_names_from_node_id(node_id);
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
    /// * `src_name`: &str - The source node name of the edge.
    /// * `dst_name`: &str - The destination node name of the edge.
    ///
    /// # Example
    /// To check if an edge in the graph you can use:
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(false, true, true, true, false, false);
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
    /// * `src_name`: &str - The source node name of the edge.
    /// * `dst_name`: &str - The destination node name of the edge.
    /// * `edge_type_name`: Option<&str> - The (optional) edge type name.
    ///
    /// # Example
    /// To check if an edge with type in the graph you can use:
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(false, true, true, true, false, false);
    /// let edge_type = Some("red");
    /// let unexistent_edge_type = Some("NonExistent");
    /// assert!(graph.has_edge_from_node_names_and_edge_type_name("ENSP00000000233", "ENSP00000432568", edge_type));
    /// assert!(!graph.has_edge_from_node_names_and_edge_type_name("ENSP00000000233", "ENSP00000432568", unexistent_edge_type));
    /// assert!(!graph.has_edge_from_node_names_and_edge_type_name("ENSP00000000233", "NonExistent", edge_type));
    /// assert!(!graph.has_edge_from_node_names_and_edge_type_name("ENSP00000000233", "NonExistent", unexistent_edge_type));
    /// ```
    pub fn has_edge_from_node_names_and_edge_type_name(
        &self,
        src_name: &str,
        dst_name: &str,
        edge_type_name: Option<&str>,
    ) -> bool {
        self.get_edge_id_from_node_names_and_edge_type_name(src_name, dst_name, edge_type_name)
            .is_ok()
    }
}
