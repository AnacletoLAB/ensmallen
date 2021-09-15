use super::*;
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
    m.add_class::<ShortestPathsResultBFS>()?;
    m.add_class::<Graph>()?;
    m.add_class::<ShortestPathsDjkstra>()?;
    m.add_wrapped(wrap_pymodule!(edge_list_utils))?;
    m.add_wrapped(wrap_pymodule!(utils))?;
    m.add_wrapped(wrap_pymodule!(preprocessing))?;
    Ok(())
}

#[pyclass]
struct ShortestPathsResultBFS {
    inner: graph::ShortestPathsResultBFS,
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
    "into_iter_finite_distances",
    "into_par_iter_node_ids_and_finite_distances",
    "get_distances",
];

pub const SHORTESTPATHSRESULTBFS_TERMS: &[&str] = &[
    "iter",
    "kth",
    "path",
    "on",
    "eccentricity",
    "par",
    "into",
    "to",
    "from",
    "parent",
    "most",
    "and",
    "has",
    "median",
    "distant",
    "distances",
    "distance",
    "node",
    "node_id",
    "node_ids",
    "get",
    "finite",
    "shortest",
    "point",
];

pub const SHORTESTPATHSRESULTBFS_TFIDF_FREQUENCIES: &[&[(&str, f64)]] = &[
    &[
        ("path", 0.5291444789015055),
        ("node_id", 0.40897582296535806),
        ("to", 0.7115822016750736),
        ("has", 0.7115822016750736),
    ],
    &[
        ("get", 0.13678294723432352),
        ("from", 0.5291444789015055),
        ("node_id", 0.40897582296535806),
        ("distance", 0.7115822016750736),
    ],
    &[
        ("get", 0.13678294723432352),
        ("parent", 0.7115822016750736),
        ("node_id", 0.40897582296535806),
        ("from", 0.5291444789015055),
    ],
    &[
        ("point", 0.2769354282101337),
        ("get", 0.07158733687030015),
        ("shortest", 0.3724168532131226),
        ("on", 0.3724168532131226),
        ("kth", 0.3724168532131226),
        ("path", 0.2769354282101337),
    ],
    &[
        ("median", 1.069761162249775),
        ("get", 0.20563342403012394),
        ("point", 0.7954923709660218),
    ],
    &[
        ("eccentricity", 1.7325479692958312),
        ("get", 0.33303674109226594),
    ],
    &[
        ("node", 0.7115822016750736),
        ("get", 0.13678294723432352),
        ("most", 0.7115822016750736),
        ("distant", 0.7115822016750736),
    ],
    &[
        ("distances", 0.40897582296535806),
        ("into", 0.5291444789015055),
        ("finite", 0.5291444789015055),
        ("iter", 0.5291444789015055),
    ],
    &[
        ("node_ids", 0.2861659123432971),
        ("par", 0.2861659123432971),
        ("and", 0.2861659123432971),
        ("into", 0.21279777966595553),
        ("finite", 0.21279777966595553),
        ("distances", 0.16447142611174181),
        ("iter", 0.21279777966595553),
    ],
    &[
        ("distances", 0.9957672211330457),
        ("get", 0.33303674109226594),
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

#[pyclass]
struct Graph {
    inner: graph::Graph,
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
    #[text_signature = "($self)"]
    /// Returns list of nodes of the various strongly connected components.
    ///
    /// This is an implementation of Tarjan algorithm.
    pub fn strongly_connected_components(&self) -> Vec<HashSet<NodeT>> {
        self.inner.strongly_connected_components().into()
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
    /// Returns 2-approximated verted cover set using greedy algorithm.
    pub fn approximated_vertex_cover_set(&self) -> HashSet<NodeT> {
        self.inner.approximated_vertex_cover_set().into()
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
    /// Print the current graph in a format compatible with Graphviz dot's format
    pub fn to_dot(&self) -> String {
        self.inner.to_dot().into()
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
}

pub const GRAPH_METHODS_NAMES: &[&str] = &[
    "get_laplacian_transformed_graph",
    "iter_laplacian_coo_matrix",
    "get_laplacian_coo_matrix_edges_number",
    "get_random_walk_normalized_laplacian_transformed_graph",
    "iter_random_walk_normalized_laplacian_coo_matrix",
    "get_symmetric_normalized_laplacian_transformed_graph",
    "iter_symmetric_normalized_laplacian_coo_matrix",
    "get_symmetric_normalized_transformed_graph",
    "iter_symmetric_normalized_coo_matrix",
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
    "strongly_connected_components",
    "sort_by_increasing_outbound_node_degree",
    "sort_by_decreasing_outbound_node_degree",
    "sort_by_node_lexicographic_order",
    "get_bfs_topological_sorting_from_node_id",
    "get_reversed_bfs_topological_sorting_from_node_id",
    "sort_by_bfs_topological_sorting_from_node_id",
    "get_dense_binary_adjacency_matrix",
    "get_dense_weighted_adjacency_matrix",
    "iter_unchecked_edge_ids_from_source_node_id",
    "iter_unchecked_edge_ids_from_destination_node_id",
    "iter_unchecked_edge_weights_from_source_node_id",
    "iter_unchecked_edge_weights_from_destination_node_id",
    "par_iter_unchecked_edge_ids_from_source_node_id",
    "iter_unchecked_edge_ids_from_node_ids",
    "iter_unchecked_neighbour_node_ids_from_source_node_id",
    "iter_unchecked_neighbour_node_ids_from_destination_node_id",
    "par_iter_unchecked_neighbour_node_ids_from_source_node_id",
    "iter_unchecked_neighbour_node_ids_intersection_from_source_node_ids",
    "iter_unchecked_neighbour_node_ids_union_from_source_node_ids",
    "iter_unchecked_neighbour_node_ids_difference_from_source_node_ids",
    "iter_unchecked_neighbour_node_names_from_source_node_id",
    "iter_edge_ids_from_node_ids",
    "iter_edge_node_ids_and_edge_type_id_from_edge_type_id",
    "iter_node_ids_and_node_type_ids_from_node_type_id",
    "iter_node_names_and_node_type_names_from_node_type_id",
    "iter_edge_node_names_and_edge_type_name_from_edge_type_id",
    "remove_components",
    "overlaps",
    "contains",
    "get_bipartite_edges",
    "get_bipartite_edge_names",
    "get_star_edges",
    "get_star_edge_names",
    "get_clique_edges",
    "get_clique_edge_names",
    "encode_edge",
    "decode_edge",
    "get_max_encodable_edge_number",
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
    "iter_node_ids",
    "par_iter_node_ids",
    "iter_node_names",
    "par_iter_node_names",
    "iter_node_urls",
    "par_iter_node_urls",
    "iter_node_ontologies",
    "par_iter_node_ontologies",
    "iter_unique_node_type_ids",
    "iter_node_type_counts",
    "iter_unique_node_type_ids_and_counts",
    "iter_unique_node_type_names",
    "iter_unique_node_type_names_and_counts",
    "iter_unique_edge_type_ids",
    "iter_edge_type_counts",
    "iter_unique_edge_type_ids_and_counts",
    "iter_unique_edge_type_names_and_counts",
    "iter_unique_edge_type_names",
    "iter_node_degrees",
    "par_iter_node_degrees",
    "iter_comulative_node_degrees",
    "par_iter_comulative_node_degrees",
    "iter_weighted_node_degrees",
    "par_iter_weighted_node_degrees",
    "iter_connected_node_ids",
    "iter_singleton_node_ids",
    "iter_singleton_node_names",
    "iter_singleton_nodes_with_selfloops_node_ids",
    "par_iter_singleton_nodes_with_selfloops_node_ids",
    "iter_singleton_nodes_with_selfloops_node_names",
    "iter_singleton_node_type_ids",
    "iter_singleton_edge_type_ids",
    "iter_singleton_node_type_names",
    "iter_singleton_edge_type_names",
    "iter_source_node_ids",
    "iter_edge_weights",
    "par_iter_edge_weights",
    "par_iter_source_node_ids",
    "par_iter_directed_source_node_ids",
    "iter_destination_node_ids",
    "par_iter_destination_node_ids",
    "iter_directed_destination_node_ids",
    "par_iter_directed_destination_node_ids",
    "iter_node_ids_and_node_type_ids",
    "iter_unchecked_node_type_ids",
    "iter_one_hot_encoded_node_type_ids",
    "iter_one_hot_encoded_known_node_type_ids",
    "par_iter_unchecked_node_ids_and_node_type_ids",
    "iter_node_names_and_node_type_names",
    "par_iter_node_names_and_node_type_names",
    "iter_edge_node_ids",
    "iter_directed_edge_node_ids",
    "iter_edges",
    "par_iter_edge_node_ids",
    "par_iter_directed_edge_node_ids",
    "par_iter_edges",
    "par_iter_directed_edges",
    "iter_edge_node_ids_and_edge_weight",
    "par_iter_edge_node_ids_and_edge_weight",
    "iter_edge_node_ids_and_edge_type_id",
    "iter_directed_edge_node_ids_and_edge_type_id",
    "iter_one_hot_encoded_edge_type_ids",
    "iter_one_hot_encoded_known_edge_type_ids",
    "iter_edge_node_names_and_edge_type_name",
    "par_iter_edge_node_names_and_edge_type_name",
    "par_iter_directed_edge_node_names_and_edge_type_name",
    "par_iter_edge_node_ids_and_edge_type_id",
    "par_iter_directed_edge_node_ids_and_edge_type_id",
    "par_iter_edge_node_names_and_edge_type_name_and_edge_weight",
    "par_iter_directed_edge_node_names_and_edge_type_name_and_edge_weight",
    "iter_edge_node_names_and_edge_type_name_and_edge_weight",
    "par_iter_edge_node_ids_and_edge_type_id_and_edge_weight",
    "par_iter_directed_edge_node_ids_and_edge_type_id_and_edge_weight",
    "iter_edge_node_ids_and_edge_type_id_and_edge_weight",
    "iter_directed_edge_node_ids_and_edge_type_id_and_edge_weight",
    "iter_unique_edge_node_ids",
    "iter_unique_source_node_ids",
    "par_iter_unique_source_node_ids",
    "iter_edge_ids_with_unknown_edge_types",
    "iter_edge_ids_with_known_edge_types",
    "iter_edge_node_ids_with_unknown_edge_types",
    "iter_edge_node_ids_with_known_edge_types",
    "iter_node_ids_with_unknown_node_types",
    "iter_node_ids_with_known_node_types",
    "iter_edge_node_names_with_unknown_edge_types",
    "iter_edge_node_names_with_known_edge_types",
    "iter_node_names_with_unknown_node_types",
    "iter_node_names_with_known_node_types",
    "par_iter_edge_ids_with_unknown_edge_types",
    "par_iter_edge_ids_with_known_edge_types",
    "par_iter_edge_node_ids_with_unknown_edge_types",
    "par_iter_edge_node_ids_with_known_edge_types",
    "par_iter_node_ids_with_unknown_node_types",
    "par_iter_node_ids_with_known_node_types",
    "par_iter_edge_node_names_with_unknown_edge_types",
    "par_iter_edge_node_names_with_known_edge_types",
    "par_iter_node_names_with_unknown_node_types",
    "par_iter_node_names_with_known_node_types",
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
    "generate_new_edges_from_node_features",
    "set_name",
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
    "get_number_of_triangles",
    "get_triads_number",
    "get_weighted_triads_number",
    "get_transitivity",
    "get_number_of_triangles_per_node",
    "iter_clustering_coefficient_per_node",
    "get_clustering_coefficient_per_node",
    "get_clustering_coefficient",
    "get_average_clustering_coefficient",
    "are_nodes_remappable",
    "remap_unchecked_from_node_ids",
    "remap_from_node_ids",
    "remap_from_node_names",
    "remap_from_graph",
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
    "report",
    "overlap_textual_report",
    "get_node_report_from_node_id",
    "get_node_report_from_node_name",
    "textual_report",
    "generate_random_connected_graph",
    "generate_random_spanning_tree",
    "generate_circle_graph",
    "generate_chain_graph",
    "generate_complete_graph",
    "generate_barbell_graph",
    "replace",
    "filter_from_ids",
    "filter_from_names",
    "drop_unknown_node_types",
    "drop_unknown_edge_types",
    "drop_singleton_nodes",
    "drop_singleton_nodes_with_selfloops",
    "drop_disconnected_nodes",
    "drop_selfloops",
    "drop_parallel_edges",
    "random_spanning_arborescence_kruskal",
    "spanning_arborescence_kruskal",
    "spanning_arborescence",
    "connected_components",
    "enable",
    "disable_all",
    "is_compatible",
    "has_same_adjacency_matrix",
    "par_iter_approximated_vertex_cover",
    "approximated_vertex_cover_set",
    "get_random_nodes",
    "get_breadth_first_search_random_nodes",
    "get_uniform_random_walk_random_nodes",
    "get_node_sampling_methods",
    "get_subsampled_nodes",
    "node2vec",
    "cooccurence_matrix",
    "get_node_label_prediction_mini_batch",
    "get_edge_prediction_mini_batch",
    "link_prediction_degrees",
    "par_iter_unchecked_edge_prediction_metrics",
    "par_iter_edge_prediction_metrics",
    "get_okapi_bm25_node_feature_propagation",
    "get_okapi_bm25_node_label_propagation",
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
    "get_transitive_closure",
    "get_all_shortest_paths",
    "get_weighted_all_shortest_paths",
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
    "par_iter_subsampled_binary_adjacency_matrix",
    "par_iter_subsampled_weighted_adjacency_matrix",
    "par_iter_subsampled_symmetric_laplacian_adjacency_matrix",
    "par_iter_undirected_with_selfloops_subsampled_symmetric_laplacian_adjacency_matrix",
    "get_sparse_edge_weighting_methods",
    "get_edge_weighting_methods",
    "par_iter_subsampled_edge_metric_matrix",
    "add_selfloops",
    "iter_degree_centrality",
    "par_iter_weighted_degree_centrality",
    "get_degree_centrality",
    "get_weighted_degree_centrality",
    "get_unchecked_closeness_centrality_from_node_id",
    "get_unchecked_weighted_closeness_centrality_from_node_id",
    "par_iter_closeness_centrality",
    "par_iter_weighted_closeness_centrality",
    "get_closeness_centrality",
    "get_weighted_closeness_centrality",
    "get_unchecked_harmonic_centrality_from_node_id",
    "get_unchecked_weighted_harmonic_centrality_from_node_id",
    "par_iter_harmonic_centrality",
    "par_iter_weighted_harmonic_centrality",
    "get_harmonic_centrality",
    "get_weighted_harmonic_centrality",
    "get_stress_centrality",
    "get_betweenness_centrality",
    "get_eigenvector_centrality",
    "get_weighted_eigenvector_centrality",
    "to_dot",
    "get_undirected_louvain_community_detection",
    "get_directed_modularity_from_node_community_memberships",
    "get_undirected_modularity_from_node_community_memberships",
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
    "from_csv",
];

pub const GRAPH_TERMS: &[&str] = &[
    "encoded",
    "cumulative",
    "graph",
    "tree",
    "representing",
    "iter",
    "edge_type_name",
    "increasing",
    "breadth",
    "probabilities",
    "random",
    "edge_ids",
    "outbound",
    "mininum",
    "node_names",
    "edge_names",
    "negative",
    "multiple",
    "coefficient",
    "upper",
    "transposed",
    "encodable",
    "have",
    "source_names",
    "order",
    "connected",
    "edge_id",
    "total",
    "generate",
    "hot",
    "indices",
    "subsampled",
    "resource",
    "naive",
    "kfold",
    "sort",
    "triangular",
    "edge_type_ids",
    "default",
    "dot",
    "identity",
    "edge_types",
    "allocation",
    "bipartite",
    "from_ids",
    "rate",
    "remap",
    "transitivity",
    "get",
    "transformed",
    "validate",
    "with",
    "degree",
    "difference",
    "set_name",
    "be",
    "not",
    "clique",
    "node_ids",
    "mean",
    "detection",
    "components",
    "component_ids",
    "bm25",
    "known",
    "density",
    "nodes",
    "reversed",
    "symmetric",
    "must",
    "diameter",
    "decode",
    "comulative",
    "replace",
    "sampling",
    "node_type_name",
    "closure",
    "human",
    "counts",
    "node_id",
    "minimum",
    "search",
    "weight",
    "weights",
    "chain",
    "sample",
    "strongly",
    "stats",
    "dense",
    "selfloop",
    "urls",
    "new",
    "all",
    "stress",
    "destination",
    "preferential",
    "sorted",
    "requirements",
    "cooccurence",
    "compatible",
    "average",
    "bidiagonal",
    "barbell",
    "first",
    "most",
    "prediction",
    "vertex",
    "adar",
    "edge_type",
    "graphs",
    "modularity",
    "bfs",
    "shortest",
    "dijkstra",
    "mode",
    "sorting",
    "csv",
    "betweenness",
    "edge",
    "paths",
    "mask",
    "propagation",
    "clustering",
    "hashmap",
    "multilabel",
    "feature",
    "edges",
    "diagonal",
    "contain",
    "decreasing",
    "maximum",
    "subgraph",
    "textual",
    "star",
    "arrowhead",
    "max",
    "labels",
    "uniform",
    "holdout",
    "louvain",
    "index",
    "normalized",
    "ontologies",
    "harmonic",
    "transitive",
    "filter",
    "from_names",
    "community",
    "trap",
    "kruskal",
    "complete",
    "edge_type_id",
    "report",
    "neighbour",
    "remappable",
    "get_name",
    "destination_names",
    "used",
    "mini",
    "contains",
    "degrees",
    "arborescence",
    "node_type_id",
    "inplace",
    "node_type_names",
    "constant",
    "unknown",
    "by",
    "laplacian",
    "okapi",
    "singleton",
    "mapping",
    "encode",
    "triangles",
    "node_name",
    "graph_name",
    "count",
    "cover",
    "node2vec",
    "unique",
    "link",
    "remove",
    "weighting",
    "central",
    "sparse",
    "is",
    "main",
    "circle",
    "overlap",
    "triads",
    "source",
    "drop",
    "approximated",
    "selfloops",
    "adjacency",
    "minmax",
    "directed",
    "topological",
    "features",
    "disconnected",
    "negatives",
    "one",
    "same",
    "of",
    "has",
    "edge_type_names",
    "anti",
    "label",
    "matrix",
    "oddities",
    "node_type_ids",
    "batch",
    "are",
    "metric",
    "lower",
    "walk",
    "and",
    "metrics",
    "node",
    "complementary",
    "set",
    "methods",
    "node_type",
    "from",
    "eccentricity",
    "lexicographic",
    "spanning",
    "disable",
    "attachment",
    "union",
    "readable",
    "k",
    "parallel",
    "add",
    "undirected",
    "node_types",
    "enable",
    "coo",
    "jaccard",
    "binary",
    "per",
    "indegrees",
    "singletons",
    "requirement",
    "homogeneous",
    "adamic",
    "path",
    "overlaps",
    "multigraph",
    "par",
    "median",
    "number",
    "to",
    "eigenvector",
    "intersection",
    "top",
    "closeness",
    "memberships",
    "unchecked",
    "centrality",
    "weighted",
    "memory",
];

pub const GRAPH_TFIDF_FREQUENCIES: &[&[(&str, f64)]] = &[
    &[
        ("get", 0.2460207953237388),
        ("laplacian", 1.6004262619602063),
        ("graph", 1.5621364120283636),
        ("transformed", 1.8862949532055344),
    ],
    &[
        ("laplacian", 1.6004262619602063),
        ("matrix", 1.3892163030605715),
        ("coo", 1.8095225182222892),
        ("iter", 0.5836532230054678),
    ],
    &[
        ("edges", 0.6923601846392166),
        ("matrix", 0.7363793606076885),
        ("laplacian", 0.8483350396087489),
        ("number", 0.5810049296545123),
        ("get", 0.13040779578930034),
        ("coo", 0.9591703120947649),
    ],
    &[
        ("normalized", 0.7137788271919129),
        ("transformed", 0.7713043299639504),
        ("walk", 0.8106190284071704),
        ("random", 0.5994414921219254),
        ("laplacian", 0.6544128761730416),
        ("get", 0.10059768456248329),
        ("graph", 0.6387561905651457),
    ],
    &[
        ("laplacian", 0.6544128761730416),
        ("normalized", 0.7137788271919129),
        ("matrix", 0.5680493116870342),
        ("random", 0.5994414921219254),
        ("iter", 0.2386552841783915),
        ("coo", 0.7399121495290593),
        ("walk", 0.8106190284071704),
    ],
    &[
        ("graph", 0.8280388084542077),
        ("transformed", 0.999864937158341),
        ("normalized", 0.9252929025156035),
        ("get", 0.13040779578930034),
        ("symmetric", 0.9252929025156035),
        ("laplacian", 0.8483350396087489),
    ],
    &[
        ("normalized", 0.9252929025156035),
        ("coo", 0.9591703120947649),
        ("iter", 0.309376003021643),
        ("matrix", 0.7363793606076885),
        ("laplacian", 0.8483350396087489),
        ("symmetric", 0.9252929025156035),
    ],
    &[
        ("symmetric", 1.242705488772741),
        ("graph", 1.1120893388302326),
        ("transformed", 1.3428587229621909),
        ("get", 0.17514290141589564),
        ("normalized", 1.242705488772741),
    ],
    &[
        ("coo", 1.2882042089239163),
        ("matrix", 0.9889870231990723),
        ("symmetric", 1.242705488772741),
        ("normalized", 1.242705488772741),
        ("iter", 0.41550438353555247),
    ],
    &[
        ("unchecked", 0.5676697648129673),
        ("is", 1.0241790295646243),
        ("node_id", 0.6586091234986925),
        ("from", 0.35292613335563944),
        ("connected", 1.0873123363008848),
    ],
    &[
        ("from", 0.26278152728588133),
        ("node_id", 0.49038678352277515),
        ("is", 0.7625826034592891),
        ("node", 0.4599105951947464),
        ("disconnected", 0.999864937158341),
        ("unchecked", 0.42267521073949105),
    ],
    &[
        ("from", 0.35292613335563944),
        ("is", 1.0241790295646243),
        ("node_id", 0.6586091234986925),
        ("singleton", 0.7444243515125029),
        ("unchecked", 0.5676697648129673),
    ],
    &[
        ("singleton", 1.0456825228820985),
        ("is", 1.4386500244680516),
        ("from", 0.4957504261763134),
        ("node_id", 0.9251390667889959),
    ],
    &[
        ("node_id", 0.3782885422136893),
        ("unchecked", 0.32605525816149983),
        ("from", 0.20271190867656422),
        ("singleton", 0.4275786542799005),
        ("is", 0.5882627164374559),
        ("with", 0.4093678499510831),
        ("selfloops", 0.5680493116870342),
    ],
    &[
        ("from", 0.26278152728588133),
        ("node_id", 0.49038678352277515),
        ("selfloops", 0.7363793606076885),
        ("singleton", 0.5542830341841882),
        ("with", 0.5306758223243937),
        ("is", 0.7625826034592891),
    ],
    &[
        ("node_name", 0.916895565662068),
        ("from", 0.35292613335563944),
        ("is", 1.0241790295646243),
        ("unchecked", 0.5676697648129673),
        ("singleton", 0.7444243515125029),
    ],
    &[
        ("singleton", 1.0456825228820985),
        ("from", 0.4957504261763134),
        ("node_name", 1.2879504362973773),
        ("is", 1.4386500244680516),
    ],
    &[
        ("has", 2.4332950520650543),
        ("node_name", 3.0508538893698427),
    ],
    &[
        ("has", 2.4332950520650543),
        ("node_type_id", 3.700336452436737),
    ],
    &[
        ("node_type_name", 3.8918333777254275),
        ("has", 2.4332950520650543),
    ],
    &[
        ("has", 2.4332950520650543),
        ("edge_type_id", 2.704729156445749),
    ],
    &[
        ("has", 2.4332950520650543),
        ("edge_type_name", 3.1393378556151865),
    ],
    &[
        ("has", 1.0272414011261481),
        ("edge", 0.7056635051756572),
        ("node_ids", 0.5980375941446159),
        ("from", 0.4957504261763134),
    ],
    &[
        ("node_id", 0.9251390667889959),
        ("selfloop", 2.1111699077238817),
        ("from", 0.4957504261763134),
        ("has", 1.0272414011261481),
    ],
    &[
        ("and", 0.5135884091206121),
        ("edge_type_id", 0.6052478569671313),
        ("from", 0.26278152728588133),
        ("node_ids", 0.3170007004851363),
        ("has", 0.5445079822951353),
        ("edge", 0.37404977151551183),
    ],
    &[
        ("from", 0.26278152728588133),
        ("trap", 0.9591703120947649),
        ("node", 0.4599105951947464),
        ("unchecked", 0.42267521073949105),
        ("is", 0.7625826034592891),
        ("node_id", 0.49038678352277515),
    ],
    &[
        ("from", 0.35292613335563944),
        ("trap", 1.2882042089239163),
        ("node_id", 0.6586091234986925),
        ("node", 0.6176783799372239),
        ("is", 1.0241790295646243),
    ],
    &[
        ("node_name", 1.2879504362973773),
        ("has", 1.0272414011261481),
        ("node_type_name", 1.6429788769312366),
        ("and", 0.9689100878988537),
    ],
    &[
        ("from", 0.4957504261763134),
        ("edge", 0.7056635051756572),
        ("node_names", 0.8617582711288273),
        ("has", 1.0272414011261481),
    ],
    &[
        ("edge_type_name", 0.7025019510285271),
        ("and", 0.5135884091206121),
        ("node_names", 0.45679064037341954),
        ("edge", 0.37404977151551183),
        ("has", 0.5445079822951353),
        ("from", 0.26278152728588133),
    ],
    &[
        ("strongly", 3.429933457288565),
        ("connected", 2.271155030697057),
        ("components", 2.804934922882217),
    ],
    &[
        ("increasing", 1.1190639955483745),
        ("by", 0.896273044061298),
        ("node", 0.4599105951947464),
        ("degree", 0.6827015466818229),
        ("outbound", 0.999864937158341),
        ("sort", 0.999864937158341),
    ],
    &[
        ("by", 0.896273044061298),
        ("sort", 0.999864937158341),
        ("outbound", 0.999864937158341),
        ("node", 0.4599105951947464),
        ("degree", 0.6827015466818229),
        ("decreasing", 1.1190639955483745),
    ],
    &[
        ("sort", 1.3428587229621909),
        ("by", 1.2037306546564002),
        ("node", 0.6176783799372239),
        ("lexicographic", 1.5029478403812442),
        ("order", 1.5029478403812442),
    ],
    &[
        ("bfs", 1.0508297599412841),
        ("topological", 1.0508297599412841),
        ("get", 0.13040779578930034),
        ("from", 0.26278152728588133),
        ("sorting", 1.0508297599412841),
        ("node_id", 0.49038678352277515),
    ],
    &[
        ("bfs", 0.8106190284071704),
        ("from", 0.20271190867656422),
        ("reversed", 0.9431671678059753),
        ("topological", 0.8106190284071704),
        ("node_id", 0.3782885422136893),
        ("sorting", 0.8106190284071704),
        ("get", 0.10059768456248329),
    ],
    &[
        ("by", 0.6913926611719697),
        ("from", 0.20271190867656422),
        ("topological", 0.8106190284071704),
        ("sort", 0.7713043299639504),
        ("bfs", 0.8106190284071704),
        ("sorting", 0.8106190284071704),
        ("node_id", 0.3782885422136893),
    ],
    &[
        ("binary", 1.5029478403812442),
        ("dense", 1.4113065245550764),
        ("adjacency", 1.2037306546564002),
        ("matrix", 0.9889870231990723),
        ("get", 0.17514290141589564),
    ],
    &[
        ("dense", 1.4113065245550764),
        ("matrix", 0.9889870231990723),
        ("weighted", 0.718771595110523),
        ("adjacency", 1.2037306546564002),
        ("get", 0.17514290141589564),
    ],
    &[
        ("edge_ids", 0.7131777350098516),
        ("source", 0.6562126797500742),
        ("node_id", 0.49038678352277515),
        ("from", 0.26278152728588133),
        ("unchecked", 0.42267521073949105),
        ("iter", 0.309376003021643),
    ],
    &[
        ("node_id", 0.49038678352277515),
        ("edge_ids", 0.7131777350098516),
        ("iter", 0.309376003021643),
        ("unchecked", 0.42267521073949105),
        ("from", 0.26278152728588133),
        ("destination", 0.7770739856712646),
    ],
    &[
        ("iter", 0.2386552841783915),
        ("edge", 0.28854518012391517),
        ("node_id", 0.3782885422136893),
        ("unchecked", 0.32605525816149983),
        ("source", 0.5062080511663406),
        ("weights", 0.5882627164374559),
        ("from", 0.20271190867656422),
    ],
    &[
        ("weights", 0.5882627164374559),
        ("destination", 0.5994414921219254),
        ("node_id", 0.3782885422136893),
        ("from", 0.20271190867656422),
        ("edge", 0.28854518012391517),
        ("iter", 0.2386552841783915),
        ("unchecked", 0.32605525816149983),
    ],
    &[
        ("unchecked", 0.32605525816149983),
        ("par", 0.38113291687472167),
        ("iter", 0.2386552841783915),
        ("from", 0.20271190867656422),
        ("edge_ids", 0.5501513800557143),
        ("source", 0.5062080511663406),
        ("node_id", 0.3782885422136893),
    ],
    &[
        ("edge_ids", 0.9578263092235365),
        ("from", 0.35292613335563944),
        ("unchecked", 0.5676697648129673),
        ("node_ids", 0.4257446581149362),
        ("iter", 0.41550438353555247),
    ],
    &[
        ("node_id", 0.3782885422136893),
        ("iter", 0.2386552841783915),
        ("neighbour", 0.6387561905651457),
        ("source", 0.5062080511663406),
        ("node_ids", 0.24453704075340602),
        ("from", 0.20271190867656422),
        ("unchecked", 0.32605525816149983),
    ],
    &[
        ("from", 0.20271190867656422),
        ("destination", 0.5994414921219254),
        ("node_id", 0.3782885422136893),
        ("neighbour", 0.6387561905651457),
        ("iter", 0.2386552841783915),
        ("node_ids", 0.24453704075340602),
        ("unchecked", 0.32605525816149983),
    ],
    &[
        ("source", 0.4017048121301286),
        ("iter", 0.18938650990211245),
        ("node_id", 0.3001934232985874),
        ("node_ids", 0.19405401749018308),
        ("par", 0.3024505959891542),
        ("from", 0.1608634019230581),
        ("unchecked", 0.25874334855406383),
        ("neighbour", 0.5068892818609322),
    ],
    &[
        ("intersection", 0.7484566653530093),
        ("unchecked", 0.25874334855406383),
        ("iter", 0.18938650990211245),
        ("source", 0.4017048121301286),
        ("neighbour", 0.5068892818609322),
        ("from", 0.1608634019230581),
        ("node_ids", 0.3697476845299576),
    ],
    &[
        ("node_ids", 0.3697476845299576),
        ("source", 0.4017048121301286),
        ("unchecked", 0.25874334855406383),
        ("neighbour", 0.5068892818609322),
        ("from", 0.1608634019230581),
        ("iter", 0.18938650990211245),
        ("union", 0.7484566653530093),
    ],
    &[
        ("iter", 0.18938650990211245),
        ("node_ids", 0.3697476845299576),
        ("from", 0.1608634019230581),
        ("source", 0.4017048121301286),
        ("neighbour", 0.5068892818609322),
        ("difference", 0.7484566653530093),
        ("unchecked", 0.25874334855406383),
    ],
    &[
        ("source", 0.5062080511663406),
        ("node_id", 0.3782885422136893),
        ("unchecked", 0.32605525816149983),
        ("iter", 0.2386552841783915),
        ("from", 0.20271190867656422),
        ("neighbour", 0.6387561905651457),
        ("node_names", 0.35237219119648877),
    ],
    &[
        ("from", 0.4957504261763134),
        ("iter", 0.5836532230054678),
        ("node_ids", 0.5980375941446159),
        ("edge_ids", 1.345445281950714),
    ],
    &[
        ("iter", 0.2386552841783915),
        ("edge_type_id", 0.8787964411571557),
        ("and", 0.3961864738450093),
        ("from", 0.20271190867656422),
        ("node_ids", 0.24453704075340602),
        ("edge", 0.28854518012391517),
    ],
    &[
        ("node_type_id", 0.8280388084542077),
        ("and", 0.5135884091206121),
        ("node_type_ids", 0.7025019510285271),
        ("iter", 0.309376003021643),
        ("from", 0.26278152728588133),
        ("node_ids", 0.3170007004851363),
    ],
    &[
        ("node_names", 0.45679064037341954),
        ("and", 0.5135884091206121),
        ("node_type_names", 0.7363793606076885),
        ("from", 0.26278152728588133),
        ("iter", 0.309376003021643),
        ("node_type_id", 0.8280388084542077),
    ],
    &[
        ("edge_type_name", 0.541915989349888),
        ("and", 0.3961864738450093),
        ("edge", 0.28854518012391517),
        ("from", 0.20271190867656422),
        ("node_names", 0.35237219119648877),
        ("edge_type_id", 0.46689335272312055),
        ("iter", 0.2386552841783915),
    ],
    &[
        ("components", 4.468192356089292),
        ("remove", 3.1393378556151865),
    ],
    &[("overlaps", 9.263184627320559)],
    &[("contains", 8.478343331774438)],
    &[
        ("edges", 1.9422875919892115),
        ("bipartite", 3.1393256883021006),
        ("get", 0.36583479131199337),
    ],
    &[
        ("edge_names", 2.9479072443926193),
        ("get", 0.36583479131199337),
        ("bipartite", 3.1393256883021006),
    ],
    &[
        ("edges", 1.9422875919892115),
        ("star", 3.1393256883021006),
        ("get", 0.36583479131199337),
    ],
    &[
        ("star", 3.1393256883021006),
        ("edge_names", 2.9479072443926193),
        ("get", 0.36583479131199337),
    ],
    &[
        ("edges", 1.9422875919892115),
        ("get", 0.36583479131199337),
        ("clique", 3.1393256883021006),
    ],
    &[
        ("clique", 3.1393256883021006),
        ("get", 0.36583479131199337),
        ("edge_names", 2.9479072443926193),
    ],
    &[("edge", 1.671552094458415), ("encode", 5.46379965208028)],
    &[("edge", 1.671552094458415), ("decode", 5.46379965208028)],
    &[
        ("edge", 0.5023638491906379),
        ("max", 1.6420759086870347),
        ("number", 0.7803129291521904),
        ("encodable", 1.6420759086870347),
        ("get", 0.17514290141589564),
    ],
    &[
        ("validate", 3.8918333777254275),
        ("node_id", 2.191438459569349),
    ],
    &[
        ("validate", 3.8918333777254275),
        ("node_ids", 1.4166114383490274),
    ],
    &[
        ("validate", 3.8918333777254275),
        ("edge_id", 2.795428871509145),
    ],
    &[
        ("edge_ids", 3.187045755560337),
        ("validate", 3.8918333777254275),
    ],
    &[
        ("node_types", 0.7249618438397802),
        ("contain", 1.3428587229621909),
        ("unknown", 0.840130805785612),
        ("must", 1.1120893388302326),
        ("not", 1.2882042089239163),
    ],
    &[
        ("must", 1.1120893388302326),
        ("contain", 1.3428587229621909),
        ("edge_types", 0.7127189189159387),
        ("unknown", 0.840130805785612),
        ("not", 1.2882042089239163),
    ],
    &[
        ("node_type_id", 3.700336452436737),
        ("validate", 3.8918333777254275),
    ],
    &[
        ("node_type_ids", 3.1393378556151865),
        ("validate", 3.8918333777254275),
    ],
    &[
        ("validate", 3.8918333777254275),
        ("edge_type_id", 2.704729156445749),
    ],
    &[
        ("validate", 3.8918333777254275),
        ("edge_type_ids", 3.6178941140020378),
    ],
    &[
        ("must", 2.322908709986271),
        ("undirected", 2.5957369637715932),
        ("be", 2.804934922882217),
    ],
    &[
        ("be", 2.804934922882217),
        ("must", 2.322908709986271),
        ("multigraph", 2.9479072443926193),
    ],
    &[
        ("must", 1.5621364120283636),
        ("not", 1.8095225182222892),
        ("multigraph", 1.9824426271900815),
        ("be", 1.8862949532055344),
    ],
    &[
        ("must", 1.5621364120283636),
        ("matrix", 1.3892163030605715),
        ("identity", 2.1111699077238817),
        ("contain", 1.8862949532055344),
    ],
    &[
        ("must", 0.8280388084542077),
        ("contain", 0.999864937158341),
        ("singleton", 0.5542830341841882),
        ("nodes", 0.5753714267498481),
        ("weighted", 0.5351825203109013),
        ("not", 0.9591703120947649),
    ],
    &[
        ("must", 2.322908709986271),
        ("edges", 1.9422875919892115),
        ("have", 3.1393256883021006),
    ],
    &[
        ("have", 3.1393256883021006),
        ("must", 2.322908709986271),
        ("nodes", 1.6140974131025445),
    ],
    &[
        ("connected", 2.271155030697057),
        ("be", 2.804934922882217),
        ("must", 2.322908709986271),
    ],
    &[
        ("iter", 1.3825382213996475),
        ("node_ids", 1.4166114383490274),
    ],
    &[
        ("iter", 0.8678967757819245),
        ("par", 1.3860327075459795),
        ("node_ids", 0.8892864449231846),
    ],
    &[
        ("iter", 1.3825382213996475),
        ("node_names", 2.041304151989104),
    ],
    &[
        ("par", 1.3860327075459795),
        ("node_names", 1.2814410947048054),
        ("iter", 0.8678967757819245),
    ],
    &[
        ("iter", 0.8678967757819245),
        ("urls", 2.9479072443926193),
        ("node", 1.2901935470720483),
    ],
    &[
        ("node", 0.8676442211356591),
        ("urls", 1.9824426271900815),
        ("par", 0.9320952439549945),
        ("iter", 0.5836532230054678),
    ],
    &[
        ("iter", 0.8678967757819245),
        ("ontologies", 2.9479072443926193),
        ("node", 1.2901935470720483),
    ],
    &[
        ("ontologies", 1.9824426271900815),
        ("iter", 0.5836532230054678),
        ("node", 0.8676442211356591),
        ("par", 0.9320952439549945),
    ],
    &[
        ("node_type_ids", 1.9707384293652452),
        ("unique", 1.9151920814783965),
        ("iter", 0.8678967757819245),
    ],
    &[
        ("counts", 2.322908709986271),
        ("node_type", 3.429933457288565),
        ("iter", 0.8678967757819245),
    ],
    &[
        ("and", 0.6897698374742284),
        ("unique", 0.916895565662068),
        ("node_type_ids", 0.9434883030478971),
        ("iter", 0.41550438353555247),
        ("counts", 1.1120893388302326),
    ],
    &[
        ("unique", 1.9151920814783965),
        ("iter", 0.8678967757819245),
        ("node_type_names", 2.0657751945261844),
    ],
    &[
        ("and", 0.6897698374742284),
        ("unique", 0.916895565662068),
        ("node_type_names", 0.9889870231990723),
        ("counts", 1.1120893388302326),
        ("iter", 0.41550438353555247),
    ],
    &[
        ("iter", 0.8678967757819245),
        ("edge_type_ids", 2.271155030697057),
        ("unique", 1.9151920814783965),
    ],
    &[
        ("edge_type", 3.429933457288565),
        ("counts", 2.322908709986271),
        ("iter", 0.8678967757819245),
    ],
    &[
        ("and", 0.6897698374742284),
        ("iter", 0.41550438353555247),
        ("counts", 1.1120893388302326),
        ("unique", 0.916895565662068),
        ("edge_type_ids", 1.0873123363008848),
    ],
    &[
        ("iter", 0.41550438353555247),
        ("edge_type_names", 1.1696413187027879),
        ("counts", 1.1120893388302326),
        ("and", 0.6897698374742284),
        ("unique", 0.916895565662068),
    ],
    &[
        ("iter", 0.8678967757819245),
        ("unique", 1.9151920814783965),
        ("edge_type_names", 2.443122069340597),
    ],
    &[
        ("iter", 0.8678967757819245),
        ("degrees", 2.1013429119884273),
        ("node", 1.2901935470720483),
    ],
    &[
        ("iter", 0.5836532230054678),
        ("node", 0.8676442211356591),
        ("degrees", 1.4131352914829942),
        ("par", 0.9320952439549945),
    ],
    &[
        ("degrees", 1.4131352914829942),
        ("node", 0.8676442211356591),
        ("iter", 0.5836532230054678),
        ("comulative", 1.8862949532055344),
    ],
    &[
        ("degrees", 1.006015018843602),
        ("comulative", 1.3428587229621909),
        ("iter", 0.41550438353555247),
        ("par", 0.6635612457370292),
        ("node", 0.6176783799372239),
    ],
    &[
        ("node", 0.8676442211356591),
        ("weighted", 1.0096484530954233),
        ("degrees", 1.4131352914829942),
        ("iter", 0.5836532230054678),
    ],
    &[
        ("weighted", 0.718771595110523),
        ("node", 0.6176783799372239),
        ("degrees", 1.006015018843602),
        ("iter", 0.41550438353555247),
        ("par", 0.6635612457370292),
    ],
    &[
        ("node_ids", 0.8892864449231846),
        ("iter", 0.8678967757819245),
        ("connected", 2.271155030697057),
    ],
    &[
        ("iter", 0.8678967757819245),
        ("singleton", 1.554937854069521),
        ("node_ids", 0.8892864449231846),
    ],
    &[
        ("iter", 0.8678967757819245),
        ("singleton", 1.554937854069521),
        ("node_names", 1.2814410947048054),
    ],
    &[
        ("nodes", 0.5753714267498481),
        ("singleton", 0.5542830341841882),
        ("iter", 0.309376003021643),
        ("with", 0.5306758223243937),
        ("node_ids", 0.3170007004851363),
        ("selfloops", 0.7363793606076885),
    ],
    &[
        ("selfloops", 0.5680493116870342),
        ("node_ids", 0.24453704075340602),
        ("singleton", 0.4275786542799005),
        ("iter", 0.2386552841783915),
        ("with", 0.4093678499510831),
        ("par", 0.38113291687472167),
        ("nodes", 0.4438464199484321),
    ],
    &[
        ("nodes", 0.5753714267498481),
        ("selfloops", 0.7363793606076885),
        ("with", 0.5306758223243937),
        ("node_names", 0.45679064037341954),
        ("singleton", 0.5542830341841882),
        ("iter", 0.309376003021643),
    ],
    &[
        ("singleton", 1.554937854069521),
        ("iter", 0.8678967757819245),
        ("node_type_ids", 1.9707384293652452),
    ],
    &[
        ("iter", 0.8678967757819245),
        ("singleton", 1.554937854069521),
        ("edge_type_ids", 2.271155030697057),
    ],
    &[
        ("singleton", 1.554937854069521),
        ("iter", 0.8678967757819245),
        ("node_type_names", 2.0657751945261844),
    ],
    &[
        ("edge_type_names", 2.443122069340597),
        ("singleton", 1.554937854069521),
        ("iter", 0.8678967757819245),
    ],
    &[
        ("source", 1.8408824970903248),
        ("node_ids", 0.8892864449231846),
        ("iter", 0.8678967757819245),
    ],
    &[
        ("weights", 2.1392835137358808),
        ("iter", 0.8678967757819245),
        ("edge", 1.0493269921053563),
    ],
    &[
        ("weights", 1.4386500244680516),
        ("edge", 0.7056635051756572),
        ("par", 0.9320952439549945),
        ("iter", 0.5836532230054678),
    ],
    &[
        ("par", 0.9320952439549945),
        ("node_ids", 0.5980375941446159),
        ("source", 1.2379778708511928),
        ("iter", 0.5836532230054678),
    ],
    &[
        ("directed", 0.9045134689315563),
        ("par", 0.6635612457370292),
        ("iter", 0.41550438353555247),
        ("node_ids", 0.4257446581149362),
        ("source", 0.8813199546982742),
    ],
    &[
        ("iter", 0.8678967757819245),
        ("node_ids", 0.8892864449231846),
        ("destination", 2.179936388475869),
    ],
    &[
        ("iter", 0.5836532230054678),
        ("node_ids", 0.5980375941446159),
        ("par", 0.9320952439549945),
        ("destination", 1.4659887380438164),
    ],
    &[
        ("node_ids", 0.5980375941446159),
        ("iter", 0.5836532230054678),
        ("directed", 1.270557477400446),
        ("destination", 1.4659887380438164),
    ],
    &[
        ("directed", 0.9045134689315563),
        ("node_ids", 0.4257446581149362),
        ("par", 0.6635612457370292),
        ("destination", 1.0436415372373469),
        ("iter", 0.41550438353555247),
    ],
    &[
        ("iter", 0.5836532230054678),
        ("node_ids", 0.5980375941446159),
        ("and", 0.9689100878988537),
        ("node_type_ids", 1.3253048842858886),
    ],
    &[
        ("unchecked", 1.1857366085956151),
        ("iter", 0.8678967757819245),
        ("node_type_ids", 1.9707384293652452),
    ],
    &[
        ("iter", 0.41550438353555247),
        ("hot", 1.1696413187027879),
        ("node_type_ids", 0.9434883030478971),
        ("one", 1.1696413187027879),
        ("encoded", 1.1696413187027879),
    ],
    &[
        ("one", 0.8708908268792546),
        ("encoded", 0.8708908268792546),
        ("node_type_ids", 0.7025019510285271),
        ("iter", 0.309376003021643),
        ("known", 0.6255440881216725),
        ("hot", 0.8708908268792546),
    ],
    &[
        ("iter", 0.309376003021643),
        ("and", 0.5135884091206121),
        ("node_type_ids", 0.7025019510285271),
        ("node_ids", 0.3170007004851363),
        ("par", 0.49407403170902714),
        ("unchecked", 0.42267521073949105),
    ],
    &[
        ("iter", 0.5836532230054678),
        ("node_type_names", 1.3892163030605715),
        ("and", 0.9689100878988537),
        ("node_names", 0.8617582711288273),
    ],
    &[
        ("node_type_names", 0.9889870231990723),
        ("par", 0.6635612457370292),
        ("and", 0.6897698374742284),
        ("iter", 0.41550438353555247),
        ("node_names", 0.6134881554465302),
    ],
    &[
        ("iter", 0.8678967757819245),
        ("edge", 1.0493269921053563),
        ("node_ids", 0.8892864449231846),
    ],
    &[
        ("edge", 0.7056635051756572),
        ("iter", 0.5836532230054678),
        ("node_ids", 0.5980375941446159),
        ("directed", 1.270557477400446),
    ],
    &[("iter", 1.3825382213996475), ("edges", 3.0940163713087676)],
    &[
        ("iter", 0.5836532230054678),
        ("edge", 0.7056635051756572),
        ("par", 0.9320952439549945),
        ("node_ids", 0.5980375941446159),
    ],
    &[
        ("iter", 0.41550438353555247),
        ("edge", 0.5023638491906379),
        ("node_ids", 0.4257446581149362),
        ("par", 0.6635612457370292),
        ("directed", 0.9045134689315563),
    ],
    &[
        ("edges", 1.9422875919892115),
        ("iter", 0.8678967757819245),
        ("par", 1.3860327075459795),
    ],
    &[
        ("par", 0.9320952439549945),
        ("iter", 0.5836532230054678),
        ("edges", 1.306171937378964),
        ("directed", 1.270557477400446),
    ],
    &[
        ("and", 0.5135884091206121),
        ("weight", 0.6923601846392166),
        ("node_ids", 0.3170007004851363),
        ("iter", 0.309376003021643),
        ("edge", 0.6919689423939748),
    ],
    &[
        ("iter", 0.2386552841783915),
        ("par", 0.38113291687472167),
        ("and", 0.3961864738450093),
        ("weight", 0.5340925443636185),
        ("edge", 0.5431057776406637),
        ("node_ids", 0.24453704075340602),
    ],
    &[
        ("and", 0.6897698374742284),
        ("edge", 0.5023638491906379),
        ("node_ids", 0.4257446581149362),
        ("iter", 0.41550438353555247),
        ("edge_type_id", 0.8128721531053885),
    ],
    &[
        ("node_ids", 0.3170007004851363),
        ("directed", 0.6734820925742219),
        ("edge", 0.37404977151551183),
        ("edge_type_id", 0.6052478569671313),
        ("iter", 0.309376003021643),
        ("and", 0.5135884091206121),
    ],
    &[
        ("iter", 0.41550438353555247),
        ("hot", 1.1696413187027879),
        ("edge_type_ids", 1.0873123363008848),
        ("encoded", 1.1696413187027879),
        ("one", 1.1696413187027879),
    ],
    &[
        ("one", 0.8708908268792546),
        ("known", 0.6255440881216725),
        ("edge_type_ids", 0.8095903628706466),
        ("iter", 0.309376003021643),
        ("encoded", 0.8708908268792546),
        ("hot", 0.8708908268792546),
    ],
    &[
        ("edge_type_name", 0.9434883030478971),
        ("iter", 0.41550438353555247),
        ("edge", 0.5023638491906379),
        ("node_names", 0.6134881554465302),
        ("and", 0.6897698374742284),
    ],
    &[
        ("par", 0.49407403170902714),
        ("edge", 0.37404977151551183),
        ("edge_type_name", 0.7025019510285271),
        ("and", 0.5135884091206121),
        ("iter", 0.309376003021643),
        ("node_names", 0.45679064037341954),
    ],
    &[
        ("par", 0.38113291687472167),
        ("iter", 0.2386552841783915),
        ("directed", 0.5195298233299449),
        ("edge", 0.28854518012391517),
        ("node_names", 0.35237219119648877),
        ("edge_type_name", 0.541915989349888),
        ("and", 0.3961864738450093),
    ],
    &[
        ("edge", 0.37404977151551183),
        ("iter", 0.309376003021643),
        ("par", 0.49407403170902714),
        ("node_ids", 0.3170007004851363),
        ("edge_type_id", 0.6052478569671313),
        ("and", 0.5135884091206121),
    ],
    &[
        ("iter", 0.2386552841783915),
        ("directed", 0.5195298233299449),
        ("node_ids", 0.24453704075340602),
        ("edge_type_id", 0.46689335272312055),
        ("edge", 0.28854518012391517),
        ("par", 0.38113291687472167),
        ("and", 0.3961864738450093),
    ],
    &[
        ("edge_type_name", 0.34920495738650303),
        ("edge", 0.35745742434068634),
        ("weight", 0.3441636118149813),
        ("iter", 0.15378695218341445),
        ("and", 0.4908063147630377),
        ("par", 0.2455980384625613),
        ("node_names", 0.22706492967401862),
    ],
    &[
        ("edge_type_name", 0.2890031250711541),
        ("iter", 0.12727456708749674),
        ("weight", 0.2848308915621068),
        ("edge", 0.2978229496510939),
        ("par", 0.20325771191290964),
        ("directed", 0.27706461049459286),
        ("node_names", 0.18791965257589885),
        ("and", 0.40892529967651725),
    ],
    &[
        ("edge", 0.4362893731943191),
        ("node_names", 0.2796273282069034),
        ("edge_type_name", 0.43004108723781687),
        ("iter", 0.18938650990211245),
        ("and", 0.5990463894343191),
        ("weight", 0.42383273971908675),
    ],
    &[
        ("weight", 0.3441636118149813),
        ("par", 0.2455980384625613),
        ("edge_type_id", 0.3008611603014561),
        ("iter", 0.15378695218341445),
        ("edge", 0.35745742434068634),
        ("and", 0.4908063147630377),
        ("node_ids", 0.157577094187897),
    ],
    &[
        ("iter", 0.12727456708749674),
        ("directed", 0.27706461049459286),
        ("weight", 0.2848308915621068),
        ("edge", 0.2978229496510939),
        ("node_ids", 0.1304113005747781),
        ("par", 0.20325771191290964),
        ("and", 0.40892529967651725),
        ("edge_type_id", 0.248993645996318),
    ],
    &[
        ("edge_type_id", 0.37050636809965865),
        ("iter", 0.18938650990211245),
        ("edge", 0.4362893731943191),
        ("node_ids", 0.19405401749018308),
        ("and", 0.5990463894343191),
        ("weight", 0.42383273971908675),
    ],
    &[
        ("weight", 0.3441636118149813),
        ("and", 0.4908063147630377),
        ("node_ids", 0.157577094187897),
        ("edge", 0.35745742434068634),
        ("iter", 0.15378695218341445),
        ("directed", 0.3347795477202934),
        ("edge_type_id", 0.3008611603014561),
    ],
    &[
        ("edge", 0.7056635051756572),
        ("unique", 1.2879504362973773),
        ("node_ids", 0.5980375941446159),
        ("iter", 0.5836532230054678),
    ],
    &[
        ("unique", 1.2879504362973773),
        ("iter", 0.5836532230054678),
        ("source", 1.2379778708511928),
        ("node_ids", 0.5980375941446159),
    ],
    &[
        ("iter", 0.41550438353555247),
        ("unique", 0.916895565662068),
        ("node_ids", 0.4257446581149362),
        ("par", 0.6635612457370292),
        ("source", 0.8813199546982742),
    ],
    &[
        ("unknown", 0.840130805785612),
        ("iter", 0.41550438353555247),
        ("edge_types", 0.7127189189159387),
        ("with", 0.7127189189159387),
        ("edge_ids", 0.9578263092235365),
    ],
    &[
        ("iter", 0.41550438353555247),
        ("known", 0.840130805785612),
        ("edge_types", 0.7127189189159387),
        ("with", 0.7127189189159387),
        ("edge_ids", 0.9578263092235365),
    ],
    &[
        ("edge", 0.37404977151551183),
        ("iter", 0.309376003021643),
        ("node_ids", 0.3170007004851363),
        ("edge_types", 0.5306758223243937),
        ("with", 0.5306758223243937),
        ("unknown", 0.6255440881216725),
    ],
    &[
        ("known", 0.6255440881216725),
        ("node_ids", 0.3170007004851363),
        ("with", 0.5306758223243937),
        ("iter", 0.309376003021643),
        ("edge", 0.37404977151551183),
        ("edge_types", 0.5306758223243937),
    ],
    &[
        ("with", 0.7127189189159387),
        ("iter", 0.41550438353555247),
        ("node_types", 0.7249618438397802),
        ("unknown", 0.840130805785612),
        ("node_ids", 0.4257446581149362),
    ],
    &[
        ("with", 0.7127189189159387),
        ("node_ids", 0.4257446581149362),
        ("known", 0.840130805785612),
        ("node_types", 0.7249618438397802),
        ("iter", 0.41550438353555247),
    ],
    &[
        ("node_names", 0.45679064037341954),
        ("edge", 0.37404977151551183),
        ("iter", 0.309376003021643),
        ("with", 0.5306758223243937),
        ("unknown", 0.6255440881216725),
        ("edge_types", 0.5306758223243937),
    ],
    &[
        ("edge", 0.37404977151551183),
        ("node_names", 0.45679064037341954),
        ("with", 0.5306758223243937),
        ("edge_types", 0.5306758223243937),
        ("iter", 0.309376003021643),
        ("known", 0.6255440881216725),
    ],
    &[
        ("node_types", 0.7249618438397802),
        ("node_names", 0.6134881554465302),
        ("iter", 0.41550438353555247),
        ("with", 0.7127189189159387),
        ("unknown", 0.840130805785612),
    ],
    &[
        ("iter", 0.41550438353555247),
        ("node_types", 0.7249618438397802),
        ("known", 0.840130805785612),
        ("with", 0.7127189189159387),
        ("node_names", 0.6134881554465302),
    ],
    &[
        ("edge_ids", 0.7131777350098516),
        ("unknown", 0.6255440881216725),
        ("par", 0.49407403170902714),
        ("iter", 0.309376003021643),
        ("edge_types", 0.5306758223243937),
        ("with", 0.5306758223243937),
    ],
    &[
        ("par", 0.49407403170902714),
        ("known", 0.6255440881216725),
        ("edge_ids", 0.7131777350098516),
        ("edge_types", 0.5306758223243937),
        ("iter", 0.309376003021643),
        ("with", 0.5306758223243937),
    ],
    &[
        ("iter", 0.2386552841783915),
        ("with", 0.4093678499510831),
        ("edge_types", 0.4093678499510831),
        ("edge", 0.28854518012391517),
        ("node_ids", 0.24453704075340602),
        ("par", 0.38113291687472167),
        ("unknown", 0.48255003833101656),
    ],
    &[
        ("known", 0.48255003833101656),
        ("iter", 0.2386552841783915),
        ("edge", 0.28854518012391517),
        ("par", 0.38113291687472167),
        ("edge_types", 0.4093678499510831),
        ("node_ids", 0.24453704075340602),
        ("with", 0.4093678499510831),
    ],
    &[
        ("par", 0.49407403170902714),
        ("iter", 0.309376003021643),
        ("node_types", 0.5397916519722127),
        ("node_ids", 0.3170007004851363),
        ("with", 0.5306758223243937),
        ("unknown", 0.6255440881216725),
    ],
    &[
        ("node_ids", 0.3170007004851363),
        ("par", 0.49407403170902714),
        ("known", 0.6255440881216725),
        ("with", 0.5306758223243937),
        ("iter", 0.309376003021643),
        ("node_types", 0.5397916519722127),
    ],
    &[
        ("edge_types", 0.4093678499510831),
        ("node_names", 0.35237219119648877),
        ("with", 0.4093678499510831),
        ("edge", 0.28854518012391517),
        ("par", 0.38113291687472167),
        ("unknown", 0.48255003833101656),
        ("iter", 0.2386552841783915),
    ],
    &[
        ("known", 0.48255003833101656),
        ("node_names", 0.35237219119648877),
        ("edge_types", 0.4093678499510831),
        ("iter", 0.2386552841783915),
        ("par", 0.38113291687472167),
        ("edge", 0.28854518012391517),
        ("with", 0.4093678499510831),
    ],
    &[
        ("par", 0.49407403170902714),
        ("node_names", 0.45679064037341954),
        ("iter", 0.309376003021643),
        ("with", 0.5306758223243937),
        ("node_types", 0.5397916519722127),
        ("unknown", 0.6255440881216725),
    ],
    &[
        ("known", 0.6255440881216725),
        ("par", 0.49407403170902714),
        ("iter", 0.309376003021643),
        ("node_names", 0.45679064037341954),
        ("node_types", 0.5397916519722127),
        ("with", 0.5306758223243937),
    ],
    &[
        ("get", 0.2460207953237388),
        ("total", 1.4954324319187933),
        ("edge", 0.7056635051756572),
        ("weights", 1.4386500244680516),
    ],
    &[
        ("weight", 1.306171937378964),
        ("mininum", 2.3066011683672523),
        ("get", 0.2460207953237388),
        ("edge", 0.7056635051756572),
    ],
    &[
        ("maximum", 1.6429788769312366),
        ("edge", 0.7056635051756572),
        ("get", 0.2460207953237388),
        ("weight", 1.306171937378964),
    ],
    &[
        ("get", 0.17514290141589564),
        ("maximum", 1.1696413187027879),
        ("unchecked", 0.5676697648129673),
        ("degree", 0.916895565662068),
        ("node", 0.6176783799372239),
    ],
    &[
        ("get", 0.17514290141589564),
        ("degree", 0.916895565662068),
        ("minimum", 1.2037306546564002),
        ("unchecked", 0.5676697648129673),
        ("node", 0.6176783799372239),
    ],
    &[
        ("degree", 0.916895565662068),
        ("node", 0.6176783799372239),
        ("weighted", 0.718771595110523),
        ("maximum", 1.1696413187027879),
        ("get", 0.17514290141589564),
    ],
    &[
        ("degree", 0.916895565662068),
        ("node", 0.6176783799372239),
        ("get", 0.17514290141589564),
        ("minimum", 1.2037306546564002),
        ("weighted", 0.718771595110523),
    ],
    &[
        ("get", 0.17514290141589564),
        ("weighted", 0.718771595110523),
        ("singleton", 0.7444243515125029),
        ("number", 0.7803129291521904),
        ("nodes", 0.7727469087476133),
    ],
    &[
        ("number", 1.629901156636506),
        ("get", 0.36583479131199337),
        ("selfloops", 2.0657751945261844),
    ],
    &[
        ("selfloops", 1.3892163030605715),
        ("unique", 1.2879504362973773),
        ("number", 1.096094708260868),
        ("get", 0.2460207953237388),
    ],
    &[
        ("edges", 0.6923601846392166),
        ("new", 1.2226558886454175),
        ("node", 0.4599105951947464),
        ("features", 1.2226558886454175),
        ("generate", 0.896273044061298),
        ("from", 0.26278152728588133),
    ],
    &[("set_name", 9.263184627320559)],
    &[
        ("inplace", 1.4954324319187933),
        ("set", 1.8095225182222892),
        ("all", 1.6429788769312366),
        ("edge_types", 1.0011463431087175),
    ],
    &[
        ("set", 2.690773728932532),
        ("edge_types", 1.4887122164692992),
        ("all", 2.443122069340597),
    ],
    &[
        ("inplace", 1.4954324319187933),
        ("node_types", 1.0183438093063337),
        ("set", 1.8095225182222892),
        ("all", 1.6429788769312366),
    ],
    &[
        ("set", 2.690773728932532),
        ("all", 2.443122069340597),
        ("node_types", 1.5142849793295328),
    ],
    &[
        ("remove", 1.9707384293652452),
        ("inplace", 2.223719384909289),
        ("node_type_ids", 1.9707384293652452),
    ],
    &[
        ("remove", 1.3253048842858886),
        ("singleton", 1.0456825228820985),
        ("inplace", 1.4954324319187933),
        ("node_types", 1.0183438093063337),
    ],
    &[
        ("edge_type_ids", 2.271155030697057),
        ("remove", 1.9707384293652452),
        ("inplace", 2.223719384909289),
    ],
    &[
        ("remove", 1.3253048842858886),
        ("edge_types", 1.0011463431087175),
        ("inplace", 1.4954324319187933),
        ("singleton", 1.0456825228820985),
    ],
    &[
        ("inplace", 2.223719384909289),
        ("node_type_name", 2.443122069340597),
        ("remove", 1.9707384293652452),
    ],
    &[
        ("remove", 3.1393378556151865),
        ("node_type_id", 3.700336452436737),
    ],
    &[
        ("remove", 1.9707384293652452),
        ("singleton", 1.554937854069521),
        ("node_types", 1.5142849793295328),
    ],
    &[
        ("node_type_name", 3.8918333777254275),
        ("remove", 3.1393378556151865),
    ],
    &[
        ("inplace", 2.223719384909289),
        ("remove", 1.9707384293652452),
        ("edge_type_name", 1.9707384293652452),
    ],
    &[
        ("remove", 3.1393378556151865),
        ("edge_type_id", 2.704729156445749),
    ],
    &[
        ("edge_types", 1.4887122164692992),
        ("remove", 1.9707384293652452),
        ("singleton", 1.554937854069521),
    ],
    &[
        ("remove", 3.1393378556151865),
        ("edge_type_name", 3.1393378556151865),
    ],
    &[
        ("remove", 1.9707384293652452),
        ("node_types", 1.5142849793295328),
        ("inplace", 2.223719384909289),
    ],
    &[
        ("remove", 3.1393378556151865),
        ("node_types", 2.4122187343400157),
    ],
    &[
        ("inplace", 2.223719384909289),
        ("edge_types", 1.4887122164692992),
        ("remove", 1.9707384293652452),
    ],
    &[
        ("remove", 3.1393378556151865),
        ("edge_types", 2.3714819519626307),
    ],
    &[
        ("edge", 0.7056635051756572),
        ("weights", 1.4386500244680516),
        ("remove", 1.3253048842858886),
        ("inplace", 1.4954324319187933),
    ],
    &[
        ("remove", 1.9707384293652452),
        ("weights", 2.1392835137358808),
        ("edge", 1.0493269921053563),
    ],
    &[
        ("get", 0.36583479131199337),
        ("stats", 3.429933457288565),
        ("memory", 2.223719384909289),
    ],
    &[
        ("get", 0.2460207953237388),
        ("memory", 1.4954324319187933),
        ("used", 2.3066011683672523),
        ("total", 1.4954324319187933),
    ],
    &[
        ("memory", 1.0646025863506097),
        ("get", 0.17514290141589564),
        ("nodes", 0.7727469087476133),
        ("requirement", 1.3428587229621909),
        ("total", 1.0646025863506097),
    ],
    &[
        ("total", 0.6114809923799893),
        ("human", 0.7399121495290593),
        ("get", 0.10059768456248329),
        ("memory", 0.6114809923799893),
        ("nodes", 0.4438464199484321),
        ("readable", 0.7399121495290593),
        ("requirement", 0.7713043299639504),
    ],
    &[
        ("total", 1.0646025863506097),
        ("edges", 0.9298675039219312),
        ("memory", 1.0646025863506097),
        ("get", 0.17514290141589564),
        ("requirement", 1.3428587229621909),
    ],
    &[
        ("human", 0.7399121495290593),
        ("readable", 0.7399121495290593),
        ("get", 0.10059768456248329),
        ("requirement", 0.7713043299639504),
        ("memory", 0.6114809923799893),
        ("total", 0.6114809923799893),
        ("edges", 0.5340925443636185),
    ],
    &[
        ("memory", 0.7926811509642554),
        ("weights", 0.7625826034592891),
        ("get", 0.13040779578930034),
        ("total", 0.7926811509642554),
        ("edge", 0.37404977151551183),
        ("requirements", 0.9252929025156035),
    ],
    &[
        ("human", 0.5871622751446547),
        ("weights", 0.4668198450127302),
        ("readable", 0.5871622751446547),
        ("memory", 0.4852448644370381),
        ("edge", 0.22897697321420768),
        ("get", 0.07982997086828104),
        ("total", 0.4852448644370381),
        ("requirements", 0.5664240009990903),
    ],
    &[
        ("requirements", 1.242705488772741),
        ("get", 0.17514290141589564),
        ("total", 1.0646025863506097),
        ("node_types", 0.7249618438397802),
        ("memory", 1.0646025863506097),
    ],
    &[
        ("memory", 0.6114809923799893),
        ("human", 0.7399121495290593),
        ("total", 0.6114809923799893),
        ("get", 0.10059768456248329),
        ("node_types", 0.416399878595431),
        ("readable", 0.7399121495290593),
        ("requirements", 0.7137788271919129),
    ],
    &[
        ("edge_types", 0.7127189189159387),
        ("total", 1.0646025863506097),
        ("memory", 1.0646025863506097),
        ("get", 0.17514290141589564),
        ("requirements", 1.242705488772741),
    ],
    &[
        ("total", 0.6114809923799893),
        ("requirements", 0.7137788271919129),
        ("human", 0.7399121495290593),
        ("get", 0.10059768456248329),
        ("readable", 0.7399121495290593),
        ("memory", 0.6114809923799893),
        ("edge_types", 0.4093678499510831),
    ],
    &[
        ("triangles", 2.1111699077238817),
        ("of", 2.1111699077238817),
        ("number", 1.096094708260868),
        ("get", 0.2460207953237388),
    ],
    &[
        ("number", 1.629901156636506),
        ("get", 0.36583479131199337),
        ("triads", 3.1393256883021006),
    ],
    &[
        ("weighted", 1.0096484530954233),
        ("triads", 2.1111699077238817),
        ("number", 1.096094708260868),
        ("get", 0.2460207953237388),
    ],
    &[
        ("transitivity", 5.46379965208028),
        ("get", 0.582765826328731),
    ],
    &[
        ("node", 0.4599105951947464),
        ("get", 0.13040779578930034),
        ("of", 1.1190639955483745),
        ("per", 1.0508297599412841),
        ("number", 0.5810049296545123),
        ("triangles", 1.1190639955483745),
    ],
    &[
        ("coefficient", 1.2037306546564002),
        ("node", 0.6176783799372239),
        ("iter", 0.41550438353555247),
        ("clustering", 1.3428587229621909),
        ("per", 1.4113065245550764),
    ],
    &[
        ("get", 0.17514290141589564),
        ("per", 1.4113065245550764),
        ("node", 0.6176783799372239),
        ("clustering", 1.3428587229621909),
        ("coefficient", 1.2037306546564002),
    ],
    &[
        ("get", 0.36583479131199337),
        ("coefficient", 2.5143271538957523),
        ("clustering", 2.804934922882217),
    ],
    &[
        ("get", 0.2460207953237388),
        ("average", 2.3066011683672523),
        ("clustering", 1.8862949532055344),
        ("coefficient", 1.6908636925621634),
    ],
    &[
        ("nodes", 1.6140974131025445),
        ("are", 3.429933457288565),
        ("remappable", 3.429933457288565),
    ],
    &[
        ("from", 0.4957504261763134),
        ("unchecked", 0.7973978156779057),
        ("node_ids", 0.5980375941446159),
        ("remap", 1.8862949532055344),
    ],
    &[
        ("remap", 2.804934922882217),
        ("from", 0.7371846492260467),
        ("node_ids", 0.8892864449231846),
    ],
    &[
        ("node_names", 1.2814410947048054),
        ("remap", 2.804934922882217),
        ("from", 0.7371846492260467),
    ],
    &[
        ("graph", 2.322908709986271),
        ("remap", 2.804934922882217),
        ("from", 0.7371846492260467),
    ],
    &[
        ("negatives", 5.46379965208028),
        ("sample", 5.46379965208028),
    ],
    &[
        ("connected", 3.6178941140020378),
        ("holdout", 3.8918333777254275),
    ],
    &[
        ("random", 3.472585060098304),
        ("holdout", 3.8918333777254275),
    ],
    &[
        ("get", 0.17514290141589564),
        ("node", 0.6176783799372239),
        ("holdout", 1.1696413187027879),
        ("label", 1.1120893388302326),
        ("indices", 1.6420759086870347),
    ],
    &[
        ("node", 0.6176783799372239),
        ("get", 0.17514290141589564),
        ("holdout", 1.1696413187027879),
        ("labels", 1.6420759086870347),
        ("label", 1.1120893388302326),
    ],
    &[
        ("graphs", 1.5029478403812442),
        ("label", 1.1120893388302326),
        ("get", 0.17514290141589564),
        ("node", 0.6176783799372239),
        ("holdout", 1.1696413187027879),
    ],
    &[
        ("get", 0.17514290141589564),
        ("edge", 0.5023638491906379),
        ("holdout", 1.1696413187027879),
        ("graphs", 1.5029478403812442),
        ("label", 1.1120893388302326),
    ],
    &[
        ("get", 0.36583479131199337),
        ("subgraph", 3.429933457288565),
        ("random", 2.179936388475869),
    ],
    &[
        ("label", 1.1120893388302326),
        ("holdout", 1.1696413187027879),
        ("random", 1.0436415372373469),
        ("node", 0.6176783799372239),
        ("get", 0.17514290141589564),
    ],
    &[
        ("kfold", 1.9824426271900815),
        ("node", 0.8676442211356591),
        ("label", 1.5621364120283636),
        ("get", 0.2460207953237388),
    ],
    &[
        ("label", 1.1120893388302326),
        ("get", 0.17514290141589564),
        ("random", 1.0436415372373469),
        ("edge", 0.5023638491906379),
        ("holdout", 1.1696413187027879),
    ],
    &[
        ("get", 0.2460207953237388),
        ("label", 1.5621364120283636),
        ("kfold", 1.9824426271900815),
        ("edge", 0.7056635051756572),
    ],
    &[
        ("get", 0.2460207953237388),
        ("prediction", 1.745611099447606),
        ("edge", 0.7056635051756572),
        ("kfold", 1.9824426271900815),
    ],
    &[
        ("node_ids", 0.24453704075340602),
        ("from", 0.20271190867656422),
        ("breadth", 0.7713043299639504),
        ("first", 0.7713043299639504),
        ("unchecked", 0.32605525816149983),
        ("get", 0.10059768456248329),
        ("search", 0.7713043299639504),
    ],
    &[
        ("from", 0.20271190867656422),
        ("unchecked", 0.32605525816149983),
        ("get", 0.10059768456248329),
        ("path", 0.5882627164374559),
        ("shortest", 0.5680493116870342),
        ("node_ids", 0.46027273657210455),
    ],
    &[
        ("path", 0.5882627164374559),
        ("node_names", 0.35237219119648877),
        ("from", 0.20271190867656422),
        ("shortest", 0.5680493116870342),
        ("node_ids", 0.24453704075340602),
        ("get", 0.10059768456248329),
        ("unchecked", 0.32605525816149983),
    ],
    &[
        ("from", 0.26278152728588133),
        ("path", 0.7625826034592891),
        ("get", 0.13040779578930034),
        ("node_ids", 0.5864316894623536),
        ("shortest", 0.7363793606076885),
    ],
    &[
        ("shortest", 0.7363793606076885),
        ("node_ids", 0.3170007004851363),
        ("get", 0.13040779578930034),
        ("path", 0.7625826034592891),
        ("from", 0.26278152728588133),
        ("node_names", 0.45679064037341954),
    ],
    &[
        ("path", 0.7625826034592891),
        ("get", 0.13040779578930034),
        ("from", 0.26278152728588133),
        ("node_names", 0.8450344322735499),
        ("shortest", 0.7363793606076885),
    ],
    &[
        ("get", 0.07982997086828104),
        ("k", 0.5486593080143868),
        ("from", 0.1608634019230581),
        ("shortest", 0.45077936138338115),
        ("path", 0.4668198450127302),
        ("node_ids", 0.3697476845299576),
        ("unchecked", 0.25874334855406383),
    ],
    &[
        ("k", 0.6913926611719697),
        ("get", 0.10059768456248329),
        ("path", 0.5882627164374559),
        ("from", 0.20271190867656422),
        ("node_ids", 0.46027273657210455),
        ("shortest", 0.5680493116870342),
    ],
    &[
        ("from", 0.20271190867656422),
        ("node_names", 0.35237219119648877),
        ("shortest", 0.5680493116870342),
        ("get", 0.10059768456248329),
        ("k", 0.6913926611719697),
        ("node_ids", 0.24453704075340602),
        ("path", 0.5882627164374559),
    ],
    &[
        ("get", 0.10059768456248329),
        ("shortest", 0.5680493116870342),
        ("from", 0.20271190867656422),
        ("k", 0.6913926611719697),
        ("node_names", 0.6632423138606159),
        ("path", 0.5882627164374559),
    ],
    &[
        ("get", 0.17514290141589564),
        ("unchecked", 0.5676697648129673),
        ("node_id", 0.6586091234986925),
        ("eccentricity", 1.242705488772741),
        ("from", 0.35292613335563944),
    ],
    &[
        ("eccentricity", 0.9252929025156035),
        ("unchecked", 0.42267521073949105),
        ("from", 0.26278152728588133),
        ("get", 0.13040779578930034),
        ("weighted", 0.5351825203109013),
        ("node_id", 0.49038678352277515),
    ],
    &[
        ("get", 0.2460207953237388),
        ("eccentricity", 1.745611099447606),
        ("node_id", 0.9251390667889959),
        ("from", 0.4957504261763134),
    ],
    &[
        ("node_id", 0.6586091234986925),
        ("get", 0.17514290141589564),
        ("eccentricity", 1.242705488772741),
        ("from", 0.35292613335563944),
        ("weighted", 0.718771595110523),
    ],
    &[
        ("node_name", 1.2879504362973773),
        ("from", 0.4957504261763134),
        ("get", 0.2460207953237388),
        ("eccentricity", 1.745611099447606),
    ],
    &[
        ("weighted", 0.718771595110523),
        ("get", 0.17514290141589564),
        ("eccentricity", 1.242705488772741),
        ("from", 0.35292613335563944),
        ("node_name", 0.916895565662068),
    ],
    &[
        ("node_ids", 0.4257446581149362),
        ("unchecked", 0.5676697648129673),
        ("get", 0.17514290141589564),
        ("from", 0.35292613335563944),
        ("dijkstra", 1.4113065245550764),
    ],
    &[
        ("get", 0.07982997086828104),
        ("shortest", 0.45077936138338115),
        ("weighted", 0.32761542166283475),
        ("unchecked", 0.25874334855406383),
        ("path", 0.4668198450127302),
        ("node_ids", 0.3697476845299576),
        ("from", 0.1608634019230581),
    ],
    &[
        ("path", 0.4668198450127302),
        ("node_names", 0.2796273282069034),
        ("shortest", 0.45077936138338115),
        ("node_ids", 0.19405401749018308),
        ("get", 0.07982997086828104),
        ("weighted", 0.32761542166283475),
        ("from", 0.1608634019230581),
        ("unchecked", 0.25874334855406383),
    ],
    &[
        ("from", 0.20271190867656422),
        ("path", 0.5882627164374559),
        ("weighted", 0.41284435516858997),
        ("shortest", 0.5680493116870342),
        ("node_ids", 0.46027273657210455),
        ("get", 0.10059768456248329),
    ],
    &[
        ("from", 0.20271190867656422),
        ("path", 0.5882627164374559),
        ("node_ids", 0.24453704075340602),
        ("shortest", 0.5680493116870342),
        ("get", 0.10059768456248329),
        ("weighted", 0.41284435516858997),
        ("node_names", 0.35237219119648877),
    ],
    &[
        ("path", 0.5882627164374559),
        ("get", 0.10059768456248329),
        ("node_names", 0.6632423138606159),
        ("from", 0.20271190867656422),
        ("weighted", 0.41284435516858997),
        ("shortest", 0.5680493116870342),
    ],
    &[
        ("search", 0.999864937158341),
        ("breadth", 0.999864937158341),
        ("get", 0.13040779578930034),
        ("from", 0.26278152728588133),
        ("node_ids", 0.3170007004851363),
        ("first", 0.999864937158341),
    ],
    &[
        ("get", 0.2460207953237388),
        ("dijkstra", 1.9824426271900815),
        ("from", 0.4957504261763134),
        ("node_ids", 0.5980375941446159),
    ],
    &[
        ("diameter", 2.9479072443926193),
        ("naive", 3.1393256883021006),
        ("get", 0.36583479131199337),
    ],
    &[("diameter", 4.695943748427725), ("get", 0.582765826328731)],
    &[
        ("diameter", 1.9824426271900815),
        ("naive", 2.1111699077238817),
        ("get", 0.2460207953237388),
        ("weighted", 1.0096484530954233),
    ],
    &[
        ("breadth", 0.999864937158341),
        ("first", 0.999864937158341),
        ("search", 0.999864937158341),
        ("node_names", 0.45679064037341954),
        ("from", 0.26278152728588133),
        ("get", 0.13040779578930034),
    ],
    &[
        ("dijkstra", 1.9824426271900815),
        ("node_names", 0.8617582711288273),
        ("get", 0.2460207953237388),
        ("from", 0.4957504261763134),
    ],
    &[
        ("number", 1.096094708260868),
        ("components", 1.8862949532055344),
        ("connected", 1.5273325015145505),
        ("get", 0.2460207953237388),
    ],
    &[
        ("connected", 1.5273325015145505),
        ("nodes", 1.0854668247307808),
        ("get", 0.2460207953237388),
        ("number", 1.096094708260868),
    ],
    &[
        ("with", 0.5306758223243937),
        ("get", 0.13040779578930034),
        ("singleton", 0.5542830341841882),
        ("selfloops", 0.7363793606076885),
        ("number", 0.5810049296545123),
        ("nodes", 0.5753714267498481),
    ],
    &[
        ("number", 1.096094708260868),
        ("get", 0.2460207953237388),
        ("singleton", 1.0456825228820985),
        ("nodes", 1.0854668247307808),
    ],
    &[
        ("number", 1.096094708260868),
        ("get", 0.2460207953237388),
        ("disconnected", 1.8862949532055344),
        ("nodes", 1.0854668247307808),
    ],
    &[
        ("node_ids", 0.8892864449231846),
        ("get", 0.36583479131199337),
        ("singleton", 1.554937854069521),
    ],
    &[
        ("singleton", 1.554937854069521),
        ("node_names", 1.2814410947048054),
        ("get", 0.36583479131199337),
    ],
    &[
        ("node_ids", 0.4257446581149362),
        ("selfloops", 0.9889870231990723),
        ("singleton", 0.7444243515125029),
        ("with", 0.7127189189159387),
        ("get", 0.17514290141589564),
    ],
    &[
        ("node_names", 0.6134881554465302),
        ("singleton", 0.7444243515125029),
        ("with", 0.7127189189159387),
        ("selfloops", 0.9889870231990723),
        ("get", 0.17514290141589564),
    ],
    &[("get", 0.582765826328731), ("density", 5.46379965208028)],
    &[
        ("nodes", 1.0854668247307808),
        ("get", 0.2460207953237388),
        ("rate", 1.745611099447606),
        ("trap", 1.8095225182222892),
    ],
    &[
        ("mean", 2.1111699077238817),
        ("degrees", 1.4131352914829942),
        ("get", 0.2460207953237388),
        ("node", 0.8676442211356591),
    ],
    &[
        ("weighted", 0.718771595110523),
        ("mean", 1.5029478403812442),
        ("degrees", 1.006015018843602),
        ("node", 0.6176783799372239),
        ("get", 0.17514290141589564),
    ],
    &[
        ("undirected", 1.745611099447606),
        ("number", 1.096094708260868),
        ("edges", 1.306171937378964),
        ("get", 0.2460207953237388),
    ],
    &[
        ("undirected", 1.242705488772741),
        ("number", 0.7803129291521904),
        ("get", 0.17514290141589564),
        ("edges", 0.9298675039219312),
        ("unique", 0.916895565662068),
    ],
    &[
        ("number", 1.629901156636506),
        ("get", 0.36583479131199337),
        ("edges", 1.9422875919892115),
    ],
    &[
        ("number", 1.096094708260868),
        ("get", 0.2460207953237388),
        ("unique", 1.2879504362973773),
        ("edges", 1.306171937378964),
    ],
    &[
        ("node", 0.8676442211356591),
        ("degrees", 1.4131352914829942),
        ("get", 0.2460207953237388),
        ("median", 2.1111699077238817),
    ],
    &[
        ("degrees", 1.006015018843602),
        ("get", 0.17514290141589564),
        ("median", 1.5029478403812442),
        ("node", 0.6176783799372239),
        ("weighted", 0.718771595110523),
    ],
    &[
        ("get", 0.2460207953237388),
        ("maximum", 1.6429788769312366),
        ("degree", 1.2879504362973773),
        ("node", 0.8676442211356591),
    ],
    &[
        ("get", 0.17514290141589564),
        ("most", 1.5029478403812442),
        ("node_id", 0.6586091234986925),
        ("central", 1.2882042089239163),
        ("unchecked", 0.5676697648129673),
    ],
    &[
        ("get", 0.2460207953237388),
        ("most", 2.1111699077238817),
        ("central", 1.8095225182222892),
        ("node_id", 0.9251390667889959),
    ],
    &[
        ("node", 0.8676442211356591),
        ("degree", 1.2879504362973773),
        ("minimum", 1.6908636925621634),
        ("get", 0.2460207953237388),
    ],
    &[
        ("degrees", 1.4131352914829942),
        ("mode", 2.3066011683672523),
        ("get", 0.2460207953237388),
        ("node", 0.8676442211356591),
    ],
    &[
        ("selfloop", 2.1111699077238817),
        ("get", 0.2460207953237388),
        ("nodes", 1.0854668247307808),
        ("rate", 1.745611099447606),
    ],
    &[("get_name", 9.263184627320559)],
    &[
        ("nodes", 1.0854668247307808),
        ("trap", 1.8095225182222892),
        ("get", 0.2460207953237388),
        ("number", 1.096094708260868),
    ],
    &[
        ("node_ids", 0.8892864449231846),
        ("source", 1.8408824970903248),
        ("get", 0.36583479131199337),
    ],
    &[
        ("directed", 1.270557477400446),
        ("node_ids", 0.5980375941446159),
        ("source", 1.2379778708511928),
        ("get", 0.2460207953237388),
    ],
    &[
        ("source_names", 5.46379965208028),
        ("get", 0.582765826328731),
    ],
    &[
        ("node_ids", 0.8892864449231846),
        ("destination", 2.179936388475869),
        ("get", 0.36583479131199337),
    ],
    &[
        ("destination", 1.4659887380438164),
        ("get", 0.2460207953237388),
        ("directed", 1.270557477400446),
        ("node_ids", 0.5980375941446159),
    ],
    &[
        ("destination_names", 5.46379965208028),
        ("get", 0.582765826328731),
    ],
    &[
        ("node_names", 2.041304151989104),
        ("get", 0.582765826328731),
    ],
    &[
        ("get", 0.36583479131199337),
        ("urls", 2.9479072443926193),
        ("node", 1.2901935470720483),
    ],
    &[
        ("node", 1.2901935470720483),
        ("get", 0.36583479131199337),
        ("ontologies", 2.9479072443926193),
    ],
    &[("get", 0.582765826328731), ("node_ids", 1.4166114383490274)],
    &[
        ("edge_type_ids", 3.6178941140020378),
        ("get", 0.582765826328731),
    ],
    &[
        ("get", 0.36583479131199337),
        ("edge_type_ids", 2.271155030697057),
        ("unique", 1.9151920814783965),
    ],
    &[
        ("edge_type_names", 3.8918333777254275),
        ("get", 0.582765826328731),
    ],
    &[
        ("get", 0.36583479131199337),
        ("unique", 1.9151920814783965),
        ("edge_type_names", 2.443122069340597),
    ],
    &[
        ("get", 0.36583479131199337),
        ("edge", 1.0493269921053563),
        ("weights", 2.1392835137358808),
    ],
    &[
        ("weighted", 1.0096484530954233),
        ("node", 0.8676442211356591),
        ("get", 0.2460207953237388),
        ("indegrees", 2.1111699077238817),
    ],
    &[
        ("node_type_ids", 3.1393378556151865),
        ("get", 0.582765826328731),
    ],
    &[
        ("known", 1.180120046798488),
        ("node_types", 1.0183438093063337),
        ("get", 0.2460207953237388),
        ("mask", 1.745611099447606),
    ],
    &[
        ("node_types", 1.0183438093063337),
        ("unknown", 1.180120046798488),
        ("get", 0.2460207953237388),
        ("mask", 1.745611099447606),
    ],
    &[
        ("node_types", 0.7249618438397802),
        ("one", 1.1696413187027879),
        ("encoded", 1.1696413187027879),
        ("hot", 1.1696413187027879),
        ("get", 0.17514290141589564),
    ],
    &[
        ("known", 0.6255440881216725),
        ("encoded", 0.8708908268792546),
        ("hot", 0.8708908268792546),
        ("one", 0.8708908268792546),
        ("node_types", 0.5397916519722127),
        ("get", 0.13040779578930034),
    ],
    &[
        ("get", 0.17514290141589564),
        ("hot", 1.1696413187027879),
        ("encoded", 1.1696413187027879),
        ("edge_types", 0.7127189189159387),
        ("one", 1.1696413187027879),
    ],
    &[
        ("edge_types", 0.5306758223243937),
        ("one", 0.8708908268792546),
        ("known", 0.6255440881216725),
        ("hot", 0.8708908268792546),
        ("get", 0.13040779578930034),
        ("encoded", 0.8708908268792546),
    ],
    &[
        ("node_type_names", 3.290729085470608),
        ("get", 0.582765826328731),
    ],
    &[
        ("get", 0.36583479131199337),
        ("unique", 1.9151920814783965),
        ("node_type_ids", 1.9707384293652452),
    ],
    &[
        ("unique", 1.9151920814783965),
        ("get", 0.36583479131199337),
        ("node_type_names", 2.0657751945261844),
    ],
    &[
        ("number", 0.7803129291521904),
        ("edges", 0.9298675039219312),
        ("unique", 0.916895565662068),
        ("get", 0.17514290141589564),
        ("directed", 0.9045134689315563),
    ],
    &[
        ("nodes", 1.6140974131025445),
        ("mapping", 3.1393256883021006),
        ("get", 0.36583479131199337),
    ],
    &[
        ("edge", 1.0493269921053563),
        ("get", 0.36583479131199337),
        ("node_ids", 0.8892864449231846),
    ],
    &[
        ("edge", 0.7056635051756572),
        ("node_ids", 0.5980375941446159),
        ("get", 0.2460207953237388),
        ("directed", 1.270557477400446),
    ],
    &[
        ("get", 0.36583479131199337),
        ("edge", 1.0493269921053563),
        ("node_names", 1.2814410947048054),
    ],
    &[
        ("node_names", 0.8617582711288273),
        ("edge", 0.7056635051756572),
        ("directed", 1.270557477400446),
        ("get", 0.2460207953237388),
    ],
    &[
        ("unknown", 1.180120046798488),
        ("node_types", 1.0183438093063337),
        ("number", 1.096094708260868),
        ("get", 0.2460207953237388),
    ],
    &[
        ("node_types", 1.0183438093063337),
        ("number", 1.096094708260868),
        ("known", 1.180120046798488),
        ("get", 0.2460207953237388),
    ],
    &[
        ("rate", 1.745611099447606),
        ("unknown", 1.180120046798488),
        ("get", 0.2460207953237388),
        ("node_types", 1.0183438093063337),
    ],
    &[
        ("known", 1.180120046798488),
        ("get", 0.2460207953237388),
        ("rate", 1.745611099447606),
        ("node_types", 1.0183438093063337),
    ],
    &[
        ("minimum", 1.6908636925621634),
        ("node_types", 1.0183438093063337),
        ("number", 1.096094708260868),
        ("get", 0.2460207953237388),
    ],
    &[
        ("number", 1.096094708260868),
        ("maximum", 1.6429788769312366),
        ("get", 0.2460207953237388),
        ("node_types", 1.0183438093063337),
    ],
    &[
        ("multilabel", 2.1111699077238817),
        ("maximum", 1.6429788769312366),
        ("count", 1.745611099447606),
        ("get", 0.2460207953237388),
    ],
    &[
        ("singleton", 1.0456825228820985),
        ("get", 0.2460207953237388),
        ("number", 1.096094708260868),
        ("node_types", 1.0183438093063337),
    ],
    &[
        ("node_type_ids", 1.9707384293652452),
        ("get", 0.36583479131199337),
        ("singleton", 1.554937854069521),
    ],
    &[
        ("node_type_names", 2.0657751945261844),
        ("get", 0.36583479131199337),
        ("singleton", 1.554937854069521),
    ],
    &[
        ("number", 1.096094708260868),
        ("get", 0.2460207953237388),
        ("edge_types", 1.0011463431087175),
        ("unknown", 1.180120046798488),
    ],
    &[
        ("get", 0.17514290141589564),
        ("edge_ids", 0.9578263092235365),
        ("edge_types", 0.7127189189159387),
        ("unknown", 0.840130805785612),
        ("with", 0.7127189189159387),
    ],
    &[
        ("edge_types", 0.7127189189159387),
        ("with", 0.7127189189159387),
        ("known", 0.840130805785612),
        ("get", 0.17514290141589564),
        ("edge_ids", 0.9578263092235365),
    ],
    &[
        ("node_ids", 0.3170007004851363),
        ("edge_types", 0.5306758223243937),
        ("edge", 0.37404977151551183),
        ("unknown", 0.6255440881216725),
        ("get", 0.13040779578930034),
        ("with", 0.5306758223243937),
    ],
    &[
        ("edge_types", 0.5306758223243937),
        ("node_ids", 0.3170007004851363),
        ("known", 0.6255440881216725),
        ("get", 0.13040779578930034),
        ("edge", 0.37404977151551183),
        ("with", 0.5306758223243937),
    ],
    &[
        ("edge", 0.37404977151551183),
        ("get", 0.13040779578930034),
        ("unknown", 0.6255440881216725),
        ("node_names", 0.45679064037341954),
        ("with", 0.5306758223243937),
        ("edge_types", 0.5306758223243937),
    ],
    &[
        ("node_names", 0.45679064037341954),
        ("known", 0.6255440881216725),
        ("edge_types", 0.5306758223243937),
        ("edge", 0.37404977151551183),
        ("get", 0.13040779578930034),
        ("with", 0.5306758223243937),
    ],
    &[
        ("with", 0.5306758223243937),
        ("mask", 0.9252929025156035),
        ("edge_types", 0.5306758223243937),
        ("edge_ids", 0.7131777350098516),
        ("get", 0.13040779578930034),
        ("unknown", 0.6255440881216725),
    ],
    &[
        ("edge_types", 0.5306758223243937),
        ("edge_ids", 0.7131777350098516),
        ("mask", 0.9252929025156035),
        ("get", 0.13040779578930034),
        ("known", 0.6255440881216725),
        ("with", 0.5306758223243937),
    ],
    &[
        ("get", 0.17514290141589564),
        ("node_ids", 0.4257446581149362),
        ("with", 0.7127189189159387),
        ("node_types", 0.7249618438397802),
        ("unknown", 0.840130805785612),
    ],
    &[
        ("node_ids", 0.4257446581149362),
        ("get", 0.17514290141589564),
        ("known", 0.840130805785612),
        ("node_types", 0.7249618438397802),
        ("with", 0.7127189189159387),
    ],
    &[
        ("get", 0.17514290141589564),
        ("node_names", 0.6134881554465302),
        ("unknown", 0.840130805785612),
        ("with", 0.7127189189159387),
        ("node_types", 0.7249618438397802),
    ],
    &[
        ("known", 0.840130805785612),
        ("node_types", 0.7249618438397802),
        ("with", 0.7127189189159387),
        ("get", 0.17514290141589564),
        ("node_names", 0.6134881554465302),
    ],
    &[
        ("node_ids", 0.3170007004851363),
        ("get", 0.13040779578930034),
        ("unknown", 0.6255440881216725),
        ("with", 0.5306758223243937),
        ("mask", 0.9252929025156035),
        ("node_types", 0.5397916519722127),
    ],
    &[
        ("node_types", 0.5397916519722127),
        ("known", 0.6255440881216725),
        ("mask", 0.9252929025156035),
        ("node_ids", 0.3170007004851363),
        ("with", 0.5306758223243937),
        ("get", 0.13040779578930034),
    ],
    &[
        ("known", 1.180120046798488),
        ("get", 0.2460207953237388),
        ("edge_types", 1.0011463431087175),
        ("number", 1.096094708260868),
    ],
    &[
        ("rate", 1.745611099447606),
        ("edge_types", 1.0011463431087175),
        ("get", 0.2460207953237388),
        ("unknown", 1.180120046798488),
    ],
    &[
        ("rate", 1.745611099447606),
        ("get", 0.2460207953237388),
        ("edge_types", 1.0011463431087175),
        ("known", 1.180120046798488),
    ],
    &[
        ("minimum", 1.6908636925621634),
        ("edge_types", 1.0011463431087175),
        ("get", 0.2460207953237388),
        ("number", 1.096094708260868),
    ],
    &[
        ("edge_types", 1.0011463431087175),
        ("number", 1.096094708260868),
        ("get", 0.2460207953237388),
        ("singleton", 1.0456825228820985),
    ],
    &[
        ("singleton", 1.554937854069521),
        ("get", 0.36583479131199337),
        ("edge_type_ids", 2.271155030697057),
    ],
    &[
        ("edge_type_names", 2.443122069340597),
        ("singleton", 1.554937854069521),
        ("get", 0.36583479131199337),
    ],
    &[
        ("nodes", 1.6140974131025445),
        ("number", 1.629901156636506),
        ("get", 0.36583479131199337),
    ],
    &[
        ("connected", 1.5273325015145505),
        ("node", 0.8676442211356591),
        ("get", 0.2460207953237388),
        ("component_ids", 2.3066011683672523),
    ],
    &[
        ("get", 0.2460207953237388),
        ("edges", 1.306171937378964),
        ("directed", 1.270557477400446),
        ("number", 1.096094708260868),
    ],
    &[
        ("number", 1.629901156636506),
        ("get", 0.36583479131199337),
        ("edge_types", 1.4887122164692992),
    ],
    &[
        ("node_types", 1.5142849793295328),
        ("get", 0.36583479131199337),
        ("number", 1.629901156636506),
    ],
    &[
        ("node", 1.2901935470720483),
        ("degrees", 2.1013429119884273),
        ("get", 0.36583479131199337),
    ],
    &[
        ("indegrees", 3.1393256883021006),
        ("node", 1.2901935470720483),
        ("get", 0.36583479131199337),
    ],
    &[
        ("node", 0.8676442211356591),
        ("weighted", 1.0096484530954233),
        ("degrees", 1.4131352914829942),
        ("get", 0.2460207953237388),
    ],
    &[
        ("get", 0.2460207953237388),
        ("not", 1.8095225182222892),
        ("singletons", 2.3066011683672523),
        ("node_ids", 0.5980375941446159),
    ],
    &[
        ("get", 0.2460207953237388),
        ("nodes", 1.0854668247307808),
        ("mapping", 2.1111699077238817),
        ("dense", 1.9824426271900815),
    ],
    &[
        ("number", 1.096094708260868),
        ("parallel", 2.1111699077238817),
        ("get", 0.2460207953237388),
        ("edges", 1.306171937378964),
    ],
    &[
        ("cumulative", 2.3066011683672523),
        ("node", 0.8676442211356591),
        ("get", 0.2460207953237388),
        ("degrees", 1.4131352914829942),
    ],
    &[
        ("number", 0.7803129291521904),
        ("get", 0.17514290141589564),
        ("source", 0.8813199546982742),
        ("nodes", 0.7727469087476133),
        ("unique", 0.916895565662068),
    ],
    &[
        ("counts", 1.5621364120283636),
        ("get", 0.2460207953237388),
        ("edge_type_id", 1.1418301968666458),
        ("hashmap", 1.8862949532055344),
    ],
    &[
        ("counts", 1.5621364120283636),
        ("get", 0.2460207953237388),
        ("hashmap", 1.8862949532055344),
        ("edge_type_names", 1.6429788769312366),
    ],
    &[
        ("node_type_id", 1.5621364120283636),
        ("counts", 1.5621364120283636),
        ("hashmap", 1.8862949532055344),
        ("get", 0.2460207953237388),
    ],
    &[
        ("counts", 1.5621364120283636),
        ("get", 0.2460207953237388),
        ("node_type_names", 1.3892163030605715),
        ("hashmap", 1.8862949532055344),
    ],
    &[
        ("inplace", 2.223719384909289),
        ("directed", 1.889328619489405),
        ("to", 2.271155030697057),
    ],
    &[("directed", 3.009654030428919), ("to", 3.6178941140020378)],
    &[
        ("triangular", 3.1393256883021006),
        ("to", 2.271155030697057),
        ("upper", 3.429933457288565),
    ],
    &[
        ("lower", 3.429933457288565),
        ("to", 2.271155030697057),
        ("triangular", 3.1393256883021006),
    ],
    &[
        ("diagonal", 3.1393256883021006),
        ("to", 2.271155030697057),
        ("main", 3.429933457288565),
    ],
    &[
        ("to", 2.271155030697057),
        ("anti", 3.429933457288565),
        ("diagonal", 3.1393256883021006),
    ],
    &[("bidiagonal", 5.46379965208028), ("to", 3.6178941140020378)],
    &[("to", 3.6178941140020378), ("arrowhead", 5.46379965208028)],
    &[("to", 3.6178941140020378), ("transposed", 5.46379965208028)],
    &[
        ("complementary", 5.46379965208028),
        ("to", 3.6178941140020378),
    ],
    &[("report", 7.26694384944415)],
    &[
        ("overlap", 3.429933457288565),
        ("textual", 3.1393256883021006),
        ("report", 2.690773728932532),
    ],
    &[
        ("from", 0.35292613335563944),
        ("report", 1.2882042089239163),
        ("node", 0.6176783799372239),
        ("get", 0.17514290141589564),
        ("node_id", 0.6586091234986925),
    ],
    &[
        ("from", 0.35292613335563944),
        ("get", 0.17514290141589564),
        ("node_name", 0.916895565662068),
        ("report", 1.2882042089239163),
        ("node", 0.6176783799372239),
    ],
    &[
        ("report", 4.286336381461596),
        ("textual", 5.000868622410895),
    ],
    &[
        ("connected", 1.5273325015145505),
        ("graph", 1.5621364120283636),
        ("random", 1.4659887380438164),
        ("generate", 1.6908636925621634),
    ],
    &[
        ("spanning", 1.8862949532055344),
        ("generate", 1.6908636925621634),
        ("random", 1.4659887380438164),
        ("tree", 2.3066011683672523),
    ],
    &[
        ("circle", 3.429933457288565),
        ("graph", 2.322908709986271),
        ("generate", 2.5143271538957523),
    ],
    &[
        ("chain", 3.429933457288565),
        ("generate", 2.5143271538957523),
        ("graph", 2.322908709986271),
    ],
    &[
        ("generate", 2.5143271538957523),
        ("complete", 3.429933457288565),
        ("graph", 2.322908709986271),
    ],
    &[
        ("generate", 2.5143271538957523),
        ("barbell", 3.429933457288565),
        ("graph", 2.322908709986271),
    ],
    &[("replace", 9.263184627320559)],
    &[
        ("from_ids", 5.46379965208028),
        ("filter", 5.000868622410895),
    ],
    &[
        ("from_names", 5.46379965208028),
        ("filter", 5.000868622410895),
    ],
    &[
        ("node_types", 1.5142849793295328),
        ("unknown", 1.754847473261407),
        ("drop", 2.5143271538957523),
    ],
    &[
        ("unknown", 1.754847473261407),
        ("drop", 2.5143271538957523),
        ("edge_types", 1.4887122164692992),
    ],
    &[
        ("drop", 2.5143271538957523),
        ("nodes", 1.6140974131025445),
        ("singleton", 1.554937854069521),
    ],
    &[
        ("singleton", 0.7444243515125029),
        ("selfloops", 0.9889870231990723),
        ("with", 0.7127189189159387),
        ("drop", 1.2037306546564002),
        ("nodes", 0.7727469087476133),
    ],
    &[
        ("nodes", 1.6140974131025445),
        ("drop", 2.5143271538957523),
        ("disconnected", 2.804934922882217),
    ],
    &[
        ("drop", 4.005261326419907),
        ("selfloops", 3.290729085470608),
    ],
    &[
        ("drop", 2.5143271538957523),
        ("edges", 1.9422875919892115),
        ("parallel", 3.1393256883021006),
    ],
    &[
        ("spanning", 1.8862949532055344),
        ("arborescence", 1.9824426271900815),
        ("kruskal", 2.1111699077238817),
        ("random", 1.4659887380438164),
    ],
    &[
        ("kruskal", 3.1393256883021006),
        ("arborescence", 2.9479072443926193),
        ("spanning", 2.804934922882217),
    ],
    &[
        ("spanning", 4.468192356089292),
        ("arborescence", 4.695943748427725),
    ],
    &[
        ("connected", 3.6178941140020378),
        ("components", 4.468192356089292),
    ],
    &[("enable", 9.263184627320559)],
    &[("all", 3.8918333777254275), ("disable", 5.46379965208028)],
    &[("compatible", 5.46379965208028), ("is", 3.407826030331003)],
    &[
        ("same", 2.3066011683672523),
        ("matrix", 1.3892163030605715),
        ("adjacency", 1.6908636925621634),
        ("has", 1.0272414011261481),
    ],
    &[
        ("cover", 1.5029478403812442),
        ("vertex", 1.5029478403812442),
        ("iter", 0.41550438353555247),
        ("approximated", 1.5029478403812442),
        ("par", 0.6635612457370292),
    ],
    &[
        ("cover", 2.1111699077238817),
        ("vertex", 2.1111699077238817),
        ("set", 1.8095225182222892),
        ("approximated", 2.1111699077238817),
    ],
    &[
        ("nodes", 1.6140974131025445),
        ("get", 0.36583479131199337),
        ("random", 2.179936388475869),
    ],
    &[
        ("breadth", 0.999864937158341),
        ("get", 0.13040779578930034),
        ("random", 0.7770739856712646),
        ("first", 0.999864937158341),
        ("nodes", 0.5753714267498481),
        ("search", 0.999864937158341),
    ],
    &[
        ("nodes", 0.5753714267498481),
        ("walk", 1.0508297599412841),
        ("random", 1.4375388116084353),
        ("get", 0.13040779578930034),
        ("uniform", 1.2226558886454175),
    ],
    &[
        ("sampling", 2.3066011683672523),
        ("node", 0.8676442211356591),
        ("get", 0.2460207953237388),
        ("methods", 1.9824426271900815),
    ],
    &[
        ("nodes", 1.6140974131025445),
        ("get", 0.36583479131199337),
        ("subsampled", 2.5957369637715932),
    ],
    &[("node2vec", 9.263184627320559)],
    &[
        ("matrix", 3.290729085470608),
        ("cooccurence", 5.46379965208028),
    ],
    &[
        ("prediction", 0.9252929025156035),
        ("get", 0.13040779578930034),
        ("mini", 1.1190639955483745),
        ("batch", 1.1190639955483745),
        ("node", 0.4599105951947464),
        ("label", 0.8280388084542077),
    ],
    &[
        ("mini", 1.5029478403812442),
        ("prediction", 1.242705488772741),
        ("batch", 1.5029478403812442),
        ("edge", 0.5023638491906379),
        ("get", 0.17514290141589564),
    ],
    &[
        ("degrees", 2.1013429119884273),
        ("link", 3.429933457288565),
        ("prediction", 2.5957369637715932),
    ],
    &[
        ("iter", 0.309376003021643),
        ("prediction", 0.9252929025156035),
        ("edge", 0.37404977151551183),
        ("unchecked", 0.42267521073949105),
        ("metrics", 1.0508297599412841),
        ("par", 0.49407403170902714),
    ],
    &[
        ("iter", 0.41550438353555247),
        ("prediction", 1.242705488772741),
        ("metrics", 1.4113065245550764),
        ("edge", 0.5023638491906379),
        ("par", 0.6635612457370292),
    ],
    &[
        ("bm25", 1.1190639955483745),
        ("feature", 1.2226558886454175),
        ("node", 0.4599105951947464),
        ("get", 0.13040779578930034),
        ("propagation", 1.1190639955483745),
        ("okapi", 1.1190639955483745),
    ],
    &[
        ("bm25", 1.1190639955483745),
        ("node", 0.4599105951947464),
        ("get", 0.13040779578930034),
        ("okapi", 1.1190639955483745),
        ("label", 0.8280388084542077),
        ("propagation", 1.1190639955483745),
    ],
    &[
        ("default", 3.429933457288565),
        ("has", 1.5275157659477847),
        ("graph_name", 3.429933457288565),
    ],
    &[("nodes", 2.5712174868561597), ("has", 2.4332950520650543)],
    &[("edges", 3.0940163713087676), ("has", 2.4332950520650543)],
    &[
        ("has", 1.5275157659477847),
        ("trap", 2.690773728932532),
        ("nodes", 1.6140974131025445),
    ],
    &[("is", 3.407826030331003), ("directed", 3.009654030428919)],
    &[
        ("weights", 2.1392835137358808),
        ("edge", 1.0493269921053563),
        ("has", 1.5275157659477847),
    ],
    &[
        ("weights", 1.0241790295646243),
        ("probabilities", 1.6420759086870347),
        ("representing", 1.6420759086870347),
        ("has", 0.7312960646721532),
        ("edge", 0.5023638491906379),
    ],
    &[
        ("weighted", 1.0096484530954233),
        ("singleton", 1.0456825228820985),
        ("has", 1.0272414011261481),
        ("nodes", 1.0854668247307808),
    ],
    &[
        ("constant", 2.3066011683672523),
        ("weights", 1.4386500244680516),
        ("edge", 0.7056635051756572),
        ("has", 1.0272414011261481),
    ],
    &[
        ("negative", 2.3066011683672523),
        ("has", 1.0272414011261481),
        ("edge", 0.7056635051756572),
        ("weights", 1.4386500244680516),
    ],
    &[
        ("has", 2.4332950520650543),
        ("edge_types", 2.3714819519626307),
    ],
    &[
        ("has", 2.4332950520650543),
        ("selfloops", 3.290729085470608),
    ],
    &[
        ("has", 1.5275157659477847),
        ("disconnected", 2.804934922882217),
        ("nodes", 1.6140974131025445),
    ],
    &[
        ("has", 1.5275157659477847),
        ("nodes", 1.6140974131025445),
        ("singleton", 1.554937854069521),
    ],
    &[
        ("has", 0.7312960646721532),
        ("nodes", 0.7727469087476133),
        ("singleton", 0.7444243515125029),
        ("selfloops", 0.9889870231990723),
        ("with", 0.7127189189159387),
    ],
    &[("connected", 3.6178941140020378), ("is", 3.407826030331003)],
    &[
        ("node_types", 2.4122187343400157),
        ("has", 2.4332950520650543),
    ],
    &[
        ("node_types", 1.5142849793295328),
        ("multilabel", 3.1393256883021006),
        ("has", 1.5275157659477847),
    ],
    &[
        ("node_types", 1.5142849793295328),
        ("has", 1.5275157659477847),
        ("unknown", 1.754847473261407),
    ],
    &[
        ("node_types", 1.5142849793295328),
        ("has", 1.5275157659477847),
        ("known", 1.754847473261407),
    ],
    &[
        ("unknown", 1.754847473261407),
        ("has", 1.5275157659477847),
        ("edge_types", 1.4887122164692992),
    ],
    &[
        ("has", 1.5275157659477847),
        ("known", 1.754847473261407),
        ("edge_types", 1.4887122164692992),
    ],
    &[
        ("homogeneous", 3.1393256883021006),
        ("has", 1.5275157659477847),
        ("node_types", 1.5142849793295328),
    ],
    &[
        ("edge_types", 1.4887122164692992),
        ("has", 1.5275157659477847),
        ("homogeneous", 3.1393256883021006),
    ],
    &[
        ("node_types", 1.5142849793295328),
        ("has", 1.5275157659477847),
        ("singleton", 1.554937854069521),
    ],
    &[
        ("node", 1.2901935470720483),
        ("oddities", 2.9479072443926193),
        ("has", 1.5275157659477847),
    ],
    &[
        ("oddities", 2.9479072443926193),
        ("has", 1.5275157659477847),
        ("node_types", 1.5142849793295328),
    ],
    &[
        ("has", 1.5275157659477847),
        ("edge_types", 1.4887122164692992),
        ("singleton", 1.554937854069521),
    ],
    &[
        ("edge_types", 1.4887122164692992),
        ("has", 1.5275157659477847),
        ("oddities", 2.9479072443926193),
    ],
    &[("multigraph", 4.695943748427725), ("is", 3.407826030331003)],
    &[
        ("outbound", 0.6120737515917357),
        ("has", 0.3333240631901977),
        ("nodes", 0.3522173192014819),
        ("sorted", 0.6432721956222057),
        ("decreasing", 0.6850422217756604),
        ("by", 0.5486593080143868),
        ("degree", 0.4179201423770398),
        ("node", 0.28153722861576613),
    ],
    &[
        ("order", 1.1190639955483745),
        ("by", 0.896273044061298),
        ("lexicographic", 1.1190639955483745),
        ("has", 0.5445079822951353),
        ("nodes", 0.5753714267498481),
        ("sorted", 1.0508297599412841),
    ],
    &[
        ("identity", 3.1393256883021006),
        ("contains", 3.1393256883021006),
        ("matrix", 2.0657751945261844),
    ],
    &[
        ("has", 0.3333240631901977),
        ("node", 0.28153722861576613),
        ("increasing", 0.6850422217756604),
        ("nodes", 0.3522173192014819),
        ("degree", 0.4179201423770398),
        ("sorted", 0.6432721956222057),
        ("outbound", 0.6120737515917357),
        ("by", 0.5486593080143868),
    ],
    &[
        ("closure", 3.429933457288565),
        ("get", 0.36583479131199337),
        ("transitive", 3.429933457288565),
    ],
    &[
        ("get", 0.2460207953237388),
        ("shortest", 1.3892163030605715),
        ("all", 1.6429788769312366),
        ("paths", 2.1111699077238817),
    ],
    &[
        ("shortest", 0.9889870231990723),
        ("weighted", 0.718771595110523),
        ("paths", 1.5029478403812442),
        ("get", 0.17514290141589564),
        ("all", 1.1696413187027879),
    ],
    &[
        ("edge_id", 0.6255440881216725),
        ("unchecked", 0.42267521073949105),
        ("edge", 0.37404977151551183),
        ("weight", 0.6923601846392166),
        ("from", 0.26278152728588133),
        ("get", 0.13040779578930034),
    ],
    &[
        ("get", 0.13040779578930034),
        ("from", 0.26278152728588133),
        ("unchecked", 0.42267521073949105),
        ("edge", 0.37404977151551183),
        ("weight", 0.6923601846392166),
        ("node_ids", 0.3170007004851363),
    ],
    &[
        ("unchecked", 0.5676697648129673),
        ("node_name", 0.916895565662068),
        ("node_id", 0.6586091234986925),
        ("from", 0.35292613335563944),
        ("get", 0.17514290141589564),
    ],
    &[
        ("get", 0.17514290141589564),
        ("edge_type_id", 0.8128721531053885),
        ("from", 0.35292613335563944),
        ("edge_type_name", 0.9434883030478971),
        ("unchecked", 0.5676697648129673),
    ],
    &[
        ("get", 0.17514290141589564),
        ("edge_type_id", 0.8128721531053885),
        ("from", 0.35292613335563944),
        ("unchecked", 0.5676697648129673),
        ("edge_type_name", 0.9434883030478971),
    ],
    &[
        ("from", 0.26278152728588133),
        ("edge", 0.37404977151551183),
        ("count", 0.9252929025156035),
        ("edge_type_id", 0.6052478569671313),
        ("unchecked", 0.42267521073949105),
        ("get", 0.13040779578930034),
    ],
    &[
        ("edge_type_id", 0.46689335272312055),
        ("get", 0.10059768456248329),
        ("from", 0.20271190867656422),
        ("edge_id", 0.48255003833101656),
        ("node_ids", 0.24453704075340602),
        ("unchecked", 0.32605525816149983),
        ("and", 0.3961864738450093),
    ],
    &[
        ("edge_ids", 0.7131777350098516),
        ("unchecked", 0.42267521073949105),
        ("minmax", 0.999864937158341),
        ("get", 0.13040779578930034),
        ("node_ids", 0.3170007004851363),
        ("from", 0.26278152728588133),
    ],
    &[
        ("from", 0.35292613335563944),
        ("edge_id", 0.840130805785612),
        ("node_ids", 0.4257446581149362),
        ("get", 0.17514290141589564),
        ("unchecked", 0.5676697648129673),
    ],
    &[
        ("unchecked", 0.5676697648129673),
        ("get", 0.17514290141589564),
        ("edge_id", 0.840130805785612),
        ("node_names", 0.6134881554465302),
        ("from", 0.35292613335563944),
    ],
    &[
        ("node_id", 0.49038678352277515),
        ("source", 0.6562126797500742),
        ("from", 0.26278152728588133),
        ("edge_id", 0.6255440881216725),
        ("get", 0.13040779578930034),
        ("unchecked", 0.42267521073949105),
    ],
    &[
        ("edge_id", 0.6255440881216725),
        ("destination", 0.7770739856712646),
        ("unchecked", 0.42267521073949105),
        ("get", 0.13040779578930034),
        ("node_id", 0.49038678352277515),
        ("from", 0.26278152728588133),
    ],
    &[
        ("from", 0.35292613335563944),
        ("source", 0.8813199546982742),
        ("get", 0.17514290141589564),
        ("edge_id", 0.840130805785612),
        ("node_id", 0.6586091234986925),
    ],
    &[
        ("edge_id", 0.840130805785612),
        ("from", 0.35292613335563944),
        ("get", 0.17514290141589564),
        ("node_id", 0.6586091234986925),
        ("destination", 1.0436415372373469),
    ],
    &[
        ("node_name", 0.6827015466818229),
        ("from", 0.26278152728588133),
        ("source", 0.6562126797500742),
        ("get", 0.13040779578930034),
        ("unchecked", 0.42267521073949105),
        ("edge_id", 0.6255440881216725),
    ],
    &[
        ("destination", 0.7770739856712646),
        ("unchecked", 0.42267521073949105),
        ("node_name", 0.6827015466818229),
        ("get", 0.13040779578930034),
        ("from", 0.26278152728588133),
        ("edge_id", 0.6255440881216725),
    ],
    &[
        ("source", 0.8813199546982742),
        ("edge_id", 0.840130805785612),
        ("from", 0.35292613335563944),
        ("node_name", 0.916895565662068),
        ("get", 0.17514290141589564),
    ],
    &[
        ("from", 0.35292613335563944),
        ("node_name", 0.916895565662068),
        ("edge_id", 0.840130805785612),
        ("destination", 1.0436415372373469),
        ("get", 0.17514290141589564),
    ],
    &[
        ("from", 0.4957504261763134),
        ("get", 0.2460207953237388),
        ("node_names", 0.8617582711288273),
        ("edge_id", 1.180120046798488),
    ],
    &[
        ("get", 0.2460207953237388),
        ("node_ids", 0.5980375941446159),
        ("edge_id", 1.180120046798488),
        ("from", 0.4957504261763134),
    ],
    &[
        ("unchecked", 0.5676697648129673),
        ("from", 0.35292613335563944),
        ("edge_id", 0.840130805785612),
        ("get", 0.17514290141589564),
        ("node_ids", 0.4257446581149362),
    ],
    &[
        ("from", 0.4957504261763134),
        ("node_ids", 0.5980375941446159),
        ("edge_id", 1.180120046798488),
        ("get", 0.2460207953237388),
    ],
    &[
        ("unchecked", 0.5676697648129673),
        ("source", 0.8813199546982742),
        ("node_id", 0.6586091234986925),
        ("unique", 0.916895565662068),
        ("get", 0.17514290141589564),
    ],
    &[
        ("edge_type_id", 0.46689335272312055),
        ("get", 0.10059768456248329),
        ("and", 0.3961864738450093),
        ("node_ids", 0.24453704075340602),
        ("edge_id", 0.48255003833101656),
        ("from", 0.20271190867656422),
        ("unchecked", 0.32605525816149983),
    ],
    &[
        ("edge_type_id", 0.6052478569671313),
        ("from", 0.26278152728588133),
        ("get", 0.13040779578930034),
        ("and", 0.5135884091206121),
        ("node_ids", 0.3170007004851363),
        ("edge_id", 0.6255440881216725),
    ],
    &[
        ("edge_type_id", 0.248993645996318),
        ("edge", 0.15388078672502406),
        ("unchecked", 0.17388486482489654),
        ("edge_id", 0.25734333701460066),
        ("get", 0.05364862042243388),
        ("node_ids", 0.1304113005747781),
        ("from", 0.10810600950702057),
        ("and", 0.40892529967651725),
        ("weight", 0.2848308915621068),
    ],
    &[
        ("weight", 0.3441636118149813),
        ("edge_id", 0.3109501636487797),
        ("edge", 0.18593547581080144),
        ("node_ids", 0.157577094187897),
        ("from", 0.1306254194788706),
        ("get", 0.06482408868017674),
        ("edge_type_id", 0.3008611603014561),
        ("and", 0.4908063147630377),
    ],
    &[
        ("top", 1.4113065245550764),
        ("central", 1.2882042089239163),
        ("get", 0.17514290141589564),
        ("node_ids", 0.4257446581149362),
        ("k", 1.2037306546564002),
    ],
    &[
        ("central", 0.9591703120947649),
        ("get", 0.13040779578930034),
        ("top", 1.0508297599412841),
        ("weighted", 0.5351825203109013),
        ("node_ids", 0.3170007004851363),
        ("k", 0.896273044061298),
    ],
    &[
        ("node_id", 0.49038678352277515),
        ("from", 0.26278152728588133),
        ("get", 0.13040779578930034),
        ("degree", 0.6827015466818229),
        ("unchecked", 0.42267521073949105),
        ("node", 0.4599105951947464),
    ],
    &[
        ("from", 0.20271190867656422),
        ("weighted", 0.41284435516858997),
        ("get", 0.10059768456248329),
        ("node", 0.354778950923278),
        ("degree", 0.526641788765303),
        ("node_id", 0.3782885422136893),
        ("unchecked", 0.32605525816149983),
    ],
    &[
        ("node_id", 0.6586091234986925),
        ("node", 0.6176783799372239),
        ("from", 0.35292613335563944),
        ("degree", 0.916895565662068),
        ("get", 0.17514290141589564),
    ],
    &[
        ("node", 0.354778950923278),
        ("degree", 0.526641788765303),
        ("node_id", 0.3782885422136893),
        ("from", 0.20271190867656422),
        ("comulative", 0.7713043299639504),
        ("unchecked", 0.32605525816149983),
        ("get", 0.10059768456248329),
    ],
    &[
        ("get", 0.13040779578930034),
        ("comulative", 0.999864937158341),
        ("from", 0.26278152728588133),
        ("node", 0.4599105951947464),
        ("node_id", 0.49038678352277515),
        ("degree", 0.6827015466818229),
    ],
    &[
        ("degree", 0.6827015466818229),
        ("from", 0.26278152728588133),
        ("node_id", 0.49038678352277515),
        ("node", 0.4599105951947464),
        ("weighted", 0.5351825203109013),
        ("get", 0.13040779578930034),
    ],
    &[
        ("from", 0.35292613335563944),
        ("degree", 0.916895565662068),
        ("get", 0.17514290141589564),
        ("node", 0.6176783799372239),
        ("node_name", 0.916895565662068),
    ],
    &[
        ("k", 1.2037306546564002),
        ("get", 0.17514290141589564),
        ("central", 1.2882042089239163),
        ("node_names", 0.6134881554465302),
        ("top", 1.4113065245550764),
    ],
    &[
        ("get", 0.17514290141589564),
        ("node_type_id", 1.1120893388302326),
        ("from", 0.35292613335563944),
        ("node_id", 0.6586091234986925),
        ("unchecked", 0.5676697648129673),
    ],
    &[
        ("node_type_ids", 1.3253048842858886),
        ("node_id", 0.9251390667889959),
        ("from", 0.4957504261763134),
        ("get", 0.2460207953237388),
    ],
    &[
        ("from", 0.35292613335563944),
        ("get", 0.17514290141589564),
        ("edge_id", 0.840130805785612),
        ("edge_type_id", 0.8128721531053885),
        ("unchecked", 0.5676697648129673),
    ],
    &[
        ("edge_type_id", 1.1418301968666458),
        ("get", 0.2460207953237388),
        ("from", 0.4957504261763134),
        ("edge_id", 1.180120046798488),
    ],
    &[
        ("node_type_names", 0.9889870231990723),
        ("from", 0.35292613335563944),
        ("unchecked", 0.5676697648129673),
        ("node_id", 0.6586091234986925),
        ("get", 0.17514290141589564),
    ],
    &[
        ("get", 0.2460207953237388),
        ("node_id", 0.9251390667889959),
        ("node_type_names", 1.3892163030605715),
        ("from", 0.4957504261763134),
    ],
    &[
        ("node_type_names", 1.3892163030605715),
        ("node_name", 1.2879504362973773),
        ("get", 0.2460207953237388),
        ("from", 0.4957504261763134),
    ],
    &[
        ("get", 0.2460207953237388),
        ("edge_id", 1.180120046798488),
        ("edge_type_name", 1.3253048842858886),
        ("from", 0.4957504261763134),
    ],
    &[
        ("get", 0.2460207953237388),
        ("edge_type_id", 1.1418301968666458),
        ("edge_type_name", 1.3253048842858886),
        ("from", 0.4957504261763134),
    ],
    &[
        ("from", 0.35292613335563944),
        ("get", 0.17514290141589564),
        ("edge", 0.5023638491906379),
        ("edge_id", 0.840130805785612),
        ("weight", 0.9298675039219312),
    ],
    &[
        ("from", 0.35292613335563944),
        ("node_ids", 0.4257446581149362),
        ("get", 0.17514290141589564),
        ("edge", 0.5023638491906379),
        ("weight", 0.9298675039219312),
    ],
    &[
        ("node_ids", 0.24453704075340602),
        ("and", 0.3961864738450093),
        ("from", 0.20271190867656422),
        ("edge", 0.28854518012391517),
        ("get", 0.10059768456248329),
        ("edge_type_id", 0.46689335272312055),
        ("weight", 0.5340925443636185),
    ],
    &[
        ("and", 0.3961864738450093),
        ("node_names", 0.35237219119648877),
        ("from", 0.20271190867656422),
        ("weight", 0.5340925443636185),
        ("edge", 0.28854518012391517),
        ("get", 0.10059768456248329),
        ("edge_type_name", 0.541915989349888),
    ],
    &[
        ("edge", 0.5023638491906379),
        ("from", 0.35292613335563944),
        ("node_names", 0.6134881554465302),
        ("get", 0.17514290141589564),
        ("weight", 0.9298675039219312),
    ],
    &[
        ("from", 0.35292613335563944),
        ("node_name", 0.916895565662068),
        ("node_id", 0.6586091234986925),
        ("unchecked", 0.5676697648129673),
        ("get", 0.17514290141589564),
    ],
    &[
        ("node_id", 0.9251390667889959),
        ("from", 0.4957504261763134),
        ("get", 0.2460207953237388),
        ("node_name", 1.2879504362973773),
    ],
    &[
        ("get", 0.2460207953237388),
        ("node_name", 1.2879504362973773),
        ("node_id", 0.9251390667889959),
        ("from", 0.4957504261763134),
    ],
    &[
        ("from", 0.4957504261763134),
        ("get", 0.2460207953237388),
        ("node_names", 0.8617582711288273),
        ("node_ids", 0.5980375941446159),
    ],
    &[
        ("edge", 0.6919689423939748),
        ("get", 0.13040779578930034),
        ("node_ids", 0.3170007004851363),
        ("node_names", 0.45679064037341954),
        ("from", 0.26278152728588133),
    ],
    &[
        ("from", 0.26278152728588133),
        ("node_ids", 0.3170007004851363),
        ("node_names", 0.45679064037341954),
        ("edge", 0.6919689423939748),
        ("get", 0.13040779578930034),
    ],
    &[
        ("node_type_ids", 1.3253048842858886),
        ("from", 0.4957504261763134),
        ("node_name", 1.2879504362973773),
        ("get", 0.2460207953237388),
    ],
    &[
        ("from", 0.4957504261763134),
        ("get", 0.2460207953237388),
        ("node_type_name", 1.6429788769312366),
        ("node_name", 1.2879504362973773),
    ],
    &[
        ("from", 0.35292613335563944),
        ("get", 0.17514290141589564),
        ("count", 1.242705488772741),
        ("edge", 0.5023638491906379),
        ("edge_type_id", 0.8128721531053885),
    ],
    &[
        ("edge_type_id", 1.1418301968666458),
        ("get", 0.2460207953237388),
        ("from", 0.4957504261763134),
        ("edge_type_name", 1.3253048842858886),
    ],
    &[
        ("count", 1.242705488772741),
        ("edge_type_name", 0.9434883030478971),
        ("from", 0.35292613335563944),
        ("get", 0.17514290141589564),
        ("edge", 0.5023638491906379),
    ],
    &[
        ("node_type_name", 1.6429788769312366),
        ("from", 0.4957504261763134),
        ("node_type_id", 1.5621364120283636),
        ("get", 0.2460207953237388),
    ],
    &[
        ("node", 0.6176783799372239),
        ("get", 0.17514290141589564),
        ("node_type_id", 1.1120893388302326),
        ("count", 1.242705488772741),
        ("from", 0.35292613335563944),
    ],
    &[
        ("node_type_name", 1.1696413187027879),
        ("node", 0.6176783799372239),
        ("count", 1.242705488772741),
        ("from", 0.35292613335563944),
        ("get", 0.17514290141589564),
    ],
    &[
        ("neighbour", 1.1120893388302326),
        ("from", 0.35292613335563944),
        ("node_id", 0.6586091234986925),
        ("get", 0.17514290141589564),
        ("node_ids", 0.4257446581149362),
    ],
    &[
        ("node_name", 0.916895565662068),
        ("neighbour", 1.1120893388302326),
        ("node_ids", 0.4257446581149362),
        ("from", 0.35292613335563944),
        ("get", 0.17514290141589564),
    ],
    &[
        ("node_names", 0.6134881554465302),
        ("get", 0.17514290141589564),
        ("from", 0.35292613335563944),
        ("neighbour", 1.1120893388302326),
        ("node_name", 0.916895565662068),
    ],
    &[
        ("node_ids", 0.4257446581149362),
        ("from", 0.35292613335563944),
        ("minmax", 1.3428587229621909),
        ("edge_ids", 0.9578263092235365),
        ("get", 0.17514290141589564),
    ],
    &[
        ("get", 0.13040779578930034),
        ("from", 0.26278152728588133),
        ("edge_id", 0.6255440881216725),
        ("and", 0.5135884091206121),
        ("node_ids", 0.3170007004851363),
        ("edge_type_id", 0.6052478569671313),
    ],
    &[
        ("edge_id", 1.180120046798488),
        ("node_names", 0.8617582711288273),
        ("get", 0.2460207953237388),
        ("from", 0.4957504261763134),
    ],
    &[
        ("and", 0.5135884091206121),
        ("node_names", 0.45679064037341954),
        ("edge_id", 0.6255440881216725),
        ("get", 0.13040779578930034),
        ("from", 0.26278152728588133),
        ("edge_type_name", 0.7025019510285271),
    ],
    &[
        ("edge_type_ids", 1.5273325015145505),
        ("from", 0.4957504261763134),
        ("edge_type_names", 1.6429788769312366),
        ("get", 0.2460207953237388),
    ],
    &[
        ("get", 0.2460207953237388),
        ("node_type_ids", 1.3253048842858886),
        ("from", 0.4957504261763134),
        ("node_type_names", 1.3892163030605715),
    ],
    &[
        ("node_type_names", 0.9889870231990723),
        ("get", 0.17514290141589564),
        ("from", 0.35292613335563944),
        ("node_type_ids", 0.9434883030478971),
        ("multiple", 1.6420759086870347),
    ],
    &[
        ("node_id", 0.3782885422136893),
        ("get", 0.10059768456248329),
        ("edge_ids", 0.5501513800557143),
        ("unchecked", 0.32605525816149983),
        ("source", 0.5062080511663406),
        ("from", 0.20271190867656422),
        ("minmax", 0.7713043299639504),
    ],
    &[
        ("minmax", 0.999864937158341),
        ("edge_ids", 0.7131777350098516),
        ("from", 0.26278152728588133),
        ("node_id", 0.49038678352277515),
        ("source", 0.6562126797500742),
        ("get", 0.13040779578930034),
    ],
    &[
        ("from", 0.4957504261763134),
        ("get", 0.2460207953237388),
        ("node_type_name", 1.6429788769312366),
        ("node_type_id", 1.5621364120283636),
    ],
    &[
        ("node_type_names", 0.9889870231990723),
        ("node_type_ids", 0.9434883030478971),
        ("get", 0.17514290141589564),
        ("unchecked", 0.5676697648129673),
        ("from", 0.35292613335563944),
    ],
    &[
        ("matrix", 0.7363793606076885),
        ("binary", 1.1190639955483745),
        ("iter", 0.309376003021643),
        ("adjacency", 0.896273044061298),
        ("subsampled", 0.9252929025156035),
        ("par", 0.49407403170902714),
    ],
    &[
        ("weighted", 0.5351825203109013),
        ("subsampled", 0.9252929025156035),
        ("iter", 0.309376003021643),
        ("par", 0.49407403170902714),
        ("adjacency", 0.896273044061298),
        ("matrix", 0.7363793606076885),
    ],
    &[
        ("par", 0.38113291687472167),
        ("symmetric", 0.7137788271919129),
        ("subsampled", 0.7137788271919129),
        ("iter", 0.2386552841783915),
        ("matrix", 0.5680493116870342),
        ("adjacency", 0.6913926611719697),
        ("laplacian", 0.6544128761730416),
    ],
    &[
        ("undirected", 0.38065736335913614),
        ("laplacian", 0.3489975753025828),
        ("symmetric", 0.38065736335913614),
        ("matrix", 0.30293999346469896),
        ("par", 0.20325771191290964),
        ("with", 0.21831536670739637),
        ("adjacency", 0.3687188487825749),
        ("subsampled", 0.38065736335913614),
        ("iter", 0.12727456708749674),
        ("selfloops", 0.30293999346469896),
    ],
    &[
        ("weighting", 1.5029478403812442),
        ("methods", 1.4113065245550764),
        ("edge", 0.5023638491906379),
        ("sparse", 1.6420759086870347),
        ("get", 0.17514290141589564),
    ],
    &[
        ("edge", 0.7056635051756572),
        ("get", 0.2460207953237388),
        ("weighting", 2.1111699077238817),
        ("methods", 1.9824426271900815),
    ],
    &[
        ("iter", 0.309376003021643),
        ("matrix", 0.7363793606076885),
        ("metric", 1.2226558886454175),
        ("par", 0.49407403170902714),
        ("edge", 0.37404977151551183),
        ("subsampled", 0.9252929025156035),
    ],
    &[("add", 5.46379965208028), ("selfloops", 3.290729085470608)],
    &[
        ("degree", 1.9151920814783965),
        ("centrality", 1.9422875919892115),
        ("iter", 0.8678967757819245),
    ],
    &[
        ("centrality", 0.9298675039219312),
        ("degree", 0.916895565662068),
        ("weighted", 0.718771595110523),
        ("iter", 0.41550438353555247),
        ("par", 0.6635612457370292),
    ],
    &[
        ("centrality", 1.9422875919892115),
        ("get", 0.36583479131199337),
        ("degree", 1.9151920814783965),
    ],
    &[
        ("degree", 1.2879504362973773),
        ("weighted", 1.0096484530954233),
        ("centrality", 1.306171937378964),
        ("get", 0.2460207953237388),
    ],
    &[
        ("closeness", 0.9252929025156035),
        ("get", 0.13040779578930034),
        ("centrality", 0.6923601846392166),
        ("node_id", 0.49038678352277515),
        ("from", 0.26278152728588133),
        ("unchecked", 0.42267521073949105),
    ],
    &[
        ("centrality", 0.5340925443636185),
        ("node_id", 0.3782885422136893),
        ("weighted", 0.41284435516858997),
        ("get", 0.10059768456248329),
        ("unchecked", 0.32605525816149983),
        ("closeness", 0.7137788271919129),
        ("from", 0.20271190867656422),
    ],
    &[
        ("iter", 0.5836532230054678),
        ("centrality", 1.306171937378964),
        ("par", 0.9320952439549945),
        ("closeness", 1.745611099447606),
    ],
    &[
        ("par", 0.6635612457370292),
        ("iter", 0.41550438353555247),
        ("centrality", 0.9298675039219312),
        ("weighted", 0.718771595110523),
        ("closeness", 1.242705488772741),
    ],
    &[
        ("centrality", 1.9422875919892115),
        ("get", 0.36583479131199337),
        ("closeness", 2.5957369637715932),
    ],
    &[
        ("weighted", 1.0096484530954233),
        ("closeness", 1.745611099447606),
        ("get", 0.2460207953237388),
        ("centrality", 1.306171937378964),
    ],
    &[
        ("centrality", 0.6923601846392166),
        ("harmonic", 0.9252929025156035),
        ("from", 0.26278152728588133),
        ("get", 0.13040779578930034),
        ("node_id", 0.49038678352277515),
        ("unchecked", 0.42267521073949105),
    ],
    &[
        ("get", 0.10059768456248329),
        ("harmonic", 0.7137788271919129),
        ("from", 0.20271190867656422),
        ("centrality", 0.5340925443636185),
        ("node_id", 0.3782885422136893),
        ("unchecked", 0.32605525816149983),
        ("weighted", 0.41284435516858997),
    ],
    &[
        ("par", 0.9320952439549945),
        ("centrality", 1.306171937378964),
        ("harmonic", 1.745611099447606),
        ("iter", 0.5836532230054678),
    ],
    &[
        ("iter", 0.41550438353555247),
        ("centrality", 0.9298675039219312),
        ("par", 0.6635612457370292),
        ("weighted", 0.718771595110523),
        ("harmonic", 1.242705488772741),
    ],
    &[
        ("harmonic", 2.5957369637715932),
        ("get", 0.36583479131199337),
        ("centrality", 1.9422875919892115),
    ],
    &[
        ("weighted", 1.0096484530954233),
        ("centrality", 1.306171937378964),
        ("harmonic", 1.745611099447606),
        ("get", 0.2460207953237388),
    ],
    &[
        ("centrality", 1.9422875919892115),
        ("stress", 3.429933457288565),
        ("get", 0.36583479131199337),
    ],
    &[
        ("centrality", 1.9422875919892115),
        ("betweenness", 3.429933457288565),
        ("get", 0.36583479131199337),
    ],
    &[
        ("get", 0.36583479131199337),
        ("centrality", 1.9422875919892115),
        ("eigenvector", 3.1393256883021006),
    ],
    &[
        ("centrality", 1.306171937378964),
        ("eigenvector", 2.1111699077238817),
        ("weighted", 1.0096484530954233),
        ("get", 0.2460207953237388),
    ],
    &[("dot", 5.46379965208028), ("to", 3.6178941140020378)],
    &[
        ("get", 0.17514290141589564),
        ("undirected", 1.242705488772741),
        ("detection", 1.6420759086870347),
        ("louvain", 1.6420759086870347),
        ("community", 1.4113065245550764),
    ],
    &[
        ("directed", 0.5195298233299449),
        ("from", 0.20271190867656422),
        ("get", 0.10059768456248329),
        ("community", 0.8106190284071704),
        ("node", 0.354778950923278),
        ("memberships", 0.8632554990139948),
        ("modularity", 0.8632554990139948),
    ],
    &[
        ("node", 0.354778950923278),
        ("community", 0.8106190284071704),
        ("undirected", 0.7137788271919129),
        ("modularity", 0.8632554990139948),
        ("from", 0.20271190867656422),
        ("memberships", 0.8632554990139948),
        ("get", 0.10059768456248329),
    ],
    &[
        ("get", 0.17514290141589564),
        ("minimum", 1.2037306546564002),
        ("preferential", 1.1120893388302326),
        ("attachment", 1.1120893388302326),
        ("unchecked", 0.5676697648129673),
    ],
    &[
        ("unchecked", 0.5676697648129673),
        ("get", 0.17514290141589564),
        ("maximum", 1.1696413187027879),
        ("preferential", 1.1120893388302326),
        ("attachment", 1.1120893388302326),
    ],
    &[
        ("preferential", 0.8280388084542077),
        ("weighted", 0.5351825203109013),
        ("minimum", 0.896273044061298),
        ("get", 0.13040779578930034),
        ("unchecked", 0.42267521073949105),
        ("attachment", 0.8280388084542077),
    ],
    &[
        ("unchecked", 0.42267521073949105),
        ("weighted", 0.5351825203109013),
        ("attachment", 0.8280388084542077),
        ("get", 0.13040779578930034),
        ("preferential", 0.8280388084542077),
        ("maximum", 0.8708908268792546),
    ],
    &[
        ("get", 0.13040779578930034),
        ("preferential", 0.8280388084542077),
        ("node_ids", 0.3170007004851363),
        ("from", 0.26278152728588133),
        ("attachment", 0.8280388084542077),
        ("unchecked", 0.42267521073949105),
    ],
    &[
        ("attachment", 1.1120893388302326),
        ("node_ids", 0.4257446581149362),
        ("from", 0.35292613335563944),
        ("preferential", 1.1120893388302326),
        ("get", 0.17514290141589564),
    ],
    &[
        ("attachment", 1.1120893388302326),
        ("get", 0.17514290141589564),
        ("node_names", 0.6134881554465302),
        ("preferential", 1.1120893388302326),
        ("from", 0.35292613335563944),
    ],
    &[
        ("attachment", 0.6387561905651457),
        ("preferential", 0.6387561905651457),
        ("unchecked", 0.32605525816149983),
        ("get", 0.10059768456248329),
        ("node_ids", 0.24453704075340602),
        ("weighted", 0.41284435516858997),
        ("from", 0.20271190867656422),
    ],
    &[
        ("node_ids", 0.3170007004851363),
        ("attachment", 0.8280388084542077),
        ("weighted", 0.5351825203109013),
        ("preferential", 0.8280388084542077),
        ("from", 0.26278152728588133),
        ("get", 0.13040779578930034),
    ],
    &[
        ("get", 0.13040779578930034),
        ("weighted", 0.5351825203109013),
        ("node_names", 0.45679064037341954),
        ("from", 0.26278152728588133),
        ("attachment", 0.8280388084542077),
        ("preferential", 0.8280388084542077),
    ],
    &[
        ("unchecked", 0.42267521073949105),
        ("node_ids", 0.3170007004851363),
        ("from", 0.26278152728588133),
        ("jaccard", 1.0508297599412841),
        ("coefficient", 0.896273044061298),
        ("get", 0.13040779578930034),
    ],
    &[
        ("jaccard", 1.4113065245550764),
        ("get", 0.17514290141589564),
        ("coefficient", 1.2037306546564002),
        ("from", 0.35292613335563944),
        ("node_ids", 0.4257446581149362),
    ],
    &[
        ("coefficient", 1.2037306546564002),
        ("get", 0.17514290141589564),
        ("jaccard", 1.4113065245550764),
        ("node_names", 0.6134881554465302),
        ("from", 0.35292613335563944),
    ],
    &[
        ("index", 0.6544128761730416),
        ("get", 0.10059768456248329),
        ("adamic", 0.8106190284071704),
        ("unchecked", 0.32605525816149983),
        ("adar", 0.8106190284071704),
        ("from", 0.20271190867656422),
        ("node_ids", 0.24453704075340602),
    ],
    &[
        ("adar", 1.0508297599412841),
        ("get", 0.13040779578930034),
        ("node_ids", 0.3170007004851363),
        ("adamic", 1.0508297599412841),
        ("index", 0.8483350396087489),
        ("from", 0.26278152728588133),
    ],
    &[
        ("node_names", 0.45679064037341954),
        ("from", 0.26278152728588133),
        ("adamic", 1.0508297599412841),
        ("get", 0.13040779578930034),
        ("adar", 1.0508297599412841),
        ("index", 0.8483350396087489),
    ],
    &[
        ("allocation", 0.7137788271919129),
        ("node_ids", 0.24453704075340602),
        ("from", 0.20271190867656422),
        ("index", 0.6544128761730416),
        ("unchecked", 0.32605525816149983),
        ("resource", 0.7137788271919129),
        ("get", 0.10059768456248329),
    ],
    &[
        ("weighted", 0.32761542166283475),
        ("allocation", 0.5664240009990903),
        ("node_ids", 0.19405401749018308),
        ("index", 0.5193137502908776),
        ("unchecked", 0.25874334855406383),
        ("from", 0.1608634019230581),
        ("resource", 0.5664240009990903),
        ("get", 0.07982997086828104),
    ],
    &[
        ("allocation", 0.9252929025156035),
        ("resource", 0.9252929025156035),
        ("from", 0.26278152728588133),
        ("node_ids", 0.3170007004851363),
        ("index", 0.8483350396087489),
        ("get", 0.13040779578930034),
    ],
    &[
        ("get", 0.13040779578930034),
        ("from", 0.26278152728588133),
        ("resource", 0.9252929025156035),
        ("allocation", 0.9252929025156035),
        ("node_names", 0.45679064037341954),
        ("index", 0.8483350396087489),
    ],
    &[
        ("resource", 0.7137788271919129),
        ("get", 0.10059768456248329),
        ("allocation", 0.7137788271919129),
        ("from", 0.20271190867656422),
        ("node_ids", 0.24453704075340602),
        ("weighted", 0.41284435516858997),
        ("index", 0.6544128761730416),
    ],
    &[
        ("node_names", 0.35237219119648877),
        ("get", 0.10059768456248329),
        ("allocation", 0.7137788271919129),
        ("resource", 0.7137788271919129),
        ("from", 0.20271190867656422),
        ("index", 0.6544128761730416),
        ("weighted", 0.41284435516858997),
    ],
    &[
        ("all", 0.6718126026170259),
        ("from", 0.20271190867656422),
        ("get", 0.10059768456248329),
        ("node_ids", 0.24453704075340602),
        ("unchecked", 0.32605525816149983),
        ("metrics", 0.8106190284071704),
        ("edge", 0.28854518012391517),
    ],
    &[("csv", 5.46379965208028), ("from", 1.1743170181337224)],
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
struct ShortestPathsDjkstra {
    inner: graph::ShortestPathsDjkstra,
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

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    ///
    pub fn into_distances(self) -> Py<PyArray1<f64>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.into_distances(), f64)
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
    "into_iter_finite_distances",
    "into_par_iter_node_ids_and_finite_distances",
    "into_distances",
];

pub const SHORTESTPATHSDJKSTRA_TERMS: &[&str] = &[
    "distance",
    "get",
    "on",
    "finite",
    "most",
    "eccentricity",
    "node_id",
    "node_ids",
    "and",
    "point",
    "into",
    "median",
    "has",
    "from",
    "parent",
    "at",
    "path",
    "shortest",
    "distances",
    "par",
    "distant",
    "node",
    "given",
    "to",
    "iter",
];

pub const SHORTESTPATHSDJKSTRA_TFIDF_FREQUENCIES: &[&[(&str, f64)]] = &[
    &[
        ("has", 0.7340532185700759),
        ("path", 0.5458543045510268),
        ("node_id", 0.4218908489537378),
        ("to", 0.7340532185700759),
    ],
    &[
        ("distance", 0.5458543045510268),
        ("from", 0.5458543045510268),
        ("node_id", 0.4218908489537378),
        ("get", 0.19382377217249755),
    ],
    &[
        ("node_id", 0.4218908489537378),
        ("parent", 0.7340532185700759),
        ("from", 0.5458543045510268),
        ("get", 0.19382377217249755),
    ],
    &[
        ("point", 0.17518972612279576),
        ("at", 0.23559140460863923),
        ("shortest", 0.23559140460863923),
        ("get", 0.06220695390671374),
        ("on", 0.23559140460863923),
        ("distance", 0.17518972612279576),
        ("given", 0.23559140460863923),
        ("path", 0.17518972612279576),
    ],
    &[
        ("point", 0.8166324241314574),
        ("median", 1.0981898545536568),
        ("get", 0.2899725725415318),
    ],
    &[
        ("get", 0.46615843940220925),
        ("eccentricity", 1.7654444497254989),
    ],
    &[
        ("most", 0.7340532185700759),
        ("get", 0.19382377217249755),
        ("distant", 0.7340532185700759),
        ("node", 0.7340532185700759),
    ],
    &[
        ("iter", 0.5458543045510268),
        ("finite", 0.5458543045510268),
        ("distances", 0.4218908489537378),
        ("into", 0.4218908489537378),
    ],
    &[
        ("and", 0.29737763652092625),
        ("finite", 0.22113500610809186),
        ("node_ids", 0.29737763652092625),
        ("par", 0.29737763652092625),
        ("into", 0.1709152692989556),
        ("iter", 0.22113500610809186),
        ("distances", 0.1709152692989556),
    ],
    &[
        ("into", 1.0146741936862047),
        ("distances", 1.0146741936862047),
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

#[pymodule]
fn edge_list_utils(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(convert_edge_list_to_numeric))?;
    m.add_wrapped(wrap_pyfunction!(densify_sparse_numeric_edge_list))?;
    m.add_wrapped(wrap_pyfunction!(are_there_selfloops_in_edge_list))?;
    m.add_wrapped(wrap_pyfunction!(get_rows_number))?;
    m.add_wrapped(wrap_pyfunction!(convert_directed_edge_list_to_undirected))?;
    m.add_wrapped(wrap_pyfunction!(add_numeric_id_to_csv))?;
    m.add_wrapped(wrap_pyfunction!(build_optimal_lists_files))?;
    m.add_wrapped(wrap_pyfunction!(filter_duplicates_from_edge_list))?;
    m.add_wrapped(wrap_pyfunction!(convert_undirected_edge_list_to_directed))?;
    m.add_wrapped(wrap_pyfunction!(get_minmax_node_from_numeric_edge_list))?;
    m.add_wrapped(wrap_pyfunction!(get_selfloops_number_from_edge_list))?;
    m.add_wrapped(wrap_pyfunction!(is_numeric_edge_list))?;
    m.add_wrapped(wrap_pyfunction!(convert_node_list_node_types_to_numeric))?;
    m.add_wrapped(wrap_pyfunction!(sort_numeric_edge_list))?;
    m.add_wrapped(wrap_pyfunction!(sort_numeric_edge_list_inplace))?;
    Ok(())
}

#[pymodule]
fn utils(_py: Python, m: &PyModule) -> PyResult<()> {
    Ok(())
}
