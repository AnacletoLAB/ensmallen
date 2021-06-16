use super::*;

#[pymethods]
impl EnsmallenGraph {
    #[automatically_generated_binding]
    #[text_signature = "($self, verbose)"]
    /// Returns unweighted laplacian transformation of the graph.
    ///
    /// Parameters
    /// ----------
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar while building the graph.
    ///
    pub fn get_laplacian_transformed_graph(&self, verbose: Option<bool>) -> EnsmallenGraph {
        EnsmallenGraph {
            graph: self.graph.get_laplacian_transformed_graph(verbose),
        }
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, verbose)"]
    /// Returns unweighted random walk normalized laplacian transformation of the graph.
    ///
    /// Parameters
    /// ----------
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar while building the graph.
    ///
    pub fn get_random_walk_normalized_laplacian_transformed_graph(
        &self,
        verbose: Option<bool>,
    ) -> EnsmallenGraph {
        EnsmallenGraph {
            graph: self
                .graph
                .get_random_walk_normalized_laplacian_transformed_graph(verbose),
        }
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, verbose)"]
    /// Returns unweighted symmetric normalized laplacian transformation of the graph.
    ///
    /// Parameters
    /// ----------
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar while building the graph.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     The graph must be undirected, as we do not currently support this transformation for directed graphs.
    ///
    pub fn get_symmetric_normalized_laplacian_transformed_graph(
        &self,
        verbose: Option<bool>,
    ) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(self
                .graph
                .get_symmetric_normalized_laplacian_transformed_graph(verbose))?,
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, verbose)"]
    /// Returns unweighted symmetric normalized transformation of the graph.
    ///
    /// Parameters
    /// ----------
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar while building the graph.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     The graph must be undirected, as we do not currently support this transformation for directed graphs.
    ///
    pub fn get_symmetric_normalized_transformed_graph(
        &self,
        verbose: Option<bool>,
    ) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(self
                .graph
                .get_symmetric_normalized_transformed_graph(verbose))?,
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, verbose)"]
    /// Returns weighted laplacian transformation of the graph.
    ///
    /// Parameters
    /// ----------
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar while building the graph.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph is not weighted it is not possible to compute the weighted laplacian transformation.
    ///
    pub fn get_weighted_laplacian_transformed_graph(
        &self,
        verbose: Option<bool>,
    ) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(self.graph.get_weighted_laplacian_transformed_graph(verbose))?,
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, verbose)"]
    /// Returns unweighted symmetric normalized laplacian transformation of the graph.
    ///
    /// Parameters
    /// ----------
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar while building the graph.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     The graph must be undirected, as we do not currently support this transformation for directed graphs.
    /// ValueError
    ///     If the graph is not weighted it is not possible to compute the weighted laplacian transformation.
    ///
    pub fn get_weighted_symmetric_normalized_laplacian_transformed_graph(
        &self,
        verbose: Option<bool>,
    ) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(self
                .graph
                .get_weighted_symmetric_normalized_laplacian_transformed_graph(verbose))?,
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, verbose)"]
    /// Returns weighted symmetric normalized transformation of the graph.
    ///
    /// Parameters
    /// ----------
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar while building the graph.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     The graph must be undirected, as we do not currently support this transformation for directed graphs.
    /// ValueError
    ///     If the graph is not weighted it is not possible to compute the weighted laplacian transformation.
    ///
    pub fn get_weighted_symmetric_normalized_transformed_graph(
        &self,
        verbose: Option<bool>,
    ) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(self
                .graph
                .get_weighted_symmetric_normalized_transformed_graph(verbose))?,
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, verbose)"]
    /// Returns weighted random walk normalized laplacian transformation of the graph.
    ///
    /// Parameters
    /// ----------
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar while building the graph.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph is not weighted it is not possible to compute the weighted laplacian transformation.
    /// ValueError
    ///     If the graph contains nodes with zero weighted degree.
    ///
    pub fn get_weighted_random_walk_normalized_laplacian_transformed_graph(
        &self,
        verbose: Option<bool>,
    ) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(self
                .graph
                .get_weighted_random_walk_normalized_laplacian_transformed_graph(verbose))?,
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_id)"]
    /// Returns boolean representing if given node is a singleton.
    ///
    /// Parameters
    /// ----------
    /// node_id: int,
    ///     The node to be checked for.
    ///
    ///
    /// Safety
    /// ------
    /// If the given node ID does not exists in the graph this method will panic.
    pub unsafe fn is_unchecked_singleton_from_node_id(&self, node_id: NodeT) -> bool {
        self.graph.is_unchecked_singleton_from_node_id(node_id)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_id)"]
    /// Returns boolean representing if given node is a singleton or a singleton with selfloop.
    ///
    /// Parameters
    /// ----------
    /// node_id: int,
    ///     The node to be checked for.
    ///
    ///
    /// Safety
    /// ------
    /// If the given node ID does not exists in the graph this method will panic.
    pub unsafe fn is_unchecked_disconnected_from_node_id(&self, node_id: NodeT) -> bool {
        self.graph.is_unchecked_disconnected_from_node_id(node_id)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_id)"]
    /// Returns boolean representing if given node is not a singleton nor a singleton with selfloop.
    ///
    /// Parameters
    /// ----------
    /// node_id: int,
    ///     The node to be checked for.
    ///
    ///
    /// Safety
    /// ------
    /// If the given node ID does not exists in the graph this method will panic.
    pub unsafe fn is_unchecked_connected_from_node_id(&self, node_id: NodeT) -> bool {
        self.graph.is_unchecked_connected_from_node_id(node_id)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_id)"]
    /// Returns boolean representing if given node is a singleton.
    ///
    /// Parameters
    /// ----------
    /// node_id: int,
    ///     The node to be checked for.
    ///
    pub fn is_singleton_from_node_id(&self, node_id: NodeT) -> PyResult<bool> {
        pe!(self.graph.is_singleton_from_node_id(node_id))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_id)"]
    /// Returns boolean representing if given node is a singleton with self-loops.
    ///
    /// Parameters
    /// ----------
    /// node_id: int,
    ///     The node to be checked for.
    ///
    pub fn is_singleton_with_selfloops_from_node_id(&self, node_id: NodeT) -> bool {
        self.graph.is_singleton_with_selfloops_from_node_id(node_id)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_name)"]
    /// Returns boolean representing if given node is a singleton.
    ///
    /// Nota that this method will raise a panic if caled with unproper
    /// parametrization.
    ///
    /// Parameters
    /// ----------
    /// node_name: str,
    ///     The node name to be checked for.
    ///
    ///
    /// Safety
    /// ------
    /// If the given node name does not exist in the graph this method will panic.
    pub unsafe fn is_unchecked_singleton_from_node_name(&self, node_name: &str) -> bool {
        self.graph.is_unchecked_singleton_from_node_name(node_name)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_name)"]
    /// Returns boolean representing if given node is a singleton.
    ///
    /// Parameters
    /// ----------
    /// node_name: str,
    ///     The node name to be checked for.
    ///
    pub fn is_singleton_from_node_name(&self, node_name: &str) -> PyResult<bool> {
        pe!(self.graph.is_singleton_from_node_name(node_name))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_name)"]
    /// Returns whether the graph has the given node name.
    ///
    /// Parameters
    /// ----------
    /// node_name: str,
    ///     Name of the node.
    ///
    pub fn has_node_name(&self, node_name: &str) -> bool {
        self.graph.has_node_name(node_name)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_type_id)"]
    /// Returns whether the graph has the given node type id.
    ///
    /// Parameters
    /// ----------
    /// node_type_id: int,
    ///     id of the node.
    ///
    pub fn has_node_type_id(&self, node_type_id: NodeTypeT) -> bool {
        self.graph.has_node_type_id(node_type_id)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_type_name)"]
    /// Returns whether the graph has the given node type name.
    ///
    /// Parameters
    /// ----------
    /// node_type_name: str,
    ///     Name of the node.
    ///
    pub fn has_node_type_name(&self, node_type_name: &str) -> bool {
        self.graph.has_node_type_name(node_type_name)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_type_id)"]
    /// Returns whether the graph has the given edge type id.
    ///
    /// Parameters
    /// ----------
    /// edge_type_id: int,
    ///     id of the edge.
    ///
    pub fn has_edge_type_id(&self, edge_type_id: EdgeTypeT) -> bool {
        self.graph.has_edge_type_id(edge_type_id)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_type_name)"]
    /// Returns whether the graph has the given edge type name.
    ///
    /// Parameters
    /// ----------
    /// edge_type_name: str,
    ///     Name of the edge.
    ///
    pub fn has_edge_type_name(&self, edge_type_name: &str) -> bool {
        self.graph.has_edge_type_name(edge_type_name)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src, dst)"]
    /// Returns whether edge passing between given node ids exists.
    ///
    /// Parameters
    /// ----------
    /// src: int,
    ///     Source node id.
    /// dst: int,
    ///     Destination node id.
    ///
    pub fn has_edge_from_node_ids(&self, src: NodeT, dst: NodeT) -> bool {
        self.graph.has_edge_from_node_ids(src, dst)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_id)"]
    /// Returns whether the given node ID has a selfloop.
    ///
    /// Parameters
    /// ----------
    /// node_id: int,
    ///     Source node id.
    ///
    pub fn has_selfloop_from_node_id(&self, node_id: NodeT) -> bool {
        self.graph.has_selfloop_from_node_id(node_id)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src, dst, edge_type)"]
    /// Returns whether edge with the given type passing between given nodes exists.
    ///
    /// Parameters
    /// ----------
    /// src: int,
    ///     The source node of the edge.
    /// dst: int,
    ///     The destination node of the edge.
    /// edge_type: Optional[int],
    ///     The (optional) edge type.
    ///
    pub fn has_edge_from_node_ids_and_edge_type_id(
        &self,
        src: NodeT,
        dst: NodeT,
        edge_type: Option<EdgeTypeT>,
    ) -> bool {
        self.graph
            .has_edge_from_node_ids_and_edge_type_id(src, dst, edge_type)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_id)"]
    /// Returns boolean representing if given node is a trap.
    ///
    /// If the provided node_id is higher than the number of nodes in the graph,
    /// the method will panic.
    ///
    /// Parameters
    /// ----------
    /// node_id: int,
    ///     Integer ID of the node, if this is bigger that the number of nodes it will panic.
    ///
    ///
    /// Safety
    /// ------
    /// If the given node ID does not exists in the graph this method will panic.
    pub unsafe fn is_unchecked_trap_node_from_node_id(&self, node_id: NodeT) -> bool {
        self.graph.is_unchecked_trap_node_from_node_id(node_id)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_id)"]
    /// Returns boolean representing if given node is a trap.
    ///
    /// Parameters
    /// ----------
    /// node_id: int,
    ///     Integer ID of the node, if this is bigger that the number of nodes it will panic.
    ///
    pub fn is_trap_node_from_node_id(&self, node_id: NodeT) -> PyResult<bool> {
        pe!(self.graph.is_trap_node_from_node_id(node_id))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_name, node_type_name)"]
    /// Returns whether the given node name and node type name exist in current graph.
    ///
    /// Parameters
    /// ----------
    /// node_name: str,
    ///     The node name.
    /// node_type_name: Optional[List[str]],
    ///     The node types name.
    ///
    pub fn has_node_name_and_node_type_name(
        &self,
        node_name: &str,
        node_type_name: Option<Vec<String>>,
    ) -> bool {
        self.graph
            .has_node_name_and_node_type_name(node_name, node_type_name)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src_name, dst_name)"]
    /// Returns whether if edge passing between given nodes exists.
    ///
    /// Parameters
    /// ----------
    /// src_name: str,
    ///     The source node name of the edge.
    /// dst_name: str,
    ///     The destination node name of the edge.
    ///
    pub fn has_edge_from_node_names(&self, src_name: &str, dst_name: &str) -> bool {
        self.graph.has_edge_from_node_names(src_name, dst_name)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src_name, dst_name, edge_type_name)"]
    /// Returns whether if edge with type passing between given nodes exists.
    ///
    /// Parameters
    /// ----------
    /// src_name: str,
    ///     The source node name of the edge.
    /// dst_name: str,
    ///     The destination node name of the edge.
    /// edge_type_name: Optional[str],
    ///     The (optional) edge type name.
    ///
    pub fn has_edge_from_node_names_and_edge_type_name(
        &self,
        src_name: &str,
        dst_name: &str,
        edge_type_name: Option<&str>,
    ) -> bool {
        self.graph
            .has_edge_from_node_names_and_edge_type_name(src_name, dst_name, edge_type_name)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns list of nodes of the various strongly connected components.
    ///
    /// This is an implementation of Tarjan algorithm.
    pub fn strongly_connected_components(&self) -> Vec<HashSet<NodeT>> {
        self.graph.strongly_connected_components()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, verbose)"]
    /// Returns graph with node IDs sorted by increasing outbound node degree.
    ///
    /// Parameters
    /// ----------
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar while building the graph.
    ///
    pub fn sort_by_increasing_outbound_node_degree(&self, verbose: Option<bool>) -> EnsmallenGraph {
        EnsmallenGraph {
            graph: self.graph.sort_by_increasing_outbound_node_degree(verbose),
        }
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, verbose)"]
    /// Returns graph with node IDs sorted by decreasing outbound node degree.
    ///
    /// Parameters
    /// ----------
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar while building the graph.
    ///
    pub fn sort_by_decreasing_outbound_node_degree(&self, verbose: Option<bool>) -> EnsmallenGraph {
        EnsmallenGraph {
            graph: self.graph.sort_by_decreasing_outbound_node_degree(verbose),
        }
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns binary dense adjacency matrix.
    ///
    /// Beware of using this method on big graphs!
    /// It'll use all of your RAM!
    pub fn get_dense_binary_adjacency_matrix(&self) -> Py<PyArray2<bool>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_2d!(gil, self.graph.get_dense_binary_adjacency_matrix(), bool)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, weight)"]
    /// Returns binary weighted adjacency matrix.
    ///
    /// Beware of using this method on big graphs!
    /// It'll use all of your RAM!
    ///
    /// Parameters
    /// ----------
    /// weight: Optional[float],
    ///     The weight value to use for absent edges. By default, `0.0`.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge weights.
    ///
    pub fn get_dense_weighted_adjacency_matrix(
        &self,
        weight: Option<WeightT>,
    ) -> PyResult<Py<PyArray2<WeightT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_2d!(
            gil,
            pe!(self.graph.get_dense_weighted_adjacency_matrix(weight))?,
            WeightT
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_names, node_types, edge_types, minimum_component_size, top_k_components, verbose)"]
    /// remove all the components that are not connected to interesting
    /// nodes and edges.
    ///
    /// Parameters
    /// ----------
    /// node_names: Optional[List[str]],
    ///     The name of the nodes of which components to keep.
    /// node_types: Optional[List[Optional[str]]],
    ///     The types of the nodes of which components to keep.
    /// edge_types: Optional[List[Optional[str]]],
    ///     The types of the edges of which components to keep.
    /// minimum_component_size: Optional[int],
    ///     Optional, Minimum size of the components to keep.
    /// top_k_components: Optional[int],
    ///     Optional, number of components to keep sorted by number of nodes.
    /// verbose: Optional[bool],
    ///     Whether to show the loading bar.
    ///
    pub fn remove_components(
        &self,
        node_names: Option<Vec<String>>,
        node_types: Option<Vec<Option<String>>>,
        edge_types: Option<Vec<Option<String>>>,
        minimum_component_size: Option<NodeT>,
        top_k_components: Option<NodeT>,
        verbose: Option<bool>,
    ) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(self.graph.remove_components(
                node_names,
                node_types,
                edge_types,
                minimum_component_size,
                top_k_components,
                verbose
            ))?,
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, other)"]
    /// Return whether given graph has any edge overlapping with current graph.
    ///
    /// Parameters
    /// ----------
    /// other: EnsmallenGraph,
    ///     The graph to check against.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If a graph is directed and the other is undirected.
    /// ValueError
    ///     If one of the two graphs has edge weights and the other does not.
    /// ValueError
    ///     If one of the two graphs has node types and the other does not.
    /// ValueError
    ///     If one of the two graphs has edge types and the other does not.
    ///
    pub fn overlaps(&self, other: &EnsmallenGraph) -> PyResult<bool> {
        pe!(self.graph.overlaps(&other.graph))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, other)"]
    /// Return true if given graph edges are all contained within current graph.
    ///
    /// Parameters
    /// ----------
    /// other: EnsmallenGraph,
    ///     The graph to check against.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If a graph is directed and the other is undirected.
    /// ValueError
    ///     If one of the two graphs has edge weights and the other does not.
    /// ValueError
    ///     If one of the two graphs has node types and the other does not.
    /// ValueError
    ///     If one of the two graphs has edge types and the other does not.
    ///
    pub fn contains(&self, other: &EnsmallenGraph) -> PyResult<bool> {
        pe!(self.graph.contains(&other.graph))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, removed_existing_edges, first_nodes_set, second_nodes_set, first_node_types_set, second_node_types_set)"]
    /// Return vector of tuple of Node IDs that form the edges of the required bipartite graph.
    ///
    /// Parameters
    /// ----------
    /// removed_existing_edges: Optional[bool],
    ///     Whether to filter out the existing edges. By default, true.
    /// first_nodes_set: Optional[Set[str]],
    ///     Optional set of nodes to use to create the first set of nodes of the graph.
    /// second_nodes_set: Optional[Set[str]],
    ///     Optional set of nodes to use to create the second set of nodes of the graph.
    /// first_node_types_set: Optional[Set[str]],
    ///     Optional set of node types to create the first set of nodes of the graph.
    /// second_node_types_set: Optional[Set[str]],
    ///     Optional set of node types to create the second set of nodes of the graph.
    ///
    pub fn get_bipartite_edges(
        &self,
        removed_existing_edges: Option<bool>,
        first_nodes_set: Option<HashSet<String>>,
        second_nodes_set: Option<HashSet<String>>,
        first_node_types_set: Option<HashSet<String>>,
        second_node_types_set: Option<HashSet<String>>,
    ) -> PyResult<Py<PyArray2<NodeT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_2d!(
            gil,
            pe!(self.graph.get_bipartite_edges(
                removed_existing_edges,
                first_nodes_set,
                second_nodes_set,
                first_node_types_set,
                second_node_types_set
            ))?,
            NodeT
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, removed_existing_edges, first_nodes_set, second_nodes_set, first_node_types_set, second_node_types_set)"]
    /// Return vector of tuple of Node IDs that form the edges of the required bipartite graph.
    ///
    /// Parameters
    /// ----------
    /// removed_existing_edges: Optional[bool],
    ///     Whether to filter out the existing edges. By default, true.
    /// first_nodes_set: Optional[Set[str]],
    ///     Optional set of nodes to use to create the first set of nodes of the graph.
    /// second_nodes_set: Optional[Set[str]],
    ///     Optional set of nodes to use to create the second set of nodes of the graph.
    /// first_node_types_set: Optional[Set[str]],
    ///     Optional set of node types to create the first set of nodes of the graph.
    /// second_node_types_set: Optional[Set[str]],
    ///     Optional set of node types to create the second set of nodes of the graph.
    ///
    pub fn get_bipartite_edge_names(
        &self,
        removed_existing_edges: Option<bool>,
        first_nodes_set: Option<HashSet<String>>,
        second_nodes_set: Option<HashSet<String>>,
        first_node_types_set: Option<HashSet<String>>,
        second_node_types_set: Option<HashSet<String>>,
    ) -> PyResult<Vec<Vec<String>>> {
        pe!(self.graph.get_bipartite_edge_names(
            removed_existing_edges,
            first_nodes_set,
            second_nodes_set,
            first_node_types_set,
            second_node_types_set
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, central_node, removed_existing_edges, star_points_nodes_set, star_points_node_types_set)"]
    /// Return vector of tuple of Node IDs that form the edges of the required star.
    ///
    /// Parameters
    /// ----------
    /// central_node: str,
    ///     Name of the node to use as center of the star.
    /// removed_existing_edges: Optional[bool],
    ///     Whether to filter out the existing edges. By default, true.
    /// star_points_nodes_set: Optional[Set[str]],
    ///     Optional set of nodes to use to create the set of star points.
    /// star_points_node_types_set: Optional[Set[str]],
    ///     Optional set of node types to create the set of star points.
    ///
    pub fn get_star_edges(
        &self,
        central_node: String,
        removed_existing_edges: Option<bool>,
        star_points_nodes_set: Option<HashSet<String>>,
        star_points_node_types_set: Option<HashSet<String>>,
    ) -> PyResult<Py<PyArray2<NodeT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_2d!(
            gil,
            pe!(self.graph.get_star_edges(
                central_node,
                removed_existing_edges,
                star_points_nodes_set,
                star_points_node_types_set
            ))?,
            NodeT
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, central_node, removed_existing_edges, star_points_nodes_set, star_points_node_types_set)"]
    /// Return vector of tuple of Node names that form the edges of the required star.
    ///
    /// Parameters
    /// ----------
    /// central_node: str,
    ///     Name of the node to use as center of the star.
    /// removed_existing_edges: Optional[bool],
    ///     Whether to filter out the existing edges. By default, true.
    /// star_points_nodes_set: Optional[Set[str]],
    ///     Optional set of nodes to use to create the set of star points.
    /// star_points_node_types_set: Optional[Set[str]],
    ///     Optional set of node types to create the set of star points.
    ///
    pub fn get_star_edge_names(
        &self,
        central_node: String,
        removed_existing_edges: Option<bool>,
        star_points_nodes_set: Option<HashSet<String>>,
        star_points_node_types_set: Option<HashSet<String>>,
    ) -> PyResult<Vec<Vec<String>>> {
        pe!(self.graph.get_star_edge_names(
            central_node,
            removed_existing_edges,
            star_points_nodes_set,
            star_points_node_types_set
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, directed, allow_selfloops, removed_existing_edges, allow_node_type_set, allow_node_set)"]
    /// Return vector of tuple of Node IDs that form the edges of the required clique.
    ///
    /// Parameters
    /// ----------
    /// directed: Optional[bool],
    ///     Whether to return the edges as directed or undirected. By default, equal to the graph.
    /// allow_selfloops: Optional[bool],
    ///     Whether to allow self-loops in the clique. By default, equal to the graph.
    /// removed_existing_edges: Optional[bool],
    ///     Whether to filter out the existing edges. By default, true.
    /// allow_node_type_set: Optional[Set[str]],
    ///     Node types to include in the clique.
    /// allow_node_set: Optional[Set[str]],
    ///     Nodes to include i the clique.
    ///
    pub fn get_clique_edges(
        &self,
        directed: Option<bool>,
        allow_selfloops: Option<bool>,
        removed_existing_edges: Option<bool>,
        allow_node_type_set: Option<HashSet<String>>,
        allow_node_set: Option<HashSet<String>>,
    ) -> Py<PyArray2<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_2d!(
            gil,
            self.graph.get_clique_edges(
                directed,
                allow_selfloops,
                removed_existing_edges,
                allow_node_type_set,
                allow_node_set
            ),
            NodeT
        )
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, directed, allow_selfloops, removed_existing_edges, allow_node_type_set, allow_node_set)"]
    /// Return vector of tuple of Node names that form the edges of the required clique.
    ///
    /// Parameters
    /// ----------
    /// directed: Optional[bool],
    ///     Whether to return the edges as directed or undirected. By default, equal to the graph.
    /// allow_selfloops: Optional[bool],
    ///     Whether to allow self-loops in the clique. By default, equal to the graph.
    /// removed_existing_edges: Optional[bool],
    ///     Whether to filter out the existing edges. By default, true.
    /// allow_node_type_set: Optional[Set[str]],
    ///     Node types to include in the clique.
    /// allow_node_set: Optional[Set[str]],
    ///     Nodes to include i the clique.
    ///
    pub fn get_clique_edge_names(
        &self,
        directed: Option<bool>,
        allow_selfloops: Option<bool>,
        removed_existing_edges: Option<bool>,
        allow_node_type_set: Option<HashSet<String>>,
        allow_node_set: Option<HashSet<String>>,
    ) -> Vec<Vec<String>> {
        self.graph.get_clique_edge_names(
            directed,
            allow_selfloops,
            removed_existing_edges,
            allow_node_type_set,
            allow_node_set,
        )
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src, dst)"]
    /// Return edge value corresponding to given node IDs.
    ///
    /// Parameters
    /// ----------
    /// src: int,
    ///     The source node ID.
    /// dst: int,
    ///     The destination node ID.
    ///
    pub fn encode_edge(&self, src: NodeT, dst: NodeT) -> u64 {
        self.graph.encode_edge(src, dst)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge)"]
    /// Returns source and destination nodes corresponding to given edge ID.
    ///
    /// Parameters
    /// ----------
    /// edge: int,
    ///     The edge value to decode.
    ///
    pub fn decode_edge(&self, edge: u64) -> (NodeT, NodeT) {
        self.graph.decode_edge(edge)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return maximum encodable edge number
    pub fn get_max_encodable_edge_number(&self) -> EdgeT {
        self.graph.get_max_encodable_edge_number()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_id)"]
    /// Validates provided node ID.
    ///
    /// Parameters
    /// ----------
    /// node_id: int,
    ///     node ID to validate.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given node ID does not exists in the graph.
    ///
    pub fn validate_node_id(&self, node_id: NodeT) -> PyResult<NodeT> {
        pe!(self.graph.validate_node_id(node_id))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_ids)"]
    /// Validates all provided node IDs.
    ///
    /// Parameters
    /// ----------
    /// node_ids: List[int],
    ///     node IDs to validate.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If any of the given node ID does not exists in the graph.
    ///
    pub fn validate_node_ids(&self, node_ids: Vec<NodeT>) -> PyResult<Py<PyArray1<NodeT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self.graph.validate_node_ids(node_ids))?,
            NodeT
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_id)"]
    /// Validates provided edge ID.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int,
    ///     Edge ID to validate.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given edge ID does not exists in the graph.
    ///
    pub fn validate_edge_id(&self, edge_id: EdgeT) -> PyResult<EdgeT> {
        pe!(self.graph.validate_edge_id(edge_id))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_ids)"]
    /// Validates provided edge IDs.
    ///
    /// Parameters
    /// ----------
    /// edge_ids: List[int],
    ///     Edge IDs to validate.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If any of the given edge ID does not exists in the graph.
    ///
    pub fn validate_edge_ids(&self, edge_ids: Vec<EdgeT>) -> PyResult<Py<PyArray1<EdgeT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self.graph.validate_edge_ids(edge_ids))?,
            EdgeT
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Raises an error if the graph contains unknown node types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not contain node types.
    /// ValueError
    ///     If the graph contains unknown node types.
    ///
    pub fn must_not_contain_unknown_node_types(&self) -> PyResult<()> {
        pe!(self.graph.must_not_contain_unknown_node_types())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Raises an error if the graph contains unknown edge types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not contain edge types.
    /// ValueError
    ///     If the graph contains unknown edge types.
    ///
    pub fn must_not_contain_unknown_edge_types(&self) -> PyResult<()> {
        pe!(self.graph.must_not_contain_unknown_edge_types())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_type_id)"]
    /// Validates provided node type ID.
    ///
    /// Parameters
    /// ----------
    /// node_type_id: Optional[int],
    ///     Node type ID to validate.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given node type ID does not exists in the graph.
    ///
    pub fn validate_node_type_id(
        &self,
        node_type_id: Option<NodeTypeT>,
    ) -> PyResult<Option<NodeTypeT>> {
        pe!(self.graph.validate_node_type_id(node_type_id))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_type_ids)"]
    /// Validates provided node type IDs.
    ///
    /// Parameters
    /// ----------
    /// node_type_ids: List[Optional[int]],
    ///     Vector of node type IDs to validate.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no node types in the graph.
    ///
    pub fn validate_node_type_ids(
        &self,
        node_type_ids: Vec<Option<NodeTypeT>>,
    ) -> PyResult<Vec<Option<NodeTypeT>>> {
        pe!(self.graph.validate_node_type_ids(node_type_ids))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_type_id)"]
    /// Validates provided edge type ID.
    ///
    /// Parameters
    /// ----------
    /// edge_type_id: Optional[int],
    ///     edge type ID to validate.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given edge type ID does not exists in the graph.
    ///
    pub fn validate_edge_type_id(
        &self,
        edge_type_id: Option<EdgeTypeT>,
    ) -> PyResult<Option<EdgeTypeT>> {
        pe!(self.graph.validate_edge_type_id(edge_type_id))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_type_ids)"]
    /// Validates provided edge type IDs.
    ///
    /// Parameters
    /// ----------
    /// edge_type_ids: List[Optional[int]],
    ///     Vector of edge type IDs to validate.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    ///
    pub fn validate_edge_type_ids(
        &self,
        edge_type_ids: Vec<Option<EdgeTypeT>>,
    ) -> PyResult<Vec<Option<EdgeTypeT>>> {
        pe!(self.graph.validate_edge_type_ids(edge_type_ids))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Raises an error if the graph does not have edge types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph is directed.
    ///
    pub fn must_be_undirected(&self) -> PyResult<()> {
        pe!(self.graph.must_be_undirected())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Raises an error if the graph does not have edge types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph is not a multigraph.
    ///
    pub fn must_be_multigraph(&self) -> PyResult<()> {
        pe!(self.graph.must_be_multigraph())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Raises an error if the graph does not have edge types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph is a multigraph.
    ///
    pub fn must_not_be_multigraph(&self) -> PyResult<()> {
        pe!(self.graph.must_not_be_multigraph())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Raises an error if the graph contains zero weighted degree.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edges.
    ///
    pub fn must_not_contain_weighted_singleton_nodes(&self) -> PyResult<()> {
        pe!(self.graph.must_not_contain_weighted_singleton_nodes())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Raises an error if the graph has a maximal weighted
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edges.
    ///
    pub fn must_have_edges(&self) -> PyResult<()> {
        pe!(self.graph.must_have_edges())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Raises an error if the graph does not have any node.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have nodes.
    ///
    pub fn must_have_nodes(&self) -> PyResult<()> {
        pe!(self.graph.must_have_nodes())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, features, neighbours_number, max_degree, distance_name, verbose)"]
    /// Returns graph with edges added extracted from given node_features.
    ///
    /// This operation might distrupt the graph topology.
    /// Proceed with caution!
    ///
    /// Parameters
    /// ----------
    /// features: List[List[float]],
    ///     node_features to use to identify the new neighbours.
    /// neighbours_number: Optional[int],
    ///     Number of neighbours to add.
    /// max_degree: Optional[int],
    ///     The maximum degree a node can have its neighbours augmented. By default 0, that is, only singletons are augmented.
    /// distance_name: Optional[str],
    ///     Name of distance to use. Can either be L2 or COSINE. By default COSINE.
    /// verbose: Optional[bool],
    ///     Whether to show loading bars.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have nodes.
    /// ValueError
    ///     If the given node_features are not provided exactly for each node.
    /// ValueError
    ///     If the node_features do not have a consistent shape.
    /// ValueError
    ///     If the provided number of neighbours is zero.
    ///
    pub fn generate_new_edges_from_node_features(
        &self,
        features: Vec<Vec<f64>>,
        neighbours_number: Option<NodeT>,
        max_degree: Option<NodeT>,
        distance_name: Option<&str>,
        verbose: Option<bool>,
    ) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(self.graph.generate_new_edges_from_node_features(
                features,
                neighbours_number,
                max_degree,
                distance_name,
                verbose
            ))?,
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, name)"]
    /// Set the name of the graph.
    ///
    /// Parameters
    /// ----------
    /// name: str,
    ///     Name of the graph.
    ///
    pub fn set_name(&mut self, name: String) {
        self.graph.set_name(name);
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_type)"]
    /// Replace all edge types (if present) and set all the edge to edge_type.
    ///
    /// This happens INPLACE, that is edits the current graph instance.
    ///
    /// Parameters
    /// ----------
    /// edge_type: str,
    ///     The edge type to assing to all the edges.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edges.
    /// ValueError
    ///     If the graph is a multigraph.
    ///
    pub fn set_inplace_all_edge_types(&mut self, edge_type: String) -> PyResult<()> {
        pe!(self.graph.set_inplace_all_edge_types(edge_type))?;
        Ok(())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_type, verbose)"]
    /// Replace all edge types (if present) and set all the edge to edge_type.
    ///
    /// This DOES NOT happen inplace, but created a new instance of the graph.
    ///
    /// Parameters
    /// ----------
    /// edge_type: str,
    ///     The edge type to assing to all the edges.
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar in the case of a multigraph.
    ///
    pub fn set_all_edge_types(
        &self,
        edge_type: String,
        verbose: Option<bool>,
    ) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(self.graph.set_all_edge_types(edge_type, verbose))?,
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_type)"]
    /// Replace all node types (if present) and set all the node to node_type.
    ///
    /// Parameters
    /// ----------
    /// node_type: str,
    ///     The node type to assing to all the nodes.
    ///
    pub fn set_inplace_all_node_types(&mut self, node_type: String) -> PyResult<()> {
        pe!(self.graph.set_inplace_all_node_types(node_type))?;
        Ok(())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_type)"]
    /// Replace all node types (if present) and set all the node to node_type.
    ///
    /// This DOES NOT happen inplace, but created a new instance of the graph.
    ///
    /// Parameters
    /// ----------
    /// node_type: str,
    ///     The node type to assing to all the nodes.
    ///
    pub fn set_all_node_types(&self, node_type: String) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(self.graph.set_all_node_types(node_type))?,
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_type_ids_to_remove)"]
    /// Remove given node type ID from all nodes.
    ///
    /// If any given node remains with no node type, that node is labeled
    /// with node type None. Note that the modification happens inplace.
    ///
    /// Parameters
    /// ----------
    /// node_type_id_to_remove: int,
    ///     The node type ID to remove.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    /// ValueError
    ///     If the given node type ID does not exists in the graph.
    ///
    pub fn remove_inplace_node_type_ids(
        &mut self,
        node_type_ids_to_remove: Vec<NodeTypeT>,
    ) -> PyResult<()> {
        pe!(self
            .graph
            .remove_inplace_node_type_ids(node_type_ids_to_remove))?;
        Ok(())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Remove singleton node types from all nodes.
    ///
    /// If any given node remains with no node type, that node is labeled
    /// with node type None. Note that the modification happens inplace.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn remove_inplace_singleton_node_types(&mut self) -> PyResult<()> {
        pe!(self.graph.remove_inplace_singleton_node_types())?;
        Ok(())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_type_ids_to_remove)"]
    /// Remove given edge type ID from all edges.
    ///
    /// Parameters
    /// ----------
    /// edge_type_id: int,
    ///     The edge type ID to remove.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph is a multigraph.
    /// ValueError
    ///     If the graph does not have edge types.
    /// ValueError
    ///     If the given edge type ID does not exists in the graph.
    ///
    pub fn remove_inplace_edge_type_ids(
        &mut self,
        edge_type_ids_to_remove: Vec<EdgeTypeT>,
    ) -> PyResult<()> {
        pe!(self
            .graph
            .remove_inplace_edge_type_ids(edge_type_ids_to_remove))?;
        Ok(())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Remove singleton edge types from all edges.
    ///
    /// If any given edge remains with no edge type, that edge is labeled
    /// with edge type None. Note that the modification happens inplace.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    ///
    pub fn remove_inplace_singleton_edge_types(&mut self) -> PyResult<()> {
        pe!(self.graph.remove_inplace_singleton_edge_types())?;
        Ok(())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_type_name)"]
    /// Remove given node type name from all nodes.
    ///
    /// If any given node remains with no node type, that node is labeled
    /// with node type None. Note that the modification happens inplace.
    ///
    /// Parameters
    /// ----------
    /// node_type_name: str,
    ///     The node type ID to remove.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    /// ValueError
    ///     If the given node type name does not exists in the graph.
    ///
    pub fn remove_inplace_node_type_name(&mut self, node_type_name: &str) -> PyResult<()> {
        pe!(self.graph.remove_inplace_node_type_name(node_type_name))?;
        Ok(())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_type_id)"]
    /// Remove given node type ID from all nodes.
    ///
    /// If any given node remains with no node type, that node is labeled
    /// with node type None. Note that the modification DOES NOT happen inplace.
    ///
    /// Parameters
    /// ----------
    /// node_type_id: int,
    ///     The node type ID to remove.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    /// ValueError
    ///     If the given node type ID does not exists in the graph.
    ///
    pub fn remove_node_type_id(&self, node_type_id: NodeTypeT) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(self.graph.remove_node_type_id(node_type_id))?,
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Remove singleton node types from all nodes.
    ///
    /// If any given node remains with no node type, that node is labeled
    /// with node type None. Note that the modification DOES NOT happen inplace.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn remove_singleton_node_types(&self) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(self.graph.remove_singleton_node_types())?,
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_type_name)"]
    /// Remove given node type name from all nodes.
    ///
    /// If any given node remains with no node type, that node is labeled
    /// with node type None. Note that the modification DOES NOT happen inplace.
    ///
    /// Parameters
    /// ----------
    /// node_type_name: str,
    ///     The node type ID to remove.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    /// ValueError
    ///     If the given node type name does not exists in the graph.
    ///
    pub fn remove_node_type_name(&self, node_type_name: &str) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(self.graph.remove_node_type_name(node_type_name))?,
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_type_name)"]
    /// Remove given edge type name from all edges.
    ///
    /// If any given edge remains with no edge type, that edge is labeled
    /// with edge type None. Note that the modification happens inplace.
    ///
    /// Parameters
    /// ----------
    /// edge_type_name: str,
    ///     The edge type ID to remove.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    /// ValueError
    ///     If the given edge type name does not exists in the graph.
    ///
    pub fn remove_inplace_edge_type_name(&mut self, edge_type_name: &str) -> PyResult<()> {
        pe!(self.graph.remove_inplace_edge_type_name(edge_type_name))?;
        Ok(())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_type_id)"]
    /// Remove given edge type ID from all edges.
    ///
    /// If any given edge remains with no edge type, that edge is labeled
    /// with edge type None. Note that the modification DOES NOT happen inplace.
    ///
    /// Parameters
    /// ----------
    /// edge_type_id: int,
    ///     The edge type ID to remove.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    /// ValueError
    ///     If the given edge type ID does not exists in the graph.
    ///
    pub fn remove_edge_type_id(&self, edge_type_id: EdgeTypeT) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(self.graph.remove_edge_type_id(edge_type_id))?,
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Remove singleton edge types from all edges.
    ///
    /// If any given edge remains with no edge type, that edge is labeled
    /// with edge type None. Note that the modification DOES NOT happen inplace.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    ///
    pub fn remove_singleton_edge_types(&self) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(self.graph.remove_singleton_edge_types())?,
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_type_name)"]
    /// Remove given edge type name from all edges.
    ///
    /// If any given edge remains with no edge type, that edge is labeled
    /// with edge type None. Note that the modification DOES NOT happen inplace.
    ///
    /// Parameters
    /// ----------
    /// edge_type_name: str,
    ///     The edge type ID to remove.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    /// ValueError
    ///     If the given edge type name does not exists in the graph.
    ///
    pub fn remove_edge_type_name(&self, edge_type_name: &str) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(self.graph.remove_edge_type_name(edge_type_name))?,
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Remove node types from the graph.
    ///
    /// Note that the modification happens inplace.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn remove_inplace_node_types(&mut self) -> PyResult<()> {
        pe!(self.graph.remove_inplace_node_types())?;
        Ok(())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Remove node types from the graph.
    ///
    /// Note that the modification does not happen inplace.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn remove_node_types(&self) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(self.graph.remove_node_types())?,
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Remove edge types from the graph.
    ///
    /// Note that the modification happens inplace.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    /// ValueError
    ///     If the graph is a multigraph.
    ///
    pub fn remove_inplace_edge_types(&mut self) -> PyResult<()> {
        pe!(self.graph.remove_inplace_edge_types())?;
        Ok(())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, verbose)"]
    /// Remove edge types from the graph.
    ///
    /// Note that the modification does not happen inplace.
    ///
    /// Parameters
    /// ----------
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar in the case of a multigraph.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    ///
    pub fn remove_edge_types(&self, verbose: Option<bool>) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(self.graph.remove_edge_types(verbose))?,
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Remove edge weights from the graph.
    ///
    /// Note that the modification happens inplace.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge weights.
    ///
    pub fn remove_inplace_edge_weights(&mut self) -> PyResult<()> {
        pe!(self.graph.remove_inplace_edge_weights())?;
        Ok(())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Remove edge weights from the graph.
    ///
    /// Note that the modification does not happen inplace.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge weights.
    ///
    pub fn remove_edge_weights(&self) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(self.graph.remove_edge_weights())?,
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, normalize, low_centrality, verbose)"]
    /// Returns total number of triangles ignoring the weights.
    ///
    /// The method dispatches the fastest method according to the current
    /// graph instance. Specifically:
    /// - For directed graphs it will use the naive algorithm.
    /// - For undirected graphs it will use Bader's version.
    ///
    /// Parameters
    /// ----------
    /// normalize: Optional[bool],
    ///     Whether to normalize the number of triangles.
    /// low_centrality: Optional[int],
    ///     The threshold over which to switch to parallel matryoshka. By default 50.
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar.
    ///
    pub fn get_number_of_triangles(
        &self,
        normalize: Option<bool>,
        low_centrality: Option<usize>,
        verbose: Option<bool>,
    ) -> EdgeT {
        self.graph
            .get_number_of_triangles(normalize, low_centrality, verbose)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns total number of triads in the graph without taking into account weights
    pub fn get_triads_number(&self) -> EdgeT {
        self.graph.get_triads_number()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns total number of triads in the weighted graph
    pub fn get_weighted_triads_number(&self) -> PyResult<f64> {
        pe!(self.graph.get_weighted_triads_number())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, low_centrality, verbose)"]
    /// Returns transitivity of the graph without taking into account weights.
    ///
    /// Parameters
    /// ----------
    /// low_centrality: Optional[int],
    ///     The threshold over which to switch to parallel matryoshka. By default 50.
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar.
    ///
    pub fn get_transitivity(&self, low_centrality: Option<usize>, verbose: Option<bool>) -> f64 {
        self.graph.get_transitivity(low_centrality, verbose)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, normalize, low_centrality, verbose)"]
    /// Returns number of triangles in the graph without taking into account the weights.
    ///
    /// The method dispatches the fastest method according to the current
    /// graph instance. Specifically:
    /// - For directed graphs it will use the naive algorithm.
    /// - For undirected graphs it will use Bader's version.
    ///
    /// Parameters
    /// ----------
    /// normalize: Optional[bool],
    ///     Whether to normalize the number of triangles.
    /// low_centrality: Optional[int],
    ///     The threshold over which to switch to parallel matryoshka. By default 50.
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar.
    ///
    pub fn get_number_of_triangles_per_node(
        &self,
        normalize: Option<bool>,
        low_centrality: Option<usize>,
        verbose: Option<bool>,
    ) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(
            gil,
            self.graph
                .get_number_of_triangles_per_node(normalize, low_centrality, verbose),
            NodeT
        )
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, low_centrality, verbose)"]
    /// Returns clustering coefficients for all nodes in the graph.
    ///
    /// Parameters
    /// ----------
    /// low_centrality: Optional[int],
    ///     The threshold over which to switch to parallel matryoshka. By default 50.
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar.
    ///
    pub fn get_clustering_coefficient_per_node(
        &self,
        low_centrality: Option<usize>,
        verbose: Option<bool>,
    ) -> Py<PyArray1<f64>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(
            gil,
            self.graph
                .get_clustering_coefficient_per_node(low_centrality, verbose),
            f64
        )
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, low_centrality, verbose)"]
    /// Returns the graph clustering coefficient.
    ///
    /// Parameters
    /// ----------
    /// low_centrality: Optional[int],
    ///     The threshold over which to switch to parallel matryoshka. By default 50.
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar.
    ///
    pub fn get_clustering_coefficient(
        &self,
        low_centrality: Option<usize>,
        verbose: Option<bool>,
    ) -> f64 {
        self.graph
            .get_clustering_coefficient(low_centrality, verbose)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, low_centrality, verbose)"]
    /// Returns the graph average clustering coefficient.
    ///
    /// Parameters
    /// ----------
    /// low_centrality: Optional[int],
    ///     The threshold over which to switch to parallel matryoshka. By default 50.
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar.
    ///
    pub fn get_average_clustering_coefficient(
        &self,
        low_centrality: Option<usize>,
        verbose: Option<bool>,
    ) -> f64 {
        self.graph
            .get_average_clustering_coefficient(low_centrality, verbose)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, other)"]
    /// Return whether nodes are remappable to those of the given graph.
    ///
    /// Parameters
    /// ----------
    /// other: EnsmallenGraph,
    ///     graph towards remap the nodes to.
    ///
    pub fn are_nodes_remappable(&self, other: &EnsmallenGraph) -> bool {
        self.graph.are_nodes_remappable(&other.graph)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_ids, verbose)"]
    /// Returns graph remapped using given node IDs ordering.
    ///
    /// Parameters
    /// ----------
    /// node_ids: List[int],
    ///     The node Ids to remap the graph to.
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar while building the graph.
    ///
    ///
    /// Safety
    /// ------
    /// This method will cause a panic if the node IDs are either:
    ///  * Not unique
    ///  * Not available for each of the node IDs of the graph.
    pub unsafe fn remap_unchecked_from_node_ids(
        &self,
        node_ids: Vec<NodeT>,
        verbose: Option<bool>,
    ) -> EnsmallenGraph {
        EnsmallenGraph {
            graph: self.graph.remap_unchecked_from_node_ids(node_ids, verbose),
        }
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_ids, verbose)"]
    /// Returns graph remapped using given node IDs ordering.
    ///
    /// Parameters
    /// ----------
    /// node_ids: List[int],
    ///     The node Ids to remap the graph to.
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar while building the graph.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given node IDs are not unique.
    /// ValueError
    ///     If the given node IDs are not available for all the values in the graph.
    ///
    pub fn remap_from_node_ids(
        &self,
        node_ids: Vec<NodeT>,
        verbose: Option<bool>,
    ) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(self.graph.remap_from_node_ids(node_ids, verbose))?,
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_names, verbose)"]
    /// Returns graph remapped using given node names ordering.
    ///
    /// Parameters
    /// ----------
    /// node_names: List[str],
    ///     The node names to remap the graph to.
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar while building the graph.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given node names are not unique.
    /// ValueError
    ///     If the given node names are not available for all the values in the graph.
    ///
    pub fn remap_from_node_names(
        &self,
        node_names: Vec<&str>,
        verbose: Option<bool>,
    ) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(self.graph.remap_from_node_names(node_names, verbose))?,
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, other, verbose)"]
    /// Return graph remapped towards nodes of the given graph.
    ///
    /// Parameters
    /// ----------
    /// other: EnsmallenGraph,
    ///     The graph to remap towards.
    /// verbose: Optional[bool],
    ///     Whether to show a loding bar.
    ///
    pub fn remap_from_graph(
        &self,
        other: &EnsmallenGraph,
        verbose: Option<bool>,
    ) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(self.graph.remap_from_graph(&other.graph, verbose))?,
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, negatives_number, random_state, seed_graph, only_from_same_component, verbose)"]
    /// Returns Graph with given amount of negative edges as positive edges.
    ///
    /// The graph generated may be used as a testing negatives partition to be
    /// fed into the argument "graph_to_avoid" of the link_prediction or the
    /// skipgrams algorithm
    ///
    /// Parameters
    /// ----------
    /// negatives_number: int,
    ///     Number of negatives edges to include.
    /// random_state: Optional[int],
    ///     random_state to use to reproduce negative edge set.
    /// seed_graph: Optional[EnsmallenGraph],
    ///     Optional graph to use to filter the negative edges. The negative edges generated when this variable is provided will always have a node within this graph.
    /// only_from_same_component: Optional[bool],
    ///     Whether to sample negative edges only from nodes that are from the same component.
    /// verbose: Optional[bool],
    ///     Whether to show the loading bar.
    ///
    pub fn sample_negatives(
        &self,
        negatives_number: EdgeT,
        random_state: Option<EdgeT>,
        seed_graph: Option<&EnsmallenGraph>,
        only_from_same_component: Option<bool>,
        verbose: Option<bool>,
    ) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(self.graph.sample_negatives(
                negatives_number,
                random_state,
                seed_graph.map(|sg| &sg.graph),
                only_from_same_component,
                verbose
            ))?,
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, train_size, random_state, edge_types, include_all_edge_types, verbose)"]
    /// Returns holdout for training ML algorithms on the graph structure.
    ///
    /// The holdouts returned are a tuple of graphs. The first one, which
    /// is the training graph, is garanteed to have the same number of
    /// graph components as the initial graph. The second graph is the graph
    /// meant for testing or validation of the algorithm, and has no garantee
    /// to be connected. It will have at most (1-train_size) edges,
    /// as the bound of connectivity which is required for the training graph
    /// may lead to more edges being left into the training partition.
    ///
    /// In the option where a list of edge types has been provided, these
    /// edge types will be those put into the validation set.
    ///
    /// Parameters
    /// ----------
    /// train_size: float,
    ///     Rate target to reserve for training.
    /// random_state: Optional[int],
    ///     The random_state to use for the holdout,
    /// edge_types: Optional[List[Optional[str]]],
    ///     Edge types to be selected for in the validation set.
    /// include_all_edge_types: Optional[bool],
    ///     Whether to include all the edges between two nodes.
    /// verbose: Optional[bool],
    ///     Whether to show the loading bar.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the edge types have been specified but the graph does not have edge types.
    /// ValueError
    ///     If the required training size is not a real value between 0 and 1.
    /// ValueError
    ///     If the current graph does not allow for the creation of a spanning tree for the requested training size.
    ///
    pub fn connected_holdout(
        &self,
        train_size: f64,
        random_state: Option<EdgeT>,
        edge_types: Option<Vec<Option<String>>>,
        include_all_edge_types: Option<bool>,
        verbose: Option<bool>,
    ) -> PyResult<(EnsmallenGraph, EnsmallenGraph)> {
        let (g1, g2) = pe!(self.graph.connected_holdout(
            train_size,
            random_state,
            edge_types,
            include_all_edge_types,
            verbose
        ))?;
        Ok((EnsmallenGraph { graph: g1 }, EnsmallenGraph { graph: g2 }))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, train_size, random_state, include_all_edge_types, edge_types, min_number_overlaps, verbose)"]
    /// Returns random holdout for training ML algorithms on the graph edges.
    ///
    /// The holdouts returned are a tuple of graphs. In neither holdouts the
    /// graph connectivity is necessarily preserved. To maintain that, use
    /// the method `connected_holdout`.
    ///
    /// Parameters
    /// ----------
    /// train_size: float,
    ///     rate target to reserve for training
    /// random_state: Optional[int],
    ///     The random_state to use for the holdout,
    /// include_all_edge_types: Optional[bool],
    ///     Whether to include all the edges between two nodes.
    /// edge_types: Optional[List[Optional[str]]],
    ///     The edges to include in validation set.
    /// min_number_overlaps: Optional[int],
    ///     The minimum number of overlaps to include the edge into the validation set.
    /// verbose: Optional[bool],
    ///     Whether to show the loading bar.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the edge types have been specified but the graph does not have edge types.
    /// ValueError
    ///     If the minimum number of overlaps have been specified but the graph is not a multigraph.
    /// ValueError
    ///     If one or more of the given edge type names is not present in the graph.
    ///
    pub fn random_holdout(
        &self,
        train_size: f64,
        random_state: Option<EdgeT>,
        include_all_edge_types: Option<bool>,
        edge_types: Option<Vec<Option<String>>>,
        min_number_overlaps: Option<EdgeT>,
        verbose: Option<bool>,
    ) -> PyResult<(EnsmallenGraph, EnsmallenGraph)> {
        let (g1, g2) = pe!(self.graph.random_holdout(
            train_size,
            random_state,
            include_all_edge_types,
            edge_types,
            min_number_overlaps,
            verbose
        ))?;
        Ok((EnsmallenGraph { graph: g1 }, EnsmallenGraph { graph: g2 }))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, train_size, use_stratification, random_state)"]
    /// Returns node-label holdout for training ML algorithms on the graph node labels.
    ///
    /// Parameters
    /// ----------
    /// train_size: float,
    ///     rate target to reserve for training,
    /// use_stratification: Optional[bool],
    ///     Whether to use node-label stratification,
    /// random_state: Optional[int],
    ///     The random_state to use for the holdout,
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    /// ValueError
    ///     If stratification is requested but the graph has a single node type.
    /// ValueError
    ///     If stratification is requested but the graph has a multilabel node types.
    ///
    pub fn node_label_holdout(
        &self,
        train_size: f64,
        use_stratification: Option<bool>,
        random_state: Option<EdgeT>,
    ) -> PyResult<(EnsmallenGraph, EnsmallenGraph)> {
        let (g1, g2) =
            pe!(self
                .graph
                .node_label_holdout(train_size, use_stratification, random_state))?;
        Ok((EnsmallenGraph { graph: g1 }, EnsmallenGraph { graph: g2 }))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, train_size, use_stratification, random_state)"]
    /// Returns edge-label holdout for training ML algorithms on the graph edge labels.
    /// This is commonly used for edge type prediction tasks.
    ///
    /// This method returns two graphs, the train and the test one.
    /// The edges of the graph will be splitted in the train and test graphs according
    /// to the `train_size` argument.
    ///
    /// If stratification is enabled, the train and test will have the same ratios of
    /// edge types.
    ///
    /// Parameters
    /// ----------
    /// train_size: float,
    ///     rate target to reserve for training,
    /// use_stratification: Optional[bool],
    ///     Whether to use edge-label stratification,
    /// random_state: Optional[int],
    ///     The random_state to use for the holdout,
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    /// ValueError
    ///     If stratification is required but the graph has singleton edge types.
    ///
    pub fn edge_label_holdout(
        &self,
        train_size: f64,
        use_stratification: Option<bool>,
        random_state: Option<EdgeT>,
    ) -> PyResult<(EnsmallenGraph, EnsmallenGraph)> {
        let (g1, g2) =
            pe!(self
                .graph
                .edge_label_holdout(train_size, use_stratification, random_state))?;
        Ok((EnsmallenGraph { graph: g1 }, EnsmallenGraph { graph: g2 }))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, nodes_number, random_state, verbose)"]
    /// Returns subgraph with given number of nodes.
    ///
    /// **This method creates a subset of the graph starting from a random node
    /// sampled using given random_state and includes all neighbouring nodes until
    /// the required number of nodes is reached**. All the edges connecting any
    /// of the selected nodes are then inserted into this graph.
    ///
    /// This is meant to execute distributed node embeddings.
    /// It may also sample singleton nodes.
    ///
    /// Parameters
    /// ----------
    /// nodes_number: int,
    ///     Number of nodes to extract.
    /// random_state: Optional[int],
    ///     Random random_state to use.
    /// verbose: Optional[bool],
    ///     Whether to show the loading bar.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the requested number of nodes is one or less.
    /// ValueError
    ///     If the graph has less than the requested number of nodes.
    ///
    pub fn random_subgraph(
        &self,
        nodes_number: NodeT,
        random_state: Option<usize>,
        verbose: Option<bool>,
    ) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(self
                .graph
                .random_subgraph(nodes_number, random_state, verbose))?,
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, k, k_index, edge_types, random_state, verbose)"]
    /// Returns train and test graph following kfold validation scheme.
    ///
    /// The edges are splitted into k chunks. The k_index-th chunk is used to build
    /// the validation graph, all the other edges create the training graph.
    ///
    /// Parameters
    /// ----------
    /// k: int,
    ///     The number of folds.
    /// k_index: int,
    ///     Which fold to use for the validation.
    /// edge_types: Optional[List[Optional[str]]],
    ///     Edge types to be selected when computing the folds (All the edge types not listed here will be always be used in the training set).
    /// random_state: Optional[int],
    ///     The random_state (seed) to use for the holdout,
    /// verbose: Optional[bool],
    ///     Whether to show the loading bar.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the number of requested k folds is one or zero.
    /// ValueError
    ///     If the given k fold index is greater than the number of k folds.
    /// ValueError
    ///     If edge types have been specified but it's an empty list.
    /// ValueError
    ///     If the number of k folds is higher than the number of edges in the graph.
    ///
    pub fn kfold(
        &self,
        k: EdgeT,
        k_index: u64,
        edge_types: Option<Vec<Option<String>>>,
        random_state: Option<EdgeT>,
        verbose: Option<bool>,
    ) -> PyResult<(EnsmallenGraph, EnsmallenGraph)> {
        let (g1, g2) = pe!(self
            .graph
            .kfold(k, k_index, edge_types, random_state, verbose))?;
        Ok((EnsmallenGraph { graph: g1 }, EnsmallenGraph { graph: g2 }))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src_node_id, dst_node_id, maximal_depth)"]
    /// Returns minimum path node IDs and distance from given node ids.
    ///
    /// Parameters
    /// ----------
    /// src_node_id: int,
    ///     Source node ID.
    /// dst_node_id: int,
    ///     Destination node ID.
    /// maximal_depth: Optional[int],
    ///     The maximal depth to execute the BFS for.
    ///
    ///
    /// Safety
    /// ------
    /// If any of the given node IDs does not exist in the graph the method will panic.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given node is a selfloop.
    /// ValueError
    ///     If there is no path between the two given nodes.
    ///
    pub unsafe fn get_unchecked_minimum_path_node_ids_from_node_ids(
        &self,
        src_node_id: NodeT,
        dst_node_id: NodeT,
        maximal_depth: Option<NodeT>,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self
                .graph
                .get_unchecked_minimum_path_node_ids_from_node_ids(
                    src_node_id,
                    dst_node_id,
                    maximal_depth
                ))?,
            NodeT
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src_node_id, dst_node_id, maximal_depth)"]
    /// Returns minimum path node names from given node ids.
    ///
    /// Parameters
    /// ----------
    /// src_node_id: int,
    ///     Source node ID.
    /// dst_node_id: int,
    ///     Destination node ID.
    /// maximal_depth: Optional[int],
    ///     The maximal depth to execute the BFS for.
    ///
    ///
    /// Safety
    /// ------
    /// If any of the given node IDs does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_minimum_path_node_names_from_node_ids(
        &self,
        src_node_id: NodeT,
        dst_node_id: NodeT,
        maximal_depth: Option<NodeT>,
    ) -> PyResult<Vec<String>> {
        pe!(self
            .graph
            .get_unchecked_minimum_path_node_names_from_node_ids(
                src_node_id,
                dst_node_id,
                maximal_depth
            ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src_node_id, dst_node_id, maximal_depth)"]
    /// Returns minimum path node names from given node ids.
    ///
    /// Parameters
    /// ----------
    /// src_node_id: int,
    ///     Source node ID.
    /// dst_node_id: int,
    ///     Destination node ID.
    /// maximal_depth: Optional[int],
    ///     The maximal depth to execute the BFS for.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If any of the given node IDs do not exist in the current graph.
    ///
    pub fn get_minimum_path_node_ids_from_node_ids(
        &self,
        src_node_id: NodeT,
        dst_node_id: NodeT,
        maximal_depth: Option<NodeT>,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self.graph.get_minimum_path_node_ids_from_node_ids(
                src_node_id,
                dst_node_id,
                maximal_depth
            ))?,
            NodeT
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src_node_name, dst_node_name, maximal_depth)"]
    /// Returns minimum path node names from given node names.
    ///
    /// Parameters
    /// ----------
    /// src_node_name: str,
    ///     Source node name.
    /// dst_node_name: str,
    ///     Destination node name.
    /// maximal_depth: Optional[int],
    ///     The maximal depth to execute the BFS for.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If any of the given node names do not exist in the current graph.
    ///
    pub fn get_minimum_path_node_ids_from_node_names(
        &self,
        src_node_name: &str,
        dst_node_name: &str,
        maximal_depth: Option<NodeT>,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self.graph.get_minimum_path_node_ids_from_node_names(
                src_node_name,
                dst_node_name,
                maximal_depth
            ))?,
            NodeT
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src_node_name, dst_node_name, maximal_depth)"]
    /// Returns minimum path node names from given node names.
    ///
    /// Parameters
    /// ----------
    /// src_node_name: str,
    ///     Source node name.
    /// dst_node_name: str,
    ///     Destination node name.
    /// maximal_depth: Optional[int],
    ///     The maximal depth to execute the BFS for.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If any of the given node names do not exist in the current graph.
    ///
    pub fn get_minimum_path_node_names_from_node_names(
        &self,
        src_node_name: &str,
        dst_node_name: &str,
        maximal_depth: Option<NodeT>,
    ) -> PyResult<Vec<String>> {
        pe!(self.graph.get_minimum_path_node_names_from_node_names(
            src_node_name,
            dst_node_name,
            maximal_depth
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src_node_id, dst_node_id, k)"]
    /// Return vector of the k minimum paths node IDs between given source node and destination node ID.
    ///
    /// Parameters
    /// ----------
    /// src_node_id: int,
    ///     Source node ID.
    /// dst_node_id: int,
    ///     Destination node ID.
    /// k: int,
    ///     Number of paths to find.
    ///
    ///
    /// Safety
    /// ------
    /// If any of the given node IDs does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_k_shortest_path_node_ids_from_node_ids(
        &self,
        src_node_id: NodeT,
        dst_node_id: NodeT,
        k: usize,
    ) -> Vec<Vec<NodeT>> {
        self.graph
            .get_unchecked_k_shortest_path_node_ids_from_node_ids(src_node_id, dst_node_id, k)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src_node_id, dst_node_id, k)"]
    /// Return vector of the k minimum paths node IDs between given source node and destination node ID.
    ///
    /// Parameters
    /// ----------
    /// src_node_id: int,
    ///     Source node ID.
    /// dst_node_id: int,
    ///     Destination node ID.
    /// maximal_depth: Optional[int],
    ///     The maximal depth to execute the BFS for.
    /// k: int,
    ///     Number of paths to find.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If any of the given node IDs does not exist in the graph.
    ///
    pub fn get_k_shortest_path_node_ids_from_node_ids(
        &self,
        src_node_id: NodeT,
        dst_node_id: NodeT,
        k: usize,
    ) -> PyResult<Vec<Vec<NodeT>>> {
        pe!(self
            .graph
            .get_k_shortest_path_node_ids_from_node_ids(src_node_id, dst_node_id, k))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src_node_name, dst_node_name, k)"]
    /// Return vector of the k minimum paths node IDs between given source node and destination node name.
    ///
    /// Parameters
    /// ----------
    /// src_node_name: str,
    ///     Source node name.
    /// dst_node_name: str,
    ///     Destination node name.
    /// k: int,
    ///     Number of paths to find.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If any of the given node names does not exist in the graph.
    ///
    pub fn get_k_shortest_path_node_ids_from_node_names(
        &self,
        src_node_name: &str,
        dst_node_name: &str,
        k: usize,
    ) -> PyResult<Vec<Vec<NodeT>>> {
        pe!(self.graph.get_k_shortest_path_node_ids_from_node_names(
            src_node_name,
            dst_node_name,
            k
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src_node_name, dst_node_name, k)"]
    /// Return vector of the k minimum paths node names between given source node and destination node name.
    ///
    /// Parameters
    /// ----------
    /// src_node_name: str,
    ///     Source node name.
    /// dst_node_name: str,
    ///     Destination node name.
    /// k: int,
    ///     Number of paths to find.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If any of the given node names does not exist in the graph.
    ///
    pub fn get_k_shortest_path_node_names_from_node_names(
        &self,
        src_node_name: &str,
        dst_node_name: &str,
        k: usize,
    ) -> PyResult<Vec<Vec<String>>> {
        pe!(self.graph.get_k_shortest_path_node_names_from_node_names(
            src_node_name,
            dst_node_name,
            k
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_id)"]
    /// Returns unweighted eccentricity of the given node.
    ///
    /// This method will panic if the given node ID does not exists in the graph.
    ///
    /// Parameters
    /// ----------
    /// node_id: int,
    ///     Node for which to compute the eccentricity.
    ///
    ///
    /// Safety
    /// ------
    /// If any of the given node IDs does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_eccentricity_from_node_id(&self, node_id: NodeT) -> NodeT {
        self.graph.get_unchecked_eccentricity_from_node_id(node_id)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_id)"]
    /// Returns unweighted eccentricity of the given node ID.
    ///
    /// Parameters
    /// ----------
    /// node_id: int,
    ///     Node for which to compute the eccentricity.
    /// use_edge_weights_as_probabilities: Optional[bool],
    ///     Whether to treat the edge weights as probabilities.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given node ID does not exist in the graph.
    ///
    pub fn get_eccentricity_from_node_id(&self, node_id: NodeT) -> PyResult<NodeT> {
        pe!(self.graph.get_eccentricity_from_node_id(node_id))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_id, use_edge_weights_as_probabilities)"]
    /// Returns weighted eccentricity of the given node ID.
    ///
    /// Parameters
    /// ----------
    /// node_id: int,
    ///     Node for which to compute the eccentricity.
    /// use_edge_weights_as_probabilities: Optional[bool],
    ///     Whether to treat the edge weights as probabilities.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given node ID does not exist in the graph.
    /// ValueError
    ///     If weights are requested to be treated as probabilities but are not between 0 and 1.
    /// ValueError
    ///     If the graph contains negative weights.
    ///
    pub fn get_weighted_eccentricity_from_node_id(
        &self,
        node_id: NodeT,
        use_edge_weights_as_probabilities: Option<bool>,
    ) -> PyResult<f64> {
        pe!(self
            .graph
            .get_weighted_eccentricity_from_node_id(node_id, use_edge_weights_as_probabilities))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_name)"]
    /// Returns unweighted eccentricity of the given node name.
    ///
    /// Parameters
    /// ----------
    /// node_name: str,
    ///     Node for which to compute the eccentricity.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given node name does not exist in the current graph instance.
    ///
    pub fn get_eccentricity_from_node_name(&self, node_name: &str) -> PyResult<NodeT> {
        pe!(self.graph.get_eccentricity_from_node_name(node_name))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_name, use_edge_weights_as_probabilities)"]
    /// Returns weighted eccentricity of the given node name.
    ///
    /// Parameters
    /// ----------
    /// node_name: str,
    ///     Node for which to compute the eccentricity.
    /// use_edge_weights_as_probabilities: Optional[bool],
    ///     Whether to treat the edge weights as probabilities.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given node name does not exist in the graph.
    /// ValueError
    ///     If weights are requested to be treated as probabilities but are not between 0 and 1.
    /// ValueError
    ///     If the graph contains negative weights.
    ///
    pub fn get_weighted_eccentricity_from_node_name(
        &self,
        node_name: &str,
        use_edge_weights_as_probabilities: Option<bool>,
    ) -> PyResult<f64> {
        pe!(self
            .graph
            .get_weighted_eccentricity_from_node_name(node_name, use_edge_weights_as_probabilities))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src_node_id, dst_node_id, use_edge_weights_as_probabilities, maximal_depth)"]
    /// Returns minimum path node IDs and distance from given node ids.
    ///
    /// Parameters
    /// ----------
    /// src_node_id: int,
    ///     Source node ID.
    /// dst_node_id: int,
    ///     Destination node ID.
    /// maximal_depth: Optional[int],
    ///     The maximal depth to execute the BFS for.
    /// use_edge_weights_as_probabilities: Optional[bool],
    ///     Whether to treat the edge weights as probabilities.
    /// maximal_depth: Optional[int],
    ///     The maximal number of iterations to execute Dijkstra for.
    ///
    ///
    /// Safety
    /// ------
    /// If any of the given node IDs does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_weighted_minimum_path_node_ids_from_node_ids(
        &self,
        src_node_id: NodeT,
        dst_node_id: NodeT,
        use_edge_weights_as_probabilities: Option<bool>,
        maximal_depth: Option<NodeT>,
    ) -> (f64, Vec<NodeT>) {
        self.graph
            .get_unchecked_weighted_minimum_path_node_ids_from_node_ids(
                src_node_id,
                dst_node_id,
                use_edge_weights_as_probabilities,
                maximal_depth,
            )
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src_node_id, dst_node_id, use_edge_weights_as_probabilities, maximal_depth)"]
    /// Returns minimum path node names from given node ids.
    ///
    /// Parameters
    /// ----------
    /// src_node_id: int,
    ///     Source node ID.
    /// dst_node_id: int,
    ///     Destination node ID.
    /// maximal_depth: Optional[int],
    ///     The maximal depth to execute the BFS for.
    /// use_edge_weights_as_probabilities: Optional[bool],
    ///     Whether to treat the edge weights as probabilities.
    /// maximal_depth: Optional[int],
    ///     The maximal number of iterations to execute Dijkstra for.
    ///
    ///
    /// Safety
    /// ------
    /// If any of the given node IDs does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_weighted_minimum_path_node_names_from_node_ids(
        &self,
        src_node_id: NodeT,
        dst_node_id: NodeT,
        use_edge_weights_as_probabilities: Option<bool>,
        maximal_depth: Option<NodeT>,
    ) -> (f64, Vec<String>) {
        self.graph
            .get_unchecked_weighted_minimum_path_node_names_from_node_ids(
                src_node_id,
                dst_node_id,
                use_edge_weights_as_probabilities,
                maximal_depth,
            )
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src_node_id, dst_node_id, use_edge_weights_as_probabilities, maximal_depth)"]
    /// Returns minimum path node names from given node ids.
    ///
    /// Parameters
    /// ----------
    /// src_node_id: int,
    ///     Source node ID.
    /// dst_node_id: int,
    ///     Destination node ID.
    /// maximal_depth: Optional[int],
    ///     The maximal depth to execute the BFS for.
    /// use_edge_weights_as_probabilities: Optional[bool],
    ///     Whether to treat the edge weights as probabilities.
    /// maximal_depth: Optional[int],
    ///     The maximal number of iterations to execute Dijkstra for.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If any of the given node IDs do not exist in the current graph.
    ///
    pub fn get_weighted_minimum_path_node_ids_from_node_ids(
        &self,
        src_node_id: NodeT,
        dst_node_id: NodeT,
        use_edge_weights_as_probabilities: Option<bool>,
        maximal_depth: Option<NodeT>,
    ) -> PyResult<(f64, Vec<NodeT>)> {
        pe!(self.graph.get_weighted_minimum_path_node_ids_from_node_ids(
            src_node_id,
            dst_node_id,
            use_edge_weights_as_probabilities,
            maximal_depth
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src_node_name, dst_node_name, use_edge_weights_as_probabilities, maximal_depth)"]
    /// Returns minimum path node names from given node names.
    ///
    /// Parameters
    /// ----------
    /// src_node_name: str,
    ///     Source node name.
    /// dst_node_name: str,
    ///     Destination node name.
    /// use_edge_weights_as_probabilities: Optional[bool],
    ///     Whether to treat the edge weights as probabilities.
    /// maximal_depth: Optional[int],
    ///     The maximal number of iterations to execute Dijkstra for.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If any of the given node names do not exist in the current graph.
    ///
    pub fn get_weighted_minimum_path_node_ids_from_node_names(
        &self,
        src_node_name: &str,
        dst_node_name: &str,
        use_edge_weights_as_probabilities: Option<bool>,
        maximal_depth: Option<NodeT>,
    ) -> PyResult<(f64, Vec<NodeT>)> {
        pe!(self
            .graph
            .get_weighted_minimum_path_node_ids_from_node_names(
                src_node_name,
                dst_node_name,
                use_edge_weights_as_probabilities,
                maximal_depth
            ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src_node_name, dst_node_name, use_edge_weights_as_probabilities, maximal_depth)"]
    /// Returns minimum path node names from given node names.
    ///
    /// Parameters
    /// ----------
    /// src_node_name: str,
    ///     Source node name.
    /// dst_node_name: str,
    ///     Destination node name.
    /// use_edge_weights_as_probabilities: Optional[bool],
    ///     Whether to treat the edge weights as probabilities.
    /// maximal_depth: Optional[int],
    ///     The maximal number of iterations to execute Dijkstra for.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If any of the given node names do not exist in the current graph.
    ///
    pub fn get_weighted_minimum_path_node_names_from_node_names(
        &self,
        src_node_name: &str,
        dst_node_name: &str,
        use_edge_weights_as_probabilities: Option<bool>,
        maximal_depth: Option<NodeT>,
    ) -> PyResult<(f64, Vec<String>)> {
        pe!(self
            .graph
            .get_weighted_minimum_path_node_names_from_node_names(
                src_node_name,
                dst_node_name,
                use_edge_weights_as_probabilities,
                maximal_depth
            ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, ignore_infinity, verbose)"]
    /// Returns diameter of the graph using naive method.
    ///
    /// Note that there exists the non-naive method for undirected graphs
    /// and it is possible to implement a faster method for directed graphs
    /// but we still need to get to it, as it will require an updated
    /// succinct data structure.
    ///
    /// Parameters
    /// ----------
    /// ignore_infinity: Optional[bool],
    ///     Whether to ignore infinite distances, which are present when in the graph exist multiple components.
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not contain nodes.
    ///
    pub fn get_diameter_naive(
        &self,
        ignore_infinity: Option<bool>,
        verbose: Option<bool>,
    ) -> PyResult<f64> {
        pe!(self.graph.get_diameter_naive(ignore_infinity, verbose))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, ignore_infinity, verbose)"]
    /// Returns diameter of the graph.
    ///
    /// Parameters
    /// ----------
    /// ignore_infinity: Optional[bool],
    ///     Whether to ignore infinite distances, which are present when in the graph exist multiple components.
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not contain nodes.
    ///
    pub fn get_diameter(
        &self,
        ignore_infinity: Option<bool>,
        verbose: Option<bool>,
    ) -> PyResult<f64> {
        pe!(self.graph.get_diameter(ignore_infinity, verbose))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, ignore_infinity, use_edge_weights_as_probabilities, verbose)"]
    /// Returns diameter of the graph using naive method.
    ///
    /// Note that there exists the non-naive method for undirected graphs
    /// and it is possible to implement a faster method for directed graphs
    /// but we still need to get to it, as it will require an updated
    /// succinct data structure.
    ///
    /// Parameters
    /// ----------
    /// ignore_infinity: Optional[bool],
    ///     Whether to ignore infinite distances, which are present when in the graph exist multiple components.
    /// use_edge_weights_as_probabilities: Optional[bool],
    ///     Whether to treat the edge weights as probabilities.
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not contain nodes.
    /// ValueError
    ///     If the graph does not have weights.
    /// ValueError
    ///     If the graph contains negative weights.
    /// ValueError
    ///     If the user has asked for the weights to be treated as probabilities but the weights are not between 0 and 1.
    ///
    pub fn get_weighted_diameter_naive(
        &self,
        ignore_infinity: Option<bool>,
        use_edge_weights_as_probabilities: Option<bool>,
        verbose: Option<bool>,
    ) -> PyResult<f64> {
        pe!(self.graph.get_weighted_diameter_naive(
            ignore_infinity,
            use_edge_weights_as_probabilities,
            verbose
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, verbose)"]
    /// Returns number a triple with (number of components, number of nodes of the smallest component, number of nodes of the biggest component )
    ///
    /// Parameters
    /// ----------
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar or not.
    ///
    pub fn get_connected_components_number(&self, verbose: Option<bool>) -> (NodeT, NodeT, NodeT) {
        self.graph.get_connected_components_number(verbose)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns number of singleton nodes within the graph.
    pub fn get_singleton_nodes_number(&self) -> NodeT {
        self.graph.get_singleton_nodes_number()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return number of weighted singleton nodes within the graph.
    ///
    /// This number represents the number of nodes that have weighted node
    /// degree equal to 0, which may happen when the graph contains edges
    /// with negative edge weights.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not contain edge weights.
    ///
    pub fn get_weighted_singleton_nodes_number(&self) -> PyResult<NodeT> {
        pe!(self.graph.get_weighted_singleton_nodes_number())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns number of disconnected nodes within the graph.
    /// A Disconnected node is a node which is nor a singleton nor a singleton
    /// with selfloops.
    pub fn get_disconnected_nodes_number(&self) -> NodeT {
        self.graph.get_disconnected_nodes_number()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns vector of singleton node IDs of the graph.
    pub fn get_singleton_node_ids(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.graph.get_singleton_node_ids(), NodeT)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns vector of singleton node names of the graph.
    pub fn get_singleton_node_names(&self) -> Vec<String> {
        self.graph.get_singleton_node_names()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns number of singleton nodes with self-loops within the graph.
    pub fn get_singleton_nodes_with_selfloops_number(&self) -> NodeT {
        self.graph.get_singleton_nodes_with_selfloops_number()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns vector of singleton_with_selfloops node IDs of the graph.
    pub fn get_singleton_with_selfloops_node_ids(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(
            gil,
            self.graph.get_singleton_with_selfloops_node_ids(),
            NodeT
        )
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns vector of singleton_with_selfloops node names of the graph.
    pub fn get_singleton_with_selfloops_node_names(&self) -> Vec<String> {
        self.graph.get_singleton_with_selfloops_node_names()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns number of not singleton nodes within the graph.
    pub fn get_connected_nodes_number(&self) -> NodeT {
        self.graph.get_connected_nodes_number()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns density of the graph.
    pub fn get_density(&self) -> PyResult<f64> {
        pe!(self.graph.get_density())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns the traps rate of the graph.
    ///
    /// THIS IS EXPERIMENTAL AND MUST BE PROVEN!
    pub fn get_trap_nodes_rate(&self) -> f64 {
        self.graph.get_trap_nodes_rate()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns unweighted mean node degree of the graph.
    pub fn get_node_degrees_mean(&self) -> PyResult<f64> {
        pe!(self.graph.get_node_degrees_mean())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns weighted mean node degree of the graph.
    pub fn get_weighted_node_degrees_mean(&self) -> PyResult<f64> {
        pe!(self.graph.get_weighted_node_degrees_mean())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns number of undirected edges of the graph.
    pub fn get_undirected_edges_number(&self) -> EdgeT {
        self.graph.get_undirected_edges_number()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns number of undirected edges of the graph.
    pub fn get_unique_undirected_edges_number(&self) -> EdgeT {
        self.graph.get_unique_undirected_edges_number()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns number of edges of the graph.
    pub fn get_edges_number(&self) -> EdgeT {
        self.graph.get_edges_number()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns number of unique edges of the graph.
    pub fn get_unique_edges_number(&self) -> EdgeT {
        self.graph.get_unique_edges_number()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns unweighted median node degree of the graph
    pub fn get_node_degrees_median(&self) -> PyResult<NodeT> {
        pe!(self.graph.get_node_degrees_median())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns weighted median node degree of the graph
    pub fn get_weighted_node_degrees_median(&self) -> PyResult<f64> {
        pe!(self.graph.get_weighted_node_degrees_median())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns maximum unweighted node degree of the graph.
    ///
    /// Safety
    /// ------
    /// The method will return an undefined value (0) when the graph
    ///  does not contain nodes. In those cases the value is not properly
    ///  defined.
    pub unsafe fn get_unchecked_maximum_node_degree(&self) -> NodeT {
        self.graph.get_unchecked_maximum_node_degree()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns maximum weighted node degree of the graph.
    ///
    /// Safety
    /// ------
    /// This method will cause a panic on graphs without
    ///  edge weights.
    pub unsafe fn get_unchecked_weighted_maximum_node_degree(&self) -> f64 {
        self.graph.get_unchecked_weighted_maximum_node_degree()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns maximum weighted node degree of the graph.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the current graph does not contain edge weights.
    ///
    pub fn get_weighted_maximum_node_degree(&self) -> PyResult<f64> {
        pe!(self.graph.get_weighted_maximum_node_degree())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns minimum weighted node degree of the graph.
    ///
    /// Safety
    /// ------
    /// This method will cause a panic on graphs without
    ///  edge weights.
    pub unsafe fn get_unchecked_weighted_minimum_node_degree(&self) -> f64 {
        self.graph.get_unchecked_weighted_minimum_node_degree()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns minimum weighted node degree of the graph.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the current graph does not contain edge weights.
    ///
    pub fn get_weighted_minimum_node_degree(&self) -> PyResult<f64> {
        pe!(self.graph.get_weighted_minimum_node_degree())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns maximum node degree of the graph.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not contain any node (is an empty graph).
    ///
    pub fn get_maximum_node_degree(&self) -> PyResult<NodeT> {
        pe!(self.graph.get_maximum_node_degree())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns maximum node degree of the graph.
    ///
    /// Safety
    /// ------
    /// This method fails with a panic if the graph does not have any node.
    pub unsafe fn get_unchecked_argmax_node_degree(&self) -> NodeT {
        self.graph.get_unchecked_argmax_node_degree()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns maximum node degree of the graph.
    pub fn get_argmax_node_degree(&self) -> PyResult<NodeT> {
        pe!(self.graph.get_argmax_node_degree())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns minimum node degree of the graph.
    ///
    /// Safety
    /// ------
    /// The method will return an undefined value (NodeT::MAX) when the graph
    ///  does not contain nodes. In those cases the value is not properly
    ///  defined.
    pub unsafe fn get_unchecked_minimum_node_degree(&self) -> NodeT {
        self.graph.get_unchecked_minimum_node_degree()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns minimum weighted node degree of the graph
    pub fn get_weighted_mininum_node_degree(&self) -> PyResult<f64> {
        pe!(self.graph.get_weighted_mininum_node_degree())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns minimum node degree of the graph.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not contain any node (is an empty graph).
    ///
    pub fn get_minimum_node_degree(&self) -> PyResult<NodeT> {
        pe!(self.graph.get_minimum_node_degree())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns mode node degree of the graph.
    pub fn get_node_degrees_mode(&self) -> PyResult<NodeT> {
        pe!(self.graph.get_node_degrees_mode())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns number of self-loops, including also those in eventual multi-edges.
    pub fn get_selfloop_number(&self) -> EdgeT {
        self.graph.get_selfloop_number()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns number of unique self-loops, excluding those in eventual multi-edges.
    pub fn get_unique_selfloop_number(&self) -> NodeT {
        self.graph.get_unique_selfloop_number()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns rate of self-loops.
    pub fn get_selfloop_nodes_rate(&self) -> PyResult<f64> {
        pe!(self.graph.get_selfloop_nodes_rate())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return name of the graph.
    pub fn get_name(&self) -> String {
        self.graph.get_name()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the number of traps (nodes without any outgoing edges that are not singletons)
    /// This also includes nodes with only a self-loops, therefore singletons with
    /// only a self-loops are not considered traps because you could make a walk on them.
    pub fn get_trap_nodes_number(&self) -> EdgeT {
        self.graph.get_trap_nodes_number()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, directed)"]
    /// Return vector of the non-unique source nodes.
    ///
    /// Parameters
    /// ----------
    /// directed: bool,
    ///     Whether to filter out the undirected edges.
    ///
    pub fn get_source_node_ids(&self, directed: bool) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.graph.get_source_node_ids(directed), NodeT)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, directed)"]
    /// Return vector of the non-unique source nodes names.
    ///
    /// Parameters
    /// ----------
    /// directed: bool,
    ///     Whether to filter out the undirected edges.
    ///
    pub fn get_source_names(&self, directed: bool) -> Vec<String> {
        self.graph.get_source_names(directed)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, directed)"]
    /// Return vector on the (non unique) destination nodes of the graph.
    ///
    /// Parameters
    /// ----------
    /// directed: bool,
    ///     Whether to filter out the undirected edges.
    ///
    pub fn get_destination_node_ids(&self, directed: bool) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.graph.get_destination_node_ids(directed), NodeT)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, directed)"]
    /// Return vector of the non-unique destination nodes names.
    ///
    /// Parameters
    /// ----------
    /// directed: bool,
    ///     Whether to filter out the undirected edges.
    ///
    pub fn get_destination_names(&self, directed: bool) -> Vec<String> {
        self.graph.get_destination_names(directed)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return vector with the sorted nodes names
    pub fn get_node_names(&self) -> Vec<String> {
        self.graph.get_node_names()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return vector with the sorted nodes Ids
    pub fn get_node_ids(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.graph.get_node_ids(), NodeT)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the edge types of the edges
    pub fn get_edge_type_ids(&self) -> PyResult<Vec<Option<EdgeTypeT>>> {
        pe!(self.graph.get_edge_type_ids())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the unique edge type IDs of the graph edges.
    pub fn get_unique_edge_type_ids(&self) -> PyResult<Py<PyArray1<EdgeTypeT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self.graph.get_unique_edge_type_ids())?,
            EdgeTypeT
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the edge types names
    pub fn get_edge_type_names(&self) -> PyResult<Vec<Option<String>>> {
        pe!(self.graph.get_edge_type_names())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the edge types names
    pub fn get_unique_edge_type_names(&self) -> PyResult<Vec<String>> {
        pe!(self.graph.get_unique_edge_type_names())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the weights of the graph edges.
    pub fn get_edge_weights(&self) -> PyResult<Py<PyArray1<WeightT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self.graph.get_edge_weights())?,
            WeightT
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return total edge weights, if graph has weights.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not contain edge weights.
    ///
    pub fn get_total_edge_weights(&self) -> PyResult<f64> {
        pe!(self.graph.get_total_edge_weights())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the minimum weight, if graph has weights.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not contain edge weights.
    ///
    pub fn get_mininum_edge_weight(&self) -> PyResult<WeightT> {
        pe!(self.graph.get_mininum_edge_weight())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the maximum weight, if graph has weights.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not contain edge weights.
    ///
    pub fn get_maximum_edge_weight(&self) -> PyResult<WeightT> {
        pe!(self.graph.get_maximum_edge_weight())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the node types of the graph nodes.
    pub fn get_node_type_ids(&self) -> PyResult<Vec<Option<Vec<NodeTypeT>>>> {
        pe!(self.graph.get_node_type_ids())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns one-hot encoded node types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn get_one_hot_encoded_node_types(&self) -> PyResult<Py<PyArray2<bool>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_2d!(
            gil,
            pe!(self.graph.get_one_hot_encoded_node_types())?,
            bool
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns one-hot encoded known node types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn get_one_hot_encoded_known_node_types(&self) -> PyResult<Py<PyArray2<bool>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_2d!(
            gil,
            pe!(self.graph.get_one_hot_encoded_known_node_types())?,
            bool
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns one-hot encoded edge types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    ///
    pub fn get_one_hot_encoded_edge_types(&self) -> PyResult<Py<PyArray2<bool>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_2d!(
            gil,
            pe!(self.graph.get_one_hot_encoded_edge_types())?,
            bool
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns one-hot encoded known edge types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    ///
    pub fn get_one_hot_encoded_known_edge_types(&self) -> PyResult<Py<PyArray2<bool>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_2d!(
            gil,
            pe!(self.graph.get_one_hot_encoded_known_edge_types())?,
            bool
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the node types names.
    pub fn get_node_type_names(&self) -> PyResult<Vec<Option<Vec<String>>>> {
        pe!(self.graph.get_node_type_names())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the unique node type IDs of the graph nodes.
    pub fn get_unique_node_type_ids(&self) -> PyResult<Py<PyArray1<NodeTypeT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self.graph.get_unique_node_type_ids())?,
            NodeTypeT
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the unique node types names.
    pub fn get_unique_node_type_names(&self) -> PyResult<Vec<String>> {
        pe!(self.graph.get_unique_node_type_names())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return number of the unique edges in the graph
    pub fn get_unique_directed_edges_number(&self) -> EdgeT {
        self.graph.get_unique_directed_edges_number()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the nodes mapping
    pub fn get_nodes_mapping(&self) -> HashMap<String, NodeT> {
        self.graph.get_nodes_mapping()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, directed)"]
    /// Return vector with the sorted edge Ids.
    ///
    /// Parameters
    /// ----------
    /// directed: bool,
    ///     Whether to filter out the undirected edges.
    ///
    pub fn get_edge_node_ids(&self, directed: bool) -> Py<PyArray2<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_2d!(gil, self.graph.get_edge_node_ids(directed), NodeT)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, directed)"]
    /// Return vector with the sorted edge names.
    ///
    /// Parameters
    /// ----------
    /// directed: bool,
    ///     Whether to filter out the undirected edges.
    ///
    pub fn get_edge_node_names(&self, directed: bool) -> Vec<(String, String)> {
        self.graph.get_edge_node_names(directed)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns number of nodes with unknown node type.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no node types in the graph.
    ///
    pub fn get_unknown_node_types_number(&self) -> PyResult<NodeT> {
        pe!(self.graph.get_unknown_node_types_number())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns the number of node with known node type.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no node types in the graph.
    ///
    pub fn get_known_node_types_number(&self) -> PyResult<NodeT> {
        pe!(self.graph.get_known_node_types_number())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns rate of unknown node types over total nodes number.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no node types in the graph.
    ///
    pub fn get_unknown_node_types_rate(&self) -> PyResult<f64> {
        pe!(self.graph.get_unknown_node_types_rate())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns rate of known node types over total nodes number.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no node types in the graph.
    ///
    pub fn get_known_node_types_rate(&self) -> PyResult<f64> {
        pe!(self.graph.get_known_node_types_rate())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns minimum number of node types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no node types in the graph.
    ///
    pub fn get_minimum_node_types_number(&self) -> PyResult<NodeT> {
        pe!(self.graph.get_minimum_node_types_number())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns maximum number of node types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no node types in the graph.
    ///
    pub fn get_maximum_node_types_number(&self) -> PyResult<NodeT> {
        pe!(self.graph.get_maximum_node_types_number())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns number of maximum multilabel count.
    ///
    /// This value is the maximum number of multilabel counts
    /// that appear in any given node in the graph
    pub fn get_maximum_multilabel_count(&self) -> PyResult<NodeTypeT> {
        pe!(self.graph.get_maximum_multilabel_count())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns number of singleton node types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn get_singleton_node_types_number(&self) -> PyResult<NodeTypeT> {
        pe!(self.graph.get_singleton_node_types_number())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns vector of singleton node types IDs.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn get_singleton_node_type_ids(&self) -> PyResult<Py<PyArray1<NodeTypeT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self.graph.get_singleton_node_type_ids())?,
            NodeTypeT
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns vector of singleton node types names.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn get_singleton_node_type_names(&self) -> PyResult<Vec<String>> {
        pe!(self.graph.get_singleton_node_type_names())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns number of unknown edge types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    ///
    pub fn get_unknown_edge_types_number(&self) -> PyResult<EdgeT> {
        pe!(self.graph.get_unknown_edge_types_number())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns edge IDs of the edges with unknown edge types
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    ///
    pub fn get_edge_ids_with_unknown_edge_types(&self) -> PyResult<Py<PyArray1<EdgeT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self.graph.get_edge_ids_with_unknown_edge_types())?,
            EdgeT
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns edge IDs of the edges with known edge types
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    ///
    pub fn get_edge_ids_with_known_edge_types(&self) -> PyResult<Py<PyArray1<EdgeT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self.graph.get_edge_ids_with_known_edge_types())?,
            EdgeT
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, directed)"]
    /// Returns edge node IDs of the edges with unknown edge types
    ///
    /// Parameters
    /// ----------
    /// directed: bool,
    ///     Whether to iterated the edges as a directed or undirected edge list.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    ///
    pub fn get_edge_node_ids_with_unknown_edge_types(
        &self,
        directed: bool,
    ) -> PyResult<Vec<(NodeT, NodeT)>> {
        pe!(self
            .graph
            .get_edge_node_ids_with_unknown_edge_types(directed))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, directed)"]
    /// Returns edge node IDs of the edges with known edge types
    ///
    /// Parameters
    /// ----------
    /// directed: bool,
    ///     Whether to iterated the edges as a directed or undirected edge list.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    ///
    pub fn get_edge_node_ids_with_known_edge_types(
        &self,
        directed: bool,
    ) -> PyResult<Vec<(NodeT, NodeT)>> {
        pe!(self.graph.get_edge_node_ids_with_known_edge_types(directed))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, directed)"]
    /// Returns edge node names of the edges with unknown edge types
    ///
    /// Parameters
    /// ----------
    /// directed: bool,
    ///     Whether to iterated the edges as a directed or undirected edge list.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    ///
    pub fn get_edge_node_names_with_unknown_edge_types(
        &self,
        directed: bool,
    ) -> PyResult<Vec<(String, String)>> {
        pe!(self
            .graph
            .get_edge_node_names_with_unknown_edge_types(directed))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, directed)"]
    /// Returns edge node names of the edges with known edge types
    ///
    /// Parameters
    /// ----------
    /// directed: bool,
    ///     Whether to iterated the edges as a directed or undirected edge list.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    ///
    pub fn get_edge_node_names_with_known_edge_types(
        &self,
        directed: bool,
    ) -> PyResult<Vec<(String, String)>> {
        pe!(self
            .graph
            .get_edge_node_names_with_known_edge_types(directed))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns a boolean vector that for each node contains whether it has an
    /// unknown node type.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    ///
    pub fn get_edge_ids_with_unknown_edge_types_mask(&self) -> PyResult<Py<PyArray1<bool>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self.graph.get_edge_ids_with_unknown_edge_types_mask())?,
            bool
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns a boolean vector that for each node contains whether it has an
    /// unknown edge type.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    ///
    pub fn get_edge_ids_with_known_edge_types_mask(&self) -> PyResult<Py<PyArray1<bool>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self.graph.get_edge_ids_with_known_edge_types_mask())?,
            bool
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns node IDs of the nodes with unknown node types
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no node types in the graph.
    ///
    pub fn get_node_ids_with_unknown_node_types(&self) -> PyResult<Py<PyArray1<NodeT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self.graph.get_node_ids_with_unknown_node_types())?,
            NodeT
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns node IDs of the nodes with known node types
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no node types in the graph.
    ///
    pub fn get_node_ids_with_known_node_types(&self) -> PyResult<Py<PyArray1<NodeT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self.graph.get_node_ids_with_known_node_types())?,
            NodeT
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns node names of the nodes with unknown node types
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no node types in the graph.
    ///
    pub fn get_node_names_with_unknown_node_types(&self) -> PyResult<Vec<String>> {
        pe!(self.graph.get_node_names_with_unknown_node_types())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns node names of the nodes with known node types
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no node types in the graph.
    ///
    pub fn get_node_names_with_known_node_types(&self) -> PyResult<Vec<String>> {
        pe!(self.graph.get_node_names_with_known_node_types())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns a boolean vector that for each node contains whether it has an
    /// unknown node type.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no node types in the graph.
    ///
    pub fn get_node_ids_with_unknown_node_types_mask(&self) -> PyResult<Py<PyArray1<bool>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self.graph.get_node_ids_with_unknown_node_types_mask())?,
            bool
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns a boolean vector that for each node contains whether it has an
    /// known node type.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no node types in the graph.
    ///
    pub fn get_node_ids_with_known_node_types_mask(&self) -> PyResult<Py<PyArray1<bool>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self.graph.get_node_ids_with_known_node_types_mask())?,
            bool
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns the number of edge with known edge type.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    ///
    pub fn get_known_edge_types_number(&self) -> PyResult<EdgeT> {
        pe!(self.graph.get_known_edge_types_number())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns rate of unknown edge types over total edges number.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    ///
    pub fn get_unknown_edge_types_rate(&self) -> PyResult<f64> {
        pe!(self.graph.get_unknown_edge_types_rate())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns rate of known edge types over total edges number.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    ///
    pub fn get_known_edge_types_rate(&self) -> PyResult<f64> {
        pe!(self.graph.get_known_edge_types_rate())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns minimum number of edge types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    ///
    pub fn get_minimum_edge_types_number(&self) -> PyResult<EdgeT> {
        pe!(self.graph.get_minimum_edge_types_number())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns number of singleton edge types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    ///
    pub fn get_singleton_edge_types_number(&self) -> PyResult<EdgeTypeT> {
        pe!(self.graph.get_singleton_edge_types_number())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns vector of singleton edge types IDs.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    ///
    pub fn get_singleton_edge_type_ids(&self) -> PyResult<Py<PyArray1<EdgeTypeT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self.graph.get_singleton_edge_type_ids())?,
            EdgeTypeT
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns vector of singleton edge types names.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    ///
    pub fn get_singleton_edge_type_names(&self) -> PyResult<Vec<String>> {
        pe!(self.graph.get_singleton_edge_type_names())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns number of nodes in the graph
    pub fn get_nodes_number(&self) -> NodeT {
        self.graph.get_nodes_number()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, verbose)"]
    /// Return a vector with the components each node belongs to.
    ///
    /// E.g. If we have two components `[0, 2, 3]` and `[1, 4, 5]` the result will look like
    /// `[0, 1, 0, 0, 1, 1]`
    ///
    /// Parameters
    /// ----------
    /// verbose: Optional[bool],
    ///     Whether to show the loading bar.
    ///
    pub fn get_node_connected_component_ids(&self, verbose: Option<bool>) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(
            gil,
            self.graph.get_node_connected_component_ids(verbose),
            NodeT
        )
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns number of directed edges in the graph
    pub fn get_directed_edges_number(&self) -> EdgeT {
        self.graph.get_directed_edges_number()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns number of edge types in the graph.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the current graph.
    ///
    pub fn get_edge_types_number(&self) -> PyResult<EdgeTypeT> {
        pe!(self.graph.get_edge_types_number())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns number of node types in the graph.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no node types in the current graph.
    ///
    pub fn get_node_types_number(&self) -> PyResult<NodeTypeT> {
        pe!(self.graph.get_node_types_number())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns the unweighted degree of every node in the graph
    pub fn get_node_degrees(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.graph.get_node_degrees(), NodeT)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns the weighted degree of every node in the graph
    pub fn get_weighted_node_degrees(&self) -> PyResult<Py<PyArray1<f64>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self.graph.get_weighted_node_degrees())?,
            f64
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return set of nodes that are not singletons
    pub fn get_not_singletons_node_ids(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.graph.get_not_singletons_node_ids(), NodeT)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return mapping from instance not trap nodes to dense nodes
    pub fn get_dense_nodes_mapping(&self) -> HashMap<NodeT, NodeT> {
        self.graph.get_dense_nodes_mapping()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return number of edges that have multigraph syblings
    pub fn get_parallel_edges_number(&self) -> EdgeT {
        self.graph.get_parallel_edges_number()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return vector with node cumulative_node_degrees, that is the comulative node degree
    pub fn get_cumulative_node_degrees(&self) -> Py<PyArray1<EdgeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.graph.get_cumulative_node_degrees(), EdgeT)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns number of the source nodes.
    pub fn get_unique_source_nodes_number(&self) -> NodeT {
        self.graph.get_unique_source_nodes_number()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns edge type IDs counts hashmap.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the current graph instance.
    ///
    pub fn get_edge_type_id_counts_hashmap(&self) -> PyResult<HashMap<EdgeTypeT, EdgeT>> {
        pe!(self.graph.get_edge_type_id_counts_hashmap())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns edge type names counts hashmap.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the current graph instance.
    ///
    pub fn get_edge_type_names_counts_hashmap(&self) -> PyResult<HashMap<String, EdgeT>> {
        pe!(self.graph.get_edge_type_names_counts_hashmap())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns node type IDs counts hashmap.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no node types in the current graph instance.
    ///
    pub fn get_node_type_id_counts_hashmap(&self) -> PyResult<HashMap<NodeTypeT, NodeT>> {
        pe!(self.graph.get_node_type_id_counts_hashmap())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns node type names counts hashmap.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no node types in the current graph instance.
    ///
    pub fn get_node_type_names_counts_hashmap(&self) -> PyResult<HashMap<String, NodeT>> {
        pe!(self.graph.get_node_type_names_counts_hashmap())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Convert inplace the graph to directed.
    pub fn to_directed_inplace(&mut self) {
        self.graph.to_directed_inplace();
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return a new instance of the current graph as directed
    pub fn to_directed(&self) -> EnsmallenGraph {
        EnsmallenGraph {
            graph: self.graph.to_directed(),
        }
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, verbose)"]
    /// Return the directed graph from the upper triangular adjacency matrix.
    ///
    /// Parameters
    /// ----------
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar.
    ///
    pub fn to_upper_triangular(&self, verbose: Option<bool>) -> EnsmallenGraph {
        EnsmallenGraph {
            graph: self.graph.to_upper_triangular(verbose),
        }
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, verbose)"]
    /// Return the directed graph from the lower triangular adjacency matrix.
    ///
    /// Parameters
    /// ----------
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar.
    ///
    pub fn to_lower_triangular(&self, verbose: Option<bool>) -> EnsmallenGraph {
        EnsmallenGraph {
            graph: self.graph.to_lower_triangular(verbose),
        }
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, verbose)"]
    /// Return the graph from the main diagonal adjacency matrix.
    ///
    /// Parameters
    /// ----------
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar.
    ///
    pub fn to_main_diagonal(&self, verbose: Option<bool>) -> EnsmallenGraph {
        EnsmallenGraph {
            graph: self.graph.to_main_diagonal(verbose),
        }
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, verbose)"]
    /// Return the graph from the anti-diagonal adjacency matrix.
    ///
    /// Parameters
    /// ----------
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar.
    ///
    pub fn to_anti_diagonal(&self, verbose: Option<bool>) -> EnsmallenGraph {
        EnsmallenGraph {
            graph: self.graph.to_anti_diagonal(verbose),
        }
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, verbose)"]
    /// Return the graph from the bidiagonal adjacency matrix.
    ///
    /// Parameters
    /// ----------
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar.
    ///
    pub fn to_bidiagonal(&self, verbose: Option<bool>) -> EnsmallenGraph {
        EnsmallenGraph {
            graph: self.graph.to_bidiagonal(verbose),
        }
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, verbose)"]
    /// Return the graph from the arrowhead adjacency matrix.
    ///
    /// Parameters
    /// ----------
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar.
    ///
    pub fn to_arrowhead(&self, verbose: Option<bool>) -> EnsmallenGraph {
        EnsmallenGraph {
            graph: self.graph.to_arrowhead(verbose),
        }
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, verbose)"]
    /// Return the graph from the transposed adjacency matrix.
    ///
    /// Parameters
    /// ----------
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar.
    ///
    pub fn to_transposed(&self, verbose: Option<bool>) -> EnsmallenGraph {
        EnsmallenGraph {
            graph: self.graph.to_transposed(verbose),
        }
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, verbose)"]
    /// Return the complementary graph.
    ///
    /// Parameters
    /// ----------
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar.
    ///
    pub fn to_complementary(&self, verbose: Option<bool>) -> EnsmallenGraph {
        EnsmallenGraph {
            graph: self.graph.to_complementary(verbose),
        }
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns report relative to the graph metrics
    ///
    /// The report includes the following metrics by default:
    /// * Name of the graph
    /// * Whether the graph is directed or undirected
    /// * Number of singleton nodes
    /// * Number of nodes
    /// - If the graph has nodes, we also compute:
    /// * Minimum unweighted node degree
    /// * Maximum unweighted node degree
    /// * Unweighted node degree mean
    /// * Number of edges
    /// * Number of self-loops
    /// * Number of singleton with self-loops
    /// * Whether the graph is a multigraph
    /// * Number of parallel edges
    /// * Number of directed edges
    /// - If the graph has edges, we also compute:
    /// * Rate of self-loops
    /// * Whether the graph has weighted edges
    /// - If the graph has weights, we also compute:
    /// * Minimum weighted node degree
    /// * Maximum weighted node degree
    /// * Weighted node degree mean
    /// * The total edge weights
    /// * Whether the graph has node types
    /// - If the graph has node types, we also compute:
    /// * Whether the graph has singleton node types
    /// * The number of node types
    /// * The number of nodes with unknown node types
    /// * The number of nodes with known node types
    /// * Whether the graph has edge types
    /// - If the graph has edge types, we also compute:
    /// * Whether the graph has singleton edge types
    /// * The number of edge types
    /// * The number of edges with unknown edge types
    /// * The number of edges with known edge types
    ///
    /// On request, since it takes more time to compute it, the method also provides:
    pub fn report(&self) -> HashMap<&str, String> {
        self.graph.report()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, other, verbose)"]
    /// Return rendered textual report about the graph overlaps.
    ///
    /// Parameters
    /// ----------
    /// other: EnsmallenGraph,
    ///     graph to create overlap report with.
    /// verbose: Optional[bool],
    ///     Whether to shor the loading bars.
    ///
    pub fn overlap_textual_report(
        &self,
        other: &EnsmallenGraph,
        verbose: Option<bool>,
    ) -> PyResult<String> {
        pe!(self.graph.overlap_textual_report(&other.graph, verbose))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_id)"]
    /// Return human-readable html report of the given node.
    ///
    /// The report, by default, is rendered using html.
    ///
    /// Parameters
    /// ----------
    /// node_id: int,
    ///     Whether to show a loading bar in graph operations.
    ///
    pub fn get_node_report_from_node_id(&self, node_id: NodeT) -> PyResult<String> {
        pe!(self.graph.get_node_report_from_node_id(node_id))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_name)"]
    /// Return human-readable html report of the given node.
    ///
    /// The report, by default, is rendered using html.
    ///
    /// Parameters
    /// ----------
    /// node_name: str,
    ///     Whether to show a loading bar in graph operations.
    ///
    pub fn get_node_report_from_node_name(&self, node_name: &str) -> PyResult<String> {
        pe!(self.graph.get_node_report_from_node_name(node_name))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return html short textual report of the graph.
    ///
    /// TODO! Add reports on triangles
    /// TODO! Add reports on connected components
    /// TODO! Add reports on various node metrics
    /// TODO! Add reports on various edge metrics
    /// NOTE! Most of the above TODOs will require first to implement the
    /// support for the fast computation of the inbound edges in a directed
    /// graphs
    pub fn textual_report(&self) -> String {
        self.graph.textual_report()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_name_mapping, node_type_name_mapping, node_type_names_mapping, edge_type_name_mapping, verbose)"]
    /// Replace given node, node type and edge type names.
    ///
    /// Parameters
    /// ----------
    /// node_name_mapping: Optional[Dict[str, str]],
    ///     The node names to replace.
    /// node_type_name_mapping: Optional[Dict[str, str]],
    ///     The node type names to replace.
    /// node_type_names_mapping: Optional[Dict[Optional[List[str]], Optional[List[str]]]],
    ///     The node type names (as vectors) to replace.
    /// edge_type_name_mapping: Optional[Dict[Optional[str], Optional[str]]],
    ///     The edge type names to replace.
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given node names mapping would lead to nodes duplication.
    ///
    pub fn replace(
        &self,
        node_name_mapping: Option<HashMap<String, String>>,
        node_type_name_mapping: Option<HashMap<String, String>>,
        node_type_names_mapping: Option<HashMap<Option<Vec<String>>, Option<Vec<String>>>>,
        edge_type_name_mapping: Option<HashMap<Option<String>, Option<String>>>,
        verbose: Option<bool>,
    ) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(self.graph.replace(
                node_name_mapping,
                node_type_name_mapping,
                node_type_names_mapping,
                edge_type_name_mapping,
                verbose
            ))?,
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_type_names, verbose)"]
    /// Replace unknown node types with given node type.
    ///
    /// Parameters
    /// ----------
    /// node_type_names: List[str],
    ///     The node types to replace the unknown with.
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar.
    ///
    pub fn replace_unknown_node_types_with_node_type_name(
        &self,
        node_type_names: Vec<String>,
        verbose: Option<bool>,
    ) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(self
                .graph
                .replace_unknown_node_types_with_node_type_name(node_type_names, verbose))?,
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_type_name, verbose)"]
    /// Replace unknown edge types with given edge type name.
    ///
    /// Parameters
    /// ----------
    /// edge_type_name: str,
    ///     The edge type name to replace the unknown with.
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar.
    ///
    pub fn replace_unknown_edge_types_with_edge_type_name(
        &self,
        edge_type_name: String,
        verbose: Option<bool>,
    ) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(self
                .graph
                .replace_unknown_edge_types_with_edge_type_name(edge_type_name, verbose))?,
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_ids_to_keep, node_ids_to_filter, node_type_ids_to_keep, node_type_ids_to_filter, node_type_id_to_keep, node_type_id_to_filter, edge_ids_to_keep, edge_ids_to_filter, edge_node_ids_to_keep, edge_node_ids_to_filter, edge_type_ids_to_keep, edge_type_ids_to_filter, min_edge_weight, max_edge_weight, filter_singleton_nodes, filter_singleton_nodes_with_selfloop, filter_selfloops, filter_parallel_edges, verbose)"]
    /// Returns a **NEW** Graph that does not have the required attributes.
    ///
    /// Parameters
    /// ----------
    /// node_ids_to_keep: Optional[List[int]],
    ///     List of node IDs to keep during filtering.
    /// node_ids_to_filter: Optional[List[int]],
    ///     List of node IDs to remove during filtering.
    /// node_type_ids_to_keep: Optional[List[Optional[List[int]]]],
    ///     List of node type IDs to keep during filtering. The node types must match entirely the given node types vector provided.
    /// node_type_ids_to_filter: Optional[List[Optional[List[int]]]],
    ///     List of node type IDs to remove during filtering. The node types must match entirely the given node types vector provided.
    /// node_type_id_to_keep: Optional[List[Optional[int]]],
    ///     List of node type IDs to keep during filtering. Any of node types must match with one of the node types given.
    /// node_type_id_to_filter: Optional[List[Optional[int]]],
    ///     List of node type IDs to remove during filtering. Any of node types must match with one of the node types given.
    /// edge_ids_to_keep: Optional[List[int]],
    ///     List of edge IDs to keep during filtering.
    /// edge_ids_to_filter: Optional[List[int]],
    ///     List of edge IDs to remove during filtering.
    /// edge_node_ids_to_keep: Optional[List[Tuple[int, int]]],
    ///     List of tuple of node IDs to keep during filtering.
    /// edge_node_ids_to_filter: Optional[List[Tuple[int, int]]],
    ///     List of tuple of node IDs to remove during filtering.
    /// edge_type_ids_to_keep: Optional[List[Optional[int]]],
    ///     List of edge type IDs to keep during filtering.
    /// edge_type_ids_to_filter: Optional[List[Optional[int]]],
    ///     List of edge type IDs to remove during filtering.
    /// min_edge_weight: Optional[float],
    ///     Minimum edge weight. Values lower than this are removed.
    /// max_edge_weight: Optional[float],
    ///     Maximum edge weight. Values higher than this are removed.
    /// filter_singleton_nodes: Optional[bool],
    ///     Whether to filter out singleton nodes.
    /// filter_singleton_nodes_with_selfloop: Optional[bool],
    ///     Whether to filter out singleton nodes with selfloops.
    /// filter_selfloops: Optional[bool],
    ///     Whether to filter out selfloops.
    /// filter_parallel_edges: Optional[bool],
    ///     Whether to filter out parallel edges.
    /// verbose: Optional[bool],
    ///     Whether to show loading bar while building the graphs.
    ///
    pub fn filter_from_ids(
        &self,
        node_ids_to_keep: Option<Vec<NodeT>>,
        node_ids_to_filter: Option<Vec<NodeT>>,
        node_type_ids_to_keep: Option<Vec<Option<Vec<NodeTypeT>>>>,
        node_type_ids_to_filter: Option<Vec<Option<Vec<NodeTypeT>>>>,
        node_type_id_to_keep: Option<Vec<Option<NodeTypeT>>>,
        node_type_id_to_filter: Option<Vec<Option<NodeTypeT>>>,
        edge_ids_to_keep: Option<Vec<EdgeT>>,
        edge_ids_to_filter: Option<Vec<EdgeT>>,
        edge_node_ids_to_keep: Option<Vec<(NodeT, NodeT)>>,
        edge_node_ids_to_filter: Option<Vec<(NodeT, NodeT)>>,
        edge_type_ids_to_keep: Option<Vec<Option<EdgeTypeT>>>,
        edge_type_ids_to_filter: Option<Vec<Option<EdgeTypeT>>>,
        min_edge_weight: Option<WeightT>,
        max_edge_weight: Option<WeightT>,
        filter_singleton_nodes: Option<bool>,
        filter_singleton_nodes_with_selfloop: Option<bool>,
        filter_selfloops: Option<bool>,
        filter_parallel_edges: Option<bool>,
        verbose: Option<bool>,
    ) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(self.graph.filter_from_ids(
                node_ids_to_keep,
                node_ids_to_filter,
                node_type_ids_to_keep,
                node_type_ids_to_filter,
                node_type_id_to_keep,
                node_type_id_to_filter,
                edge_ids_to_keep,
                edge_ids_to_filter,
                edge_node_ids_to_keep,
                edge_node_ids_to_filter,
                edge_type_ids_to_keep,
                edge_type_ids_to_filter,
                min_edge_weight,
                max_edge_weight,
                filter_singleton_nodes,
                filter_singleton_nodes_with_selfloop,
                filter_selfloops,
                filter_parallel_edges,
                verbose
            ))?,
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_names_to_keep, node_names_to_filter, node_type_names_to_keep, node_type_names_to_filter, node_type_name_to_keep, node_type_name_to_filter, edge_node_names_to_keep, edge_node_names_to_filter, edge_type_names_to_keep, edge_type_names_to_filter, min_edge_weight, max_edge_weight, filter_singleton_nodes, filter_singleton_nodes_with_selfloop, filter_selfloops, filter_parallel_edges, verbose)"]
    /// Returns a **NEW** Graph that does not have the required attributes.
    ///
    /// Parameters
    /// ----------
    /// node_names_to_keep: Optional[List[str]],
    ///     List of node names to keep during filtering.
    /// node_names_to_filter: Optional[List[str]],
    ///     List of node names to remove during filtering.
    /// node_type_names_to_keep: Optional[List[Optional[List[str]]]],
    ///     List of node type names to keep during filtering. The node types must match entirely the given node types vector provided.
    /// node_type_names_to_filter: Optional[List[Optional[List[str]]]],
    ///     List of node type names to remove during filtering. The node types must match entirely the given node types vector provided.
    /// node_type_name_to_keep: Optional[List[Optional[str]]],
    ///     List of node type name to keep during filtering. Any of node types must match with one of the node types given.
    /// node_type_name_to_filter: Optional[List[Optional[str]]],
    ///     List of node type name to remove during filtering. Any of node types must match with one of the node types given.
    /// edge_node_names_to_keep: Optional[List[Tuple[str, str]]],
    ///     List of tuple of node names to keep during filtering.
    /// edge_node_names_to_filter: Optional[List[Tuple[str, str]]],
    ///     List of tuple of node names to remove during filtering.
    /// edge_type_names_to_keep: Optional[List[Optional[str]]],
    ///     List of edge type names to keep during filtering.
    /// edge_type_names_to_filter: Optional[List[Optional[str]]],
    ///     List of edge type names to remove during filtering.
    /// min_edge_weight: Optional[float],
    ///     Minimum edge weight. Values lower than this are removed.
    /// max_edge_weight: Optional[float],
    ///     Maximum edge weight. Values higher than this are removed.
    /// filter_singleton_nodes: Optional[bool],
    ///     Whether to filter out singletons.
    /// filter_singleton_nodes_with_selfloop: Optional[bool],
    ///     Whether to filter out singleton nodes with selfloops.
    /// filter_selfloops: Optional[bool],
    ///     Whether to filter out selfloops.
    /// filter_parallel_edges: Optional[bool],
    ///     Whether to filter out parallel edges.
    /// verbose: Optional[bool],
    ///     Whether to show loading bar while building the graphs.
    ///
    pub fn filter_from_names(
        &self,
        node_names_to_keep: Option<Vec<&str>>,
        node_names_to_filter: Option<Vec<&str>>,
        node_type_names_to_keep: Option<Vec<Option<Vec<&str>>>>,
        node_type_names_to_filter: Option<Vec<Option<Vec<&str>>>>,
        node_type_name_to_keep: Option<Vec<Option<String>>>,
        node_type_name_to_filter: Option<Vec<Option<String>>>,
        edge_node_names_to_keep: Option<Vec<(&str, &str)>>,
        edge_node_names_to_filter: Option<Vec<(&str, &str)>>,
        edge_type_names_to_keep: Option<Vec<Option<String>>>,
        edge_type_names_to_filter: Option<Vec<Option<String>>>,
        min_edge_weight: Option<WeightT>,
        max_edge_weight: Option<WeightT>,
        filter_singleton_nodes: Option<bool>,
        filter_singleton_nodes_with_selfloop: Option<bool>,
        filter_selfloops: Option<bool>,
        filter_parallel_edges: Option<bool>,
        verbose: Option<bool>,
    ) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(self.graph.filter_from_names(
                node_names_to_keep,
                node_names_to_filter,
                node_type_names_to_keep,
                node_type_names_to_filter,
                node_type_name_to_keep,
                node_type_name_to_filter,
                edge_node_names_to_keep,
                edge_node_names_to_filter,
                edge_type_names_to_keep,
                edge_type_names_to_filter,
                min_edge_weight,
                max_edge_weight,
                filter_singleton_nodes,
                filter_singleton_nodes_with_selfloop,
                filter_selfloops,
                filter_parallel_edges,
                verbose
            ))?,
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, verbose)"]
    /// Returns new graph without unknown node types and relative nodes.
    ///
    /// Note that this method will remove ALL nodes labeled with unknown node
    /// type!
    ///
    /// Parameters
    /// ----------
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar while building the graph.
    ///
    pub fn drop_unknown_node_types(&self, verbose: Option<bool>) -> EnsmallenGraph {
        EnsmallenGraph {
            graph: self.graph.drop_unknown_node_types(verbose),
        }
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, verbose)"]
    /// Returns new graph without unknown edge types and relative edges.
    ///
    /// Note that this method will remove ALL edges labeled with unknown edge
    /// type!
    ///
    /// Parameters
    /// ----------
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar while building the graph.
    ///
    pub fn drop_unknown_edge_types(&self, verbose: Option<bool>) -> EnsmallenGraph {
        EnsmallenGraph {
            graph: self.graph.drop_unknown_edge_types(verbose),
        }
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, verbose)"]
    /// Returns new graph without singleton nodes.
    ///
    /// A node is singleton when does not have neither incoming or outgoing edges.
    ///
    /// Parameters
    /// ----------
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar while building the graph.
    ///
    pub fn drop_singleton_nodes(&self, verbose: Option<bool>) -> EnsmallenGraph {
        EnsmallenGraph {
            graph: self.graph.drop_singleton_nodes(verbose),
        }
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, verbose)"]
    /// Returns new graph without singleton nodes with selfloops.
    ///
    /// A node is singleton with selfloop when does not have neither incoming or outgoing edges.
    ///
    /// Parameters
    /// ----------
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar while building the graph.
    ///
    pub fn drop_singleton_nodes_with_selfloops(&self, verbose: Option<bool>) -> EnsmallenGraph {
        EnsmallenGraph {
            graph: self.graph.drop_singleton_nodes_with_selfloops(verbose),
        }
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, verbose)"]
    /// Returns new graph without disconnected nodes.
    ///
    /// A disconnected node is a node with no connection to any other node.
    ///
    /// Parameters
    /// ----------
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar while building the graph.
    ///
    pub fn drop_disconnected_nodes(&self, verbose: Option<bool>) -> EnsmallenGraph {
        EnsmallenGraph {
            graph: self.graph.drop_disconnected_nodes(verbose),
        }
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, verbose)"]
    /// Returns new graph without selfloops.
    ///
    /// Parameters
    /// ----------
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar while building the graph.
    ///
    pub fn drop_selfloops(&self, verbose: Option<bool>) -> EnsmallenGraph {
        EnsmallenGraph {
            graph: self.graph.drop_selfloops(verbose),
        }
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, verbose)"]
    /// Returns new graph without parallel edges.
    ///
    /// Parameters
    /// ----------
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar while building the graph.
    ///
    pub fn drop_parallel_edges(&self, verbose: Option<bool>) -> EnsmallenGraph {
        EnsmallenGraph {
            graph: self.graph.drop_parallel_edges(verbose),
        }
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, random_state, undesired_edge_types, verbose)"]
    /// Returns set of edges composing a spanning tree and connected components.
    ///
    /// The spanning tree is NOT minimal.
    /// The given random_state is NOT the root of the tree.
    ///
    /// This method, additionally, allows for undesired edge types to be
    /// used to build the spanning tree only in extremis when it is utterly
    /// necessary in order to complete the spanning arborescence.
    ///
    /// The quintuple returned contains:
    /// - Set of the edges used in order to build the spanning arborescence.
    /// - Vector of the connected component of each node.
    /// - Number of connected components.
    /// - Minimum component size.
    /// - Maximum component size.
    ///
    /// Parameters
    /// ----------
    /// random_state: Optional[int],
    ///     The random_state to use for the holdout,
    /// undesired_edge_types: Optional[Set[Optional[int]]],
    ///     Which edge types id to try to avoid.
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar or not.
    ///
    pub fn random_spanning_arborescence_kruskal(
        &self,
        random_state: Option<EdgeT>,
        undesired_edge_types: Option<HashSet<Option<EdgeTypeT>>>,
        verbose: Option<bool>,
    ) -> (HashSet<(NodeT, NodeT)>, Vec<NodeT>, NodeT, NodeT, NodeT) {
        self.graph
            .random_spanning_arborescence_kruskal(random_state, undesired_edge_types, verbose)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, verbose)"]
    /// Returns consistent spanning arborescence using Kruskal.
    ///
    /// The spanning tree is NOT minimal.
    ///
    /// The quintuple returned contains:
    /// - Set of the edges used in order to build the spanning arborescence.
    /// - Vector of the connected component of each node.
    /// - Number of connected components.
    /// - Minimum component size.
    /// - Maximum component size.
    ///
    /// Parameters
    /// ----------
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar or not.
    ///
    pub fn spanning_arborescence_kruskal(
        &self,
        verbose: Option<bool>,
    ) -> (HashSet<(NodeT, NodeT)>, Vec<NodeT>, NodeT, NodeT, NodeT) {
        self.graph.spanning_arborescence_kruskal(verbose)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, verbose)"]
    /// Compute the connected components building in parallel a spanning tree using [bader's algorithm](https://www.sciencedirect.com/science/article/abs/pii/S0743731505000882).
    ///
    /// **This works only for undirected graphs.**
    ///
    /// This method is **not thread save and not deterministic** but by design of the algorithm this
    /// shouldn't matter but if we will encounter non-detemristic bugs here is where we want to look.
    ///
    /// The returned quadruple contains:
    /// - Vector of the connected component for each node.
    /// - Number of connected components.
    /// - Minimum connected component size.
    /// - Maximum connected component size.
    ///
    /// Parameters
    /// ----------
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar or not.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given graph is directed.
    /// ValueError
    ///     If the system configuration does not allow for the creation of the thread pool.
    ///
    pub fn connected_components(
        &self,
        verbose: Option<bool>,
    ) -> PyResult<(Vec<NodeT>, NodeT, NodeT, NodeT)> {
        pe!(self.graph.connected_components(verbose))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, vector_sources, vector_destinations, vector_cumulative_node_degrees)"]
    /// Enable extra perks that buys you time as you accept to spend more memory.
    ///
    /// Parameters
    /// ----------
    /// vector_sources: Optional[bool],
    ///     Whether to cache sources into a vector for faster walks.
    /// vector_destinations: Optional[bool],
    ///     Whether to cache destinations into a vector for faster walks.
    /// vector_cumulative_node_degrees: Optional[bool],
    ///     Whether to cache cumulative_node_degrees into a vector for faster walks.
    ///
    pub fn enable(
        &mut self,
        vector_sources: Option<bool>,
        vector_destinations: Option<bool>,
        vector_cumulative_node_degrees: Option<bool>,
    ) -> PyResult<()> {
        pe!(self.graph.enable(
            vector_sources,
            vector_destinations,
            vector_cumulative_node_degrees
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Disable all extra perks, reducing memory impact but incresing time requirements
    pub fn disable_all(&mut self) {
        self.graph.disable_all();
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns 2-approximated verted cover set using greedy algorithm.
    pub fn approximated_vertex_cover_set(&self) -> HashSet<NodeT> {
        self.graph.approximated_vertex_cover_set()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, features, iterations, maximal_distance, k1, b, include_central_node, verbose)"]
    /// Returns okapi node features propagation within given maximal distance.
    ///
    /// Parameters
    /// ----------
    /// features: List[Optional[List[float]]],
    ///     The features to propagate. Use None to represent eventual unknown features.
    /// iterations: Optional[int],
    ///     The number of iterations to execute. By default one.
    /// maximal_distance: Optional[int],
    ///     The distance to consider for the cooccurrences. The default value is 3.
    /// k1: Optional[float],
    ///     The k1 parameter from okapi. Tipicaly between 1.2 and 2.0. It can be seen as a smoothing.
    /// b: Optional[float],
    ///     The b parameter from okapi. Tipicaly 0.75.
    /// include_central_node: Optional[bool],
    ///     Whether to include the central node. By default true.
    /// verbose: Optional[bool],
    ///     Whether to show loading bar.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn get_okapi_bm25_node_feature_propagation(
        &self,
        features: Vec<Vec<f64>>,
        iterations: Option<usize>,
        maximal_distance: Option<usize>,
        k1: Option<f64>,
        b: Option<f64>,
        include_central_node: Option<bool>,
        verbose: Option<bool>,
    ) -> PyResult<Py<PyArray2<f64>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_2d!(
            gil,
            pe!(self.graph.get_okapi_bm25_node_feature_propagation(
                features,
                iterations,
                maximal_distance,
                k1,
                b,
                include_central_node,
                verbose
            ))?,
            f64
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, iterations, maximal_distance, k1, b, verbose)"]
    /// Returns okapi node label propagation within given maximal distance.
    ///
    /// Parameters
    /// ----------
    /// iterations: Optional[int],
    ///     The number of iterations to execute. By default one.
    /// maximal_distance: Optional[int],
    ///     The distance to consider for the cooccurrences. The default value is 3.
    /// k1: Optional[float],
    ///     The k1 parameter from okapi. Tipicaly between 1.2 and 2.0. It can be seen as a smoothing.
    /// b: Optional[float],
    ///     The b parameter from okapi. Tipicaly 0.75.
    /// verbose: Optional[bool],
    ///     Whether to show loading bar.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn get_okapi_bm25_node_label_propagation(
        &self,
        iterations: Option<usize>,
        maximal_distance: Option<usize>,
        k1: Option<f64>,
        b: Option<f64>,
        verbose: Option<bool>,
    ) -> PyResult<Py<PyArray2<f64>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_2d!(
            gil,
            pe!(self.graph.get_okapi_bm25_node_label_propagation(
                iterations,
                maximal_distance,
                k1,
                b,
                verbose
            ))?,
            f64
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return if graph has name that is not the default one.
    ///
    /// TODO: use a default for the default graph name
    pub fn has_default_graph_name(&self) -> bool {
        self.graph.has_default_graph_name()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return if the graph has any nodes.
    pub fn has_nodes(&self) -> bool {
        self.graph.has_nodes()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return if the graph has any edges.
    pub fn has_edges(&self) -> bool {
        self.graph.has_edges()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return whether the graph has trap nodes.
    pub fn has_trap_nodes(&self) -> bool {
        self.graph.has_trap_nodes()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns boolean representing if graph is directed.
    pub fn is_directed(&self) -> bool {
        self.graph.is_directed()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns boolean representing whether graph has weights.
    pub fn has_edge_weights(&self) -> bool {
        self.graph.has_edge_weights()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns whether graph has weights that can represent probabilities.
    pub fn has_edge_weights_representing_probabilities(&self) -> PyResult<bool> {
        pe!(self.graph.has_edge_weights_representing_probabilities())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns whether a graph has one or more weighted singleton nodes.
    ///
    /// A weighted singleton node is a node whose weighted node degree is 0.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not contain edge weights.
    ///
    pub fn has_weighted_singleton_nodes(&self) -> PyResult<bool> {
        pe!(self.graph.has_weighted_singleton_nodes())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns whether the graph has constant weights.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not contain edge weights.
    ///
    pub fn has_constant_edge_weights(&self) -> PyResult<bool> {
        pe!(self.graph.has_constant_edge_weights())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns boolean representing whether graph has negative weights.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not contain weights.
    ///
    pub fn has_negative_edge_weights(&self) -> PyResult<bool> {
        pe!(self.graph.has_negative_edge_weights())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns boolean representing whether graph has edge types.
    pub fn has_edge_types(&self) -> bool {
        self.graph.has_edge_types()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns boolean representing if graph has self-loops.
    pub fn has_selfloops(&self) -> bool {
        self.graph.has_selfloops()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns boolean representing if nodes which are nor singletons nor
    /// singletons with selfloops.
    pub fn has_disconnected_nodes(&self) -> bool {
        self.graph.has_disconnected_nodes()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns boolean representing if graph has singletons.
    pub fn has_singleton_nodes(&self) -> bool {
        self.graph.has_singleton_nodes()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns boolean representing if graph has singletons
    pub fn has_singleton_nodes_with_selfloops(&self) -> bool {
        self.graph.has_singleton_nodes_with_selfloops()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, verbose)"]
    /// Returns whether the graph is connected.
    ///
    /// Parameters
    /// ----------
    /// verbose: Optional[bool],
    ///     Whether to show the loading bar while computing the connected components, if necessary.
    ///
    pub fn is_connected(&self, verbose: Option<bool>) -> bool {
        self.graph.is_connected(verbose)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns boolean representing if graph has node types
    pub fn has_node_types(&self) -> bool {
        self.graph.has_node_types()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns boolean representing if graph has multilabel node types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn has_multilabel_node_types(&self) -> PyResult<bool> {
        pe!(self.graph.has_multilabel_node_types())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns whether there are unknown node types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn has_unknown_node_types(&self) -> PyResult<bool> {
        pe!(self.graph.has_unknown_node_types())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns whether there are known node types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn has_known_node_types(&self) -> PyResult<bool> {
        pe!(self.graph.has_known_node_types())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns whether there are unknown edge types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn has_unknown_edge_types(&self) -> PyResult<bool> {
        pe!(self.graph.has_unknown_edge_types())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns whether there are known edge types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    ///
    pub fn has_known_edge_types(&self) -> PyResult<bool> {
        pe!(self.graph.has_known_edge_types())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns whether the nodes have an homogenous node type.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn has_homogeneous_node_types(&self) -> PyResult<bool> {
        pe!(self.graph.has_homogeneous_node_types())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns whether the edges have an homogenous edge type.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    ///
    pub fn has_homogeneous_edge_types(&self) -> PyResult<bool> {
        pe!(self.graph.has_homogeneous_edge_types())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns whether there is at least singleton node type, that is a node type that only appears once.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn has_singleton_node_types(&self) -> PyResult<bool> {
        pe!(self.graph.has_singleton_node_types())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return whether the graph has any known node-related graph oddities
    pub fn has_node_oddities(&self) -> bool {
        self.graph.has_node_oddities()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return whether the graph has any known node type-related graph oddities.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn has_node_types_oddities(&self) -> PyResult<bool> {
        pe!(self.graph.has_node_types_oddities())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns whether there is at least singleton edge type, that is a edge type that only appears once.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    ///
    pub fn has_singleton_edge_types(&self) -> PyResult<bool> {
        pe!(self.graph.has_singleton_edge_types())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return whether the graph has any known edge type-related graph oddities.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    ///
    pub fn has_edge_types_oddities(&self) -> PyResult<bool> {
        pe!(self.graph.has_edge_types_oddities())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return if there are multiple edges between two node
    pub fn is_multigraph(&self) -> bool {
        self.graph.is_multigraph()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns whether the node IDs are sorted by decreasing outbound node degree.
    pub fn has_nodes_sorted_by_decreasing_outbound_node_degree(&self) -> bool {
        self.graph
            .has_nodes_sorted_by_decreasing_outbound_node_degree()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns whether the node IDs are sorted by increasing outbound node degree.
    pub fn has_nodes_sorted_by_increasing_outbound_node_degree(&self) -> bool {
        self.graph
            .has_nodes_sorted_by_increasing_outbound_node_degree()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, iterations, verbose)"]
    /// Returns graph to the i-th transitivity closure iteration.
    ///
    /// Parameters
    /// ----------
    /// iterations: Optional[int],
    ///     The number of iterations of the transitive closure to execute. If None, the complete transitive closure is computed.
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar while building the graph.
    ///
    pub fn get_transitive_closure(
        &self,
        iterations: Option<NodeT>,
        verbose: Option<bool>,
    ) -> EnsmallenGraph {
        EnsmallenGraph {
            graph: self.graph.get_transitive_closure(iterations, verbose),
        }
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, iterations, verbose)"]
    /// Returns graph with unweighted shortest paths computed up to the given depth.
    ///
    /// The returned graph will have no selfloops.
    ///
    /// Parameters
    /// ----------
    /// iterations: Optional[int],
    ///     The number of iterations of the transitive closure to execute. If None, the complete transitive closure is computed.
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar while building the graph.
    ///
    pub fn get_all_shortest_paths(
        &self,
        iterations: Option<NodeT>,
        verbose: Option<bool>,
    ) -> EnsmallenGraph {
        EnsmallenGraph {
            graph: self.graph.get_all_shortest_paths(iterations, verbose),
        }
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, iterations, use_edge_weights_as_probabilities, verbose)"]
    /// Returns graph with weighted shortest paths computed up to the given depth.
    ///
    /// The returned graph will have no selfloops.
    ///
    /// Parameters
    /// ----------
    /// iterations: Optional[int],
    ///     The number of iterations of the transitive closure to execute. If None, the complete transitive closure is computed.
    /// use_edge_weights_as_probabilities: Optional[bool],
    ///     Whether to treat the edge weights as probabilities.
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar while building the graph.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have weights.
    /// ValueError
    ///     If the graph contains negative weights.
    /// ValueError
    ///     If the user has asked for the weights to be treated as probabilities but the weights are not between 0 and 1.
    ///
    pub fn get_weighted_all_shortest_paths(
        &self,
        iterations: Option<NodeT>,
        use_edge_weights_as_probabilities: Option<bool>,
        verbose: Option<bool>,
    ) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(self.graph.get_weighted_all_shortest_paths(
                iterations,
                use_edge_weights_as_probabilities,
                verbose
            ))?,
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_id)"]
    /// Returns option with the weight of the given edge id.
    ///
    /// This method will raise a panic if the given edge ID is higher than
    /// the number of edges in the graph. Additionally, it will simply
    /// return None if there are no graph weights.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int,
    ///     The edge whose edge weight is to be returned.
    ///
    ///
    /// Safety
    /// ------
    /// If the given edge ID does not exists in the graph this method will panic.
    pub unsafe fn get_unchecked_edge_weight_from_edge_id(&self, edge_id: EdgeT) -> Option<WeightT> {
        self.graph.get_unchecked_edge_weight_from_edge_id(edge_id)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src, dst)"]
    /// Returns option with the weight of the given node ids.
    ///
    /// This method will raise a panic if the given node IDs are higher than
    /// the number of nodes in the graph.
    ///
    /// Parameters
    /// ----------
    /// src: int,
    ///     The source node ID.
    /// dst: int,
    ///     The destination node ID.
    ///
    ///
    /// Safety
    /// ------
    /// If either of the two given node IDs does not exists in the graph.
    pub unsafe fn get_unchecked_edge_weight_from_node_ids(
        &self,
        src: NodeT,
        dst: NodeT,
    ) -> WeightT {
        self.graph.get_unchecked_edge_weight_from_node_ids(src, dst)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_name)"]
    /// Returns node id from given node name raising a panic if used unproperly.
    ///
    /// Parameters
    /// ----------
    /// node_name: str,
    ///     The node name whose node ID is to be returned.
    ///
    ///
    /// Safety
    /// ------
    /// If the given node name does not exists in the considered graph the method will panic.
    pub unsafe fn get_unchecked_node_id_from_node_name(&self, node_name: &str) -> NodeT {
        self.graph.get_unchecked_node_id_from_node_name(node_name)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_type_name)"]
    /// Return edge type ID corresponding to the given edge type name.
    ///
    /// Parameters
    /// ----------
    /// edge_type_name: str,
    ///     The edge type name whose edge type ID is to be returned.
    ///
    ///
    /// Safety
    /// ------
    /// If the given edge type name does not exists in the considered graph the method will panic.
    pub unsafe fn get_unchecked_edge_type_id_from_edge_type_name(
        &self,
        edge_type_name: &str,
    ) -> Option<EdgeTypeT> {
        self.graph
            .get_unchecked_edge_type_id_from_edge_type_name(edge_type_name)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_type_id)"]
    /// Return edge type ID corresponding to the given edge type name
    /// raising panic if edge type ID does not exists in current graph.
    ///
    /// Parameters
    /// ----------
    /// edge_type_id: Optional[int],
    ///     The edge type naIDme whose edge type name is to be returned.
    ///
    ///
    /// Safety
    /// ------
    /// If the given edge type ID does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_edge_type_name_from_edge_type_id(
        &self,
        edge_type_id: Option<EdgeTypeT>,
    ) -> Option<String> {
        self.graph
            .get_unchecked_edge_type_name_from_edge_type_id(edge_type_id)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_type)"]
    /// Return number of edges of the given edge type without checks.
    ///
    /// Parameters
    /// ----------
    /// edge_type: Optional[int],
    ///     The edge type to retrieve count of.
    ///
    ///
    /// Safety
    /// ------
    /// If the given edge type ID does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_edge_count_from_edge_type_id(
        &self,
        edge_type: Option<EdgeTypeT>,
    ) -> EdgeT {
        self.graph
            .get_unchecked_edge_count_from_edge_type_id(edge_type)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src, dst, edge_type)"]
    /// Return edge ID without any checks for given tuple of nodes and edge type.
    ///
    /// Parameters
    /// ----------
    /// src: int,
    ///     Source node of the edge.
    /// dst: int,
    ///     Destination node of the edge.
    /// edge_type: Optional[int],
    ///     Edge Type of the edge.
    ///
    ///
    /// Safety
    /// ------
    /// If the given node IDs or edge type does not exists in the graph this method will panic.
    pub unsafe fn get_unchecked_edge_id_from_node_ids_and_edge_type_id(
        &self,
        src: NodeT,
        dst: NodeT,
        edge_type: Option<EdgeTypeT>,
    ) -> EdgeT {
        self.graph
            .get_unchecked_edge_id_from_node_ids_and_edge_type_id(src, dst, edge_type)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src, dst)"]
    /// Return range of outbound edges IDs for all the edges bewteen the given
    /// source and destination nodes.
    /// This operation is meaningfull only in a multigraph.
    ///
    /// Parameters
    /// ----------
    /// src: int,
    ///     Source node.
    /// dst: int,
    ///     Destination node.
    ///
    ///
    /// Safety
    /// ------
    /// If the given node type IDs do not exist in the graph this method will panic.
    pub unsafe fn get_unchecked_minmax_edge_ids_from_node_ids(
        &self,
        src: NodeT,
        dst: NodeT,
    ) -> (EdgeT, EdgeT) {
        self.graph
            .get_unchecked_minmax_edge_ids_from_node_ids(src, dst)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_id)"]
    /// Returns node IDs corresponding to given edge ID.
    ///
    /// The method will panic if the given edge ID does not exists in the
    /// current graph instance.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int,
    ///     The edge ID whose source and destination node IDs are to e retrieved.
    ///
    ///
    /// Safety
    /// ------
    /// If the given edge ID does not exist in the current graph the method will raise a panic.
    pub unsafe fn get_unchecked_node_ids_from_edge_id(&self, edge_id: EdgeT) -> (NodeT, NodeT) {
        self.graph.get_unchecked_node_ids_from_edge_id(edge_id)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_id)"]
    /// Returns node names corresponding to given edge ID.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int,
    ///     The edge ID whose source and destination node IDs are to e retrieved.
    ///
    ///
    /// Safety
    /// ------
    /// If the given edge ID does not exist in the current graph the method will raise a panic.
    pub unsafe fn get_unchecked_node_names_from_edge_id(&self, edge_id: EdgeT) -> (String, String) {
        self.graph.get_unchecked_node_names_from_edge_id(edge_id)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_id)"]
    /// Returns the source of given edge id without making any boundary check.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int,
    ///     The edge ID whose source is to be retrieved.
    ///
    ///
    /// Safety
    /// ------
    /// If the given edge ID does not exist in the current graph the method will cause an out of bounds.
    pub unsafe fn get_unchecked_source_node_id_from_edge_id(&self, edge_id: EdgeT) -> NodeT {
        self.graph
            .get_unchecked_source_node_id_from_edge_id(edge_id)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_id)"]
    /// Returns the destination of given edge id without making any boundary check.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int,
    ///     The edge ID whose destination is to be retrieved.
    ///
    ///
    /// Safety
    /// ------
    /// If the given edge ID does not exist in the current graph the method will cause an out of bounds.
    pub unsafe fn get_unchecked_destination_node_id_from_edge_id(&self, edge_id: EdgeT) -> NodeT {
        self.graph
            .get_unchecked_destination_node_id_from_edge_id(edge_id)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_id)"]
    /// Returns source node ID corresponding to given edge ID.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int,
    ///     The edge ID whose source node ID is to be retrieved.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given edge ID does not exist in the current graph.
    ///
    pub fn get_source_node_id_from_edge_id(&self, edge_id: EdgeT) -> PyResult<NodeT> {
        pe!(self.graph.get_source_node_id_from_edge_id(edge_id))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_id)"]
    /// Returns destination node ID corresponding to given edge ID.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int,
    ///     The edge ID whose destination node ID is to be retrieved.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given edge ID does not exist in the current graph.
    ///
    pub fn get_destination_node_id_from_edge_id(&self, edge_id: EdgeT) -> PyResult<NodeT> {
        pe!(self.graph.get_destination_node_id_from_edge_id(edge_id))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_id)"]
    /// Returns source node name corresponding to given edge ID.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int,
    ///     The edge ID whose source node name is to be retrieved.
    ///
    ///
    /// Safety
    /// ------
    /// If the given edge ID does not exist in the current graph the method will raise a panic.
    pub unsafe fn get_unchecked_source_node_name_from_edge_id(&self, edge_id: EdgeT) -> String {
        self.graph
            .get_unchecked_source_node_name_from_edge_id(edge_id)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_id)"]
    /// Returns destination node name corresponding to given edge ID.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int,
    ///     The edge ID whose destination node name is to be retrieved.
    ///
    ///
    /// Safety
    /// ------
    /// If the given edge ID does not exist in the current graph the method will raise a panic.
    pub unsafe fn get_unchecked_destination_node_name_from_edge_id(
        &self,
        edge_id: EdgeT,
    ) -> String {
        self.graph
            .get_unchecked_destination_node_name_from_edge_id(edge_id)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_id)"]
    /// Returns source node name corresponding to given edge ID.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int,
    ///     The edge ID whose source node name is to be retrieved.
    ///
    ///
    /// Raises
    /// -------
    ///
    pub fn get_source_node_name_from_edge_id(&self, edge_id: EdgeT) -> PyResult<String> {
        pe!(self.graph.get_source_node_name_from_edge_id(edge_id))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_id)"]
    /// Returns destination node name corresponding to given edge ID.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int,
    ///     The edge ID whose destination node name is to be retrieved.
    ///
    ///
    /// Raises
    /// -------
    ///
    pub fn get_destination_node_name_from_edge_id(&self, edge_id: EdgeT) -> PyResult<String> {
        pe!(self.graph.get_destination_node_name_from_edge_id(edge_id))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_id)"]
    /// Returns node names corresponding to given edge ID.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int,
    ///     The edge ID whose source and destination node IDs are to e retrieved.
    ///
    pub fn get_node_names_from_edge_id(&self, edge_id: EdgeT) -> PyResult<(String, String)> {
        pe!(self.graph.get_node_names_from_edge_id(edge_id))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_id)"]
    /// Returns node names corresponding to given edge ID.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int,
    ///     The edge ID whose source and destination node IDs are to e retrieved.
    ///
    pub fn get_node_ids_from_edge_id(&self, edge_id: EdgeT) -> PyResult<(NodeT, NodeT)> {
        pe!(self.graph.get_node_ids_from_edge_id(edge_id))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src, dst)"]
    /// Returns edge ID corresponding to given source and destination node IDs.
    ///
    /// The method will panic if the given source and destination node IDs do
    /// not correspond to an edge in this graph instance.
    ///
    /// Parameters
    /// ----------
    /// src: int,
    ///     The source node ID.
    /// dst: int,
    ///     The destination node ID.
    ///
    ///
    /// Safety
    /// ------
    /// If any of the given node IDs do not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_edge_id_from_node_ids(&self, src: NodeT, dst: NodeT) -> EdgeT {
        self.graph.get_unchecked_edge_id_from_node_ids(src, dst)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src, dst)"]
    /// Returns edge ID corresponding to given source and destination node IDs.
    ///
    /// Parameters
    /// ----------
    /// src: int,
    ///     The source node ID.
    /// dst: int,
    ///     The destination node ID.
    ///
    pub fn get_edge_id_from_node_ids(&self, src: NodeT, dst: NodeT) -> PyResult<EdgeT> {
        pe!(self.graph.get_edge_id_from_node_ids(src, dst))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, source_id)"]
    /// Returns edge ID corresponding to given source and destination node IDs.
    ///
    /// Parameters
    /// ----------
    /// source_id: int,
    ///     The source node ID.
    ///
    ///
    /// Safety
    /// ------
    /// If the given source node ID does not exist in the current graph the method will panic.
    pub unsafe fn get_unchecked_unique_source_node_id(&self, source_id: NodeT) -> NodeT {
        self.graph.get_unchecked_unique_source_node_id(source_id)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_id)"]
    /// Return the src, dst, edge type of a given edge ID.
    ///
    /// This method will raise a panic when an improper configuration is used.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int,
    ///     The edge ID whose source, destination and edge type are to be retrieved.
    ///
    ///
    /// Safety
    /// ------
    /// If the given edge ID does not exist in the current graph the method will raise a panic.
    pub unsafe fn get_unchecked_node_ids_and_edge_type_id_from_edge_id(
        &self,
        edge_id: EdgeT,
    ) -> (NodeT, NodeT, Option<EdgeTypeT>) {
        self.graph
            .get_unchecked_node_ids_and_edge_type_id_from_edge_id(edge_id)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_id)"]
    /// Return the src, dst, edge type of a given edge ID.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int,
    ///     The edge ID whose source, destination and edge type are to be retrieved.
    ///
    pub fn get_node_ids_and_edge_type_id_from_edge_id(
        &self,
        edge_id: EdgeT,
    ) -> PyResult<(NodeT, NodeT, Option<EdgeTypeT>)> {
        pe!(self
            .graph
            .get_node_ids_and_edge_type_id_from_edge_id(edge_id))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_id)"]
    /// Return the src, dst, edge type and weight of a given edge ID.
    ///
    /// This method will raise a panic when an improper configuration is used.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int,
    ///     The edge ID whose source, destination, edge type and weight are to be retrieved.
    ///
    ///
    /// Safety
    /// ------
    /// If the given edge ID does not exist in the current graph the method will raise a panic.
    pub unsafe fn get_unchecked_node_ids_and_edge_type_id_and_edge_weight_from_edge_id(
        &self,
        edge_id: EdgeT,
    ) -> (NodeT, NodeT, Option<EdgeTypeT>, Option<WeightT>) {
        self.graph
            .get_unchecked_node_ids_and_edge_type_id_and_edge_weight_from_edge_id(edge_id)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_id)"]
    /// Return the src, dst, edge type and weight of a given edge ID.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int,
    ///     The edge ID whose source, destination, edge type and weight are to be retrieved.
    ///
    pub fn get_node_ids_and_edge_type_id_and_edge_weight_from_edge_id(
        &self,
        edge_id: EdgeT,
    ) -> PyResult<(NodeT, NodeT, Option<EdgeTypeT>, Option<WeightT>)> {
        pe!(self
            .graph
            .get_node_ids_and_edge_type_id_and_edge_weight_from_edge_id(edge_id))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, k)"]
    /// Return vector with unweighted top k central node Ids.
    ///
    /// If the k passed is bigger than the number of nodes this method will return
    /// all the nodes in the graph.
    ///
    /// Parameters
    /// ----------
    /// k: int,
    ///     Number of central nodes to extract.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given value k is zero.
    /// ValueError
    ///     If the graph has no nodes.
    ///
    pub fn get_top_k_central_node_ids(&self, k: NodeT) -> PyResult<Py<PyArray1<NodeT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self.graph.get_top_k_central_node_ids(k))?,
            NodeT
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, k)"]
    /// Return vector with weighted top k central node Ids.
    ///
    /// If the k passed is bigger than the number of nodes this method will return
    /// all the nodes in the graph.
    ///
    /// Parameters
    /// ----------
    /// k: int,
    ///     Number of central nodes to extract.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the current graph instance does not contain edge weights.
    /// ValueError
    ///     If the given value k is zero.
    ///
    pub fn get_weighted_top_k_central_node_ids(&self, k: NodeT) -> PyResult<Py<PyArray1<NodeT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self.graph.get_weighted_top_k_central_node_ids(k))?,
            NodeT
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_id)"]
    /// Returns the number of outbound neighbours of given node.
    ///
    /// Parameters
    /// ----------
    /// node_id: int,
    ///     Integer ID of the node.
    ///
    ///
    /// Safety
    /// ------
    /// If the given node ID does not exist in the current graph the method will raise a panic.
    pub unsafe fn get_unchecked_node_degree_from_node_id(&self, node_id: NodeT) -> NodeT {
        self.graph.get_unchecked_node_degree_from_node_id(node_id)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_id)"]
    /// Returns the weighted sum of outbound neighbours of given node.
    ///
    /// The method will panic if the given node id is higher than the number of
    /// nodes in the graph.
    ///
    /// Parameters
    /// ----------
    /// node_id: int,
    ///     Integer ID of the node.
    ///
    ///
    /// Safety
    /// ------
    /// If the given node ID does not exist in the current graph the method will raise a panic.
    pub unsafe fn get_unchecked_weighted_node_degree_from_node_id(&self, node_id: NodeT) -> f64 {
        self.graph
            .get_unchecked_weighted_node_degree_from_node_id(node_id)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_id)"]
    /// Returns the number of outbound neighbours of given node ID.
    ///
    /// Parameters
    /// ----------
    /// node_id: int,
    ///     Integer ID of the node.
    ///
    pub fn get_node_degree_from_node_id(&self, node_id: NodeT) -> PyResult<NodeT> {
        pe!(self.graph.get_node_degree_from_node_id(node_id))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_id)"]
    /// Returns the weighted sum of outbound neighbours of given node ID.
    ///
    /// Parameters
    /// ----------
    /// node_id: int,
    ///     Integer ID of the node.
    ///
    pub fn get_weighted_node_degree_from_node_id(&self, node_id: NodeT) -> PyResult<f64> {
        pe!(self.graph.get_weighted_node_degree_from_node_id(node_id))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_name)"]
    /// Returns the number of outbound neighbours of given node name.
    ///
    /// Parameters
    /// ----------
    /// node_name: str,
    ///     Integer ID of the node.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given node name does not exist in the graph.
    ///
    pub fn get_node_degree_from_node_name(&self, node_name: &str) -> PyResult<NodeT> {
        pe!(self.graph.get_node_degree_from_node_name(node_name))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, k)"]
    /// Return vector with top k central node names.
    ///
    /// Parameters
    /// ----------
    /// k: int,
    ///     Number of central nodes to extract.
    ///
    pub fn get_top_k_central_node_names(&self, k: NodeT) -> PyResult<Vec<String>> {
        pe!(self.graph.get_top_k_central_node_names(k))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_id)"]
    /// Returns option with vector of node types of given node.
    ///
    /// This method will panic if the given node ID is greater than
    /// the number of nodes in the graph.
    /// Furthermore, if the graph does NOT have node types, it will NOT
    /// return neither an error or a panic.
    ///
    /// Parameters
    /// ----------
    /// node_id: int,
    ///     node whose node type is to be returned.
    ///
    ///
    /// Safety
    /// ------
    /// Even though the method will return an option when the node types are
    ///  not available for the current graph, the behaviour is undefined.
    pub unsafe fn get_unchecked_node_type_id_from_node_id(
        &self,
        node_id: NodeT,
    ) -> Option<Vec<NodeTypeT>> {
        self.graph.get_unchecked_node_type_id_from_node_id(node_id)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_id)"]
    /// Returns node type of given node.
    ///
    /// Parameters
    /// ----------
    /// node_id: int,
    ///     node whose node type is to be returned.
    ///
    pub fn get_node_type_id_from_node_id(
        &self,
        node_id: NodeT,
    ) -> PyResult<Option<Vec<NodeTypeT>>> {
        pe!(self.graph.get_node_type_id_from_node_id(node_id))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_id)"]
    /// Returns edge type of given edge.
    ///
    /// This method will panic if the given edge ID is greater than
    /// the number of edges in the graph.
    /// Furthermore, if the graph does NOT have edge types, it will NOT
    /// return neither an error or a panic.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int,
    ///     edge whose edge type is to be returned.
    ///
    ///
    /// Safety
    /// ------
    /// If the given edge ID does not exist in the current graph the method will raise a panic.
    pub unsafe fn get_unchecked_edge_type_id_from_edge_id(
        &self,
        edge_id: EdgeT,
    ) -> Option<EdgeTypeT> {
        self.graph.get_unchecked_edge_type_id_from_edge_id(edge_id)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_id)"]
    /// Returns edge type of given edge.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int,
    ///     edge whose edge type is to be returned.
    ///
    pub fn get_edge_type_id_from_edge_id(&self, edge_id: EdgeT) -> PyResult<Option<EdgeTypeT>> {
        pe!(self.graph.get_edge_type_id_from_edge_id(edge_id))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_id)"]
    /// Returns result of option with the node type of the given node id.
    ///
    /// Parameters
    /// ----------
    /// node_id: int,
    ///     The node ID whose node types are to be returned.
    ///
    ///
    /// Safety
    /// ------
    /// This method will return an iterator of None values when the graph
    ///  does not contain node types.
    pub unsafe fn get_unchecked_node_type_names_from_node_id(
        &self,
        node_id: NodeT,
    ) -> Option<Vec<String>> {
        self.graph
            .get_unchecked_node_type_names_from_node_id(node_id)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_id)"]
    /// Returns result of option with the node type of the given node id.
    ///
    /// Parameters
    /// ----------
    /// node_id: int,
    ///     The node ID whose node types are to be returned.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the node types are not available for the current graph instance.
    ///
    pub fn get_node_type_names_from_node_id(
        &self,
        node_id: NodeT,
    ) -> PyResult<Option<Vec<String>>> {
        pe!(self.graph.get_node_type_names_from_node_id(node_id))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_name)"]
    /// Returns result of option with the node type of the given node name.
    ///
    /// Parameters
    /// ----------
    /// node_name: str,
    ///     The node name whose node types are to be returned.
    ///
    pub fn get_node_type_names_from_node_name(
        &self,
        node_name: &str,
    ) -> PyResult<Option<Vec<String>>> {
        pe!(self.graph.get_node_type_names_from_node_name(node_name))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_id)"]
    /// Returns option with the edge type of the given edge id.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int,
    ///     The edge ID whose edge type is to be returned.
    ///
    pub fn get_edge_type_name_from_edge_id(&self, edge_id: EdgeT) -> PyResult<Option<String>> {
        pe!(self.graph.get_edge_type_name_from_edge_id(edge_id))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_type_id)"]
    /// Return edge type name of given edge type.
    ///
    /// Parameters
    /// ----------
    /// edge_type_id: int,
    ///     Id of the edge type.
    ///
    pub fn get_edge_type_name_from_edge_type_id(
        &self,
        edge_type_id: EdgeTypeT,
    ) -> PyResult<String> {
        pe!(self
            .graph
            .get_edge_type_name_from_edge_type_id(edge_type_id))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_id)"]
    /// Returns weight of the given edge id.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int,
    ///     The edge ID whose weight is to be returned.
    ///
    pub fn get_edge_weight_from_edge_id(&self, edge_id: EdgeT) -> PyResult<WeightT> {
        pe!(self.graph.get_edge_weight_from_edge_id(edge_id))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src, dst)"]
    /// Returns weight of the given node ids.
    ///
    /// Parameters
    /// ----------
    /// src: int,
    ///     The node ID of the source node.
    /// dst: int,
    ///     The node ID of the destination node.
    ///
    pub fn get_edge_weight_from_node_ids(&self, src: NodeT, dst: NodeT) -> PyResult<WeightT> {
        pe!(self.graph.get_edge_weight_from_node_ids(src, dst))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src, dst, edge_type)"]
    /// Returns weight of the given node ids and edge type.
    ///
    /// Parameters
    /// ----------
    /// src: int,
    ///     The node ID of the source node.
    /// dst: int,
    ///     The node ID of the destination node.
    /// edge_type: Optional[int],
    ///     The edge type ID of the edge.
    ///
    pub fn get_edge_weight_from_node_ids_and_edge_type_id(
        &self,
        src: NodeT,
        dst: NodeT,
        edge_type: Option<EdgeTypeT>,
    ) -> PyResult<WeightT> {
        pe!(self
            .graph
            .get_edge_weight_from_node_ids_and_edge_type_id(src, dst, edge_type))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src, dst, edge_type)"]
    /// Returns weight of the given node names and edge type.
    ///
    /// Parameters
    /// ----------
    /// src: str,
    ///     The node name of the source node.
    /// dst: str,
    ///     The node name of the destination node.
    /// edge_type: Optional[str],
    ///     The edge type name of the edge.
    ///
    pub fn get_edge_weight_from_node_names_and_edge_type_name(
        &self,
        src: &str,
        dst: &str,
        edge_type: Option<&str>,
    ) -> PyResult<WeightT> {
        pe!(self
            .graph
            .get_edge_weight_from_node_names_and_edge_type_name(src, dst, edge_type))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src_name, dst_name)"]
    /// Returns weight of the given node names.
    ///
    /// Parameters
    /// ----------
    /// src_name: str,
    ///     The node name of the source node.
    /// dst_name: str,
    ///     The node name of the destination node.
    ///
    pub fn get_edge_weight_from_node_names(
        &self,
        src_name: &str,
        dst_name: &str,
    ) -> PyResult<WeightT> {
        pe!(self
            .graph
            .get_edge_weight_from_node_names(src_name, dst_name))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_id)"]
    /// Returns result with the node name.
    ///
    /// Parameters
    /// ----------
    /// node_id: int,
    ///     The node ID whose name is to be returned.
    ///
    ///
    /// Safety
    /// ------
    /// If the given node ID does not exist in the current graph the method will raise a panic.
    pub unsafe fn get_unchecked_node_name_from_node_id(&self, node_id: NodeT) -> String {
        self.graph.get_unchecked_node_name_from_node_id(node_id)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_id)"]
    /// Returns result with the node name.
    ///
    /// Parameters
    /// ----------
    /// node_id: int,
    ///     The node ID whose name is to be returned.
    ///
    pub fn get_node_name_from_node_id(&self, node_id: NodeT) -> PyResult<String> {
        pe!(self.graph.get_node_name_from_node_id(node_id))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_name)"]
    /// Returns result with the node ID.
    ///
    /// Parameters
    /// ----------
    /// node_name: str,
    ///     The node name whose node ID is to be returned.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     When the given node name does not exists in the current graph.
    ///
    pub fn get_node_id_from_node_name(&self, node_name: &str) -> PyResult<NodeT> {
        pe!(self.graph.get_node_id_from_node_name(node_name))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_names)"]
    /// Returns result with the node IDs.
    ///
    /// Parameters
    /// ----------
    /// node_names: List[str],
    ///     The node names whose node IDs is to be returned.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     When any of the given node name does not exists in the current graph.
    ///
    pub fn get_node_ids_from_node_names(
        &self,
        node_names: Vec<&str>,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self.graph.get_node_ids_from_node_names(node_names))?,
            NodeT
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_node_names)"]
    /// Returns result with the edge node IDs.
    ///
    /// Parameters
    /// ----------
    /// edge_node_names: List[Tuple[str, str]],
    ///     The node names whose node IDs is to be returned.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     When any of the given node name does not exists in the current graph.
    ///
    pub fn get_edge_node_ids_from_edge_node_names(
        &self,
        edge_node_names: Vec<(&str, &str)>,
    ) -> PyResult<Vec<(NodeT, NodeT)>> {
        pe!(self
            .graph
            .get_edge_node_ids_from_edge_node_names(edge_node_names))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_node_ids)"]
    /// Returns result with the edge node names.
    ///
    /// Parameters
    /// ----------
    /// edge_node_ids: List[Tuple[int, int]],
    ///     The node names whose node names is to be returned.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     When any of the given node IDs does not exists in the current graph.
    ///
    pub fn get_edge_node_names_from_edge_node_ids(
        &self,
        edge_node_ids: Vec<(NodeT, NodeT)>,
    ) -> PyResult<Vec<(String, String)>> {
        pe!(self
            .graph
            .get_edge_node_names_from_edge_node_ids(edge_node_ids))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_name)"]
    /// Return node type ID for the given node name if available.
    ///
    /// Parameters
    /// ----------
    /// node_name: str,
    ///     Name of the node.
    ///
    pub fn get_node_type_id_from_node_name(
        &self,
        node_name: &str,
    ) -> PyResult<Option<Vec<NodeTypeT>>> {
        pe!(self.graph.get_node_type_id_from_node_name(node_name))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_name)"]
    /// Return node type name for the given node name if available.
    ///
    /// Parameters
    /// ----------
    /// node_name: str,
    ///     Name of the node.
    ///
    pub fn get_node_type_name_from_node_name(
        &self,
        node_name: &str,
    ) -> PyResult<Option<Vec<String>>> {
        pe!(self.graph.get_node_type_name_from_node_name(node_name))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_type_id)"]
    /// Return number of edges with given edge type ID.
    ///
    /// If None is given as an edge type ID, the unknown edge type IDs
    /// will be returned.
    ///
    /// Parameters
    /// ----------
    /// edge_type_id: Optional[int],
    ///     The edge type ID to count the edges of.
    ///
    pub fn get_edge_count_from_edge_type_id(
        &self,
        edge_type_id: Option<EdgeTypeT>,
    ) -> PyResult<EdgeT> {
        pe!(self.graph.get_edge_count_from_edge_type_id(edge_type_id))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_type_name)"]
    /// Return edge type ID curresponding to given edge type name.
    ///
    /// If None is given as an edge type ID, None is returned.
    ///
    /// Parameters
    /// ----------
    /// edge_type_name: Optional[str],
    ///     The edge type name whose ID is to be returned.
    ///
    pub fn get_edge_type_id_from_edge_type_name(
        &self,
        edge_type_name: Option<&str>,
    ) -> PyResult<Option<EdgeTypeT>> {
        pe!(self
            .graph
            .get_edge_type_id_from_edge_type_name(edge_type_name))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_type_name)"]
    /// Return number of edges with given edge type name.
    ///
    /// If None is given as an edge type name, the unknown edge types
    /// will be returned.
    ///
    /// Parameters
    /// ----------
    /// edge_type_name: Optional[str],
    ///     The edge type name to count the edges of.
    ///
    pub fn get_edge_count_from_edge_type_name(
        &self,
        edge_type_name: Option<&str>,
    ) -> PyResult<EdgeT> {
        pe!(self
            .graph
            .get_edge_count_from_edge_type_name(edge_type_name))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_type_name)"]
    /// Return node type ID curresponding to given node type name.
    ///
    /// If None is given as an node type ID, None is returned.
    ///
    /// Parameters
    /// ----------
    /// node_type_name: str,
    ///     The node type name whose ID is to be returned.
    ///
    pub fn get_node_type_id_from_node_type_name(
        &self,
        node_type_name: &str,
    ) -> PyResult<NodeTypeT> {
        pe!(self
            .graph
            .get_node_type_id_from_node_type_name(node_type_name))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_type_id)"]
    /// Return number of nodes with given node type ID.
    ///
    /// If None is given as an node type ID, the unknown node types
    /// will be returned.
    ///
    /// Parameters
    /// ----------
    /// node_type_id: Optional[int],
    ///     The node type ID to count the nodes of.
    ///
    pub fn get_node_count_from_node_type_id(
        &self,
        node_type_id: Option<NodeTypeT>,
    ) -> PyResult<NodeT> {
        pe!(self.graph.get_node_count_from_node_type_id(node_type_id))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_type_name)"]
    /// Return number of nodes with given node type name.
    ///
    /// If None is given as an node type name, the unknown node types
    /// will be returned.
    ///
    /// Parameters
    /// ----------
    /// node_type_name: Optional[str],
    ///     The node type name to count the nodes of.
    ///
    pub fn get_node_count_from_node_type_name(
        &self,
        node_type_name: Option<&str>,
    ) -> PyResult<NodeT> {
        pe!(self
            .graph
            .get_node_count_from_node_type_name(node_type_name))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_id)"]
    /// Return vector of destinations for the given source node ID.
    ///
    /// Parameters
    /// ----------
    /// node_id: int,
    ///     Node ID whose neighbours are to be retrieved.
    ///
    pub fn get_neighbour_node_ids_from_node_id(
        &self,
        node_id: NodeT,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self.graph.get_neighbour_node_ids_from_node_id(node_id))?,
            NodeT
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_name)"]
    /// Return vector of destinations for the given source node name.
    ///
    /// Parameters
    /// ----------
    /// node_name: str,
    ///     Node ID whose neighbours are to be retrieved.
    ///
    pub fn get_neighbour_node_ids_from_node_name(
        &self,
        node_name: &str,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self.graph.get_neighbour_node_ids_from_node_name(node_name))?,
            NodeT
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_name)"]
    /// Return vector of destination names for the given source node name.
    ///
    /// Parameters
    /// ----------
    /// node_name: str,
    ///     Node name whose neighbours are to be retrieved.
    ///
    pub fn get_neighbour_node_names_from_node_name(
        &self,
        node_name: &str,
    ) -> PyResult<Vec<String>> {
        pe!(self
            .graph
            .get_neighbour_node_names_from_node_name(node_name))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src, dst)"]
    /// Return range of outbound edges IDs for all the edges bewteen the given
    /// source and destination nodes.
    /// This operation is meaningfull only in a multigraph.
    ///
    /// Parameters
    /// ----------
    /// src: int,
    ///     Source node.
    /// dst: int,
    ///     Destination node.
    ///
    pub fn get_minmax_edge_ids_from_node_ids(
        &self,
        src: NodeT,
        dst: NodeT,
    ) -> PyResult<(EdgeT, EdgeT)> {
        pe!(self.graph.get_minmax_edge_ids_from_node_ids(src, dst))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src, dst, edge_type)"]
    /// Return edge ID for given tuple of nodes and edge type.
    ///
    /// This method will return an error if the graph does not contain the
    /// requested edge with edge type.
    ///
    /// Parameters
    /// ----------
    /// src: int,
    ///     Source node of the edge.
    /// dst: int,
    ///     Destination node of the edge.
    /// edge_type: Optional[int],
    ///     Edge Type of the edge.
    ///
    pub fn get_edge_id_from_node_ids_and_edge_type_id(
        &self,
        src: NodeT,
        dst: NodeT,
        edge_type: Option<EdgeTypeT>,
    ) -> PyResult<EdgeT> {
        pe!(self
            .graph
            .get_edge_id_from_node_ids_and_edge_type_id(src, dst, edge_type))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src_name, dst_name)"]
    /// Return edge ID for given tuple of node names.
    ///
    /// This method will return an error if the graph does not contain the
    /// requested edge with edge type.
    ///
    /// Parameters
    /// ----------
    /// src_name: str,
    ///     Source node name of the edge.
    /// dst_name: str,
    ///     Destination node name of the edge.
    ///
    pub fn get_edge_id_from_node_names(&self, src_name: &str, dst_name: &str) -> PyResult<EdgeT> {
        pe!(self.graph.get_edge_id_from_node_names(src_name, dst_name))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src_name, dst_name, edge_type_name)"]
    /// Return edge ID for given tuple of node names and edge type name.
    ///
    /// This method will return an error if the graph does not contain the
    /// requested edge with edge type.
    ///
    /// Parameters
    /// ----------
    /// src_name: str,
    ///     Source node name of the edge.
    /// dst_name: str,
    ///     Destination node name of the edge.
    /// edge_type_name: Optional[str],
    ///     Edge type name.
    ///
    pub fn get_edge_id_from_node_names_and_edge_type_name(
        &self,
        src_name: &str,
        dst_name: &str,
        edge_type_name: Option<&str>,
    ) -> PyResult<EdgeT> {
        pe!(self.graph.get_edge_id_from_node_names_and_edge_type_name(
            src_name,
            dst_name,
            edge_type_name
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_type_names)"]
    /// Return translated edge types from string to internal edge ID.
    ///
    /// Parameters
    /// ----------
    /// edge_type_names: List[Optional[str]],
    ///     Vector of edge types to be converted.
    ///
    pub fn get_edge_type_ids_from_edge_type_names(
        &self,
        edge_type_names: Vec<Option<String>>,
    ) -> PyResult<Vec<Option<EdgeTypeT>>> {
        pe!(self
            .graph
            .get_edge_type_ids_from_edge_type_names(edge_type_names))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_type_names)"]
    /// Return translated node types from string to internal node ID.
    ///
    /// Parameters
    /// ----------
    /// node_type_names: List[Optional[str]],
    ///     Vector of node types to be converted.
    ///
    pub fn get_node_type_ids_from_node_type_names(
        &self,
        node_type_names: Vec<Option<String>>,
    ) -> PyResult<Vec<Option<NodeTypeT>>> {
        pe!(self
            .graph
            .get_node_type_ids_from_node_type_names(node_type_names))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_type_names)"]
    /// Return translated node types from string to internal node ID.
    ///
    /// Parameters
    /// ----------
    /// node_type_names: List[Optional[List[str]]],
    ///     Vector of node types to be converted.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    /// ValueError
    ///     If any of the given node type names do not exists in the graph.
    ///
    pub fn get_multiple_node_type_ids_from_node_type_names(
        &self,
        node_type_names: Vec<Option<Vec<&str>>>,
    ) -> PyResult<Vec<Option<Vec<NodeTypeT>>>> {
        pe!(self
            .graph
            .get_multiple_node_type_ids_from_node_type_names(node_type_names))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src)"]
    /// Return range of outbound edges IDs which have as source the given Node.
    ///
    /// The method will panic if the given source node ID is higher than
    /// the number of nodes in the graph.
    ///
    /// Parameters
    /// ----------
    /// src: int,
    ///     Node for which we need to compute the cumulative_node_degrees range.
    ///
    ///
    /// Safety
    /// ------
    /// If the given node ID does not exist in the current graph the method will raise a panic.
    pub unsafe fn get_unchecked_minmax_edge_ids_from_source_node_id(
        &self,
        src: NodeT,
    ) -> (EdgeT, EdgeT) {
        self.graph
            .get_unchecked_minmax_edge_ids_from_source_node_id(src)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src)"]
    /// Return range of outbound edges IDs which have as source the given Node.
    ///
    /// Parameters
    /// ----------
    /// src: int,
    ///     Node for which we need to compute the cumulative_node_degrees range.
    ///
    pub fn get_minmax_edge_ids_from_source_node_id(&self, src: NodeT) -> PyResult<(EdgeT, EdgeT)> {
        pe!(self.graph.get_minmax_edge_ids_from_source_node_id(src))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_type_id)"]
    /// Return node type name of given node type.
    ///
    /// There is no need for a unchecked version since we will have to map
    /// on the note_types anyway.
    ///
    /// Parameters
    /// ----------
    /// node_type_id: int,
    ///     Id of the node type.
    ///
    pub fn get_node_type_name_from_node_type_id(
        &self,
        node_type_id: NodeTypeT,
    ) -> PyResult<String> {
        pe!(self
            .graph
            .get_node_type_name_from_node_type_id(node_type_id))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_type_ids)"]
    /// Return node type name of given node type.
    ///
    /// Parameters
    /// ----------
    /// node_type_ids: List[int],
    ///     Id of the node type.
    ///
    ///
    /// Safety
    /// ------
    /// The method will panic if the graph does not contain node types.
    pub unsafe fn get_unchecked_node_type_names_from_node_type_ids(
        &self,
        node_type_ids: Vec<NodeTypeT>,
    ) -> Vec<String> {
        self.graph
            .get_unchecked_node_type_names_from_node_type_ids(node_type_ids)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_type_name, weight, verbose)"]
    /// Returns new graph with added in missing self-loops with given edge type and weight.
    ///
    /// Parameters
    /// ----------
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the edge type for the new singletons is provided but the graph does not have edge types.
    /// ValueError
    ///     If the edge weight for the new singletons is provided but the graph does not have edge weights.
    /// ValueError
    ///     If the edge weight for the new singletons is NOT provided but the graph does have edge weights.
    ///
    pub fn add_selfloops(
        &self,
        edge_type_name: Option<&str>,
        weight: Option<WeightT>,
        verbose: Option<bool>,
    ) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(self.graph.add_selfloops(edge_type_name, weight, verbose))?,
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns vector of unweighted degree centrality for all nodes
    pub fn get_degree_centrality(&self) -> PyResult<Py<PyArray1<f64>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self.graph.get_degree_centrality())?,
            f64
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns vector of weighted degree centrality for all nodes
    pub fn get_weighted_degree_centrality(&self) -> PyResult<Py<PyArray1<f64>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self.graph.get_weighted_degree_centrality())?,
            f64
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_id)"]
    /// Return closeness centrality of the requested node.
    ///
    /// If the given node ID does not exist in the current graph the method
    /// will panic.
    ///
    /// Parameters
    /// ----------
    /// node_id: int,
    ///     The node ID whose closeness centrality is to be computed.
    /// verbose: Optional[bool],
    ///     Whether to show an indicative progress bar.
    ///
    ///
    /// Safety
    /// ------
    /// If the given node ID does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_closeness_centrality_from_node_id(&self, node_id: NodeT) -> f64 {
        self.graph
            .get_unchecked_closeness_centrality_from_node_id(node_id)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_id, use_edge_weights_as_probabilities)"]
    /// Return closeness centrality of the requested node.
    ///
    /// If the given node ID does not exist in the current graph the method
    /// will panic.
    ///
    /// Parameters
    /// ----------
    /// node_id: int,
    ///     The node ID whose closeness centrality is to be computed.
    /// use_edge_weights_as_probabilities: bool,
    ///     Whether to treat the edge weights as probabilities.
    ///
    ///
    /// Safety
    /// ------
    /// If the given node ID does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_weighted_closeness_centrality_from_node_id(
        &self,
        node_id: NodeT,
        use_edge_weights_as_probabilities: bool,
    ) -> f64 {
        self.graph
            .get_unchecked_weighted_closeness_centrality_from_node_id(
                node_id,
                use_edge_weights_as_probabilities,
            )
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, verbose)"]
    /// Return closeness centrality for all nodes.
    ///
    /// Parameters
    /// ----------
    /// verbose: Optional[bool],
    ///     Whether to show an indicative progress bar.
    ///
    pub fn get_closeness_centrality(&self, verbose: Option<bool>) -> Py<PyArray1<f64>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.graph.get_closeness_centrality(verbose), f64)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, use_edge_weights_as_probabilities, verbose)"]
    /// Return closeness centrality for all nodes.
    ///
    /// Parameters
    /// ----------
    /// use_edge_weights_as_probabilities: bool,
    ///     Whether to treat the edge weights as probabilities.
    /// verbose: Optional[bool],
    ///     Whether to show an indicative progress bar.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have weights.
    /// ValueError
    ///     If the graph contains negative weights.
    /// ValueError
    ///     If the user has asked for the weights to be treated as probabilities but the weights are not between 0 and 1.
    ///
    pub fn get_weighted_closeness_centrality(
        &self,
        use_edge_weights_as_probabilities: Option<bool>,
        verbose: Option<bool>,
    ) -> PyResult<Py<PyArray1<f64>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self
                .graph
                .get_weighted_closeness_centrality(use_edge_weights_as_probabilities, verbose))?,
            f64
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_id)"]
    /// Return harmonic centrality of the requested node.
    ///
    /// If the given node ID does not exist in the current graph the method
    /// will panic.
    ///
    /// Parameters
    /// ----------
    /// node_id: int,
    ///     The node ID whose harmonic centrality is to be computed.
    ///
    ///
    /// Safety
    /// ------
    /// If the given node ID does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_harmonic_centrality_from_node_id(&self, node_id: NodeT) -> f64 {
        self.graph
            .get_unchecked_harmonic_centrality_from_node_id(node_id)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_id, use_edge_weights_as_probabilities)"]
    /// Return harmonic centrality of the requested node.
    ///
    /// If the given node ID does not exist in the current graph the method
    /// will panic.
    ///
    /// Parameters
    /// ----------
    /// node_id: int,
    ///     The node ID whose harmonic centrality is to be computed.
    /// use_edge_weights_as_probabilities: bool,
    ///     Whether to treat the edge weights as probabilities.
    ///
    ///
    /// Safety
    /// ------
    /// If the given node ID does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_weighted_harmonic_centrality_from_node_id(
        &self,
        node_id: NodeT,
        use_edge_weights_as_probabilities: bool,
    ) -> f64 {
        self.graph
            .get_unchecked_weighted_harmonic_centrality_from_node_id(
                node_id,
                use_edge_weights_as_probabilities,
            )
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, verbose)"]
    /// Return harmonic centrality for all nodes.
    ///
    /// Parameters
    /// ----------
    /// verbose: Optional[bool],
    ///     Whether to show an indicative progress bar.
    ///
    pub fn get_harmonic_centrality(&self, verbose: Option<bool>) -> Py<PyArray1<f64>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.graph.get_harmonic_centrality(verbose), f64)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, use_edge_weights_as_probabilities, verbose)"]
    /// Return harmonic centrality for all nodes.
    ///
    /// Parameters
    /// ----------
    /// use_edge_weights_as_probabilities: Optional[bool],
    ///     Whether to treat the edge weights as probabilities.
    /// verbose: Optional[bool],
    ///     Whether to show an indicative progress bar.
    ///
    pub fn get_weighted_harmonic_centrality(
        &self,
        use_edge_weights_as_probabilities: Option<bool>,
        verbose: Option<bool>,
    ) -> PyResult<Py<PyArray1<f64>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self
                .graph
                .get_weighted_harmonic_centrality(use_edge_weights_as_probabilities, verbose))?,
            f64
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, normalize, verbose)"]
    /// Returns vector of stress centrality for all nodes.
    ///
    /// Parameters
    /// ----------
    /// normalize: Optional[bool],
    ///     Whether to normalize the values. By default, it is false.
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar. By default, it is true.
    ///
    pub fn get_stress_centrality(
        &self,
        normalize: Option<bool>,
        verbose: Option<bool>,
    ) -> Py<PyArray1<f64>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(
            gil,
            self.graph.get_stress_centrality(normalize, verbose),
            f64
        )
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, normalize, verbose)"]
    /// Returns vector of betweenness centrality for all nodes.
    ///
    /// Parameters
    /// ----------
    /// normalize: Optional[bool],
    ///     Whether to normalize the values. By default, it is false.
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar. By default, it is true.
    ///
    pub fn get_betweenness_centrality(
        &self,
        normalize: Option<bool>,
        verbose: Option<bool>,
    ) -> Py<PyArray1<f64>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(
            gil,
            self.graph.get_betweenness_centrality(normalize, verbose),
            f64
        )
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, maximum_iterations_number, tollerance)"]
    /// Returns vector with unweighted eigenvector centrality.
    ///
    /// Parameters
    /// ----------
    /// maximum_iterations_number: Optional[int],
    ///     The maximum number of iterations to consider.
    /// tollerance: Optional[float],
    ///     The maximum error tollerance for convergence.
    ///
    pub fn get_eigenvector_centrality(
        &self,
        maximum_iterations_number: Option<usize>,
        tollerance: Option<f64>,
    ) -> PyResult<Py<PyArray1<f64>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self
                .graph
                .get_eigenvector_centrality(maximum_iterations_number, tollerance))?,
            f64
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, maximum_iterations_number, tollerance)"]
    /// Returns vector with unweighted eigenvector centrality.
    ///
    /// Parameters
    /// ----------
    /// maximum_iterations_number: Optional[int],
    ///     The maximum number of iterations to consider.
    /// tollerance: Optional[float],
    ///     The maximum error tollerance for convergence.
    ///
    pub fn get_weighted_eigenvector_centrality(
        &self,
        maximum_iterations_number: Option<usize>,
        tollerance: Option<f64>,
    ) -> PyResult<Py<PyArray1<f64>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self
                .graph
                .get_weighted_eigenvector_centrality(maximum_iterations_number, tollerance))?,
            f64
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, use_node_names)"]
    /// Print the current graph in a format compatible with Graphviz dot's format
    pub fn to_dot(&self, use_node_names: Option<bool>) -> String {
        self.graph.to_dot(use_node_names)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns the minumum unweighted preferential attachment score.
    ///
    /// Safety
    /// ------
    /// If the graph does not contain nodes, the return value will be undefined.
    pub unsafe fn get_unchecked_minimum_preferential_attachment(&self) -> f64 {
        self.graph.get_unchecked_minimum_preferential_attachment()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns the maximum unweighted preferential attachment score.
    ///
    /// Safety
    /// ------
    /// If the graph does not contain nodes, the return value will be undefined.
    pub unsafe fn get_unchecked_maximum_preferential_attachment(&self) -> f64 {
        self.graph.get_unchecked_maximum_preferential_attachment()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns the minumum weighted preferential attachment score.
    ///
    /// Safety
    /// ------
    /// If the graph does not contain nodes, the return value will be undefined.
    pub unsafe fn get_unchecked_weighted_minimum_preferential_attachment(&self) -> f64 {
        self.graph
            .get_unchecked_weighted_minimum_preferential_attachment()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns the maximum weighted preferential attachment score.
    ///
    /// Safety
    /// ------
    /// If the graph does not contain nodes, the return value will be undefined.
    pub unsafe fn get_unchecked_weighted_maximum_preferential_attachment(&self) -> f64 {
        self.graph
            .get_unchecked_weighted_maximum_preferential_attachment()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, source_node_id, destination_node_id, normalize)"]
    /// Returns the unweighted preferential attachment from the given node IDs.
    ///
    /// Parameters
    /// ----------
    /// source_node_id: int,
    ///     Node ID of the first node.
    /// destination_node_id: int,
    ///     Node ID of the second node.
    /// normalize: bool,
    ///     Whether to normalize within 0 to 1.
    ///
    ///
    /// Safety
    /// ------
    /// If either of the provided one and two node IDs are higher than the
    ///  number of nodes in the graph.
    pub unsafe fn get_unchecked_preferential_attachment_from_node_ids(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
        normalize: bool,
    ) -> f64 {
        self.graph
            .get_unchecked_preferential_attachment_from_node_ids(
                source_node_id,
                destination_node_id,
                normalize,
            )
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, source_node_id, destination_node_id, normalize)"]
    /// Returns the unweighted preferential attachment from the given node IDs.
    ///
    /// Parameters
    /// ----------
    /// source_node_id: int,
    ///     Node ID of the first node.
    /// destination_node_id: int,
    ///     Node ID of the second node.
    /// normalize: bool,
    ///     Whether to normalize by the square of maximum degree.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If either of the node IDs are higher than the number of nodes in the graph.
    ///
    pub fn get_preferential_attachment_from_node_ids(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
        normalize: bool,
    ) -> PyResult<f64> {
        pe!(self.graph.get_preferential_attachment_from_node_ids(
            source_node_id,
            destination_node_id,
            normalize
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, first_node_name, second_node_name, normalize)"]
    /// Returns the unweighted preferential attachment from the given node names.
    ///
    /// Parameters
    /// ----------
    /// first_node_name: str,
    ///     Node name of the first node.
    /// second_node_name: str,
    ///     Node name of the second node.
    /// normalize: bool,
    ///     Whether to normalize by the square of maximum degree.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If either of the given node names do not exist in the current graph.
    ///
    pub fn get_preferential_attachment_from_node_names(
        &self,
        first_node_name: &str,
        second_node_name: &str,
        normalize: bool,
    ) -> PyResult<f64> {
        pe!(self.graph.get_preferential_attachment_from_node_names(
            first_node_name,
            second_node_name,
            normalize
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, source_node_id, destination_node_id, normalize)"]
    /// Returns the weighted preferential attachment from the given node IDs.
    ///
    /// Parameters
    /// ----------
    /// source_node_id: int,
    ///     Node ID of the first node.
    /// destination_node_id: int,
    ///     Node ID of the second node.
    /// normalize: bool,
    ///     Whether to normalize within 0 to 1.
    ///
    ///
    /// Safety
    /// ------
    /// If either of the provided one and two node IDs are higher than the
    ///  number of nodes in the graph.
    pub unsafe fn get_unchecked_weighted_preferential_attachment_from_node_ids(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
        normalize: bool,
    ) -> f64 {
        self.graph
            .get_unchecked_weighted_preferential_attachment_from_node_ids(
                source_node_id,
                destination_node_id,
                normalize,
            )
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, source_node_id, destination_node_id, normalize)"]
    /// Returns the weighted preferential attachment from the given node IDs.
    ///
    /// Parameters
    /// ----------
    /// source_node_id: int,
    ///     Node ID of the first node.
    /// destination_node_id: int,
    ///     Node ID of the second node.
    /// normalize: bool,
    ///     Whether to normalize by the square of maximum degree.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If either of the node IDs are higher than the number of nodes in the graph.
    ///
    pub fn get_weighted_preferential_attachment_from_node_ids(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
        normalize: bool,
    ) -> PyResult<f64> {
        pe!(self
            .graph
            .get_weighted_preferential_attachment_from_node_ids(
                source_node_id,
                destination_node_id,
                normalize
            ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, first_node_name, second_node_name, normalize)"]
    /// Returns the weighted preferential attachment from the given node names.
    ///
    /// Parameters
    /// ----------
    /// first_node_name: str,
    ///     Node name of the first node.
    /// second_node_name: str,
    ///     Node name of the second node.
    /// normalize: bool,
    ///     Whether to normalize by the square of maximum degree.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If either of the given node names do not exist in the current graph.
    ///
    pub fn get_weighted_preferential_attachment_from_node_names(
        &self,
        first_node_name: &str,
        second_node_name: &str,
        normalize: bool,
    ) -> PyResult<f64> {
        pe!(self
            .graph
            .get_weighted_preferential_attachment_from_node_names(
                first_node_name,
                second_node_name,
                normalize
            ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, source_node_id, destination_node_id)"]
    /// Returns the Jaccard index for the two given nodes from the given node IDs.
    ///
    /// Parameters
    /// ----------
    /// source_node_id: int,
    ///     Node ID of the first node.
    /// destination_node_id: int,
    ///     Node ID of the second node.
    ///
    ///
    /// Safety
    /// ------
    /// If either of the provided one and two node IDs are higher than the
    ///  number of nodes in the graph.
    pub unsafe fn get_unchecked_jaccard_coefficient_from_node_ids(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
    ) -> f64 {
        self.graph
            .get_unchecked_jaccard_coefficient_from_node_ids(source_node_id, destination_node_id)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, source_node_id, destination_node_id)"]
    /// Returns the Jaccard index for the two given nodes from the given node IDs.
    ///
    /// Parameters
    /// ----------
    /// source_node_id: int,
    ///     Node ID of the first node.
    /// destination_node_id: int,
    ///     Node ID of the second node.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If either of the node IDs are higher than the number of nodes in the graph.
    ///
    pub fn get_jaccard_coefficient_from_node_ids(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
    ) -> PyResult<f64> {
        pe!(self
            .graph
            .get_jaccard_coefficient_from_node_ids(source_node_id, destination_node_id))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, first_node_name, second_node_name)"]
    /// Returns the Jaccard index for the two given nodes from the given node names.
    ///
    /// Parameters
    /// ----------
    /// first_node_name: str,
    ///     Node name of the first node.
    /// second_node_name: str,
    ///     Node name of the second node.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If either of the given node names do not exist in the current graph.
    ///
    pub fn get_jaccard_coefficient_from_node_names(
        &self,
        first_node_name: &str,
        second_node_name: &str,
    ) -> PyResult<f64> {
        pe!(self
            .graph
            .get_jaccard_coefficient_from_node_names(first_node_name, second_node_name))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, source_node_id, destination_node_id)"]
    /// Returns the Adamic/Adar Index for the given pair of nodes from the given node IDs.
    ///
    /// Parameters
    /// ----------
    /// source_node_id: int,
    ///     Node ID of the first node.
    /// destination_node_id: int,
    ///     Node ID of the second node.
    ///
    ///
    /// Safety
    /// ------
    /// If either of the provided one and two node IDs are higher than the
    ///  number of nodes in the graph.
    pub unsafe fn get_unchecked_adamic_adar_index_from_node_ids(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
    ) -> f64 {
        self.graph
            .get_unchecked_adamic_adar_index_from_node_ids(source_node_id, destination_node_id)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, source_node_id, destination_node_id)"]
    /// Returns the Adamic/Adar Index for the given pair of nodes from the given node IDs.
    ///
    /// Parameters
    /// ----------
    /// source_node_id: int,
    ///     Node ID of the first node.
    /// destination_node_id: int,
    ///     Node ID of the second node.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If either of the node IDs are higher than the number of nodes in the graph.
    ///
    pub fn get_adamic_adar_index_from_node_ids(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
    ) -> PyResult<f64> {
        pe!(self
            .graph
            .get_adamic_adar_index_from_node_ids(source_node_id, destination_node_id))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, first_node_name, second_node_name)"]
    /// Returns the Adamic/Adar Index for the given pair of nodes from the given node names.
    ///
    /// Parameters
    /// ----------
    /// first_node_name: str,
    ///     Node name of the first node.
    /// second_node_name: str,
    ///     Node name of the second node.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If either of the given node names do not exist in the current graph.
    ///
    pub fn get_adamic_adar_index_from_node_names(
        &self,
        first_node_name: &str,
        second_node_name: &str,
    ) -> PyResult<f64> {
        pe!(self
            .graph
            .get_adamic_adar_index_from_node_names(first_node_name, second_node_name))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, source_node_id, destination_node_id)"]
    /// Returns the unweighted Resource Allocation Index for the given pair of nodes from the given node IDs.
    ///
    /// Parameters
    /// ----------
    /// source_node_id: int,
    ///     Node ID of the first node.
    /// destination_node_id: int,
    ///     Node ID of the second node.
    ///
    ///
    /// Safety
    /// ------
    /// If either of the provided one and two node IDs are higher than the
    ///  number of nodes in the graph.
    pub unsafe fn get_unchecked_resource_allocation_index_from_node_ids(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
    ) -> f64 {
        self.graph
            .get_unchecked_resource_allocation_index_from_node_ids(
                source_node_id,
                destination_node_id,
            )
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, source_node_id, destination_node_id)"]
    /// Returns the weighted Resource Allocation Index for the given pair of nodes from the given node IDs.
    ///
    /// Parameters
    /// ----------
    /// source_node_id: int,
    ///     Node ID of the first node.
    /// destination_node_id: int,
    ///     Node ID of the second node.
    ///
    ///
    /// Safety
    /// ------
    /// If either of the provided one and two node IDs are higher than the
    ///  number of nodes in the graph.
    pub unsafe fn get_unchecked_weighted_resource_allocation_index_from_node_ids(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
    ) -> f64 {
        self.graph
            .get_unchecked_weighted_resource_allocation_index_from_node_ids(
                source_node_id,
                destination_node_id,
            )
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, source_node_id, destination_node_id)"]
    /// Returns the unweighted Resource Allocation Index for the given pair of nodes from the given node IDs.
    ///
    /// Parameters
    /// ----------
    /// source_node_id: int,
    ///     Node ID of the first node.
    /// destination_node_id: int,
    ///     Node ID of the second node.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If either of the node IDs are higher than the number of nodes in the graph.
    ///
    pub fn get_resource_allocation_index_from_node_ids(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
    ) -> PyResult<f64> {
        pe!(self
            .graph
            .get_resource_allocation_index_from_node_ids(source_node_id, destination_node_id))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, first_node_name, second_node_name)"]
    /// Returns the unweighted Resource Allocation Index for the given pair of nodes from the given node names.
    ///
    /// Parameters
    /// ----------
    /// first_node_name: str,
    ///     Node name of the first node.
    /// second_node_name: str,
    ///     Node name of the second node.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If either of the given node names do not exist in the current graph.
    ///
    pub fn get_resource_allocation_index_from_node_names(
        &self,
        first_node_name: &str,
        second_node_name: &str,
    ) -> PyResult<f64> {
        pe!(self
            .graph
            .get_resource_allocation_index_from_node_names(first_node_name, second_node_name))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, source_node_id, destination_node_id)"]
    /// Returns the weighted Resource Allocation Index for the given pair of nodes from the given node IDs.
    ///
    /// Parameters
    /// ----------
    /// source_node_id: int,
    ///     Node ID of the first node.
    /// destination_node_id: int,
    ///     Node ID of the second node.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If either of the node IDs are higher than the number of nodes in the graph.
    ///
    pub fn get_weighted_resource_allocation_index_from_node_ids(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
    ) -> PyResult<f64> {
        pe!(self
            .graph
            .get_weighted_resource_allocation_index_from_node_ids(
                source_node_id,
                destination_node_id
            ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, first_node_name, second_node_name)"]
    /// Returns the weighted Resource Allocation Index for the given pair of nodes from the given node names.
    ///
    /// Parameters
    /// ----------
    /// first_node_name: str,
    ///     Node name of the first node.
    /// second_node_name: str,
    ///     Node name of the second node.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If either of the given node names do not exist in the current graph.
    ///
    pub fn get_weighted_resource_allocation_index_from_node_names(
        &self,
        first_node_name: &str,
        second_node_name: &str,
    ) -> PyResult<f64> {
        pe!(self
            .graph
            .get_weighted_resource_allocation_index_from_node_names(
                first_node_name,
                second_node_name
            ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, source_node_id, destination_node_id, normalize)"]
    /// Returns all the implemented edge metrics for the two given node IDs.
    ///
    /// Specifically, the returned values are:
    /// * Adamic Adar
    /// * Jaccard coefficient
    /// * Resource allocation index
    /// * Preferential attachment
    ///
    /// Parameters
    /// ----------
    /// source_node_id: int,
    ///     Node ID of the first node.
    /// destination_node_id: int,
    ///     Node ID of the second node.
    /// normalize: bool,
    ///     Whether to normalize within 0 to 1.
    ///
    ///
    /// Safety
    /// ------
    /// If the given node IDs do not exist in the graph this method will panic.
    pub unsafe fn get_unchecked_all_edge_metrics_from_node_ids(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
        normalize: bool,
    ) -> Py<PyArray1<f64>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(
            gil,
            self.graph.get_unchecked_all_edge_metrics_from_node_ids(
                source_node_id,
                destination_node_id,
                normalize
            ),
            f64
        )
    }
}
