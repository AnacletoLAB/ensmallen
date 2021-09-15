use super::*;
use graph::{EdgeT, EdgeTypeT, NodeT, NodeTypeT, Result, WeightT};
use pyo3::class::basic::PyObjectProtocol;
use pyo3::{wrap_pyfunction, wrap_pymodule};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use strsim::*;

/// Returns the given method name separated in the component parts.
///
/// # Implementative details
/// The methods contains terms such as:
/// * `node_name`
/// * `node_type_id`
/// * `node_id`
///
/// Since these terms are functionally a single word, we do not split
/// the terms composed by the words:
/// * `id` or `ids`
/// * `type` or `types`
/// * `name` or `names`
///
/// # Arguments
/// * `method_name`: &str - Name of the method to split.
fn split_words(method_name: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for word in method_name.split("_") {
        match word {
            "type" | "types" | "id" | "ids" | "name" | "names" => match result.last_mut() {
                Some(last) => {
                    last.push('_');
                    last.extend(word.chars());
                }
                None => {
                    result.push(word.to_string());
                }
            },
            _ => {
                result.push(word.to_string());
            }
        };
    }

    result.into_iter().filter(|x| !x.is_empty()).collect()
}

#[pymodule]
fn ensmallen(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Graph>()?;
    m.add_class::<ShortestPathsDjkstra>()?;
    m.add_class::<ShortestPathsResultBFS>()?;
    m.add_wrapped(wrap_pymodule!(edge_list_utils))?;
    m.add_wrapped(wrap_pymodule!(utils))?;
    m.add_wrapped(wrap_pymodule!(preprocessing))?;
    Ok(())
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct Graph {
    pub inner: graph::Graph,
}

impl From<graph::Graph> for Graph {
    fn from(val: graph::Graph) -> Graph {
        Graph { inner: val }
    }
}

impl From<Graph> for graph::Graph {
    fn from(val: Graph) -> graph::Graph {
        val.inner
    }
}

#[pymethods]
impl Graph {
    #[automatically_generated_binding]
    #[text_signature = "($self, other)"]
    /// Return whether nodes are remappable to those of the given graph.
    ///
    /// Parameters
    /// ----------
    /// other: Graph,
    ///     graph towards remap the nodes to.
    ///
    pub fn are_nodes_remappable(&self, other: &Graph) -> bool {
        self.inner.are_nodes_remappable(&other.inner).into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_ids)"]
    /// Returns graph remapped using given node IDs ordering.
    ///
    /// Parameters
    /// ----------
    /// node_ids: List[int],
    ///     The node Ids to remap the graph to.
    ///
    ///
    /// Safety
    /// ------
    /// This method will cause a panic if the node IDs are either:
    ///  * Not unique
    ///  * Not available for each of the node IDs of the graph.
    pub unsafe fn remap_unchecked_from_node_ids(&self, node_ids: Vec<NodeT>) -> Graph {
        self.inner
            .remap_unchecked_from_node_ids(node_ids.into())
            .into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_ids)"]
    /// Returns graph remapped using given node IDs ordering.
    ///
    /// Parameters
    /// ----------
    /// node_ids: List[int],
    ///     The node Ids to remap the graph to.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given node IDs are not unique.
    /// ValueError
    ///     If the given node IDs are not available for all the values in the graph.
    ///
    pub fn remap_from_node_ids(&self, node_ids: Vec<NodeT>) -> PyResult<Graph> {
        Ok(pe!(self.inner.remap_from_node_ids(node_ids.into()))?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_names)"]
    /// Returns graph remapped using given node names ordering.
    ///
    /// Parameters
    /// ----------
    /// node_names: List[str],
    ///     The node names to remap the graph to.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given node names are not unique.
    /// ValueError
    ///     If the given node names are not available for all the values in the graph.
    ///
    pub fn remap_from_node_names(&self, node_names: Vec<&str>) -> PyResult<Graph> {
        Ok(pe!(self.inner.remap_from_node_names(node_names.into()))?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, other)"]
    /// Return graph remapped towards nodes of the given graph.
    ///
    /// Parameters
    /// ----------
    /// other: Graph,
    ///     The graph to remap towards.
    ///
    pub fn remap_from_graph(&self, other: &Graph) -> PyResult<Graph> {
        Ok(pe!(self.inner.remap_from_graph(&other.inner))?.into())
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
        pe!(self.inner.set_inplace_all_edge_types(edge_type))?;
        Ok(())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_type)"]
    /// Replace all edge types (if present) and set all the edge to edge_type.
    ///
    /// This DOES NOT happen inplace, but created a new instance of the graph.
    ///
    /// Parameters
    /// ----------
    /// edge_type: str,
    ///     The edge type to assing to all the edges.
    ///
    pub fn set_all_edge_types(&self, edge_type: String) -> PyResult<Graph> {
        Ok(pe!(self.inner.set_all_edge_types(edge_type))?.into())
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
        pe!(self.inner.set_inplace_all_node_types(node_type))?;
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
    pub fn set_all_node_types(&self, node_type: String) -> PyResult<Graph> {
        Ok(pe!(self.inner.set_all_node_types(node_type))?.into())
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
            .inner
            .remove_inplace_node_type_ids(node_type_ids_to_remove.into()))?;
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
        pe!(self.inner.remove_inplace_singleton_node_types())?;
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
            .inner
            .remove_inplace_edge_type_ids(edge_type_ids_to_remove.into()))?;
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
        pe!(self.inner.remove_inplace_singleton_edge_types())?;
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
        pe!(self
            .inner
            .remove_inplace_node_type_name(node_type_name.into()))?;
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
    pub fn remove_node_type_id(&self, node_type_id: NodeTypeT) -> PyResult<Graph> {
        Ok(pe!(self.inner.remove_node_type_id(node_type_id.into()))?.into())
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
    pub fn remove_singleton_node_types(&self) -> PyResult<Graph> {
        Ok(pe!(self.inner.remove_singleton_node_types())?.into())
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
    pub fn remove_node_type_name(&self, node_type_name: &str) -> PyResult<Graph> {
        Ok(pe!(self.inner.remove_node_type_name(node_type_name.into()))?.into())
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
        pe!(self
            .inner
            .remove_inplace_edge_type_name(edge_type_name.into()))?;
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
    pub fn remove_edge_type_id(&self, edge_type_id: EdgeTypeT) -> PyResult<Graph> {
        Ok(pe!(self.inner.remove_edge_type_id(edge_type_id.into()))?.into())
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
    pub fn remove_singleton_edge_types(&self) -> PyResult<Graph> {
        Ok(pe!(self.inner.remove_singleton_edge_types())?.into())
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
    pub fn remove_edge_type_name(&self, edge_type_name: &str) -> PyResult<Graph> {
        Ok(pe!(self.inner.remove_edge_type_name(edge_type_name.into()))?.into())
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
        pe!(self.inner.remove_inplace_node_types())?;
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
    pub fn remove_node_types(&self) -> PyResult<Graph> {
        Ok(pe!(self.inner.remove_node_types())?.into())
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
        pe!(self.inner.remove_inplace_edge_types())?;
        Ok(())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Remove edge types from the graph.
    ///
    /// Note that the modification does not happen inplace.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    ///
    pub fn remove_edge_types(&self) -> PyResult<Graph> {
        Ok(pe!(self.inner.remove_edge_types())?.into())
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
        pe!(self.inner.remove_inplace_edge_weights())?;
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
    pub fn remove_edge_weights(&self) -> PyResult<Graph> {
        Ok(pe!(self.inner.remove_edge_weights())?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_type_name, weight)"]
    /// Returns new graph with added in missing self-loops with given edge type and weight.
    ///
    /// Parameters
    /// ----------
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the edge type for the new selfloops is provided but the graph does not have edge types.
    /// ValueError
    ///     If the edge weight for the new selfloops is provided but the graph does not have edge weights.
    /// ValueError
    ///     If the edge weight for the new selfloops is NOT provided but the graph does have edge weights.
    ///
    pub fn add_selfloops(
        &self,
        edge_type_name: Option<&str>,
        weight: Option<WeightT>,
    ) -> PyResult<Graph> {
        Ok(pe!(self
            .inner
            .add_selfloops(edge_type_name.into(), weight.into()))?
        .into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, other)"]
    /// Return whether given graph has any edge overlapping with current graph.
    ///
    /// Parameters
    /// ----------
    /// other: Graph,
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
    pub fn overlaps(&self, other: &Graph) -> PyResult<bool> {
        Ok(pe!(self.inner.overlaps(&other.inner))?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, other)"]
    /// Return true if given graph edges are all contained within current graph.
    ///
    /// Parameters
    /// ----------
    /// other: Graph,
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
    pub fn contains(&self, other: &Graph) -> PyResult<bool> {
        Ok(pe!(self.inner.contains(&other.inner))?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, number_of_nodes_to_sample, random_state)"]
    /// Return random unique sorted numbers.
    ///
    /// Parameters
    /// ----------
    /// number_of_nodes_to_sample: int,
    ///     The number of nodes to sample.
    /// random_state: int,
    ///     The random state to use to reproduce the sampling.
    ///
    pub fn get_random_nodes(
        &self,
        number_of_nodes_to_sample: NodeT,
        random_state: u64,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self
                .inner
                .get_random_nodes(number_of_nodes_to_sample.into(), random_state.into()))?,
            NodeT
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, number_of_nodes_to_sample, root_node)"]
    /// Return nodes sampled from the neighbourhood of given root nodes.
    ///
    /// Parameters
    /// ----------
    /// number_of_nodes_to_sample: int,
    ///     The number of nodes to sample.
    /// root_node: int,
    ///     The root node from .
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the number of requested nodes is higher than the number of nodes in the graph.
    /// ValueError
    ///     If the given root node does not exist in the curret graph instance.
    ///
    pub fn get_breadth_first_search_random_nodes(
        &self,
        number_of_nodes_to_sample: NodeT,
        root_node: NodeT,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self.inner.get_breadth_first_search_random_nodes(
                number_of_nodes_to_sample.into(),
                root_node.into()
            ))?,
            NodeT
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node, random_state, walk_length, unique)"]
    /// Returns unique nodes sampled from uniform random walk.
    ///
    /// Parameters
    /// ----------
    /// node: int,
    ///     Node from where to start the random walks.
    /// random_state: int,
    ///     the random_state to use for extracting the nodes and edges.
    /// walk_length: int,
    ///     Length of the random walk.
    /// unique: Optional[bool],
    ///     Whether to make the sampled nodes unique.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given node does not exist in the current slack.
    ///
    pub fn get_uniform_random_walk_random_nodes(
        &self,
        node: NodeT,
        random_state: u64,
        walk_length: u64,
        unique: Option<bool>,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self.inner.get_uniform_random_walk_random_nodes(
                node.into(),
                random_state.into(),
                walk_length.into(),
                unique.into()
            ))?,
            NodeT
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return list of the supported node sampling methods
    pub fn get_node_sampling_methods(&self) -> Vec<&str> {
        self.inner.get_node_sampling_methods().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, number_of_nodes_to_sample, random_state, root_node, node_sampling_method, unique)"]
    /// Return subsampled nodes according to the given method and parameters.
    ///
    /// Parameters
    /// ----------
    /// number_of_nodes_to_sample: int,
    ///     The number of nodes to sample.
    /// random_state: int,
    ///     The random state to reproduce the sampling.
    /// root_node: Optional[int],
    ///     The (optional) root node to use to sample. In not provided, a random one is sampled.
    /// node_sampling_method: str,
    ///     The method to use to sample the nodes. Can either be random nodes, breath first search-based or uniform random walk-based.
    /// unique: Optional[bool],
    ///     Whether to make the sampled nodes unique.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given node sampling method is not supported.
    ///
    pub fn get_subsampled_nodes(
        &self,
        number_of_nodes_to_sample: NodeT,
        random_state: u64,
        root_node: Option<NodeT>,
        node_sampling_method: &str,
        unique: Option<bool>,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self.inner.get_subsampled_nodes(
                number_of_nodes_to_sample.into(),
                random_state.into(),
                root_node.into(),
                node_sampling_method.into(),
                unique.into()
            ))?,
            NodeT
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return if graph has name that is not the default one.
    ///
    /// TODO: use a default for the default graph name
    pub fn has_default_graph_name(&self) -> bool {
        self.inner.has_default_graph_name().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return if the graph has any nodes.
    pub fn has_nodes(&self) -> bool {
        self.inner.has_nodes().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return if the graph has any edges.
    pub fn has_edges(&self) -> bool {
        self.inner.has_edges().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return whether the graph has trap nodes.
    pub fn has_trap_nodes(&self) -> bool {
        self.inner.has_trap_nodes().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns boolean representing if graph is directed.
    pub fn is_directed(&self) -> bool {
        self.inner.is_directed().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns boolean representing whether graph has weights.
    pub fn has_edge_weights(&self) -> bool {
        self.inner.has_edge_weights().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns whether graph has weights that can represent probabilities
    pub fn has_edge_weights_representing_probabilities(&self) -> PyResult<bool> {
        Ok(pe!(self.inner.has_edge_weights_representing_probabilities())?.into())
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
        Ok(pe!(self.inner.has_weighted_singleton_nodes())?.into())
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
        Ok(pe!(self.inner.has_constant_edge_weights())?.into())
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
        Ok(pe!(self.inner.has_negative_edge_weights())?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns boolean representing whether graph has edge types.
    pub fn has_edge_types(&self) -> bool {
        self.inner.has_edge_types().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns boolean representing if graph has self-loops.
    pub fn has_selfloops(&self) -> bool {
        self.inner.has_selfloops().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns boolean representing if nodes which are nor singletons nor
    /// singletons with selfloops.
    pub fn has_disconnected_nodes(&self) -> bool {
        self.inner.has_disconnected_nodes().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns boolean representing if graph has singletons.
    pub fn has_singleton_nodes(&self) -> bool {
        self.inner.has_singleton_nodes().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns boolean representing if graph has singletons
    pub fn has_singleton_nodes_with_selfloops(&self) -> bool {
        self.inner.has_singleton_nodes_with_selfloops().into()
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
        self.inner.is_connected(verbose.into()).into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns boolean representing if graph has node types
    pub fn has_node_types(&self) -> bool {
        self.inner.has_node_types().into()
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
        Ok(pe!(self.inner.has_multilabel_node_types())?.into())
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
        Ok(pe!(self.inner.has_unknown_node_types())?.into())
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
        Ok(pe!(self.inner.has_known_node_types())?.into())
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
        Ok(pe!(self.inner.has_unknown_edge_types())?.into())
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
        Ok(pe!(self.inner.has_known_edge_types())?.into())
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
        Ok(pe!(self.inner.has_homogeneous_node_types())?.into())
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
        Ok(pe!(self.inner.has_homogeneous_edge_types())?.into())
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
        Ok(pe!(self.inner.has_singleton_node_types())?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return whether the graph has any known node-related graph oddities
    pub fn has_node_oddities(&self) -> bool {
        self.inner.has_node_oddities().into()
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
        Ok(pe!(self.inner.has_node_types_oddities())?.into())
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
        Ok(pe!(self.inner.has_singleton_edge_types())?.into())
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
        Ok(pe!(self.inner.has_edge_types_oddities())?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return if there are multiple edges between two node
    pub fn is_multigraph(&self) -> bool {
        self.inner.is_multigraph().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns whether the node IDs are sorted by decreasing outbound node degree.
    pub fn has_nodes_sorted_by_decreasing_outbound_node_degree(&self) -> bool {
        self.inner
            .has_nodes_sorted_by_decreasing_outbound_node_degree()
            .into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns whether the node IDs are sorted by decreasing outbound node degree.
    pub fn has_nodes_sorted_by_lexicographic_order(&self) -> bool {
        self.inner.has_nodes_sorted_by_lexicographic_order().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns whether the graph contains the indentity matrix.
    pub fn contains_identity_matrix(&self) -> bool {
        self.inner.contains_identity_matrix().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns whether the node IDs are sorted by increasing outbound node degree.
    pub fn has_nodes_sorted_by_increasing_outbound_node_degree(&self) -> bool {
        self.inner
            .has_nodes_sorted_by_increasing_outbound_node_degree()
            .into()
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
        self.inner
            .get_connected_components_number(verbose.into())
            .into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns number of connected nodes in the graph.
    pub fn get_connected_nodes_number(&self) -> NodeT {
        self.inner.get_connected_nodes_number().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns number of singleton nodes with selfloops within the graph.
    pub fn get_singleton_nodes_with_selfloops_number(&self) -> NodeT {
        self.inner
            .get_singleton_nodes_with_selfloops_number()
            .into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns number of singleton nodes within the graph.
    pub fn get_singleton_nodes_number(&self) -> NodeT {
        self.inner.get_singleton_nodes_number().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns number of disconnected nodes within the graph.
    /// A Disconnected node is a node which is nor a singleton nor a singleton
    /// with selfloops.
    pub fn get_disconnected_nodes_number(&self) -> NodeT {
        self.inner.get_disconnected_nodes_number().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns vector of singleton node IDs of the graph.
    pub fn get_singleton_node_ids(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_singleton_node_ids(), NodeT)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns vector of singleton node names of the graph.
    pub fn get_singleton_node_names(&self) -> Vec<String> {
        self.inner.get_singleton_node_names().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns vector of singleton_with_selfloops node IDs of the graph.
    pub fn get_singleton_with_selfloops_node_ids(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(
            gil,
            self.inner.get_singleton_with_selfloops_node_ids(),
            NodeT
        )
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns vector of singleton_with_selfloops node names of the graph.
    pub fn get_singleton_with_selfloops_node_names(&self) -> Vec<String> {
        self.inner.get_singleton_with_selfloops_node_names().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns density of the graph.
    pub fn get_density(&self) -> PyResult<f64> {
        Ok(pe!(self.inner.get_density())?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns the traps rate of the graph.
    ///
    /// THIS IS EXPERIMENTAL AND MUST BE PROVEN!
    pub fn get_trap_nodes_rate(&self) -> f64 {
        self.inner.get_trap_nodes_rate().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns unweighted mean node degree of the graph.
    pub fn get_node_degrees_mean(&self) -> PyResult<f64> {
        Ok(pe!(self.inner.get_node_degrees_mean())?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns weighted mean node degree of the graph.
    pub fn get_weighted_node_degrees_mean(&self) -> PyResult<f64> {
        Ok(pe!(self.inner.get_weighted_node_degrees_mean())?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns number of undirected edges of the graph.
    pub fn get_undirected_edges_number(&self) -> EdgeT {
        self.inner.get_undirected_edges_number().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns number of undirected edges of the graph.
    pub fn get_unique_undirected_edges_number(&self) -> EdgeT {
        self.inner.get_unique_undirected_edges_number().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns number of edges of the graph.
    pub fn get_edges_number(&self) -> EdgeT {
        self.inner.get_edges_number().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns number of unique edges of the graph.
    pub fn get_unique_edges_number(&self) -> EdgeT {
        self.inner.get_unique_edges_number().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns unweighted median node degree of the graph
    pub fn get_node_degrees_median(&self) -> PyResult<NodeT> {
        Ok(pe!(self.inner.get_node_degrees_median())?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns weighted median node degree of the graph
    pub fn get_weighted_node_degrees_median(&self) -> PyResult<f64> {
        Ok(pe!(self.inner.get_weighted_node_degrees_median())?.into())
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
        Ok(pe!(self.inner.get_maximum_node_degree())?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns maximum node degree of the graph.
    ///
    /// Safety
    /// ------
    /// This method fails with a panic if the graph does not have any node.
    pub unsafe fn get_unchecked_most_central_node_id(&self) -> NodeT {
        self.inner.get_unchecked_most_central_node_id().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns maximum node degree of the graph.
    pub fn get_most_central_node_id(&self) -> PyResult<NodeT> {
        Ok(pe!(self.inner.get_most_central_node_id())?.into())
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
        Ok(pe!(self.inner.get_minimum_node_degree())?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns mode node degree of the graph.
    pub fn get_node_degrees_mode(&self) -> PyResult<NodeT> {
        Ok(pe!(self.inner.get_node_degrees_mode())?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns rate of self-loops.
    pub fn get_selfloop_nodes_rate(&self) -> PyResult<f64> {
        Ok(pe!(self.inner.get_selfloop_nodes_rate())?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return name of the graph.
    pub fn get_name(&self) -> String {
        self.inner.get_name().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the number of traps (nodes without any outgoing edges that are not singletons)
    /// This also includes nodes with only a self-loops, therefore singletons with
    /// only a self-loops are not considered traps because you could make a walk on them.
    pub fn get_trap_nodes_number(&self) -> NodeT {
        self.inner.get_trap_nodes_number().into()
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
        to_ndarray_1d!(gil, self.inner.get_source_node_ids(directed.into()), NodeT)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return vector on the (non unique) directed source nodes of the graph
    pub fn get_directed_source_node_ids(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_directed_source_node_ids(), NodeT)
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
        self.inner.get_source_names(directed.into()).into()
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
        to_ndarray_1d!(
            gil,
            self.inner.get_destination_node_ids(directed.into()),
            NodeT
        )
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return vector on the (non unique) directed destination nodes of the graph
    pub fn get_directed_destination_node_ids(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_directed_destination_node_ids(), NodeT)
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
        self.inner.get_destination_names(directed.into()).into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return vector with the sorted nodes names
    pub fn get_node_names(&self) -> Vec<String> {
        self.inner.get_node_names().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return vector with the node URLs.
    pub fn get_node_urls(&self) -> Vec<Option<String>> {
        self.inner.get_node_urls().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return vector with the node predicted ontology.
    pub fn get_node_ontologies(&self) -> Vec<Option<String>> {
        self.inner.get_node_ontologies().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return vector with the sorted nodes Ids
    pub fn get_node_ids(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_node_ids(), NodeT)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the edge types of the edges
    pub fn get_edge_type_ids(&self) -> PyResult<Vec<Option<EdgeTypeT>>> {
        Ok(pe!(self.inner.get_edge_type_ids())?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the unique edge type IDs of the graph edges.
    pub fn get_unique_edge_type_ids(&self) -> PyResult<Py<PyArray1<EdgeTypeT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self.inner.get_unique_edge_type_ids())?,
            EdgeTypeT
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the edge types names
    pub fn get_edge_type_names(&self) -> PyResult<Vec<Option<String>>> {
        Ok(pe!(self.inner.get_edge_type_names())?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the edge types names
    pub fn get_unique_edge_type_names(&self) -> PyResult<Vec<String>> {
        Ok(pe!(self.inner.get_unique_edge_type_names())?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the weights of the graph edges.
    pub fn get_edge_weights(&self) -> PyResult<Py<PyArray1<WeightT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self.inner.get_edge_weights())?,
            WeightT
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the weighted indegree (total weighted inbound edge weights) for each node.
    pub fn get_weighted_node_indegrees(&self) -> PyResult<Py<PyArray1<f64>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self.inner.get_weighted_node_indegrees())?,
            f64
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the node types of the graph nodes.
    pub fn get_node_type_ids(&self) -> PyResult<Vec<Option<Vec<NodeTypeT>>>> {
        Ok(pe!(self.inner.get_node_type_ids())?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns boolean mask of known node types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn get_known_node_types_mask(&self) -> PyResult<Py<PyArray1<bool>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self.inner.get_known_node_types_mask())?,
            bool
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns boolean mask of unknown node types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn get_unknown_node_types_mask(&self) -> PyResult<Py<PyArray1<bool>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self.inner.get_unknown_node_types_mask())?,
            bool
        ))
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
            pe!(self.inner.get_one_hot_encoded_node_types())?,
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
            pe!(self.inner.get_one_hot_encoded_known_node_types())?,
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
            pe!(self.inner.get_one_hot_encoded_edge_types())?,
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
            pe!(self.inner.get_one_hot_encoded_known_edge_types())?,
            bool
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the node types names.
    pub fn get_node_type_names(&self) -> PyResult<Vec<Option<Vec<String>>>> {
        Ok(pe!(self.inner.get_node_type_names())?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the unique node type IDs of the graph nodes.
    pub fn get_unique_node_type_ids(&self) -> PyResult<Py<PyArray1<NodeTypeT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self.inner.get_unique_node_type_ids())?,
            NodeTypeT
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the unique node types names.
    pub fn get_unique_node_type_names(&self) -> PyResult<Vec<String>> {
        Ok(pe!(self.inner.get_unique_node_type_names())?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return number of the unique edges in the graph
    pub fn get_unique_directed_edges_number(&self) -> EdgeT {
        self.inner.get_unique_directed_edges_number().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the nodes mapping
    pub fn get_nodes_mapping(&self) -> HashMap<String, NodeT> {
        self.inner.get_nodes_mapping().into()
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
        to_ndarray_2d!(gil, self.inner.get_edge_node_ids(directed.into()), NodeT)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return vector with the sorted directed edge Ids
    pub fn get_directed_edge_node_ids(&self) -> Py<PyArray2<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_2d!(gil, self.inner.get_directed_edge_node_ids(), NodeT)
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
        self.inner.get_edge_node_names(directed.into()).into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return vector with the sorted directed edge names
    pub fn get_directed_edge_node_names(&self) -> Vec<(String, String)> {
        self.inner.get_directed_edge_node_names().into()
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
        Ok(pe!(self.inner.get_unknown_node_types_number())?.into())
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
        Ok(pe!(self.inner.get_known_node_types_number())?.into())
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
        Ok(pe!(self.inner.get_unknown_node_types_rate())?.into())
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
        Ok(pe!(self.inner.get_known_node_types_rate())?.into())
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
        Ok(pe!(self.inner.get_minimum_node_types_number())?.into())
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
        Ok(pe!(self.inner.get_maximum_node_types_number())?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns number of maximum multilabel count.
    ///
    /// This value is the maximum number of multilabel counts
    /// that appear in any given node in the graph
    pub fn get_maximum_multilabel_count(&self) -> PyResult<NodeTypeT> {
        Ok(pe!(self.inner.get_maximum_multilabel_count())?.into())
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
        Ok(pe!(self.inner.get_singleton_node_types_number())?.into())
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
            pe!(self.inner.get_singleton_node_type_ids())?,
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
        Ok(pe!(self.inner.get_singleton_node_type_names())?.into())
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
        Ok(pe!(self.inner.get_unknown_edge_types_number())?.into())
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
            pe!(self.inner.get_edge_ids_with_unknown_edge_types())?,
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
            pe!(self.inner.get_edge_ids_with_known_edge_types())?,
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
        Ok(pe!(self
            .inner
            .get_edge_node_ids_with_unknown_edge_types(directed.into()))?
        .into())
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
        Ok(pe!(self
            .inner
            .get_edge_node_ids_with_known_edge_types(directed.into()))?
        .into())
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
        Ok(pe!(self
            .inner
            .get_edge_node_names_with_unknown_edge_types(directed.into()))?
        .into())
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
        Ok(pe!(self
            .inner
            .get_edge_node_names_with_known_edge_types(directed.into()))?
        .into())
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
            pe!(self.inner.get_edge_ids_with_unknown_edge_types_mask())?,
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
            pe!(self.inner.get_edge_ids_with_known_edge_types_mask())?,
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
            pe!(self.inner.get_node_ids_with_unknown_node_types())?,
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
            pe!(self.inner.get_node_ids_with_known_node_types())?,
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
        Ok(pe!(self.inner.get_node_names_with_unknown_node_types())?.into())
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
        Ok(pe!(self.inner.get_node_names_with_known_node_types())?.into())
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
            pe!(self.inner.get_node_ids_with_unknown_node_types_mask())?,
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
            pe!(self.inner.get_node_ids_with_known_node_types_mask())?,
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
        Ok(pe!(self.inner.get_known_edge_types_number())?.into())
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
        Ok(pe!(self.inner.get_unknown_edge_types_rate())?.into())
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
        Ok(pe!(self.inner.get_known_edge_types_rate())?.into())
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
        Ok(pe!(self.inner.get_minimum_edge_types_number())?.into())
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
        Ok(pe!(self.inner.get_singleton_edge_types_number())?.into())
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
            pe!(self.inner.get_singleton_edge_type_ids())?,
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
        Ok(pe!(self.inner.get_singleton_edge_type_names())?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns number of nodes in the graph
    pub fn get_nodes_number(&self) -> NodeT {
        self.inner.get_nodes_number().into()
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
            self.inner.get_node_connected_component_ids(verbose.into()),
            NodeT
        )
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns number of directed edges in the graph
    pub fn get_directed_edges_number(&self) -> EdgeT {
        self.inner.get_directed_edges_number().into()
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
        Ok(pe!(self.inner.get_edge_types_number())?.into())
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
        Ok(pe!(self.inner.get_node_types_number())?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns the unweighted degree of every node in the graph
    pub fn get_node_degrees(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_node_degrees(), NodeT)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the indegree for each node.
    pub fn get_node_indegrees(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_node_indegrees(), NodeT)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns the weighted degree of every node in the graph
    pub fn get_weighted_node_degrees(&self) -> PyResult<Py<PyArray1<f64>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self.inner.get_weighted_node_degrees())?,
            f64
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return set of nodes that are not singletons
    pub fn get_not_singletons_node_ids(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_not_singletons_node_ids(), NodeT)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return mapping from instance not trap nodes to dense nodes
    pub fn get_dense_nodes_mapping(&self) -> HashMap<NodeT, NodeT> {
        self.inner.get_dense_nodes_mapping().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return number of edges that have multigraph syblings
    pub fn get_parallel_edges_number(&self) -> EdgeT {
        self.inner.get_parallel_edges_number().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return vector with node cumulative_node_degrees, that is the comulative node degree
    pub fn get_cumulative_node_degrees(&self) -> Py<PyArray1<EdgeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_cumulative_node_degrees(), EdgeT)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns number of the source nodes.
    pub fn get_unique_source_nodes_number(&self) -> NodeT {
        self.inner.get_unique_source_nodes_number().into()
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
        Ok(pe!(self.inner.get_edge_type_id_counts_hashmap())?.into())
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
        Ok(pe!(self.inner.get_edge_type_names_counts_hashmap())?.into())
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
        Ok(pe!(self.inner.get_node_type_id_counts_hashmap())?.into())
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
        Ok(pe!(self.inner.get_node_type_names_counts_hashmap())?.into())
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
        Ok(pe!(self.inner.get_total_edge_weights())?.into())
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
        Ok(pe!(self.inner.get_mininum_edge_weight())?.into())
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
        Ok(pe!(self.inner.get_maximum_edge_weight())?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the maximum node degree.
    ///
    /// Safety
    /// ------
    /// The method will return an undefined value (0) when the graph
    /// does not contain nodes. In those cases the value is not properly
    /// defined.
    pub unsafe fn get_unchecked_maximum_node_degree(&self) -> NodeT {
        self.inner.get_unchecked_maximum_node_degree().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the minimum node degree.
    ///
    /// Safety
    /// ------
    /// The method will return an undefined value (0) when the graph
    /// does not contain nodes. In those cases the value is not properly
    /// defined.
    pub unsafe fn get_unchecked_minimum_node_degree(&self) -> NodeT {
        self.inner.get_unchecked_minimum_node_degree().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the maximum weighted node degree
    pub fn get_weighted_maximum_node_degree(&self) -> PyResult<f64> {
        Ok(pe!(self.inner.get_weighted_maximum_node_degree())?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the minimum weighted node degree
    pub fn get_weighted_minimum_node_degree(&self) -> PyResult<f64> {
        Ok(pe!(self.inner.get_weighted_minimum_node_degree())?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the number of weighted singleton nodes, i.e. nodes with weighted node degree equal to zero
    pub fn get_weighted_singleton_nodes_number(&self) -> PyResult<NodeT> {
        Ok(pe!(self.inner.get_weighted_singleton_nodes_number())?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns number of self-loops, including also those in eventual multi-edges.
    pub fn get_selfloops_number(&self) -> EdgeT {
        self.inner.get_selfloops_number().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns number of unique self-loops, excluding those in eventual multi-edges.
    pub fn get_unique_selfloops_number(&self) -> NodeT {
        self.inner.get_unique_selfloops_number().into()
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
        self.inner
            .is_unchecked_connected_from_node_id(node_id.into())
            .into()
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
    pub unsafe fn is_unchecked_disconnected_node_from_node_id(&self, node_id: NodeT) -> bool {
        self.inner
            .is_unchecked_disconnected_node_from_node_id(node_id.into())
            .into()
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
        self.inner
            .is_unchecked_singleton_from_node_id(node_id.into())
            .into()
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
        Ok(pe!(self.inner.is_singleton_from_node_id(node_id.into()))?.into())
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
    pub unsafe fn is_unchecked_singleton_with_selfloops_from_node_id(
        &self,
        node_id: NodeT,
    ) -> bool {
        self.inner
            .is_unchecked_singleton_with_selfloops_from_node_id(node_id.into())
            .into()
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
    pub fn is_singleton_with_selfloops_from_node_id(&self, node_id: NodeT) -> PyResult<bool> {
        Ok(pe!(self
            .inner
            .is_singleton_with_selfloops_from_node_id(node_id.into()))?
        .into())
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
        self.inner
            .is_unchecked_singleton_from_node_name(node_name.into())
            .into()
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
        Ok(pe!(self.inner.is_singleton_from_node_name(node_name.into()))?.into())
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
        self.inner.has_node_name(node_name.into()).into()
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
        self.inner.has_node_type_id(node_type_id.into()).into()
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
        self.inner.has_node_type_name(node_type_name.into()).into()
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
        self.inner.has_edge_type_id(edge_type_id.into()).into()
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
        self.inner.has_edge_type_name(edge_type_name.into()).into()
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
        self.inner
            .has_edge_from_node_ids(src.into(), dst.into())
            .into()
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
        self.inner.has_selfloop_from_node_id(node_id.into()).into()
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
        self.inner
            .has_edge_from_node_ids_and_edge_type_id(src.into(), dst.into(), edge_type.into())
            .into()
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
        self.inner
            .is_unchecked_trap_node_from_node_id(node_id.into())
            .into()
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
        Ok(pe!(self.inner.is_trap_node_from_node_id(node_id.into()))?.into())
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
        self.inner
            .has_node_name_and_node_type_name(node_name.into(), node_type_name.into())
            .into()
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
        self.inner
            .has_edge_from_node_names(src_name.into(), dst_name.into())
            .into()
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
        self.inner
            .has_edge_from_node_names_and_edge_type_name(
                src_name.into(),
                dst_name.into(),
                edge_type_name.into(),
            )
            .into()
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
        Ok(pe!(self.inner.validate_node_id(node_id.into()))?.into())
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
            pe!(self.inner.validate_node_ids(node_ids.into()))?,
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
        Ok(pe!(self.inner.validate_edge_id(edge_id.into()))?.into())
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
            pe!(self.inner.validate_edge_ids(edge_ids.into()))?,
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
        Ok(pe!(self.inner.must_not_contain_unknown_node_types())?.into())
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
        Ok(pe!(self.inner.must_not_contain_unknown_edge_types())?.into())
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
        Ok(pe!(self.inner.validate_node_type_id(node_type_id.into()))?.into())
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
        Ok(pe!(self.inner.validate_node_type_ids(node_type_ids.into()))?.into())
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
        Ok(pe!(self.inner.validate_edge_type_id(edge_type_id.into()))?.into())
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
        Ok(pe!(self.inner.validate_edge_type_ids(edge_type_ids.into()))?.into())
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
        Ok(pe!(self.inner.must_be_undirected())?.into())
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
        Ok(pe!(self.inner.must_be_multigraph())?.into())
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
        Ok(pe!(self.inner.must_not_be_multigraph())?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Raises an error if the graph does not include the identity matrix.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph is a multigraph.
    ///
    pub fn must_contain_identity_matrix(&self) -> PyResult<()> {
        Ok(pe!(self.inner.must_contain_identity_matrix())?.into())
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
        Ok(pe!(self.inner.must_not_contain_weighted_singleton_nodes())?.into())
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
        Ok(pe!(self.inner.must_have_edges())?.into())
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
        Ok(pe!(self.inner.must_have_nodes())?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Raises an error if the graph is not connected.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph is not connected.
    ///
    pub fn must_be_connected(&self) -> PyResult<()> {
        Ok(pe!(self.inner.must_be_connected())?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_name_mapping, node_type_name_mapping, edge_type_name_mapping)"]
    /// Replace given node, node type and edge type names.
    ///
    /// Parameters
    /// ----------
    /// node_name_mapping: Optional[Dict[str, str]],
    ///     The node names to replace.
    /// node_type_name_mapping: Optional[Dict[str, str]],
    ///     The node type names to replace.
    /// edge_type_name_mapping: Optional[Dict[str, str]],
    ///     The edge type names to replace.
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
        edge_type_name_mapping: Option<HashMap<String, String>>,
    ) -> PyResult<Graph> {
        Ok(pe!(self.inner.replace(
            node_name_mapping.into(),
            node_type_name_mapping.into(),
            edge_type_name_mapping.into()
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Print the current graph in a format compatible with Graphviz dot's format
    pub fn to_dot(&self) -> String {
        self.inner.to_dot().into()
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
    ) -> PyResult<Graph> {
        Ok(pe!(self.inner.remove_components(
            node_names.into(),
            node_types.into(),
            edge_types.into(),
            minimum_component_size.into(),
            top_k_components.into(),
            verbose.into()
        ))?
        .into())
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
        Ok(pe!(self.inner.enable(
            vector_sources.into(),
            vector_destinations.into(),
            vector_cumulative_node_degrees.into()
        ))?
        .into())
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
        self.inner
            .random_spanning_arborescence_kruskal(
                random_state.into(),
                undesired_edge_types.into(),
                verbose.into(),
            )
            .into()
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
        self.inner
            .spanning_arborescence_kruskal(verbose.into())
            .into()
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
        Ok(pe!(self.inner.connected_components(verbose.into()))?.into())
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
    pub fn report(&self) -> HashMap<&'static str, String> {
        self.inner.report().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, other, verbose)"]
    /// Return rendered textual report about the graph overlaps.
    ///
    /// Parameters
    /// ----------
    /// other: Graph,
    ///     graph to create overlap report with.
    /// verbose: Optional[bool],
    ///     Whether to shor the loading bars.
    ///
    pub fn overlap_textual_report(&self, other: &Graph, verbose: Option<bool>) -> PyResult<String> {
        Ok(pe!(self
            .inner
            .overlap_textual_report(&other.inner, verbose.into()))?
        .into())
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
        Ok(pe!(self.inner.get_node_report_from_node_id(node_id.into()))?.into())
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
        Ok(pe!(self.inner.get_node_report_from_node_name(node_name.into()))?.into())
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
        self.inner.textual_report().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns binary dense adjacency matrix.
    ///
    /// Beware of using this method on big graphs!
    /// It'll use all of your RAM!
    pub fn get_dense_binary_adjacency_matrix(&self) -> Py<PyArray2<bool>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_2d!(gil, self.inner.get_dense_binary_adjacency_matrix(), bool)
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
            pe!(self
                .inner
                .get_dense_weighted_adjacency_matrix(weight.into()))?,
            WeightT
        ))
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
    ) -> PyResult<Graph> {
        Ok(pe!(self.inner.generate_new_edges_from_node_features(
            features.into(),
            neighbours_number.into(),
            max_degree.into(),
            distance_name.into(),
            verbose.into()
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns the minumum unweighted preferential attachment score.
    ///
    /// Safety
    /// ------
    /// If the graph does not contain nodes, the return value will be undefined.
    pub unsafe fn get_unchecked_minimum_preferential_attachment(&self) -> f64 {
        self.inner
            .get_unchecked_minimum_preferential_attachment()
            .into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns the maximum unweighted preferential attachment score.
    ///
    /// Safety
    /// ------
    /// If the graph does not contain nodes, the return value will be undefined.
    pub unsafe fn get_unchecked_maximum_preferential_attachment(&self) -> f64 {
        self.inner
            .get_unchecked_maximum_preferential_attachment()
            .into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns the minumum weighted preferential attachment score.
    ///
    /// Safety
    /// ------
    /// If the graph does not contain nodes, the return value will be undefined.
    pub unsafe fn get_unchecked_weighted_minimum_preferential_attachment(&self) -> f64 {
        self.inner
            .get_unchecked_weighted_minimum_preferential_attachment()
            .into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns the maximum weighted preferential attachment score.
    ///
    /// Safety
    /// ------
    /// If the graph does not contain nodes, the return value will be undefined.
    pub unsafe fn get_unchecked_weighted_maximum_preferential_attachment(&self) -> f64 {
        self.inner
            .get_unchecked_weighted_maximum_preferential_attachment()
            .into()
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
        self.inner
            .get_unchecked_preferential_attachment_from_node_ids(
                source_node_id.into(),
                destination_node_id.into(),
                normalize.into(),
            )
            .into()
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
        Ok(pe!(self.inner.get_preferential_attachment_from_node_ids(
            source_node_id.into(),
            destination_node_id.into(),
            normalize.into()
        ))?
        .into())
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
        Ok(pe!(self.inner.get_preferential_attachment_from_node_names(
            first_node_name.into(),
            second_node_name.into(),
            normalize.into()
        ))?
        .into())
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
        self.inner
            .get_unchecked_weighted_preferential_attachment_from_node_ids(
                source_node_id.into(),
                destination_node_id.into(),
                normalize.into(),
            )
            .into()
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
        Ok(pe!(self
            .inner
            .get_weighted_preferential_attachment_from_node_ids(
                source_node_id.into(),
                destination_node_id.into(),
                normalize.into()
            ))?
        .into())
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
        Ok(pe!(self
            .inner
            .get_weighted_preferential_attachment_from_node_names(
                first_node_name.into(),
                second_node_name.into(),
                normalize.into()
            ))?
        .into())
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
        self.inner
            .get_unchecked_jaccard_coefficient_from_node_ids(
                source_node_id.into(),
                destination_node_id.into(),
            )
            .into()
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
        Ok(pe!(self.inner.get_jaccard_coefficient_from_node_ids(
            source_node_id.into(),
            destination_node_id.into()
        ))?
        .into())
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
        Ok(pe!(self.inner.get_jaccard_coefficient_from_node_names(
            first_node_name.into(),
            second_node_name.into()
        ))?
        .into())
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
        self.inner
            .get_unchecked_adamic_adar_index_from_node_ids(
                source_node_id.into(),
                destination_node_id.into(),
            )
            .into()
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
        Ok(pe!(self.inner.get_adamic_adar_index_from_node_ids(
            source_node_id.into(),
            destination_node_id.into()
        ))?
        .into())
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
        Ok(pe!(self.inner.get_adamic_adar_index_from_node_names(
            first_node_name.into(),
            second_node_name.into()
        ))?
        .into())
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
        self.inner
            .get_unchecked_resource_allocation_index_from_node_ids(
                source_node_id.into(),
                destination_node_id.into(),
            )
            .into()
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
        self.inner
            .get_unchecked_weighted_resource_allocation_index_from_node_ids(
                source_node_id.into(),
                destination_node_id.into(),
            )
            .into()
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
        Ok(pe!(self.inner.get_resource_allocation_index_from_node_ids(
            source_node_id.into(),
            destination_node_id.into()
        ))?
        .into())
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
        Ok(
            pe!(self.inner.get_resource_allocation_index_from_node_names(
                first_node_name.into(),
                second_node_name.into()
            ))?
            .into(),
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
        Ok(pe!(self
            .inner
            .get_weighted_resource_allocation_index_from_node_ids(
                source_node_id.into(),
                destination_node_id.into()
            ))?
        .into())
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
        Ok(pe!(self
            .inner
            .get_weighted_resource_allocation_index_from_node_names(
                first_node_name.into(),
                second_node_name.into()
            ))?
        .into())
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
            self.inner.get_unchecked_all_edge_metrics_from_node_ids(
                source_node_id.into(),
                destination_node_id.into(),
                normalize.into()
            ),
            f64
        )
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns list of nodes of the various strongly connected components.
    ///
    /// This is an implementation of Tarjan algorithm.
    pub fn strongly_connected_components(&self) -> Vec<HashSet<NodeT>> {
        self.inner.strongly_connected_components().into()
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
        self.inner
            .get_unchecked_edge_weight_from_edge_id(edge_id.into())
            .into()
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
        self.inner
            .get_unchecked_edge_weight_from_node_ids(src.into(), dst.into())
            .into()
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
        self.inner
            .get_unchecked_node_id_from_node_name(node_name.into())
            .into()
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
        self.inner
            .get_unchecked_edge_type_id_from_edge_type_name(edge_type_name.into())
            .into()
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
        self.inner
            .get_unchecked_edge_type_name_from_edge_type_id(edge_type_id.into())
            .into()
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
        self.inner
            .get_unchecked_edge_count_from_edge_type_id(edge_type.into())
            .into()
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
        self.inner
            .get_unchecked_edge_id_from_node_ids_and_edge_type_id(
                src.into(),
                dst.into(),
                edge_type.into(),
            )
            .into()
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
        self.inner
            .get_unchecked_minmax_edge_ids_from_node_ids(src.into(), dst.into())
            .into()
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
        self.inner
            .get_unchecked_node_ids_from_edge_id(edge_id.into())
            .into()
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
        self.inner
            .get_unchecked_node_names_from_edge_id(edge_id.into())
            .into()
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
        self.inner
            .get_unchecked_source_node_id_from_edge_id(edge_id.into())
            .into()
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
        self.inner
            .get_unchecked_destination_node_id_from_edge_id(edge_id.into())
            .into()
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
        Ok(pe!(self.inner.get_source_node_id_from_edge_id(edge_id.into()))?.into())
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
        Ok(pe!(self
            .inner
            .get_destination_node_id_from_edge_id(edge_id.into()))?
        .into())
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
        self.inner
            .get_unchecked_source_node_name_from_edge_id(edge_id.into())
            .into()
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
        self.inner
            .get_unchecked_destination_node_name_from_edge_id(edge_id.into())
            .into()
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
        Ok(pe!(self.inner.get_source_node_name_from_edge_id(edge_id.into()))?.into())
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
        Ok(pe!(self
            .inner
            .get_destination_node_name_from_edge_id(edge_id.into()))?
        .into())
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
        Ok(pe!(self.inner.get_node_names_from_edge_id(edge_id.into()))?.into())
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
        Ok(pe!(self.inner.get_node_ids_from_edge_id(edge_id.into()))?.into())
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
        self.inner
            .get_unchecked_edge_id_from_node_ids(src.into(), dst.into())
            .into()
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
        Ok(pe!(self.inner.get_edge_id_from_node_ids(src.into(), dst.into()))?.into())
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
        self.inner
            .get_unchecked_unique_source_node_id(source_id.into())
            .into()
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
        self.inner
            .get_unchecked_node_ids_and_edge_type_id_from_edge_id(edge_id.into())
            .into()
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
        Ok(pe!(self
            .inner
            .get_node_ids_and_edge_type_id_from_edge_id(edge_id.into()))?
        .into())
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
        self.inner
            .get_unchecked_node_ids_and_edge_type_id_and_edge_weight_from_edge_id(edge_id.into())
            .into()
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
        Ok(pe!(self
            .inner
            .get_node_ids_and_edge_type_id_and_edge_weight_from_edge_id(edge_id.into()))?
        .into())
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
            pe!(self.inner.get_top_k_central_node_ids(k.into()))?,
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
            pe!(self.inner.get_weighted_top_k_central_node_ids(k.into()))?,
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
        self.inner
            .get_unchecked_node_degree_from_node_id(node_id.into())
            .into()
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
        self.inner
            .get_unchecked_weighted_node_degree_from_node_id(node_id.into())
            .into()
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
        Ok(pe!(self.inner.get_node_degree_from_node_id(node_id.into()))?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_id)"]
    /// Returns the comulative node degree up to the given node.
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
    pub unsafe fn get_unchecked_comulative_node_degree_from_node_id(
        &self,
        node_id: NodeT,
    ) -> EdgeT {
        self.inner
            .get_unchecked_comulative_node_degree_from_node_id(node_id.into())
            .into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_id)"]
    /// Returns the comulative node degree up to the given node.
    ///
    /// Parameters
    /// ----------
    /// node_id: int,
    ///     Integer ID of the node.
    ///
    pub fn get_comulative_node_degree_from_node_id(&self, node_id: NodeT) -> PyResult<EdgeT> {
        Ok(pe!(self
            .inner
            .get_comulative_node_degree_from_node_id(node_id.into()))?
        .into())
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
        Ok(pe!(self
            .inner
            .get_weighted_node_degree_from_node_id(node_id.into()))?
        .into())
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
        Ok(pe!(self.inner.get_node_degree_from_node_name(node_name.into()))?.into())
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
        Ok(pe!(self.inner.get_top_k_central_node_names(k.into()))?.into())
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
        self.inner
            .get_unchecked_node_type_id_from_node_id(node_id.into())
            .into()
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
    pub fn get_node_type_ids_from_node_id(
        &self,
        node_id: NodeT,
    ) -> PyResult<Option<Vec<NodeTypeT>>> {
        Ok(pe!(self.inner.get_node_type_ids_from_node_id(node_id.into()))?.into())
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
        self.inner
            .get_unchecked_edge_type_id_from_edge_id(edge_id.into())
            .into()
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
        Ok(pe!(self.inner.get_edge_type_id_from_edge_id(edge_id.into()))?.into())
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
        self.inner
            .get_unchecked_node_type_names_from_node_id(node_id.into())
            .into()
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
        Ok(pe!(self.inner.get_node_type_names_from_node_id(node_id.into()))?.into())
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
        Ok(pe!(self
            .inner
            .get_node_type_names_from_node_name(node_name.into()))?
        .into())
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
        Ok(pe!(self.inner.get_edge_type_name_from_edge_id(edge_id.into()))?.into())
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
        Ok(pe!(self
            .inner
            .get_edge_type_name_from_edge_type_id(edge_type_id.into()))?
        .into())
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
        Ok(pe!(self.inner.get_edge_weight_from_edge_id(edge_id.into()))?.into())
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
        Ok(pe!(self
            .inner
            .get_edge_weight_from_node_ids(src.into(), dst.into()))?
        .into())
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
        Ok(
            pe!(self.inner.get_edge_weight_from_node_ids_and_edge_type_id(
                src.into(),
                dst.into(),
                edge_type.into()
            ))?
            .into(),
        )
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
        Ok(pe!(self
            .inner
            .get_edge_weight_from_node_names_and_edge_type_name(
                src.into(),
                dst.into(),
                edge_type.into()
            ))?
        .into())
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
        Ok(pe!(self
            .inner
            .get_edge_weight_from_node_names(src_name.into(), dst_name.into()))?
        .into())
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
        self.inner
            .get_unchecked_node_name_from_node_id(node_id.into())
            .into()
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
        Ok(pe!(self.inner.get_node_name_from_node_id(node_id.into()))?.into())
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
        Ok(pe!(self.inner.get_node_id_from_node_name(node_name.into()))?.into())
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
            pe!(self.inner.get_node_ids_from_node_names(node_names.into()))?,
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
        Ok(pe!(self
            .inner
            .get_edge_node_ids_from_edge_node_names(edge_node_names.into()))?
        .into())
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
        Ok(pe!(self
            .inner
            .get_edge_node_names_from_edge_node_ids(edge_node_ids.into()))?
        .into())
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
    pub fn get_node_type_ids_from_node_name(
        &self,
        node_name: &str,
    ) -> PyResult<Option<Vec<NodeTypeT>>> {
        Ok(pe!(self
            .inner
            .get_node_type_ids_from_node_name(node_name.into()))?
        .into())
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
        Ok(pe!(self
            .inner
            .get_node_type_name_from_node_name(node_name.into()))?
        .into())
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
        Ok(pe!(self
            .inner
            .get_edge_count_from_edge_type_id(edge_type_id.into()))?
        .into())
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
        Ok(pe!(self
            .inner
            .get_edge_type_id_from_edge_type_name(edge_type_name.into()))?
        .into())
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
        Ok(pe!(self
            .inner
            .get_edge_count_from_edge_type_name(edge_type_name.into()))?
        .into())
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
        Ok(pe!(self
            .inner
            .get_node_type_id_from_node_type_name(node_type_name.into()))?
        .into())
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
        Ok(pe!(self
            .inner
            .get_node_count_from_node_type_id(node_type_id.into()))?
        .into())
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
        Ok(pe!(self
            .inner
            .get_node_count_from_node_type_name(node_type_name.into()))?
        .into())
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
            pe!(self
                .inner
                .get_neighbour_node_ids_from_node_id(node_id.into()))?,
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
            pe!(self
                .inner
                .get_neighbour_node_ids_from_node_name(node_name.into()))?,
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
        Ok(pe!(self
            .inner
            .get_neighbour_node_names_from_node_name(node_name.into()))?
        .into())
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
        Ok(pe!(self
            .inner
            .get_minmax_edge_ids_from_node_ids(src.into(), dst.into()))?
        .into())
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
        Ok(pe!(self.inner.get_edge_id_from_node_ids_and_edge_type_id(
            src.into(),
            dst.into(),
            edge_type.into()
        ))?
        .into())
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
        Ok(pe!(self
            .inner
            .get_edge_id_from_node_names(src_name.into(), dst_name.into()))?
        .into())
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
        Ok(
            pe!(self.inner.get_edge_id_from_node_names_and_edge_type_name(
                src_name.into(),
                dst_name.into(),
                edge_type_name.into()
            ))?
            .into(),
        )
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
        Ok(pe!(self
            .inner
            .get_edge_type_ids_from_edge_type_names(edge_type_names.into()))?
        .into())
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
        Ok(pe!(self
            .inner
            .get_node_type_ids_from_node_type_names(node_type_names.into()))?
        .into())
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
        Ok(pe!(self
            .inner
            .get_multiple_node_type_ids_from_node_type_names(node_type_names.into()))?
        .into())
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
        self.inner
            .get_unchecked_minmax_edge_ids_from_source_node_id(src.into())
            .into()
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
        Ok(pe!(self
            .inner
            .get_minmax_edge_ids_from_source_node_id(src.into()))?
        .into())
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
        Ok(pe!(self
            .inner
            .get_node_type_name_from_node_type_id(node_type_id.into()))?
        .into())
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
        self.inner
            .get_unchecked_node_type_names_from_node_type_ids(node_type_ids.into())
            .into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns vector of unweighted degree centrality for all nodes
    pub fn get_degree_centrality(&self) -> PyResult<Py<PyArray1<f64>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self.inner.get_degree_centrality())?,
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
            pe!(self.inner.get_weighted_degree_centrality())?,
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
        self.inner
            .get_unchecked_closeness_centrality_from_node_id(node_id.into())
            .into()
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
        self.inner
            .get_unchecked_weighted_closeness_centrality_from_node_id(
                node_id.into(),
                use_edge_weights_as_probabilities.into(),
            )
            .into()
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
        to_ndarray_1d!(
            gil,
            self.inner.get_closeness_centrality(verbose.into()),
            f64
        )
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
            pe!(self.inner.get_weighted_closeness_centrality(
                use_edge_weights_as_probabilities.into(),
                verbose.into()
            ))?,
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
        self.inner
            .get_unchecked_harmonic_centrality_from_node_id(node_id.into())
            .into()
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
        self.inner
            .get_unchecked_weighted_harmonic_centrality_from_node_id(
                node_id.into(),
                use_edge_weights_as_probabilities.into(),
            )
            .into()
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
        to_ndarray_1d!(gil, self.inner.get_harmonic_centrality(verbose.into()), f64)
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
            pe!(self.inner.get_weighted_harmonic_centrality(
                use_edge_weights_as_probabilities.into(),
                verbose.into()
            ))?,
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
            self.inner
                .get_stress_centrality(normalize.into(), verbose.into()),
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
            self.inner
                .get_betweenness_centrality(normalize.into(), verbose.into()),
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
                .inner
                .get_eigenvector_centrality(maximum_iterations_number.into(), tollerance.into()))?,
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
            pe!(self.inner.get_weighted_eigenvector_centrality(
                maximum_iterations_number.into(),
                tollerance.into()
            ))?,
            f64
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns 2-approximated verted cover set using greedy algorithm.
    pub fn approximated_vertex_cover_set(&self) -> HashSet<NodeT> {
        self.inner.approximated_vertex_cover_set().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns graph with node IDs sorted by increasing outbound node degree
    pub fn sort_by_increasing_outbound_node_degree(&self) -> Graph {
        self.inner.sort_by_increasing_outbound_node_degree().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns graph with node IDs sorted by decreasing outbound node degree
    pub fn sort_by_decreasing_outbound_node_degree(&self) -> Graph {
        self.inner.sort_by_decreasing_outbound_node_degree().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns graph with node IDs sorted by lexicographic order
    pub fn sort_by_node_lexicographic_order(&self) -> Graph {
        self.inner.sort_by_node_lexicographic_order().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, root_node_id)"]
    /// Returns topological sorting map using breadth-first search from the given node.
    ///
    /// Parameters
    /// ----------
    /// root_node_id: int,
    ///     Node ID of node to be used as root of BFS
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given root node ID does not exist in the graph
    ///
    pub fn get_bfs_topological_sorting_from_node_id(
        &self,
        root_node_id: NodeT,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self
                .inner
                .get_bfs_topological_sorting_from_node_id(root_node_id.into()))?,
            NodeT
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, root_node_id)"]
    /// Returns topological sorting reversed map using breadth-first search from the given node.
    ///
    /// Parameters
    /// ----------
    /// root_node_id: int,
    ///     Node ID of node to be used as root of BFS
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given root node ID does not exist in the graph
    ///
    pub fn get_reversed_bfs_topological_sorting_from_node_id(
        &self,
        root_node_id: NodeT,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self
                .inner
                .get_reversed_bfs_topological_sorting_from_node_id(root_node_id.into()))?,
            NodeT
        ))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, root_node_id)"]
    /// Returns graph with node IDs sorted using a BFS
    ///
    /// Parameters
    /// ----------
    /// root_node_id: int,
    ///     Node ID of node to be used as root of BFS
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given root node ID does not exist in the graph
    ///
    pub fn sort_by_bfs_topological_sorting_from_node_id(
        &self,
        root_node_id: NodeT,
    ) -> PyResult<Graph> {
        Ok(pe!(self
            .inner
            .sort_by_bfs_topological_sorting_from_node_id(root_node_id.into()))?
        .into())
    }

    #[staticmethod]
    #[automatically_generated_binding]
    #[text_signature = "(random_state, minimum_node_id, minimum_node_sampling, maximum_node_sampling, nodes_number, include_selfloops, node_type, edge_type, weight, directed, name)"]
    /// Creates new random connected graph with given sizes and types.
    ///
    /// Parameters
    /// ----------
    /// random_state: int,
    ///     The random state to use to reproduce the sampling.
    /// minimum_node_id: int,
    ///     The minimum node ID for the connected graph.
    /// minimum_node_sampling: int,
    ///     The minimum amount of nodes to sample per node.
    /// maximum_node_sampling: int,
    ///     The maximum amount of nodes to sample per node.
    /// nodes_number: Optional[int],
    ///     Number of nodes in the chain. By default 10.
    /// include_selfloops: Optional[bool],
    ///     Whether to include selfloops.
    /// node_type: Optional[str],
    ///     The node type to use for the chain. By default 'chain'.
    /// edge_type: Optional[str],
    ///     The node type to use for the chain. By default 'chain'.
    /// weight: Optional[float],
    ///     The weight to use for the edges in the chain. By default None.
    /// directed: Optional[bool],
    ///     Whether the graph is to built as directed. By default false.
    /// name: Optional[str],
    ///     Name of the graph. By default 'Chain'.
    ///
    pub fn generate_random_connected_graph(
        random_state: Option<u64>,
        minimum_node_id: Option<NodeT>,
        minimum_node_sampling: Option<NodeT>,
        maximum_node_sampling: Option<NodeT>,
        nodes_number: Option<NodeT>,
        include_selfloops: Option<bool>,
        node_type: Option<&str>,
        edge_type: Option<&str>,
        weight: Option<WeightT>,
        directed: Option<bool>,
        name: Option<&str>,
    ) -> PyResult<Graph> {
        Ok(pe!(graph::Graph::generate_random_connected_graph(
            random_state.into(),
            minimum_node_id.into(),
            minimum_node_sampling.into(),
            maximum_node_sampling.into(),
            nodes_number.into(),
            include_selfloops.into(),
            node_type.into(),
            edge_type.into(),
            weight.into(),
            directed.into(),
            name.into()
        ))?
        .into())
    }

    #[staticmethod]
    #[automatically_generated_binding]
    #[text_signature = "(random_state, minimum_node_id, nodes_number, include_selfloops, node_type, edge_type, weight, directed, name)"]
    /// Creates new random connected graph with given sizes and types.
    ///
    /// Parameters
    /// ----------
    /// random_state: int,
    ///     The random state to use to reproduce the sampling.
    /// minimum_node_id: int,
    ///     The minimum node ID for the connected graph.
    /// minimum_node_sampling: int,
    ///     The minimum amount of nodes to sample per node.
    /// maximum_node_sampling: int,
    ///     The maximum amount of nodes to sample per node.
    /// nodes_number: Optional[int],
    ///     Number of nodes in the chain. By default 10.
    /// include_selfloops: Optional[bool],
    ///     Whether to include selfloops.
    /// node_type: Optional[str],
    ///     The node type to use for the chain. By default 'chain'.
    /// edge_type: Optional[str],
    ///     The node type to use for the chain. By default 'chain'.
    /// weight: Optional[float],
    ///     The weight to use for the edges in the chain. By default None.
    /// directed: Optional[bool],
    ///     Whether the graph is to built as directed. By default false.
    /// name: Optional[str],
    ///     Name of the graph. By default 'Chain'.
    ///
    pub fn generate_random_spanning_tree(
        random_state: Option<u64>,
        minimum_node_id: Option<NodeT>,
        nodes_number: Option<NodeT>,
        include_selfloops: Option<bool>,
        node_type: Option<&str>,
        edge_type: Option<&str>,
        weight: Option<WeightT>,
        directed: Option<bool>,
        name: Option<&str>,
    ) -> PyResult<Graph> {
        Ok(pe!(graph::Graph::generate_random_spanning_tree(
            random_state.into(),
            minimum_node_id.into(),
            nodes_number.into(),
            include_selfloops.into(),
            node_type.into(),
            edge_type.into(),
            weight.into(),
            directed.into(),
            name.into()
        ))?
        .into())
    }

    #[staticmethod]
    #[automatically_generated_binding]
    #[text_signature = "(minimum_node_id, nodes_number, include_selfloops, node_type, edge_type, weight, directed, name)"]
    /// Creates new circle graph with given sizes and types.
    ///
    /// Parameters
    /// ----------
    /// minimum_node_id: Optional[int],
    ///     Minimum node ID to start with. May be needed when circleing graphs. By default 0.
    /// nodes_number: Optional[int],
    ///     Number of nodes in the circle. By default 10.
    /// include_selfloops: Optional[bool],
    ///     Whether to include selfloops.
    /// node_type: Optional[str],
    ///     The node type to use for the circle. By default 'circle'.
    /// edge_type: Optional[str],
    ///     The node type to use for the circle. By default 'circle'.
    /// weight: Optional[float],
    ///     The weight to use for the edges in the circle. By default None.
    /// directed: Optional[bool],
    ///     Whether the graph is to built as directed. By default false.
    /// name: Optional[str],
    ///     Name of the graph. By default 'Circle'.
    ///
    pub fn generate_circle_graph(
        minimum_node_id: Option<NodeT>,
        nodes_number: Option<NodeT>,
        include_selfloops: Option<bool>,
        node_type: Option<&str>,
        edge_type: Option<&str>,
        weight: Option<WeightT>,
        directed: Option<bool>,
        name: Option<&str>,
    ) -> PyResult<Graph> {
        Ok(pe!(graph::Graph::generate_circle_graph(
            minimum_node_id.into(),
            nodes_number.into(),
            include_selfloops.into(),
            node_type.into(),
            edge_type.into(),
            weight.into(),
            directed.into(),
            name.into()
        ))?
        .into())
    }

    #[staticmethod]
    #[automatically_generated_binding]
    #[text_signature = "(minimum_node_id, nodes_number, include_selfloops, node_type, edge_type, weight, directed, name)"]
    /// Creates new chain graph with given sizes and types.
    ///
    /// Parameters
    /// ----------
    /// minimum_node_id: Optional[int],
    ///     Minimum node ID to start with. May be needed when chaining graphs. By default 0.
    /// nodes_number: Optional[int],
    ///     Number of nodes in the chain. By default 10.
    /// include_selfloops: Optional[bool],
    ///     Whether to include selfloops.
    /// node_type: Optional[str],
    ///     The node type to use for the chain. By default 'chain'.
    /// edge_type: Optional[str],
    ///     The node type to use for the chain. By default 'chain'.
    /// weight: Optional[float],
    ///     The weight to use for the edges in the chain. By default None.
    /// directed: Optional[bool],
    ///     Whether the graph is to built as directed. By default false.
    /// name: Optional[str],
    ///     Name of the graph. By default 'Chain'.
    ///
    pub fn generate_chain_graph(
        minimum_node_id: Option<NodeT>,
        nodes_number: Option<NodeT>,
        include_selfloops: Option<bool>,
        node_type: Option<&str>,
        edge_type: Option<&str>,
        weight: Option<WeightT>,
        directed: Option<bool>,
        name: Option<&str>,
    ) -> PyResult<Graph> {
        Ok(pe!(graph::Graph::generate_chain_graph(
            minimum_node_id.into(),
            nodes_number.into(),
            include_selfloops.into(),
            node_type.into(),
            edge_type.into(),
            weight.into(),
            directed.into(),
            name.into()
        ))?
        .into())
    }

    #[staticmethod]
    #[automatically_generated_binding]
    #[text_signature = "(minimum_node_id, nodes_number, include_selfloops, node_type, edge_type, weight, directed, name)"]
    /// Creates new complete graph with given sizes and types.
    ///
    /// Parameters
    /// ----------
    /// minimum_node_id: Optional[int],
    ///     Minimum node ID to start with. May be needed when combining graphs. By default 0.
    /// nodes_number: Optional[int],
    ///     Number of nodes in the chain. By default 10.
    /// include_selfloops: Optional[bool],
    ///     Whether to include selfloops.
    /// node_type: Optional[str],
    ///     The node type to use. By default 'complete'.
    /// edge_type: Optional[str],
    ///     The node type to use. By default 'complete'.
    /// weight: Optional[float],
    ///     The weight to use for the edges. By default None.
    /// directed: Optional[bool],
    ///     Whether the graph is to built as directed. By default false.
    /// name: Optional[str],
    ///     Name of the graph. By default 'Complete'.
    ///
    pub fn generate_complete_graph(
        minimum_node_id: Option<NodeT>,
        nodes_number: Option<NodeT>,
        include_selfloops: Option<bool>,
        node_type: Option<&str>,
        edge_type: Option<&str>,
        weight: Option<WeightT>,
        directed: Option<bool>,
        name: Option<&str>,
    ) -> PyResult<Graph> {
        Ok(pe!(graph::Graph::generate_complete_graph(
            minimum_node_id.into(),
            nodes_number.into(),
            include_selfloops.into(),
            node_type.into(),
            edge_type.into(),
            weight.into(),
            directed.into(),
            name.into()
        ))?
        .into())
    }

    #[staticmethod]
    #[automatically_generated_binding]
    #[text_signature = "(minimum_node_id, left_clique_nodes_number, right_clique_nodes_number, chain_nodes_number, include_selfloops, left_clique_node_type, right_clique_node_type, chain_node_type, left_clique_edge_type, right_clique_edge_type, chain_edge_type, left_clique_weight, right_clique_weight, chain_weight, directed, name)"]
    /// Creates new barbell graph with given sizes and types.
    ///
    /// Parameters
    /// ----------
    /// minimum_node_id: Optional[int],
    ///     Minimum node ID to start with. May be needed when chaining graphs. By default 0.
    /// left_clique_nodes_number: Optional[int],
    ///     Number of nodes in the left clique. By default 10.
    /// right_clique_nodes_number: Optional[int],
    ///      Number of nodes in the right clique. By default equal to the left clique.
    /// chain_nodes_number: Optional[int],
    ///     Number of nodes in the chain. By default 10.
    /// include_selfloops: Optional[bool],
    ///     Whether to include selfloops.
    /// left_clique_node_type: Optional[str],
    ///     The node type to use for the left clique. By default 'left_clique'.
    /// right_clique_node_type: Optional[str],
    ///     The node type to use for the right clique. By default 'right_clique'.
    /// chain_node_type: Optional[str],
    ///     The node type to use for the chain. By default 'chain'.
    /// left_clique_edge_type: Optional[str],
    ///     The node type to use for the left clique. By default 'left_clique'.
    /// right_clique_edge_type: Optional[str],
    ///     The node type to use for the right clique. By default 'right_clique'.
    /// chain_edge_type: Optional[str],
    ///     The node type to use for the chain. By default 'chain'.
    /// left_clique_weight: Optional[float],
    ///     The weight to use for the edges in the left clique. By default None.
    /// right_clique_weight: Optional[float],
    ///     The weight to use for the edges in the right clique. By default None.
    /// chain_weight: Optional[float],
    ///     The weight to use for the edges in the chain. By default None.
    /// directed: Optional[bool],
    ///     Whether the graph is to built as directed. By default false.
    /// name: Optional[str],
    ///     Name of the graph. By default 'Barbell'.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the edge weights are provided only for a subset.
    ///
    pub fn generate_barbell_graph(
        minimum_node_id: Option<NodeT>,
        left_clique_nodes_number: Option<NodeT>,
        right_clique_nodes_number: Option<NodeT>,
        chain_nodes_number: Option<NodeT>,
        include_selfloops: Option<bool>,
        left_clique_node_type: Option<&str>,
        right_clique_node_type: Option<&str>,
        chain_node_type: Option<&str>,
        left_clique_edge_type: Option<&str>,
        right_clique_edge_type: Option<&str>,
        chain_edge_type: Option<&str>,
        left_clique_weight: Option<WeightT>,
        right_clique_weight: Option<WeightT>,
        chain_weight: Option<WeightT>,
        directed: Option<bool>,
        name: Option<&str>,
    ) -> PyResult<Graph> {
        Ok(pe!(graph::Graph::generate_barbell_graph(
            minimum_node_id.into(),
            left_clique_nodes_number.into(),
            right_clique_nodes_number.into(),
            chain_nodes_number.into(),
            include_selfloops.into(),
            left_clique_node_type.into(),
            right_clique_node_type.into(),
            chain_node_type.into(),
            left_clique_edge_type.into(),
            right_clique_edge_type.into(),
            chain_edge_type.into(),
            left_clique_weight.into(),
            right_clique_weight.into(),
            chain_weight.into(),
            directed.into(),
            name.into()
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src_node_id, dst_node_id, compute_predecessors, maximal_depth)"]
    /// Returns vector of minimum paths distances and vector of nodes predecessors, if requested.
    ///
    /// Parameters
    /// ----------
    /// src_node_id: int,
    ///     Root of the tree of minimum paths.
    /// maybe_dst_node_id: Optional[int],
    ///     Optional target destination. If provided, Dijkstra will stop upon reaching this node.
    /// maybe_dst_node_ids: Optional[List[int]],
    ///     Optional target destinations. If provided, Dijkstra will stop upon reaching all of these nodes.
    /// compute_distances: Optional[bool],
    ///     Whether to compute the vector of distances.
    /// compute_predecessors: Optional[bool],
    ///     Whether to compute the vector of predecessors.
    /// compute_visited: Optional[bool],
    ///     Whether to compute the vector of visited nodes.
    /// maximal_depth: Optional[int],
    ///     The maximal depth to execute the DFS for.
    ///
    ///
    /// Safety
    /// ------
    /// If any of the given node IDs does not exist in the graph the method will panic.
    ///
    ///  TODO! Explore chains accelerations!
    pub unsafe fn get_unchecked_breadth_first_search_from_node_ids(
        &self,
        src_node_id: NodeT,
        dst_node_id: Option<NodeT>,
        compute_predecessors: Option<bool>,
        maximal_depth: Option<NodeT>,
    ) -> ShortestPathsResultBFS {
        self.inner
            .get_unchecked_breadth_first_search_from_node_ids(
                src_node_id.into(),
                dst_node_id.into(),
                compute_predecessors.into(),
                maximal_depth.into(),
            )
            .into()
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
    pub unsafe fn get_unchecked_shortest_path_node_ids_from_node_ids(
        &self,
        src_node_id: NodeT,
        dst_node_id: NodeT,
        maximal_depth: Option<NodeT>,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self
                .inner
                .get_unchecked_shortest_path_node_ids_from_node_ids(
                    src_node_id.into(),
                    dst_node_id.into(),
                    maximal_depth.into()
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
    pub unsafe fn get_unchecked_shortest_path_node_names_from_node_ids(
        &self,
        src_node_id: NodeT,
        dst_node_id: NodeT,
        maximal_depth: Option<NodeT>,
    ) -> PyResult<Vec<String>> {
        Ok(pe!(self
            .inner
            .get_unchecked_shortest_path_node_names_from_node_ids(
                src_node_id.into(),
                dst_node_id.into(),
                maximal_depth.into()
            ))?
        .into())
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
    pub fn get_shortest_path_node_ids_from_node_ids(
        &self,
        src_node_id: NodeT,
        dst_node_id: NodeT,
        maximal_depth: Option<NodeT>,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self.inner.get_shortest_path_node_ids_from_node_ids(
                src_node_id.into(),
                dst_node_id.into(),
                maximal_depth.into()
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
    pub fn get_shortest_path_node_ids_from_node_names(
        &self,
        src_node_name: &str,
        dst_node_name: &str,
        maximal_depth: Option<NodeT>,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(
            gil,
            pe!(self.inner.get_shortest_path_node_ids_from_node_names(
                src_node_name.into(),
                dst_node_name.into(),
                maximal_depth.into()
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
    pub fn get_shortest_path_node_names_from_node_names(
        &self,
        src_node_name: &str,
        dst_node_name: &str,
        maximal_depth: Option<NodeT>,
    ) -> PyResult<Vec<String>> {
        Ok(pe!(self.inner.get_shortest_path_node_names_from_node_names(
            src_node_name.into(),
            dst_node_name.into(),
            maximal_depth.into()
        ))?
        .into())
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
        self.inner
            .get_unchecked_k_shortest_path_node_ids_from_node_ids(
                src_node_id.into(),
                dst_node_id.into(),
                k.into(),
            )
            .into()
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
        Ok(pe!(self.inner.get_k_shortest_path_node_ids_from_node_ids(
            src_node_id.into(),
            dst_node_id.into(),
            k.into()
        ))?
        .into())
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
        Ok(pe!(self.inner.get_k_shortest_path_node_ids_from_node_names(
            src_node_name.into(),
            dst_node_name.into(),
            k.into()
        ))?
        .into())
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
        Ok(
            pe!(self.inner.get_k_shortest_path_node_names_from_node_names(
                src_node_name.into(),
                dst_node_name.into(),
                k.into()
            ))?
            .into(),
        )
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
        self.inner
            .get_unchecked_eccentricity_from_node_id(node_id.into())
            .into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_id, use_edge_weights_as_probabilities)"]
    /// Returns weighted eccentricity of the given node.
    ///
    /// This method will panic if the given node ID does not exists in the graph.
    ///
    /// Parameters
    /// ----------
    /// node_id: int,
    ///     Node for which to compute the eccentricity.
    /// use_edge_weights_as_probabilities: Optional[bool],
    ///     Whether to treat the edge weights as probabilities.
    ///
    ///
    /// Safety
    /// ------
    /// If any of the given node IDs does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_weighted_eccentricity_from_node_id(
        &self,
        node_id: NodeT,
        use_edge_weights_as_probabilities: Option<bool>,
    ) -> f64 {
        self.inner
            .get_unchecked_weighted_eccentricity_from_node_id(
                node_id.into(),
                use_edge_weights_as_probabilities.into(),
            )
            .into()
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
        Ok(pe!(self.inner.get_eccentricity_from_node_id(node_id.into()))?.into())
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
        Ok(pe!(self.inner.get_weighted_eccentricity_from_node_id(
            node_id.into(),
            use_edge_weights_as_probabilities.into()
        ))?
        .into())
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
        Ok(pe!(self.inner.get_eccentricity_from_node_name(node_name.into()))?.into())
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
        Ok(pe!(self.inner.get_weighted_eccentricity_from_node_name(
            node_name.into(),
            use_edge_weights_as_probabilities.into()
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src_node_id, maybe_dst_node_id, maybe_dst_node_ids, compute_predecessors, maximal_depth, use_edge_weights_as_probabilities)"]
    /// Returns vector of minimum paths distances and vector of nodes predecessors, if requested.
    ///
    /// Parameters
    /// ----------
    /// src_node_id: int,
    ///     Root of the tree of minimum paths.
    /// maybe_dst_node_id: Optional[int],
    ///     Optional target destination. If provided, Dijkstra will stop upon reaching this node.
    /// maybe_dst_node_ids: Optional[List[int]],
    ///     Optional target destinations. If provided, Dijkstra will stop upon reaching all of these nodes.
    /// compute_predecessors: bool,
    ///     Whether to compute the vector of predecessors.
    /// maximal_depth: Optional[int],
    ///     The maximal number of iterations to execute Dijkstra for.
    /// use_edge_weights_as_probabilities: Optional[bool],
    ///     Whether to treat the edge weights as probabilities.
    ///
    ///
    /// Safety
    /// ------
    /// If any of the given node IDs does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_dijkstra_from_node_ids(
        &self,
        src_node_id: NodeT,
        maybe_dst_node_id: Option<NodeT>,
        maybe_dst_node_ids: Option<Vec<NodeT>>,
        compute_predecessors: Option<bool>,
        maximal_depth: Option<NodeT>,
        use_edge_weights_as_probabilities: Option<bool>,
    ) -> ShortestPathsDjkstra {
        self.inner
            .get_unchecked_dijkstra_from_node_ids(
                src_node_id.into(),
                maybe_dst_node_id.into(),
                maybe_dst_node_ids.into(),
                compute_predecessors.into(),
                maximal_depth.into(),
                use_edge_weights_as_probabilities.into(),
            )
            .into()
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
    /// use_edge_weights_as_probabilities: Optional[bool],
    ///     Whether to treat the edge weights as probabilities.
    /// maximal_depth: Optional[int],
    ///     The maximal number of iterations to execute Dijkstra for.
    ///
    ///
    /// Safety
    /// ------
    /// If any of the given node IDs does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_weighted_shortest_path_node_ids_from_node_ids(
        &self,
        src_node_id: NodeT,
        dst_node_id: NodeT,
        use_edge_weights_as_probabilities: Option<bool>,
        maximal_depth: Option<NodeT>,
    ) -> (f64, Vec<NodeT>) {
        self.inner
            .get_unchecked_weighted_shortest_path_node_ids_from_node_ids(
                src_node_id.into(),
                dst_node_id.into(),
                use_edge_weights_as_probabilities.into(),
                maximal_depth.into(),
            )
            .into()
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
    /// use_edge_weights_as_probabilities: Optional[bool],
    ///     Whether to treat the edge weights as probabilities.
    /// maximal_depth: Optional[int],
    ///     The maximal number of iterations to execute Dijkstra for.
    ///
    ///
    /// Safety
    /// ------
    /// If any of the given node IDs does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_weighted_shortest_path_node_names_from_node_ids(
        &self,
        src_node_id: NodeT,
        dst_node_id: NodeT,
        use_edge_weights_as_probabilities: Option<bool>,
        maximal_depth: Option<NodeT>,
    ) -> (f64, Vec<String>) {
        self.inner
            .get_unchecked_weighted_shortest_path_node_names_from_node_ids(
                src_node_id.into(),
                dst_node_id.into(),
                use_edge_weights_as_probabilities.into(),
                maximal_depth.into(),
            )
            .into()
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
    pub fn get_weighted_shortest_path_node_ids_from_node_ids(
        &self,
        src_node_id: NodeT,
        dst_node_id: NodeT,
        use_edge_weights_as_probabilities: Option<bool>,
        maximal_depth: Option<NodeT>,
    ) -> PyResult<(f64, Vec<NodeT>)> {
        Ok(pe!(self
            .inner
            .get_weighted_shortest_path_node_ids_from_node_ids(
                src_node_id.into(),
                dst_node_id.into(),
                use_edge_weights_as_probabilities.into(),
                maximal_depth.into()
            ))?
        .into())
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
    pub fn get_weighted_shortest_path_node_ids_from_node_names(
        &self,
        src_node_name: &str,
        dst_node_name: &str,
        use_edge_weights_as_probabilities: Option<bool>,
        maximal_depth: Option<NodeT>,
    ) -> PyResult<(f64, Vec<NodeT>)> {
        Ok(pe!(self
            .inner
            .get_weighted_shortest_path_node_ids_from_node_names(
                src_node_name.into(),
                dst_node_name.into(),
                use_edge_weights_as_probabilities.into(),
                maximal_depth.into()
            ))?
        .into())
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
    pub fn get_weighted_shortest_path_node_names_from_node_names(
        &self,
        src_node_name: &str,
        dst_node_name: &str,
        use_edge_weights_as_probabilities: Option<bool>,
        maximal_depth: Option<NodeT>,
    ) -> PyResult<(f64, Vec<String>)> {
        Ok(pe!(self
            .inner
            .get_weighted_shortest_path_node_names_from_node_names(
                src_node_name.into(),
                dst_node_name.into(),
                use_edge_weights_as_probabilities.into(),
                maximal_depth.into()
            ))?
        .into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src_node_id, dst_node_id, compute_predecessors, maximal_depth)"]
    /// Returns vector of minimum paths distances and vector of nodes predecessors from given source node ID and optional destination node ID.
    ///
    /// Parameters
    /// ----------
    /// src_node_id: int,
    ///     Node ID root of the tree of minimum paths.
    /// compute_predecessors: Optional[bool],
    ///     Whether to compute the vector of predecessors.
    /// maximal_depth: Optional[int],
    ///     The maximal number of iterations to execute the DFS for.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given source node ID does not exist in the current graph.
    /// ValueError
    ///     If the given optional destination node ID does not exist in the current graph.
    ///
    pub fn get_breadth_first_search_from_node_ids(
        &self,
        src_node_id: NodeT,
        dst_node_id: Option<NodeT>,
        compute_predecessors: Option<bool>,
        maximal_depth: Option<NodeT>,
    ) -> PyResult<ShortestPathsResultBFS> {
        Ok(pe!(self.inner.get_breadth_first_search_from_node_ids(
            src_node_id.into(),
            dst_node_id.into(),
            compute_predecessors.into(),
            maximal_depth.into()
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src_node_id, maybe_dst_node_id, maybe_dst_node_ids, compute_predecessors, maximal_depth, use_edge_weights_as_probabilities)"]
    /// Returns vector of minimum paths distances and vector of nodes predecessors from given source node ID and optional destination node ID.
    ///
    /// Parameters
    /// ----------
    /// src_node_id: int,
    ///     Node ID root of the tree of minimum paths.
    /// maybe_dst_node_id: Optional[int],
    ///     Optional target destination. If provided, Dijkstra will stop upon reaching this node.
    /// maybe_dst_node_ids: Optional[List[int]],
    ///     Optional target destinations. If provided, Dijkstra will stop upon reaching all of these nodes.
    /// compute_predecessors: Optional[bool],
    ///     Whether to compute the vector of predecessors.
    /// maximal_depth: Optional[int],
    ///     The maximal depth to execute the DFS for.
    /// use_edge_weights_as_probabilities: Optional[bool],
    ///     Whether to treat the edge weights as probabilities.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the weights are to be used and the graph does not have weights.
    /// ValueError
    ///     If the given source node ID does not exist in the current graph.
    /// ValueError
    ///     If the given optional destination node ID does not exist in the current graph.
    /// ValueError
    ///     If weights are requested to be treated as probabilities but are not between 0 and 1.
    /// ValueError
    ///     If the graph contains negative weights.
    ///
    pub fn get_dijkstra_from_node_ids(
        &self,
        src_node_id: NodeT,
        maybe_dst_node_id: Option<NodeT>,
        maybe_dst_node_ids: Option<Vec<NodeT>>,
        compute_predecessors: Option<bool>,
        maximal_depth: Option<NodeT>,
        use_edge_weights_as_probabilities: Option<bool>,
    ) -> PyResult<ShortestPathsDjkstra> {
        Ok(pe!(self.inner.get_dijkstra_from_node_ids(
            src_node_id.into(),
            maybe_dst_node_id.into(),
            maybe_dst_node_ids.into(),
            compute_predecessors.into(),
            maximal_depth.into(),
            use_edge_weights_as_probabilities.into()
        ))?
        .into())
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
        Ok(pe!(self
            .inner
            .get_diameter_naive(ignore_infinity.into(), verbose.into()))?
        .into())
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
        Ok(pe!(self
            .inner
            .get_diameter(ignore_infinity.into(), verbose.into()))?
        .into())
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
        Ok(pe!(self.inner.get_weighted_diameter_naive(
            ignore_infinity.into(),
            use_edge_weights_as_probabilities.into(),
            verbose.into()
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src_node_name, dst_node_name, compute_predecessors, maximal_depth)"]
    /// Returns vector of minimum paths distances and vector of nodes predecessors from given source node name and optional destination node name.
    ///
    /// Parameters
    /// ----------
    /// src_node_name: str,
    ///     Node name root of the tree of minimum paths.
    /// dst_node_name: Optional[str],
    ///     Destination node name.
    /// compute_predecessors: Optional[bool],
    ///     Whether to compute the vector of predecessors.
    /// maximal_depth: Optional[int],
    ///     The maximal depth to execute the DFS for.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the weights are to be used and the graph does not have weights.
    /// ValueError
    ///     If the given source node name does not exist in the current graph.
    /// ValueError
    ///     If the given optional destination node name does not exist in the current graph.
    ///
    pub fn get_breadth_first_search_from_node_names(
        &self,
        src_node_name: &str,
        dst_node_name: Option<&str>,
        compute_predecessors: Option<bool>,
        maximal_depth: Option<NodeT>,
    ) -> PyResult<ShortestPathsResultBFS> {
        Ok(pe!(self.inner.get_breadth_first_search_from_node_names(
            src_node_name.into(),
            dst_node_name.into(),
            compute_predecessors.into(),
            maximal_depth.into()
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src_node_name, maybe_dst_node_name, maybe_dst_node_names, compute_predecessors, maximal_depth, use_edge_weights_as_probabilities)"]
    /// Returns vector of minimum paths distances and vector of nodes predecessors from given source node name and optional destination node name.
    ///
    /// Parameters
    /// ----------
    /// src_node_name: str,
    ///     Node name root of the tree of minimum paths.
    /// maybe_dst_node_name: Optional[str],
    ///     Optional target destination node name. If provided, Dijkstra will stop upon reaching this node.
    /// maybe_dst_node_names: Optional[List[str]],
    ///     Optional target destination node names. If provided, Dijkstra will stop upon reaching all of these nodes.
    /// compute_predecessors: Optional[bool],
    ///     Whether to compute the vector of predecessors.
    /// maximal_depth: Optional[int],
    ///     The maximal depth to execute the DFS for.
    /// use_edge_weights_as_probabilities: Optional[bool],
    ///     Whether to treat the edge weights as probabilities.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the weights are to be used and the graph does not have weights.
    /// ValueError
    ///     If the given source node name does not exist in the current graph.
    /// ValueError
    ///     If the given optional destination node name does not exist in the current graph.
    ///
    pub fn get_dijkstra_from_node_names(
        &self,
        src_node_name: &str,
        maybe_dst_node_name: Option<&str>,
        maybe_dst_node_names: Option<Vec<&str>>,
        compute_predecessors: Option<bool>,
        maximal_depth: Option<NodeT>,
        use_edge_weights_as_probabilities: Option<bool>,
    ) -> PyResult<ShortestPathsDjkstra> {
        Ok(pe!(self.inner.get_dijkstra_from_node_names(
            src_node_name.into(),
            maybe_dst_node_name.into(),
            maybe_dst_node_names.into(),
            compute_predecessors.into(),
            maximal_depth.into(),
            use_edge_weights_as_probabilities.into()
        ))?
        .into())
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
            pe!(self.inner.get_bipartite_edges(
                removed_existing_edges.into(),
                first_nodes_set.into(),
                second_nodes_set.into(),
                first_node_types_set.into(),
                second_node_types_set.into()
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
        Ok(pe!(self.inner.get_bipartite_edge_names(
            removed_existing_edges.into(),
            first_nodes_set.into(),
            second_nodes_set.into(),
            first_node_types_set.into(),
            second_node_types_set.into()
        ))?
        .into())
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
            pe!(self.inner.get_star_edges(
                central_node.into(),
                removed_existing_edges.into(),
                star_points_nodes_set.into(),
                star_points_node_types_set.into()
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
        Ok(pe!(self.inner.get_star_edge_names(
            central_node.into(),
            removed_existing_edges.into(),
            star_points_nodes_set.into(),
            star_points_node_types_set.into()
        ))?
        .into())
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
            self.inner.get_clique_edges(
                directed.into(),
                allow_selfloops.into(),
                removed_existing_edges.into(),
                allow_node_type_set.into(),
                allow_node_set.into()
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
        self.inner
            .get_clique_edge_names(
                directed.into(),
                allow_selfloops.into(),
                removed_existing_edges.into(),
                allow_node_type_set.into(),
                allow_node_set.into(),
            )
            .into()
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
            pe!(self.inner.get_okapi_bm25_node_feature_propagation(
                features.into(),
                iterations.into(),
                maximal_distance.into(),
                k1.into(),
                b.into(),
                include_central_node.into(),
                verbose.into()
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
            pe!(self.inner.get_okapi_bm25_node_label_propagation(
                iterations.into(),
                maximal_distance.into(),
                k1.into(),
                b.into(),
                verbose.into()
            ))?,
            f64
        ))
    }

    #[staticmethod]
    #[automatically_generated_binding]
    #[text_signature = "(node_type_path, node_type_list_separator, node_types_column_number, node_types_column, node_types_ids_column_number, node_types_ids_column, node_types_number, numeric_node_type_ids, minimum_node_type_id, node_type_list_header, node_type_list_rows_to_skip, node_type_list_is_correct, node_type_list_max_rows_number, node_type_list_comment_symbol, load_node_type_list_in_parallel, node_path, node_list_separator, node_list_header, node_list_rows_to_skip, node_list_is_correct, node_list_max_rows_number, node_list_comment_symbol, default_node_type, nodes_column_number, nodes_column, node_types_separator, node_list_node_types_column_number, node_list_node_types_column, node_ids_column, node_ids_column_number, nodes_number, minimum_node_id, numeric_node_ids, node_list_numeric_node_type_ids, skip_node_types_if_unavailable, load_node_list_in_parallel, edge_type_path, edge_types_column_number, edge_types_column, edge_types_ids_column_number, edge_types_ids_column, edge_types_number, numeric_edge_type_ids, minimum_edge_type_id, edge_type_list_separator, edge_type_list_header, edge_type_list_rows_to_skip, edge_type_list_is_correct, edge_type_list_max_rows_number, edge_type_list_comment_symbol, load_edge_type_list_in_parallel, edge_path, edge_list_separator, edge_list_header, edge_list_rows_to_skip, sources_column_number, sources_column, destinations_column_number, destinations_column, edge_list_edge_types_column_number, edge_list_edge_types_column, default_edge_type, weights_column_number, weights_column, default_weight, edge_ids_column, edge_ids_column_number, edge_list_numeric_edge_type_ids, edge_list_numeric_node_ids, skip_weights_if_unavailable, skip_edge_types_if_unavailable, edge_list_is_complete, edge_list_may_contain_duplicates, edge_list_is_sorted, edge_list_is_correct, edge_list_max_rows_number, edge_list_comment_symbol, edges_number, load_edge_list_in_parallel, verbose, may_have_singletons, may_have_singleton_with_selfloops, directed, name)"]
    /// Return graph renderized from given CSVs or TSVs-like files.
    ///
    /// Parameters
    /// ----------
    /// node_type_path: Optional[str],
    ///     The path to the file with the unique node type names.
    /// node_type_list_separator: Optional[str],
    ///     The separator to use for the node types file. Note that if this is not provided, one will be automatically detected among the following`: comma, semi-column, tab and space.
    /// node_types_column_number: Optional[int],
    ///     The number of the column of the node types file from where to load the node types.
    /// node_types_column: Optional[str],
    ///     The name of the column of the node types file from where to load the node types.
    /// node_types_number: Optional[int],
    ///     The number of the unique node types. This will be used in order to allocate the correct size for the data structure.
    /// numeric_node_type_ids: Optional[bool],
    ///     Whether the node type names should be loaded as numeric values, i.e. casted from string to a numeric representation.
    /// minimum_node_type_id: Optional[int],
    ///     The minimum node type ID to be used when using numeric node type IDs.
    /// node_type_list_header: Optional[bool],
    ///     Whether the node type file has an header.
    /// node_type_list_rows_to_skip: Optional[int],
    ///     The number of lines to skip in the node types file`: the header is already skipped if it has been specified that the file has an header.
    /// node_type_list_is_correct: Optional[bool],
    ///     Whether the node types file can be assumed to be correct, i.e. does not have something wrong in it. If this parameter is passed as true on a malformed file, the constructor will crash.
    /// node_type_list_max_rows_number: Optional[int],
    ///     The maximum number of lines to be loaded from the node types file.
    /// node_type_list_comment_symbol: Optional[str],
    ///     The comment symbol to skip lines in the node types file. Lines starting with this symbol will be skipped.
    /// load_node_type_list_in_parallel: Optional[bool],
    ///     Whether to load the node type list in parallel. Note that when loading in parallel, the internal order of the node type IDs may result changed across different iterations. We are working to get this to be stable.
    /// node_path: Optional[str],
    ///     The path to the file with the unique node names.
    /// node_list_separator: Optional[str],
    ///     The separator to use for the nodes file. Note that if this is not provided, one will be automatically detected among the following`: comma, semi-column, tab and space.
    /// node_list_header: Optional[bool],
    ///     Whether the nodes file has an header.
    /// node_list_rows_to_skip: Optional[int],
    ///     Number of rows to skip in the node list file.
    /// node_list_is_correct: Optional[bool],
    ///     Whether the nodes file can be assumed to be correct, i.e. does not have something wrong in it. If this parameter is passed as true on a malformed file, the constructor will crash.
    /// node_list_max_rows_number: Optional[int],
    ///     The maximum number of lines to be loaded from the nodes file.
    /// node_list_comment_symbol: Optional[str],
    ///     The comment symbol to skip lines in the nodes file. Lines starting with this symbol will be skipped.
    /// default_node_type: Optional[str],
    ///     The node type to be used when the node type for a given node in the node file is None.
    /// nodes_column_number: Optional[int],
    ///     The number of the column of the node file from where to load the node names.
    /// nodes_column: Optional[str],
    ///     The name of the column of the node file from where to load the node names.
    /// node_types_separator: Optional[str],
    ///     The node types separator.
    /// node_list_node_types_column_number: Optional[int],
    ///     The number of the column of the node file from where to load the node types.
    /// node_list_node_types_column: Optional[str],
    ///     The name of the column of the node file from where to load the node types.
    /// node_ids_column: Optional[str],
    ///     The name of the column of the node file from where to load the node IDs.
    /// node_ids_column_number: Optional[int],
    ///     The number of the column of the node file from where to load the node IDs
    /// nodes_number: Optional[int],
    ///     The expected number of nodes. Note that this must be the EXACT number of nodes in the graph.
    /// minimum_node_id: Optional[int],
    ///     The minimum node ID to be used, when loading the node IDs as numerical.
    /// numeric_node_ids: Optional[bool],
    ///     Whether to load the numeric node IDs as numeric.
    /// node_list_numeric_node_type_ids: Optional[bool],
    ///     Whether to load the node types IDs in the node file to be numeric.
    /// skip_node_types_if_unavailable: Optional[bool],
    ///     Whether to skip the node types without raising an error if these are unavailable.
    /// load_node_list_in_parallel: Optional[bool],
    ///     Whether to load the node list in parallel. When loading in parallel, without node IDs, the nodes may not be loaded in a deterministic order.
    /// edge_type_path: Optional[str],
    ///     The path to the file with the unique edge type names.
    /// edge_types_column_number: Optional[int],
    ///     The number of the column of the edge types file from where to load the edge types.
    /// edge_types_column: Optional[str],
    ///     The name of the column of the edge types file from where to load the edge types.
    /// edge_types_number: Optional[int],
    ///     The number of the unique edge types. This will be used in order to allocate the correct size for the data structure.
    /// numeric_edge_type_ids: Optional[bool],
    ///     Whether the edge type names should be loaded as numeric values, i.e. casted from string to a numeric representation.
    /// minimum_edge_type_id: Optional[int],
    ///     The minimum edge type ID to be used when using numeric edge type IDs.
    /// edge_type_list_separator: Optional[str],
    ///     The separator to use for the edge type list. Note that, if None is provided, one will be attempted to be detected automatically between ';', ',', tab or space.
    /// edge_type_list_header: Optional[bool],
    ///     Whether the edge type file has an header.
    /// edge_type_list_rows_to_skip: Optional[int],
    ///     Number of rows to skip in the edge type list file.
    /// edge_type_list_is_correct: Optional[bool],
    ///     Whether the edge types file can be assumed to be correct, i.e. does not have something wrong in it. If this parameter is passed as true on a malformed file, the constructor will crash.
    /// edge_type_list_max_rows_number: Optional[int],
    ///     The maximum number of lines to be loaded from the edge types file.
    /// edge_type_list_comment_symbol: Optional[str],
    ///     The comment symbol to skip lines in the edge types file. Lines starting with this symbol will be skipped.
    /// load_edge_type_list_in_parallel: Optional[bool],
    ///     Whether to load the edge type list in parallel. When loading in parallel, without edge type IDs, the edge types may not be loaded in a deterministic order.
    /// edge_path: Optional[str],
    ///     The path to the file with the edge list.
    /// edge_list_separator: Optional[str],
    ///     The separator to use for the edge list. Note that, if None is provided, one will be attempted to be detected automatically between ';', ',', tab or space.
    /// edge_list_header: Optional[bool],
    ///     Whether the edges file has an header.
    /// edge_list_rows_to_skip: Optional[int],
    ///     Number of rows to skip in the edge list file.
    /// sources_column_number: Optional[int],
    ///     The number of the column of the edges file from where to load the source nodes.
    /// sources_column: Optional[str],
    ///     The name of the column of the edges file from where to load the source nodes.
    /// destinations_column_number: Optional[int],
    ///     The number of the column of the edges file from where to load the destinaton nodes.
    /// destinations_column: Optional[str],
    ///     The name of the column of the edges file from where to load the destinaton nodes.
    /// edge_list_edge_types_column_number: Optional[int],
    ///     The number of the column of the edges file from where to load the edge types.
    /// edge_list_edge_types_column: Optional[str],
    ///     The name of the column of the edges file from where to load the edge types.
    /// default_edge_type: Optional[str],
    ///     The edge type to be used when the edge type for a given edge in the edge file is None.
    /// weights_column_number: Optional[int],
    ///     The number of the column of the edges file from where to load the edge weights.
    /// weights_column: Optional[str],
    ///     The name of the column of the edges file from where to load the edge weights.
    /// default_weight: Optional[float],
    ///     The edge weight to be used when the edge weight for a given edge in the edge file is None.
    /// edge_ids_column: Optional[str],
    ///     The name of the column of the edges file from where to load the edge IDs.
    /// edge_ids_column_number: Optional[int],
    ///     The number of the column of the edges file from where to load the edge IDs.
    /// edge_list_numeric_edge_type_ids: Optional[bool],
    ///     Whether to load the edge type IDs as numeric from the edge list.
    /// edge_list_numeric_node_ids: Optional[bool],
    ///     Whether to load the edge node IDs as numeric from the edge list.
    /// skip_weights_if_unavailable: Optional[bool],
    ///     Whether to skip the weights without raising an error if these are unavailable.
    /// skip_edge_types_if_unavailable: Optional[bool],
    ///     Whether to skip the edge types without raising an error if these are unavailable.
    /// edge_list_is_complete: Optional[bool],
    ///     Whether to consider the edge list as complete, i.e. the edges are presented in both directions when loading an undirected graph.
    /// edge_list_may_contain_duplicates: Optional[bool],
    ///     Whether the edge list may contain duplicates. If the edge list surely DOES NOT contain duplicates, a validation step may be skipped. By default, it is assumed that the edge list may contain duplicates.
    /// edge_list_is_sorted: Optional[bool],
    ///     Whether the edge list is sorted. Note that a sorted edge list has the minimal memory peak, but requires the nodes number and the edges number.
    /// edge_list_is_correct: Optional[bool],
    ///     Whether the edges file can be assumed to be correct, i.e. does not have something wrong in it. If this parameter is passed as true on a malformed file, the constructor will crash.
    /// edge_list_max_rows_number: Optional[int],
    ///     The maximum number of lines to be loaded from the edges file.
    /// edge_list_comment_symbol: Optional[str],
    ///     The comment symbol to skip lines in the edges file. Lines starting with this symbol will be skipped.
    /// edges_number: Optional[int],
    ///     The expected number of edges. Note that this must be the EXACT number of edges in the graph.
    /// load_edge_list_in_parallel: Optional[bool],
    ///     Whether to load the edge list in parallel. Note that, if the edge IDs indices are not given, it is NOT possible to load a sorted edge list. Similarly, when loading in parallel, without edge IDs, the edges may not be loaded in a deterministic order.
    /// verbose: Optional[bool],
    ///     Whether to show a loading bar while reading the files. Note that, if parallel loading is enabled, loading bars will not be showed because they are a synchronization bottleneck.
    /// may_have_singletons: Optional[bool],
    ///     Whether the graph may be expected to have singleton nodes. If it is said that it surely DOES NOT have any, it will allow for some speedups and lower mempry peaks.
    /// may_have_singleton_with_selfloops: Optional[bool],
    ///     Whether the graph may be expected to have singleton nodes with selfloops. If it is said that it surely DOES NOT have any, it will allow for some speedups and lower mempry peaks.
    /// directed: bool,
    ///     Whether to load the graph as directed or undirected.
    /// name: Optional[str],
    ///     The name of the graph to be loaded.
    ///
    pub fn from_csv(
        node_type_path: Option<String>,
        node_type_list_separator: Option<String>,
        node_types_column_number: Option<usize>,
        node_types_column: Option<String>,
        node_types_ids_column_number: Option<usize>,
        node_types_ids_column: Option<String>,
        node_types_number: Option<NodeTypeT>,
        numeric_node_type_ids: Option<bool>,
        minimum_node_type_id: Option<NodeTypeT>,
        node_type_list_header: Option<bool>,
        node_type_list_rows_to_skip: Option<usize>,
        node_type_list_is_correct: Option<bool>,
        node_type_list_max_rows_number: Option<usize>,
        node_type_list_comment_symbol: Option<String>,
        load_node_type_list_in_parallel: Option<bool>,
        node_path: Option<String>,
        node_list_separator: Option<String>,
        node_list_header: Option<bool>,
        node_list_rows_to_skip: Option<usize>,
        node_list_is_correct: Option<bool>,
        node_list_max_rows_number: Option<usize>,
        node_list_comment_symbol: Option<String>,
        default_node_type: Option<String>,
        nodes_column_number: Option<usize>,
        nodes_column: Option<String>,
        node_types_separator: Option<String>,
        node_list_node_types_column_number: Option<usize>,
        node_list_node_types_column: Option<String>,
        node_ids_column: Option<String>,
        node_ids_column_number: Option<usize>,
        nodes_number: Option<NodeT>,
        minimum_node_id: Option<NodeT>,
        numeric_node_ids: Option<bool>,
        node_list_numeric_node_type_ids: Option<bool>,
        skip_node_types_if_unavailable: Option<bool>,
        load_node_list_in_parallel: Option<bool>,
        edge_type_path: Option<String>,
        edge_types_column_number: Option<usize>,
        edge_types_column: Option<String>,
        edge_types_ids_column_number: Option<usize>,
        edge_types_ids_column: Option<String>,
        edge_types_number: Option<EdgeTypeT>,
        numeric_edge_type_ids: Option<bool>,
        minimum_edge_type_id: Option<EdgeTypeT>,
        edge_type_list_separator: Option<String>,
        edge_type_list_header: Option<bool>,
        edge_type_list_rows_to_skip: Option<usize>,
        edge_type_list_is_correct: Option<bool>,
        edge_type_list_max_rows_number: Option<usize>,
        edge_type_list_comment_symbol: Option<String>,
        load_edge_type_list_in_parallel: Option<bool>,
        edge_path: Option<String>,
        edge_list_separator: Option<String>,
        edge_list_header: Option<bool>,
        edge_list_rows_to_skip: Option<usize>,
        sources_column_number: Option<usize>,
        sources_column: Option<String>,
        destinations_column_number: Option<usize>,
        destinations_column: Option<String>,
        edge_list_edge_types_column_number: Option<usize>,
        edge_list_edge_types_column: Option<String>,
        default_edge_type: Option<String>,
        weights_column_number: Option<usize>,
        weights_column: Option<String>,
        default_weight: Option<WeightT>,
        edge_ids_column: Option<String>,
        edge_ids_column_number: Option<usize>,
        edge_list_numeric_edge_type_ids: Option<bool>,
        edge_list_numeric_node_ids: Option<bool>,
        skip_weights_if_unavailable: Option<bool>,
        skip_edge_types_if_unavailable: Option<bool>,
        edge_list_is_complete: Option<bool>,
        edge_list_may_contain_duplicates: Option<bool>,
        edge_list_is_sorted: Option<bool>,
        edge_list_is_correct: Option<bool>,
        edge_list_max_rows_number: Option<usize>,
        edge_list_comment_symbol: Option<String>,
        edges_number: Option<EdgeT>,
        load_edge_list_in_parallel: Option<bool>,
        verbose: Option<bool>,
        may_have_singletons: Option<bool>,
        may_have_singleton_with_selfloops: Option<bool>,
        directed: bool,
        name: Option<String>,
    ) -> PyResult<Graph> {
        Ok(pe!(graph::Graph::from_csv(
            node_type_path.into(),
            node_type_list_separator.into(),
            node_types_column_number.into(),
            node_types_column.into(),
            node_types_ids_column_number.into(),
            node_types_ids_column.into(),
            node_types_number.into(),
            numeric_node_type_ids.into(),
            minimum_node_type_id.into(),
            node_type_list_header.into(),
            node_type_list_rows_to_skip.into(),
            node_type_list_is_correct.into(),
            node_type_list_max_rows_number.into(),
            node_type_list_comment_symbol.into(),
            load_node_type_list_in_parallel.into(),
            node_path.into(),
            node_list_separator.into(),
            node_list_header.into(),
            node_list_rows_to_skip.into(),
            node_list_is_correct.into(),
            node_list_max_rows_number.into(),
            node_list_comment_symbol.into(),
            default_node_type.into(),
            nodes_column_number.into(),
            nodes_column.into(),
            node_types_separator.into(),
            node_list_node_types_column_number.into(),
            node_list_node_types_column.into(),
            node_ids_column.into(),
            node_ids_column_number.into(),
            nodes_number.into(),
            minimum_node_id.into(),
            numeric_node_ids.into(),
            node_list_numeric_node_type_ids.into(),
            skip_node_types_if_unavailable.into(),
            load_node_list_in_parallel.into(),
            edge_type_path.into(),
            edge_types_column_number.into(),
            edge_types_column.into(),
            edge_types_ids_column_number.into(),
            edge_types_ids_column.into(),
            edge_types_number.into(),
            numeric_edge_type_ids.into(),
            minimum_edge_type_id.into(),
            edge_type_list_separator.into(),
            edge_type_list_header.into(),
            edge_type_list_rows_to_skip.into(),
            edge_type_list_is_correct.into(),
            edge_type_list_max_rows_number.into(),
            edge_type_list_comment_symbol.into(),
            load_edge_type_list_in_parallel.into(),
            edge_path.into(),
            edge_list_separator.into(),
            edge_list_header.into(),
            edge_list_rows_to_skip.into(),
            sources_column_number.into(),
            sources_column.into(),
            destinations_column_number.into(),
            destinations_column.into(),
            edge_list_edge_types_column_number.into(),
            edge_list_edge_types_column.into(),
            default_edge_type.into(),
            weights_column_number.into(),
            weights_column.into(),
            default_weight.into(),
            edge_ids_column.into(),
            edge_ids_column_number.into(),
            edge_list_numeric_edge_type_ids.into(),
            edge_list_numeric_node_ids.into(),
            skip_weights_if_unavailable.into(),
            skip_edge_types_if_unavailable.into(),
            edge_list_is_complete.into(),
            edge_list_may_contain_duplicates.into(),
            edge_list_is_sorted.into(),
            edge_list_is_correct.into(),
            edge_list_max_rows_number.into(),
            edge_list_comment_symbol.into(),
            edges_number.into(),
            load_edge_list_in_parallel.into(),
            verbose.into(),
            may_have_singletons.into(),
            may_have_singleton_with_selfloops.into(),
            directed.into(),
            name.into()
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Convert inplace the graph to directed.
    pub fn to_directed_inplace(&mut self) {
        self.inner.to_directed_inplace();
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return a new instance of the current graph as directed
    pub fn to_directed(&self) -> Graph {
        self.inner.to_directed().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the directed graph from the upper triangular adjacency matrix.
    pub fn to_upper_triangular(&self) -> Graph {
        self.inner.to_upper_triangular().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the directed graph from the lower triangular adjacency matrix.
    pub fn to_lower_triangular(&self) -> Graph {
        self.inner.to_lower_triangular().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the graph from the main diagonal adjacency matrix.
    pub fn to_main_diagonal(&self) -> Graph {
        self.inner.to_main_diagonal().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the graph from the anti-diagonal adjacency matrix.
    pub fn to_anti_diagonal(&self) -> Graph {
        self.inner.to_anti_diagonal().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the graph from the bidiagonal adjacency matrix.
    pub fn to_bidiagonal(&self) -> Graph {
        self.inner.to_bidiagonal().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the graph from the arrowhead adjacency matrix.
    pub fn to_arrowhead(&self) -> Graph {
        self.inner.to_arrowhead().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the graph from the transposed adjacency matrix.
    pub fn to_transposed(&self) -> Graph {
        self.inner.to_transposed().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the complementary graph.
    pub fn to_complementary(&self) -> Graph {
        self.inner.to_complementary().into()
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
        self.inner
            .get_number_of_triangles(normalize.into(), low_centrality.into(), verbose.into())
            .into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns total number of triads in the graph without taking into account weights
    pub fn get_triads_number(&self) -> EdgeT {
        self.inner.get_triads_number().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns total number of triads in the weighted graph
    pub fn get_weighted_triads_number(&self) -> PyResult<f64> {
        Ok(pe!(self.inner.get_weighted_triads_number())?.into())
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
        self.inner
            .get_transitivity(low_centrality.into(), verbose.into())
            .into()
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
            self.inner.get_number_of_triangles_per_node(
                normalize.into(),
                low_centrality.into(),
                verbose.into()
            ),
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
            self.inner
                .get_clustering_coefficient_per_node(low_centrality.into(), verbose.into()),
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
        self.inner
            .get_clustering_coefficient(low_centrality.into(), verbose.into())
            .into()
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
        self.inner
            .get_average_clustering_coefficient(low_centrality.into(), verbose.into())
            .into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return list of the supported sparse edge weighting methods
    pub fn get_sparse_edge_weighting_methods(&self) -> Vec<&str> {
        self.inner.get_sparse_edge_weighting_methods().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return list of the supported edge weighting methods
    pub fn get_edge_weighting_methods(&self) -> Vec<&str> {
        self.inner.get_edge_weighting_methods().into()
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
        self.inner.encode_edge(src.into(), dst.into()).into()
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
        self.inner.decode_edge(edge.into()).into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return maximum encodable edge number
    pub fn get_max_encodable_edge_number(&self) -> EdgeT {
        self.inner.get_max_encodable_edge_number().into()
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
    /// seed_graph: Optional[Graph],
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
        seed_graph: Option<&Graph>,
        only_from_same_component: Option<bool>,
        verbose: Option<bool>,
    ) -> PyResult<Graph> {
        Ok(pe!(self.inner.sample_negatives(
            negatives_number.into(),
            random_state.into(),
            seed_graph.map(|sg| &sg.inner),
            only_from_same_component.into(),
            verbose.into()
        ))?
        .into())
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
    ) -> PyResult<(Graph, Graph)> {
        let (g1, g2) = pe!(self.inner.connected_holdout(
            train_size.into(),
            random_state.into(),
            edge_types.into(),
            include_all_edge_types.into(),
            verbose.into()
        ))?;
        Ok((g1.into(), g2.into()))
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
    ) -> PyResult<(Graph, Graph)> {
        let (g1, g2) = pe!(self.inner.random_holdout(
            train_size.into(),
            random_state.into(),
            include_all_edge_types.into(),
            edge_types.into(),
            min_number_overlaps.into(),
            verbose.into()
        ))?;
        Ok((g1.into(), g2.into()))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, train_size, use_stratification, random_state)"]
    /// Returns node-label holdout indices for training ML algorithms on the graph node labels.
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
    pub fn get_node_label_holdout_indices(
        &self,
        train_size: f64,
        use_stratification: Option<bool>,
        random_state: Option<EdgeT>,
    ) -> PyResult<(Vec<NodeT>, Vec<NodeT>)> {
        Ok(pe!(self.inner.get_node_label_holdout_indices(
            train_size.into(),
            use_stratification.into(),
            random_state.into()
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, train_size, use_stratification, random_state)"]
    /// Returns node-label holdout indices for training ML algorithms on the graph node labels.
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
    pub fn get_node_label_holdout_labels(
        &self,
        train_size: f64,
        use_stratification: Option<bool>,
        random_state: Option<EdgeT>,
    ) -> PyResult<(Vec<Option<Vec<NodeTypeT>>>, Vec<Option<Vec<NodeTypeT>>>)> {
        Ok(pe!(self.inner.get_node_label_holdout_labels(
            train_size.into(),
            use_stratification.into(),
            random_state.into()
        ))?
        .into())
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
    pub fn get_node_label_holdout_graphs(
        &self,
        train_size: f64,
        use_stratification: Option<bool>,
        random_state: Option<EdgeT>,
    ) -> PyResult<(Graph, Graph)> {
        let (g1, g2) = pe!(self.inner.get_node_label_holdout_graphs(
            train_size.into(),
            use_stratification.into(),
            random_state.into()
        ))?;
        Ok((g1.into(), g2.into()))
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
    pub fn get_edge_label_holdout_graphs(
        &self,
        train_size: f64,
        use_stratification: Option<bool>,
        random_state: Option<EdgeT>,
    ) -> PyResult<(Graph, Graph)> {
        let (g1, g2) = pe!(self.inner.get_edge_label_holdout_graphs(
            train_size.into(),
            use_stratification.into(),
            random_state.into()
        ))?;
        Ok((g1.into(), g2.into()))
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
    pub fn get_random_subgraph(
        &self,
        nodes_number: NodeT,
        random_state: Option<usize>,
        verbose: Option<bool>,
    ) -> PyResult<Graph> {
        Ok(pe!(self.inner.get_random_subgraph(
            nodes_number.into(),
            random_state.into(),
            verbose.into()
        ))?
        .into())
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
    pub fn get_node_label_random_holdout(
        &self,
        train_size: f64,
        use_stratification: Option<bool>,
        random_state: Option<EdgeT>,
    ) -> PyResult<(Graph, Graph)> {
        let (g1, g2) = pe!(self.inner.get_node_label_random_holdout(
            train_size.into(),
            use_stratification.into(),
            random_state.into()
        ))?;
        Ok((g1.into(), g2.into()))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, k, k_index, use_stratification, random_state)"]
    /// Returns node-label fold for training ML algorithms on the graph node labels.
    ///
    /// Parameters
    /// ----------
    /// k: int,
    ///     The number of folds.
    /// k_index: int,
    ///     Which fold to use for the validation.
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
    pub fn get_node_label_kfold(
        &self,
        k: usize,
        k_index: usize,
        use_stratification: Option<bool>,
        random_state: Option<EdgeT>,
    ) -> PyResult<(Graph, Graph)> {
        let (g1, g2) = pe!(self.inner.get_node_label_kfold(
            k.into(),
            k_index.into(),
            use_stratification.into(),
            random_state.into()
        ))?;
        Ok((g1.into(), g2.into()))
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
    pub fn get_edge_label_random_holdout(
        &self,
        train_size: f64,
        use_stratification: Option<bool>,
        random_state: Option<EdgeT>,
    ) -> PyResult<(Graph, Graph)> {
        let (g1, g2) = pe!(self.inner.get_edge_label_random_holdout(
            train_size.into(),
            use_stratification.into(),
            random_state.into()
        ))?;
        Ok((g1.into(), g2.into()))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, k, k_index, use_stratification, random_state)"]
    /// Returns edge-label kfold for training ML algorithms on the graph edge labels.
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
    /// k: int,
    ///     The number of folds.
    /// k_index: int,
    ///     Which fold to use for the validation.
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
    pub fn get_edge_label_kfold(
        &self,
        k: usize,
        k_index: usize,
        use_stratification: Option<bool>,
        random_state: Option<EdgeT>,
    ) -> PyResult<(Graph, Graph)> {
        let (g1, g2) = pe!(self.inner.get_edge_label_kfold(
            k.into(),
            k_index.into(),
            use_stratification.into(),
            random_state.into()
        ))?;
        Ok((g1.into(), g2.into()))
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
    pub fn get_edge_prediction_kfold(
        &self,
        k: usize,
        k_index: usize,
        edge_types: Option<Vec<Option<String>>>,
        random_state: Option<EdgeT>,
        verbose: Option<bool>,
    ) -> PyResult<(Graph, Graph)> {
        let (g1, g2) = pe!(self.inner.get_edge_prediction_kfold(
            k.into(),
            k_index.into(),
            edge_types.into(),
            random_state.into(),
            verbose.into()
        ))?;
        Ok((g1.into(), g2.into()))
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, recursion_minimum_improvement, first_phase_minimum_improvement, patience, random_state)"]
    /// Returns vector of vectors of communities for each layer of hierarchy minimizing undirected modularity.
    ///
    /// Parameters
    /// ----------
    /// recursion_minimum_improvement: Optional[float],
    ///     The minimum improvement to warrant another resursion round. By default, zero.
    /// first_phase_minimum_improvement: Optional[float],
    ///     The minimum improvement to warrant another first phase iteration. By default, `0.00001` (not zero because of numerical instability).
    /// patience: Optional[int],
    ///     How many iterations of the first phase to wait for before stopping. By default, `5`.
    /// random_state: Optional[int],
    ///     The random state to use to reproduce this modularity computation. By default, 42.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph is not directed.
    /// ValueError
    ///     If the `recursion_minimum_improvement` has an invalid value, i.e. NaN or infinity.
    /// ValueError
    ///     If the `first_phase_minimum_improvement` has an invalid value, i.e. NaN or infinity.
    ///
    pub fn get_undirected_louvain_community_detection(
        &self,
        recursion_minimum_improvement: Option<f64>,
        first_phase_minimum_improvement: Option<f64>,
        patience: Option<usize>,
        random_state: Option<u64>,
    ) -> PyResult<Vec<Vec<usize>>> {
        Ok(pe!(self.inner.get_undirected_louvain_community_detection(
            recursion_minimum_improvement.into(),
            first_phase_minimum_improvement.into(),
            patience.into(),
            random_state.into()
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_community_memberships)"]
    /// Returns the directed modularity of the graph from the given memberships.
    ///
    /// Parameters
    /// ----------
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the number of provided memberships does not match the number of nodes of the graph.
    ///
    pub fn get_directed_modularity_from_node_community_memberships(
        &self,
        node_community_memberships: Vec<NodeT>,
    ) -> PyResult<f64> {
        Ok(pe!(self
            .inner
            .get_directed_modularity_from_node_community_memberships(&node_community_memberships))?
        .into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_community_memberships)"]
    /// Returns the undirected modularity of the graph from the given memberships.
    ///
    /// Parameters
    /// ----------
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the number of provided memberships does not match the number of nodes of the graph.
    ///
    pub fn get_undirected_modularity_from_node_community_memberships(
        &self,
        node_community_memberships: Vec<NodeT>,
    ) -> PyResult<f64> {
        Ok(pe!(self
            .inner
            .get_undirected_modularity_from_node_community_memberships(
                &node_community_memberships
            ))?
        .into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, other)"]
    /// Return true if the graphs are compatible.
    ///
    /// Parameters
    /// ----------
    /// other: Graph,
    ///     The other graph.
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
    pub fn is_compatible(&self, other: &Graph) -> PyResult<bool> {
        Ok(pe!(self.inner.is_compatible(&other.inner))?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, other)"]
    /// Return true if the graphs share the same adjacency matrix.
    ///
    /// Parameters
    /// ----------
    /// other: Graph,
    ///     The other graph.
    ///
    pub fn has_same_adjacency_matrix(&self, other: &Graph) -> PyResult<bool> {
        Ok(pe!(self.inner.has_same_adjacency_matrix(&other.inner))?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns unweighted laplacian transformation of the graph
    pub fn get_laplacian_transformed_graph(&self) -> Graph {
        self.inner.get_laplacian_transformed_graph().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns number of edges in the laplacian COO matrix representation of the graph
    pub fn get_laplacian_coo_matrix_edges_number(&self) -> EdgeT {
        self.inner.get_laplacian_coo_matrix_edges_number().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns unweighted random walk normalized laplacian transformation of the graph
    pub fn get_random_walk_normalized_laplacian_transformed_graph(&self) -> Graph {
        self.inner
            .get_random_walk_normalized_laplacian_transformed_graph()
            .into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns unweighted symmetric normalized laplacian transformation of the graph.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     The graph must be undirected, as we do not currently support this transformation for directed graphs.
    ///
    pub fn get_symmetric_normalized_laplacian_transformed_graph(&self) -> PyResult<Graph> {
        Ok(pe!(self
            .inner
            .get_symmetric_normalized_laplacian_transformed_graph())?
        .into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns unweighted symmetric normalized transformation of the graph.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     The graph must be undirected, as we do not currently support this transformation for directed graphs.
    ///
    pub fn get_symmetric_normalized_transformed_graph(&self) -> PyResult<Graph> {
        Ok(pe!(self.inner.get_symmetric_normalized_transformed_graph())?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns a string describing the memory usage of all the fields of all the
    /// structures used to store the current graph
    pub fn get_memory_stats(&self) -> String {
        self.inner.get_memory_stats().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns how many bytes are currently used to store the given graph
    pub fn get_total_memory_used(&self) -> usize {
        self.inner.get_total_memory_used().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns how many bytes are currently used to store the nodes
    pub fn get_nodes_total_memory_requirement(&self) -> usize {
        self.inner.get_nodes_total_memory_requirement().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns human readable amount of how many bytes are currently used to store the nodes
    pub fn get_nodes_total_memory_requirement_human_readable(&self) -> String {
        self.inner
            .get_nodes_total_memory_requirement_human_readable()
            .into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns how many bytes are currently used to store the edges
    pub fn get_edges_total_memory_requirement(&self) -> usize {
        self.inner.get_edges_total_memory_requirement().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns human readable amount of how many bytes are currently used to store the edges
    pub fn get_edges_total_memory_requirement_human_readable(&self) -> String {
        self.inner
            .get_edges_total_memory_requirement_human_readable()
            .into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns how many bytes are currently used to store the edge weights
    pub fn get_edge_weights_total_memory_requirements(&self) -> usize {
        self.inner
            .get_edge_weights_total_memory_requirements()
            .into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns human readable amount of how many bytes are currently used to store the edge weights
    pub fn get_edge_weights_total_memory_requirements_human_readable(&self) -> String {
        self.inner
            .get_edge_weights_total_memory_requirements_human_readable()
            .into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns how many bytes are currently used to store the node types
    pub fn get_node_types_total_memory_requirements(&self) -> PyResult<usize> {
        Ok(pe!(self.inner.get_node_types_total_memory_requirements())?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns human readable amount of how many bytes are currently used to store the node types
    pub fn get_node_types_total_memory_requirements_human_readable(&self) -> PyResult<String> {
        Ok(pe!(self
            .inner
            .get_node_types_total_memory_requirements_human_readable())?
        .into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns how many bytes are currently used to store the edge types
    pub fn get_edge_types_total_memory_requirements(&self) -> PyResult<usize> {
        Ok(pe!(self.inner.get_edge_types_total_memory_requirements())?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns human readable amount of how many bytes are currently used to store the edge types
    pub fn get_edge_types_total_memory_requirements_human_readable(&self) -> PyResult<String> {
        Ok(pe!(self
            .inner
            .get_edge_types_total_memory_requirements_human_readable())?
        .into())
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
    ) -> Graph {
        self.inner
            .get_transitive_closure(iterations.into(), verbose.into())
            .into()
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
    ) -> Graph {
        self.inner
            .get_all_shortest_paths(iterations.into(), verbose.into())
            .into()
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
    ) -> PyResult<Graph> {
        Ok(pe!(self.inner.get_weighted_all_shortest_paths(
            iterations.into(),
            use_edge_weights_as_probabilities.into(),
            verbose.into()
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_ids_to_keep, node_ids_to_filter, node_type_ids_to_keep, node_type_ids_to_filter, node_type_id_to_keep, node_type_id_to_filter, edge_ids_to_keep, edge_ids_to_filter, edge_node_ids_to_keep, edge_node_ids_to_filter, edge_type_ids_to_keep, edge_type_ids_to_filter, min_edge_weight, max_edge_weight, filter_singleton_nodes, filter_singleton_nodes_with_selfloop, filter_selfloops, filter_parallel_edges)"]
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
    ) -> PyResult<Graph> {
        Ok(pe!(self.inner.filter_from_ids(
            node_ids_to_keep.into(),
            node_ids_to_filter.into(),
            node_type_ids_to_keep.into(),
            node_type_ids_to_filter.into(),
            node_type_id_to_keep.into(),
            node_type_id_to_filter.into(),
            edge_ids_to_keep.into(),
            edge_ids_to_filter.into(),
            edge_node_ids_to_keep.into(),
            edge_node_ids_to_filter.into(),
            edge_type_ids_to_keep.into(),
            edge_type_ids_to_filter.into(),
            min_edge_weight.into(),
            max_edge_weight.into(),
            filter_singleton_nodes.into(),
            filter_singleton_nodes_with_selfloop.into(),
            filter_selfloops.into(),
            filter_parallel_edges.into()
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_names_to_keep, node_names_to_filter, node_type_names_to_keep, node_type_names_to_filter, node_type_name_to_keep, node_type_name_to_filter, edge_node_names_to_keep, edge_node_names_to_filter, edge_type_names_to_keep, edge_type_names_to_filter, min_edge_weight, max_edge_weight, filter_singleton_nodes, filter_singleton_nodes_with_selfloop, filter_selfloops, filter_parallel_edges)"]
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
    ) -> PyResult<Graph> {
        Ok(pe!(self.inner.filter_from_names(
            node_names_to_keep.into(),
            node_names_to_filter.into(),
            node_type_names_to_keep.into(),
            node_type_names_to_filter.into(),
            node_type_name_to_keep.into(),
            node_type_name_to_filter.into(),
            edge_node_names_to_keep.into(),
            edge_node_names_to_filter.into(),
            edge_type_names_to_keep.into(),
            edge_type_names_to_filter.into(),
            min_edge_weight.into(),
            max_edge_weight.into(),
            filter_singleton_nodes.into(),
            filter_singleton_nodes_with_selfloop.into(),
            filter_selfloops.into(),
            filter_parallel_edges.into()
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns new graph without unknown node types and relative nodes.
    ///
    /// Note that this method will remove ALL nodes labeled with unknown node
    /// type!
    pub fn drop_unknown_node_types(&self) -> Graph {
        self.inner.drop_unknown_node_types().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns new graph without unknown edge types and relative edges.
    ///
    /// Note that this method will remove ALL edges labeled with unknown edge
    /// type!
    pub fn drop_unknown_edge_types(&self) -> Graph {
        self.inner.drop_unknown_edge_types().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns new graph without singleton nodes.
    ///
    /// A node is singleton when does not have neither incoming or outgoing edges.
    pub fn drop_singleton_nodes(&self) -> Graph {
        self.inner.drop_singleton_nodes().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns new graph without singleton nodes with selfloops.
    ///
    /// A node is singleton with selfloop when does not have neither incoming or outgoing edges.
    pub fn drop_singleton_nodes_with_selfloops(&self) -> Graph {
        self.inner.drop_singleton_nodes_with_selfloops().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns new graph without disconnected nodes.
    ///
    /// A disconnected node is a node with no connection to any other node.
    pub fn drop_disconnected_nodes(&self) -> Graph {
        self.inner.drop_disconnected_nodes().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns new graph without selfloops.
    pub fn drop_selfloops(&self) -> Graph {
        self.inner.drop_selfloops().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns new graph without parallel edges
    pub fn drop_parallel_edges(&self) -> Graph {
        self.inner.drop_parallel_edges().into()
    }
}

pub const GRAPH_METHODS_NAMES: &[&str] = &[
    "are_nodes_remappable",
    "remap_unchecked_from_node_ids",
    "remap_from_node_ids",
    "remap_from_node_names",
    "remap_from_graph",
    "set_inplace_all_edge_types",
    "set_all_edge_types",
    "set_inplace_all_node_types",
    "set_all_node_types",
    "remove_inplace_node_type_ids",
    "remove_inplace_singleton_node_types",
    "remove_inplace_edge_type_ids",
    "remove_inplace_singleton_edge_types",
    "remove_inplace_node_type_name",
    "remove_node_type_id",
    "remove_singleton_node_types",
    "remove_node_type_name",
    "remove_inplace_edge_type_name",
    "remove_edge_type_id",
    "remove_singleton_edge_types",
    "remove_edge_type_name",
    "remove_inplace_node_types",
    "remove_node_types",
    "remove_inplace_edge_types",
    "remove_edge_types",
    "remove_inplace_edge_weights",
    "remove_edge_weights",
    "add_selfloops",
    "overlaps",
    "contains",
    "get_random_nodes",
    "get_breadth_first_search_random_nodes",
    "get_uniform_random_walk_random_nodes",
    "get_node_sampling_methods",
    "get_subsampled_nodes",
    "has_default_graph_name",
    "has_nodes",
    "has_edges",
    "has_trap_nodes",
    "is_directed",
    "has_edge_weights",
    "has_edge_weights_representing_probabilities",
    "has_weighted_singleton_nodes",
    "has_constant_edge_weights",
    "has_negative_edge_weights",
    "has_edge_types",
    "has_selfloops",
    "has_disconnected_nodes",
    "has_singleton_nodes",
    "has_singleton_nodes_with_selfloops",
    "is_connected",
    "has_node_types",
    "has_multilabel_node_types",
    "has_unknown_node_types",
    "has_known_node_types",
    "has_unknown_edge_types",
    "has_known_edge_types",
    "has_homogeneous_node_types",
    "has_homogeneous_edge_types",
    "has_singleton_node_types",
    "has_node_oddities",
    "has_node_types_oddities",
    "has_singleton_edge_types",
    "has_edge_types_oddities",
    "is_multigraph",
    "has_nodes_sorted_by_decreasing_outbound_node_degree",
    "has_nodes_sorted_by_lexicographic_order",
    "contains_identity_matrix",
    "has_nodes_sorted_by_increasing_outbound_node_degree",
    "get_connected_components_number",
    "get_connected_nodes_number",
    "get_singleton_nodes_with_selfloops_number",
    "get_singleton_nodes_number",
    "get_disconnected_nodes_number",
    "get_singleton_node_ids",
    "get_singleton_node_names",
    "get_singleton_with_selfloops_node_ids",
    "get_singleton_with_selfloops_node_names",
    "get_density",
    "get_trap_nodes_rate",
    "get_node_degrees_mean",
    "get_weighted_node_degrees_mean",
    "get_undirected_edges_number",
    "get_unique_undirected_edges_number",
    "get_edges_number",
    "get_unique_edges_number",
    "get_node_degrees_median",
    "get_weighted_node_degrees_median",
    "get_maximum_node_degree",
    "get_unchecked_most_central_node_id",
    "get_most_central_node_id",
    "get_minimum_node_degree",
    "get_node_degrees_mode",
    "get_selfloop_nodes_rate",
    "get_name",
    "get_trap_nodes_number",
    "get_source_node_ids",
    "get_directed_source_node_ids",
    "get_source_names",
    "get_destination_node_ids",
    "get_directed_destination_node_ids",
    "get_destination_names",
    "get_node_names",
    "get_node_urls",
    "get_node_ontologies",
    "get_node_ids",
    "get_edge_type_ids",
    "get_unique_edge_type_ids",
    "get_edge_type_names",
    "get_unique_edge_type_names",
    "get_edge_weights",
    "get_weighted_node_indegrees",
    "get_node_type_ids",
    "get_known_node_types_mask",
    "get_unknown_node_types_mask",
    "get_one_hot_encoded_node_types",
    "get_one_hot_encoded_known_node_types",
    "get_one_hot_encoded_edge_types",
    "get_one_hot_encoded_known_edge_types",
    "get_node_type_names",
    "get_unique_node_type_ids",
    "get_unique_node_type_names",
    "get_unique_directed_edges_number",
    "get_nodes_mapping",
    "get_edge_node_ids",
    "get_directed_edge_node_ids",
    "get_edge_node_names",
    "get_directed_edge_node_names",
    "get_unknown_node_types_number",
    "get_known_node_types_number",
    "get_unknown_node_types_rate",
    "get_known_node_types_rate",
    "get_minimum_node_types_number",
    "get_maximum_node_types_number",
    "get_maximum_multilabel_count",
    "get_singleton_node_types_number",
    "get_singleton_node_type_ids",
    "get_singleton_node_type_names",
    "get_unknown_edge_types_number",
    "get_edge_ids_with_unknown_edge_types",
    "get_edge_ids_with_known_edge_types",
    "get_edge_node_ids_with_unknown_edge_types",
    "get_edge_node_ids_with_known_edge_types",
    "get_edge_node_names_with_unknown_edge_types",
    "get_edge_node_names_with_known_edge_types",
    "get_edge_ids_with_unknown_edge_types_mask",
    "get_edge_ids_with_known_edge_types_mask",
    "get_node_ids_with_unknown_node_types",
    "get_node_ids_with_known_node_types",
    "get_node_names_with_unknown_node_types",
    "get_node_names_with_known_node_types",
    "get_node_ids_with_unknown_node_types_mask",
    "get_node_ids_with_known_node_types_mask",
    "get_known_edge_types_number",
    "get_unknown_edge_types_rate",
    "get_known_edge_types_rate",
    "get_minimum_edge_types_number",
    "get_singleton_edge_types_number",
    "get_singleton_edge_type_ids",
    "get_singleton_edge_type_names",
    "get_nodes_number",
    "get_node_connected_component_ids",
    "get_directed_edges_number",
    "get_edge_types_number",
    "get_node_types_number",
    "get_node_degrees",
    "get_node_indegrees",
    "get_weighted_node_degrees",
    "get_not_singletons_node_ids",
    "get_dense_nodes_mapping",
    "get_parallel_edges_number",
    "get_cumulative_node_degrees",
    "get_unique_source_nodes_number",
    "get_edge_type_id_counts_hashmap",
    "get_edge_type_names_counts_hashmap",
    "get_node_type_id_counts_hashmap",
    "get_node_type_names_counts_hashmap",
    "get_total_edge_weights",
    "get_mininum_edge_weight",
    "get_maximum_edge_weight",
    "get_unchecked_maximum_node_degree",
    "get_unchecked_minimum_node_degree",
    "get_weighted_maximum_node_degree",
    "get_weighted_minimum_node_degree",
    "get_weighted_singleton_nodes_number",
    "get_selfloops_number",
    "get_unique_selfloops_number",
    "is_unchecked_connected_from_node_id",
    "is_unchecked_disconnected_node_from_node_id",
    "is_unchecked_singleton_from_node_id",
    "is_singleton_from_node_id",
    "is_unchecked_singleton_with_selfloops_from_node_id",
    "is_singleton_with_selfloops_from_node_id",
    "is_unchecked_singleton_from_node_name",
    "is_singleton_from_node_name",
    "has_node_name",
    "has_node_type_id",
    "has_node_type_name",
    "has_edge_type_id",
    "has_edge_type_name",
    "has_edge_from_node_ids",
    "has_selfloop_from_node_id",
    "has_edge_from_node_ids_and_edge_type_id",
    "is_unchecked_trap_node_from_node_id",
    "is_trap_node_from_node_id",
    "has_node_name_and_node_type_name",
    "has_edge_from_node_names",
    "has_edge_from_node_names_and_edge_type_name",
    "validate_node_id",
    "validate_node_ids",
    "validate_edge_id",
    "validate_edge_ids",
    "must_not_contain_unknown_node_types",
    "must_not_contain_unknown_edge_types",
    "validate_node_type_id",
    "validate_node_type_ids",
    "validate_edge_type_id",
    "validate_edge_type_ids",
    "must_be_undirected",
    "must_be_multigraph",
    "must_not_be_multigraph",
    "must_contain_identity_matrix",
    "must_not_contain_weighted_singleton_nodes",
    "must_have_edges",
    "must_have_nodes",
    "must_be_connected",
    "replace",
    "to_dot",
    "remove_components",
    "enable",
    "random_spanning_arborescence_kruskal",
    "spanning_arborescence_kruskal",
    "connected_components",
    "report",
    "overlap_textual_report",
    "get_node_report_from_node_id",
    "get_node_report_from_node_name",
    "textual_report",
    "get_dense_binary_adjacency_matrix",
    "get_dense_weighted_adjacency_matrix",
    "generate_new_edges_from_node_features",
    "get_unchecked_minimum_preferential_attachment",
    "get_unchecked_maximum_preferential_attachment",
    "get_unchecked_weighted_minimum_preferential_attachment",
    "get_unchecked_weighted_maximum_preferential_attachment",
    "get_unchecked_preferential_attachment_from_node_ids",
    "get_preferential_attachment_from_node_ids",
    "get_preferential_attachment_from_node_names",
    "get_unchecked_weighted_preferential_attachment_from_node_ids",
    "get_weighted_preferential_attachment_from_node_ids",
    "get_weighted_preferential_attachment_from_node_names",
    "get_unchecked_jaccard_coefficient_from_node_ids",
    "get_jaccard_coefficient_from_node_ids",
    "get_jaccard_coefficient_from_node_names",
    "get_unchecked_adamic_adar_index_from_node_ids",
    "get_adamic_adar_index_from_node_ids",
    "get_adamic_adar_index_from_node_names",
    "get_unchecked_resource_allocation_index_from_node_ids",
    "get_unchecked_weighted_resource_allocation_index_from_node_ids",
    "get_resource_allocation_index_from_node_ids",
    "get_resource_allocation_index_from_node_names",
    "get_weighted_resource_allocation_index_from_node_ids",
    "get_weighted_resource_allocation_index_from_node_names",
    "get_unchecked_all_edge_metrics_from_node_ids",
    "strongly_connected_components",
    "get_unchecked_edge_weight_from_edge_id",
    "get_unchecked_edge_weight_from_node_ids",
    "get_unchecked_node_id_from_node_name",
    "get_unchecked_edge_type_id_from_edge_type_name",
    "get_unchecked_edge_type_name_from_edge_type_id",
    "get_unchecked_edge_count_from_edge_type_id",
    "get_unchecked_edge_id_from_node_ids_and_edge_type_id",
    "get_unchecked_minmax_edge_ids_from_node_ids",
    "get_unchecked_node_ids_from_edge_id",
    "get_unchecked_node_names_from_edge_id",
    "get_unchecked_source_node_id_from_edge_id",
    "get_unchecked_destination_node_id_from_edge_id",
    "get_source_node_id_from_edge_id",
    "get_destination_node_id_from_edge_id",
    "get_unchecked_source_node_name_from_edge_id",
    "get_unchecked_destination_node_name_from_edge_id",
    "get_source_node_name_from_edge_id",
    "get_destination_node_name_from_edge_id",
    "get_node_names_from_edge_id",
    "get_node_ids_from_edge_id",
    "get_unchecked_edge_id_from_node_ids",
    "get_edge_id_from_node_ids",
    "get_unchecked_unique_source_node_id",
    "get_unchecked_node_ids_and_edge_type_id_from_edge_id",
    "get_node_ids_and_edge_type_id_from_edge_id",
    "get_unchecked_node_ids_and_edge_type_id_and_edge_weight_from_edge_id",
    "get_node_ids_and_edge_type_id_and_edge_weight_from_edge_id",
    "get_top_k_central_node_ids",
    "get_weighted_top_k_central_node_ids",
    "get_unchecked_node_degree_from_node_id",
    "get_unchecked_weighted_node_degree_from_node_id",
    "get_node_degree_from_node_id",
    "get_unchecked_comulative_node_degree_from_node_id",
    "get_comulative_node_degree_from_node_id",
    "get_weighted_node_degree_from_node_id",
    "get_node_degree_from_node_name",
    "get_top_k_central_node_names",
    "get_unchecked_node_type_id_from_node_id",
    "get_node_type_ids_from_node_id",
    "get_unchecked_edge_type_id_from_edge_id",
    "get_edge_type_id_from_edge_id",
    "get_unchecked_node_type_names_from_node_id",
    "get_node_type_names_from_node_id",
    "get_node_type_names_from_node_name",
    "get_edge_type_name_from_edge_id",
    "get_edge_type_name_from_edge_type_id",
    "get_edge_weight_from_edge_id",
    "get_edge_weight_from_node_ids",
    "get_edge_weight_from_node_ids_and_edge_type_id",
    "get_edge_weight_from_node_names_and_edge_type_name",
    "get_edge_weight_from_node_names",
    "get_unchecked_node_name_from_node_id",
    "get_node_name_from_node_id",
    "get_node_id_from_node_name",
    "get_node_ids_from_node_names",
    "get_edge_node_ids_from_edge_node_names",
    "get_edge_node_names_from_edge_node_ids",
    "get_node_type_ids_from_node_name",
    "get_node_type_name_from_node_name",
    "get_edge_count_from_edge_type_id",
    "get_edge_type_id_from_edge_type_name",
    "get_edge_count_from_edge_type_name",
    "get_node_type_id_from_node_type_name",
    "get_node_count_from_node_type_id",
    "get_node_count_from_node_type_name",
    "get_neighbour_node_ids_from_node_id",
    "get_neighbour_node_ids_from_node_name",
    "get_neighbour_node_names_from_node_name",
    "get_minmax_edge_ids_from_node_ids",
    "get_edge_id_from_node_ids_and_edge_type_id",
    "get_edge_id_from_node_names",
    "get_edge_id_from_node_names_and_edge_type_name",
    "get_edge_type_ids_from_edge_type_names",
    "get_node_type_ids_from_node_type_names",
    "get_multiple_node_type_ids_from_node_type_names",
    "get_unchecked_minmax_edge_ids_from_source_node_id",
    "get_minmax_edge_ids_from_source_node_id",
    "get_node_type_name_from_node_type_id",
    "get_unchecked_node_type_names_from_node_type_ids",
    "get_degree_centrality",
    "get_weighted_degree_centrality",
    "get_unchecked_closeness_centrality_from_node_id",
    "get_unchecked_weighted_closeness_centrality_from_node_id",
    "get_closeness_centrality",
    "get_weighted_closeness_centrality",
    "get_unchecked_harmonic_centrality_from_node_id",
    "get_unchecked_weighted_harmonic_centrality_from_node_id",
    "get_harmonic_centrality",
    "get_weighted_harmonic_centrality",
    "get_stress_centrality",
    "get_betweenness_centrality",
    "get_eigenvector_centrality",
    "get_weighted_eigenvector_centrality",
    "approximated_vertex_cover_set",
    "sort_by_increasing_outbound_node_degree",
    "sort_by_decreasing_outbound_node_degree",
    "sort_by_node_lexicographic_order",
    "get_bfs_topological_sorting_from_node_id",
    "get_reversed_bfs_topological_sorting_from_node_id",
    "sort_by_bfs_topological_sorting_from_node_id",
    "generate_random_connected_graph",
    "generate_random_spanning_tree",
    "generate_circle_graph",
    "generate_chain_graph",
    "generate_complete_graph",
    "generate_barbell_graph",
    "get_unchecked_breadth_first_search_from_node_ids",
    "get_unchecked_shortest_path_node_ids_from_node_ids",
    "get_unchecked_shortest_path_node_names_from_node_ids",
    "get_shortest_path_node_ids_from_node_ids",
    "get_shortest_path_node_ids_from_node_names",
    "get_shortest_path_node_names_from_node_names",
    "get_unchecked_k_shortest_path_node_ids_from_node_ids",
    "get_k_shortest_path_node_ids_from_node_ids",
    "get_k_shortest_path_node_ids_from_node_names",
    "get_k_shortest_path_node_names_from_node_names",
    "get_unchecked_eccentricity_from_node_id",
    "get_unchecked_weighted_eccentricity_from_node_id",
    "get_eccentricity_from_node_id",
    "get_weighted_eccentricity_from_node_id",
    "get_eccentricity_from_node_name",
    "get_weighted_eccentricity_from_node_name",
    "get_unchecked_dijkstra_from_node_ids",
    "get_unchecked_weighted_shortest_path_node_ids_from_node_ids",
    "get_unchecked_weighted_shortest_path_node_names_from_node_ids",
    "get_weighted_shortest_path_node_ids_from_node_ids",
    "get_weighted_shortest_path_node_ids_from_node_names",
    "get_weighted_shortest_path_node_names_from_node_names",
    "get_breadth_first_search_from_node_ids",
    "get_dijkstra_from_node_ids",
    "get_diameter_naive",
    "get_diameter",
    "get_weighted_diameter_naive",
    "get_breadth_first_search_from_node_names",
    "get_dijkstra_from_node_names",
    "get_bipartite_edges",
    "get_bipartite_edge_names",
    "get_star_edges",
    "get_star_edge_names",
    "get_clique_edges",
    "get_clique_edge_names",
    "get_okapi_bm25_node_feature_propagation",
    "get_okapi_bm25_node_label_propagation",
    "from_csv",
    "to_directed_inplace",
    "to_directed",
    "to_upper_triangular",
    "to_lower_triangular",
    "to_main_diagonal",
    "to_anti_diagonal",
    "to_bidiagonal",
    "to_arrowhead",
    "to_transposed",
    "to_complementary",
    "get_number_of_triangles",
    "get_triads_number",
    "get_weighted_triads_number",
    "get_transitivity",
    "get_number_of_triangles_per_node",
    "get_clustering_coefficient_per_node",
    "get_clustering_coefficient",
    "get_average_clustering_coefficient",
    "get_sparse_edge_weighting_methods",
    "get_edge_weighting_methods",
    "encode_edge",
    "decode_edge",
    "get_max_encodable_edge_number",
    "sample_negatives",
    "connected_holdout",
    "random_holdout",
    "get_node_label_holdout_indices",
    "get_node_label_holdout_labels",
    "get_node_label_holdout_graphs",
    "get_edge_label_holdout_graphs",
    "get_random_subgraph",
    "get_node_label_random_holdout",
    "get_node_label_kfold",
    "get_edge_label_random_holdout",
    "get_edge_label_kfold",
    "get_edge_prediction_kfold",
    "get_undirected_louvain_community_detection",
    "get_directed_modularity_from_node_community_memberships",
    "get_undirected_modularity_from_node_community_memberships",
    "is_compatible",
    "has_same_adjacency_matrix",
    "get_laplacian_transformed_graph",
    "get_laplacian_coo_matrix_edges_number",
    "get_random_walk_normalized_laplacian_transformed_graph",
    "get_symmetric_normalized_laplacian_transformed_graph",
    "get_symmetric_normalized_transformed_graph",
    "get_memory_stats",
    "get_total_memory_used",
    "get_nodes_total_memory_requirement",
    "get_nodes_total_memory_requirement_human_readable",
    "get_edges_total_memory_requirement",
    "get_edges_total_memory_requirement_human_readable",
    "get_edge_weights_total_memory_requirements",
    "get_edge_weights_total_memory_requirements_human_readable",
    "get_node_types_total_memory_requirements",
    "get_node_types_total_memory_requirements_human_readable",
    "get_edge_types_total_memory_requirements",
    "get_edge_types_total_memory_requirements_human_readable",
    "get_transitive_closure",
    "get_all_shortest_paths",
    "get_weighted_all_shortest_paths",
    "filter_from_ids",
    "filter_from_names",
    "drop_unknown_node_types",
    "drop_unknown_edge_types",
    "drop_singleton_nodes",
    "drop_singleton_nodes_with_selfloops",
    "drop_disconnected_nodes",
    "drop_selfloops",
    "drop_parallel_edges",
];

pub const GRAPH_TERMS: &[&str] = &[
    "node_type_ids",
    "graph_name",
    "from_names",
    "topological",
    "walk",
    "to",
    "connected",
    "adar",
    "number",
    "diagonal",
    "component_ids",
    "mask",
    "community",
    "maximum",
    "matrix",
    "cumulative",
    "edge_type_ids",
    "selfloop",
    "transposed",
    "weights",
    "is",
    "by",
    "degrees",
    "subgraph",
    "same",
    "and",
    "urls",
    "multiple",
    "node_type_id",
    "per",
    "feature",
    "encodable",
    "lexicographic",
    "with",
    "sample",
    "okapi",
    "attachment",
    "edges",
    "coo",
    "enable",
    "weight",
    "sparse",
    "of",
    "unchecked",
    "nodes",
    "node_names",
    "coefficient",
    "reversed",
    "main",
    "textual",
    "strongly",
    "remappable",
    "be",
    "hashmap",
    "minimum",
    "neighbour",
    "centrality",
    "weighted",
    "used",
    "disconnected",
    "triads",
    "ontologies",
    "uniform",
    "methods",
    "edge_type_name",
    "path",
    "encode",
    "edge_id",
    "known",
    "naive",
    "decode",
    "laplacian",
    "undirected",
    "node_id",
    "unique",
    "chain",
    "multilabel",
    "edge",
    "get",
    "modularity",
    "node_types",
    "contain",
    "count",
    "not",
    "readable",
    "destination_names",
    "arborescence",
    "inplace",
    "features",
    "k",
    "circle",
    "arrowhead",
    "must",
    "lower",
    "bipartite",
    "homogeneous",
    "node_name",
    "mean",
    "order",
    "top",
    "subsampled",
    "approximated",
    "set",
    "diameter",
    "density",
    "median",
    "node_ids",
    "kruskal",
    "identity",
    "unknown",
    "destination",
    "edge_names",
    "bidiagonal",
    "replace",
    "sort",
    "edge_type_id",
    "new",
    "mininum",
    "adjacency",
    "preferential",
    "eigenvector",
    "max",
    "triangular",
    "paths",
    "report",
    "probabilities",
    "sampling",
    "have",
    "labels",
    "detection",
    "search",
    "node_type_names",
    "bfs",
    "stats",
    "harmonic",
    "jaccard",
    "metrics",
    "rate",
    "edge_ids",
    "parallel",
    "cover",
    "memberships",
    "from_ids",
    "weighting",
    "transitivity",
    "spanning",
    "has",
    "trap",
    "hot",
    "total",
    "stress",
    "negatives",
    "compatible",
    "requirements",
    "requirement",
    "betweenness",
    "vertex",
    "dot",
    "complete",
    "node",
    "representing",
    "closeness",
    "label",
    "complementary",
    "transitive",
    "upper",
    "eccentricity",
    "negative",
    "allocation",
    "dijkstra",
    "graphs",
    "edge_types",
    "louvain",
    "clique",
    "contains",
    "memory",
    "all",
    "symmetric",
    "one",
    "overlaps",
    "breadth",
    "oddities",
    "mode",
    "shortest",
    "singleton",
    "mapping",
    "source_names",
    "increasing",
    "counts",
    "first",
    "dense",
    "from",
    "binary",
    "comulative",
    "tree",
    "clustering",
    "kfold",
    "human",
    "average",
    "closure",
    "triangles",
    "propagation",
    "adamic",
    "directed",
    "minmax",
    "decreasing",
    "random",
    "csv",
    "barbell",
    "overlap",
    "holdout",
    "edge_type_names",
    "sorting",
    "remap",
    "remove",
    "index",
    "bm25",
    "multigraph",
    "normalized",
    "most",
    "graph",
    "central",
    "sorted",
    "filter",
    "singletons",
    "generate",
    "resource",
    "are",
    "get_name",
    "outbound",
    "transformed",
    "default",
    "drop",
    "selfloops",
    "constant",
    "add",
    "node_type_name",
    "components",
    "encoded",
    "degree",
    "star",
    "validate",
    "prediction",
    "indices",
    "source",
    "indegrees",
    "anti",
];

pub const GRAPH_TFIDF_FREQUENCIES: &[&[(&str, f64)]] = &[
    &[
        ("are", 3.2095419138451797),
        ("nodes", 1.481292910973371),
        ("remappable", 3.2095419138451797),
    ],
    &[
        ("node_ids", 0.6794034214683663),
        ("unchecked", 0.7655591993114581),
        ("from", 0.4272087119935153),
        ("remap", 1.7398419866455679),
    ],
    &[
        ("node_ids", 1.014615153739677),
        ("remap", 2.598264873244137),
        ("from", 0.6379897705864173),
    ],
    &[
        ("remap", 2.598264873244137),
        ("from", 0.6379897705864173),
        ("node_names", 1.348890740551726),
    ],
    &[
        ("remap", 2.598264873244137),
        ("from", 0.6379897705864173),
        ("graph", 2.12682127667469),
    ],
    &[
        ("all", 1.5495183660894714),
        ("set", 1.6650760165708902),
        ("edge_types", 0.9918987196305551),
        ("inplace", 1.3591947455333753),
    ],
    &[
        ("all", 2.3140372355418344),
        ("set", 2.486610024556645),
        ("edge_types", 1.481292910973371),
    ],
    &[
        ("node_types", 0.9702938033760223),
        ("all", 1.5495183660894714),
        ("inplace", 1.3591947455333753),
        ("set", 1.6650760165708902),
    ],
    &[
        ("node_types", 1.4490283171628935),
        ("set", 2.486610024556645),
        ("all", 2.3140372355418344),
    ],
    &[
        ("node_type_ids", 2.12682127667469),
        ("inplace", 2.0298095978395323),
        ("remove", 1.7823826979024435),
    ],
    &[
        ("inplace", 1.3591947455333753),
        ("node_types", 0.9702938033760223),
        ("remove", 1.1935135197395608),
        ("singleton", 1.0148339846689118),
    ],
    &[
        ("remove", 1.7823826979024435),
        ("inplace", 2.0298095978395323),
        ("edge_type_ids", 2.393659738503486),
    ],
    &[
        ("remove", 1.1935135197395608),
        ("edge_types", 0.9918987196305551),
        ("singleton", 1.0148339846689118),
        ("inplace", 1.3591947455333753),
    ],
    &[
        ("node_type_name", 2.2443954188550714),
        ("remove", 1.7823826979024435),
        ("inplace", 2.0298095978395323),
    ],
    &[
        ("node_type_id", 3.598335411599793),
        ("remove", 2.8576117759841533),
    ],
    &[
        ("singleton", 1.5155442360736466),
        ("node_types", 1.4490283171628935),
        ("remove", 1.7823826979024435),
    ],
    &[
        ("node_type_name", 3.598335411599793),
        ("remove", 2.8576117759841533),
    ],
    &[
        ("remove", 1.7823826979024435),
        ("edge_type_name", 2.0298095978395323),
        ("inplace", 2.0298095978395323),
    ],
    &[
        ("remove", 2.8576117759841533),
        ("edge_type_id", 2.812999291736001),
    ],
    &[
        ("edge_types", 1.481292910973371),
        ("remove", 1.7823826979024435),
        ("singleton", 1.5155442360736466),
    ],
    &[
        ("remove", 2.8576117759841533),
        ("edge_type_name", 3.2542998855509446),
    ],
    &[
        ("inplace", 2.0298095978395323),
        ("node_types", 1.4490283171628935),
        ("remove", 1.7823826979024435),
    ],
    &[
        ("node_types", 2.323160109067567),
        ("remove", 2.8576117759841533),
    ],
    &[
        ("remove", 1.7823826979024435),
        ("edge_types", 1.481292910973371),
        ("inplace", 2.0298095978395323),
    ],
    &[
        ("edge_types", 2.374888440659132),
        ("remove", 2.8576117759841533),
    ],
    &[
        ("weights", 1.424155402074),
        ("remove", 1.1935135197395608),
        ("inplace", 1.3591947455333753),
        ("edge", 0.8389813251504573),
    ],
    &[
        ("remove", 1.7823826979024435),
        ("weights", 2.12682127667469),
        ("edge", 1.2529274055795787),
    ],
    &[
        ("add", 5.145710166123201),
        ("selfloops", 3.2542998855509446),
    ],
    &[("overlaps", 8.803250046116208)],
    &[("contains", 8.023659988757329)],
    &[
        ("random", 2.0298095978395323),
        ("nodes", 1.481292910973371),
        ("get", 0.21612215427148931),
    ],
    &[
        ("get", 0.07632665073926659),
        ("search", 0.9176146525871081),
        ("first", 0.9176146525871081),
        ("breadth", 0.9176146525871081),
        ("random", 0.7168564868499797),
        ("nodes", 0.5231399207523589),
    ],
    &[
        ("uniform", 1.133495941297005),
        ("walk", 1.033116858428441),
        ("get", 0.07632665073926659),
        ("nodes", 0.5231399207523589),
        ("random", 1.3292333120870812),
    ],
    &[
        ("sampling", 2.1491634040506558),
        ("node", 0.8245103641128158),
        ("get", 0.14471904004780678),
        ("methods", 1.833476819479088),
    ],
    &[
        ("subsampled", 3.2095419138451797),
        ("get", 0.21612215427148931),
        ("nodes", 1.481292910973371),
    ],
    &[
        ("default", 3.2095419138451797),
        ("graph_name", 3.2095419138451797),
        ("has", 1.348890740551726),
    ],
    &[("nodes", 2.374888440659132), ("has", 2.162614162072378)],
    &[("has", 2.162614162072378), ("edges", 2.954145194114302)],
    &[
        ("nodes", 1.481292910973371),
        ("has", 1.348890740551726),
        ("trap", 2.486610024556645),
    ],
    &[("is", 3.1218997764463956), ("directed", 3.4098342251591442)],
    &[
        ("has", 1.348890740551726),
        ("weights", 2.12682127667469),
        ("edge", 1.2529274055795787),
    ],
    &[
        ("probabilities", 1.5255162287186315),
        ("weights", 1.0108920401242731),
        ("edge", 0.5955245770036008),
        ("representing", 1.5255162287186315),
        ("has", 0.6411365767193442),
    ],
    &[
        ("nodes", 0.9918987196305551),
        ("has", 0.9032399929569503),
        ("weighted", 0.9400680145942347),
        ("singleton", 1.0148339846689118),
    ],
    &[
        ("edge", 0.8389813251504573),
        ("has", 0.9032399929569503),
        ("constant", 2.1491634040506558),
        ("weights", 1.424155402074),
    ],
    &[
        ("has", 0.9032399929569503),
        ("edge", 0.8389813251504573),
        ("weights", 1.424155402074),
        ("negative", 2.1491634040506558),
    ],
    &[
        ("edge_types", 2.374888440659132),
        ("has", 2.162614162072378),
    ],
    &[
        ("selfloops", 3.2542998855509446),
        ("has", 2.162614162072378),
    ],
    &[
        ("has", 1.348890740551726),
        ("disconnected", 2.598264873244137),
        ("nodes", 1.481292910973371),
    ],
    &[
        ("has", 1.348890740551726),
        ("singleton", 1.5155442360736466),
        ("nodes", 1.481292910973371),
    ],
    &[
        ("nodes", 0.7040681928557436),
        ("with", 0.8213562238441001),
        ("singleton", 0.7203480713238173),
        ("has", 0.6411365767193442),
        ("selfloops", 0.9647817557251608),
    ],
    &[
        ("is", 3.1218997764463956),
        ("connected", 3.4098342251591442),
    ],
    &[
        ("node_types", 2.323160109067567),
        ("has", 2.162614162072378),
    ],
    &[
        ("node_types", 1.4490283171628935),
        ("has", 1.348890740551726),
        ("multilabel", 2.925314276142877),
    ],
    &[
        ("unknown", 1.8116741102988692),
        ("node_types", 1.4490283171628935),
        ("has", 1.348890740551726),
    ],
    &[
        ("node_types", 1.4490283171628935),
        ("has", 1.348890740551726),
        ("known", 1.8753329839556023),
    ],
    &[
        ("unknown", 1.8116741102988692),
        ("edge_types", 1.481292910973371),
        ("has", 1.348890740551726),
    ],
    &[
        ("has", 1.348890740551726),
        ("known", 1.8753329839556023),
        ("edge_types", 1.481292910973371),
    ],
    &[
        ("node_types", 1.4490283171628935),
        ("has", 1.348890740551726),
        ("homogeneous", 2.925314276142877),
    ],
    &[
        ("has", 1.348890740551726),
        ("homogeneous", 2.925314276142877),
        ("edge_types", 1.481292910973371),
    ],
    &[
        ("node_types", 1.4490283171628935),
        ("singleton", 1.5155442360736466),
        ("has", 1.348890740551726),
    ],
    &[
        ("oddities", 2.7380983172757327),
        ("node", 1.2313165983713443),
        ("has", 1.348890740551726),
    ],
    &[
        ("has", 1.348890740551726),
        ("node_types", 1.4490283171628935),
        ("oddities", 2.7380983172757327),
    ],
    &[
        ("singleton", 1.5155442360736466),
        ("edge_types", 1.481292910973371),
        ("has", 1.348890740551726),
    ],
    &[
        ("has", 1.348890740551726),
        ("edge_types", 1.481292910973371),
        ("oddities", 2.7380983172757327),
    ],
    &[
        ("is", 3.1218997764463956),
        ("multigraph", 4.389866443641716),
    ],
    &[
        ("by", 0.4989100606092201),
        ("has", 0.29082296118121864),
        ("nodes", 0.3193690769719561),
        ("sorted", 0.5903382955314346),
        ("degree", 0.3842845076912147),
        ("decreasing", 0.6307023501589091),
        ("node", 0.2654737915566615),
        ("outbound", 0.5601899854846155),
    ],
    &[
        ("by", 0.8172355697185438),
        ("nodes", 0.5231399207523589),
        ("order", 1.033116858428441),
        ("sorted", 0.9669988468185846),
        ("lexicographic", 1.033116858428441),
        ("has", 0.47638018779967434),
    ],
    &[
        ("matrix", 2.393659738503486),
        ("identity", 2.925314276142877),
        ("contains", 2.925314276142877),
    ],
    &[
        ("has", 0.29082296118121864),
        ("sorted", 0.5903382955314346),
        ("node", 0.2654737915566615),
        ("outbound", 0.5601899854846155),
        ("by", 0.4989100606092201),
        ("degree", 0.3842845076912147),
        ("increasing", 0.6307023501589091),
        ("nodes", 0.3193690769719561),
    ],
    &[
        ("components", 1.7398419866455679),
        ("get", 0.14471904004780678),
        ("connected", 1.424155402074),
        ("number", 0.9702938033760223),
    ],
    &[
        ("get", 0.14471904004780678),
        ("connected", 1.424155402074),
        ("nodes", 0.9918987196305551),
        ("number", 0.9702938033760223),
    ],
    &[
        ("singleton", 0.5352362693987905),
        ("nodes", 0.5231399207523589),
        ("number", 0.5117452148680048),
        ("selfloops", 0.7168564868499797),
        ("get", 0.07632665073926659),
        ("with", 0.610287801964798),
    ],
    &[
        ("singleton", 1.0148339846689118),
        ("get", 0.14471904004780678),
        ("number", 0.9702938033760223),
        ("nodes", 0.9918987196305551),
    ],
    &[
        ("disconnected", 1.7398419866455679),
        ("number", 0.9702938033760223),
        ("nodes", 0.9918987196305551),
        ("get", 0.14471904004780678),
    ],
    &[
        ("singleton", 1.5155442360736466),
        ("node_ids", 1.014615153739677),
        ("get", 0.21612215427148931),
    ],
    &[
        ("singleton", 1.5155442360736466),
        ("get", 0.21612215427148931),
        ("node_names", 1.348890740551726),
    ],
    &[
        ("get", 0.10272427111936212),
        ("selfloops", 0.9647817557251608),
        ("node_ids", 0.4822532076172131),
        ("singleton", 0.7203480713238173),
        ("with", 0.8213562238441001),
    ],
    &[
        ("selfloops", 0.9647817557251608),
        ("node_names", 0.6411365767193442),
        ("with", 0.8213562238441001),
        ("get", 0.10272427111936212),
        ("singleton", 0.7203480713238173),
    ],
    &[("get", 0.34649865812996966), ("density", 5.145710166123201)],
    &[
        ("nodes", 0.9918987196305551),
        ("get", 0.14471904004780678),
        ("trap", 1.6650760165708902),
        ("rate", 1.6028349371446486),
    ],
    &[
        ("node", 0.8245103641128158),
        ("mean", 1.9588397834945597),
        ("degrees", 1.5028850309181347),
        ("get", 0.14471904004780678),
    ],
    &[
        ("node", 0.5852528192273099),
        ("mean", 1.390420976622124),
        ("weighted", 0.6672777926796519),
        ("get", 0.10272427111936212),
        ("degrees", 1.0667757976163075),
    ],
    &[
        ("get", 0.14471904004780678),
        ("number", 0.9702938033760223),
        ("undirected", 1.6650760165708902),
        ("edges", 1.2338317815179038),
    ],
    &[
        ("undirected", 1.1819019812740101),
        ("edges", 0.8757967880277655),
        ("number", 0.6887326207422982),
        ("unique", 1.0108920401242731),
        ("get", 0.10272427111936212),
    ],
    &[
        ("edges", 1.8425936389723871),
        ("number", 1.4490283171628935),
        ("get", 0.21612215427148931),
    ],
    &[
        ("get", 0.14471904004780678),
        ("number", 0.9702938033760223),
        ("unique", 1.424155402074),
        ("edges", 1.2338317815179038),
    ],
    &[
        ("get", 0.14471904004780678),
        ("degrees", 1.5028850309181347),
        ("node", 0.8245103641128158),
        ("median", 1.9588397834945597),
    ],
    &[
        ("median", 1.390420976622124),
        ("get", 0.10272427111936212),
        ("node", 0.5852528192273099),
        ("weighted", 0.6672777926796519),
        ("degrees", 1.0667757976163075),
    ],
    &[
        ("maximum", 1.5028850309181347),
        ("degree", 1.1935135197395608),
        ("node", 0.8245103641128158),
        ("get", 0.14471904004780678),
    ],
    &[
        ("get", 0.10272427111936212),
        ("node_id", 0.6173484597137204),
        ("most", 1.390420976622124),
        ("central", 1.1819019812740101),
        ("unchecked", 0.543408184037245),
    ],
    &[
        ("get", 0.14471904004780678),
        ("node_id", 0.8697270420244625),
        ("most", 1.9588397834945597),
        ("central", 1.6650760165708902),
    ],
    &[
        ("degree", 1.1935135197395608),
        ("node", 0.8245103641128158),
        ("minimum", 1.5495183660894714),
        ("get", 0.14471904004780678),
    ],
    &[
        ("mode", 2.1491634040506558),
        ("get", 0.14471904004780678),
        ("node", 0.8245103641128158),
        ("degrees", 1.5028850309181347),
    ],
    &[
        ("selfloop", 1.9588397834945597),
        ("rate", 1.6028349371446486),
        ("nodes", 0.9918987196305551),
        ("get", 0.14471904004780678),
    ],
    &[("get_name", 8.803250046116208)],
    &[
        ("get", 0.14471904004780678),
        ("number", 0.9702938033760223),
        ("trap", 1.6650760165708902),
        ("nodes", 0.9918987196305551),
    ],
    &[
        ("get", 0.21612215427148931),
        ("source", 2.12682127667469),
        ("node_ids", 1.014615153739677),
    ],
    &[
        ("node_ids", 0.6794034214683663),
        ("get", 0.14471904004780678),
        ("source", 1.424155402074),
        ("directed", 1.424155402074),
    ],
    &[
        ("source_names", 5.145710166123201),
        ("get", 0.34649865812996966),
    ],
    &[
        ("get", 0.21612215427148931),
        ("destination", 2.393659738503486),
        ("node_ids", 1.014615153739677),
    ],
    &[
        ("directed", 1.424155402074),
        ("get", 0.14471904004780678),
        ("node_ids", 0.6794034214683663),
        ("destination", 1.6028349371446486),
    ],
    &[
        ("get", 0.34649865812996966),
        ("destination_names", 5.145710166123201),
    ],
    &[
        ("get", 0.34649865812996966),
        ("node_names", 2.162614162072378),
    ],
    &[
        ("node", 1.2313165983713443),
        ("get", 0.21612215427148931),
        ("urls", 3.2095419138451797),
    ],
    &[
        ("node", 1.2313165983713443),
        ("get", 0.21612215427148931),
        ("ontologies", 3.2095419138451797),
    ],
    &[
        ("node_ids", 1.626685568049184),
        ("get", 0.34649865812996966),
    ],
    &[
        ("edge_type_ids", 3.8376439944667244),
        ("get", 0.34649865812996966),
    ],
    &[
        ("unique", 2.12682127667469),
        ("get", 0.21612215427148931),
        ("edge_type_ids", 2.393659738503486),
    ],
    &[
        ("get", 0.34649865812996966),
        ("edge_type_names", 3.9866668907948735),
    ],
    &[
        ("unique", 2.12682127667469),
        ("get", 0.21612215427148931),
        ("edge_type_names", 2.486610024556645),
    ],
    &[
        ("edge", 1.2529274055795787),
        ("weights", 2.12682127667469),
        ("get", 0.21612215427148931),
    ],
    &[
        ("get", 0.14471904004780678),
        ("indegrees", 1.9588397834945597),
        ("weighted", 0.9400680145942347),
        ("node", 0.8245103641128158),
    ],
    &[
        ("get", 0.34649865812996966),
        ("node_type_ids", 3.4098342251591442),
    ],
    &[
        ("get", 0.14471904004780678),
        ("node_types", 0.9702938033760223),
        ("mask", 1.6028349371446486),
        ("known", 1.2557545991658023),
    ],
    &[
        ("get", 0.14471904004780678),
        ("unknown", 1.2131275435676325),
        ("mask", 1.6028349371446486),
        ("node_types", 0.9702938033760223),
    ],
    &[
        ("hot", 1.2349722599181758),
        ("one", 1.2349722599181758),
        ("node_types", 0.6887326207422982),
        ("encoded", 1.2349722599181758),
        ("get", 0.10272427111936212),
    ],
    &[
        ("hot", 0.9176146525871081),
        ("get", 0.07632665073926659),
        ("encoded", 0.9176146525871081),
        ("node_types", 0.5117452148680048),
        ("one", 0.9176146525871081),
        ("known", 0.6623008463371057),
    ],
    &[
        ("one", 1.2349722599181758),
        ("edge_types", 0.7040681928557436),
        ("get", 0.10272427111936212),
        ("hot", 1.2349722599181758),
        ("encoded", 1.2349722599181758),
    ],
    &[
        ("known", 0.6623008463371057),
        ("one", 0.9176146525871081),
        ("edge_types", 0.5231399207523589),
        ("get", 0.07632665073926659),
        ("encoded", 0.9176146525871081),
        ("hot", 0.9176146525871081),
    ],
    &[
        ("get", 0.34649865812996966),
        ("node_type_names", 3.4098342251591442),
    ],
    &[
        ("get", 0.21612215427148931),
        ("unique", 2.12682127667469),
        ("node_type_ids", 2.12682127667469),
    ],
    &[
        ("unique", 2.12682127667469),
        ("node_type_names", 2.12682127667469),
        ("get", 0.21612215427148931),
    ],
    &[
        ("get", 0.10272427111936212),
        ("unique", 1.0108920401242731),
        ("number", 0.6887326207422982),
        ("directed", 1.0108920401242731),
        ("edges", 0.8757967880277655),
    ],
    &[
        ("mapping", 2.925314276142877),
        ("get", 0.21612215427148931),
        ("nodes", 1.481292910973371),
    ],
    &[
        ("get", 0.21612215427148931),
        ("node_ids", 1.014615153739677),
        ("edge", 1.2529274055795787),
    ],
    &[
        ("edge", 0.8389813251504573),
        ("get", 0.14471904004780678),
        ("directed", 1.424155402074),
        ("node_ids", 0.6794034214683663),
    ],
    &[
        ("node_names", 1.348890740551726),
        ("get", 0.21612215427148931),
        ("edge", 1.2529274055795787),
    ],
    &[
        ("get", 0.14471904004780678),
        ("node_names", 0.9032399929569503),
        ("edge", 0.8389813251504573),
        ("directed", 1.424155402074),
    ],
    &[
        ("unknown", 1.2131275435676325),
        ("get", 0.14471904004780678),
        ("node_types", 0.9702938033760223),
        ("number", 0.9702938033760223),
    ],
    &[
        ("known", 1.2557545991658023),
        ("get", 0.14471904004780678),
        ("node_types", 0.9702938033760223),
        ("number", 0.9702938033760223),
    ],
    &[
        ("node_types", 0.9702938033760223),
        ("get", 0.14471904004780678),
        ("unknown", 1.2131275435676325),
        ("rate", 1.6028349371446486),
    ],
    &[
        ("get", 0.14471904004780678),
        ("rate", 1.6028349371446486),
        ("node_types", 0.9702938033760223),
        ("known", 1.2557545991658023),
    ],
    &[
        ("get", 0.14471904004780678),
        ("number", 0.9702938033760223),
        ("minimum", 1.5495183660894714),
        ("node_types", 0.9702938033760223),
    ],
    &[
        ("get", 0.14471904004780678),
        ("node_types", 0.9702938033760223),
        ("number", 0.9702938033760223),
        ("maximum", 1.5028850309181347),
    ],
    &[
        ("maximum", 1.5028850309181347),
        ("get", 0.14471904004780678),
        ("count", 1.6028349371446486),
        ("multilabel", 1.9588397834945597),
    ],
    &[
        ("node_types", 0.9702938033760223),
        ("get", 0.14471904004780678),
        ("number", 0.9702938033760223),
        ("singleton", 1.0148339846689118),
    ],
    &[
        ("node_type_ids", 2.12682127667469),
        ("get", 0.21612215427148931),
        ("singleton", 1.5155442360736466),
    ],
    &[
        ("singleton", 1.5155442360736466),
        ("node_type_names", 2.12682127667469),
        ("get", 0.21612215427148931),
    ],
    &[
        ("get", 0.14471904004780678),
        ("unknown", 1.2131275435676325),
        ("edge_types", 0.9918987196305551),
        ("number", 0.9702938033760223),
    ],
    &[
        ("edge_types", 0.7040681928557436),
        ("edge_ids", 1.0373605636974754),
        ("unknown", 0.8611005341566724),
        ("with", 0.8213562238441001),
        ("get", 0.10272427111936212),
    ],
    &[
        ("with", 0.8213562238441001),
        ("edge_ids", 1.0373605636974754),
        ("get", 0.10272427111936212),
        ("edge_types", 0.7040681928557436),
        ("known", 0.8913580124735546),
    ],
    &[
        ("edge", 0.4424893542713664),
        ("with", 0.610287801964798),
        ("node_ids", 0.3583259510590198),
        ("edge_types", 0.5231399207523589),
        ("get", 0.07632665073926659),
        ("unknown", 0.6398187984765752),
    ],
    &[
        ("edge", 0.4424893542713664),
        ("with", 0.610287801964798),
        ("edge_types", 0.5231399207523589),
        ("known", 0.6623008463371057),
        ("get", 0.07632665073926659),
        ("node_ids", 0.3583259510590198),
    ],
    &[
        ("edge", 0.4424893542713664),
        ("node_names", 0.47638018779967434),
        ("edge_types", 0.5231399207523589),
        ("with", 0.610287801964798),
        ("get", 0.07632665073926659),
        ("unknown", 0.6398187984765752),
    ],
    &[
        ("edge_types", 0.5231399207523589),
        ("node_names", 0.47638018779967434),
        ("get", 0.07632665073926659),
        ("with", 0.610287801964798),
        ("known", 0.6623008463371057),
        ("edge", 0.4424893542713664),
    ],
    &[
        ("edge_ids", 0.7707843197448777),
        ("edge_types", 0.5231399207523589),
        ("get", 0.07632665073926659),
        ("unknown", 0.6398187984765752),
        ("mask", 0.8453554031295414),
        ("with", 0.610287801964798),
    ],
    &[
        ("get", 0.07632665073926659),
        ("edge_ids", 0.7707843197448777),
        ("mask", 0.8453554031295414),
        ("edge_types", 0.5231399207523589),
        ("known", 0.6623008463371057),
        ("with", 0.610287801964798),
    ],
    &[
        ("unknown", 0.8611005341566724),
        ("with", 0.8213562238441001),
        ("node_ids", 0.4822532076172131),
        ("node_types", 0.6887326207422982),
        ("get", 0.10272427111936212),
    ],
    &[
        ("node_types", 0.6887326207422982),
        ("known", 0.8913580124735546),
        ("with", 0.8213562238441001),
        ("node_ids", 0.4822532076172131),
        ("get", 0.10272427111936212),
    ],
    &[
        ("unknown", 0.8611005341566724),
        ("get", 0.10272427111936212),
        ("node_types", 0.6887326207422982),
        ("node_names", 0.6411365767193442),
        ("with", 0.8213562238441001),
    ],
    &[
        ("node_types", 0.6887326207422982),
        ("get", 0.10272427111936212),
        ("node_names", 0.6411365767193442),
        ("with", 0.8213562238441001),
        ("known", 0.8913580124735546),
    ],
    &[
        ("mask", 0.8453554031295414),
        ("with", 0.610287801964798),
        ("unknown", 0.6398187984765752),
        ("node_types", 0.5117452148680048),
        ("node_ids", 0.3583259510590198),
        ("get", 0.07632665073926659),
    ],
    &[
        ("node_types", 0.5117452148680048),
        ("known", 0.6623008463371057),
        ("node_ids", 0.3583259510590198),
        ("with", 0.610287801964798),
        ("mask", 0.8453554031295414),
        ("get", 0.07632665073926659),
    ],
    &[
        ("known", 1.2557545991658023),
        ("number", 0.9702938033760223),
        ("edge_types", 0.9918987196305551),
        ("get", 0.14471904004780678),
    ],
    &[
        ("get", 0.14471904004780678),
        ("rate", 1.6028349371446486),
        ("edge_types", 0.9918987196305551),
        ("unknown", 1.2131275435676325),
    ],
    &[
        ("known", 1.2557545991658023),
        ("rate", 1.6028349371446486),
        ("get", 0.14471904004780678),
        ("edge_types", 0.9918987196305551),
    ],
    &[
        ("get", 0.14471904004780678),
        ("minimum", 1.5495183660894714),
        ("edge_types", 0.9918987196305551),
        ("number", 0.9702938033760223),
    ],
    &[
        ("edge_types", 0.9918987196305551),
        ("get", 0.14471904004780678),
        ("number", 0.9702938033760223),
        ("singleton", 1.0148339846689118),
    ],
    &[
        ("singleton", 1.5155442360736466),
        ("get", 0.21612215427148931),
        ("edge_type_ids", 2.393659738503486),
    ],
    &[
        ("singleton", 1.5155442360736466),
        ("edge_type_names", 2.486610024556645),
        ("get", 0.21612215427148931),
    ],
    &[
        ("get", 0.21612215427148931),
        ("nodes", 1.481292910973371),
        ("number", 1.4490283171628935),
    ],
    &[
        ("node", 0.8245103641128158),
        ("component_ids", 2.1491634040506558),
        ("connected", 1.424155402074),
        ("get", 0.14471904004780678),
    ],
    &[
        ("number", 0.9702938033760223),
        ("directed", 1.424155402074),
        ("edges", 1.2338317815179038),
        ("get", 0.14471904004780678),
    ],
    &[
        ("get", 0.21612215427148931),
        ("edge_types", 1.481292910973371),
        ("number", 1.4490283171628935),
    ],
    &[
        ("get", 0.21612215427148931),
        ("node_types", 1.4490283171628935),
        ("number", 1.4490283171628935),
    ],
    &[
        ("degrees", 2.2443954188550714),
        ("get", 0.21612215427148931),
        ("node", 1.2313165983713443),
    ],
    &[
        ("get", 0.21612215427148931),
        ("indegrees", 2.925314276142877),
        ("node", 1.2313165983713443),
    ],
    &[
        ("get", 0.14471904004780678),
        ("degrees", 1.5028850309181347),
        ("weighted", 0.9400680145942347),
        ("node", 0.8245103641128158),
    ],
    &[
        ("singletons", 2.1491634040506558),
        ("node_ids", 0.6794034214683663),
        ("get", 0.14471904004780678),
        ("not", 1.6650760165708902),
    ],
    &[
        ("dense", 1.833476819479088),
        ("get", 0.14471904004780678),
        ("mapping", 1.9588397834945597),
        ("nodes", 0.9918987196305551),
    ],
    &[
        ("number", 0.9702938033760223),
        ("get", 0.14471904004780678),
        ("edges", 1.2338317815179038),
        ("parallel", 1.9588397834945597),
    ],
    &[
        ("get", 0.14471904004780678),
        ("cumulative", 2.1491634040506558),
        ("node", 0.8245103641128158),
        ("degrees", 1.5028850309181347),
    ],
    &[
        ("number", 0.6887326207422982),
        ("unique", 1.0108920401242731),
        ("source", 1.0108920401242731),
        ("nodes", 0.7040681928557436),
        ("get", 0.10272427111936212),
    ],
    &[
        ("edge_type_id", 1.1748806167165462),
        ("counts", 1.7398419866455679),
        ("get", 0.14471904004780678),
        ("hashmap", 1.7398419866455679),
    ],
    &[
        ("edge_type_names", 1.6650760165708902),
        ("hashmap", 1.7398419866455679),
        ("get", 0.14471904004780678),
        ("counts", 1.7398419866455679),
    ],
    &[
        ("node_type_id", 1.5028850309181347),
        ("get", 0.14471904004780678),
        ("counts", 1.7398419866455679),
        ("hashmap", 1.7398419866455679),
    ],
    &[
        ("hashmap", 1.7398419866455679),
        ("get", 0.14471904004780678),
        ("counts", 1.7398419866455679),
        ("node_type_names", 1.424155402074),
    ],
    &[
        ("weights", 1.424155402074),
        ("total", 1.3591947455333753),
        ("get", 0.14471904004780678),
        ("edge", 0.8389813251504573),
    ],
    &[
        ("edge", 0.8389813251504573),
        ("get", 0.14471904004780678),
        ("mininum", 2.1491634040506558),
        ("weight", 1.390261099613418),
    ],
    &[
        ("weight", 1.390261099613418),
        ("get", 0.14471904004780678),
        ("maximum", 1.5028850309181347),
        ("edge", 0.8389813251504573),
    ],
    &[
        ("unchecked", 0.543408184037245),
        ("get", 0.10272427111936212),
        ("maximum", 1.0667757976163075),
        ("degree", 0.8471781345830511),
        ("node", 0.5852528192273099),
    ],
    &[
        ("unchecked", 0.543408184037245),
        ("degree", 0.8471781345830511),
        ("node", 0.5852528192273099),
        ("minimum", 1.0998770078216682),
        ("get", 0.10272427111936212),
    ],
    &[
        ("get", 0.10272427111936212),
        ("weighted", 0.6672777926796519),
        ("maximum", 1.0667757976163075),
        ("degree", 0.8471781345830511),
        ("node", 0.5852528192273099),
    ],
    &[
        ("get", 0.10272427111936212),
        ("weighted", 0.6672777926796519),
        ("node", 0.5852528192273099),
        ("minimum", 1.0998770078216682),
        ("degree", 0.8471781345830511),
    ],
    &[
        ("weighted", 0.6672777926796519),
        ("nodes", 0.7040681928557436),
        ("singleton", 0.7203480713238173),
        ("get", 0.10272427111936212),
        ("number", 0.6887326207422982),
    ],
    &[
        ("get", 0.21612215427148931),
        ("number", 1.4490283171628935),
        ("selfloops", 2.0298095978395323),
    ],
    &[
        ("selfloops", 1.3591947455333753),
        ("number", 0.9702938033760223),
        ("unique", 1.424155402074),
        ("get", 0.14471904004780678),
    ],
    &[
        ("unchecked", 0.543408184037245),
        ("connected", 1.0108920401242731),
        ("node_id", 0.6173484597137204),
        ("from", 0.30324070378630485),
        ("is", 0.9255299306898463),
    ],
    &[
        ("from", 0.22531527394273332),
        ("unchecked", 0.40376559716520727),
        ("disconnected", 0.9176146525871081),
        ("node_id", 0.45870503392758394),
        ("is", 0.6876914189677453),
        ("node", 0.4348571865302264),
    ],
    &[
        ("node_id", 0.6173484597137204),
        ("from", 0.30324070378630485),
        ("unchecked", 0.543408184037245),
        ("singleton", 0.7203480713238173),
        ("is", 0.9255299306898463),
    ],
    &[
        ("from", 0.4272087119935153),
        ("node_id", 0.8697270420244625),
        ("singleton", 1.0148339846689118),
        ("is", 1.3038963591117803),
    ],
    &[
        ("with", 0.47005269386267773),
        ("selfloops", 0.5521334715718345),
        ("is", 0.5296701048117646),
        ("singleton", 0.4122468931443976),
        ("unchecked", 0.31098623637168116),
        ("from", 0.1735411573756114),
        ("node_id", 0.3533014033573437),
    ],
    &[
        ("node_id", 0.45870503392758394),
        ("is", 0.6876914189677453),
        ("singleton", 0.5352362693987905),
        ("with", 0.610287801964798),
        ("from", 0.22531527394273332),
        ("selfloops", 0.7168564868499797),
    ],
    &[
        ("from", 0.30324070378630485),
        ("node_name", 0.8213562238441001),
        ("singleton", 0.7203480713238173),
        ("unchecked", 0.543408184037245),
        ("is", 0.9255299306898463),
    ],
    &[
        ("node_name", 1.1571353386765968),
        ("is", 1.3038963591117803),
        ("from", 0.4272087119935153),
        ("singleton", 1.0148339846689118),
    ],
    &[("has", 2.162614162072378), ("node_name", 2.770512034862583)],
    &[
        ("has", 2.162614162072378),
        ("node_type_id", 3.598335411599793),
    ],
    &[
        ("has", 2.162614162072378),
        ("node_type_name", 3.598335411599793),
    ],
    &[
        ("has", 2.162614162072378),
        ("edge_type_id", 2.812999291736001),
    ],
    &[
        ("edge_type_name", 3.2542998855509446),
        ("has", 2.162614162072378),
    ],
    &[
        ("node_ids", 0.6794034214683663),
        ("edge", 0.8389813251504573),
        ("from", 0.4272087119935153),
        ("has", 0.9032399929569503),
    ],
    &[
        ("has", 0.9032399929569503),
        ("selfloop", 1.9588397834945597),
        ("node_id", 0.8697270420244625),
        ("from", 0.4272087119935153),
    ],
    &[
        ("and", 0.6876914189677453),
        ("node_ids", 0.3583259510590198),
        ("has", 0.47638018779967434),
        ("edge", 0.4424893542713664),
        ("edge_type_id", 0.6196468858751043),
        ("from", 0.22531527394273332),
    ],
    &[
        ("unchecked", 0.40376559716520727),
        ("is", 0.6876914189677453),
        ("trap", 0.8781821350470027),
        ("node_id", 0.45870503392758394),
        ("node", 0.4348571865302264),
        ("from", 0.22531527394273332),
    ],
    &[
        ("node_id", 0.6173484597137204),
        ("from", 0.30324070378630485),
        ("is", 0.9255299306898463),
        ("node", 0.5852528192273099),
        ("trap", 1.1819019812740101),
    ],
    &[
        ("node_type_name", 1.5028850309181347),
        ("node_name", 1.1571353386765968),
        ("has", 0.9032399929569503),
        ("and", 1.3038963591117803),
    ],
    &[
        ("edge", 0.8389813251504573),
        ("node_names", 0.9032399929569503),
        ("from", 0.4272087119935153),
        ("has", 0.9032399929569503),
    ],
    &[
        ("node_names", 0.47638018779967434),
        ("has", 0.47638018779967434),
        ("edge", 0.4424893542713664),
        ("and", 0.6876914189677453),
        ("from", 0.22531527394273332),
        ("edge_type_name", 0.7168564868499797),
    ],
    &[
        ("node_id", 2.0823745990940266),
        ("validate", 3.598335411599793),
    ],
    &[
        ("validate", 3.598335411599793),
        ("node_ids", 1.626685568049184),
    ],
    &[
        ("edge_id", 2.519082833893534),
        ("validate", 3.598335411599793),
    ],
    &[
        ("validate", 3.598335411599793),
        ("edge_ids", 3.499115052376106),
    ],
    &[
        ("node_types", 0.6887326207422982),
        ("contain", 1.2349722599181758),
        ("unknown", 0.8611005341566724),
        ("not", 1.1819019812740101),
        ("must", 1.0108920401242731),
    ],
    &[
        ("not", 1.1819019812740101),
        ("contain", 1.2349722599181758),
        ("edge_types", 0.7040681928557436),
        ("unknown", 0.8611005341566724),
        ("must", 1.0108920401242731),
    ],
    &[
        ("node_type_id", 3.598335411599793),
        ("validate", 3.598335411599793),
    ],
    &[
        ("validate", 3.598335411599793),
        ("node_type_ids", 3.4098342251591442),
    ],
    &[
        ("validate", 3.598335411599793),
        ("edge_type_id", 2.812999291736001),
    ],
    &[
        ("validate", 3.598335411599793),
        ("edge_type_ids", 3.8376439944667244),
    ],
    &[
        ("must", 2.12682127667469),
        ("be", 2.598264873244137),
        ("undirected", 2.486610024556645),
    ],
    &[
        ("be", 2.598264873244137),
        ("multigraph", 2.7380983172757327),
        ("must", 2.12682127667469),
    ],
    &[
        ("multigraph", 1.833476819479088),
        ("be", 1.7398419866455679),
        ("must", 1.424155402074),
        ("not", 1.6650760165708902),
    ],
    &[
        ("must", 1.424155402074),
        ("matrix", 1.6028349371446486),
        ("contain", 1.7398419866455679),
        ("identity", 1.9588397834945597),
    ],
    &[
        ("singleton", 0.5352362693987905),
        ("must", 0.7511175581086876),
        ("contain", 0.9176146525871081),
        ("nodes", 0.5231399207523589),
        ("weighted", 0.49580375185868525),
        ("not", 0.8781821350470027),
    ],
    &[
        ("must", 2.12682127667469),
        ("have", 2.925314276142877),
        ("edges", 1.8425936389723871),
    ],
    &[
        ("must", 2.12682127667469),
        ("nodes", 1.481292910973371),
        ("have", 2.925314276142877),
    ],
    &[
        ("be", 2.598264873244137),
        ("connected", 2.12682127667469),
        ("must", 2.12682127667469),
    ],
    &[("replace", 8.803250046116208)],
    &[("to", 3.328681597854794), ("dot", 5.145710166123201)],
    &[
        ("remove", 2.8576117759841533),
        ("components", 4.16567794764063),
    ],
    &[("enable", 8.803250046116208)],
    &[
        ("spanning", 1.833476819479088),
        ("random", 1.3591947455333753),
        ("kruskal", 1.9588397834945597),
        ("arborescence", 1.9588397834945597),
    ],
    &[
        ("spanning", 2.7380983172757327),
        ("arborescence", 2.925314276142877),
        ("kruskal", 2.925314276142877),
    ],
    &[
        ("components", 4.16567794764063),
        ("connected", 3.4098342251591442),
    ],
    &[("report", 6.820365772112874)],
    &[
        ("overlap", 3.2095419138451797),
        ("textual", 2.925314276142877),
        ("report", 2.486610024556645),
    ],
    &[
        ("report", 1.1819019812740101),
        ("from", 0.30324070378630485),
        ("node_id", 0.6173484597137204),
        ("node", 0.5852528192273099),
        ("get", 0.10272427111936212),
    ],
    &[
        ("from", 0.30324070378630485),
        ("node_name", 0.8213562238441001),
        ("get", 0.10272427111936212),
        ("node", 0.5852528192273099),
        ("report", 1.1819019812740101),
    ],
    &[
        ("report", 3.9866668907948735),
        ("textual", 4.690021135078359),
    ],
    &[
        ("dense", 1.3014360089247288),
        ("get", 0.10272427111936212),
        ("adjacency", 1.3014360089247288),
        ("matrix", 1.1377221033835068),
        ("binary", 1.5255162287186315),
    ],
    &[
        ("weighted", 0.6672777926796519),
        ("dense", 1.3014360089247288),
        ("matrix", 1.1377221033835068),
        ("get", 0.10272427111936212),
        ("adjacency", 1.3014360089247288),
    ],
    &[
        ("node", 0.4348571865302264),
        ("edges", 0.6507384752401234),
        ("new", 1.133495941297005),
        ("generate", 0.8172355697185438),
        ("features", 1.133495941297005),
        ("from", 0.22531527394273332),
    ],
    &[
        ("preferential", 1.0108920401242731),
        ("attachment", 1.0108920401242731),
        ("minimum", 1.0998770078216682),
        ("unchecked", 0.543408184037245),
        ("get", 0.10272427111936212),
    ],
    &[
        ("maximum", 1.0667757976163075),
        ("preferential", 1.0108920401242731),
        ("attachment", 1.0108920401242731),
        ("unchecked", 0.543408184037245),
        ("get", 0.10272427111936212),
    ],
    &[
        ("minimum", 0.8172355697185438),
        ("attachment", 0.7511175581086876),
        ("weighted", 0.49580375185868525),
        ("get", 0.07632665073926659),
        ("unchecked", 0.40376559716520727),
        ("preferential", 0.7511175581086876),
    ],
    &[
        ("attachment", 0.7511175581086876),
        ("get", 0.07632665073926659),
        ("maximum", 0.7926405593781355),
        ("unchecked", 0.40376559716520727),
        ("preferential", 0.7511175581086876),
        ("weighted", 0.49580375185868525),
    ],
    &[
        ("get", 0.07632665073926659),
        ("attachment", 0.7511175581086876),
        ("from", 0.22531527394273332),
        ("preferential", 0.7511175581086876),
        ("unchecked", 0.40376559716520727),
        ("node_ids", 0.3583259510590198),
    ],
    &[
        ("get", 0.10272427111936212),
        ("attachment", 1.0108920401242731),
        ("node_ids", 0.4822532076172131),
        ("from", 0.30324070378630485),
        ("preferential", 1.0108920401242731),
    ],
    &[
        ("preferential", 1.0108920401242731),
        ("node_names", 0.6411365767193442),
        ("attachment", 1.0108920401242731),
        ("get", 0.10272427111936212),
        ("from", 0.30324070378630485),
    ],
    &[
        ("unchecked", 0.31098623637168116),
        ("preferential", 0.5785218555243384),
        ("node_ids", 0.27598794869230175),
        ("attachment", 0.5785218555243384),
        ("from", 0.1735411573756114),
        ("weighted", 0.3818753847579611),
        ("get", 0.05878791559982294),
    ],
    &[
        ("from", 0.22531527394273332),
        ("weighted", 0.49580375185868525),
        ("node_ids", 0.3583259510590198),
        ("preferential", 0.7511175581086876),
        ("get", 0.07632665073926659),
        ("attachment", 0.7511175581086876),
    ],
    &[
        ("attachment", 0.7511175581086876),
        ("weighted", 0.49580375185868525),
        ("node_names", 0.47638018779967434),
        ("preferential", 0.7511175581086876),
        ("from", 0.22531527394273332),
        ("get", 0.07632665073926659),
    ],
    &[
        ("jaccard", 0.9669988468185846),
        ("coefficient", 0.8453554031295414),
        ("from", 0.22531527394273332),
        ("unchecked", 0.40376559716520727),
        ("get", 0.07632665073926659),
        ("node_ids", 0.3583259510590198),
    ],
    &[
        ("get", 0.10272427111936212),
        ("jaccard", 1.3014360089247288),
        ("from", 0.30324070378630485),
        ("node_ids", 0.4822532076172131),
        ("coefficient", 1.1377221033835068),
    ],
    &[
        ("get", 0.10272427111936212),
        ("jaccard", 1.3014360089247288),
        ("coefficient", 1.1377221033835068),
        ("from", 0.30324070378630485),
        ("node_names", 0.6411365767193442),
    ],
    &[
        ("adamic", 0.7447968179042792),
        ("from", 0.1735411573756114),
        ("adar", 0.7447968179042792),
        ("unchecked", 0.31098623637168116),
        ("index", 0.5936694862927796),
        ("node_ids", 0.27598794869230175),
        ("get", 0.05878791559982294),
    ],
    &[
        ("index", 0.7707843197448777),
        ("from", 0.22531527394273332),
        ("node_ids", 0.3583259510590198),
        ("adamic", 0.9669988468185846),
        ("get", 0.07632665073926659),
        ("adar", 0.9669988468185846),
    ],
    &[
        ("node_names", 0.47638018779967434),
        ("get", 0.07632665073926659),
        ("index", 0.7707843197448777),
        ("from", 0.22531527394273332),
        ("adamic", 0.9669988468185846),
        ("adar", 0.9669988468185846),
    ],
    &[
        ("get", 0.05878791559982294),
        ("unchecked", 0.31098623637168116),
        ("index", 0.5936694862927796),
        ("from", 0.1735411573756114),
        ("allocation", 0.6511052379436727),
        ("resource", 0.6511052379436727),
        ("node_ids", 0.27598794869230175),
    ],
    &[
        ("weighted", 0.30268075577680353),
        ("index", 0.47055226905137437),
        ("allocation", 0.5160767972409035),
        ("node_ids", 0.21875261991139386),
        ("unchecked", 0.2464928424774615),
        ("from", 0.13755159606876494),
        ("get", 0.046596275734230565),
        ("resource", 0.5160767972409035),
    ],
    &[
        ("get", 0.07632665073926659),
        ("resource", 0.8453554031295414),
        ("index", 0.7707843197448777),
        ("node_ids", 0.3583259510590198),
        ("from", 0.22531527394273332),
        ("allocation", 0.8453554031295414),
    ],
    &[
        ("get", 0.07632665073926659),
        ("resource", 0.8453554031295414),
        ("index", 0.7707843197448777),
        ("node_names", 0.47638018779967434),
        ("from", 0.22531527394273332),
        ("allocation", 0.8453554031295414),
    ],
    &[
        ("index", 0.5936694862927796),
        ("from", 0.1735411573756114),
        ("weighted", 0.3818753847579611),
        ("get", 0.05878791559982294),
        ("allocation", 0.6511052379436727),
        ("node_ids", 0.27598794869230175),
        ("resource", 0.6511052379436727),
    ],
    &[
        ("index", 0.5936694862927796),
        ("weighted", 0.3818753847579611),
        ("from", 0.1735411573756114),
        ("node_names", 0.3669150683614046),
        ("get", 0.05878791559982294),
        ("allocation", 0.6511052379436727),
        ("resource", 0.6511052379436727),
    ],
    &[
        ("node_ids", 0.27598794869230175),
        ("metrics", 0.8730353432818593),
        ("unchecked", 0.31098623637168116),
        ("from", 0.1735411573756114),
        ("get", 0.05878791559982294),
        ("all", 0.6294469262368764),
        ("edge", 0.3408118469862679),
    ],
    &[
        ("strongly", 3.2095419138451797),
        ("connected", 2.12682127667469),
        ("components", 2.598264873244137),
    ],
    &[
        ("get", 0.07632665073926659),
        ("from", 0.22531527394273332),
        ("edge", 0.4424893542713664),
        ("unchecked", 0.40376559716520727),
        ("edge_id", 0.5549030310349807),
        ("weight", 0.7332412746210047),
    ],
    &[
        ("weight", 0.7332412746210047),
        ("from", 0.22531527394273332),
        ("unchecked", 0.40376559716520727),
        ("get", 0.07632665073926659),
        ("node_ids", 0.3583259510590198),
        ("edge", 0.4424893542713664),
    ],
    &[
        ("node_name", 0.8213562238441001),
        ("from", 0.30324070378630485),
        ("unchecked", 0.543408184037245),
        ("get", 0.10272427111936212),
        ("node_id", 0.6173484597137204),
    ],
    &[
        ("from", 0.30324070378630485),
        ("edge_type_id", 0.8339521528377006),
        ("get", 0.10272427111936212),
        ("edge_type_name", 0.9647817557251608),
        ("unchecked", 0.543408184037245),
    ],
    &[
        ("edge_type_id", 0.8339521528377006),
        ("get", 0.10272427111936212),
        ("unchecked", 0.543408184037245),
        ("from", 0.30324070378630485),
        ("edge_type_name", 0.9647817557251608),
    ],
    &[
        ("unchecked", 0.40376559716520727),
        ("count", 0.8453554031295414),
        ("from", 0.22531527394273332),
        ("get", 0.07632665073926659),
        ("edge_type_id", 0.6196468858751043),
        ("edge", 0.4424893542713664),
    ],
    &[
        ("unchecked", 0.31098623637168116),
        ("edge_id", 0.4273945239128388),
        ("node_ids", 0.27598794869230175),
        ("from", 0.1735411573756114),
        ("and", 0.5296701048117646),
        ("get", 0.05878791559982294),
        ("edge_type_id", 0.47726119875162193),
    ],
    &[
        ("minmax", 0.9176146525871081),
        ("get", 0.07632665073926659),
        ("from", 0.22531527394273332),
        ("unchecked", 0.40376559716520727),
        ("edge_ids", 0.7707843197448777),
        ("node_ids", 0.3583259510590198),
    ],
    &[
        ("node_ids", 0.4822532076172131),
        ("unchecked", 0.543408184037245),
        ("get", 0.10272427111936212),
        ("edge_id", 0.7468165948970197),
        ("from", 0.30324070378630485),
    ],
    &[
        ("edge_id", 0.7468165948970197),
        ("get", 0.10272427111936212),
        ("unchecked", 0.543408184037245),
        ("node_names", 0.6411365767193442),
        ("from", 0.30324070378630485),
    ],
    &[
        ("unchecked", 0.40376559716520727),
        ("source", 0.7511175581086876),
        ("get", 0.07632665073926659),
        ("node_id", 0.45870503392758394),
        ("from", 0.22531527394273332),
        ("edge_id", 0.5549030310349807),
    ],
    &[
        ("from", 0.22531527394273332),
        ("edge_id", 0.5549030310349807),
        ("destination", 0.8453554031295414),
        ("node_id", 0.45870503392758394),
        ("unchecked", 0.40376559716520727),
        ("get", 0.07632665073926659),
    ],
    &[
        ("get", 0.10272427111936212),
        ("edge_id", 0.7468165948970197),
        ("from", 0.30324070378630485),
        ("source", 1.0108920401242731),
        ("node_id", 0.6173484597137204),
    ],
    &[
        ("edge_id", 0.7468165948970197),
        ("node_id", 0.6173484597137204),
        ("destination", 1.1377221033835068),
        ("get", 0.10272427111936212),
        ("from", 0.30324070378630485),
    ],
    &[
        ("edge_id", 0.5549030310349807),
        ("node_name", 0.610287801964798),
        ("from", 0.22531527394273332),
        ("source", 0.7511175581086876),
        ("unchecked", 0.40376559716520727),
        ("get", 0.07632665073926659),
    ],
    &[
        ("node_name", 0.610287801964798),
        ("edge_id", 0.5549030310349807),
        ("destination", 0.8453554031295414),
        ("from", 0.22531527394273332),
        ("unchecked", 0.40376559716520727),
        ("get", 0.07632665073926659),
    ],
    &[
        ("from", 0.30324070378630485),
        ("source", 1.0108920401242731),
        ("edge_id", 0.7468165948970197),
        ("node_name", 0.8213562238441001),
        ("get", 0.10272427111936212),
    ],
    &[
        ("get", 0.10272427111936212),
        ("from", 0.30324070378630485),
        ("node_name", 0.8213562238441001),
        ("edge_id", 0.7468165948970197),
        ("destination", 1.1377221033835068),
    ],
    &[
        ("from", 0.4272087119935153),
        ("edge_id", 1.0521231207343864),
        ("get", 0.14471904004780678),
        ("node_names", 0.9032399929569503),
    ],
    &[
        ("get", 0.14471904004780678),
        ("from", 0.4272087119935153),
        ("node_ids", 0.6794034214683663),
        ("edge_id", 1.0521231207343864),
    ],
    &[
        ("from", 0.30324070378630485),
        ("unchecked", 0.543408184037245),
        ("node_ids", 0.4822532076172131),
        ("edge_id", 0.7468165948970197),
        ("get", 0.10272427111936212),
    ],
    &[
        ("node_ids", 0.6794034214683663),
        ("edge_id", 1.0521231207343864),
        ("get", 0.14471904004780678),
        ("from", 0.4272087119935153),
    ],
    &[
        ("unique", 1.0108920401242731),
        ("unchecked", 0.543408184037245),
        ("node_id", 0.6173484597137204),
        ("source", 1.0108920401242731),
        ("get", 0.10272427111936212),
    ],
    &[
        ("from", 0.1735411573756114),
        ("edge_id", 0.4273945239128388),
        ("unchecked", 0.31098623637168116),
        ("edge_type_id", 0.47726119875162193),
        ("get", 0.05878791559982294),
        ("node_ids", 0.27598794869230175),
        ("and", 0.5296701048117646),
    ],
    &[
        ("and", 0.6876914189677453),
        ("from", 0.22531527394273332),
        ("get", 0.07632665073926659),
        ("node_ids", 0.3583259510590198),
        ("edge_id", 0.5549030310349807),
        ("edge_type_id", 0.6196468858751043),
    ],
    &[
        ("and", 0.5457565852235744),
        ("unchecked", 0.16537344698914663),
        ("edge_type_id", 0.2537936420356482),
        ("weight", 0.3003193880561748),
        ("edge_id", 0.22727599288116793),
        ("edge", 0.1812338403410739),
        ("from", 0.09228414647711466),
        ("node_ids", 0.1467623742298385),
        ("get", 0.031261705847426505),
    ],
    &[
        ("edge", 0.21915088224596296),
        ("and", 0.6556592841033845),
        ("edge_id", 0.27482579555506437),
        ("weight", 0.3631510468697076),
        ("edge_type_id", 0.3068913645258291),
        ("node_ids", 0.1774674295509701),
        ("from", 0.1115914780579301),
        ("get", 0.03780215882466524),
    ],
    &[
        ("k", 1.0998770078216682),
        ("get", 0.10272427111936212),
        ("central", 1.1819019812740101),
        ("top", 1.3014360089247288),
        ("node_ids", 0.4822532076172131),
    ],
    &[
        ("node_ids", 0.3583259510590198),
        ("get", 0.07632665073926659),
        ("central", 0.8781821350470027),
        ("weighted", 0.49580375185868525),
        ("k", 0.8172355697185438),
        ("top", 0.9669988468185846),
    ],
    &[
        ("from", 0.22531527394273332),
        ("node_id", 0.45870503392758394),
        ("get", 0.07632665073926659),
        ("degree", 0.6294741144196445),
        ("node", 0.4348571865302264),
        ("unchecked", 0.40376559716520727),
    ],
    &[
        ("node", 0.33493343847935564),
        ("from", 0.1735411573756114),
        ("unchecked", 0.31098623637168116),
        ("weighted", 0.3818753847579611),
        ("degree", 0.4848302755637319),
        ("node_id", 0.3533014033573437),
        ("get", 0.05878791559982294),
    ],
    &[
        ("node_id", 0.6173484597137204),
        ("get", 0.10272427111936212),
        ("node", 0.5852528192273099),
        ("degree", 0.8471781345830511),
        ("from", 0.30324070378630485),
    ],
    &[
        ("node_id", 0.3533014033573437),
        ("unchecked", 0.31098623637168116),
        ("from", 0.1735411573756114),
        ("get", 0.05878791559982294),
        ("degree", 0.4848302755637319),
        ("node", 0.33493343847935564),
        ("comulative", 0.7957218886168173),
    ],
    &[
        ("node", 0.4348571865302264),
        ("comulative", 1.033116858428441),
        ("degree", 0.6294741144196445),
        ("from", 0.22531527394273332),
        ("get", 0.07632665073926659),
        ("node_id", 0.45870503392758394),
    ],
    &[
        ("node_id", 0.45870503392758394),
        ("get", 0.07632665073926659),
        ("degree", 0.6294741144196445),
        ("from", 0.22531527394273332),
        ("weighted", 0.49580375185868525),
        ("node", 0.4348571865302264),
    ],
    &[
        ("node_name", 0.8213562238441001),
        ("get", 0.10272427111936212),
        ("node", 0.5852528192273099),
        ("from", 0.30324070378630485),
        ("degree", 0.8471781345830511),
    ],
    &[
        ("top", 1.3014360089247288),
        ("central", 1.1819019812740101),
        ("k", 1.0998770078216682),
        ("node_names", 0.6411365767193442),
        ("get", 0.10272427111936212),
    ],
    &[
        ("from", 0.30324070378630485),
        ("node_id", 0.6173484597137204),
        ("unchecked", 0.543408184037245),
        ("node_type_id", 1.0667757976163075),
        ("get", 0.10272427111936212),
    ],
    &[
        ("get", 0.14471904004780678),
        ("node_id", 0.8697270420244625),
        ("node_type_ids", 1.424155402074),
        ("from", 0.4272087119935153),
    ],
    &[
        ("get", 0.10272427111936212),
        ("edge_id", 0.7468165948970197),
        ("edge_type_id", 0.8339521528377006),
        ("from", 0.30324070378630485),
        ("unchecked", 0.543408184037245),
    ],
    &[
        ("edge_id", 1.0521231207343864),
        ("edge_type_id", 1.1748806167165462),
        ("get", 0.14471904004780678),
        ("from", 0.4272087119935153),
    ],
    &[
        ("node_id", 0.6173484597137204),
        ("get", 0.10272427111936212),
        ("node_type_names", 1.0108920401242731),
        ("from", 0.30324070378630485),
        ("unchecked", 0.543408184037245),
    ],
    &[
        ("get", 0.14471904004780678),
        ("node_type_names", 1.424155402074),
        ("from", 0.4272087119935153),
        ("node_id", 0.8697270420244625),
    ],
    &[
        ("node_type_names", 1.424155402074),
        ("node_name", 1.1571353386765968),
        ("get", 0.14471904004780678),
        ("from", 0.4272087119935153),
    ],
    &[
        ("get", 0.14471904004780678),
        ("from", 0.4272087119935153),
        ("edge_type_name", 1.3591947455333753),
        ("edge_id", 1.0521231207343864),
    ],
    &[
        ("get", 0.14471904004780678),
        ("edge_type_name", 1.3591947455333753),
        ("edge_type_id", 1.1748806167165462),
        ("from", 0.4272087119935153),
    ],
    &[
        ("edge", 0.5955245770036008),
        ("from", 0.30324070378630485),
        ("edge_id", 0.7468165948970197),
        ("get", 0.10272427111936212),
        ("weight", 0.9868332326984337),
    ],
    &[
        ("get", 0.10272427111936212),
        ("node_ids", 0.4822532076172131),
        ("weight", 0.9868332326984337),
        ("edge", 0.5955245770036008),
        ("from", 0.30324070378630485),
    ],
    &[
        ("edge", 0.3408118469862679),
        ("weight", 0.5647532775147736),
        ("edge_type_id", 0.47726119875162193),
        ("node_ids", 0.27598794869230175),
        ("and", 0.5296701048117646),
        ("get", 0.05878791559982294),
        ("from", 0.1735411573756114),
    ],
    &[
        ("node_names", 0.3669150683614046),
        ("and", 0.5296701048117646),
        ("weight", 0.5647532775147736),
        ("from", 0.1735411573756114),
        ("get", 0.05878791559982294),
        ("edge_type_name", 0.5521334715718345),
        ("edge", 0.3408118469862679),
    ],
    &[
        ("weight", 0.9868332326984337),
        ("node_names", 0.6411365767193442),
        ("edge", 0.5955245770036008),
        ("get", 0.10272427111936212),
        ("from", 0.30324070378630485),
    ],
    &[
        ("from", 0.30324070378630485),
        ("node_id", 0.6173484597137204),
        ("unchecked", 0.543408184037245),
        ("get", 0.10272427111936212),
        ("node_name", 0.8213562238441001),
    ],
    &[
        ("get", 0.14471904004780678),
        ("from", 0.4272087119935153),
        ("node_id", 0.8697270420244625),
        ("node_name", 1.1571353386765968),
    ],
    &[
        ("node_name", 1.1571353386765968),
        ("from", 0.4272087119935153),
        ("node_id", 0.8697270420244625),
        ("get", 0.14471904004780678),
    ],
    &[
        ("get", 0.14471904004780678),
        ("from", 0.4272087119935153),
        ("node_names", 0.9032399929569503),
        ("node_ids", 0.6794034214683663),
    ],
    &[
        ("from", 0.22531527394273332),
        ("node_ids", 0.3583259510590198),
        ("node_names", 0.47638018779967434),
        ("get", 0.07632665073926659),
        ("edge", 0.8204872254500389),
    ],
    &[
        ("from", 0.22531527394273332),
        ("get", 0.07632665073926659),
        ("edge", 0.8204872254500389),
        ("node_names", 0.47638018779967434),
        ("node_ids", 0.3583259510590198),
    ],
    &[
        ("node_type_ids", 1.424155402074),
        ("node_name", 1.1571353386765968),
        ("from", 0.4272087119935153),
        ("get", 0.14471904004780678),
    ],
    &[
        ("node_type_name", 1.5028850309181347),
        ("get", 0.14471904004780678),
        ("node_name", 1.1571353386765968),
        ("from", 0.4272087119935153),
    ],
    &[
        ("from", 0.30324070378630485),
        ("get", 0.10272427111936212),
        ("edge_type_id", 0.8339521528377006),
        ("edge", 0.5955245770036008),
        ("count", 1.1377221033835068),
    ],
    &[
        ("edge_type_id", 1.1748806167165462),
        ("get", 0.14471904004780678),
        ("from", 0.4272087119935153),
        ("edge_type_name", 1.3591947455333753),
    ],
    &[
        ("edge_type_name", 0.9647817557251608),
        ("edge", 0.5955245770036008),
        ("count", 1.1377221033835068),
        ("get", 0.10272427111936212),
        ("from", 0.30324070378630485),
    ],
    &[
        ("node_type_name", 1.5028850309181347),
        ("from", 0.4272087119935153),
        ("get", 0.14471904004780678),
        ("node_type_id", 1.5028850309181347),
    ],
    &[
        ("node_type_id", 1.0667757976163075),
        ("count", 1.1377221033835068),
        ("from", 0.30324070378630485),
        ("get", 0.10272427111936212),
        ("node", 0.5852528192273099),
    ],
    &[
        ("from", 0.30324070378630485),
        ("count", 1.1377221033835068),
        ("get", 0.10272427111936212),
        ("node_type_name", 1.0667757976163075),
        ("node", 0.5852528192273099),
    ],
    &[
        ("node_id", 0.6173484597137204),
        ("get", 0.10272427111936212),
        ("node_ids", 0.4822532076172131),
        ("from", 0.30324070378630485),
        ("neighbour", 1.3014360089247288),
    ],
    &[
        ("from", 0.30324070378630485),
        ("get", 0.10272427111936212),
        ("node_name", 0.8213562238441001),
        ("neighbour", 1.3014360089247288),
        ("node_ids", 0.4822532076172131),
    ],
    &[
        ("from", 0.30324070378630485),
        ("node_name", 0.8213562238441001),
        ("neighbour", 1.3014360089247288),
        ("get", 0.10272427111936212),
        ("node_names", 0.6411365767193442),
    ],
    &[
        ("from", 0.30324070378630485),
        ("edge_ids", 1.0373605636974754),
        ("get", 0.10272427111936212),
        ("node_ids", 0.4822532076172131),
        ("minmax", 1.2349722599181758),
    ],
    &[
        ("edge_id", 0.5549030310349807),
        ("get", 0.07632665073926659),
        ("edge_type_id", 0.6196468858751043),
        ("and", 0.6876914189677453),
        ("from", 0.22531527394273332),
        ("node_ids", 0.3583259510590198),
    ],
    &[
        ("node_names", 0.9032399929569503),
        ("get", 0.14471904004780678),
        ("edge_id", 1.0521231207343864),
        ("from", 0.4272087119935153),
    ],
    &[
        ("get", 0.07632665073926659),
        ("edge_id", 0.5549030310349807),
        ("and", 0.6876914189677453),
        ("from", 0.22531527394273332),
        ("node_names", 0.47638018779967434),
        ("edge_type_name", 0.7168564868499797),
    ],
    &[
        ("from", 0.4272087119935153),
        ("edge_type_names", 1.6650760165708902),
        ("edge_type_ids", 1.6028349371446486),
        ("get", 0.14471904004780678),
    ],
    &[
        ("node_type_names", 1.424155402074),
        ("get", 0.14471904004780678),
        ("node_type_ids", 1.424155402074),
        ("from", 0.4272087119935153),
    ],
    &[
        ("from", 0.30324070378630485),
        ("node_type_names", 1.0108920401242731),
        ("multiple", 1.5255162287186315),
        ("get", 0.10272427111936212),
        ("node_type_ids", 1.0108920401242731),
    ],
    &[
        ("get", 0.05878791559982294),
        ("unchecked", 0.31098623637168116),
        ("node_id", 0.3533014033573437),
        ("source", 0.5785218555243384),
        ("minmax", 0.7067603809019184),
        ("from", 0.1735411573756114),
        ("edge_ids", 0.5936694862927796),
    ],
    &[
        ("source", 0.7511175581086876),
        ("get", 0.07632665073926659),
        ("node_id", 0.45870503392758394),
        ("minmax", 0.9176146525871081),
        ("edge_ids", 0.7707843197448777),
        ("from", 0.22531527394273332),
    ],
    &[
        ("from", 0.4272087119935153),
        ("node_type_id", 1.5028850309181347),
        ("node_type_name", 1.5028850309181347),
        ("get", 0.14471904004780678),
    ],
    &[
        ("from", 0.30324070378630485),
        ("node_type_names", 1.0108920401242731),
        ("unchecked", 0.543408184037245),
        ("node_type_ids", 1.0108920401242731),
        ("get", 0.10272427111936212),
    ],
    &[
        ("degree", 1.7823826979024435),
        ("centrality", 1.9472274690662594),
        ("get", 0.21612215427148931),
    ],
    &[
        ("centrality", 1.3038963591117803),
        ("get", 0.14471904004780678),
        ("weighted", 0.9400680145942347),
        ("degree", 1.1935135197395608),
    ],
    &[
        ("closeness", 0.9176146525871081),
        ("node_id", 0.45870503392758394),
        ("centrality", 0.6876914189677453),
        ("unchecked", 0.40376559716520727),
        ("get", 0.07632665073926659),
        ("from", 0.22531527394273332),
    ],
    &[
        ("closeness", 0.7067603809019184),
        ("from", 0.1735411573756114),
        ("unchecked", 0.31098623637168116),
        ("weighted", 0.3818753847579611),
        ("get", 0.05878791559982294),
        ("node_id", 0.3533014033573437),
        ("centrality", 0.5296701048117646),
    ],
    &[
        ("closeness", 2.598264873244137),
        ("centrality", 1.9472274690662594),
        ("get", 0.21612215427148931),
    ],
    &[
        ("closeness", 1.7398419866455679),
        ("weighted", 0.9400680145942347),
        ("get", 0.14471904004780678),
        ("centrality", 1.3038963591117803),
    ],
    &[
        ("get", 0.07632665073926659),
        ("unchecked", 0.40376559716520727),
        ("harmonic", 0.9176146525871081),
        ("node_id", 0.45870503392758394),
        ("centrality", 0.6876914189677453),
        ("from", 0.22531527394273332),
    ],
    &[
        ("harmonic", 0.7067603809019184),
        ("node_id", 0.3533014033573437),
        ("weighted", 0.3818753847579611),
        ("get", 0.05878791559982294),
        ("from", 0.1735411573756114),
        ("unchecked", 0.31098623637168116),
        ("centrality", 0.5296701048117646),
    ],
    &[
        ("get", 0.21612215427148931),
        ("harmonic", 2.598264873244137),
        ("centrality", 1.9472274690662594),
    ],
    &[
        ("get", 0.14471904004780678),
        ("weighted", 0.9400680145942347),
        ("harmonic", 1.7398419866455679),
        ("centrality", 1.3038963591117803),
    ],
    &[
        ("centrality", 1.9472274690662594),
        ("stress", 3.2095419138451797),
        ("get", 0.21612215427148931),
    ],
    &[
        ("centrality", 1.9472274690662594),
        ("get", 0.21612215427148931),
        ("betweenness", 3.2095419138451797),
    ],
    &[
        ("get", 0.21612215427148931),
        ("eigenvector", 2.925314276142877),
        ("centrality", 1.9472274690662594),
    ],
    &[
        ("weighted", 0.9400680145942347),
        ("get", 0.14471904004780678),
        ("eigenvector", 1.9588397834945597),
        ("centrality", 1.3038963591117803),
    ],
    &[
        ("approximated", 2.1491634040506558),
        ("cover", 2.1491634040506558),
        ("vertex", 2.1491634040506558),
        ("set", 1.6650760165708902),
    ],
    &[
        ("by", 0.8172355697185438),
        ("node", 0.4348571865302264),
        ("degree", 0.6294741144196445),
        ("sort", 0.9176146525871081),
        ("outbound", 0.9176146525871081),
        ("increasing", 1.033116858428441),
    ],
    &[
        ("by", 0.8172355697185438),
        ("decreasing", 1.033116858428441),
        ("node", 0.4348571865302264),
        ("degree", 0.6294741144196445),
        ("sort", 0.9176146525871081),
        ("outbound", 0.9176146525871081),
    ],
    &[
        ("by", 1.0998770078216682),
        ("node", 0.5852528192273099),
        ("lexicographic", 1.390420976622124),
        ("order", 1.390420976622124),
        ("sort", 1.2349722599181758),
    ],
    &[
        ("get", 0.07632665073926659),
        ("topological", 0.9669988468185846),
        ("sorting", 0.9669988468185846),
        ("bfs", 0.9669988468185846),
        ("node_id", 0.45870503392758394),
        ("from", 0.22531527394273332),
    ],
    &[
        ("node_id", 0.3533014033573437),
        ("get", 0.05878791559982294),
        ("reversed", 0.8730353432818593),
        ("sorting", 0.7447968179042792),
        ("from", 0.1735411573756114),
        ("bfs", 0.7447968179042792),
        ("topological", 0.7447968179042792),
    ],
    &[
        ("by", 0.6294469262368764),
        ("topological", 0.7447968179042792),
        ("bfs", 0.7447968179042792),
        ("sorting", 0.7447968179042792),
        ("from", 0.1735411573756114),
        ("node_id", 0.3533014033573437),
        ("sort", 0.7067603809019184),
    ],
    &[
        ("connected", 1.424155402074),
        ("random", 1.3591947455333753),
        ("graph", 1.424155402074),
        ("generate", 1.5495183660894714),
    ],
    &[
        ("generate", 1.5495183660894714),
        ("random", 1.3591947455333753),
        ("spanning", 1.833476819479088),
        ("tree", 2.1491634040506558),
    ],
    &[
        ("circle", 3.2095419138451797),
        ("generate", 2.3140372355418344),
        ("graph", 2.12682127667469),
    ],
    &[
        ("generate", 2.3140372355418344),
        ("chain", 3.2095419138451797),
        ("graph", 2.12682127667469),
    ],
    &[
        ("graph", 2.12682127667469),
        ("generate", 2.3140372355418344),
        ("complete", 3.2095419138451797),
    ],
    &[
        ("generate", 2.3140372355418344),
        ("barbell", 3.2095419138451797),
        ("graph", 2.12682127667469),
    ],
    &[
        ("from", 0.1735411573756114),
        ("node_ids", 0.27598794869230175),
        ("first", 0.7067603809019184),
        ("search", 0.7067603809019184),
        ("breadth", 0.7067603809019184),
        ("get", 0.05878791559982294),
        ("unchecked", 0.31098623637168116),
    ],
    &[
        ("get", 0.05878791559982294),
        ("from", 0.1735411573756114),
        ("path", 0.5296701048117646),
        ("shortest", 0.5101139101355411),
        ("node_ids", 0.5204668365245463),
        ("unchecked", 0.31098623637168116),
    ],
    &[
        ("path", 0.5296701048117646),
        ("from", 0.1735411573756114),
        ("node_names", 0.3669150683614046),
        ("node_ids", 0.27598794869230175),
        ("unchecked", 0.31098623637168116),
        ("shortest", 0.5101139101355411),
        ("get", 0.05878791559982294),
    ],
    &[
        ("shortest", 0.6623008463371057),
        ("from", 0.22531527394273332),
        ("get", 0.07632665073926659),
        ("path", 0.6876914189677453),
        ("node_ids", 0.664426980114099),
    ],
    &[
        ("shortest", 0.6623008463371057),
        ("path", 0.6876914189677453),
        ("from", 0.22531527394273332),
        ("node_ids", 0.3583259510590198),
        ("node_names", 0.47638018779967434),
        ("get", 0.07632665073926659),
    ],
    &[
        ("from", 0.22531527394273332),
        ("shortest", 0.6623008463371057),
        ("node_names", 0.8833294061746341),
        ("get", 0.07632665073926659),
        ("path", 0.6876914189677453),
    ],
    &[
        ("path", 0.41982529913107036),
        ("unchecked", 0.2464928424774615),
        ("from", 0.13755159606876494),
        ("shortest", 0.4043247352796732),
        ("get", 0.046596275734230565),
        ("k", 0.4989100606092201),
        ("node_ids", 0.4174728053039836),
    ],
    &[
        ("shortest", 0.5101139101355411),
        ("k", 0.6294469262368764),
        ("from", 0.1735411573756114),
        ("get", 0.05878791559982294),
        ("node_ids", 0.5204668365245463),
        ("path", 0.5296701048117646),
    ],
    &[
        ("shortest", 0.5101139101355411),
        ("k", 0.6294469262368764),
        ("get", 0.05878791559982294),
        ("node_ids", 0.27598794869230175),
        ("node_names", 0.3669150683614046),
        ("path", 0.5296701048117646),
        ("from", 0.1735411573756114),
    ],
    &[
        ("k", 0.6294469262368764),
        ("get", 0.05878791559982294),
        ("path", 0.5296701048117646),
        ("shortest", 0.5101139101355411),
        ("node_names", 0.6919400858193145),
        ("from", 0.1735411573756114),
    ],
    &[
        ("from", 0.30324070378630485),
        ("get", 0.10272427111936212),
        ("node_id", 0.6173484597137204),
        ("unchecked", 0.543408184037245),
        ("eccentricity", 1.1377221033835068),
    ],
    &[
        ("unchecked", 0.40376559716520727),
        ("from", 0.22531527394273332),
        ("node_id", 0.45870503392758394),
        ("get", 0.07632665073926659),
        ("eccentricity", 0.8453554031295414),
        ("weighted", 0.49580375185868525),
    ],
    &[
        ("get", 0.14471904004780678),
        ("eccentricity", 1.6028349371446486),
        ("from", 0.4272087119935153),
        ("node_id", 0.8697270420244625),
    ],
    &[
        ("from", 0.30324070378630485),
        ("weighted", 0.6672777926796519),
        ("node_id", 0.6173484597137204),
        ("eccentricity", 1.1377221033835068),
        ("get", 0.10272427111936212),
    ],
    &[
        ("node_name", 1.1571353386765968),
        ("eccentricity", 1.6028349371446486),
        ("get", 0.14471904004780678),
        ("from", 0.4272087119935153),
    ],
    &[
        ("weighted", 0.6672777926796519),
        ("eccentricity", 1.1377221033835068),
        ("node_name", 0.8213562238441001),
        ("get", 0.10272427111936212),
        ("from", 0.30324070378630485),
    ],
    &[
        ("dijkstra", 1.3014360089247288),
        ("unchecked", 0.543408184037245),
        ("node_ids", 0.4822532076172131),
        ("get", 0.10272427111936212),
        ("from", 0.30324070378630485),
    ],
    &[
        ("from", 0.13755159606876494),
        ("unchecked", 0.2464928424774615),
        ("get", 0.046596275734230565),
        ("node_ids", 0.4174728053039836),
        ("path", 0.41982529913107036),
        ("weighted", 0.30268075577680353),
        ("shortest", 0.4043247352796732),
    ],
    &[
        ("node_names", 0.29082296118121864),
        ("unchecked", 0.2464928424774615),
        ("weighted", 0.30268075577680353),
        ("shortest", 0.4043247352796732),
        ("get", 0.046596275734230565),
        ("path", 0.41982529913107036),
        ("from", 0.13755159606876494),
        ("node_ids", 0.21875261991139386),
    ],
    &[
        ("node_ids", 0.5204668365245463),
        ("from", 0.1735411573756114),
        ("path", 0.5296701048117646),
        ("shortest", 0.5101139101355411),
        ("get", 0.05878791559982294),
        ("weighted", 0.3818753847579611),
    ],
    &[
        ("from", 0.1735411573756114),
        ("node_names", 0.3669150683614046),
        ("weighted", 0.3818753847579611),
        ("path", 0.5296701048117646),
        ("get", 0.05878791559982294),
        ("shortest", 0.5101139101355411),
        ("node_ids", 0.27598794869230175),
    ],
    &[
        ("get", 0.05878791559982294),
        ("weighted", 0.3818753847579611),
        ("path", 0.5296701048117646),
        ("from", 0.1735411573756114),
        ("node_names", 0.6919400858193145),
        ("shortest", 0.5101139101355411),
    ],
    &[
        ("node_ids", 0.3583259510590198),
        ("first", 0.9176146525871081),
        ("search", 0.9176146525871081),
        ("get", 0.07632665073926659),
        ("breadth", 0.9176146525871081),
        ("from", 0.22531527394273332),
    ],
    &[
        ("node_ids", 0.6794034214683663),
        ("from", 0.4272087119935153),
        ("dijkstra", 1.833476819479088),
        ("get", 0.14471904004780678),
    ],
    &[
        ("naive", 2.925314276142877),
        ("get", 0.21612215427148931),
        ("diameter", 2.7380983172757327),
    ],
    &[
        ("get", 0.34649865812996966),
        ("diameter", 4.389866443641716),
    ],
    &[
        ("weighted", 0.9400680145942347),
        ("diameter", 1.833476819479088),
        ("get", 0.14471904004780678),
        ("naive", 1.9588397834945597),
    ],
    &[
        ("breadth", 0.9176146525871081),
        ("node_names", 0.47638018779967434),
        ("search", 0.9176146525871081),
        ("first", 0.9176146525871081),
        ("get", 0.07632665073926659),
        ("from", 0.22531527394273332),
    ],
    &[
        ("from", 0.4272087119935153),
        ("dijkstra", 1.833476819479088),
        ("get", 0.14471904004780678),
        ("node_names", 0.9032399929569503),
    ],
    &[
        ("edges", 1.8425936389723871),
        ("get", 0.21612215427148931),
        ("bipartite", 2.925314276142877),
    ],
    &[
        ("bipartite", 2.925314276142877),
        ("get", 0.21612215427148931),
        ("edge_names", 2.7380983172757327),
    ],
    &[
        ("edges", 1.8425936389723871),
        ("star", 2.925314276142877),
        ("get", 0.21612215427148931),
    ],
    &[
        ("edge_names", 2.7380983172757327),
        ("get", 0.21612215427148931),
        ("star", 2.925314276142877),
    ],
    &[
        ("get", 0.21612215427148931),
        ("edges", 1.8425936389723871),
        ("clique", 2.925314276142877),
    ],
    &[
        ("clique", 2.925314276142877),
        ("get", 0.21612215427148931),
        ("edge_names", 2.7380983172757327),
    ],
    &[
        ("feature", 1.133495941297005),
        ("node", 0.4348571865302264),
        ("get", 0.07632665073926659),
        ("bm25", 1.033116858428441),
        ("propagation", 1.033116858428441),
        ("okapi", 1.033116858428441),
    ],
    &[
        ("get", 0.07632665073926659),
        ("node", 0.4348571865302264),
        ("label", 0.7707843197448777),
        ("propagation", 1.033116858428441),
        ("okapi", 1.033116858428441),
        ("bm25", 1.033116858428441),
    ],
    &[("from", 1.0228595034785064), ("csv", 5.145710166123201)],
    &[
        ("inplace", 2.0298095978395323),
        ("to", 2.076203820513492),
        ("directed", 2.12682127667469),
    ],
    &[("directed", 3.4098342251591442), ("to", 3.328681597854794)],
    &[
        ("upper", 3.2095419138451797),
        ("triangular", 2.925314276142877),
        ("to", 2.076203820513492),
    ],
    &[
        ("lower", 3.2095419138451797),
        ("triangular", 2.925314276142877),
        ("to", 2.076203820513492),
    ],
    &[
        ("to", 2.076203820513492),
        ("diagonal", 2.925314276142877),
        ("main", 3.2095419138451797),
    ],
    &[
        ("anti", 3.2095419138451797),
        ("diagonal", 2.925314276142877),
        ("to", 2.076203820513492),
    ],
    &[("to", 3.328681597854794), ("bidiagonal", 5.145710166123201)],
    &[("to", 3.328681597854794), ("arrowhead", 5.145710166123201)],
    &[("to", 3.328681597854794), ("transposed", 5.145710166123201)],
    &[
        ("to", 3.328681597854794),
        ("complementary", 5.145710166123201),
    ],
    &[
        ("get", 0.14471904004780678),
        ("number", 0.9702938033760223),
        ("triangles", 1.9588397834945597),
        ("of", 1.9588397834945597),
    ],
    &[
        ("triads", 2.925314276142877),
        ("number", 1.4490283171628935),
        ("get", 0.21612215427148931),
    ],
    &[
        ("weighted", 0.9400680145942347),
        ("number", 0.9702938033760223),
        ("triads", 1.9588397834945597),
        ("get", 0.14471904004780678),
    ],
    &[
        ("transitivity", 5.145710166123201),
        ("get", 0.34649865812996966),
    ],
    &[
        ("triangles", 1.033116858428441),
        ("number", 0.5117452148680048),
        ("get", 0.07632665073926659),
        ("node", 0.4348571865302264),
        ("of", 1.033116858428441),
        ("per", 1.033116858428441),
    ],
    &[
        ("get", 0.10272427111936212),
        ("node", 0.5852528192273099),
        ("clustering", 1.3014360089247288),
        ("per", 1.390420976622124),
        ("coefficient", 1.1377221033835068),
    ],
    &[
        ("get", 0.21612215427148931),
        ("coefficient", 2.393659738503486),
        ("clustering", 2.7380983172757327),
    ],
    &[
        ("average", 2.1491634040506558),
        ("clustering", 1.833476819479088),
        ("coefficient", 1.6028349371446486),
        ("get", 0.14471904004780678),
    ],
    &[
        ("edge", 0.5955245770036008),
        ("sparse", 1.5255162287186315),
        ("weighting", 1.390420976622124),
        ("methods", 1.3014360089247288),
        ("get", 0.10272427111936212),
    ],
    &[
        ("methods", 1.833476819479088),
        ("get", 0.14471904004780678),
        ("edge", 0.8389813251504573),
        ("weighting", 1.9588397834945597),
    ],
    &[("edge", 2.0087605837124465), ("encode", 5.145710166123201)],
    &[("decode", 5.145710166123201), ("edge", 2.0087605837124465)],
    &[
        ("number", 0.6887326207422982),
        ("max", 1.5255162287186315),
        ("edge", 0.5955245770036008),
        ("get", 0.10272427111936212),
        ("encodable", 1.5255162287186315),
    ],
    &[
        ("negatives", 5.145710166123201),
        ("sample", 5.145710166123201),
    ],
    &[
        ("holdout", 3.598335411599793),
        ("connected", 3.4098342251591442),
    ],
    &[
        ("random", 3.2542998855509446),
        ("holdout", 3.598335411599793),
    ],
    &[
        ("indices", 1.5255162287186315),
        ("get", 0.10272427111936212),
        ("node", 0.5852528192273099),
        ("holdout", 1.0667757976163075),
        ("label", 1.0373605636974754),
    ],
    &[
        ("holdout", 1.0667757976163075),
        ("get", 0.10272427111936212),
        ("label", 1.0373605636974754),
        ("node", 0.5852528192273099),
        ("labels", 1.5255162287186315),
    ],
    &[
        ("get", 0.10272427111936212),
        ("label", 1.0373605636974754),
        ("node", 0.5852528192273099),
        ("holdout", 1.0667757976163075),
        ("graphs", 1.390420976622124),
    ],
    &[
        ("graphs", 1.390420976622124),
        ("get", 0.10272427111936212),
        ("label", 1.0373605636974754),
        ("edge", 0.5955245770036008),
        ("holdout", 1.0667757976163075),
    ],
    &[
        ("random", 2.0298095978395323),
        ("subgraph", 3.2095419138451797),
        ("get", 0.21612215427148931),
    ],
    &[
        ("node", 0.5852528192273099),
        ("holdout", 1.0667757976163075),
        ("random", 0.9647817557251608),
        ("label", 1.0373605636974754),
        ("get", 0.10272427111936212),
    ],
    &[
        ("kfold", 1.833476819479088),
        ("label", 1.4614445381394745),
        ("node", 0.8245103641128158),
        ("get", 0.14471904004780678),
    ],
    &[
        ("random", 0.9647817557251608),
        ("get", 0.10272427111936212),
        ("edge", 0.5955245770036008),
        ("holdout", 1.0667757976163075),
        ("label", 1.0373605636974754),
    ],
    &[
        ("kfold", 1.833476819479088),
        ("edge", 0.8389813251504573),
        ("label", 1.4614445381394745),
        ("get", 0.14471904004780678),
    ],
    &[
        ("get", 0.14471904004780678),
        ("prediction", 2.1491634040506558),
        ("kfold", 1.833476819479088),
        ("edge", 0.8389813251504573),
    ],
    &[
        ("louvain", 1.5255162287186315),
        ("get", 0.10272427111936212),
        ("detection", 1.5255162287186315),
        ("undirected", 1.1819019812740101),
        ("community", 1.3014360089247288),
    ],
    &[
        ("directed", 0.5785218555243384),
        ("memberships", 0.7957218886168173),
        ("node", 0.33493343847935564),
        ("modularity", 0.7957218886168173),
        ("community", 0.7447968179042792),
        ("get", 0.05878791559982294),
        ("from", 0.1735411573756114),
    ],
    &[
        ("modularity", 0.7957218886168173),
        ("get", 0.05878791559982294),
        ("node", 0.33493343847935564),
        ("community", 0.7447968179042792),
        ("from", 0.1735411573756114),
        ("undirected", 0.6763888725154819),
        ("memberships", 0.7957218886168173),
    ],
    &[
        ("is", 3.1218997764463956),
        ("compatible", 5.145710166123201),
    ],
    &[
        ("matrix", 1.6028349371446486),
        ("adjacency", 1.833476819479088),
        ("same", 2.1491634040506558),
        ("has", 0.9032399929569503),
    ],
    &[
        ("graph", 1.424155402074),
        ("transformed", 1.7398419866455679),
        ("laplacian", 1.7398419866455679),
        ("get", 0.14471904004780678),
    ],
    &[
        ("get", 0.07632665073926659),
        ("matrix", 0.8453554031295414),
        ("number", 0.5117452148680048),
        ("laplacian", 0.9176146525871081),
        ("edges", 0.6507384752401234),
        ("coo", 1.133495941297005),
    ],
    &[
        ("normalized", 0.7447968179042792),
        ("transformed", 0.7067603809019184),
        ("graph", 0.5785218555243384),
        ("random", 0.5521334715718345),
        ("walk", 0.7957218886168173),
        ("laplacian", 0.7067603809019184),
        ("get", 0.05878791559982294),
    ],
    &[
        ("transformed", 0.9176146525871081),
        ("graph", 0.7511175581086876),
        ("laplacian", 0.9176146525871081),
        ("symmetric", 1.033116858428441),
        ("normalized", 0.9669988468185846),
        ("get", 0.07632665073926659),
    ],
    &[
        ("transformed", 1.2349722599181758),
        ("normalized", 1.3014360089247288),
        ("symmetric", 1.390420976622124),
        ("graph", 1.0108920401242731),
        ("get", 0.10272427111936212),
    ],
    &[
        ("memory", 2.0298095978395323),
        ("stats", 3.2095419138451797),
        ("get", 0.21612215427148931),
    ],
    &[
        ("memory", 1.3591947455333753),
        ("total", 1.3591947455333753),
        ("used", 2.1491634040506558),
        ("get", 0.14471904004780678),
    ],
    &[
        ("nodes", 0.7040681928557436),
        ("total", 0.9647817557251608),
        ("get", 0.10272427111936212),
        ("requirement", 1.2349722599181758),
        ("memory", 0.9647817557251608),
    ],
    &[
        ("readable", 0.6763888725154819),
        ("get", 0.05878791559982294),
        ("nodes", 0.4029301064597355),
        ("human", 0.6763888725154819),
        ("memory", 0.5521334715718345),
        ("total", 0.5521334715718345),
        ("requirement", 0.7067603809019184),
    ],
    &[
        ("requirement", 1.2349722599181758),
        ("total", 0.9647817557251608),
        ("memory", 0.9647817557251608),
        ("edges", 0.8757967880277655),
        ("get", 0.10272427111936212),
    ],
    &[
        ("requirement", 0.7067603809019184),
        ("readable", 0.6763888725154819),
        ("human", 0.6763888725154819),
        ("edges", 0.5012084008592964),
        ("memory", 0.5521334715718345),
        ("get", 0.05878791559982294),
        ("total", 0.5521334715718345),
    ],
    &[
        ("requirements", 0.8453554031295414),
        ("edge", 0.4424893542713664),
        ("total", 0.7168564868499797),
        ("memory", 0.7168564868499797),
        ("get", 0.07632665073926659),
        ("weights", 0.7511175581086876),
    ],
    &[
        ("get", 0.046596275734230565),
        ("weights", 0.4585460059817457),
        ("edge", 0.27013311551587593),
        ("readable", 0.5361170248293621),
        ("human", 0.5361170248293621),
        ("memory", 0.4376301357338248),
        ("requirements", 0.5160767972409035),
        ("total", 0.4376301357338248),
    ],
    &[
        ("total", 0.9647817557251608),
        ("get", 0.10272427111936212),
        ("memory", 0.9647817557251608),
        ("requirements", 1.1377221033835068),
        ("node_types", 0.6887326207422982),
    ],
    &[
        ("memory", 0.5521334715718345),
        ("human", 0.6763888725154819),
        ("node_types", 0.3941537354107489),
        ("get", 0.05878791559982294),
        ("total", 0.5521334715718345),
        ("requirements", 0.6511052379436727),
        ("readable", 0.6763888725154819),
    ],
    &[
        ("total", 0.9647817557251608),
        ("requirements", 1.1377221033835068),
        ("get", 0.10272427111936212),
        ("memory", 0.9647817557251608),
        ("edge_types", 0.7040681928557436),
    ],
    &[
        ("human", 0.6763888725154819),
        ("edge_types", 0.4029301064597355),
        ("readable", 0.6763888725154819),
        ("total", 0.5521334715718345),
        ("get", 0.05878791559982294),
        ("requirements", 0.6511052379436727),
        ("memory", 0.5521334715718345),
    ],
    &[
        ("transitive", 3.2095419138451797),
        ("get", 0.21612215427148931),
        ("closure", 3.2095419138451797),
    ],
    &[
        ("all", 1.5495183660894714),
        ("paths", 1.9588397834945597),
        ("shortest", 1.2557545991658023),
        ("get", 0.14471904004780678),
    ],
    &[
        ("weighted", 0.6672777926796519),
        ("all", 1.0998770078216682),
        ("shortest", 0.8913580124735546),
        ("paths", 1.390420976622124),
        ("get", 0.10272427111936212),
    ],
    &[
        ("filter", 4.690021135078359),
        ("from_ids", 5.145710166123201),
    ],
    &[
        ("filter", 4.690021135078359),
        ("from_names", 5.145710166123201),
    ],
    &[
        ("unknown", 1.8116741102988692),
        ("drop", 2.3140372355418344),
        ("node_types", 1.4490283171628935),
    ],
    &[
        ("drop", 2.3140372355418344),
        ("edge_types", 1.481292910973371),
        ("unknown", 1.8116741102988692),
    ],
    &[
        ("drop", 2.3140372355418344),
        ("nodes", 1.481292910973371),
        ("singleton", 1.5155442360736466),
    ],
    &[
        ("drop", 1.0998770078216682),
        ("with", 0.8213562238441001),
        ("singleton", 0.7203480713238173),
        ("selfloops", 0.9647817557251608),
        ("nodes", 0.7040681928557436),
    ],
    &[
        ("nodes", 1.481292910973371),
        ("drop", 2.3140372355418344),
        ("disconnected", 2.598264873244137),
    ],
    &[
        ("drop", 3.709988916595787),
        ("selfloops", 3.2542998855509446),
    ],
    &[
        ("edges", 1.8425936389723871),
        ("drop", 2.3140372355418344),
        ("parallel", 2.925314276142877),
    ],
];

#[pymethods]
impl Graph {
    fn _repr_html_(&self) -> String {
        self.__repr__()
    }
}

#[pyproto]
impl PyObjectProtocol for Graph {
    fn __str__(&'p self) -> String {
        self.inner.to_string()
    }
    fn __repr__(&'p self) -> String {
        self.__str__()
    }

    fn __hash__(&'p self) -> PyResult<isize> {
        let mut hasher = DefaultHasher::new();
        self.inner.hash(&mut hasher);
        Ok(hasher.finish() as isize)
    }

    fn __getattr__(&self, name: String) -> PyResult<()> {
        // split the query into tokens
        let tokens = split_words(&name);

        // compute the similarities between all the terms and tokens
        let tokens_expanded = tokens
            .iter()
            .map(|token| {
                let mut similarities = GRAPH_TERMS
                    .iter()
                    .map(move |term| (*term, jaro_winkler(token, term) as f64))
                    .collect::<Vec<(&str, f64)>>();

                similarities.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());

                similarities.into_iter().take(1)
            })
            .flatten()
            .collect::<Vec<(&str, f64)>>();

        // Compute the weighted ranking of each method ("document")
        // where the conribution of each term is weighted by it's similarity
        // with the query tokens
        let mut doc_scores = GRAPH_TFIDF_FREQUENCIES
            .par_iter()
            .enumerate()
            // for each document
            .map(|(id, frequencies_doc)| {
                (
                    id,
                    (jaro_winkler(&name, GRAPH_METHODS_NAMES[id]).exp() - 1.0)
                        * frequencies_doc
                            .iter()
                            .map(|(term, weight)| {
                                match tokens_expanded.iter().find(|(token, _)| token == term) {
                                    Some((_, similarity)) => (similarity.exp() - 1.0) * weight,
                                    None => 0.0,
                                }
                            })
                            .sum::<f64>(),
                )
            })
            .collect::<Vec<(usize, f64)>>();

        // sort the scores in a decreasing order
        doc_scores.sort_by(|(_, d1), (_, d2)| d2.partial_cmp(d1).unwrap());

        Err(PyAttributeError::new_err(format!(
            "The method '{}' does not exists, did you mean one of the following?\n{}",
            &name,
            doc_scores
                .iter()
                .map(|(method_id, _)| {
                    format!("* '{}'", GRAPH_METHODS_NAMES[*method_id].to_string())
                })
                .take(10)
                .collect::<Vec<String>>()
                .join("\n"),
        )))
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct ShortestPathsDjkstra {
    pub inner: graph::ShortestPathsDjkstra,
}

impl From<graph::ShortestPathsDjkstra> for ShortestPathsDjkstra {
    fn from(val: graph::ShortestPathsDjkstra) -> ShortestPathsDjkstra {
        ShortestPathsDjkstra { inner: val }
    }
}

impl From<ShortestPathsDjkstra> for graph::ShortestPathsDjkstra {
    fn from(val: ShortestPathsDjkstra) -> graph::ShortestPathsDjkstra {
        val.inner
    }
}

#[pymethods]
impl ShortestPathsDjkstra {
    #[automatically_generated_binding]
    #[text_signature = "($self, node_id)"]
    ///
    pub fn has_path_to_node_id(&self, node_id: NodeT) -> PyResult<bool> {
        Ok(pe!(self.inner.has_path_to_node_id(node_id.into()))?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_id)"]
    ///
    pub fn get_distance_from_node_id(&self, node_id: NodeT) -> PyResult<f64> {
        Ok(pe!(self.inner.get_distance_from_node_id(node_id.into()))?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_id)"]
    ///
    pub fn get_parent_from_node_id(&self, node_id: NodeT) -> PyResult<Option<NodeT>> {
        Ok(pe!(self.inner.get_parent_from_node_id(node_id.into()))?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, dst_node_id, distance)"]
    /// Returns node at just before given distance on minimum path to given destination node.
    ///
    /// Parameters
    /// ----------
    /// dst_node_id: int,
    ///     The node to start computing predecessors from.
    /// distance: float,
    ///     The distance to aim for.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the predecessors vector was not requested.
    ///
    pub fn get_point_at_given_distance_on_shortest_path(
        &self,
        dst_node_id: NodeT,
        distance: f64,
    ) -> PyResult<NodeT> {
        Ok(pe!(self
            .inner
            .get_point_at_given_distance_on_shortest_path(dst_node_id.into(), distance.into()))?
        .into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, dst_node_id)"]
    ///
    pub fn get_median_point(&self, dst_node_id: NodeT) -> PyResult<NodeT> {
        Ok(pe!(self.inner.get_median_point(dst_node_id.into()))?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    ///
    pub fn get_eccentricity(&self) -> f64 {
        self.inner.get_eccentricity().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    ///
    pub fn get_most_distant_node(&self) -> NodeT {
        self.inner.get_most_distant_node().into()
    }
}

pub const SHORTESTPATHSDJKSTRA_METHODS_NAMES: &[&str] = &[
    "has_path_to_node_id",
    "get_distance_from_node_id",
    "get_parent_from_node_id",
    "get_point_at_given_distance_on_shortest_path",
    "get_median_point",
    "get_eccentricity",
    "get_most_distant_node",
];

pub const SHORTESTPATHSDJKSTRA_TERMS: &[&str] = &[
    "get",
    "from",
    "path",
    "distant",
    "to",
    "distance",
    "node",
    "on",
    "node_id",
    "median",
    "at",
    "has",
    "eccentricity",
    "point",
    "most",
    "shortest",
    "parent",
    "given",
];

pub const SHORTESTPATHSDJKSTRA_TFIDF_FREQUENCIES: &[&[(&str, f64)]] = &[
    &[
        ("path", 0.42482838141517315),
        ("to", 0.6114019719594267),
        ("node_id", 0.3019354990220349),
        ("has", 0.6114019719594267),
    ],
    &[
        ("get", 0.07583805514570643),
        ("node_id", 0.3019354990220349),
        ("from", 0.42482838141517315),
        ("distance", 0.42482838141517315),
    ],
    &[
        ("parent", 0.6114019719594267),
        ("get", 0.07583805514570643),
        ("from", 0.42482838141517315),
        ("node_id", 0.3019354990220349),
    ],
    &[
        ("at", 0.1957472442483003),
        ("point", 0.13601360275953528),
        ("on", 0.1957472442483003),
        ("distance", 0.13601360275953528),
        ("given", 0.1957472442483003),
        ("shortest", 0.1957472442483003),
        ("get", 0.0242804095910044),
        ("path", 0.13601360275953528),
    ],
    &[
        ("get", 0.11361399204847342),
        ("point", 0.6364410091389575),
        ("median", 0.9159493693128016),
    ],
    &[
        ("get", 0.18302557989571702),
        ("eccentricity", 1.475541537190835),
    ],
    &[
        ("most", 0.6114019719594267),
        ("node", 0.6114019719594267),
        ("get", 0.07583805514570643),
        ("distant", 0.6114019719594267),
    ],
];

#[pymethods]
impl ShortestPathsDjkstra {
    fn _repr_html_(&self) -> String {
        self.__repr__()
    }
}

#[pyproto]
impl PyObjectProtocol for ShortestPathsDjkstra {
    fn __str__(&'p self) -> String {
        self.inner.to_string()
    }
    fn __repr__(&'p self) -> String {
        self.__str__()
    }

    fn __hash__(&'p self) -> PyResult<isize> {
        let mut hasher = DefaultHasher::new();
        self.inner.hash(&mut hasher);
        Ok(hasher.finish() as isize)
    }

    fn __getattr__(&self, name: String) -> PyResult<()> {
        // split the query into tokens
        let tokens = split_words(&name);

        // compute the similarities between all the terms and tokens
        let tokens_expanded = tokens
            .iter()
            .map(|token| {
                let mut similarities = SHORTESTPATHSDJKSTRA_TERMS
                    .iter()
                    .map(move |term| (*term, jaro_winkler(token, term) as f64))
                    .collect::<Vec<(&str, f64)>>();

                similarities.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());

                similarities.into_iter().take(1)
            })
            .flatten()
            .collect::<Vec<(&str, f64)>>();

        // Compute the weighted ranking of each method ("document")
        // where the conribution of each term is weighted by it's similarity
        // with the query tokens
        let mut doc_scores = SHORTESTPATHSDJKSTRA_TFIDF_FREQUENCIES
            .par_iter()
            .enumerate()
            // for each document
            .map(|(id, frequencies_doc)| {
                (
                    id,
                    (jaro_winkler(&name, SHORTESTPATHSDJKSTRA_METHODS_NAMES[id]).exp() - 1.0)
                        * frequencies_doc
                            .iter()
                            .map(|(term, weight)| {
                                match tokens_expanded.iter().find(|(token, _)| token == term) {
                                    Some((_, similarity)) => (similarity.exp() - 1.0) * weight,
                                    None => 0.0,
                                }
                            })
                            .sum::<f64>(),
                )
            })
            .collect::<Vec<(usize, f64)>>();

        // sort the scores in a decreasing order
        doc_scores.sort_by(|(_, d1), (_, d2)| d2.partial_cmp(d1).unwrap());

        Err(PyAttributeError::new_err(format!(
            "The method '{}' does not exists, did you mean one of the following?\n{}",
            &name,
            doc_scores
                .iter()
                .map(|(method_id, _)| {
                    format!(
                        "* '{}'",
                        SHORTESTPATHSDJKSTRA_METHODS_NAMES[*method_id].to_string()
                    )
                })
                .take(10)
                .collect::<Vec<String>>()
                .join("\n"),
        )))
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct ShortestPathsResultBFS {
    pub inner: graph::ShortestPathsResultBFS,
}

impl From<graph::ShortestPathsResultBFS> for ShortestPathsResultBFS {
    fn from(val: graph::ShortestPathsResultBFS) -> ShortestPathsResultBFS {
        ShortestPathsResultBFS { inner: val }
    }
}

impl From<ShortestPathsResultBFS> for graph::ShortestPathsResultBFS {
    fn from(val: ShortestPathsResultBFS) -> graph::ShortestPathsResultBFS {
        val.inner
    }
}

#[pymethods]
impl ShortestPathsResultBFS {
    #[automatically_generated_binding]
    #[text_signature = "($self, node_id)"]
    ///
    pub fn has_path_to_node_id(&self, node_id: NodeT) -> PyResult<bool> {
        Ok(pe!(self.inner.has_path_to_node_id(node_id.into()))?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_id)"]
    ///
    pub fn get_distance_from_node_id(&self, node_id: NodeT) -> PyResult<NodeT> {
        Ok(pe!(self.inner.get_distance_from_node_id(node_id.into()))?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_id)"]
    ///
    pub fn get_parent_from_node_id(&self, node_id: NodeT) -> PyResult<NodeT> {
        Ok(pe!(self.inner.get_parent_from_node_id(node_id.into()))?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, dst_node_id, k)"]
    /// Returns node at the `len - k` position on minimum path to given destination node.
    ///
    /// Parameters
    /// ----------
    /// dst_node_id: int,
    ///     The node to start computing predecessors from.
    /// k: int,
    ///     Steps to go back.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the predecessors vector was not requested.
    ///
    pub fn get_kth_point_on_shortest_path(&self, dst_node_id: NodeT, k: NodeT) -> PyResult<NodeT> {
        Ok(pe!(self
            .inner
            .get_kth_point_on_shortest_path(dst_node_id.into(), k.into()))?
        .into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, dst_node_id)"]
    ///
    pub fn get_median_point(&self, dst_node_id: NodeT) -> PyResult<NodeT> {
        Ok(pe!(self.inner.get_median_point(dst_node_id.into()))?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    ///
    pub fn get_eccentricity(&self) -> NodeT {
        self.inner.get_eccentricity().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    ///
    pub fn get_most_distant_node(&self) -> NodeT {
        self.inner.get_most_distant_node().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    ///
    pub fn get_distances(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_distances(), NodeT)
    }
}

pub const SHORTESTPATHSRESULTBFS_METHODS_NAMES: &[&str] = &[
    "has_path_to_node_id",
    "get_distance_from_node_id",
    "get_parent_from_node_id",
    "get_kth_point_on_shortest_path",
    "get_median_point",
    "get_eccentricity",
    "get_most_distant_node",
    "get_distances",
];

pub const SHORTESTPATHSRESULTBFS_TERMS: &[&str] = &[
    "most",
    "kth",
    "node",
    "path",
    "distance",
    "node_id",
    "get",
    "distant",
    "point",
    "parent",
    "shortest",
    "on",
    "distances",
    "from",
    "to",
    "eccentricity",
    "median",
    "has",
];

pub const SHORTESTPATHSRESULTBFS_TFIDF_FREQUENCIES: &[&[(&str, f64)]] = &[
    &[
        ("node_id", 0.3162746727065207),
        ("to", 0.6000118314966928),
        ("path", 0.4289501330069269),
        ("has", 0.6000118314966928),
    ],
    &[
        ("distance", 0.6000118314966928),
        ("from", 0.4289501330069269),
        ("node_id", 0.3162746727065207),
        ("get", 0.06105456289866842),
    ],
    &[
        ("get", 0.06105456289866842),
        ("parent", 0.6000118314966928),
        ("node_id", 0.3162746727065207),
        ("from", 0.4289501330069269),
    ],
    &[
        ("point", 0.22203874189121262),
        ("kth", 0.3105859211453293),
        ("path", 0.22203874189121262),
        ("on", 0.3105859211453293),
        ("get", 0.03160385622847988),
        ("shortest", 0.3105859211453293),
    ],
    &[
        ("point", 0.6511320160981572),
        ("median", 0.9107979773464258),
        ("get", 0.09267879311173854),
    ],
    &[
        ("get", 0.15237248262318975),
        ("eccentricity", 1.4974358676545705),
    ],
    &[
        ("distant", 0.6000118314966928),
        ("get", 0.06105456289866842),
        ("node", 0.6000118314966928),
        ("most", 0.6000118314966928),
    ],
    &[
        ("get", 0.15237248262318975),
        ("distances", 1.4974358676545705),
    ],
];

#[pymethods]
impl ShortestPathsResultBFS {
    fn _repr_html_(&self) -> String {
        self.__repr__()
    }
}

#[pyproto]
impl PyObjectProtocol for ShortestPathsResultBFS {
    fn __str__(&'p self) -> String {
        self.inner.to_string()
    }
    fn __repr__(&'p self) -> String {
        self.__str__()
    }

    fn __hash__(&'p self) -> PyResult<isize> {
        let mut hasher = DefaultHasher::new();
        self.inner.hash(&mut hasher);
        Ok(hasher.finish() as isize)
    }

    fn __getattr__(&self, name: String) -> PyResult<()> {
        // split the query into tokens
        let tokens = split_words(&name);

        // compute the similarities between all the terms and tokens
        let tokens_expanded = tokens
            .iter()
            .map(|token| {
                let mut similarities = SHORTESTPATHSRESULTBFS_TERMS
                    .iter()
                    .map(move |term| (*term, jaro_winkler(token, term) as f64))
                    .collect::<Vec<(&str, f64)>>();

                similarities.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());

                similarities.into_iter().take(1)
            })
            .flatten()
            .collect::<Vec<(&str, f64)>>();

        // Compute the weighted ranking of each method ("document")
        // where the conribution of each term is weighted by it's similarity
        // with the query tokens
        let mut doc_scores = SHORTESTPATHSRESULTBFS_TFIDF_FREQUENCIES
            .par_iter()
            .enumerate()
            // for each document
            .map(|(id, frequencies_doc)| {
                (
                    id,
                    (jaro_winkler(&name, SHORTESTPATHSRESULTBFS_METHODS_NAMES[id]).exp() - 1.0)
                        * frequencies_doc
                            .iter()
                            .map(|(term, weight)| {
                                match tokens_expanded.iter().find(|(token, _)| token == term) {
                                    Some((_, similarity)) => (similarity.exp() - 1.0) * weight,
                                    None => 0.0,
                                }
                            })
                            .sum::<f64>(),
                )
            })
            .collect::<Vec<(usize, f64)>>();

        // sort the scores in a decreasing order
        doc_scores.sort_by(|(_, d1), (_, d2)| d2.partial_cmp(d1).unwrap());

        Err(PyAttributeError::new_err(format!(
            "The method '{}' does not exists, did you mean one of the following?\n{}",
            &name,
            doc_scores
                .iter()
                .map(|(method_id, _)| {
                    format!(
                        "* '{}'",
                        SHORTESTPATHSRESULTBFS_METHODS_NAMES[*method_id].to_string()
                    )
                })
                .take(10)
                .collect::<Vec<String>>()
                .join("\n"),
        )))
    }
}

#[pymodule]
fn edge_list_utils(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(add_numeric_id_to_csv))?;
    m.add_wrapped(wrap_pyfunction!(convert_node_list_node_types_to_numeric))?;
    m.add_wrapped(wrap_pyfunction!(is_numeric_edge_list))?;
    m.add_wrapped(wrap_pyfunction!(get_selfloops_number_from_edge_list))?;
    m.add_wrapped(wrap_pyfunction!(are_there_selfloops_in_edge_list))?;
    m.add_wrapped(wrap_pyfunction!(sort_numeric_edge_list))?;
    m.add_wrapped(wrap_pyfunction!(sort_numeric_edge_list_inplace))?;
    m.add_wrapped(wrap_pyfunction!(convert_directed_edge_list_to_undirected))?;
    m.add_wrapped(wrap_pyfunction!(build_optimal_lists_files))?;
    m.add_wrapped(wrap_pyfunction!(get_minmax_node_from_numeric_edge_list))?;
    m.add_wrapped(wrap_pyfunction!(get_rows_number))?;
    m.add_wrapped(wrap_pyfunction!(filter_duplicates_from_edge_list))?;
    m.add_wrapped(wrap_pyfunction!(convert_undirected_edge_list_to_directed))?;
    m.add_wrapped(wrap_pyfunction!(convert_edge_list_to_numeric))?;
    m.add_wrapped(wrap_pyfunction!(densify_sparse_numeric_edge_list))?;
    Ok(())
}

#[pyfunction]
#[automatically_generated_binding]
#[text_signature = "(original_csv_path, original_csv_separator, original_csv_header, target_csv_path, target_csv_separator, target_csv_header, target_csv_ids_column, target_csv_ids_column_number, comment_symbol, max_rows_number, rows_to_skip, lines_number, verbose)"]
/// Create a new CSV with the lines number added to it.
///
/// Parameters
/// ----------
/// original_csv_path: str,
///     The path from where to load the original CSV.
/// original_csv_separator: Optional[str],
///     Separator to use for the original CSV.
/// original_csv_header: Optional[bool],
///     Whether the original CSV has an header.
/// target_csv_path: str,
///     The path from where to load the target CSV. This cannot be the same as the original CSV.
/// target_csv_separator: Optional[str],
///     Separator to use for the target CSV. If None, the one provided from the original CSV will be used.
/// target_csv_header: Optional[bool],
///     Whether the target CSV has an header. If None, the one provided from the original CSV will be used.
/// target_csv_ids_column: Optional[str],
///     The column name to use for the ids in the target list.
/// target_csv_ids_column_number: Optional[int],
///     The column number to use for the ids in the target list.
/// comment_symbol: Optional[str],
///     The comment symbol to use within the original CSV.
/// max_rows_number: Optional[int],
///     The amount of rows to load from the original CSV.
/// rows_to_skip: Optional[int],
///     The amount of rows to skip from the original CSV.
/// verbose: Optional[bool],
///     Whether to show the loading bar while processing the file.
///
///
/// Raises
/// -------
/// ValueError
///     If there are problems with opening the original or target file.
/// ValueError
///     If the original and target paths are identical.
///
pub fn add_numeric_id_to_csv(
    original_csv_path: &str,
    original_csv_separator: Option<String>,
    original_csv_header: Option<bool>,
    target_csv_path: &str,
    target_csv_separator: Option<String>,
    target_csv_header: Option<bool>,
    target_csv_ids_column: Option<String>,
    target_csv_ids_column_number: Option<usize>,
    comment_symbol: Option<String>,
    max_rows_number: Option<usize>,
    rows_to_skip: Option<usize>,
    lines_number: Option<usize>,
    verbose: Option<bool>,
) -> PyResult<usize> {
    Ok(pe!(graph::add_numeric_id_to_csv(
        original_csv_path.into(),
        original_csv_separator.into(),
        original_csv_header.into(),
        target_csv_path.into(),
        target_csv_separator.into(),
        target_csv_header.into(),
        target_csv_ids_column.into(),
        target_csv_ids_column_number.into(),
        comment_symbol.into(),
        max_rows_number.into(),
        rows_to_skip.into(),
        lines_number.into(),
        verbose.into()
    ))?
    .into())
}

#[pyfunction]
#[automatically_generated_binding]
#[text_signature = "(original_node_type_path, original_node_type_list_separator, original_node_types_column_number, original_node_types_column, node_types_number, original_numeric_node_type_ids, original_minimum_node_type_id, original_node_type_list_header, original_node_type_list_rows_to_skip, original_node_type_list_is_correct, original_node_type_list_max_rows_number, original_node_type_list_comment_symbol, original_load_node_type_list_in_parallel, target_node_type_list_path, target_node_type_list_separator, target_node_type_list_header, target_node_type_list_node_types_column, target_node_type_list_node_types_column_number, original_node_path, original_node_list_separator, original_node_list_header, node_list_rows_to_skip, node_list_max_rows_number, node_list_comment_symbol, default_node_type, original_nodes_column_number, original_nodes_column, original_node_types_separator, original_node_list_node_types_column_number, original_node_list_node_types_column, original_minimum_node_id, original_numeric_node_ids, original_node_list_numeric_node_type_ids, original_skip_node_types_if_unavailable, target_node_path, target_node_list_separator, target_node_list_header, target_nodes_column_number, target_nodes_column, target_node_types_separator, target_node_list_node_types_column_number, target_node_list_node_types_column, nodes_number)"]
///
pub fn convert_node_list_node_types_to_numeric(
    original_node_type_path: Option<String>,
    original_node_type_list_separator: Option<String>,
    original_node_types_column_number: Option<usize>,
    original_node_types_column: Option<String>,
    node_types_number: Option<NodeTypeT>,
    original_numeric_node_type_ids: Option<bool>,
    original_minimum_node_type_id: Option<NodeTypeT>,
    original_node_type_list_header: Option<bool>,
    original_node_type_list_rows_to_skip: Option<usize>,
    original_node_type_list_is_correct: Option<bool>,
    original_node_type_list_max_rows_number: Option<usize>,
    original_node_type_list_comment_symbol: Option<String>,
    original_load_node_type_list_in_parallel: Option<bool>,
    target_node_type_list_path: Option<String>,
    target_node_type_list_separator: Option<String>,
    target_node_type_list_header: Option<bool>,
    target_node_type_list_node_types_column: Option<String>,
    target_node_type_list_node_types_column_number: Option<usize>,
    original_node_path: String,
    original_node_list_separator: Option<String>,
    original_node_list_header: Option<bool>,
    node_list_rows_to_skip: Option<usize>,
    node_list_max_rows_number: Option<usize>,
    node_list_comment_symbol: Option<String>,
    default_node_type: Option<String>,
    original_nodes_column_number: Option<usize>,
    original_nodes_column: Option<String>,
    original_node_types_separator: Option<String>,
    original_node_list_node_types_column_number: Option<usize>,
    original_node_list_node_types_column: Option<String>,
    original_minimum_node_id: Option<NodeT>,
    original_numeric_node_ids: Option<bool>,
    original_node_list_numeric_node_type_ids: Option<bool>,
    original_skip_node_types_if_unavailable: Option<bool>,
    target_node_path: String,
    target_node_list_separator: Option<String>,
    target_node_list_header: Option<bool>,
    target_nodes_column_number: Option<usize>,
    target_nodes_column: Option<String>,
    target_node_types_separator: Option<String>,
    target_node_list_node_types_column_number: Option<usize>,
    target_node_list_node_types_column: Option<String>,
    nodes_number: Option<NodeT>,
) -> PyResult<(NodeT, Option<NodeTypeT>)> {
    Ok(pe!(graph::convert_node_list_node_types_to_numeric(
        original_node_type_path.into(),
        original_node_type_list_separator.into(),
        original_node_types_column_number.into(),
        original_node_types_column.into(),
        node_types_number.into(),
        original_numeric_node_type_ids.into(),
        original_minimum_node_type_id.into(),
        original_node_type_list_header.into(),
        original_node_type_list_rows_to_skip.into(),
        original_node_type_list_is_correct.into(),
        original_node_type_list_max_rows_number.into(),
        original_node_type_list_comment_symbol.into(),
        original_load_node_type_list_in_parallel.into(),
        target_node_type_list_path.into(),
        target_node_type_list_separator.into(),
        target_node_type_list_header.into(),
        target_node_type_list_node_types_column.into(),
        target_node_type_list_node_types_column_number.into(),
        original_node_path.into(),
        original_node_list_separator.into(),
        original_node_list_header.into(),
        node_list_rows_to_skip.into(),
        node_list_max_rows_number.into(),
        node_list_comment_symbol.into(),
        default_node_type.into(),
        original_nodes_column_number.into(),
        original_nodes_column.into(),
        original_node_types_separator.into(),
        original_node_list_node_types_column_number.into(),
        original_node_list_node_types_column.into(),
        original_minimum_node_id.into(),
        original_numeric_node_ids.into(),
        original_node_list_numeric_node_type_ids.into(),
        original_skip_node_types_if_unavailable.into(),
        target_node_path.into(),
        target_node_list_separator.into(),
        target_node_list_header.into(),
        target_nodes_column_number.into(),
        target_nodes_column.into(),
        target_node_types_separator.into(),
        target_node_list_node_types_column_number.into(),
        target_node_list_node_types_column.into(),
        nodes_number.into()
    ))?
    .into())
}

#[pyfunction]
#[automatically_generated_binding]
#[text_signature = "(path, separator, header, sources_column, sources_column_number, destinations_column, destinations_column_number, comment_symbol, max_rows_number, rows_to_skip, edges_number, load_edge_list_in_parallel, verbose, name)"]
/// Return number of selfloops in the given edge list.
///
/// Parameters
/// ----------
/// path: str,
///     The path from where to load the edge list.
/// separator: Optional[str],
///     The separator for the rows in the edge list.
/// header: Optional[bool],
///     Whether the edge list has an header.
/// sources_column: Optional[str],
///     The column name to use for the source nodes.
/// sources_column_number: Optional[int],
///     The column number to use for the source nodes.
/// destinations_column: Optional[str],
///     The column name to use for the destination nodes.
/// destinations_column_number: Optional[int],
///     The column number to use for the destination nodes.
/// comment_symbol: Optional[str],
///     The comment symbol to use for the lines to skip.
/// max_rows_number: Optional[int],
///     The number of rows to read at most. Note that this parameter is ignored when reading in parallel.
/// rows_to_skip: Optional[int],
///     Number of rows to skip in the edge list.
/// edges_number: Optional[int],
///     Number of edges in the edge list.
/// load_edge_list_in_parallel: Optional[bool],
///     Whether to execute the task in parallel or sequential. Generally, parallel is preferable.
/// verbose: Optional[bool],
///     Whether to show the loading bar while processing the file.
/// name: Optional[str],
///     The name of the graph to display in the loading bar.
///
pub fn is_numeric_edge_list(
    path: &str,
    separator: Option<String>,
    header: Option<bool>,
    sources_column: Option<String>,
    sources_column_number: Option<usize>,
    destinations_column: Option<String>,
    destinations_column_number: Option<usize>,
    comment_symbol: Option<String>,
    max_rows_number: Option<usize>,
    rows_to_skip: Option<usize>,
    edges_number: Option<EdgeT>,
    load_edge_list_in_parallel: Option<bool>,
    verbose: Option<bool>,
    name: Option<String>,
) -> PyResult<bool> {
    Ok(pe!(graph::is_numeric_edge_list(
        path.into(),
        separator.into(),
        header.into(),
        sources_column.into(),
        sources_column_number.into(),
        destinations_column.into(),
        destinations_column_number.into(),
        comment_symbol.into(),
        max_rows_number.into(),
        rows_to_skip.into(),
        edges_number.into(),
        load_edge_list_in_parallel.into(),
        verbose.into(),
        name.into()
    ))?
    .into())
}

#[pyfunction]
#[automatically_generated_binding]
#[text_signature = "(path, separator, header, sources_column, sources_column_number, destinations_column, destinations_column_number, comment_symbol, max_rows_number, rows_to_skip, edges_number, load_edge_list_in_parallel, verbose, name)"]
/// Return number of selfloops in the given edge list.
///
/// Parameters
/// ----------
/// path: str,
///     The path from where to load the edge list.
/// separator: Optional[str],
///     The separator for the rows in the edge list.
/// header: Optional[bool],
///     Whether the edge list has an header.
/// sources_column: Optional[str],
///     The column name to use for the source nodes.
/// sources_column_number: Optional[int],
///     The column number to use for the source nodes.
/// destinations_column: Optional[str],
///     The column name to use for the destination nodes.
/// destinations_column_number: Optional[int],
///     The column number to use for the destination nodes.
/// comment_symbol: Optional[str],
///     The comment symbol to use for the lines to skip.
/// max_rows_number: Optional[int],
///     The number of rows to read at most. Note that this parameter is ignored when reading in parallel.
/// rows_to_skip: Optional[int],
///     Number of rows to skip in the edge list.
/// edges_number: Optional[int],
///     Number of edges in the edge list.
/// load_edge_list_in_parallel: Optional[bool],
///     Whether to execute the task in parallel or sequential. Generally, parallel is preferable.
/// verbose: Optional[bool],
///     Whether to show the loading bar while processing the file.
/// name: Optional[str],
///     The name of the graph to display in the loading bar.
///
pub fn get_selfloops_number_from_edge_list(
    path: &str,
    separator: Option<String>,
    header: Option<bool>,
    sources_column: Option<String>,
    sources_column_number: Option<usize>,
    destinations_column: Option<String>,
    destinations_column_number: Option<usize>,
    comment_symbol: Option<String>,
    max_rows_number: Option<usize>,
    rows_to_skip: Option<usize>,
    edges_number: Option<EdgeT>,
    load_edge_list_in_parallel: Option<bool>,
    verbose: Option<bool>,
    name: Option<String>,
) -> PyResult<EdgeT> {
    Ok(pe!(graph::get_selfloops_number_from_edge_list(
        path.into(),
        separator.into(),
        header.into(),
        sources_column.into(),
        sources_column_number.into(),
        destinations_column.into(),
        destinations_column_number.into(),
        comment_symbol.into(),
        max_rows_number.into(),
        rows_to_skip.into(),
        edges_number.into(),
        load_edge_list_in_parallel.into(),
        verbose.into(),
        name.into()
    ))?
    .into())
}

#[pyfunction]
#[automatically_generated_binding]
#[text_signature = "(path, separator, header, sources_column, sources_column_number, destinations_column, destinations_column_number, comment_symbol, max_rows_number, rows_to_skip, edges_number, load_edge_list_in_parallel, verbose, name)"]
/// Return whether there are selfloops in the edge list.
///
/// Parameters
/// ----------
/// path: str,
///     The path from where to load the edge list.
/// separator: Optional[str],
///     The separator for the rows in the edge list.
/// header: Optional[bool],
///     Whether the edge list has an header.
/// sources_column: Optional[str],
///     The column name to use for the source nodes.
/// sources_column_number: Optional[int],
///     The column number to use for the source nodes.
/// destinations_column: Optional[str],
///     The column name to use for the destination nodes.
/// destinations_column_number: Optional[int],
///     The column number to use for the destination nodes.
/// comment_symbol: Optional[str],
///     The comment symbol to use for the lines to skip.
/// max_rows_number: Optional[int],
///     The number of rows to read at most. Note that this parameter is ignored when reading in parallel.
/// rows_to_skip: Optional[int],
///     Number of rows to skip in the edge list.
/// edges_number: Optional[int],
///     Number of edges in the edge list.
/// load_edge_list_in_parallel: Optional[bool],
///     Whether to execute the task in parallel or sequential. Generally, parallel is preferable.
/// verbose: Optional[bool],
///     Whether to show the loading bar while processing the file.
/// name: Optional[str],
///     The name of the graph to display in the loading bar.
///
pub fn are_there_selfloops_in_edge_list(
    path: &str,
    separator: Option<String>,
    header: Option<bool>,
    sources_column: Option<String>,
    sources_column_number: Option<usize>,
    destinations_column: Option<String>,
    destinations_column_number: Option<usize>,
    comment_symbol: Option<String>,
    max_rows_number: Option<usize>,
    rows_to_skip: Option<usize>,
    edges_number: Option<EdgeT>,
    load_edge_list_in_parallel: Option<bool>,
    verbose: Option<bool>,
    name: Option<String>,
) -> PyResult<bool> {
    Ok(pe!(graph::are_there_selfloops_in_edge_list(
        path.into(),
        separator.into(),
        header.into(),
        sources_column.into(),
        sources_column_number.into(),
        destinations_column.into(),
        destinations_column_number.into(),
        comment_symbol.into(),
        max_rows_number.into(),
        rows_to_skip.into(),
        edges_number.into(),
        load_edge_list_in_parallel.into(),
        verbose.into(),
        name.into()
    ))?
    .into())
}

#[pyfunction]
#[automatically_generated_binding]
#[text_signature = "(path, target_path, separator, header, sources_column, sources_column_number, destinations_column, destinations_column_number, edge_types_column, edge_types_column_number, rows_to_skip, skip_edge_types_if_unavailable)"]
/// Sort given numeric edge list in place using the sort command.
///
/// Parameters
/// ----------
/// path: str,
///     The path from where to load the edge list.
/// target_path: str,
///     The where to store the edge list.
/// separator: Optional[str],
///     The separator for the rows in the edge list.
/// header: Optional[bool],
///     Whether the edge list has an header.
/// sources_column: Optional[str],
///     The column name to use for the source nodes.
/// sources_column_number: Optional[int],
///     The column number to use for the source nodes.
/// destinations_column: Optional[str],
///     The column name to use for the destination nodes.
/// destinations_column_number: Optional[int],
///     The column number to use for the destination nodes.
/// edge_types_column: Optional[str],
///     The column name to use for the edge types.
/// edge_types_column_number: Optional[int],
///     The column number to use for the edge types.
/// rows_to_skip: Optional[int],
///     Number of rows to skip in the edge list.
/// skip_edge_types_if_unavailable: Optional[bool],
///     Whether to automatically skip the edge types if they are not available.
///
pub fn sort_numeric_edge_list(
    path: &str,
    target_path: &str,
    separator: Option<String>,
    header: Option<bool>,
    sources_column: Option<String>,
    sources_column_number: Option<usize>,
    destinations_column: Option<String>,
    destinations_column_number: Option<usize>,
    edge_types_column: Option<String>,
    edge_types_column_number: Option<usize>,
    rows_to_skip: Option<usize>,
    skip_edge_types_if_unavailable: Option<bool>,
) -> PyResult<()> {
    Ok(pe!(graph::sort_numeric_edge_list(
        path.into(),
        target_path.into(),
        separator.into(),
        header.into(),
        sources_column.into(),
        sources_column_number.into(),
        destinations_column.into(),
        destinations_column_number.into(),
        edge_types_column.into(),
        edge_types_column_number.into(),
        rows_to_skip.into(),
        skip_edge_types_if_unavailable.into()
    ))?
    .into())
}

#[pyfunction]
#[automatically_generated_binding]
#[text_signature = "(path, separator, header, sources_column, sources_column_number, destinations_column, destinations_column_number, edge_types_column, edge_types_column_number, rows_to_skip, skip_edge_types_if_unavailable)"]
/// Sort given numeric edge list in place using the sort command.
///
/// Parameters
/// ----------
/// path: str,
///     The path from where to load the edge list.
/// separator: Optional[str],
///     The separator for the rows in the edge list.
/// header: Optional[bool],
///     Whether the edge list has an header.
/// sources_column: Optional[str],
///     The column name to use for the source nodes.
/// sources_column_number: Optional[int],
///     The column number to use for the source nodes.
/// destinations_column: Optional[str],
///     The column name to use for the destination nodes.
/// destinations_column_number: Optional[int],
///     The column number to use for the destination nodes.
/// edge_types_column: Optional[str],
///     The column name to use for the edge types.
/// edge_types_column_number: Optional[int],
///     The column number to use for the edge types.
/// rows_to_skip: Optional[int],
///     Number of rows to skip in the edge list.
/// skip_edge_types_if_unavailable: Optional[bool],
///     Whether to automatically skip the edge types if they are not available.
///
pub fn sort_numeric_edge_list_inplace(
    path: &str,
    separator: Option<String>,
    header: Option<bool>,
    sources_column: Option<String>,
    sources_column_number: Option<usize>,
    destinations_column: Option<String>,
    destinations_column_number: Option<usize>,
    edge_types_column: Option<String>,
    edge_types_column_number: Option<usize>,
    rows_to_skip: Option<usize>,
    skip_edge_types_if_unavailable: Option<bool>,
) -> PyResult<()> {
    Ok(pe!(graph::sort_numeric_edge_list_inplace(
        path.into(),
        separator.into(),
        header.into(),
        sources_column.into(),
        sources_column_number.into(),
        destinations_column.into(),
        destinations_column_number.into(),
        edge_types_column.into(),
        edge_types_column_number.into(),
        rows_to_skip.into(),
        skip_edge_types_if_unavailable.into()
    ))?
    .into())
}

#[pyfunction]
#[automatically_generated_binding]
#[text_signature = "(original_edge_path, original_edge_list_separator, original_edge_list_header, original_sources_column, original_sources_column_number, original_destinations_column, original_destinations_column_number, original_edge_list_edge_type_column, original_edge_list_edge_type_column_number, original_weights_column, original_weights_column_number, target_edge_path, target_edge_list_separator, target_edge_list_header, target_sources_column_number, target_sources_column, target_destinations_column_number, target_destinations_column, target_edge_list_edge_type_column, target_edge_list_edge_type_column_number, target_weights_column, target_weights_column_number, comment_symbol, default_edge_type, default_weight, max_rows_number, rows_to_skip, edges_number, skip_edge_types_if_unavailable, skip_weights_if_unavailable, verbose, name)"]
/// Create a new undirected edge list from a given directed one by duplicating the undirected edges.
///
/// Parameters
/// ----------
/// original_edge_path: str,
///     The path from where to load the original edge list.
/// original_edge_list_separator: Optional[str],
///     Separator to use for the original edge list.
/// original_edge_list_header: Optional[bool],
///     Whether the original edge list has an header.
/// original_sources_column: Optional[str],
///     The column name to use to load the sources in the original edges list.
/// original_sources_column_number: Optional[int],
///     The column number to use to load the sources in the original edges list.
/// original_destinations_column: Optional[str],
///     The column name to use to load the destinations in the original edges list.
/// original_destinations_column_number: Optional[int],
///     The column number to use to load the destinations in the original edges list.
/// original_edge_list_edge_type_column: Optional[str],
///     The column name to use for the edge types in the original edges list.
/// original_edge_list_edge_type_column_number: Optional[int],
///     The column number to use for the edge types in the original edges list.
/// original_weights_column: Optional[str],
///     The column name to use for the weights in the original edges list.
/// original_weights_column_number: Optional[int],
///     The column number to use for the weights in the original edges list.
/// target_edge_path: str,
///     The path from where to load the target edge list. This must be different from the original edge list path.
/// target_edge_list_separator: Optional[str],
///     Separator to use for the target edge list. If None, the one provided from the original edge list will be used.
/// target_edge_list_header: Optional[bool],
///     Whether the target edge list has an header. If None, the one provided from the original edge list will be used.
/// target_sources_column: Optional[str],
///     The column name to use to load the sources in the target edges list. If None, the one provided from the original edge list will be used.
/// target_sources_column_number: Optional[int],
///     The column number to use to load the sources in the target edges list. If None, the one provided from the original edge list will be used.
/// target_destinations_column: Optional[str],
///     The column name to use to load the destinations in the target edges list. If None, the one provided from the original edge list will be used.
/// target_destinations_column_number: Optional[int],
///     The column number to use to load the destinations in the target edges list. If None, the one provided from the original edge list will be used.
/// target_edge_list_edge_type_column: Optional[str],
///     The column name to use for the edge types in the target edges list. If None, the one provided from the original edge list will be used.
/// target_edge_list_edge_type_column_number: Optional[int],
///     The column number to use for the edge types in the target edges list. If None, the one provided from the original edge list will be used.
/// target_weights_column: Optional[str],
///     The column name to use for the weights in the target edges list. If None, the one provided from the original edge list will be used.
/// target_weights_column_number: Optional[int],
///     The column number to use for the weights in the target edges list. If None, the one provided from the original edge list will be used.
/// comment_symbol: Optional[str],
///     The comment symbol to use within the original edge list.
/// default_edge_type: Optional[str],
///     The default edge type to use within the original edge list.
/// default_weight: Optional[float],
///     The default weight to use within the original edge list.
/// max_rows_number: Optional[int],
///     The amount of rows to load from the original edge list.
/// rows_to_skip: Optional[int],
///     The amount of rows to skip from the original edge list.
/// edges_number: Optional[int],
///     The expected number of edges. It will be used for the loading bar.
/// skip_edge_types_if_unavailable: Optional[bool],
///     Whether to automatically skip the edge types if they are not available.
/// skip_weights_if_unavailable: Optional[bool],
///     Whether to automatically skip the weights if they are not available.
/// verbose: Optional[bool],
///     Whether to show the loading bar while processing the file.
/// name: Optional[str],
///     The name of the graph to display in the loading bar.
///
///
/// Raises
/// -------
/// ValueError
///     If there are problems with opening the original or target file.
/// ValueError
///     If the original and target paths are identical.
///
pub fn convert_directed_edge_list_to_undirected(
    original_edge_path: &str,
    original_edge_list_separator: Option<String>,
    original_edge_list_header: Option<bool>,
    original_sources_column: Option<String>,
    original_sources_column_number: Option<usize>,
    original_destinations_column: Option<String>,
    original_destinations_column_number: Option<usize>,
    original_edge_list_edge_type_column: Option<String>,
    original_edge_list_edge_type_column_number: Option<usize>,
    original_weights_column: Option<String>,
    original_weights_column_number: Option<usize>,
    target_edge_path: &str,
    target_edge_list_separator: Option<String>,
    target_edge_list_header: Option<bool>,
    target_sources_column_number: Option<usize>,
    target_sources_column: Option<String>,
    target_destinations_column_number: Option<usize>,
    target_destinations_column: Option<String>,
    target_edge_list_edge_type_column: Option<String>,
    target_edge_list_edge_type_column_number: Option<usize>,
    target_weights_column: Option<String>,
    target_weights_column_number: Option<usize>,
    comment_symbol: Option<String>,
    default_edge_type: Option<String>,
    default_weight: Option<WeightT>,
    max_rows_number: Option<usize>,
    rows_to_skip: Option<usize>,
    edges_number: Option<usize>,
    skip_edge_types_if_unavailable: Option<bool>,
    skip_weights_if_unavailable: Option<bool>,
    verbose: Option<bool>,
    name: Option<String>,
) -> PyResult<EdgeT> {
    Ok(pe!(graph::convert_directed_edge_list_to_undirected(
        original_edge_path.into(),
        original_edge_list_separator.into(),
        original_edge_list_header.into(),
        original_sources_column.into(),
        original_sources_column_number.into(),
        original_destinations_column.into(),
        original_destinations_column_number.into(),
        original_edge_list_edge_type_column.into(),
        original_edge_list_edge_type_column_number.into(),
        original_weights_column.into(),
        original_weights_column_number.into(),
        target_edge_path.into(),
        target_edge_list_separator.into(),
        target_edge_list_header.into(),
        target_sources_column_number.into(),
        target_sources_column.into(),
        target_destinations_column_number.into(),
        target_destinations_column.into(),
        target_edge_list_edge_type_column.into(),
        target_edge_list_edge_type_column_number.into(),
        target_weights_column.into(),
        target_weights_column_number.into(),
        comment_symbol.into(),
        default_edge_type.into(),
        default_weight.into(),
        max_rows_number.into(),
        rows_to_skip.into(),
        edges_number.into(),
        skip_edge_types_if_unavailable.into(),
        skip_weights_if_unavailable.into(),
        verbose.into(),
        name.into()
    ))?
    .into())
}

#[pyfunction]
#[automatically_generated_binding]
#[text_signature = "(original_node_type_path, original_node_type_list_separator, original_node_types_column_number, original_node_types_column, original_numeric_node_type_ids, original_minimum_node_type_id, original_node_type_list_header, original_node_type_list_rows_to_skip, original_node_type_list_max_rows_number, original_node_type_list_comment_symbol, original_load_node_type_list_in_parallel, original_node_type_list_is_correct, node_types_number, target_node_type_list_path, target_node_type_list_separator, target_node_type_list_node_types_column_number, target_node_type_list_node_types_column, target_node_type_list_header, original_node_path, original_node_list_separator, original_node_list_header, node_list_rows_to_skip, node_list_is_correct, node_list_max_rows_number, node_list_comment_symbol, default_node_type, original_nodes_column_number, original_nodes_column, original_node_types_separator, original_node_list_node_types_column_number, original_node_list_node_types_column, nodes_number, original_minimum_node_id, original_numeric_node_ids, original_node_list_numeric_node_type_ids, original_skip_node_types_if_unavailable, original_load_node_list_in_parallel, maximum_node_id, target_node_path, target_node_list_separator, target_node_list_header, target_nodes_column, target_nodes_column_number, target_node_types_separator, target_node_list_node_types_column, target_node_list_node_types_column_number, original_edge_type_path, original_edge_type_list_separator, original_edge_types_column_number, original_edge_types_column, original_numeric_edge_type_ids, original_minimum_edge_type_id, original_edge_type_list_header, edge_type_list_rows_to_skip, edge_type_list_max_rows_number, edge_type_list_comment_symbol, load_edge_type_list_in_parallel, edge_type_list_is_correct, edge_types_number, target_edge_type_list_path, target_edge_type_list_separator, target_edge_type_list_edge_types_column_number, target_edge_type_list_edge_types_column, target_edge_type_list_header, original_edge_path, original_edge_list_separator, original_edge_list_header, original_sources_column_number, original_sources_column, original_destinations_column_number, original_destinations_column, original_edge_list_edge_types_column_number, original_edge_list_edge_types_column, default_edge_type, original_weights_column_number, original_weights_column, default_weight, original_edge_list_numeric_node_ids, skip_weights_if_unavailable, skip_edge_types_if_unavailable, edge_list_comment_symbol, edge_list_max_rows_number, edge_list_rows_to_skip, load_edge_list_in_parallel, edges_number, target_edge_path, target_edge_list_separator, verbose, directed, name)"]
/// TODO: write the docstrin
pub fn build_optimal_lists_files(
    original_node_type_path: Option<String>,
    original_node_type_list_separator: Option<String>,
    original_node_types_column_number: Option<usize>,
    original_node_types_column: Option<String>,
    original_numeric_node_type_ids: Option<bool>,
    original_minimum_node_type_id: Option<NodeTypeT>,
    original_node_type_list_header: Option<bool>,
    original_node_type_list_rows_to_skip: Option<usize>,
    original_node_type_list_max_rows_number: Option<usize>,
    original_node_type_list_comment_symbol: Option<String>,
    original_load_node_type_list_in_parallel: Option<bool>,
    original_node_type_list_is_correct: Option<bool>,
    node_types_number: Option<NodeTypeT>,
    target_node_type_list_path: Option<String>,
    target_node_type_list_separator: Option<String>,
    target_node_type_list_node_types_column_number: Option<usize>,
    target_node_type_list_node_types_column: Option<String>,
    target_node_type_list_header: Option<bool>,
    original_node_path: Option<String>,
    original_node_list_separator: Option<String>,
    original_node_list_header: Option<bool>,
    node_list_rows_to_skip: Option<usize>,
    node_list_is_correct: Option<bool>,
    node_list_max_rows_number: Option<usize>,
    node_list_comment_symbol: Option<String>,
    default_node_type: Option<String>,
    original_nodes_column_number: Option<usize>,
    original_nodes_column: Option<String>,
    original_node_types_separator: Option<String>,
    original_node_list_node_types_column_number: Option<usize>,
    original_node_list_node_types_column: Option<String>,
    nodes_number: Option<NodeT>,
    original_minimum_node_id: Option<NodeT>,
    original_numeric_node_ids: Option<bool>,
    original_node_list_numeric_node_type_ids: Option<bool>,
    original_skip_node_types_if_unavailable: Option<bool>,
    original_load_node_list_in_parallel: Option<bool>,
    maximum_node_id: Option<EdgeT>,
    target_node_path: Option<String>,
    target_node_list_separator: Option<String>,
    target_node_list_header: Option<bool>,
    target_nodes_column: Option<String>,
    target_nodes_column_number: Option<usize>,
    target_node_types_separator: Option<String>,
    target_node_list_node_types_column: Option<String>,
    target_node_list_node_types_column_number: Option<usize>,
    original_edge_type_path: Option<String>,
    original_edge_type_list_separator: Option<String>,
    original_edge_types_column_number: Option<usize>,
    original_edge_types_column: Option<String>,
    original_numeric_edge_type_ids: Option<bool>,
    original_minimum_edge_type_id: Option<EdgeTypeT>,
    original_edge_type_list_header: Option<bool>,
    edge_type_list_rows_to_skip: Option<usize>,
    edge_type_list_max_rows_number: Option<usize>,
    edge_type_list_comment_symbol: Option<String>,
    load_edge_type_list_in_parallel: Option<bool>,
    edge_type_list_is_correct: Option<bool>,
    edge_types_number: Option<NodeTypeT>,
    target_edge_type_list_path: Option<String>,
    target_edge_type_list_separator: Option<String>,
    target_edge_type_list_edge_types_column_number: Option<usize>,
    target_edge_type_list_edge_types_column: Option<String>,
    target_edge_type_list_header: Option<bool>,
    original_edge_path: String,
    original_edge_list_separator: Option<String>,
    original_edge_list_header: Option<bool>,
    original_sources_column_number: Option<usize>,
    original_sources_column: Option<String>,
    original_destinations_column_number: Option<usize>,
    original_destinations_column: Option<String>,
    original_edge_list_edge_types_column_number: Option<usize>,
    original_edge_list_edge_types_column: Option<String>,
    default_edge_type: Option<String>,
    original_weights_column_number: Option<usize>,
    original_weights_column: Option<String>,
    default_weight: Option<WeightT>,
    original_edge_list_numeric_node_ids: Option<bool>,
    skip_weights_if_unavailable: Option<bool>,
    skip_edge_types_if_unavailable: Option<bool>,
    edge_list_comment_symbol: Option<String>,
    edge_list_max_rows_number: Option<usize>,
    edge_list_rows_to_skip: Option<usize>,
    load_edge_list_in_parallel: Option<bool>,
    edges_number: Option<EdgeT>,
    target_edge_path: String,
    target_edge_list_separator: Option<String>,
    verbose: Option<bool>,
    directed: bool,
    name: Option<String>,
) -> PyResult<(Option<NodeTypeT>, NodeT, Option<EdgeTypeT>, EdgeT)> {
    Ok(pe!(graph::build_optimal_lists_files(
        original_node_type_path.into(),
        original_node_type_list_separator.into(),
        original_node_types_column_number.into(),
        original_node_types_column.into(),
        original_numeric_node_type_ids.into(),
        original_minimum_node_type_id.into(),
        original_node_type_list_header.into(),
        original_node_type_list_rows_to_skip.into(),
        original_node_type_list_max_rows_number.into(),
        original_node_type_list_comment_symbol.into(),
        original_load_node_type_list_in_parallel.into(),
        original_node_type_list_is_correct.into(),
        node_types_number.into(),
        target_node_type_list_path.into(),
        target_node_type_list_separator.into(),
        target_node_type_list_node_types_column_number.into(),
        target_node_type_list_node_types_column.into(),
        target_node_type_list_header.into(),
        original_node_path.into(),
        original_node_list_separator.into(),
        original_node_list_header.into(),
        node_list_rows_to_skip.into(),
        node_list_is_correct.into(),
        node_list_max_rows_number.into(),
        node_list_comment_symbol.into(),
        default_node_type.into(),
        original_nodes_column_number.into(),
        original_nodes_column.into(),
        original_node_types_separator.into(),
        original_node_list_node_types_column_number.into(),
        original_node_list_node_types_column.into(),
        nodes_number.into(),
        original_minimum_node_id.into(),
        original_numeric_node_ids.into(),
        original_node_list_numeric_node_type_ids.into(),
        original_skip_node_types_if_unavailable.into(),
        original_load_node_list_in_parallel.into(),
        maximum_node_id.into(),
        target_node_path.into(),
        target_node_list_separator.into(),
        target_node_list_header.into(),
        target_nodes_column.into(),
        target_nodes_column_number.into(),
        target_node_types_separator.into(),
        target_node_list_node_types_column.into(),
        target_node_list_node_types_column_number.into(),
        original_edge_type_path.into(),
        original_edge_type_list_separator.into(),
        original_edge_types_column_number.into(),
        original_edge_types_column.into(),
        original_numeric_edge_type_ids.into(),
        original_minimum_edge_type_id.into(),
        original_edge_type_list_header.into(),
        edge_type_list_rows_to_skip.into(),
        edge_type_list_max_rows_number.into(),
        edge_type_list_comment_symbol.into(),
        load_edge_type_list_in_parallel.into(),
        edge_type_list_is_correct.into(),
        edge_types_number.into(),
        target_edge_type_list_path.into(),
        target_edge_type_list_separator.into(),
        target_edge_type_list_edge_types_column_number.into(),
        target_edge_type_list_edge_types_column.into(),
        target_edge_type_list_header.into(),
        original_edge_path.into(),
        original_edge_list_separator.into(),
        original_edge_list_header.into(),
        original_sources_column_number.into(),
        original_sources_column.into(),
        original_destinations_column_number.into(),
        original_destinations_column.into(),
        original_edge_list_edge_types_column_number.into(),
        original_edge_list_edge_types_column.into(),
        default_edge_type.into(),
        original_weights_column_number.into(),
        original_weights_column.into(),
        default_weight.into(),
        original_edge_list_numeric_node_ids.into(),
        skip_weights_if_unavailable.into(),
        skip_edge_types_if_unavailable.into(),
        edge_list_comment_symbol.into(),
        edge_list_max_rows_number.into(),
        edge_list_rows_to_skip.into(),
        load_edge_list_in_parallel.into(),
        edges_number.into(),
        target_edge_path.into(),
        target_edge_list_separator.into(),
        verbose.into(),
        directed.into(),
        name.into()
    ))?
    .into())
}

#[pyfunction]
#[automatically_generated_binding]
#[text_signature = "(path, separator, header, sources_column, sources_column_number, destinations_column, destinations_column_number, comment_symbol, max_rows_number, rows_to_skip, edges_number, load_edge_list_in_parallel, verbose, name)"]
/// Return minimum and maximum node number from given numeric edge list.
///
/// Parameters
/// ----------
/// path: str,
///     The path from where to load the edge list.
/// separator: Optional[str],
///     The separator for the rows in the edge list.
/// header: Optional[bool],
///     Whether the edge list has an header.
/// sources_column: Optional[str],
///     The column name to use for the source nodes.
/// sources_column_number: Optional[int],
///     The column number to use for the source nodes.
/// destinations_column: Optional[str],
///     The column name to use for the destination nodes.
/// destinations_column_number: Optional[int],
///     The column number to use for the destination nodes.
/// comment_symbol: Optional[str],
///     The comment symbol to use for the lines to skip.
/// max_rows_number: Optional[int],
///     The number of rows to read at most. Note that this parameter is ignored when reading in parallel.
/// rows_to_skip: Optional[int],
///     Number of rows to skip in the edge list.
/// edges_number: Optional[int],
///     Number of edges in the edge list.
/// load_edge_list_in_parallel: Optional[bool],
///     Whether to execute the task in parallel or sequential. Generally, parallel is preferable.
/// verbose: Optional[bool],
///     Whether to show the loading bar while processing the file.
/// name: Optional[str],
///     The name of the graph to display in the loading bar.
///
///
/// Raises
/// -------
/// ValueError
///     If there are problems with the edge list file.
/// ValueError
///     If the elements in the edge list are not numeric.
/// ValueError
///     If the edge list is empty.
///
pub fn get_minmax_node_from_numeric_edge_list(
    path: &str,
    separator: Option<String>,
    header: Option<bool>,
    sources_column: Option<String>,
    sources_column_number: Option<usize>,
    destinations_column: Option<String>,
    destinations_column_number: Option<usize>,
    comment_symbol: Option<String>,
    max_rows_number: Option<usize>,
    rows_to_skip: Option<usize>,
    edges_number: Option<EdgeT>,
    load_edge_list_in_parallel: Option<bool>,
    verbose: Option<bool>,
    name: Option<String>,
) -> PyResult<(EdgeT, EdgeT, EdgeT)> {
    Ok(pe!(graph::get_minmax_node_from_numeric_edge_list(
        path.into(),
        separator.into(),
        header.into(),
        sources_column.into(),
        sources_column_number.into(),
        destinations_column.into(),
        destinations_column_number.into(),
        comment_symbol.into(),
        max_rows_number.into(),
        rows_to_skip.into(),
        edges_number.into(),
        load_edge_list_in_parallel.into(),
        verbose.into(),
        name.into()
    ))?
    .into())
}

#[pyfunction]
#[automatically_generated_binding]
#[text_signature = "(file_path)"]
/// Return number of rows in given CSV path.
///
/// Parameters
/// ----------
/// file_path: str,
///     The path from where to load the original CSV.
///
///
/// Raises
/// -------
/// ValueError
///     If there are problems with opening the file.
///
pub fn get_rows_number(file_path: &str) -> PyResult<usize> {
    Ok(pe!(graph::get_rows_number(file_path.into()))?.into())
}

#[pyfunction]
#[automatically_generated_binding]
#[text_signature = "(original_edge_path, original_edge_list_separator, original_edge_list_header, original_edge_list_sources_column, original_edge_list_sources_column_number, original_edge_list_destinations_column, original_edge_list_destinations_column_number, original_edge_list_edge_type_column, original_edge_list_edge_type_column_number, original_edge_list_weights_column, original_edge_list_weights_column_number, target_edge_path, target_edge_list_separator, target_edge_list_header, target_edge_list_sources_column_number, target_edge_list_sources_column, target_edge_list_destinations_column_number, target_edge_list_destinations_column, target_edge_list_edge_type_column, target_edge_list_edge_type_column_number, target_edge_list_weights_column, target_edge_list_weights_column_number, comment_symbol, default_edge_type, default_weight, max_rows_number, rows_to_skip, edges_number, skip_edge_types_if_unavailable, skip_weights_if_unavailable, verbose, name)"]
/// Create a new edge list from a given one filtering duplicates.
///
/// Parameters
/// ----------
/// original_edge_path: str,
///     The path from where to load the original edge list.
/// original_edge_list_separator: Optional[str],
///     Separator to use for the original edge list.
/// original_edge_list_header: Optional[bool],
///     Whether the original edge list has an header.
/// original_edge_list_sources_column: Optional[str],
///     The column name to use to load the sources in the original edges list.
/// original_edge_list_sources_column_number: Optional[int],
///     The column number to use to load the sources in the original edges list.
/// original_edge_list_destinations_column: Optional[str],
///     The column name to use to load the destinations in the original edges list.
/// original_edge_list_destinations_column_number: Optional[int],
///     The column number to use to load the destinations in the original edges list.
/// original_edge_list_edge_type_column: Optional[str],
///     The column name to use for the edge types in the original edges list.
/// original_edge_list_edge_type_column_number: Optional[int],
///     The column number to use for the edge types in the original edges list.
/// original_edge_list_weights_column: Optional[str],
///     The column name to use for the weights in the original edges list.
/// original_edge_list_weights_column_number: Optional[int],
///     The column number to use for the weights in the original edges list.
/// target_edge_path: str,
///     The path from where to load the target edge list.
/// target_edge_list_separator: Optional[str],
///     Separator to use for the target edge list.
/// target_edge_list_header: Optional[bool],
///     Whether the target edge list has an header.
/// target_edge_list_sources_column: Optional[str],
///     The column name to use to load the sources in the target edges list.
/// target_edge_list_sources_column_number: Optional[int],
///     The column number to use to load the sources in the target edges list.
/// target_edge_list_destinations_column: Optional[str],
///     The column name to use to load the destinations in the target edges list.
/// target_edge_list_destinations_column_number: Optional[int],
///     The column number to use to load the destinations in the target edges list.
/// target_edge_list_edge_type_column: Optional[str],
///     The column name to use for the edge types in the target edges list.
/// target_edge_list_edge_type_column_number: Optional[int],
///     The column number to use for the edge types in the target edges list.
/// target_edge_list_weights_column: Optional[str],
///     The column name to use for the weights in the target edges list.
/// target_edge_list_weights_column_number: Optional[int],
///     The column number to use for the weights in the target edges list.
/// comment_symbol: Optional[str],
///     The comment symbol to use within the original edge list.
/// default_edge_type: Optional[str],
///     The default edge type to use within the original edge list.
/// default_weight: Optional[float],
///     The default weight to use within the original edge list.
/// max_rows_number: Optional[int],
///     The amount of rows to load from the original edge list.
/// rows_to_skip: Optional[int],
///     The amount of rows to skip from the original edge list.
/// edges_number: Optional[int],
///     The expected number of edges. It will be used for the loading bar.
/// skip_edge_types_if_unavailable: Optional[bool],
///     Whether to automatically skip the edge types if they are not available.
/// skip_weights_if_unavailable: Optional[bool],
///     Whether to automatically skip the weights if they are not available.
/// verbose: Optional[bool],
///     Whether to show the loading bar while processing the file.
/// name: Optional[str],
///     The name of the graph to display in the loading bar.
///
pub fn filter_duplicates_from_edge_list(
    original_edge_path: &str,
    original_edge_list_separator: Option<String>,
    original_edge_list_header: Option<bool>,
    original_edge_list_sources_column: Option<String>,
    original_edge_list_sources_column_number: Option<usize>,
    original_edge_list_destinations_column: Option<String>,
    original_edge_list_destinations_column_number: Option<usize>,
    original_edge_list_edge_type_column: Option<String>,
    original_edge_list_edge_type_column_number: Option<usize>,
    original_edge_list_weights_column: Option<String>,
    original_edge_list_weights_column_number: Option<usize>,
    target_edge_path: &str,
    target_edge_list_separator: Option<String>,
    target_edge_list_header: Option<bool>,
    target_edge_list_sources_column_number: Option<usize>,
    target_edge_list_sources_column: Option<String>,
    target_edge_list_destinations_column_number: Option<usize>,
    target_edge_list_destinations_column: Option<String>,
    target_edge_list_edge_type_column: Option<String>,
    target_edge_list_edge_type_column_number: Option<usize>,
    target_edge_list_weights_column: Option<String>,
    target_edge_list_weights_column_number: Option<usize>,
    comment_symbol: Option<String>,
    default_edge_type: Option<String>,
    default_weight: Option<WeightT>,
    max_rows_number: Option<usize>,
    rows_to_skip: Option<usize>,
    edges_number: Option<usize>,
    skip_edge_types_if_unavailable: Option<bool>,
    skip_weights_if_unavailable: Option<bool>,
    verbose: Option<bool>,
    name: Option<String>,
) -> PyResult<()> {
    Ok(pe!(graph::filter_duplicates_from_edge_list(
        original_edge_path.into(),
        original_edge_list_separator.into(),
        original_edge_list_header.into(),
        original_edge_list_sources_column.into(),
        original_edge_list_sources_column_number.into(),
        original_edge_list_destinations_column.into(),
        original_edge_list_destinations_column_number.into(),
        original_edge_list_edge_type_column.into(),
        original_edge_list_edge_type_column_number.into(),
        original_edge_list_weights_column.into(),
        original_edge_list_weights_column_number.into(),
        target_edge_path.into(),
        target_edge_list_separator.into(),
        target_edge_list_header.into(),
        target_edge_list_sources_column_number.into(),
        target_edge_list_sources_column.into(),
        target_edge_list_destinations_column_number.into(),
        target_edge_list_destinations_column.into(),
        target_edge_list_edge_type_column.into(),
        target_edge_list_edge_type_column_number.into(),
        target_edge_list_weights_column.into(),
        target_edge_list_weights_column_number.into(),
        comment_symbol.into(),
        default_edge_type.into(),
        default_weight.into(),
        max_rows_number.into(),
        rows_to_skip.into(),
        edges_number.into(),
        skip_edge_types_if_unavailable.into(),
        skip_weights_if_unavailable.into(),
        verbose.into(),
        name.into()
    ))?
    .into())
}

#[pyfunction]
#[automatically_generated_binding]
#[text_signature = "(original_edge_path, original_edge_list_separator, original_edge_list_header, original_sources_column, original_sources_column_number, original_destinations_column, original_destinations_column_number, original_edge_list_edge_type_column, original_edge_list_edge_type_column_number, original_weights_column, original_weights_column_number, target_edge_path, target_edge_list_separator, target_edge_list_header, target_sources_column, target_sources_column_number, target_destinations_column, target_destinations_column_number, target_edge_list_edge_type_column, target_edge_list_edge_type_column_number, target_weights_column, target_weights_column_number, comment_symbol, default_edge_type, default_weight, max_rows_number, rows_to_skip, edges_number, skip_edge_types_if_unavailable, skip_weights_if_unavailable, verbose, name)"]
/// Create a new directed edge list from a given undirected one by duplicating the undirected edges.
///
/// Parameters
/// ----------
/// original_edge_path: str,
///     The path from where to load the original edge list.
/// original_edge_list_separator: Optional[str],
///     Separator to use for the original edge list.
/// original_edge_list_header: Optional[bool],
///     Whether the original edge list has an header.
/// original_sources_column: Optional[str],
///     The column name to use to load the sources in the original edges list.
/// original_sources_column_number: Optional[int],
///     The column number to use to load the sources in the original edges list.
/// original_destinations_column: Optional[str],
///     The column name to use to load the destinations in the original edges list.
/// original_destinations_column_number: Optional[int],
///     The column number to use to load the destinations in the original edges list.
/// original_edge_list_edge_type_column: Optional[str],
///     The column name to use for the edge types in the original edges list.
/// original_edge_list_edge_type_column_number: Optional[int],
///     The column number to use for the edge types in the original edges list.
/// original_weights_column: Optional[str],
///     The column name to use for the weights in the original edges list.
/// original_weights_column_number: Optional[int],
///     The column number to use for the weights in the original edges list.
/// target_edge_path: str,
///     The path from where to load the target edge list. This must be different from the original edge list path.
/// target_edge_list_separator: Optional[str],
///     Separator to use for the target edge list. If None, the one provided from the original edge list will be used.
/// target_edge_list_header: Optional[bool],
///     Whether the target edge list has an header. If None, the one provided from the original edge list will be used.
/// target_sources_column: Optional[str],
///     The column name to use to load the sources in the target edges list. If None, the one provided from the original edge list will be used.
/// target_sources_column_number: Optional[int],
///     The column number to use to load the sources in the target edges list. If None, the one provided from the original edge list will be used.
/// target_destinations_column: Optional[str],
///     The column name to use to load the destinations in the target edges list. If None, the one provided from the original edge list will be used.
/// target_destinations_column_number: Optional[int],
///     The column number to use to load the destinations in the target edges list. If None, the one provided from the original edge list will be used.
/// target_edge_list_edge_type_column: Optional[str],
///     The column name to use for the edge types in the target edges list. If None, the one provided from the original edge list will be used.
/// target_edge_list_edge_type_column_number: Optional[int],
///     The column number to use for the edge types in the target edges list. If None, the one provided from the original edge list will be used.
/// target_weights_column: Optional[str],
///     The column name to use for the weights in the target edges list. If None, the one provided from the original edge list will be used.
/// target_weights_column_number: Optional[int],
///     The column number to use for the weights in the target edges list. If None, the one provided from the original edge list will be used.
/// comment_symbol: Optional[str],
///     The comment symbol to use within the original edge list.
/// default_edge_type: Optional[str],
///     The default edge type to use within the original edge list.
/// default_weight: Optional[float],
///     The default weight to use within the original edge list.
/// max_rows_number: Optional[int],
///     The amount of rows to load from the original edge list.
/// rows_to_skip: Optional[int],
///     The amount of rows to skip from the original edge list.
/// edges_number: Optional[int],
///     The expected number of edges. It will be used for the loading bar.
/// skip_edge_types_if_unavailable: Optional[bool],
///     Whether to automatically skip the edge types if they are not available.
/// skip_weights_if_unavailable: Optional[bool],
///     Whether to automatically skip the weights if they are not available.
/// verbose: Optional[bool],
///     Whether to show the loading bar while processing the file.
/// name: Optional[str],
///     The name of the graph to display in the loading bar.
///
pub fn convert_undirected_edge_list_to_directed(
    original_edge_path: &str,
    original_edge_list_separator: Option<String>,
    original_edge_list_header: Option<bool>,
    original_sources_column: Option<String>,
    original_sources_column_number: Option<usize>,
    original_destinations_column: Option<String>,
    original_destinations_column_number: Option<usize>,
    original_edge_list_edge_type_column: Option<String>,
    original_edge_list_edge_type_column_number: Option<usize>,
    original_weights_column: Option<String>,
    original_weights_column_number: Option<usize>,
    target_edge_path: &str,
    target_edge_list_separator: Option<String>,
    target_edge_list_header: Option<bool>,
    target_sources_column: Option<String>,
    target_sources_column_number: Option<usize>,
    target_destinations_column: Option<String>,
    target_destinations_column_number: Option<usize>,
    target_edge_list_edge_type_column: Option<String>,
    target_edge_list_edge_type_column_number: Option<usize>,
    target_weights_column: Option<String>,
    target_weights_column_number: Option<usize>,
    comment_symbol: Option<String>,
    default_edge_type: Option<String>,
    default_weight: Option<WeightT>,
    max_rows_number: Option<usize>,
    rows_to_skip: Option<usize>,
    edges_number: Option<usize>,
    skip_edge_types_if_unavailable: Option<bool>,
    skip_weights_if_unavailable: Option<bool>,
    verbose: Option<bool>,
    name: Option<String>,
) -> PyResult<EdgeT> {
    Ok(pe!(graph::convert_undirected_edge_list_to_directed(
        original_edge_path.into(),
        original_edge_list_separator.into(),
        original_edge_list_header.into(),
        original_sources_column.into(),
        original_sources_column_number.into(),
        original_destinations_column.into(),
        original_destinations_column_number.into(),
        original_edge_list_edge_type_column.into(),
        original_edge_list_edge_type_column_number.into(),
        original_weights_column.into(),
        original_weights_column_number.into(),
        target_edge_path.into(),
        target_edge_list_separator.into(),
        target_edge_list_header.into(),
        target_sources_column.into(),
        target_sources_column_number.into(),
        target_destinations_column.into(),
        target_destinations_column_number.into(),
        target_edge_list_edge_type_column.into(),
        target_edge_list_edge_type_column_number.into(),
        target_weights_column.into(),
        target_weights_column_number.into(),
        comment_symbol.into(),
        default_edge_type.into(),
        default_weight.into(),
        max_rows_number.into(),
        rows_to_skip.into(),
        edges_number.into(),
        skip_edge_types_if_unavailable.into(),
        skip_weights_if_unavailable.into(),
        verbose.into(),
        name.into()
    ))?
    .into())
}

#[pyfunction]
#[automatically_generated_binding]
#[text_signature = "(original_node_path, original_node_list_separator, original_node_list_header, node_list_rows_to_skip, node_list_is_correct, node_list_max_rows_number, node_list_comment_symbol, original_nodes_column_number, original_nodes_column, nodes_number, original_minimum_node_id, original_numeric_node_ids, original_load_node_list_in_parallel, original_edge_type_path, original_edge_types_column_number, original_edge_types_column, edge_types_number, original_numeric_edge_type_ids, original_minimum_edge_type_id, original_edge_type_list_separator, original_edge_type_list_header, edge_type_list_rows_to_skip, edge_type_list_is_correct, edge_type_list_max_rows_number, edge_type_list_comment_symbol, load_edge_type_list_in_parallel, original_edge_path, original_edge_list_separator, original_edge_list_header, original_sources_column_number, original_sources_column, original_destinations_column_number, original_destinations_column, original_edge_list_edge_types_column, original_edge_list_edge_types_column_number, original_weights_column, original_weights_column_number, target_edge_path, target_edge_list_separator, target_edge_list_header, target_sources_column, target_sources_column_number, target_destinations_column, target_destinations_column_number, target_edge_list_edge_types_column, target_edge_list_edge_types_column_number, target_weights_column, target_weights_column_number, target_node_path, target_node_list_separator, target_node_list_header, target_nodes_column, target_nodes_column_number, target_edge_type_list_path, target_edge_type_list_separator, target_edge_type_list_header, target_edge_type_list_edge_types_column, target_edge_type_list_edge_types_column_number, comment_symbol, default_edge_type, default_weight, max_rows_number, rows_to_skip, edges_number, skip_edge_types_if_unavailable, skip_weights_if_unavailable, directed, verbose, name)"]
/// Create a new edge list starting from given one with node IDs densified.
///
/// Parameters
/// ----------
/// original_edge_path: str,
///     The path from where to load the original edge list.
/// original_edge_list_separator: Optional[str],
///     Separator to use for the original edge list.
/// original_edge_list_header: Optional[bool],
///     Whether the original edge list has an header.
/// original_sources_column: Optional[str],
///     The column name to use to load the sources in the original edges list.
/// original_sources_column_number: Optional[int],
///     The column number to use to load the sources in the original edges list.
/// original_destinations_column: Optional[str],
///     The column name to use to load the destinations in the original edges list.
/// original_destinations_column_number: Optional[int],
///     The column number to use to load the destinations in the original edges list.
/// original_edge_list_edge_types_column: Optional[str],
///     The column name to use for the edge types in the original edges list.
/// original_edge_list_edge_types_column_number: Optional[int],
///     The column number to use for the edge types in the original edges list.
/// original_weights_column: Optional[str],
///     The column name to use for the weights in the original edges list.
/// original_weights_column_number: Optional[int],
///     The column number to use for the weights in the original edges list.
/// target_edge_path: str,
///     The path from where to load the target edge list. This must be different from the original edge list path.
/// target_edge_list_separator: Optional[str],
///     Separator to use for the target edge list. If None, the one provided from the original edge list will be used.
/// target_edge_list_header: Optional[bool],
///     Whether the target edge list has an header. If None, the one provided from the original edge list will be used.
/// target_sources_column: Optional[str],
///     The column name to use to load the sources in the target edges list. If None, the one provided from the original edge list will be used.
/// target_sources_column_number: Optional[int],
///     The column number to use to load the sources in the target edges list. If None, the one provided from the original edge list will be used.
/// target_destinations_column: Optional[str],
///     The column name to use to load the destinations in the target edges list. If None, the one provided from the original edge list will be used.
/// target_destinations_column_number: Optional[int],
///     The column number to use to load the destinations in the target edges list. If None, the one provided from the original edge list will be used.
/// target_edge_list_edge_types_column: Optional[str],
///     The column name to use for the edge types in the target edges list. If None, the one provided from the original edge list will be used.
/// target_edge_list_edge_types_column_number: Optional[int],
///     The column number to use for the edge types in the target edges list. If None, the one provided from the original edge list will be used.
/// target_weights_column: Optional[str],
///     The column name to use for the weights in the target edges list. If None, the one provided from the original edge list will be used.
/// target_weights_column_number: Optional[int],
///     The column number to use for the weights in the target edges list. If None, the one provided from the original edge list will be used.
/// target_node_path: Optional[str],
///     The optional name for the node list to be written out.
/// target_node_list_separator: Optional[str],
///     The separator to use for the node list.
/// target_node_list_header: Optional[bool],
///     Whether to add the header to the node list.
/// target_nodes_column: Optional[str],
///     The column name for the node names.
/// target_nodes_column_number: Optional[int],
///     The column number for the node names.
/// target_node_ids_column: Optional[str],
///     The column name for the node IDs.
/// target_node_ids_column_number: Optional[int],
///     The column number for the node IDs.
/// target_edge_type_list_path: Optional[str],
///     The optional path where to store the parsed edge types.
/// target_edge_type_list_edge_types_column_number: Optional[int],
///     The column number where to store the edge type names.
/// target_edge_type_list_edge_types_column: Optional[str],
///     The column name where to store the edge type names.
/// target_edge_types_ids_column_number: Optional[int],
///     The column number where to the store the edge type IDs.
/// target_edge_types_ids_column: Optional[str],
///     The column name where to store the edge type IDs.
/// comment_symbol: Optional[str],
///     The comment symbol to use within the original edge list.
/// default_edge_type: Optional[str],
///     The default edge type to use within the original edge list.
/// default_weight: Optional[float],
///     The default weight to use within the original edge list.
/// max_rows_number: Optional[int],
///     The amount of rows to load from the original edge list.
/// rows_to_skip: Optional[int],
///     The amount of rows to skip from the original edge list.
/// edges_number: Optional[int],
///     The expected number of edges. It will be used for the loading bar.
/// skip_edge_types_if_unavailable: Optional[bool],
///     Whether to automatically skip the edge types if they are not available.
/// skip_weights_if_unavailable: Optional[bool],
///     Whether to automatically skip the weights if they are not available.
/// verbose: Optional[bool],
///     Whether to show the loading bar while processing the file.
/// name: Optional[str],
///     The name of the graph to display in the loading bar.
///
///
/// Raises
/// -------
/// ValueError
///     If there are problems with opening the original or target file.
/// ValueError
///     If the original and target paths are identical.
///
pub fn convert_edge_list_to_numeric(
    original_node_path: Option<String>,
    original_node_list_separator: Option<String>,
    original_node_list_header: Option<bool>,
    node_list_rows_to_skip: Option<usize>,
    node_list_is_correct: Option<bool>,
    node_list_max_rows_number: Option<usize>,
    node_list_comment_symbol: Option<String>,
    original_nodes_column_number: Option<usize>,
    original_nodes_column: Option<String>,
    nodes_number: Option<NodeT>,
    original_minimum_node_id: Option<NodeT>,
    original_numeric_node_ids: Option<bool>,
    original_load_node_list_in_parallel: Option<bool>,
    original_edge_type_path: Option<String>,
    original_edge_types_column_number: Option<usize>,
    original_edge_types_column: Option<String>,
    edge_types_number: Option<EdgeTypeT>,
    original_numeric_edge_type_ids: Option<bool>,
    original_minimum_edge_type_id: Option<EdgeTypeT>,
    original_edge_type_list_separator: Option<String>,
    original_edge_type_list_header: Option<bool>,
    edge_type_list_rows_to_skip: Option<usize>,
    edge_type_list_is_correct: Option<bool>,
    edge_type_list_max_rows_number: Option<usize>,
    edge_type_list_comment_symbol: Option<String>,
    load_edge_type_list_in_parallel: Option<bool>,
    original_edge_path: &str,
    original_edge_list_separator: Option<String>,
    original_edge_list_header: Option<bool>,
    original_sources_column_number: Option<usize>,
    original_sources_column: Option<String>,
    original_destinations_column_number: Option<usize>,
    original_destinations_column: Option<String>,
    original_edge_list_edge_types_column: Option<String>,
    original_edge_list_edge_types_column_number: Option<usize>,
    original_weights_column: Option<String>,
    original_weights_column_number: Option<usize>,
    target_edge_path: &str,
    target_edge_list_separator: Option<String>,
    target_edge_list_header: Option<bool>,
    target_sources_column: Option<String>,
    target_sources_column_number: Option<usize>,
    target_destinations_column: Option<String>,
    target_destinations_column_number: Option<usize>,
    target_edge_list_edge_types_column: Option<String>,
    target_edge_list_edge_types_column_number: Option<usize>,
    target_weights_column: Option<String>,
    target_weights_column_number: Option<usize>,
    target_node_path: Option<&str>,
    target_node_list_separator: Option<String>,
    target_node_list_header: Option<bool>,
    target_nodes_column: Option<String>,
    target_nodes_column_number: Option<usize>,
    target_edge_type_list_path: Option<String>,
    target_edge_type_list_separator: Option<String>,
    target_edge_type_list_header: Option<bool>,
    target_edge_type_list_edge_types_column: Option<String>,
    target_edge_type_list_edge_types_column_number: Option<usize>,
    comment_symbol: Option<String>,
    default_edge_type: Option<String>,
    default_weight: Option<WeightT>,
    max_rows_number: Option<usize>,
    rows_to_skip: Option<usize>,
    edges_number: Option<usize>,
    skip_edge_types_if_unavailable: Option<bool>,
    skip_weights_if_unavailable: Option<bool>,
    directed: bool,
    verbose: Option<bool>,
    name: Option<String>,
) -> PyResult<(NodeT, Option<EdgeTypeT>)> {
    Ok(pe!(graph::convert_edge_list_to_numeric(
        original_node_path.into(),
        original_node_list_separator.into(),
        original_node_list_header.into(),
        node_list_rows_to_skip.into(),
        node_list_is_correct.into(),
        node_list_max_rows_number.into(),
        node_list_comment_symbol.into(),
        original_nodes_column_number.into(),
        original_nodes_column.into(),
        nodes_number.into(),
        original_minimum_node_id.into(),
        original_numeric_node_ids.into(),
        original_load_node_list_in_parallel.into(),
        original_edge_type_path.into(),
        original_edge_types_column_number.into(),
        original_edge_types_column.into(),
        edge_types_number.into(),
        original_numeric_edge_type_ids.into(),
        original_minimum_edge_type_id.into(),
        original_edge_type_list_separator.into(),
        original_edge_type_list_header.into(),
        edge_type_list_rows_to_skip.into(),
        edge_type_list_is_correct.into(),
        edge_type_list_max_rows_number.into(),
        edge_type_list_comment_symbol.into(),
        load_edge_type_list_in_parallel.into(),
        original_edge_path.into(),
        original_edge_list_separator.into(),
        original_edge_list_header.into(),
        original_sources_column_number.into(),
        original_sources_column.into(),
        original_destinations_column_number.into(),
        original_destinations_column.into(),
        original_edge_list_edge_types_column.into(),
        original_edge_list_edge_types_column_number.into(),
        original_weights_column.into(),
        original_weights_column_number.into(),
        target_edge_path.into(),
        target_edge_list_separator.into(),
        target_edge_list_header.into(),
        target_sources_column.into(),
        target_sources_column_number.into(),
        target_destinations_column.into(),
        target_destinations_column_number.into(),
        target_edge_list_edge_types_column.into(),
        target_edge_list_edge_types_column_number.into(),
        target_weights_column.into(),
        target_weights_column_number.into(),
        target_node_path.into(),
        target_node_list_separator.into(),
        target_node_list_header.into(),
        target_nodes_column.into(),
        target_nodes_column_number.into(),
        target_edge_type_list_path.into(),
        target_edge_type_list_separator.into(),
        target_edge_type_list_header.into(),
        target_edge_type_list_edge_types_column.into(),
        target_edge_type_list_edge_types_column_number.into(),
        comment_symbol.into(),
        default_edge_type.into(),
        default_weight.into(),
        max_rows_number.into(),
        rows_to_skip.into(),
        edges_number.into(),
        skip_edge_types_if_unavailable.into(),
        skip_weights_if_unavailable.into(),
        directed.into(),
        verbose.into(),
        name.into()
    ))?
    .into())
}

#[pyfunction]
#[automatically_generated_binding]
#[text_signature = "(maximum_node_id, original_edge_path, original_edge_list_separator, original_edge_list_header, original_sources_column, original_sources_column_number, original_destinations_column, original_destinations_column_number, original_edge_list_edge_types_column, original_edge_list_edge_types_column_number, original_weights_column, original_weights_column_number, original_edge_type_path, original_edge_types_column_number, original_edge_types_column, edge_types_number, original_numeric_edge_type_ids, original_minimum_edge_type_id, original_edge_type_list_separator, original_edge_type_list_header, edge_type_list_rows_to_skip, edge_type_list_is_correct, edge_type_list_max_rows_number, edge_type_list_comment_symbol, load_edge_type_list_in_parallel, target_edge_path, target_edge_list_separator, target_edge_list_header, target_sources_column, target_sources_column_number, target_destinations_column, target_destinations_column_number, target_edge_list_edge_types_column, target_edge_list_edge_types_column_number, target_weights_column, target_weights_column_number, target_node_path, target_node_list_separator, target_node_list_header, target_nodes_column, target_nodes_column_number, target_edge_type_list_path, target_edge_type_list_separator, target_edge_type_list_header, target_edge_type_list_edge_types_column, target_edge_type_list_edge_types_column_number, comment_symbol, default_edge_type, default_weight, max_rows_number, rows_to_skip, edges_number, skip_edge_types_if_unavailable, skip_weights_if_unavailable, directed, verbose, name)"]
/// Create a new edge list starting from given numeric one with node IDs densified and returns the number of unique nodes.
///
/// This method is meant as a solution to parse very large sparse numeric graphs,
/// like for instance ClueWeb.
///
/// Safety
/// ------
/// This method will panic if the node IDs are not numeric.
///  TODO: In the future we may handle this case as a normal error.
///
/// Parameters
/// ----------
/// maximum_node_id: Optional[int],
///     The maximum node ID present in this graph. If available, optimal memory allocation will be used.
/// original_edge_path: str,
///     The path from where to load the original edge list.
/// original_edge_list_separator: Optional[str],
///     Separator to use for the original edge list.
/// original_edge_list_header: Optional[bool],
///     Whether the original edge list has an header.
/// original_sources_column: Optional[str],
///     The column name to use to load the sources in the original edges list.
/// original_sources_column_number: Optional[int],
///     The column number to use to load the sources in the original edges list.
/// original_destinations_column: Optional[str],
///     The column name to use to load the destinations in the original edges list.
/// original_destinations_column_number: Optional[int],
///     The column number to use to load the destinations in the original edges list.
/// original_edge_list_edge_types_column: Optional[str],
///     The column name to use for the edge types in the original edges list.
/// original_edge_list_edge_types_column_number: Optional[int],
///     The column number to use for the edge types in the original edges list.
/// original_weights_column: Optional[str],
///     The column name to use for the weights in the original edges list.
/// original_weights_column_number: Optional[int],
///     The column number to use for the weights in the original edges list.
/// target_edge_path: str,
///     The path from where to load the target edge list.
/// target_edge_list_separator: Optional[str],
///     Separator to use for the target edge list.
/// target_edge_list_header: Optional[bool],
///     Whether the target edge list has an header.
/// target_sources_column: Optional[str],
///     The column name to use to load the sources in the target edges list.
/// target_sources_column_number: Optional[int],
///     The column number to use to load the sources in the target edges list.
/// target_destinations_column: Optional[str],
///     The column name to use to load the destinations in the target edges list.
/// target_destinations_column_number: Optional[int],
///     The column number to use to load the destinations in the target edges list.
/// target_edge_list_edge_types_column: Optional[str],
///     The column name to use for the edge types in the target edges list.
/// target_edge_list_edge_types_column_number: Optional[int],
///     The column number to use for the edge types in the target edges list.
/// target_weights_column: Optional[str],
///     The column name to use for the weights in the target edges list.
/// target_weights_column_number: Optional[int],
///     The column number to use for the weights in the target edges list.
/// comment_symbol: Optional[str],
///     The comment symbol to use within the original edge list.
/// default_edge_type: Optional[str],
///     The default edge type to use within the original edge list.
/// default_weight: Optional[float],
///     The default weight to use within the original edge list.
/// max_rows_number: Optional[int],
///     The amount of rows to load from the original edge list.
/// rows_to_skip: Optional[int],
///     The amount of rows to skip from the original edge list.
/// edges_number: Optional[int],
///     The expected number of edges. It will be used for the loading bar.
/// skip_edge_types_if_unavailable: Optional[bool],
///     Whether to automatically skip the edge types if they are not available.
/// skip_weights_if_unavailable: Optional[bool],
///     Whether to automatically skip the weights if they are not available.
/// verbose: Optional[bool],
///     Whether to show the loading bar while processing the file.
/// name: Optional[str],
///     The name of the graph to display in the loading bar.
///
pub fn densify_sparse_numeric_edge_list(
    maximum_node_id: Option<EdgeT>,
    original_edge_path: &str,
    original_edge_list_separator: Option<String>,
    original_edge_list_header: Option<bool>,
    original_sources_column: Option<String>,
    original_sources_column_number: Option<usize>,
    original_destinations_column: Option<String>,
    original_destinations_column_number: Option<usize>,
    original_edge_list_edge_types_column: Option<String>,
    original_edge_list_edge_types_column_number: Option<usize>,
    original_weights_column: Option<String>,
    original_weights_column_number: Option<usize>,
    original_edge_type_path: Option<String>,
    original_edge_types_column_number: Option<usize>,
    original_edge_types_column: Option<String>,
    edge_types_number: Option<EdgeTypeT>,
    original_numeric_edge_type_ids: Option<bool>,
    original_minimum_edge_type_id: Option<EdgeTypeT>,
    original_edge_type_list_separator: Option<String>,
    original_edge_type_list_header: Option<bool>,
    edge_type_list_rows_to_skip: Option<usize>,
    edge_type_list_is_correct: Option<bool>,
    edge_type_list_max_rows_number: Option<usize>,
    edge_type_list_comment_symbol: Option<String>,
    load_edge_type_list_in_parallel: Option<bool>,
    target_edge_path: &str,
    target_edge_list_separator: Option<String>,
    target_edge_list_header: Option<bool>,
    target_sources_column: Option<String>,
    target_sources_column_number: Option<usize>,
    target_destinations_column: Option<String>,
    target_destinations_column_number: Option<usize>,
    target_edge_list_edge_types_column: Option<String>,
    target_edge_list_edge_types_column_number: Option<usize>,
    target_weights_column: Option<String>,
    target_weights_column_number: Option<usize>,
    target_node_path: Option<&str>,
    target_node_list_separator: Option<String>,
    target_node_list_header: Option<bool>,
    target_nodes_column: Option<String>,
    target_nodes_column_number: Option<usize>,
    target_edge_type_list_path: Option<String>,
    target_edge_type_list_separator: Option<String>,
    target_edge_type_list_header: Option<bool>,
    target_edge_type_list_edge_types_column: Option<String>,
    target_edge_type_list_edge_types_column_number: Option<usize>,
    comment_symbol: Option<String>,
    default_edge_type: Option<String>,
    default_weight: Option<WeightT>,
    max_rows_number: Option<usize>,
    rows_to_skip: Option<usize>,
    edges_number: Option<usize>,
    skip_edge_types_if_unavailable: Option<bool>,
    skip_weights_if_unavailable: Option<bool>,
    directed: bool,
    verbose: Option<bool>,
    name: Option<String>,
) -> PyResult<(NodeT, Option<EdgeTypeT>)> {
    Ok(pe!(graph::densify_sparse_numeric_edge_list(
        maximum_node_id.into(),
        original_edge_path.into(),
        original_edge_list_separator.into(),
        original_edge_list_header.into(),
        original_sources_column.into(),
        original_sources_column_number.into(),
        original_destinations_column.into(),
        original_destinations_column_number.into(),
        original_edge_list_edge_types_column.into(),
        original_edge_list_edge_types_column_number.into(),
        original_weights_column.into(),
        original_weights_column_number.into(),
        original_edge_type_path.into(),
        original_edge_types_column_number.into(),
        original_edge_types_column.into(),
        edge_types_number.into(),
        original_numeric_edge_type_ids.into(),
        original_minimum_edge_type_id.into(),
        original_edge_type_list_separator.into(),
        original_edge_type_list_header.into(),
        edge_type_list_rows_to_skip.into(),
        edge_type_list_is_correct.into(),
        edge_type_list_max_rows_number.into(),
        edge_type_list_comment_symbol.into(),
        load_edge_type_list_in_parallel.into(),
        target_edge_path.into(),
        target_edge_list_separator.into(),
        target_edge_list_header.into(),
        target_sources_column.into(),
        target_sources_column_number.into(),
        target_destinations_column.into(),
        target_destinations_column_number.into(),
        target_edge_list_edge_types_column.into(),
        target_edge_list_edge_types_column_number.into(),
        target_weights_column.into(),
        target_weights_column_number.into(),
        target_node_path.into(),
        target_node_list_separator.into(),
        target_node_list_header.into(),
        target_nodes_column.into(),
        target_nodes_column_number.into(),
        target_edge_type_list_path.into(),
        target_edge_type_list_separator.into(),
        target_edge_type_list_header.into(),
        target_edge_type_list_edge_types_column.into(),
        target_edge_type_list_edge_types_column_number.into(),
        comment_symbol.into(),
        default_edge_type.into(),
        default_weight.into(),
        max_rows_number.into(),
        rows_to_skip.into(),
        edges_number.into(),
        skip_edge_types_if_unavailable.into(),
        skip_weights_if_unavailable.into(),
        directed.into(),
        verbose.into(),
        name.into()
    ))?
    .into())
}

#[pymodule]
fn utils(_py: Python, m: &PyModule) -> PyResult<()> {
    Ok(())
}
