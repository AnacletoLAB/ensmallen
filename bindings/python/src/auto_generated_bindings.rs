#[allow(unused_variables)]
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
    method_name
        .split("_")
        .filter(|x| !x.is_empty())
        .map(|x| x.to_lowercase())
        .collect()
}

#[pymodule]
fn ensmallen(_py: Python, _m: &PyModule) -> PyResult<()> {
    _m.add_class::<ShortestPathsResultBFS>()?;
    _m.add_class::<Circle>()?;
    _m.add_class::<Clique>()?;
    _m.add_class::<Star>()?;
    _m.add_class::<Tendril>()?;
    _m.add_class::<Chain>()?;
    _m.add_class::<NodeTuple>()?;
    _m.add_class::<ShortestPathsDjkstra>()?;
    _m.add_class::<Graph>()?;
    _m.add_class::<DendriticTree>()?;
    _m.add_wrapped(wrap_pymodule!(edge_list_utils))?;
    _m.add_wrapped(wrap_pymodule!(utils))?;
    _m.add_wrapped(wrap_pymodule!(preprocessing))?;
    env_logger::init();
    Ok(())
}

///
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
    /// dst_node_id: int
    ///     The node to start computing predecessors from.
    /// k: int
    ///     Steps to go back.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the predecessors vector was not requested.
    ///
    pub unsafe fn get_unchecked_kth_point_on_shortest_path(
        &self,
        dst_node_id: NodeT,
        k: NodeT,
    ) -> PyResult<NodeT> {
        Ok(pe!(self
            .inner
            .get_unchecked_kth_point_on_shortest_path(dst_node_id.into(), k.into()))?
        .into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, dst_node_id, k)"]
    /// Returns node at the `len - k` position on minimum path to given destination node.
    ///
    /// Parameters
    /// ----------
    /// dst_node_id: int
    ///     The node to start computing predecessors from.
    /// k: int
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
    pub fn get_median_point_to_most_distant_node(&self) -> PyResult<NodeT> {
        Ok(pe!(self.inner.get_median_point_to_most_distant_node())?.into())
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
    /// Returns the number of shortest paths starting from the root node.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If neither predecessors nor distances were computed for this BFS.
    ///
    pub fn get_number_of_shortest_paths(&self) -> PyResult<NodeT> {
        Ok(pe!(self.inner.get_number_of_shortest_paths())?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_id)"]
    /// Returns the number of shortest paths passing through the given node.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     The node id.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If neither predecessors nor distances were computed for this BFS.
    /// ValueError
    ///     If the given node ID does not exist in the current graph instance.
    ///
    pub fn get_number_of_shortest_paths_from_node_id(&self, node_id: NodeT) -> PyResult<NodeT> {
        Ok(pe!(self
            .inner
            .get_number_of_shortest_paths_from_node_id(node_id.into()))?
        .into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, source_node_id)"]
    /// Return list of successors of a given node.
    ///
    /// Parameters
    /// ----------
    /// source_node_id: int
    ///     The node for which to return the successors.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given node ID does not exist in the graph.
    ///
    pub fn get_successors_from_node_id(
        &self,
        source_node_id: NodeT,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_successors_from_node_id(source_node_id.into()))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    ///
    pub fn get_distances(&self) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(gil, pe!(self.inner.get_distances())?, NodeT)
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    ///
    pub fn get_predecessors(&self) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(gil, pe!(self.inner.get_predecessors())?, NodeT)
        })
    }
}

pub const SHORTESTPATHSRESULTBFS_METHODS_NAMES: &[&str] = &[
    "has_path_to_node_id",
    "get_distance_from_node_id",
    "get_parent_from_node_id",
    "get_unchecked_kth_point_on_shortest_path",
    "get_kth_point_on_shortest_path",
    "get_median_point",
    "get_median_point_to_most_distant_node",
    "get_eccentricity",
    "get_most_distant_node",
    "get_number_of_shortest_paths",
    "get_number_of_shortest_paths_from_node_id",
    "get_successors_from_node_id",
    "get_distances",
    "get_predecessors",
];

pub const SHORTESTPATHSRESULTBFS_TERMS: &[&str] = &[
    "has",
    "path",
    "to",
    "node",
    "id",
    "get",
    "distance",
    "from",
    "parent",
    "unchecked",
    "kth",
    "point",
    "on",
    "shortest",
    "median",
    "most",
    "distant",
    "eccentricity",
    "number",
    "of",
    "paths",
    "successors",
    "distances",
    "predecessors",
];

pub const SHORTESTPATHSRESULTBFS_TFIDF_FREQUENCIES: &[&[(&str, f64)]] = &[
    &[
        ("node", 0.19600563),
        ("has", 0.6511166),
        ("to", 0.5066672),
        ("id", 0.28371012),
        ("path", 0.4115208),
    ],
    &[
        ("distance", 0.6511166),
        ("from", 0.34045506),
        ("get", 0.029793462),
        ("id", 0.28371012),
        ("node", 0.19600563),
    ],
    &[
        ("node", 0.19600563),
        ("get", 0.029793462),
        ("id", 0.28371012),
        ("from", 0.34045506),
        ("parent", 0.6511166),
    ],
    &[
        ("unchecked", 0.37579286),
        ("kth", 0.29242367),
        ("shortest", 0.19649409),
        ("path", 0.2375098),
        ("get", 0.017195338),
        ("point", 0.19649409),
        ("on", 0.29242367),
    ],
    &[
        ("get", 0.022245023),
        ("point", 0.25419775),
        ("kth", 0.37829858),
        ("shortest", 0.25419775),
        ("on", 0.37829858),
        ("path", 0.30725837),
    ],
    &[
        ("point", 0.70445216),
        ("get", 0.06164711),
        ("median", 1.0483699),
    ],
    &[
        ("point", 0.19649409),
        ("get", 0.017195338),
        ("median", 0.29242367),
        ("most", 0.29242367),
        ("node", 0.11312492),
        ("distant", 0.29242367),
        ("to", 0.29242367),
    ],
    &[("get", 0.097392075), ("eccentricity", 2.1284401)],
    &[
        ("node", 0.27426687),
        ("distant", 0.70896953),
        ("get", 0.04168941),
        ("most", 0.70896953),
    ],
    &[
        ("number", 0.5066672),
        ("of", 0.5066672),
        ("paths", 0.5066672),
        ("shortest", 0.34045506),
        ("get", 0.029793462),
    ],
    &[
        ("node", 0.08991296),
        ("of", 0.23242162),
        ("id", 0.13014531),
        ("get", 0.013667048),
        ("number", 0.23242162),
        ("shortest", 0.15617572),
        ("from", 0.15617572),
        ("paths", 0.23242162),
    ],
    &[
        ("get", 0.029793462),
        ("id", 0.28371012),
        ("successors", 0.6511166),
        ("node", 0.19600563),
        ("from", 0.34045506),
    ],
    &[("get", 0.097392075), ("distances", 2.1284401)],
    &[("predecessors", 2.1284401), ("get", 0.097392075)],
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
                    jaro_winkler(&name, SHORTESTPATHSRESULTBFS_METHODS_NAMES[id])
                        * frequencies_doc
                            .iter()
                            .map(|(term, weight)| {
                                match tokens_expanded.iter().find(|(token, _)| token == term) {
                                    Some((_, similarity)) => similarity * weight,
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

///
#[pyclass]
#[derive(Debug, Clone)]
pub struct Circle {
    pub inner: graph::Circle,
}

impl From<graph::Circle> for Circle {
    fn from(val: graph::Circle) -> Circle {
        Circle { inner: val }
    }
}

impl From<Circle> for graph::Circle {
    fn from(val: Circle) -> graph::Circle {
        val.inner
    }
}

#[pymethods]
impl Circle {
    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the first node ID of the Circle
    pub fn get_root_node_id(&self) -> NodeT {
        self.inner.get_root_node_id().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the first node name of the circle
    pub fn get_root_node_name(&self) -> String {
        self.inner.get_root_node_name().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return length of the Circle
    pub fn len(&self) -> NodeT {
        self.inner.len().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the node IDs of the nodes composing the Circle
    pub fn get_circle_node_ids(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_circle_node_ids(), NodeT)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, k)"]
    /// Return the first `k` node IDs of the nodes composing the Circle.
    ///
    /// Parameters
    /// ----------
    ///
    pub fn get_first_k_circle_node_ids(&self, k: usize) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_first_k_circle_node_ids(k.into()), NodeT)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, k)"]
    /// Return the first `k` node names of the nodes composing the Circle.
    ///
    /// Parameters
    /// ----------
    ///
    pub fn get_first_k_circle_node_names(&self, k: usize) -> Vec<String> {
        self.inner
            .get_first_k_circle_node_names(k.into())
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the node names of the nodes composing the Circle
    pub fn get_circle_node_names(&self) -> Vec<String> {
        self.inner
            .get_circle_node_names()
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }
}

pub const CIRCLE_METHODS_NAMES: &[&str] = &[
    "get_root_node_id",
    "get_root_node_name",
    "len",
    "get_circle_node_ids",
    "get_first_k_circle_node_ids",
    "get_first_k_circle_node_names",
    "get_circle_node_names",
];

pub const CIRCLE_TERMS: &[&str] = &[
    "get", "root", "node", "id", "name", "len", "circle", "ids", "first", "k", "names",
];

pub const CIRCLE_TFIDF_FREQUENCIES: &[&[(&str, f64)]] = &[
    &[
        ("id", 0.611402),
        ("node", 0.07583805),
        ("get", 0.07583805),
        ("root", 0.42482838),
    ],
    &[
        ("node", 0.07583805),
        ("name", 0.611402),
        ("get", 0.07583805),
        ("root", 0.42482838),
    ],
    &[("len", 2.5416396)],
    &[
        ("get", 0.07583805),
        ("ids", 0.42482838),
        ("node", 0.07583805),
        ("circle", 0.21014561),
    ],
    &[
        ("k", 0.22323874),
        ("first", 0.22323874),
        ("circle", 0.110427275),
        ("get", 0.039851367),
        ("node", 0.039851367),
        ("ids", 0.22323874),
    ],
    &[
        ("circle", 0.110427275),
        ("first", 0.22323874),
        ("get", 0.039851367),
        ("k", 0.22323874),
        ("node", 0.039851367),
        ("names", 0.22323874),
    ],
    &[
        ("circle", 0.21014561),
        ("names", 0.42482838),
        ("node", 0.07583805),
        ("get", 0.07583805),
    ],
];

#[pymethods]
impl Circle {
    fn _repr_html_(&self) -> String {
        self.__repr__()
    }
}

#[pyproto]
impl PyObjectProtocol for Circle {
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
                let mut similarities = CIRCLE_TERMS
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
        let mut doc_scores = CIRCLE_TFIDF_FREQUENCIES
            .par_iter()
            .enumerate()
            // for each document
            .map(|(id, frequencies_doc)| {
                (
                    id,
                    jaro_winkler(&name, CIRCLE_METHODS_NAMES[id])
                        * frequencies_doc
                            .iter()
                            .map(|(term, weight)| {
                                match tokens_expanded.iter().find(|(token, _)| token == term) {
                                    Some((_, similarity)) => similarity * weight,
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
                    format!("* '{}'", CIRCLE_METHODS_NAMES[*method_id].to_string())
                })
                .take(10)
                .collect::<Vec<String>>()
                .join("\n"),
        )))
    }
}

///
#[pyclass]
#[derive(Debug, Clone)]
pub struct Clique {
    pub inner: graph::Clique,
}

impl From<graph::Clique> for Clique {
    fn from(val: graph::Clique) -> Clique {
        Clique { inner: val }
    }
}

impl From<Clique> for graph::Clique {
    fn from(val: Clique) -> graph::Clique {
        val.inner
    }
}

#[pymethods]
impl Clique {
    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return length of the Clique
    pub fn len(&self) -> NodeT {
        self.inner.len().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the node IDs of the nodes composing the clique
    pub fn get_node_ids(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_node_ids(), NodeT)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the node names of the nodes composing the Clique
    pub fn get_node_names(&self) -> Vec<String> {
        self.inner
            .get_node_names()
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }
}

pub const CLIQUE_METHODS_NAMES: &[&str] = &["len", "get_node_ids", "get_node_names"];

pub const CLIQUE_TERMS: &[&str] = &["len", "get", "node", "ids", "names"];

pub const CLIQUE_TFIDF_FREQUENCIES: &[&[(&str, f64)]] = &[
    &[("len", 1.3203471)],
    &[
        ("get", 0.18176937),
        ("ids", 0.37932625),
        ("node", 0.18176937),
    ],
    &[
        ("get", 0.18176937),
        ("node", 0.18176937),
        ("names", 0.37932625),
    ],
];

#[pymethods]
impl Clique {
    fn _repr_html_(&self) -> String {
        self.__repr__()
    }
}

#[pyproto]
impl PyObjectProtocol for Clique {
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
                let mut similarities = CLIQUE_TERMS
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
        let mut doc_scores = CLIQUE_TFIDF_FREQUENCIES
            .par_iter()
            .enumerate()
            // for each document
            .map(|(id, frequencies_doc)| {
                (
                    id,
                    jaro_winkler(&name, CLIQUE_METHODS_NAMES[id])
                        * frequencies_doc
                            .iter()
                            .map(|(term, weight)| {
                                match tokens_expanded.iter().find(|(token, _)| token == term) {
                                    Some((_, similarity)) => similarity * weight,
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
                    format!("* '{}'", CLIQUE_METHODS_NAMES[*method_id].to_string())
                })
                .take(10)
                .collect::<Vec<String>>()
                .join("\n"),
        )))
    }
}

///
#[pyclass]
#[derive(Debug, Clone)]
pub struct Star {
    pub inner: graph::Star,
}

impl From<graph::Star> for Star {
    fn from(val: graph::Star) -> Star {
        Star { inner: val }
    }
}

impl From<Star> for graph::Star {
    fn from(val: Star) -> graph::Star {
        val.inner
    }
}

#[pymethods]
impl Star {
    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the central node ID of the Star
    pub fn get_root_node_id(&self) -> NodeT {
        self.inner.get_root_node_id().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the central node name of the star
    pub fn get_root_node_name(&self) -> String {
        self.inner.get_root_node_name().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return length of the Star
    pub fn len(&self) -> NodeT {
        self.inner.len().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the node IDs of the nodes composing the Star
    pub fn get_star_node_ids(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_star_node_ids(), NodeT)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, k)"]
    /// Return the first `k` node IDs of the nodes composing the star.
    ///
    /// Parameters
    /// ----------
    ///
    pub fn get_first_k_star_node_ids(&self, k: usize) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_first_k_star_node_ids(k.into()), NodeT)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, k)"]
    /// Return the first `k` node names of the nodes composing the star.
    ///
    /// Parameters
    /// ----------
    ///
    pub fn get_first_k_star_node_names(&self, k: usize) -> Vec<String> {
        self.inner
            .get_first_k_star_node_names(k.into())
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the node names of the nodes composing the star
    pub fn get_star_node_names(&self) -> Vec<String> {
        self.inner
            .get_star_node_names()
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }
}

pub const STAR_METHODS_NAMES: &[&str] = &[
    "get_root_node_id",
    "get_root_node_name",
    "len",
    "get_star_node_ids",
    "get_first_k_star_node_ids",
    "get_first_k_star_node_names",
    "get_star_node_names",
];

pub const STAR_TERMS: &[&str] = &[
    "get", "root", "node", "id", "name", "len", "star", "ids", "first", "k", "names",
];

pub const STAR_TFIDF_FREQUENCIES: &[&[(&str, f64)]] = &[
    &[
        ("get", 0.07583805),
        ("root", 0.42482838),
        ("node", 0.07583805),
        ("id", 0.611402),
    ],
    &[
        ("get", 0.07583805),
        ("name", 0.611402),
        ("node", 0.07583805),
        ("root", 0.42482838),
    ],
    &[("len", 2.5416396)],
    &[
        ("node", 0.07583805),
        ("ids", 0.42482838),
        ("star", 0.21014561),
        ("get", 0.07583805),
    ],
    &[
        ("first", 0.22323874),
        ("k", 0.22323874),
        ("star", 0.110427275),
        ("node", 0.039851367),
        ("get", 0.039851367),
        ("ids", 0.22323874),
    ],
    &[
        ("node", 0.039851367),
        ("star", 0.110427275),
        ("names", 0.22323874),
        ("k", 0.22323874),
        ("get", 0.039851367),
        ("first", 0.22323874),
    ],
    &[
        ("get", 0.07583805),
        ("star", 0.21014561),
        ("names", 0.42482838),
        ("node", 0.07583805),
    ],
];

#[pymethods]
impl Star {
    fn _repr_html_(&self) -> String {
        self.__repr__()
    }
}

#[pyproto]
impl PyObjectProtocol for Star {
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
                let mut similarities = STAR_TERMS
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
        let mut doc_scores = STAR_TFIDF_FREQUENCIES
            .par_iter()
            .enumerate()
            // for each document
            .map(|(id, frequencies_doc)| {
                (
                    id,
                    jaro_winkler(&name, STAR_METHODS_NAMES[id])
                        * frequencies_doc
                            .iter()
                            .map(|(term, weight)| {
                                match tokens_expanded.iter().find(|(token, _)| token == term) {
                                    Some((_, similarity)) => similarity * weight,
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
                    format!("* '{}'", STAR_METHODS_NAMES[*method_id].to_string())
                })
                .take(10)
                .collect::<Vec<String>>()
                .join("\n"),
        )))
    }
}

///
#[pyclass]
#[derive(Debug, Clone)]
pub struct Tendril {
    pub inner: graph::Tendril,
}

impl From<graph::Tendril> for Tendril {
    fn from(val: graph::Tendril) -> Tendril {
        Tendril { inner: val }
    }
}

impl From<Tendril> for graph::Tendril {
    fn from(val: Tendril) -> graph::Tendril {
        val.inner
    }
}

#[pymethods]
impl Tendril {
    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the first node ID of the Tendril
    pub fn get_root_node_id(&self) -> NodeT {
        self.inner.get_root_node_id().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the first node name of the Tendril
    pub fn get_root_node_name(&self) -> String {
        self.inner.get_root_node_name().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return length of the Tendril
    pub fn len(&self) -> NodeT {
        self.inner.len().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the node IDs of the nodes composing the Tendril
    pub fn get_tendril_node_ids(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_tendril_node_ids(), NodeT)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, k)"]
    /// Return the first `k` node IDs of the nodes composing the Tendril.
    ///
    /// Parameters
    /// ----------
    ///
    pub fn get_first_k_tendril_node_ids(&self, k: usize) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(
            gil,
            self.inner.get_first_k_tendril_node_ids(k.into()),
            NodeT
        )
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, k)"]
    /// Return the first `k` node names of the nodes composing the Tendril.
    ///
    /// Parameters
    /// ----------
    ///
    pub fn get_first_k_tendril_node_names(&self, k: usize) -> Vec<String> {
        self.inner
            .get_first_k_tendril_node_names(k.into())
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the node names of the nodes composing the Tendril
    pub fn get_tendril_node_names(&self) -> Vec<String> {
        self.inner
            .get_tendril_node_names()
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }
}

pub const TENDRIL_METHODS_NAMES: &[&str] = &[
    "get_root_node_id",
    "get_root_node_name",
    "len",
    "get_tendril_node_ids",
    "get_first_k_tendril_node_ids",
    "get_first_k_tendril_node_names",
    "get_tendril_node_names",
];

pub const TENDRIL_TERMS: &[&str] = &[
    "get", "root", "node", "id", "name", "len", "tendril", "ids", "first", "k", "names",
];

pub const TENDRIL_TFIDF_FREQUENCIES: &[&[(&str, f64)]] = &[
    &[
        ("get", 0.07583805),
        ("root", 0.42482838),
        ("node", 0.07583805),
        ("id", 0.611402),
    ],
    &[
        ("name", 0.611402),
        ("root", 0.42482838),
        ("get", 0.07583805),
        ("node", 0.07583805),
    ],
    &[("len", 2.5416396)],
    &[
        ("node", 0.07583805),
        ("ids", 0.42482838),
        ("get", 0.07583805),
        ("tendril", 0.21014561),
    ],
    &[
        ("get", 0.039851367),
        ("tendril", 0.110427275),
        ("k", 0.22323874),
        ("ids", 0.22323874),
        ("first", 0.22323874),
        ("node", 0.039851367),
    ],
    &[
        ("tendril", 0.110427275),
        ("first", 0.22323874),
        ("k", 0.22323874),
        ("node", 0.039851367),
        ("names", 0.22323874),
        ("get", 0.039851367),
    ],
    &[
        ("names", 0.42482838),
        ("tendril", 0.21014561),
        ("node", 0.07583805),
        ("get", 0.07583805),
    ],
];

#[pymethods]
impl Tendril {
    fn _repr_html_(&self) -> String {
        self.__repr__()
    }
}

#[pyproto]
impl PyObjectProtocol for Tendril {
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
                let mut similarities = TENDRIL_TERMS
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
        let mut doc_scores = TENDRIL_TFIDF_FREQUENCIES
            .par_iter()
            .enumerate()
            // for each document
            .map(|(id, frequencies_doc)| {
                (
                    id,
                    jaro_winkler(&name, TENDRIL_METHODS_NAMES[id])
                        * frequencies_doc
                            .iter()
                            .map(|(term, weight)| {
                                match tokens_expanded.iter().find(|(token, _)| token == term) {
                                    Some((_, similarity)) => similarity * weight,
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
                    format!("* '{}'", TENDRIL_METHODS_NAMES[*method_id].to_string())
                })
                .take(10)
                .collect::<Vec<String>>()
                .join("\n"),
        )))
    }
}

///
#[pyclass]
#[derive(Debug, Clone)]
pub struct Chain {
    pub inner: graph::Chain,
}

impl From<graph::Chain> for Chain {
    fn from(val: graph::Chain) -> Chain {
        Chain { inner: val }
    }
}

impl From<Chain> for graph::Chain {
    fn from(val: Chain) -> graph::Chain {
        val.inner
    }
}

#[pymethods]
impl Chain {
    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the first node ID of the chain
    pub fn get_root_node_id(&self) -> NodeT {
        self.inner.get_root_node_id().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the first node name of the chain
    pub fn get_root_node_name(&self) -> String {
        self.inner.get_root_node_name().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return length of the chain
    pub fn len(&self) -> NodeT {
        self.inner.len().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the node IDs of the nodes composing the chain
    pub fn get_chain_node_ids(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_chain_node_ids(), NodeT)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, k)"]
    /// Return the first `k` node IDs of the nodes composing the chain.
    ///
    /// Parameters
    /// ----------
    ///
    pub fn get_first_k_chain_node_ids(&self, k: usize) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_first_k_chain_node_ids(k.into()), NodeT)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, k)"]
    /// Return the first `k` node names of the nodes composing the chain.
    ///
    /// Parameters
    /// ----------
    ///
    pub fn get_first_k_chain_node_names(&self, k: usize) -> Vec<String> {
        self.inner
            .get_first_k_chain_node_names(k.into())
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the node names of the nodes composing the chain
    pub fn get_chain_node_names(&self) -> Vec<String> {
        self.inner
            .get_chain_node_names()
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }
}

pub const CHAIN_METHODS_NAMES: &[&str] = &[
    "get_root_node_id",
    "get_root_node_name",
    "len",
    "get_chain_node_ids",
    "get_first_k_chain_node_ids",
    "get_first_k_chain_node_names",
    "get_chain_node_names",
];

pub const CHAIN_TERMS: &[&str] = &[
    "get", "root", "node", "id", "name", "len", "chain", "ids", "first", "k", "names",
];

pub const CHAIN_TFIDF_FREQUENCIES: &[&[(&str, f64)]] = &[
    &[
        ("get", 0.07583805),
        ("id", 0.611402),
        ("root", 0.42482838),
        ("node", 0.07583805),
    ],
    &[
        ("root", 0.42482838),
        ("get", 0.07583805),
        ("node", 0.07583805),
        ("name", 0.611402),
    ],
    &[("len", 2.5416396)],
    &[
        ("get", 0.07583805),
        ("chain", 0.21014561),
        ("node", 0.07583805),
        ("ids", 0.42482838),
    ],
    &[
        ("get", 0.039851367),
        ("node", 0.039851367),
        ("chain", 0.110427275),
        ("first", 0.22323874),
        ("k", 0.22323874),
        ("ids", 0.22323874),
    ],
    &[
        ("names", 0.22323874),
        ("chain", 0.110427275),
        ("first", 0.22323874),
        ("node", 0.039851367),
        ("get", 0.039851367),
        ("k", 0.22323874),
    ],
    &[
        ("node", 0.07583805),
        ("names", 0.42482838),
        ("get", 0.07583805),
        ("chain", 0.21014561),
    ],
];

#[pymethods]
impl Chain {
    fn _repr_html_(&self) -> String {
        self.__repr__()
    }
}

#[pyproto]
impl PyObjectProtocol for Chain {
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
                let mut similarities = CHAIN_TERMS
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
        let mut doc_scores = CHAIN_TFIDF_FREQUENCIES
            .par_iter()
            .enumerate()
            // for each document
            .map(|(id, frequencies_doc)| {
                (
                    id,
                    jaro_winkler(&name, CHAIN_METHODS_NAMES[id])
                        * frequencies_doc
                            .iter()
                            .map(|(term, weight)| {
                                match tokens_expanded.iter().find(|(token, _)| token == term) {
                                    Some((_, similarity)) => similarity * weight,
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
                    format!("* '{}'", CHAIN_METHODS_NAMES[*method_id].to_string())
                })
                .take(10)
                .collect::<Vec<String>>()
                .join("\n"),
        )))
    }
}

///
#[pyclass]
#[derive(Debug, Clone)]
pub struct NodeTuple {
    pub inner: graph::NodeTuple,
}

impl From<graph::NodeTuple> for NodeTuple {
    fn from(val: graph::NodeTuple) -> NodeTuple {
        NodeTuple { inner: val }
    }
}

impl From<NodeTuple> for graph::NodeTuple {
    fn from(val: NodeTuple) -> graph::NodeTuple {
        val.inner
    }
}

#[pymethods]
impl NodeTuple {
    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the first node ID of the tuple
    pub fn get_root_node_id(&self) -> NodeT {
        self.inner.get_root_node_id().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the first node name of the tuple
    pub fn get_root_node_name(&self) -> String {
        self.inner.get_root_node_name().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return length of the tuple
    pub fn len(&self) -> NodeT {
        self.inner.len().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the node IDs of the nodes composing the tuple
    pub fn get_node_ids(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_node_ids(), NodeT)
    }
}

pub const NODETUPLE_METHODS_NAMES: &[&str] = &[
    "get_root_node_id",
    "get_root_node_name",
    "len",
    "get_node_ids",
];

pub const NODETUPLE_TERMS: &[&str] = &["get", "root", "node", "id", "name", "len", "ids"];

pub const NODETUPLE_TFIDF_FREQUENCIES: &[&[(&str, f64)]] = &[
    &[
        ("node", 0.1049044),
        ("root", 0.20386682),
        ("get", 0.1049044),
        ("id", 0.35410967),
    ],
    &[
        ("node", 0.1049044),
        ("name", 0.35410967),
        ("root", 0.20386682),
        ("get", 0.1049044),
    ],
    &[("len", 1.7199612)],
    &[
        ("get", 0.16212498),
        ("ids", 0.5472604),
        ("node", 0.16212498),
    ],
];

#[pymethods]
impl NodeTuple {
    fn _repr_html_(&self) -> String {
        self.__repr__()
    }
}

#[pyproto]
impl PyObjectProtocol for NodeTuple {
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
                let mut similarities = NODETUPLE_TERMS
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
        let mut doc_scores = NODETUPLE_TFIDF_FREQUENCIES
            .par_iter()
            .enumerate()
            // for each document
            .map(|(id, frequencies_doc)| {
                (
                    id,
                    jaro_winkler(&name, NODETUPLE_METHODS_NAMES[id])
                        * frequencies_doc
                            .iter()
                            .map(|(term, weight)| {
                                match tokens_expanded.iter().find(|(token, _)| token == term) {
                                    Some((_, similarity)) => similarity * weight,
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
                    format!("* '{}'", NODETUPLE_METHODS_NAMES[*method_id].to_string())
                })
                .take(10)
                .collect::<Vec<String>>()
                .join("\n"),
        )))
    }
}

///
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
    /// dst_node_id: int
    ///     The node to start computing predecessors from.
    /// distance: float
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
    /// Returns the number of shortest paths starting from the root node
    pub fn get_number_of_shortest_paths(&self) -> NodeT {
        self.inner.get_number_of_shortest_paths().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_id)"]
    /// Returns the number of shortest paths passing through the given node.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     The node id.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If neither predecessors nor distances were computed for this BFS.
    /// ValueError
    ///     If the given node ID does not exist in the current graph instance.
    ///
    pub fn get_number_of_shortest_paths_from_node_id(&self, node_id: NodeT) -> PyResult<NodeT> {
        Ok(pe!(self
            .inner
            .get_number_of_shortest_paths_from_node_id(node_id.into()))?
        .into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, source_node_id)"]
    /// Return list of successors of a given node.
    ///
    /// Parameters
    /// ----------
    /// source_node_id: int
    ///     The node for which to return the successors.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given node ID does not exist in the graph.
    ///
    pub fn get_successors_from_node_id(
        &self,
        source_node_id: NodeT,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_successors_from_node_id(source_node_id.into()))?,
                NodeT
            )
        })
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
    "get_number_of_shortest_paths",
    "get_number_of_shortest_paths_from_node_id",
    "get_successors_from_node_id",
];

pub const SHORTESTPATHSDJKSTRA_TERMS: &[&str] = &[
    "has",
    "path",
    "to",
    "node",
    "id",
    "get",
    "distance",
    "from",
    "parent",
    "point",
    "at",
    "given",
    "on",
    "shortest",
    "median",
    "eccentricity",
    "most",
    "distant",
    "number",
    "of",
    "paths",
    "successors",
];

pub const SHORTESTPATHSDJKSTRA_TFIDF_FREQUENCIES: &[&[(&str, f64)]] = &[
    &[
        ("to", 0.58600885),
        ("node", 0.15473326),
        ("has", 0.58600885),
        ("path", 0.435766),
        ("id", 0.20386681),
    ],
    &[
        ("from", 0.2628876),
        ("distance", 0.435766),
        ("node", 0.15473326),
        ("get", 0.043118663),
        ("id", 0.20386681),
    ],
    &[
        ("node", 0.15473326),
        ("id", 0.20386681),
        ("parent", 0.58600885),
        ("from", 0.2628876),
        ("get", 0.043118663),
    ],
    &[
        ("path", 0.20130494),
        ("distance", 0.20130494),
        ("point", 0.20130494),
        ("given", 0.2707106),
        ("on", 0.2707106),
        ("get", 0.019918947),
        ("shortest", 0.15558861),
        ("at", 0.2707106),
    ],
    &[
        ("median", 1.2002592),
        ("point", 0.8925329),
        ("get", 0.088315345),
    ],
    &[("get", 0.13830516), ("eccentricity", 1.8796511)],
    &[
        ("node", 0.21561193),
        ("most", 0.81656975),
        ("distant", 0.81656975),
        ("get", 0.060083386),
    ],
    &[
        ("number", 0.435766),
        ("get", 0.043118663),
        ("shortest", 0.3368036),
        ("of", 0.435766),
        ("paths", 0.435766),
    ],
    &[
        ("get", 0.019918947),
        ("paths", 0.20130494),
        ("number", 0.20130494),
        ("shortest", 0.15558861),
        ("of", 0.20130494),
        ("node", 0.071480036),
        ("from", 0.12144262),
        ("id", 0.094177596),
    ],
    &[
        ("from", 0.2628876),
        ("successors", 0.58600885),
        ("get", 0.043118663),
        ("id", 0.20386681),
        ("node", 0.15473326),
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
                    jaro_winkler(&name, SHORTESTPATHSDJKSTRA_METHODS_NAMES[id])
                        * frequencies_doc
                            .iter()
                            .map(|(term, weight)| {
                                match tokens_expanded.iter().find(|(token, _)| token == term) {
                                    Some((_, similarity)) => similarity * weight,
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

/// This is the main struct in Ensmallen, it allows to load and manipulate Graphs efficently.
///  You are not supposed to directly instantiate this struct but instead you should use the
///  static method `from_csv`, which allows to load the graph from an edge-list.
///
///  To get information about a loaded graph, you can call the `textual_report` method which
///  generates an human-readable HTML report.
///
///  By default we use EliasFano to store the Adjacency Matrix, this allows to save memory but
///  is slower than a CSR. For this reason you can use the `enable` method to enable optimizzations
///  which speeds up the operations at the cost of more memory usage. You can check the memory usage
///  in bytes using `get_total_memory_used` and you can get a detailed memory report of each data-structure
///  inside Graph using `memory_stats`.
///
///  You can pre-compute the memory needed (in bits) to store the adjacency matrix of a Graph with $|E|$ edges and $|V|$ nodes:
///   $$2 |E| + |E| \\left\\lceil \\log_2 \\frac{|V|^2}{|E|} \\right\\rceil$$
///
///  Most Graph properties are automatically cached to speed up.
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
    /// node_id: int
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
    /// node_id: int
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
    /// node_id: int
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
    /// node_id: int
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
    /// node_id: int
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
    /// node_id: int
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
    /// node_name: str
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
    /// node_name: str
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
    /// node_name: str
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
    /// node_type_id: int
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
    /// node_type_name: str
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
    /// edge_type_id: int
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
    /// edge_type_name: str
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
    /// src: int
    ///     Source node id.
    /// dst: int
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
    /// node_id: int
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
    /// src: int
    ///     The source node of the edge.
    /// dst: int
    ///     The destination node of the edge.
    /// edge_type: Optional[int]
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
    /// node_id: int
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
    /// node_id: int
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
    /// node_name: str
    ///     The node name.
    /// node_type_name: Optional[List[str]]
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
    /// src_name: str
    ///     The source node name of the edge.
    /// dst_name: str
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
    /// src_name: str
    ///     The source node name of the edge.
    /// dst_name: str
    ///     The destination node name of the edge.
    /// edge_type_name: Optional[str]
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
        self.inner
            .strongly_connected_components()
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
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
    /// root_node_id: int
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
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_bfs_topological_sorting_from_node_id(root_node_id.into()))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, root_node_id)"]
    /// Returns topological sorting reversed map using breadth-first search from the given node.
    ///
    /// Parameters
    /// ----------
    /// root_node_id: int
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
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_reversed_bfs_topological_sorting_from_node_id(root_node_id.into()))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, root_node_id)"]
    /// Returns graph with node IDs sorted using a BFS
    ///
    /// Parameters
    /// ----------
    /// root_node_id: int
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
    /// weight: Optional[float]
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
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_2d!(
                gil,
                pe!(self
                    .inner
                    .get_dense_weighted_adjacency_matrix(weight.into()))?,
                WeightT
            )
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_names, node_types, edge_types, minimum_component_size, top_k_components, verbose)"]
    /// remove all the components that are not connected to interesting
    /// nodes and edges.
    ///
    /// Parameters
    /// ----------
    /// node_names: Optional[List[str]]
    ///     The name of the nodes of which components to keep.
    /// node_types: Optional[List[Optional[str]]]
    ///     The types of the nodes of which components to keep.
    /// edge_types: Optional[List[Optional[str]]]
    ///     The types of the edges of which components to keep.
    /// minimum_component_size: Optional[int]
    ///     Optional, Minimum size of the components to keep.
    /// top_k_components: Optional[int]
    ///     Optional, number of components to keep sorted by number of nodes.
    /// verbose: Optional[bool]
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
    /// other: Graph
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
    /// other: Graph
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
    /// removed_existing_edges: Optional[bool]
    ///     Whether to filter out the existing edges. By default, true.
    /// first_nodes_set: Optional[Set[str]]
    ///     Optional set of nodes to use to create the first set of nodes of the graph.
    /// second_nodes_set: Optional[Set[str]]
    ///     Optional set of nodes to use to create the second set of nodes of the graph.
    /// first_node_types_set: Optional[Set[str]]
    ///     Optional set of node types to create the first set of nodes of the graph.
    /// second_node_types_set: Optional[Set[str]]
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
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_2d!(
                gil,
                pe!(self.inner.get_bipartite_edges(
                    removed_existing_edges.into(),
                    first_nodes_set.into(),
                    second_nodes_set.into(),
                    first_node_types_set.into(),
                    second_node_types_set.into()
                ))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, removed_existing_edges, first_nodes_set, second_nodes_set, first_node_types_set, second_node_types_set)"]
    /// Return vector of tuple of Node IDs that form the edges of the required bipartite graph.
    ///
    /// Parameters
    /// ----------
    /// removed_existing_edges: Optional[bool]
    ///     Whether to filter out the existing edges. By default, true.
    /// first_nodes_set: Optional[Set[str]]
    ///     Optional set of nodes to use to create the first set of nodes of the graph.
    /// second_nodes_set: Optional[Set[str]]
    ///     Optional set of nodes to use to create the second set of nodes of the graph.
    /// first_node_types_set: Optional[Set[str]]
    ///     Optional set of node types to create the first set of nodes of the graph.
    /// second_node_types_set: Optional[Set[str]]
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
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, central_node, removed_existing_edges, star_points_nodes_set, star_points_node_types_set)"]
    /// Return vector of tuple of Node IDs that form the edges of the required star.
    ///
    /// Parameters
    /// ----------
    /// central_node: str
    ///     Name of the node to use as center of the star.
    /// removed_existing_edges: Optional[bool]
    ///     Whether to filter out the existing edges. By default, true.
    /// star_points_nodes_set: Optional[Set[str]]
    ///     Optional set of nodes to use to create the set of star points.
    /// star_points_node_types_set: Optional[Set[str]]
    ///     Optional set of node types to create the set of star points.
    ///
    pub fn get_star_edges(
        &self,
        central_node: String,
        removed_existing_edges: Option<bool>,
        star_points_nodes_set: Option<HashSet<String>>,
        star_points_node_types_set: Option<HashSet<String>>,
    ) -> PyResult<Py<PyArray2<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_2d!(
                gil,
                pe!(self.inner.get_star_edges(
                    central_node.into(),
                    removed_existing_edges.into(),
                    star_points_nodes_set.into(),
                    star_points_node_types_set.into()
                ))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, central_node, removed_existing_edges, star_points_nodes_set, star_points_node_types_set)"]
    /// Return vector of tuple of Node names that form the edges of the required star.
    ///
    /// Parameters
    /// ----------
    /// central_node: str
    ///     Name of the node to use as center of the star.
    /// removed_existing_edges: Optional[bool]
    ///     Whether to filter out the existing edges. By default, true.
    /// star_points_nodes_set: Optional[Set[str]]
    ///     Optional set of nodes to use to create the set of star points.
    /// star_points_node_types_set: Optional[Set[str]]
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
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, directed, allow_selfloops, removed_existing_edges, allow_node_type_set, allow_node_set)"]
    /// Return vector of tuple of Node IDs that form the edges of the required clique.
    ///
    /// Parameters
    /// ----------
    /// directed: Optional[bool]
    ///     Whether to return the edges as directed or undirected. By default, equal to the graph.
    /// allow_selfloops: Optional[bool]
    ///     Whether to allow self-loops in the clique. By default, equal to the graph.
    /// removed_existing_edges: Optional[bool]
    ///     Whether to filter out the existing edges. By default, true.
    /// allow_node_type_set: Optional[Set[str]]
    ///     Node types to include in the clique.
    /// allow_node_set: Optional[Set[str]]
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
    /// directed: Optional[bool]
    ///     Whether to return the edges as directed or undirected. By default, equal to the graph.
    /// allow_selfloops: Optional[bool]
    ///     Whether to allow self-loops in the clique. By default, equal to the graph.
    /// removed_existing_edges: Optional[bool]
    ///     Whether to filter out the existing edges. By default, true.
    /// allow_node_type_set: Optional[Set[str]]
    ///     Node types to include in the clique.
    /// allow_node_set: Optional[Set[str]]
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
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src, dst)"]
    /// Return edge value corresponding to given node IDs.
    ///
    /// Parameters
    /// ----------
    /// src: int
    ///     The source node ID.
    /// dst: int
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
    /// edge: int
    ///     The edge value to decode.
    ///
    pub fn decode_edge(&self, edge: u64) -> (NodeT, NodeT) {
        let (subresult_0, subresult_1) = self.inner.decode_edge(edge.into());
        (subresult_0.into(), subresult_1.into())
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
    /// node_id: int
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
    /// node_ids: List[int]
    ///     node IDs to validate.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If any of the given node ID does not exists in the graph.
    ///
    pub fn validate_node_ids(&self, node_ids: Vec<NodeT>) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.validate_node_ids(node_ids.into()))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_id)"]
    /// Validates provided edge ID.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int
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
    /// edge_ids: List[int]
    ///     Edge IDs to validate.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If any of the given edge ID does not exists in the graph.
    ///
    pub fn validate_edge_ids(&self, edge_ids: Vec<EdgeT>) -> PyResult<Py<PyArray1<EdgeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.validate_edge_ids(edge_ids.into()))?,
                EdgeT
            )
        })
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
        Ok(pe!(self.inner.must_not_contain_unknown_node_types())?)
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
        Ok(pe!(self.inner.must_not_contain_unknown_edge_types())?)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_type_id)"]
    /// Validates provided node type ID.
    ///
    /// Parameters
    /// ----------
    /// node_type_id: Optional[int]
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
    /// node_type_ids: List[Optional[int]]
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
        Ok(
            pe!(self.inner.validate_node_type_ids(node_type_ids.into()))?
                .into_iter()
                .map(|x| x.into())
                .collect::<Vec<_>>(),
        )
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_type_id)"]
    /// Validates provided edge type ID.
    ///
    /// Parameters
    /// ----------
    /// edge_type_id: Optional[int]
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
    /// edge_type_ids: List[Optional[int]]
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
        Ok(
            pe!(self.inner.validate_edge_type_ids(edge_type_ids.into()))?
                .into_iter()
                .map(|x| x.into())
                .collect::<Vec<_>>(),
        )
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
        Ok(pe!(self.inner.must_be_undirected())?)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Raises an error if the graph contains trap nodes.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph contains trap nodes.
    ///
    pub fn must_not_have_trap_nodes(&self) -> PyResult<()> {
        Ok(pe!(self.inner.must_not_have_trap_nodes())?)
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
        Ok(pe!(self.inner.must_be_multigraph())?)
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
        Ok(pe!(self.inner.must_not_be_multigraph())?)
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
        Ok(pe!(self.inner.must_contain_identity_matrix())?)
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
        Ok(pe!(self.inner.must_not_contain_weighted_singleton_nodes())?)
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
        Ok(pe!(self.inner.must_have_edges())?)
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
        Ok(pe!(self.inner.must_have_nodes())?)
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
        Ok(pe!(self.inner.must_be_connected())?)
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
    /// features: List[List[float]]
    ///     node_features to use to identify the new neighbours.
    /// neighbours_number: Optional[int]
    ///     Number of neighbours to add.
    /// max_degree: Optional[int]
    ///     The maximum degree a node can have its neighbours augmented. By default 0, that is, only singletons are augmented.
    /// distance_name: Optional[str]
    ///     Name of distance to use. Can either be L2 or COSINE. By default COSINE.
    /// verbose: Optional[bool]
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
    /// edge_type: str
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
        Ok({
            pe!(self.inner.set_inplace_all_edge_types(edge_type))?;
            ()
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_type)"]
    /// Replace all edge types (if present) and set all the edge to edge_type.
    ///
    /// This DOES NOT happen inplace, but created a new instance of the graph.
    ///
    /// Parameters
    /// ----------
    /// edge_type: str
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
    /// node_type: str
    ///     The node type to assing to all the nodes.
    ///
    pub fn set_inplace_all_node_types(&mut self, node_type: String) -> PyResult<()> {
        Ok({
            pe!(self.inner.set_inplace_all_node_types(node_type))?;
            ()
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_type)"]
    /// Replace all node types (if present) and set all the node to node_type.
    ///
    /// This DOES NOT happen inplace, but created a new instance of the graph.
    ///
    /// Parameters
    /// ----------
    /// node_type: str
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
    /// node_type_id_to_remove: int
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
        Ok({
            pe!(self
                .inner
                .remove_inplace_node_type_ids(node_type_ids_to_remove.into()))?;
            ()
        })
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
        Ok({
            pe!(self.inner.remove_inplace_singleton_node_types())?;
            ()
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Remove homogeneous node types from all nodes.
    ///
    /// If any given node remains with no node type, that node is labeled
    /// with node type None. Note that the modification happens inplace.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn remove_inplace_homogeneous_node_types(&mut self) -> PyResult<()> {
        Ok({
            pe!(self.inner.remove_inplace_homogeneous_node_types())?;
            ()
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_type_ids_to_remove)"]
    /// Remove given edge type ID from all edges.
    ///
    /// Parameters
    /// ----------
    /// edge_type_id: int
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
        Ok({
            pe!(self
                .inner
                .remove_inplace_edge_type_ids(edge_type_ids_to_remove.into()))?;
            ()
        })
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
        Ok({
            pe!(self.inner.remove_inplace_singleton_edge_types())?;
            ()
        })
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
    /// node_type_name: str
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
        Ok({
            pe!(self
                .inner
                .remove_inplace_node_type_name(node_type_name.into()))?;
            ()
        })
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
    /// node_type_id: int
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
    #[text_signature = "($self)"]
    /// Remove homogeneous node types from all nodes.
    ///
    /// If any given node remains with no node type, that node is labeled
    /// with node type None. Note that the modification DOES NOT happen inplace.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn remove_homogeneous_node_types(&self) -> PyResult<Graph> {
        Ok(pe!(self.inner.remove_homogeneous_node_types())?.into())
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
    /// node_type_name: str
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
    /// edge_type_name: str
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
        Ok({
            pe!(self
                .inner
                .remove_inplace_edge_type_name(edge_type_name.into()))?;
            ()
        })
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
    /// edge_type_id: int
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
    /// edge_type_name: str
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
        Ok({
            pe!(self.inner.remove_inplace_node_types())?;
            ()
        })
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
        Ok({
            pe!(self.inner.remove_inplace_edge_types())?;
            ()
        })
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
        Ok({
            pe!(self.inner.remove_inplace_edge_weights())?;
            ()
        })
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
    #[text_signature = "($self, denominator)"]
    /// Divide edge weights in place.
    ///
    /// Note that the modification happens inplace.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge weights.
    ///
    pub fn divide_edge_weights_inplace(&mut self, denominator: WeightT) -> PyResult<()> {
        Ok(pe!(self
            .inner
            .divide_edge_weights_inplace(denominator.into()))?)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, denominator)"]
    /// Divide edge weights.
    ///
    /// Note that the modification does not happen inplace.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge weights.
    ///
    pub fn divide_edge_weights(&self, denominator: WeightT) -> PyResult<Graph> {
        Ok(pe!(self.inner.divide_edge_weights(denominator.into()))?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, denominator)"]
    /// Multiply edge weights in place.
    ///
    /// Note that the modification happens inplace.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge weights.
    ///
    pub fn multiply_edge_weights_inplace(&mut self, denominator: WeightT) -> PyResult<()> {
        Ok(pe!(self
            .inner
            .multiply_edge_weights_inplace(denominator.into()))?)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, denominator)"]
    /// Multiply edge weights.
    ///
    /// Note that the modification does not happen inplace.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge weights.
    ///
    pub fn multiply_edge_weights(&self, denominator: WeightT) -> PyResult<Graph> {
        Ok(pe!(self.inner.multiply_edge_weights(denominator.into()))?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, minimum_number_of_nodes_per_circle, compute_circle_nodes)"]
    /// Return vector of Circles in the current graph instance.
    ///
    /// Parameters
    /// ----------
    ///
    pub fn get_circles(
        &self,
        minimum_number_of_nodes_per_circle: Option<NodeT>,
        compute_circle_nodes: Option<bool>,
    ) -> PyResult<Vec<Circle>> {
        Ok(pe!(self.inner.get_circles(
            minimum_number_of_nodes_per_circle.into(),
            compute_circle_nodes.into()
        ))?
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<_>>())
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
    /// normalize: Optional[bool]
    ///     Whether to normalize the number of triangles.
    /// low_centrality: Optional[int]
    ///     The threshold over which to switch to parallel matryoshka. By default 50.
    /// verbose: Optional[bool]
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
    /// low_centrality: Optional[int]
    ///     The threshold over which to switch to parallel matryoshka. By default 50.
    /// verbose: Optional[bool]
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
    /// normalize: Optional[bool]
    ///     Whether to normalize the number of triangles.
    /// low_centrality: Optional[int]
    ///     The threshold over which to switch to parallel matryoshka. By default 50.
    /// verbose: Optional[bool]
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
    /// low_centrality: Optional[int]
    ///     The threshold over which to switch to parallel matryoshka. By default 50.
    /// verbose: Optional[bool]
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
    /// low_centrality: Optional[int]
    ///     The threshold over which to switch to parallel matryoshka. By default 50.
    /// verbose: Optional[bool]
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
    /// low_centrality: Optional[int]
    ///     The threshold over which to switch to parallel matryoshka. By default 50.
    /// verbose: Optional[bool]
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
    /// other: Graph
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
    /// node_ids: List[int]
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
    /// node_ids: List[int]
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
    /// node_names: List[str]
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
    #[text_signature = "($self, node_names_map)"]
    /// Returns graph remapped using given node names mapping hashmap.
    ///
    /// Parameters
    /// ----------
    /// node_names_map: Dict[str, str]
    ///     The node names to remap the graph to.
    ///
    pub fn remap_from_node_names_map(
        &self,
        node_names_map: HashMap<String, String>,
    ) -> PyResult<Graph> {
        Ok(pe!(self.inner.remap_from_node_names_map(node_names_map.into()))?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, other)"]
    /// Return graph remapped towards nodes of the given graph.
    ///
    /// Parameters
    /// ----------
    /// other: Graph
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
    /// negatives_number: int
    ///     Number of negatives edges to include.
    /// random_state: Optional[int]
    ///     random_state to use to reproduce negative edge set.
    /// seed_graph: Optional[Graph]
    ///     Optional graph to use to filter the negative edges. The negative edges generated when this variable is provided will always have a node within this graph.
    /// only_from_same_component: Optional[bool]
    ///     Whether to sample negative edges only from nodes that are from the same component.
    /// verbose: Optional[bool]
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
    /// train_size: float
    ///     Rate target to reserve for training.
    /// random_state: Optional[int]
    ///     The random_state to use for the holdout,
    /// edge_types: Optional[List[Optional[str]]]
    ///     Edge types to be selected for in the validation set.
    /// include_all_edge_types: Optional[bool]
    ///     Whether to include all the edges between two nodes.
    /// verbose: Optional[bool]
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
        Ok({
            let (subresult_0, subresult_1) = pe!(self.inner.connected_holdout(
                train_size.into(),
                random_state.into(),
                edge_types.into(),
                include_all_edge_types.into(),
                verbose.into()
            ))?
            .into();
            (subresult_0.into(), subresult_1.into())
        })
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
    /// train_size: float
    ///     rate target to reserve for training
    /// random_state: Optional[int]
    ///     The random_state to use for the holdout,
    /// include_all_edge_types: Optional[bool]
    ///     Whether to include all the edges between two nodes.
    /// edge_types: Optional[List[Optional[str]]]
    ///     The edges to include in validation set.
    /// min_number_overlaps: Optional[int]
    ///     The minimum number of overlaps to include the edge into the validation set.
    /// verbose: Optional[bool]
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
        Ok({
            let (subresult_0, subresult_1) = pe!(self.inner.random_holdout(
                train_size.into(),
                random_state.into(),
                include_all_edge_types.into(),
                edge_types.into(),
                min_number_overlaps.into(),
                verbose.into()
            ))?
            .into();
            (subresult_0.into(), subresult_1.into())
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, train_size, use_stratification, random_state)"]
    /// Returns node-label holdout indices for training ML algorithms on the graph node labels.
    ///
    /// Parameters
    /// ----------
    /// train_size: float
    ///     rate target to reserve for training,
    /// use_stratification: Optional[bool]
    ///     Whether to use node-label stratification,
    /// random_state: Optional[int]
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
    ) -> PyResult<(Py<PyArray1<NodeT>>, Py<PyArray1<NodeT>>)> {
        Ok({
            let (subresult_0, subresult_1) = pe!(self.inner.get_node_label_holdout_indices(
                train_size.into(),
                use_stratification.into(),
                random_state.into()
            ))?
            .into();
            (
                {
                    let gil = pyo3::Python::acquire_gil();
                    to_ndarray_1d!(gil, subresult_0, NodeT)
                },
                {
                    let gil = pyo3::Python::acquire_gil();
                    to_ndarray_1d!(gil, subresult_1, NodeT)
                },
            )
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, train_size, use_stratification, random_state)"]
    /// Returns node-label holdout indices for training ML algorithms on the graph node labels.
    ///
    /// Parameters
    /// ----------
    /// train_size: float
    ///     rate target to reserve for training,
    /// use_stratification: Optional[bool]
    ///     Whether to use node-label stratification,
    /// random_state: Optional[int]
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
        Ok({
            let (subresult_0, subresult_1) = pe!(self.inner.get_node_label_holdout_labels(
                train_size.into(),
                use_stratification.into(),
                random_state.into()
            ))?
            .into();
            (
                subresult_0
                    .into_iter()
                    .map(|x| x.into())
                    .collect::<Vec<_>>(),
                subresult_1
                    .into_iter()
                    .map(|x| x.into())
                    .collect::<Vec<_>>(),
            )
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, train_size, use_stratification, random_state)"]
    /// Returns node-label holdout for training ML algorithms on the graph node labels.
    ///
    /// Parameters
    /// ----------
    /// train_size: float
    ///     rate target to reserve for training,
    /// use_stratification: Optional[bool]
    ///     Whether to use node-label stratification,
    /// random_state: Optional[int]
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
        Ok({
            let (subresult_0, subresult_1) = pe!(self.inner.get_node_label_holdout_graphs(
                train_size.into(),
                use_stratification.into(),
                random_state.into()
            ))?
            .into();
            (subresult_0.into(), subresult_1.into())
        })
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
    /// train_size: float
    ///     rate target to reserve for training,
    /// use_stratification: Optional[bool]
    ///     Whether to use edge-label stratification,
    /// random_state: Optional[int]
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
        Ok({
            let (subresult_0, subresult_1) = pe!(self.inner.get_edge_label_holdout_graphs(
                train_size.into(),
                use_stratification.into(),
                random_state.into()
            ))?
            .into();
            (subresult_0.into(), subresult_1.into())
        })
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
    /// nodes_number: int
    ///     Number of nodes to extract.
    /// random_state: Optional[int]
    ///     Random random_state to use.
    /// verbose: Optional[bool]
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
    /// train_size: float
    ///     rate target to reserve for training,
    /// use_stratification: Optional[bool]
    ///     Whether to use node-label stratification,
    /// random_state: Optional[int]
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
        Ok({
            let (subresult_0, subresult_1) = pe!(self.inner.get_node_label_random_holdout(
                train_size.into(),
                use_stratification.into(),
                random_state.into()
            ))?
            .into();
            (subresult_0.into(), subresult_1.into())
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, k, k_index, use_stratification, random_state)"]
    /// Returns node-label fold for training ML algorithms on the graph node labels.
    ///
    /// Parameters
    /// ----------
    /// k: int
    ///     The number of folds.
    /// k_index: int
    ///     Which fold to use for the validation.
    /// use_stratification: Optional[bool]
    ///     Whether to use node-label stratification,
    /// random_state: Optional[int]
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
        Ok({
            let (subresult_0, subresult_1) = pe!(self.inner.get_node_label_kfold(
                k.into(),
                k_index.into(),
                use_stratification.into(),
                random_state.into()
            ))?
            .into();
            (subresult_0.into(), subresult_1.into())
        })
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
    /// train_size: float
    ///     rate target to reserve for training,
    /// use_stratification: Optional[bool]
    ///     Whether to use edge-label stratification,
    /// random_state: Optional[int]
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
        Ok({
            let (subresult_0, subresult_1) = pe!(self.inner.get_edge_label_random_holdout(
                train_size.into(),
                use_stratification.into(),
                random_state.into()
            ))?
            .into();
            (subresult_0.into(), subresult_1.into())
        })
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
    /// k: int
    ///     The number of folds.
    /// k_index: int
    ///     Which fold to use for the validation.
    /// use_stratification: Optional[bool]
    ///     Whether to use edge-label stratification,
    /// random_state: Optional[int]
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
        Ok({
            let (subresult_0, subresult_1) = pe!(self.inner.get_edge_label_kfold(
                k.into(),
                k_index.into(),
                use_stratification.into(),
                random_state.into()
            ))?
            .into();
            (subresult_0.into(), subresult_1.into())
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
    /// k: int
    ///     The number of folds.
    /// k_index: int
    ///     Which fold to use for the validation.
    /// edge_types: Optional[List[Optional[str]]]
    ///     Edge types to be selected when computing the folds (All the edge types not listed here will be always be used in the training set).
    /// random_state: Optional[int]
    ///     The random_state (seed) to use for the holdout,
    /// verbose: Optional[bool]
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
        Ok({
            let (subresult_0, subresult_1) = pe!(self.inner.get_edge_prediction_kfold(
                k.into(),
                k_index.into(),
                edge_types.into(),
                random_state.into(),
                verbose.into()
            ))?
            .into();
            (subresult_0.into(), subresult_1.into())
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return vector of node tuples in the current graph instance
    pub fn get_node_tuples(&self) -> PyResult<Vec<NodeTuple>> {
        Ok(pe!(self.inner.get_node_tuples())?
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, minimum_number_of_nodes_per_chain, compute_chain_nodes)"]
    /// Return vector of chains in the current graph instance.
    ///
    /// Parameters
    /// ----------
    ///
    pub fn get_chains(
        &self,
        minimum_number_of_nodes_per_chain: Option<NodeT>,
        compute_chain_nodes: Option<bool>,
    ) -> PyResult<Vec<Chain>> {
        Ok(pe!(self.inner.get_chains(
            minimum_number_of_nodes_per_chain.into(),
            compute_chain_nodes.into()
        ))?
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src_node_id)"]
    /// Returns shortest path result for the BFS from given source node ID.
    ///
    /// Parameters
    /// ----------
    /// src_node_id: int
    ///     Root of the tree of minimum paths.
    ///
    ///
    /// Safety
    /// ------
    /// If any of the given node ID does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_breadth_first_search_predecessors_parallel_from_node_id(
        &self,
        src_node_id: NodeT,
    ) -> ShortestPathsResultBFS {
        self.inner
            .get_unchecked_breadth_first_search_predecessors_parallel_from_node_id(
                src_node_id.into(),
            )
            .into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src_node_ids, maximal_depth)"]
    /// Returns shortest path result for the BFS from given source node IDs, treating the set of source nodes as an hyper-node.
    ///
    /// Parameters
    /// ----------
    /// src_node_ids: List[int]
    ///     Roots of the tree of minimum paths.
    /// maximal_depth: Optional[int]
    ///     The maximal depth to run the BFS for.
    ///
    ///
    /// Safety
    /// ------
    /// If any of the given node IDs does not exist in the graph the method will panic.
    ///  The provided list of node ids must be non-empty, or the method will panic.
    pub unsafe fn get_unchecked_breadth_first_search_distances_parallel_from_node_ids(
        &self,
        src_node_ids: Vec<NodeT>,
        maximal_depth: Option<NodeT>,
    ) -> ShortestPathsResultBFS {
        self.inner
            .get_unchecked_breadth_first_search_distances_parallel_from_node_ids(
                src_node_ids.into(),
                maximal_depth.into(),
            )
            .into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src_node_id, maximal_depth)"]
    /// Returns shortest path result for the BFS from given source node ID.
    ///
    /// Parameters
    /// ----------
    /// src_node_id: int
    ///     Root of the tree of minimum paths.
    /// maximal_depth: Optional[int]
    ///     The maximal depth to run the BFS for.
    ///
    ///
    /// Safety
    /// ------
    /// If any of the given node ID does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_breadth_first_search_distances_parallel_from_node_id(
        &self,
        src_node_id: NodeT,
        maximal_depth: Option<NodeT>,
    ) -> ShortestPathsResultBFS {
        self.inner
            .get_unchecked_breadth_first_search_distances_parallel_from_node_id(
                src_node_id.into(),
                maximal_depth.into(),
            )
            .into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src_node_id)"]
    /// Returns shortest path result for the BFS from given source node ID.
    ///
    /// Parameters
    /// ----------
    /// src_node_id: int
    ///     Root of the tree of minimum paths.
    ///
    ///
    /// Safety
    /// ------
    /// If any of the given node ID does not exist in the graph the method will panic.
    ///
    ///  TODO! Explore chains accelerations!
    pub unsafe fn get_unchecked_breadth_first_search_distances_sequential_from_node_id(
        &self,
        src_node_id: NodeT,
    ) -> ShortestPathsResultBFS {
        self.inner
            .get_unchecked_breadth_first_search_distances_sequential_from_node_id(
                src_node_id.into(),
            )
            .into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src_node_ids, dst_node_id, compute_predecessors, maximal_depth)"]
    /// Returns vector of minimum paths distances and vector of nodes predecessors, if requested, treating the set of source nodes as an hyper-node.
    ///
    /// Parameters
    /// ----------
    /// src_node_ids: List[int]
    ///     Root of the tree of minimum paths.
    /// maybe_dst_node_id: Optional[int]
    ///     Optional target destination. If provided, the breadth first search will stop upon reaching this node.
    /// compute_predecessors: Optional[bool]
    ///     Whether to compute the vector of predecessors.
    /// maximal_depth: Optional[int]
    ///     The maximal depth to execute the DFS for.
    ///
    ///
    /// Safety
    /// ------
    /// If any of the given node IDs does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_breadth_first_search_from_node_ids(
        &self,
        src_node_ids: Vec<NodeT>,
        dst_node_id: Option<NodeT>,
        compute_predecessors: Option<bool>,
        maximal_depth: Option<NodeT>,
    ) -> ShortestPathsResultBFS {
        self.inner
            .get_unchecked_breadth_first_search_from_node_ids(
                src_node_ids.into(),
                dst_node_id.into(),
                compute_predecessors.into(),
                maximal_depth.into(),
            )
            .into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src_node_id, dst_node_id, compute_predecessors, maximal_depth)"]
    /// Returns vector of minimum paths distances and vector of nodes predecessors, if requested.
    ///
    /// Parameters
    /// ----------
    /// src_node_id: int
    ///     Root of the tree of minimum paths.
    /// maybe_dst_node_id: Optional[int]
    ///     Optional target destination. If provided, breadth first search will stop upon reaching this node.
    /// compute_predecessors: Optional[bool]
    ///     Whether to compute the vector of predecessors.
    /// maximal_depth: Optional[int]
    ///     The maximal depth to execute the DFS for.
    ///
    ///
    /// Safety
    /// ------
    /// If any of the given node IDs does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_breadth_first_search_from_node_id(
        &self,
        src_node_id: NodeT,
        dst_node_id: Option<NodeT>,
        compute_predecessors: Option<bool>,
        maximal_depth: Option<NodeT>,
    ) -> ShortestPathsResultBFS {
        self.inner
            .get_unchecked_breadth_first_search_from_node_id(
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
    /// src_node_id: int
    ///     Source node ID.
    /// dst_node_id: int
    ///     Destination node ID.
    /// maximal_depth: Optional[int]
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
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_unchecked_shortest_path_node_ids_from_node_ids(
                        src_node_id.into(),
                        dst_node_id.into(),
                        maximal_depth.into()
                    ))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src_node_id, dst_node_id, maximal_depth)"]
    /// Returns minimum path node names from given node ids.
    ///
    /// Parameters
    /// ----------
    /// src_node_id: int
    ///     Source node ID.
    /// dst_node_id: int
    ///     Destination node ID.
    /// maximal_depth: Optional[int]
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
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src_node_id, dst_node_id, maximal_depth)"]
    /// Returns minimum path node names from given node ids.
    ///
    /// Parameters
    /// ----------
    /// src_node_id: int
    ///     Source node ID.
    /// dst_node_id: int
    ///     Destination node ID.
    /// maximal_depth: Optional[int]
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
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_shortest_path_node_ids_from_node_ids(
                    src_node_id.into(),
                    dst_node_id.into(),
                    maximal_depth.into()
                ))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src_node_name, dst_node_name, maximal_depth)"]
    /// Returns minimum path node names from given node names.
    ///
    /// Parameters
    /// ----------
    /// src_node_name: str
    ///     Source node name.
    /// dst_node_name: str
    ///     Destination node name.
    /// maximal_depth: Optional[int]
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
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_shortest_path_node_ids_from_node_names(
                    src_node_name.into(),
                    dst_node_name.into(),
                    maximal_depth.into()
                ))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src_node_name, dst_node_name, maximal_depth)"]
    /// Returns minimum path node names from given node names.
    ///
    /// Parameters
    /// ----------
    /// src_node_name: str
    ///     Source node name.
    /// dst_node_name: str
    ///     Destination node name.
    /// maximal_depth: Optional[int]
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
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src_node_id, dst_node_id, k)"]
    /// Return vector of the k minimum paths node IDs between given source node and destination node ID.
    ///
    /// Parameters
    /// ----------
    /// src_node_id: int
    ///     Source node ID.
    /// dst_node_id: int
    ///     Destination node ID.
    /// k: int
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
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src_node_id, dst_node_id, k)"]
    /// Return vector of the k minimum paths node IDs between given source node and destination node ID.
    ///
    /// Parameters
    /// ----------
    /// src_node_id: int
    ///     Source node ID.
    /// dst_node_id: int
    ///     Destination node ID.
    /// maximal_depth: Optional[int]
    ///     The maximal depth to execute the BFS for.
    /// k: int
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
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src_node_name, dst_node_name, k)"]
    /// Return vector of the k minimum paths node IDs between given source node and destination node name.
    ///
    /// Parameters
    /// ----------
    /// src_node_name: str
    ///     Source node name.
    /// dst_node_name: str
    ///     Destination node name.
    /// k: int
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
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src_node_name, dst_node_name, k)"]
    /// Return vector of the k minimum paths node names between given source node and destination node name.
    ///
    /// Parameters
    /// ----------
    /// src_node_name: str
    ///     Source node name.
    /// dst_node_name: str
    ///     Destination node name.
    /// k: int
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
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>(),
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
    /// node_id: int
    ///     Node for which to compute the eccentricity.
    ///
    ///
    /// Safety
    /// ------
    /// If any of the given node IDs does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_eccentricity_and_most_distant_node_id_from_node_id(
        &self,
        node_id: NodeT,
    ) -> (NodeT, NodeT) {
        let (subresult_0, subresult_1) = self
            .inner
            .get_unchecked_eccentricity_and_most_distant_node_id_from_node_id(node_id.into());
        (subresult_0.into(), subresult_1.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_id, use_edge_weights_as_probabilities)"]
    /// Returns weighted eccentricity of the given node.
    ///
    /// This method will panic if the given node ID does not exists in the graph.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     Node for which to compute the eccentricity.
    /// use_edge_weights_as_probabilities: Optional[bool]
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
    /// node_id: int
    ///     Node for which to compute the eccentricity.
    /// use_edge_weights_as_probabilities: Optional[bool]
    ///     Whether to treat the edge weights as probabilities.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given node ID does not exist in the graph.
    ///
    pub fn get_eccentricity_and_most_distant_node_id_from_node_id(
        &self,
        node_id: NodeT,
    ) -> PyResult<(NodeT, NodeT)> {
        Ok({
            let (subresult_0, subresult_1) = pe!(self
                .inner
                .get_eccentricity_and_most_distant_node_id_from_node_id(node_id.into()))?
            .into();
            (subresult_0.into(), subresult_1.into())
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_id, use_edge_weights_as_probabilities)"]
    /// Returns weighted eccentricity of the given node ID.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     Node for which to compute the eccentricity.
    /// use_edge_weights_as_probabilities: Optional[bool]
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
    /// node_name: str
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
    /// node_name: str
    ///     Node for which to compute the eccentricity.
    /// use_edge_weights_as_probabilities: Optional[bool]
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
    #[text_signature = "($self, src_node_ids, maybe_dst_node_id, maybe_dst_node_ids, compute_predecessors, maximal_depth, use_edge_weights_as_probabilities)"]
    /// Returns vector of minimum paths distances and vector of nodes predecessors, if requested, from the given root nodes (treated as an hyper-node).
    ///
    /// Parameters
    /// ----------
    /// src_node_id: List[int]
    ///     Root of the tree of minimum paths.
    /// maybe_dst_node_id: Optional[int]
    ///     Optional target destination. If provided, Dijkstra will stop upon reaching this node.
    /// maybe_dst_node_ids: Optional[List[int]]
    ///     Optional target destinations. If provided, Dijkstra will stop upon reaching all of these nodes.
    /// compute_predecessors: bool
    ///     Whether to compute the vector of predecessors.
    /// maximal_depth: Optional[int]
    ///     The maximal number of iterations to execute Dijkstra for.
    /// use_edge_weights_as_probabilities: Optional[bool]
    ///     Whether to treat the edge weights as probabilities.
    ///
    ///
    /// Safety
    /// ------
    /// If any of the given node IDs does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_dijkstra_from_node_ids(
        &self,
        src_node_ids: Vec<NodeT>,
        maybe_dst_node_id: Option<NodeT>,
        maybe_dst_node_ids: Option<Vec<NodeT>>,
        compute_predecessors: Option<bool>,
        maximal_depth: Option<NodeT>,
        use_edge_weights_as_probabilities: Option<bool>,
    ) -> ShortestPathsDjkstra {
        self.inner
            .get_unchecked_dijkstra_from_node_ids(
                src_node_ids.into(),
                maybe_dst_node_id.into(),
                maybe_dst_node_ids.into(),
                compute_predecessors.into(),
                maximal_depth.into(),
                use_edge_weights_as_probabilities.into(),
            )
            .into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src_node_id, maybe_dst_node_id, maybe_dst_node_ids, compute_predecessors, maximal_depth, use_edge_weights_as_probabilities)"]
    /// Returns vector of minimum paths distances and vector of nodes predecessors, if requested.
    ///
    /// Parameters
    /// ----------
    /// src_node_id: int
    ///     Root of the tree of minimum paths.
    /// maybe_dst_node_id: Optional[int]
    ///     Optional target destination. If provided, Dijkstra will stop upon reaching this node.
    /// maybe_dst_node_ids: Optional[List[int]]
    ///     Optional target destinations. If provided, Dijkstra will stop upon reaching all of these nodes.
    /// compute_predecessors: bool
    ///     Whether to compute the vector of predecessors.
    /// maximal_depth: Optional[int]
    ///     The maximal number of iterations to execute Dijkstra for.
    /// use_edge_weights_as_probabilities: Optional[bool]
    ///     Whether to treat the edge weights as probabilities.
    ///
    ///
    /// Safety
    /// ------
    /// If any of the given node IDs does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_dijkstra_from_node_id(
        &self,
        src_node_id: NodeT,
        maybe_dst_node_id: Option<NodeT>,
        maybe_dst_node_ids: Option<Vec<NodeT>>,
        compute_predecessors: Option<bool>,
        maximal_depth: Option<NodeT>,
        use_edge_weights_as_probabilities: Option<bool>,
    ) -> ShortestPathsDjkstra {
        self.inner
            .get_unchecked_dijkstra_from_node_id(
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
    /// src_node_id: int
    ///     Source node ID.
    /// dst_node_id: int
    ///     Destination node ID.
    /// use_edge_weights_as_probabilities: Optional[bool]
    ///     Whether to treat the edge weights as probabilities.
    /// maximal_depth: Optional[int]
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
    ) -> (f64, Py<PyArray1<NodeT>>) {
        let (subresult_0, subresult_1) = self
            .inner
            .get_unchecked_weighted_shortest_path_node_ids_from_node_ids(
                src_node_id.into(),
                dst_node_id.into(),
                use_edge_weights_as_probabilities.into(),
                maximal_depth.into(),
            );
        (subresult_0.into(), {
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(gil, subresult_1, NodeT)
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src_node_id, dst_node_id, use_edge_weights_as_probabilities, maximal_depth)"]
    /// Returns minimum path node names from given node ids.
    ///
    /// Parameters
    /// ----------
    /// src_node_id: int
    ///     Source node ID.
    /// dst_node_id: int
    ///     Destination node ID.
    /// use_edge_weights_as_probabilities: Optional[bool]
    ///     Whether to treat the edge weights as probabilities.
    /// maximal_depth: Optional[int]
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
        let (subresult_0, subresult_1) = self
            .inner
            .get_unchecked_weighted_shortest_path_node_names_from_node_ids(
                src_node_id.into(),
                dst_node_id.into(),
                use_edge_weights_as_probabilities.into(),
                maximal_depth.into(),
            );
        (
            subresult_0.into(),
            subresult_1
                .into_iter()
                .map(|x| x.into())
                .collect::<Vec<_>>(),
        )
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src_node_id, dst_node_id, use_edge_weights_as_probabilities, maximal_depth)"]
    /// Returns minimum path node names from given node ids.
    ///
    /// Parameters
    /// ----------
    /// src_node_id: int
    ///     Source node ID.
    /// dst_node_id: int
    ///     Destination node ID.
    /// use_edge_weights_as_probabilities: Optional[bool]
    ///     Whether to treat the edge weights as probabilities.
    /// maximal_depth: Optional[int]
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
    ) -> PyResult<(f64, Py<PyArray1<NodeT>>)> {
        Ok({
            let (subresult_0, subresult_1) = pe!(self
                .inner
                .get_weighted_shortest_path_node_ids_from_node_ids(
                    src_node_id.into(),
                    dst_node_id.into(),
                    use_edge_weights_as_probabilities.into(),
                    maximal_depth.into()
                ))?
            .into();
            (subresult_0.into(), {
                let gil = pyo3::Python::acquire_gil();
                to_ndarray_1d!(gil, subresult_1, NodeT)
            })
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src_node_name, dst_node_name, use_edge_weights_as_probabilities, maximal_depth)"]
    /// Returns minimum path node names from given node names.
    ///
    /// Parameters
    /// ----------
    /// src_node_name: str
    ///     Source node name.
    /// dst_node_name: str
    ///     Destination node name.
    /// use_edge_weights_as_probabilities: Optional[bool]
    ///     Whether to treat the edge weights as probabilities.
    /// maximal_depth: Optional[int]
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
    ) -> PyResult<(f64, Py<PyArray1<NodeT>>)> {
        Ok({
            let (subresult_0, subresult_1) = pe!(self
                .inner
                .get_weighted_shortest_path_node_ids_from_node_names(
                    src_node_name.into(),
                    dst_node_name.into(),
                    use_edge_weights_as_probabilities.into(),
                    maximal_depth.into()
                ))?
            .into();
            (subresult_0.into(), {
                let gil = pyo3::Python::acquire_gil();
                to_ndarray_1d!(gil, subresult_1, NodeT)
            })
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src_node_name, dst_node_name, use_edge_weights_as_probabilities, maximal_depth)"]
    /// Returns minimum path node names from given node names.
    ///
    /// Parameters
    /// ----------
    /// src_node_name: str
    ///     Source node name.
    /// dst_node_name: str
    ///     Destination node name.
    /// use_edge_weights_as_probabilities: Optional[bool]
    ///     Whether to treat the edge weights as probabilities.
    /// maximal_depth: Optional[int]
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
        Ok({
            let (subresult_0, subresult_1) = pe!(self
                .inner
                .get_weighted_shortest_path_node_names_from_node_names(
                    src_node_name.into(),
                    dst_node_name.into(),
                    use_edge_weights_as_probabilities.into(),
                    maximal_depth.into()
                ))?
            .into();
            (
                subresult_0.into(),
                subresult_1
                    .into_iter()
                    .map(|x| x.into())
                    .collect::<Vec<_>>(),
            )
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src_node_id, dst_node_id, compute_predecessors, maximal_depth)"]
    /// Returns vector of minimum paths distances and vector of nodes predecessors from given source node ID and optional destination node ID.
    ///
    /// Parameters
    /// ----------
    /// src_node_id: int
    ///     Node ID root of the tree of minimum paths.
    /// compute_predecessors: Optional[bool]
    ///     Whether to compute the vector of predecessors.
    /// maximal_depth: Optional[int]
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
    /// src_node_id: int
    ///     Node ID root of the tree of minimum paths.
    /// maybe_dst_node_id: Optional[int]
    ///     Optional target destination. If provided, Dijkstra will stop upon reaching this node.
    /// maybe_dst_node_ids: Optional[List[int]]
    ///     Optional target destinations. If provided, Dijkstra will stop upon reaching all of these nodes.
    /// compute_predecessors: Optional[bool]
    ///     Whether to compute the vector of predecessors.
    /// maximal_depth: Optional[int]
    ///     The maximal depth to execute the DFS for.
    /// use_edge_weights_as_probabilities: Optional[bool]
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
    /// ignore_infinity: Optional[bool]
    ///     Whether to ignore infinite distances, which are present when in the graph exist multiple components.
    /// verbose: Optional[bool]
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
    /// ignore_infinity: Optional[bool]
    ///     Whether to ignore infinite distances, which are present when in the graph exist multiple components.
    /// verbose: Optional[bool]
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
    /// ignore_infinity: Optional[bool]
    ///     Whether to ignore infinite distances, which are present when in the graph exist multiple components.
    /// use_edge_weights_as_probabilities: Optional[bool]
    ///     Whether to treat the edge weights as probabilities.
    /// verbose: Optional[bool]
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
    /// src_node_name: str
    ///     Node name root of the tree of minimum paths.
    /// dst_node_name: Optional[str]
    ///     Destination node name.
    /// compute_predecessors: Optional[bool]
    ///     Whether to compute the vector of predecessors.
    /// maximal_depth: Optional[int]
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
    /// src_node_name: str
    ///     Node name root of the tree of minimum paths.
    /// maybe_dst_node_name: Optional[str]
    ///     Optional target destination node name. If provided, Dijkstra will stop upon reaching this node.
    /// maybe_dst_node_names: Optional[List[str]]
    ///     Optional target destination node names. If provided, Dijkstra will stop upon reaching all of these nodes.
    /// compute_predecessors: Optional[bool]
    ///     Whether to compute the vector of predecessors.
    /// maximal_depth: Optional[int]
    ///     The maximal depth to execute the DFS for.
    /// use_edge_weights_as_probabilities: Optional[bool]
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
    /// verbose: Optional[bool]
    ///     Whether to show a loading bar or not.
    ///
    pub fn get_connected_components_number(&self, verbose: Option<bool>) -> (NodeT, NodeT, NodeT) {
        let (subresult_0, subresult_1, subresult_2) =
            self.inner.get_connected_components_number(verbose.into());
        (subresult_0.into(), subresult_1.into(), subresult_2.into())
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
        self.inner
            .get_singleton_node_names()
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
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
        self.inner
            .get_singleton_with_selfloops_node_names()
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
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
    /// directed: bool
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
    /// directed: bool
    ///     Whether to filter out the undirected edges.
    ///
    pub fn get_source_names(&self, directed: bool) -> Vec<String> {
        self.inner
            .get_source_names(directed.into())
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, directed)"]
    /// Return vector on the (non unique) destination nodes of the graph.
    ///
    /// Parameters
    /// ----------
    /// directed: bool
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
    /// directed: bool
    ///     Whether to filter out the undirected edges.
    ///
    pub fn get_destination_names(&self, directed: bool) -> Vec<String> {
        self.inner
            .get_destination_names(directed.into())
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return vector with the sorted nodes names
    pub fn get_node_names(&self) -> Vec<String> {
        self.inner
            .get_node_names()
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return vector with the node URLs.
    pub fn get_node_urls(&self) -> Vec<Option<String>> {
        self.inner
            .get_node_urls()
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return vector with the node predicted ontology.
    pub fn get_node_ontologies(&self) -> Vec<Option<String>> {
        self.inner
            .get_node_ontologies()
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
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
        Ok(pe!(self.inner.get_edge_type_ids())?
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the unique edge type IDs of the graph edges.
    pub fn get_unique_edge_type_ids(&self) -> PyResult<Py<PyArray1<EdgeTypeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(gil, pe!(self.inner.get_unique_edge_type_ids())?, EdgeTypeT)
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the edge types names
    pub fn get_edge_type_names(&self) -> PyResult<Vec<Option<String>>> {
        Ok(pe!(self.inner.get_edge_type_names())?
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the edge types names
    pub fn get_unique_edge_type_names(&self) -> PyResult<Vec<String>> {
        Ok(pe!(self.inner.get_unique_edge_type_names())?
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the weights of the graph edges.
    pub fn get_edge_weights(&self) -> PyResult<Py<PyArray1<WeightT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(gil, pe!(self.inner.get_edge_weights())?, WeightT)
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the weighted indegree (total weighted inbound edge weights) for each node.
    pub fn get_weighted_node_indegrees(&self) -> PyResult<Py<PyArray1<f64>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(gil, pe!(self.inner.get_weighted_node_indegrees())?, f64)
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the node types of the graph nodes.
    pub fn get_node_type_ids(&self) -> PyResult<Vec<Option<Vec<NodeTypeT>>>> {
        Ok(pe!(self.inner.get_node_type_ids())?
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>())
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
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(gil, pe!(self.inner.get_known_node_types_mask())?, bool)
        })
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
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(gil, pe!(self.inner.get_unknown_node_types_mask())?, bool)
        })
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
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_2d!(gil, pe!(self.inner.get_one_hot_encoded_node_types())?, bool)
        })
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
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_2d!(
                gil,
                pe!(self.inner.get_one_hot_encoded_known_node_types())?,
                bool
            )
        })
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
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_2d!(gil, pe!(self.inner.get_one_hot_encoded_edge_types())?, bool)
        })
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
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_2d!(
                gil,
                pe!(self.inner.get_one_hot_encoded_known_edge_types())?,
                bool
            )
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the node types names.
    pub fn get_node_type_names(&self) -> PyResult<Vec<Option<Vec<String>>>> {
        Ok(pe!(self.inner.get_node_type_names())?
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the unique node type IDs of the graph nodes.
    pub fn get_unique_node_type_ids(&self) -> PyResult<Py<PyArray1<NodeTypeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(gil, pe!(self.inner.get_unique_node_type_ids())?, NodeTypeT)
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the unique node types names.
    pub fn get_unique_node_type_names(&self) -> PyResult<Vec<String>> {
        Ok(pe!(self.inner.get_unique_node_type_names())?
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>())
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
    /// directed: bool
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
    /// directed: bool
    ///     Whether to filter out the undirected edges.
    ///
    pub fn get_edge_node_names(&self, directed: bool) -> Vec<(String, String)> {
        self.inner
            .get_edge_node_names(directed.into())
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return vector with the sorted directed edge names
    pub fn get_directed_edge_node_names(&self) -> Vec<(String, String)> {
        self.inner
            .get_directed_edge_node_names()
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
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
    /// Returns number of homogeneous node types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn get_homogeneous_node_types_number(&self) -> PyResult<NodeTypeT> {
        Ok(pe!(self.inner.get_homogeneous_node_types_number())?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns list of homogeneous node type IDs.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn get_homogeneous_node_type_ids(&self) -> PyResult<Py<PyArray1<NodeTypeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_homogeneous_node_type_ids())?,
                NodeTypeT
            )
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns list of homogeneous node type names.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn get_homogeneous_node_type_names(&self) -> PyResult<Vec<String>> {
        Ok(pe!(self.inner.get_homogeneous_node_type_names())?
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>())
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
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_singleton_node_type_ids())?,
                NodeTypeT
            )
        })
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
        Ok(pe!(self.inner.get_singleton_node_type_names())?
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>())
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
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_edge_ids_with_unknown_edge_types())?,
                EdgeT
            )
        })
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
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_edge_ids_with_known_edge_types())?,
                EdgeT
            )
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, directed)"]
    /// Returns edge node IDs of the edges with unknown edge types
    ///
    /// Parameters
    /// ----------
    /// directed: bool
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
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, directed)"]
    /// Returns edge node IDs of the edges with known edge types
    ///
    /// Parameters
    /// ----------
    /// directed: bool
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
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, directed)"]
    /// Returns edge node names of the edges with unknown edge types
    ///
    /// Parameters
    /// ----------
    /// directed: bool
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
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, directed)"]
    /// Returns edge node names of the edges with known edge types
    ///
    /// Parameters
    /// ----------
    /// directed: bool
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
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<_>>())
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
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_edge_ids_with_unknown_edge_types_mask())?,
                bool
            )
        })
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
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_edge_ids_with_known_edge_types_mask())?,
                bool
            )
        })
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
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_node_ids_with_unknown_node_types())?,
                NodeT
            )
        })
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
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_node_ids_with_known_node_types())?,
                NodeT
            )
        })
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
        Ok(pe!(self.inner.get_node_names_with_unknown_node_types())?
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_type_id)"]
    /// Returns node IDs of the nodes with given node type ID.
    ///
    /// Parameters
    /// ----------
    /// node_type_id: int
    ///     The node type ID to filter for.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no node types in the graph.
    ///
    pub fn get_node_ids_from_node_type_id(
        &self,
        node_type_id: NodeTypeT,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_node_ids_from_node_type_id(node_type_id.into()))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_type_id)"]
    /// Returns node names of the nodes with given node type ID.
    ///
    /// Parameters
    /// ----------
    /// node_type_id: int
    ///     The node type ID to filter for.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no node types in the graph.
    ///
    pub fn get_node_names_from_node_type_id(
        &self,
        node_type_id: NodeTypeT,
    ) -> PyResult<Vec<String>> {
        Ok(pe!(self
            .inner
            .get_node_names_from_node_type_id(node_type_id.into()))?
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_type_name)"]
    /// Returns node IDs of the nodes with given node type name.
    ///
    /// Parameters
    /// ----------
    /// node_type_name: str
    ///     The node type ID to filter for.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no node types in the graph.
    ///
    pub fn get_node_ids_from_node_type_name(
        &self,
        node_type_name: &str,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_node_ids_from_node_type_name(node_type_name.into()))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_type_name)"]
    /// Returns node names of the nodes with given node type name.
    ///
    /// Parameters
    /// ----------
    /// node_type_name: str
    ///     The node type ID to filter for.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no node types in the graph.
    ///
    pub fn get_node_names_from_node_type_name(
        &self,
        node_type_name: &str,
    ) -> PyResult<Vec<String>> {
        Ok(pe!(self
            .inner
            .get_node_names_from_node_type_name(node_type_name.into()))?
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<_>>())
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
        Ok(pe!(self.inner.get_node_names_with_known_node_types())?
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>())
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
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_node_ids_with_unknown_node_types_mask())?,
                bool
            )
        })
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
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_node_ids_with_known_node_types_mask())?,
                bool
            )
        })
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
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_singleton_edge_type_ids())?,
                EdgeTypeT
            )
        })
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
        Ok(pe!(self.inner.get_singleton_edge_type_names())?
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>())
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
    /// verbose: Optional[bool]
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
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(gil, pe!(self.inner.get_weighted_node_degrees())?, f64)
        })
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
    /// Return vector wit
    pub fn get_reciprocal_sqrt_degrees(&self) -> Py<PyArray1<WeightT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_reciprocal_sqrt_degrees(), WeightT)
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
        ()
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
    #[text_signature = "($self, minimum_degree, clique_per_node)"]
    /// Returns graph cliques with at least `minimum_degree` nodes.
    ///
    /// Parameters
    /// ----------
    ///
    pub fn get_approximated_cliques(
        &self,
        minimum_degree: Option<NodeT>,
        clique_per_node: Option<usize>,
    ) -> PyResult<Vec<Clique>> {
        Ok(pe!(self
            .inner
            .get_approximated_cliques(minimum_degree.into(), clique_per_node.into()))?
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, minimum_degree, clique_per_node)"]
    /// Returns number of graph cliques with at least `minimum_degree` nodes.
    ///
    /// Parameters
    /// ----------
    ///
    pub fn get_approximated_cliques_number(
        &self,
        minimum_degree: Option<NodeT>,
        clique_per_node: Option<usize>,
    ) -> PyResult<usize> {
        Ok(pe!(self
            .inner
            .get_approximated_cliques_number(minimum_degree.into(), clique_per_node.into()))?
        .into())
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
    /// other: Graph
    ///     graph to create overlap report with.
    /// verbose: Optional[bool]
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
    /// node_id: int
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
    /// node_name: str
    ///     Whether to show a loading bar in graph operations.
    ///
    pub fn get_node_report_from_node_name(&self, node_name: &str) -> PyResult<String> {
        Ok(pe!(self.inner.get_node_report_from_node_name(node_name.into()))?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return html short textual report of the graph.
    ///
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
    /// random_state: int
    ///     The random state to use to reproduce the sampling.
    /// minimum_node_id: int
    ///     The minimum node ID for the connected graph.
    /// minimum_node_sampling: int
    ///     The minimum amount of nodes to sample per node.
    /// maximum_node_sampling: int
    ///     The maximum amount of nodes to sample per node.
    /// nodes_number: Optional[int]
    ///     Number of nodes in the chain. By default 10.
    /// include_selfloops: Optional[bool]
    ///     Whether to include selfloops.
    /// node_type: Optional[str]
    ///     The node type to use for the chain. By default 'chain'.
    /// edge_type: Optional[str]
    ///     The node type to use for the chain. By default 'chain'.
    /// weight: Optional[float]
    ///     The weight to use for the edges in the chain. By default None.
    /// directed: Optional[bool]
    ///     Whether the graph is to built as directed. By default false.
    /// name: Optional[str]
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
    /// random_state: int
    ///     The random state to use to reproduce the sampling.
    /// minimum_node_id: int
    ///     The minimum node ID for the connected graph.
    /// minimum_node_sampling: int
    ///     The minimum amount of nodes to sample per node.
    /// maximum_node_sampling: int
    ///     The maximum amount of nodes to sample per node.
    /// nodes_number: Optional[int]
    ///     Number of nodes in the chain. By default 10.
    /// include_selfloops: Optional[bool]
    ///     Whether to include selfloops.
    /// node_type: Optional[str]
    ///     The node type to use for the chain. By default 'chain'.
    /// edge_type: Optional[str]
    ///     The node type to use for the chain. By default 'chain'.
    /// weight: Optional[float]
    ///     The weight to use for the edges in the chain. By default None.
    /// directed: Optional[bool]
    ///     Whether the graph is to built as directed. By default false.
    /// name: Optional[str]
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
    /// Creates new star graph with given sizes and types.
    ///
    /// Parameters
    /// ----------
    /// minimum_node_id: Optional[int]
    ///     Minimum node ID to start with. May be needed when circleing graphs. By default 0.
    /// nodes_number: Optional[int]
    ///     Number of nodes in the star. By default 10.
    /// include_selfloops: Optional[bool]
    ///     Whether to include selfloops.
    /// node_type: Optional[str]
    ///     The node type to use for the star. By default 'star'.
    /// edge_type: Optional[str]
    ///     The node type to use for the star. By default 'star'.
    /// weight: Optional[float]
    ///     The weight to use for the edges in the star. By default None.
    /// directed: Optional[bool]
    ///     Whether the graph is to built as directed. By default false.
    /// name: Optional[str]
    ///     Name of the graph. By default 'Star'.
    ///
    pub fn generate_star_graph(
        minimum_node_id: Option<NodeT>,
        nodes_number: Option<NodeT>,
        include_selfloops: Option<bool>,
        node_type: Option<&str>,
        edge_type: Option<&str>,
        weight: Option<WeightT>,
        directed: Option<bool>,
        name: Option<&str>,
    ) -> PyResult<Graph> {
        Ok(pe!(graph::Graph::generate_star_graph(
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
    /// Creates new wheel graph with given sizes and types.
    ///
    /// Parameters
    /// ----------
    /// minimum_node_id: Optional[int]
    ///     Minimum node ID to start with. May be needed when circleing graphs. By default 0.
    /// nodes_number: Optional[int]
    ///     Number of nodes in the wheel. By default 10.
    /// include_selfloops: Optional[bool]
    ///     Whether to include selfloops.
    /// node_type: Optional[str]
    ///     The node type to use for the wheel. By default 'wheel'.
    /// edge_type: Optional[str]
    ///     The node type to use for the wheel. By default 'wheel'.
    /// weight: Optional[float]
    ///     The weight to use for the edges in the wheel. By default None.
    /// directed: Optional[bool]
    ///     Whether the graph is to built as directed. By default false.
    /// name: Optional[str]
    ///     Name of the graph. By default 'Wheel'.
    ///
    pub fn generate_wheel_graph(
        minimum_node_id: Option<NodeT>,
        nodes_number: Option<NodeT>,
        include_selfloops: Option<bool>,
        node_type: Option<&str>,
        edge_type: Option<&str>,
        weight: Option<WeightT>,
        directed: Option<bool>,
        name: Option<&str>,
    ) -> PyResult<Graph> {
        Ok(pe!(graph::Graph::generate_wheel_graph(
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
    /// minimum_node_id: Optional[int]
    ///     Minimum node ID to start with. May be needed when circleing graphs. By default 0.
    /// nodes_number: Optional[int]
    ///     Number of nodes in the circle. By default 10.
    /// include_selfloops: Optional[bool]
    ///     Whether to include selfloops.
    /// node_type: Optional[str]
    ///     The node type to use for the circle. By default 'circle'.
    /// edge_type: Optional[str]
    ///     The node type to use for the circle. By default 'circle'.
    /// weight: Optional[float]
    ///     The weight to use for the edges in the circle. By default None.
    /// directed: Optional[bool]
    ///     Whether the graph is to built as directed. By default false.
    /// name: Optional[str]
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
    /// minimum_node_id: Optional[int]
    ///     Minimum node ID to start with. May be needed when chaining graphs. By default 0.
    /// nodes_number: Optional[int]
    ///     Number of nodes in the chain. By default 10.
    /// include_selfloops: Optional[bool]
    ///     Whether to include selfloops.
    /// node_type: Optional[str]
    ///     The node type to use for the chain. By default 'chain'.
    /// edge_type: Optional[str]
    ///     The node type to use for the chain. By default 'chain'.
    /// weight: Optional[float]
    ///     The weight to use for the edges in the chain. By default None.
    /// directed: Optional[bool]
    ///     Whether the graph is to built as directed. By default false.
    /// name: Optional[str]
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
    /// minimum_node_id: Optional[int]
    ///     Minimum node ID to start with. May be needed when combining graphs. By default 0.
    /// nodes_number: Optional[int]
    ///     Number of nodes in the chain. By default 10.
    /// include_selfloops: Optional[bool]
    ///     Whether to include selfloops.
    /// node_type: Optional[str]
    ///     The node type to use. By default 'complete'.
    /// edge_type: Optional[str]
    ///     The node type to use. By default 'complete'.
    /// weight: Optional[float]
    ///     The weight to use for the edges. By default None.
    /// directed: Optional[bool]
    ///     Whether the graph is to built as directed. By default false.
    /// name: Optional[str]
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
    /// minimum_node_id: Optional[int]
    ///     Minimum node ID to start with. May be needed when chaining graphs. By default 0.
    /// left_clique_nodes_number: Optional[int]
    ///     Number of nodes in the left clique. By default 10.
    /// right_clique_nodes_number: Optional[int]
    ///      Number of nodes in the right clique. By default equal to the left clique.
    /// chain_nodes_number: Optional[int]
    ///     Number of nodes in the chain. By default 10.
    /// include_selfloops: Optional[bool]
    ///     Whether to include selfloops.
    /// left_clique_node_type: Optional[str]
    ///     The node type to use for the left clique. By default 'left_clique'.
    /// right_clique_node_type: Optional[str]
    ///     The node type to use for the right clique. By default 'right_clique'.
    /// chain_node_type: Optional[str]
    ///     The node type to use for the chain. By default 'chain'.
    /// left_clique_edge_type: Optional[str]
    ///     The node type to use for the left clique. By default 'left_clique'.
    /// right_clique_edge_type: Optional[str]
    ///     The node type to use for the right clique. By default 'right_clique'.
    /// chain_edge_type: Optional[str]
    ///     The node type to use for the chain. By default 'chain'.
    /// left_clique_weight: Optional[float]
    ///     The weight to use for the edges in the left clique. By default None.
    /// right_clique_weight: Optional[float]
    ///     The weight to use for the edges in the right clique. By default None.
    /// chain_weight: Optional[float]
    ///     The weight to use for the edges in the chain. By default None.
    /// directed: Optional[bool]
    ///     Whether the graph is to built as directed. By default false.
    /// name: Optional[str]
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

    #[staticmethod]
    #[automatically_generated_binding]
    #[text_signature = "(minimum_node_id, clique_nodes_number, chain_nodes_number, include_selfloops, clique_node_type, chain_node_type, clique_edge_type, chain_edge_type, clique_weight, chain_weight, directed, name)"]
    /// Creates new lollipop graph with given sizes and types.
    ///
    /// Parameters
    /// ----------
    /// minimum_node_id: Optional[int]
    ///     Minimum node ID to start with. May be needed when chaining graphs. By default 0.
    /// clique_nodes_number: Optional[int]
    ///     Number of nodes in the left clique. By default 10.
    /// chain_nodes_number: Optional[int]
    ///     Number of nodes in the chain. By default 10.
    /// include_selfloops: Optional[bool]
    ///     Whether to include selfloops.
    /// clique_node_type: Optional[str]
    ///     The node type to use for the left clique. By default 'clique'.
    /// chain_node_type: Optional[str]
    ///     The node type to use for the chain. By default 'chain'.
    /// clique_edge_type: Optional[str]
    ///     The node type to use for the left clique. By default 'clique'.
    /// chain_edge_type: Optional[str]
    ///     The node type to use for the chain. By default 'chain'.
    /// clique_weight: Optional[float]
    ///     The weight to use for the edges in the left clique. By default None.
    /// chain_weight: Optional[float]
    ///     The weight to use for the edges in the chain. By default None.
    /// directed: Optional[bool]
    ///     Whether the graph is to built as directed. By default false.
    /// name: Optional[str]
    ///     Name of the graph. By default 'Lollipop'.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the edge weights are provided only for a subset.
    ///
    pub fn generate_lollipop_graph(
        minimum_node_id: Option<NodeT>,
        clique_nodes_number: Option<NodeT>,
        chain_nodes_number: Option<NodeT>,
        include_selfloops: Option<bool>,
        clique_node_type: Option<&str>,
        chain_node_type: Option<&str>,
        clique_edge_type: Option<&str>,
        chain_edge_type: Option<&str>,
        clique_weight: Option<WeightT>,
        chain_weight: Option<WeightT>,
        directed: Option<bool>,
        name: Option<&str>,
    ) -> PyResult<Graph> {
        Ok(pe!(graph::Graph::generate_lollipop_graph(
            minimum_node_id.into(),
            clique_nodes_number.into(),
            chain_nodes_number.into(),
            include_selfloops.into(),
            clique_node_type.into(),
            chain_node_type.into(),
            clique_edge_type.into(),
            chain_edge_type.into(),
            clique_weight.into(),
            chain_weight.into(),
            directed.into(),
            name.into()
        ))?
        .into())
    }

    #[staticmethod]
    #[automatically_generated_binding]
    #[text_signature = "(sides, minimum_node_id, node_type, weight, directed, name)"]
    /// Creates new squared lattice graph with given sizes and types.
    ///
    /// Parameters
    /// ----------
    /// sides: List[int]
    ///     Sides of the hyper-dimensional lattice with square cell.
    /// minimum_node_id: Optional[int]
    ///     Minimum node ID to start with. May be needed when chaining graphs. By default 0.
    /// node_type: Optional[str]
    ///     The node type to use for the squared lattice. By default 'squared_lattice'.
    /// weight: Optional[float]
    ///     The weight to use for the edges in the left clique. By default None.
    /// directed: Optional[bool]
    ///     Whether the graph is to built as directed. By default false.
    /// name: Optional[str]
    ///     Name of the graph. By default 'Lollipop'.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the edge weights are provided only for a subset.
    ///
    pub fn generate_squared_lattice_graph(
        sides: Vec<NodeT>,
        minimum_node_id: Option<NodeT>,
        node_type: Option<&str>,
        weight: Option<WeightT>,
        directed: Option<bool>,
        name: Option<&str>,
    ) -> PyResult<Graph> {
        Ok(pe!(graph::Graph::generate_squared_lattice_graph(
            &sides,
            minimum_node_id.into(),
            node_type.into(),
            weight.into(),
            directed.into(),
            name.into()
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_ids_to_keep, node_ids_to_filter, node_type_ids_to_keep, node_type_ids_to_filter, node_type_id_to_keep, node_type_id_to_filter, edge_ids_to_keep, edge_ids_to_filter, edge_node_ids_to_keep, edge_node_ids_to_filter, edge_type_ids_to_keep, edge_type_ids_to_filter, min_edge_weight, max_edge_weight, filter_singleton_nodes, filter_singleton_nodes_with_selfloop, filter_selfloops, filter_parallel_edges)"]
    /// Returns a **NEW** Graph that does not have the required attributes.
    ///
    /// Parameters
    /// ----------
    /// node_ids_to_keep: Optional[List[int]]
    ///     List of node IDs to keep during filtering.
    /// node_ids_to_filter: Optional[List[int]]
    ///     List of node IDs to remove during filtering.
    /// node_type_ids_to_keep: Optional[List[Optional[List[int]]]]
    ///     List of node type IDs to keep during filtering. The node types must match entirely the given node types vector provided.
    /// node_type_ids_to_filter: Optional[List[Optional[List[int]]]]
    ///     List of node type IDs to remove during filtering. The node types must match entirely the given node types vector provided.
    /// node_type_id_to_keep: Optional[List[Optional[int]]]
    ///     List of node type IDs to keep during filtering. Any of node types must match with one of the node types given.
    /// node_type_id_to_filter: Optional[List[Optional[int]]]
    ///     List of node type IDs to remove during filtering. Any of node types must match with one of the node types given.
    /// edge_ids_to_keep: Optional[List[int]]
    ///     List of edge IDs to keep during filtering.
    /// edge_ids_to_filter: Optional[List[int]]
    ///     List of edge IDs to remove during filtering.
    /// edge_node_ids_to_keep: Optional[List[Tuple[int, int]]]
    ///     List of tuple of node IDs to keep during filtering.
    /// edge_node_ids_to_filter: Optional[List[Tuple[int, int]]]
    ///     List of tuple of node IDs to remove during filtering.
    /// edge_type_ids_to_keep: Optional[List[Optional[int]]]
    ///     List of edge type IDs to keep during filtering.
    /// edge_type_ids_to_filter: Optional[List[Optional[int]]]
    ///     List of edge type IDs to remove during filtering.
    /// min_edge_weight: Optional[float]
    ///     Minimum edge weight. Values lower than this are removed.
    /// max_edge_weight: Optional[float]
    ///     Maximum edge weight. Values higher than this are removed.
    /// filter_singleton_nodes: Optional[bool]
    ///     Whether to filter out singleton nodes.
    /// filter_singleton_nodes_with_selfloop: Optional[bool]
    ///     Whether to filter out singleton nodes with selfloops.
    /// filter_selfloops: Optional[bool]
    ///     Whether to filter out selfloops.
    /// filter_parallel_edges: Optional[bool]
    ///     Whether to filter out parallel edges.
    /// verbose: Optional[bool]
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
    /// node_names_to_keep: Optional[List[str]]
    ///     List of node names to keep during filtering.
    /// node_names_to_filter: Optional[List[str]]
    ///     List of node names to remove during filtering.
    /// node_type_names_to_keep: Optional[List[Optional[List[str]]]]
    ///     List of node type names to keep during filtering. The node types must match entirely the given node types vector provided.
    /// node_type_names_to_filter: Optional[List[Optional[List[str]]]]
    ///     List of node type names to remove during filtering. The node types must match entirely the given node types vector provided.
    /// node_type_name_to_keep: Optional[List[Optional[str]]]
    ///     List of node type name to keep during filtering. Any of node types must match with one of the node types given.
    /// node_type_name_to_filter: Optional[List[Optional[str]]]
    ///     List of node type name to remove during filtering. Any of node types must match with one of the node types given.
    /// edge_node_names_to_keep: Optional[List[Tuple[str, str]]]
    ///     List of tuple of node names to keep during filtering.
    /// edge_node_names_to_filter: Optional[List[Tuple[str, str]]]
    ///     List of tuple of node names to remove during filtering.
    /// edge_type_names_to_keep: Optional[List[Optional[str]]]
    ///     List of edge type names to keep during filtering.
    /// edge_type_names_to_filter: Optional[List[Optional[str]]]
    ///     List of edge type names to remove during filtering.
    /// min_edge_weight: Optional[float]
    ///     Minimum edge weight. Values lower than this are removed.
    /// max_edge_weight: Optional[float]
    ///     Maximum edge weight. Values higher than this are removed.
    /// filter_singleton_nodes: Optional[bool]
    ///     Whether to filter out singletons.
    /// filter_singleton_nodes_with_selfloop: Optional[bool]
    ///     Whether to filter out singleton nodes with selfloops.
    /// filter_selfloops: Optional[bool]
    ///     Whether to filter out selfloops.
    /// filter_parallel_edges: Optional[bool]
    ///     Whether to filter out parallel edges.
    /// verbose: Optional[bool]
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
    /// Returns new graph without tendrils
    pub fn drop_tendrils(&self) -> PyResult<Graph> {
        Ok(pe!(self.inner.drop_tendrils())?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns new graph without tendrils
    pub fn drop_dendritic_trees(&self) -> PyResult<Graph> {
        Ok(pe!(self.inner.drop_dendritic_trees())?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, minimum_node_degree)"]
    /// Returns new graph without isomorphic nodes, only keeping the smallest node ID of each group.
    ///
    /// Parameters
    /// ----------
    /// minimum_node_degree: Optional[int]
    ///     Minimum node degree for the topological synonims. By default equal to 5.
    ///
    pub fn drop_isomorphic_nodes(&self, minimum_node_degree: Option<NodeT>) -> Graph {
        self.inner
            .drop_isomorphic_nodes(minimum_node_degree.into())
            .into()
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
    /// random_state: Optional[int]
    ///     The random_state to use for the holdout,
    /// undesired_edge_types: Optional[Set[Optional[int]]]
    ///     Which edge types id to try to avoid.
    /// verbose: Optional[bool]
    ///     Whether to show a loading bar or not.
    ///
    pub fn random_spanning_arborescence_kruskal(
        &self,
        random_state: Option<EdgeT>,
        undesired_edge_types: Option<HashSet<Option<EdgeTypeT>>>,
        verbose: Option<bool>,
    ) -> (
        HashSet<(NodeT, NodeT)>,
        Py<PyArray1<NodeT>>,
        NodeT,
        NodeT,
        NodeT,
    ) {
        let (subresult_0, subresult_1, subresult_2, subresult_3, subresult_4) =
            self.inner.random_spanning_arborescence_kruskal(
                random_state.into(),
                undesired_edge_types.into(),
                verbose.into(),
            );
        (
            subresult_0.into(),
            {
                let gil = pyo3::Python::acquire_gil();
                to_ndarray_1d!(gil, subresult_1, NodeT)
            },
            subresult_2.into(),
            subresult_3.into(),
            subresult_4.into(),
        )
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
    /// verbose: Optional[bool]
    ///     Whether to show a loading bar or not.
    ///
    pub fn spanning_arborescence_kruskal(
        &self,
        verbose: Option<bool>,
    ) -> (
        HashSet<(NodeT, NodeT)>,
        Py<PyArray1<NodeT>>,
        NodeT,
        NodeT,
        NodeT,
    ) {
        let (subresult_0, subresult_1, subresult_2, subresult_3, subresult_4) =
            self.inner.spanning_arborescence_kruskal(verbose.into());
        (
            subresult_0.into(),
            {
                let gil = pyo3::Python::acquire_gil();
                to_ndarray_1d!(gil, subresult_1, NodeT)
            },
            subresult_2.into(),
            subresult_3.into(),
            subresult_4.into(),
        )
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
    /// verbose: Optional[bool]
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
    pub fn get_connected_components(
        &self,
        verbose: Option<bool>,
    ) -> PyResult<(Py<PyArray1<NodeT>>, NodeT, NodeT, NodeT)> {
        Ok({
            let (subresult_0, subresult_1, subresult_2, subresult_3) =
                pe!(self.inner.get_connected_components(verbose.into()))?.into();
            (
                {
                    let gil = pyo3::Python::acquire_gil();
                    to_ndarray_1d!(gil, subresult_0, NodeT)
                },
                subresult_1.into(),
                subresult_2.into(),
                subresult_3.into(),
            )
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, vector_sources, vector_destinations, vector_cumulative_node_degrees, vector_reciprocal_sqrt_degrees)"]
    /// Enable extra perks that buys you time as you accept to spend more memory.
    ///
    /// Parameters
    /// ----------
    /// vector_sources: Optional[bool]
    ///     Whether to cache sources into a vector for faster walks.
    /// vector_destinations: Optional[bool]
    ///     Whether to cache destinations into a vector for faster walks.
    /// vector_cumulative_node_degrees: Optional[bool]
    ///     Whether to cache cumulative_node_degrees into a vector for faster walks.
    /// vector_reciprocal_sqrt_degrees: Optional[bool]
    ///     Whether to cache reciprocal_sqrt_degrees into a vector for faster laplacian kernel computation.
    ///
    pub fn enable(
        &mut self,
        vector_sources: Option<bool>,
        vector_destinations: Option<bool>,
        vector_cumulative_node_degrees: Option<bool>,
        vector_reciprocal_sqrt_degrees: Option<bool>,
    ) -> PyResult<()> {
        Ok(pe!(self.inner.enable(
            vector_sources.into(),
            vector_destinations.into(),
            vector_cumulative_node_degrees.into(),
            vector_reciprocal_sqrt_degrees.into()
        ))?)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, other)"]
    /// Return true if the graphs are compatible.
    ///
    /// Parameters
    /// ----------
    /// other: Graph
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
    /// other: Graph
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
    #[text_signature = "($self, random_state)"]
    /// Return random number.
    ///
    /// Parameters
    /// ----------
    /// random_state: int
    ///     The random state to use to reproduce the sampling.
    ///
    pub fn get_random_node(&self, random_state: u64) -> NodeT {
        self.inner.get_random_node(random_state.into()).into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, number_of_nodes_to_sample, random_state)"]
    /// Return random unique sorted numbers.
    ///
    /// Parameters
    /// ----------
    /// number_of_nodes_to_sample: int
    ///     The number of nodes to sample.
    /// random_state: int
    ///     The random state to use to reproduce the sampling.
    ///
    pub fn get_random_nodes(
        &self,
        number_of_nodes_to_sample: NodeT,
        random_state: u64,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_random_nodes(number_of_nodes_to_sample.into(), random_state.into()))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, number_of_nodes_to_sample, root_node)"]
    /// Return nodes sampled from the neighbourhood of given root nodes.
    ///
    /// Parameters
    /// ----------
    /// number_of_nodes_to_sample: int
    ///     The number of nodes to sample.
    /// root_node: int
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
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_breadth_first_search_random_nodes(
                    number_of_nodes_to_sample.into(),
                    root_node.into()
                ))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node, random_state, walk_length, unique)"]
    /// Returns unique nodes sampled from uniform random walk.
    ///
    /// Parameters
    /// ----------
    /// node: int
    ///     Node from where to start the random walks.
    /// random_state: int
    ///     the random_state to use for extracting the nodes and edges.
    /// walk_length: int
    ///     Length of the random walk.
    /// unique: Optional[bool]
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
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_uniform_random_walk_random_nodes(
                    node.into(),
                    random_state.into(),
                    walk_length.into(),
                    unique.into()
                ))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return list of the supported node sampling methods
    pub fn get_node_sampling_methods(&self) -> Vec<&str> {
        self.inner
            .get_node_sampling_methods()
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, number_of_nodes_to_sample, random_state, root_node, node_sampling_method, unique)"]
    /// Return subsampled nodes according to the given method and parameters.
    ///
    /// Parameters
    /// ----------
    /// number_of_nodes_to_sample: int
    ///     The number of nodes to sample.
    /// random_state: int
    ///     The random state to reproduce the sampling.
    /// root_node: Optional[int]
    ///     The (optional) root node to use to sample. In not provided, a random one is sampled.
    /// node_sampling_method: str
    ///     The method to use to sample the nodes. Can either be random nodes, breath first search-based or uniform random walk-based.
    /// unique: Optional[bool]
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
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_subsampled_nodes(
                    number_of_nodes_to_sample.into(),
                    random_state.into(),
                    root_node.into(),
                    node_sampling_method.into(),
                    unique.into()
                ))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, features, iterations, maximal_distance, k1, b, include_central_node, verbose)"]
    /// Returns okapi node features propagation within given maximal distance.
    ///
    /// Parameters
    /// ----------
    /// features: List[Optional[List[float]]]
    ///     The features to propagate. Use None to represent eventual unknown features.
    /// iterations: Optional[int]
    ///     The number of iterations to execute. By default one.
    /// maximal_distance: Optional[int]
    ///     The distance to consider for the cooccurrences. The default value is 3.
    /// k1: Optional[float]
    ///     The k1 parameter from okapi. Tipicaly between 1.2 and 2.0. It can be seen as a smoothing.
    /// b: Optional[float]
    ///     The b parameter from okapi. Tipicaly 0.75.
    /// include_central_node: Optional[bool]
    ///     Whether to include the central node. By default true.
    /// verbose: Optional[bool]
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
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_2d!(
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
            )
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, iterations, maximal_distance, k1, b, verbose)"]
    /// Returns okapi node label propagation within given maximal distance.
    ///
    /// Parameters
    /// ----------
    /// iterations: Optional[int]
    ///     The number of iterations to execute. By default one.
    /// maximal_distance: Optional[int]
    ///     The distance to consider for the cooccurrences. The default value is 3.
    /// k1: Optional[float]
    ///     The k1 parameter from okapi. Tipicaly between 1.2 and 2.0. It can be seen as a smoothing.
    /// b: Optional[float]
    ///     The b parameter from okapi. Tipicaly 0.75.
    /// verbose: Optional[bool]
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
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_2d!(
                gil,
                pe!(self.inner.get_okapi_bm25_node_label_propagation(
                    iterations.into(),
                    maximal_distance.into(),
                    k1.into(),
                    b.into(),
                    verbose.into()
                ))?,
                f64
            )
        })
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
    /// verbose: Optional[bool]
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
    #[text_signature = "($self)"]
    /// Returns vector of detected dentritic trees
    pub fn get_dendritic_trees(&self) -> PyResult<Vec<DendriticTree>> {
        Ok(pe!(self.inner.get_dendritic_trees())?
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, iterations, verbose)"]
    /// Returns graph to the i-th transitivity closure iteration.
    ///
    /// Parameters
    /// ----------
    /// iterations: Optional[int]
    ///     The number of iterations of the transitive closure to execute. If None, the complete transitive closure is computed.
    /// verbose: Optional[bool]
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
    /// iterations: Optional[int]
    ///     The number of iterations of the transitive closure to execute. If None, the complete transitive closure is computed.
    /// verbose: Optional[bool]
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
    /// iterations: Optional[int]
    ///     The number of iterations of the transitive closure to execute. If None, the complete transitive closure is computed.
    /// use_edge_weights_as_probabilities: Optional[bool]
    ///     Whether to treat the edge weights as probabilities.
    /// verbose: Optional[bool]
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
    /// edge_id: int
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
    /// src: int
    ///     The source node ID.
    /// dst: int
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
    /// node_name: str
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
    /// edge_type_name: str
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
    /// edge_type_id: Optional[int]
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
    /// edge_type: Optional[int]
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
    /// src: int
    ///     Source node of the edge.
    /// dst: int
    ///     Destination node of the edge.
    /// edge_type: Optional[int]
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
    /// src: int
    ///     Source node.
    /// dst: int
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
        let (subresult_0, subresult_1) = self
            .inner
            .get_unchecked_minmax_edge_ids_from_node_ids(src.into(), dst.into());
        (subresult_0.into(), subresult_1.into())
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
    /// edge_id: int
    ///     The edge ID whose source and destination node IDs are to e retrieved.
    ///
    ///
    /// Safety
    /// ------
    /// If the given edge ID does not exist in the current graph the method will raise a panic.
    pub unsafe fn get_unchecked_node_ids_from_edge_id(&self, edge_id: EdgeT) -> (NodeT, NodeT) {
        let (subresult_0, subresult_1) = self
            .inner
            .get_unchecked_node_ids_from_edge_id(edge_id.into());
        (subresult_0.into(), subresult_1.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_id)"]
    /// Returns node names corresponding to given edge ID.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int
    ///     The edge ID whose source and destination node IDs are to e retrieved.
    ///
    ///
    /// Safety
    /// ------
    /// If the given edge ID does not exist in the current graph the method will raise a panic.
    pub unsafe fn get_unchecked_node_names_from_edge_id(&self, edge_id: EdgeT) -> (String, String) {
        let (subresult_0, subresult_1) = self
            .inner
            .get_unchecked_node_names_from_edge_id(edge_id.into());
        (subresult_0.into(), subresult_1.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_id)"]
    /// Returns the source of given edge id without making any boundary check.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int
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
    /// edge_id: int
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
    /// edge_id: int
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
    /// edge_id: int
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
    /// edge_id: int
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
    /// edge_id: int
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
    /// edge_id: int
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
    /// edge_id: int
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
    /// edge_id: int
    ///     The edge ID whose source and destination node IDs are to e retrieved.
    ///
    pub fn get_node_names_from_edge_id(&self, edge_id: EdgeT) -> PyResult<(String, String)> {
        Ok({
            let (subresult_0, subresult_1) =
                pe!(self.inner.get_node_names_from_edge_id(edge_id.into()))?.into();
            (subresult_0.into(), subresult_1.into())
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_id)"]
    /// Returns node names corresponding to given edge ID.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int
    ///     The edge ID whose source and destination node IDs are to e retrieved.
    ///
    pub fn get_node_ids_from_edge_id(&self, edge_id: EdgeT) -> PyResult<(NodeT, NodeT)> {
        Ok({
            let (subresult_0, subresult_1) =
                pe!(self.inner.get_node_ids_from_edge_id(edge_id.into()))?.into();
            (subresult_0.into(), subresult_1.into())
        })
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
    /// src: int
    ///     The source node ID.
    /// dst: int
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
    /// src: int
    ///     The source node ID.
    /// dst: int
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
    /// source_id: int
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
    /// edge_id: int
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
        let (subresult_0, subresult_1, subresult_2) = self
            .inner
            .get_unchecked_node_ids_and_edge_type_id_from_edge_id(edge_id.into());
        (subresult_0.into(), subresult_1.into(), subresult_2.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_id)"]
    /// Return the src, dst, edge type of a given edge ID.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int
    ///     The edge ID whose source, destination and edge type are to be retrieved.
    ///
    pub fn get_node_ids_and_edge_type_id_from_edge_id(
        &self,
        edge_id: EdgeT,
    ) -> PyResult<(NodeT, NodeT, Option<EdgeTypeT>)> {
        Ok({
            let (subresult_0, subresult_1, subresult_2) = pe!(self
                .inner
                .get_node_ids_and_edge_type_id_from_edge_id(edge_id.into()))?
            .into();
            (subresult_0.into(), subresult_1.into(), subresult_2.into())
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_id)"]
    /// Return the src, dst, edge type and weight of a given edge ID.
    ///
    /// This method will raise a panic when an improper configuration is used.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int
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
        let (subresult_0, subresult_1, subresult_2, subresult_3) = self
            .inner
            .get_unchecked_node_ids_and_edge_type_id_and_edge_weight_from_edge_id(edge_id.into());
        (
            subresult_0.into(),
            subresult_1.into(),
            subresult_2.into(),
            subresult_3.into(),
        )
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_id)"]
    /// Return the src, dst, edge type and weight of a given edge ID.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int
    ///     The edge ID whose source, destination, edge type and weight are to be retrieved.
    ///
    pub fn get_node_ids_and_edge_type_id_and_edge_weight_from_edge_id(
        &self,
        edge_id: EdgeT,
    ) -> PyResult<(NodeT, NodeT, Option<EdgeTypeT>, Option<WeightT>)> {
        Ok({
            let (subresult_0, subresult_1, subresult_2, subresult_3) = pe!(self
                .inner
                .get_node_ids_and_edge_type_id_and_edge_weight_from_edge_id(edge_id.into()))?
            .into();
            (
                subresult_0.into(),
                subresult_1.into(),
                subresult_2.into(),
                subresult_3.into(),
            )
        })
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
    /// k: int
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
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_top_k_central_node_ids(k.into()))?,
                NodeT
            )
        })
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
    /// k: int
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
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_weighted_top_k_central_node_ids(k.into()))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_id)"]
    /// Returns the number of outbound neighbours of given node.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
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
    /// node_id: int
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
    /// node_id: int
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
    /// node_id: int
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
    /// node_id: int
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
    /// Returns the reciprocal squared root node degree up to the given node.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     Integer ID of the node.
    ///
    ///
    /// Safety
    /// ------
    /// If the given node ID does not exist in the current graph the method will raise a panic.
    pub unsafe fn get_unchecked_reciprocal_sqrt_degree_from_node_id(
        &self,
        node_id: NodeT,
    ) -> WeightT {
        self.inner
            .get_unchecked_reciprocal_sqrt_degree_from_node_id(node_id.into())
            .into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_id)"]
    /// Returns the reciprocal squared root node degree up to the given node.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     Integer ID of the node.
    ///
    pub fn get_reciprocal_sqrt_degree_from_node_id(&self, node_id: NodeT) -> PyResult<WeightT> {
        Ok(pe!(self
            .inner
            .get_reciprocal_sqrt_degree_from_node_id(node_id.into()))?
        .into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_ids)"]
    /// Return vector with reciprocal squared root degree of the provided nodes.
    ///
    /// Parameters
    /// ----------
    /// node_ids: List[int]
    ///     The vector of node IDs whose reciprocal squared root degree is to be retrieved.
    ///
    ///
    /// Safety
    /// ------
    /// This method makes the assumption that the provided node IDs exist in the graph, that is
    ///  they are not higher than the number of nodes in the graph.
    pub unsafe fn get_unchecked_reciprocal_sqrt_degrees_from_node_ids(
        &self,
        node_ids: Vec<NodeT>,
    ) -> Py<PyArray1<WeightT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(
            gil,
            self.inner
                .get_unchecked_reciprocal_sqrt_degrees_from_node_ids(&node_ids),
            WeightT
        )
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_id)"]
    /// Returns the weighted sum of outbound neighbours of given node ID.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
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
    /// node_name: str
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
    /// k: int
    ///     Number of central nodes to extract.
    ///
    pub fn get_top_k_central_node_names(&self, k: NodeT) -> PyResult<Vec<String>> {
        Ok(pe!(self.inner.get_top_k_central_node_names(k.into()))?
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>())
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
    /// node_id: int
    ///     node whose node type is to be returned.
    ///
    ///
    /// Safety
    /// ------
    /// Even though the method will return an option when the node types are
    ///  not available for the current graph, the behaviour is undefined.
    pub unsafe fn get_unchecked_node_type_ids_from_node_id(
        &self,
        node_id: NodeT,
    ) -> Option<Vec<NodeTypeT>> {
        self.inner
            .get_unchecked_node_type_ids_from_node_id(node_id.into())
            .into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_id)"]
    /// Returns node type of given node.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
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
    /// edge_id: int
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
    /// edge_id: int
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
    /// node_id: int
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
    /// node_id: int
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
    /// node_name: str
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
    /// edge_id: int
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
    /// edge_type_id: int
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
    /// edge_id: int
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
    /// src: int
    ///     The node ID of the source node.
    /// dst: int
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
    /// src: int
    ///     The node ID of the source node.
    /// dst: int
    ///     The node ID of the destination node.
    /// edge_type: Optional[int]
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
    /// src: str
    ///     The node name of the source node.
    /// dst: str
    ///     The node name of the destination node.
    /// edge_type: Optional[str]
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
    /// src_name: str
    ///     The node name of the source node.
    /// dst_name: str
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
    /// node_id: int
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
    /// node_id: int
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
    /// node_name: str
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
    /// node_names: List[str]
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
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_node_ids_from_node_names(node_names.into()))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_node_names)"]
    /// Returns result with the edge node IDs.
    ///
    /// Parameters
    /// ----------
    /// edge_node_names: List[Tuple[str, str]]
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
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_node_ids)"]
    /// Returns result with the edge node names.
    ///
    /// Parameters
    /// ----------
    /// edge_node_ids: List[Tuple[int, int]]
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
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_name)"]
    /// Return node type ID for the given node name if available.
    ///
    /// Parameters
    /// ----------
    /// node_name: str
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
    /// node_name: str
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
    /// edge_type_id: Optional[int]
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
    /// edge_type_name: Optional[str]
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
    /// edge_type_name: Optional[str]
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
    /// node_type_name: str
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
    /// node_type_id: Optional[int]
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
    /// node_type_name: Optional[str]
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
    /// node_id: int
    ///     Node ID whose neighbours are to be retrieved.
    ///
    pub fn get_neighbour_node_ids_from_node_id(
        &self,
        node_id: NodeT,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_neighbour_node_ids_from_node_id(node_id.into()))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_name)"]
    /// Return vector of destinations for the given source node name.
    ///
    /// Parameters
    /// ----------
    /// node_name: str
    ///     Node ID whose neighbours are to be retrieved.
    ///
    pub fn get_neighbour_node_ids_from_node_name(
        &self,
        node_name: &str,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_neighbour_node_ids_from_node_name(node_name.into()))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_name)"]
    /// Return vector of destination names for the given source node name.
    ///
    /// Parameters
    /// ----------
    /// node_name: str
    ///     Node name whose neighbours are to be retrieved.
    ///
    pub fn get_neighbour_node_names_from_node_name(
        &self,
        node_name: &str,
    ) -> PyResult<Vec<String>> {
        Ok(pe!(self
            .inner
            .get_neighbour_node_names_from_node_name(node_name.into()))?
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src, dst)"]
    /// Return range of outbound edges IDs for all the edges bewteen the given
    /// source and destination nodes.
    /// This operation is meaningfull only in a multigraph.
    ///
    /// Parameters
    /// ----------
    /// src: int
    ///     Source node.
    /// dst: int
    ///     Destination node.
    ///
    pub fn get_minmax_edge_ids_from_node_ids(
        &self,
        src: NodeT,
        dst: NodeT,
    ) -> PyResult<(EdgeT, EdgeT)> {
        Ok({
            let (subresult_0, subresult_1) = pe!(self
                .inner
                .get_minmax_edge_ids_from_node_ids(src.into(), dst.into()))?
            .into();
            (subresult_0.into(), subresult_1.into())
        })
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
    /// src: int
    ///     Source node of the edge.
    /// dst: int
    ///     Destination node of the edge.
    /// edge_type: Optional[int]
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
    /// src_name: str
    ///     Source node name of the edge.
    /// dst_name: str
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
    /// src_name: str
    ///     Source node name of the edge.
    /// dst_name: str
    ///     Destination node name of the edge.
    /// edge_type_name: Optional[str]
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
    /// edge_type_names: List[Optional[str]]
    ///     Vector of edge types to be converted.
    ///
    pub fn get_edge_type_ids_from_edge_type_names(
        &self,
        edge_type_names: Vec<Option<String>>,
    ) -> PyResult<Vec<Option<EdgeTypeT>>> {
        Ok(pe!(self
            .inner
            .get_edge_type_ids_from_edge_type_names(edge_type_names.into()))?
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_type_names)"]
    /// Return translated node types from string to internal node ID.
    ///
    /// Parameters
    /// ----------
    /// node_type_names: List[Optional[str]]
    ///     Vector of node types to be converted.
    ///
    pub fn get_node_type_ids_from_node_type_names(
        &self,
        node_type_names: Vec<Option<String>>,
    ) -> PyResult<Vec<Option<NodeTypeT>>> {
        Ok(pe!(self
            .inner
            .get_node_type_ids_from_node_type_names(node_type_names.into()))?
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_type_names)"]
    /// Return translated node types from string to internal node ID.
    ///
    /// Parameters
    /// ----------
    /// node_type_names: List[Optional[List[str]]]
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
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<_>>())
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
    /// src: int
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
        let (subresult_0, subresult_1) = self
            .inner
            .get_unchecked_minmax_edge_ids_from_source_node_id(src.into());
        (subresult_0.into(), subresult_1.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, src)"]
    /// Return range of outbound edges IDs which have as source the given Node.
    ///
    /// Parameters
    /// ----------
    /// src: int
    ///     Node for which we need to compute the cumulative_node_degrees range.
    ///
    pub fn get_minmax_edge_ids_from_source_node_id(&self, src: NodeT) -> PyResult<(EdgeT, EdgeT)> {
        Ok({
            let (subresult_0, subresult_1) = pe!(self
                .inner
                .get_minmax_edge_ids_from_source_node_id(src.into()))?
            .into();
            (subresult_0.into(), subresult_1.into())
        })
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
    /// node_type_id: int
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
    /// node_type_ids: List[int]
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
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_type_id)"]
    /// Return number of nodes with the provided node type ID.
    ///
    /// Parameters
    /// ----------
    /// node_type_id: int
    ///     The node type to return the number of nodes of.
    ///
    ///
    /// Safety
    /// ------
    /// The method may panic if an invalid node type (one not present in the graph)
    ///  is provided. If the graph does not have node types, zero will be returned.
    pub unsafe fn get_unchecked_number_of_nodes_from_node_type_id(
        &self,
        node_type_id: NodeTypeT,
    ) -> NodeT {
        self.inner
            .get_unchecked_number_of_nodes_from_node_type_id(node_type_id.into())
            .into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_type_id)"]
    /// Return number of nodes with the provided node type ID.
    ///
    /// Parameters
    /// ----------
    /// node_type_id: int
    ///     The node type to return the number of nodes of.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    /// ValueError
    ///     If the provided node type ID does not exist in the graph.
    ///
    pub fn get_number_of_nodes_from_node_type_id(
        &self,
        node_type_id: NodeTypeT,
    ) -> PyResult<NodeT> {
        Ok(pe!(self
            .inner
            .get_number_of_nodes_from_node_type_id(node_type_id.into()))?
        .into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_type_name)"]
    /// Return number of nodes with the provided node type name.
    ///
    /// Parameters
    /// ----------
    /// node_type_name: str
    ///     The node type to return the number of nodes of.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    /// ValueError
    ///     If the provided node type name does not exist in the graph.
    ///
    pub fn get_number_of_nodes_from_node_type_name(&self, node_type_name: &str) -> PyResult<NodeT> {
        Ok(pe!(self
            .inner
            .get_number_of_nodes_from_node_type_name(node_type_name.into()))?
        .into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_type_id)"]
    /// Return number of edges with the provided edge type ID.
    ///
    /// Parameters
    /// ----------
    /// edge_type_id: int
    ///     The edge type to return the number of edges of.
    ///
    ///
    /// Safety
    /// ------
    /// The method may panic if an invalid edge type (one not present in the graph)
    ///  is provided. If the graph does not have edge types, zero will be returned.
    pub unsafe fn get_unchecked_number_of_edges_from_edge_type_id(
        &self,
        edge_type_id: EdgeTypeT,
    ) -> EdgeT {
        self.inner
            .get_unchecked_number_of_edges_from_edge_type_id(edge_type_id.into())
            .into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_type_id)"]
    /// Return number of edges with the provided edge type ID.
    ///
    /// Parameters
    /// ----------
    /// edge_type_id: int
    ///     The edge type to return the number of edges of.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    /// ValueError
    ///     If the provided edge type ID does not exist in the graph.
    ///
    pub fn get_number_of_edges_from_edge_type_id(
        &self,
        edge_type_id: EdgeTypeT,
    ) -> PyResult<EdgeT> {
        Ok(pe!(self
            .inner
            .get_number_of_edges_from_edge_type_id(edge_type_id.into()))?
        .into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, edge_type_name)"]
    /// Return number of edges with the provided edge type name.
    ///
    /// Parameters
    /// ----------
    /// edge_type_name: str
    ///     The edge type to return the number of edges of.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    /// ValueError
    ///     If the provided edge type name does not exist in the graph.
    ///
    pub fn get_number_of_edges_from_edge_type_name(&self, edge_type_name: &str) -> PyResult<EdgeT> {
        Ok(pe!(self
            .inner
            .get_number_of_edges_from_edge_type_name(edge_type_name.into()))?
        .into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_ids)"]
    /// Returns node type IDs counts hashmap for the provided node IDs.
    ///
    /// Parameters
    /// ----------
    /// node_ids: List[int]
    ///     The node IDs to consider for this count.
    ///
    ///
    /// Safety
    /// ------
    /// Must have node types and the provided node IDs must exit in the graph
    ///  or the result will be undefined and most likely will lead to panic.
    pub unsafe fn get_unchecked_node_type_id_counts_hashmap_from_node_ids(
        &self,
        node_ids: Vec<NodeT>,
    ) -> PyResult<HashMap<NodeTypeT, NodeT>> {
        Ok(pe!(self
            .inner
            .get_unchecked_node_type_id_counts_hashmap_from_node_ids(&node_ids))?
        .into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_ids)"]
    /// Returns edge type IDs counts hashmap for the provided node IDs.
    ///
    /// Parameters
    /// ----------
    /// node_ids: List[int]
    ///     The node IDs to consider for this count.
    ///
    ///
    /// Safety
    /// ------
    /// Must have edge types and the provided node IDs must exit in the graph
    ///  or the result will be undefined and most likely will lead to panic.
    pub unsafe fn get_unchecked_edge_type_id_counts_hashmap_from_node_ids(
        &self,
        node_ids: Vec<NodeT>,
    ) -> PyResult<HashMap<EdgeTypeT, EdgeT>> {
        Ok(pe!(self
            .inner
            .get_unchecked_edge_type_id_counts_hashmap_from_node_ids(&node_ids))?
        .into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, minimum_number_of_nodes_per_tendril, compute_tendril_nodes)"]
    /// Return vector of Tendrils in the current graph instance.
    ///
    /// Parameters
    /// ----------
    ///
    pub fn get_tendrils(
        &self,
        minimum_number_of_nodes_per_tendril: Option<NodeT>,
        compute_tendril_nodes: Option<bool>,
    ) -> PyResult<Vec<Tendril>> {
        Ok(pe!(self.inner.get_tendrils(
            minimum_number_of_nodes_per_tendril.into(),
            compute_tendril_nodes.into()
        ))?
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, number_of_nodes_above_threshold)"]
    /// Return threshold representing cutuoff point in graph node degree geometric distribution to have the given amount of elements above cutoff.
    ///
    /// Parameters
    /// ----------
    /// number_of_elements_above_threshold: int
    ///     Number of elements expected to be above cutoff threshold.
    ///
    pub fn get_node_degree_geometric_distribution_threshold(
        &self,
        number_of_nodes_above_threshold: NodeT,
    ) -> f64 {
        self.inner
            .get_node_degree_geometric_distribution_threshold(
                number_of_nodes_above_threshold.into(),
            )
            .into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return list of the supported sparse edge weighting methods
    pub fn get_sparse_edge_weighting_methods(&self) -> Vec<&str> {
        self.inner
            .get_sparse_edge_weighting_methods()
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return list of the supported edge weighting methods
    pub fn get_edge_weighting_methods(&self) -> Vec<&str> {
        self.inner
            .get_edge_weighting_methods()
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
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
    pub fn get_degree_centrality(&self) -> PyResult<Py<PyArray1<f32>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(gil, pe!(self.inner.get_degree_centrality())?, f32)
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns vector of weighted degree centrality for all nodes
    pub fn get_weighted_degree_centrality(&self) -> PyResult<Py<PyArray1<f64>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(gil, pe!(self.inner.get_weighted_degree_centrality())?, f64)
        })
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
    /// node_id: int
    ///     The node ID whose closeness centrality is to be computed.
    /// verbose: Optional[bool]
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
    /// node_id: int
    ///     The node ID whose closeness centrality is to be computed.
    /// use_edge_weights_as_probabilities: bool
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
    /// verbose: Optional[bool]
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
    /// use_edge_weights_as_probabilities: bool
    ///     Whether to treat the edge weights as probabilities.
    /// verbose: Optional[bool]
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
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_weighted_closeness_centrality(
                    use_edge_weights_as_probabilities.into(),
                    verbose.into()
                ))?,
                f64
            )
        })
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
    /// node_id: int
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
    /// node_id: int
    ///     The node ID whose harmonic centrality is to be computed.
    /// use_edge_weights_as_probabilities: bool
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
    /// verbose: Optional[bool]
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
    /// use_edge_weights_as_probabilities: Optional[bool]
    ///     Whether to treat the edge weights as probabilities.
    /// verbose: Optional[bool]
    ///     Whether to show an indicative progress bar.
    ///
    pub fn get_weighted_harmonic_centrality(
        &self,
        use_edge_weights_as_probabilities: Option<bool>,
        verbose: Option<bool>,
    ) -> PyResult<Py<PyArray1<f64>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_weighted_harmonic_centrality(
                    use_edge_weights_as_probabilities.into(),
                    verbose.into()
                ))?,
                f64
            )
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, normalize, verbose)"]
    /// Returns vector of stress centrality for all nodes.
    ///
    /// Parameters
    /// ----------
    /// normalize: Optional[bool]
    ///     Whether to normalize the values. By default, it is false.
    /// verbose: Optional[bool]
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
    /// normalize: Optional[bool]
    ///     Whether to normalize the values. By default, it is false.
    /// verbose: Optional[bool]
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
    #[text_signature = "($self, node_id, ant, maximum_samples_number, random_state)"]
    /// Returns the unweighted approximated betweenness centrality of the given node id.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     The node ID for which to compute the approximated betweenness centrality.
    /// constant: Optional[float]
    ///     The constant factor to use to regulate the sampling. By default 2.0. It must be greater or equal than 2.0.
    /// maximum_samples_number: Optional[float]
    ///     The maximum number of samples to sample. By default `nodes_number / 20`, as suggested in the paper.
    /// random_state: Optional[int]
    ///     The random state to use for the sampling. By default 42.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the provided node ID does not exist in the current graph instance.
    ///
    pub fn get_approximated_betweenness_centrality_from_node_id(
        &self,
        node_id: NodeT,
        ant: Option<f64>,
        maximum_samples_number: Option<f64>,
        random_state: Option<u64>,
    ) -> PyResult<f64> {
        Ok(pe!(self
            .inner
            .get_approximated_betweenness_centrality_from_node_id(
                node_id.into(),
                ant.into(),
                maximum_samples_number.into(),
                random_state.into()
            ))?
        .into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_name, ant, maximum_samples_number, random_state)"]
    /// Returns the unweighted approximated betweenness centrality of the given node id.
    ///
    /// Parameters
    /// ----------
    /// node_name: str
    ///     The node name for which to compute the approximated betweenness centrality.
    /// constant: Optional[float]
    ///     The constant factor to use to regulate the sampling. By default 2.0. It must be greater or equal than 2.0.
    /// maximum_samples_number: Optional[float]
    ///     The maximum number of samples to sample. By default `nodes_number / 20`, as suggested in the paper.
    /// random_state: Optional[int]
    ///     The random state to use for the sampling. By default 42.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the provided node name does not exist in the current graph instance.
    ///
    pub fn get_approximated_betweenness_centrality_from_node_name(
        &self,
        node_name: &str,
        ant: Option<f64>,
        maximum_samples_number: Option<f64>,
        random_state: Option<u64>,
    ) -> PyResult<f64> {
        Ok(pe!(self
            .inner
            .get_approximated_betweenness_centrality_from_node_name(
                node_name.into(),
                ant.into(),
                maximum_samples_number.into(),
                random_state.into()
            ))?
        .into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_id, ant, use_edge_weights_as_probabilities, maximum_samples_number, random_state)"]
    /// Returns the weighted approximated betweenness centrality of the given node id.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     The node ID for which to compute the approximated betweenness centrality.
    /// constant: Optional[float]
    ///     The constant factor to use to regulate the sampling. By default 2.0. It must be greater or equal than 2.0.
    /// use_edge_weights_as_probabilities: Optional[bool]
    ///     Whether to consider the edge weights as probabilities.
    /// maximum_samples_number: Optional[float]
    ///     The maximum number of samples to sample. By default `nodes_number / 20`, as suggested in the paper.
    /// random_state: Optional[int]
    ///     The random state to use for the sampling. By default 42.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the provided node ID does not exist in the current graph instance.
    ///
    pub fn get_weighted_approximated_betweenness_centrality_from_node_id(
        &self,
        node_id: NodeT,
        ant: Option<f64>,
        use_edge_weights_as_probabilities: Option<bool>,
        maximum_samples_number: Option<f64>,
        random_state: Option<u64>,
    ) -> PyResult<f64> {
        Ok(pe!(self
            .inner
            .get_weighted_approximated_betweenness_centrality_from_node_id(
                node_id.into(),
                ant.into(),
                use_edge_weights_as_probabilities.into(),
                maximum_samples_number.into(),
                random_state.into()
            ))?
        .into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_name, ant, use_edge_weights_as_probabilities, maximum_samples_number, random_state)"]
    /// Returns the weighted approximated betweenness centrality of the given node id.
    ///
    /// Parameters
    /// ----------
    /// node_name: str
    ///     The node name for which to compute the approximated betweenness centrality.
    /// constant: Optional[float]
    ///     The constant factor to use to regulate the sampling. By default 2.0. It must be greater or equal than 2.0.
    /// use_edge_weights_as_probabilities: Optional[bool]
    ///     Whether to consider the edge weights as probabilities.
    /// maximum_samples_number: Optional[float]
    ///     The maximum number of samples to sample. By default `nodes_number / 20`, as suggested in the paper.
    /// random_state: Optional[int]
    ///     The random state to use for the sampling. By default 42.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the provided node name does not exist in the current graph instance.
    ///
    pub fn get_weighted_approximated_betweenness_centrality_from_node_name(
        &self,
        node_name: &str,
        ant: Option<f64>,
        use_edge_weights_as_probabilities: Option<bool>,
        maximum_samples_number: Option<f64>,
        random_state: Option<u64>,
    ) -> PyResult<f64> {
        Ok(pe!(self
            .inner
            .get_weighted_approximated_betweenness_centrality_from_node_name(
                node_name.into(),
                ant.into(),
                use_edge_weights_as_probabilities.into(),
                maximum_samples_number.into(),
                random_state.into()
            ))?
        .into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, maximum_iterations_number, tollerance)"]
    /// Returns vector with unweighted eigenvector centrality.
    ///
    /// Parameters
    /// ----------
    /// maximum_iterations_number: Optional[int]
    ///     The maximum number of iterations to consider.
    /// tollerance: Optional[float]
    ///     The maximum error tollerance for convergence.
    ///
    pub fn get_eigenvector_centrality(
        &self,
        maximum_iterations_number: Option<usize>,
        tollerance: Option<f64>,
    ) -> PyResult<Py<PyArray1<f64>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_eigenvector_centrality(
                    maximum_iterations_number.into(),
                    tollerance.into()
                ))?,
                f64
            )
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, maximum_iterations_number, tollerance)"]
    /// Returns vector with unweighted eigenvector centrality.
    ///
    /// Parameters
    /// ----------
    /// maximum_iterations_number: Optional[int]
    ///     The maximum number of iterations to consider.
    /// tollerance: Optional[float]
    ///     The maximum error tollerance for convergence.
    ///
    pub fn get_weighted_eigenvector_centrality(
        &self,
        maximum_iterations_number: Option<usize>,
        tollerance: Option<f64>,
    ) -> PyResult<Py<PyArray1<f64>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_weighted_eigenvector_centrality(
                    maximum_iterations_number.into(),
                    tollerance.into()
                ))?,
                f64
            )
        })
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Print the current graph in a format compatible with Graphviz dot's format
    pub fn to_dot(&self) -> String {
        self.inner.to_dot().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, minimum_number_of_nodes_per_star)"]
    /// Return vector of Stars in the current graph instance.
    ///
    /// Parameters
    /// ----------
    ///
    pub fn get_stars(
        &self,
        minimum_number_of_nodes_per_star: Option<NodeT>,
    ) -> PyResult<Vec<Star>> {
        Ok(pe!(self
            .inner
            .get_stars(minimum_number_of_nodes_per_star.into()))?
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, recursion_minimum_improvement, first_phase_minimum_improvement, patience, random_state)"]
    /// Returns vector of vectors of communities for each layer of hierarchy minimizing undirected modularity.
    ///
    /// Parameters
    /// ----------
    /// recursion_minimum_improvement: Optional[float]
    ///     The minimum improvement to warrant another resursion round. By default, zero.
    /// first_phase_minimum_improvement: Optional[float]
    ///     The minimum improvement to warrant another first phase iteration. By default, `0.00001` (not zero because of numerical instability).
    /// patience: Optional[int]
    ///     How many iterations of the first phase to wait for before stopping. By default, `5`.
    /// random_state: Optional[int]
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
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<_>>())
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
    /// source_node_id: int
    ///     Node ID of the first node.
    /// destination_node_id: int
    ///     Node ID of the second node.
    /// normalize: bool
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
    /// source_node_id: int
    ///     Node ID of the first node.
    /// destination_node_id: int
    ///     Node ID of the second node.
    /// normalize: bool
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
    /// first_node_name: str
    ///     Node name of the first node.
    /// second_node_name: str
    ///     Node name of the second node.
    /// normalize: bool
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
    /// source_node_id: int
    ///     Node ID of the first node.
    /// destination_node_id: int
    ///     Node ID of the second node.
    /// normalize: bool
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
    /// source_node_id: int
    ///     Node ID of the first node.
    /// destination_node_id: int
    ///     Node ID of the second node.
    /// normalize: bool
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
    /// first_node_name: str
    ///     Node name of the first node.
    /// second_node_name: str
    ///     Node name of the second node.
    /// normalize: bool
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
    /// source_node_id: int
    ///     Node ID of the first node.
    /// destination_node_id: int
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
    /// source_node_id: int
    ///     Node ID of the first node.
    /// destination_node_id: int
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
    /// first_node_name: str
    ///     Node name of the first node.
    /// second_node_name: str
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
    /// source_node_id: int
    ///     Node ID of the first node.
    /// destination_node_id: int
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
    /// source_node_id: int
    ///     Node ID of the first node.
    /// destination_node_id: int
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
    /// first_node_name: str
    ///     Node name of the first node.
    /// second_node_name: str
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
    /// source_node_id: int
    ///     Node ID of the first node.
    /// destination_node_id: int
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
    /// source_node_id: int
    ///     Node ID of the first node.
    /// destination_node_id: int
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
    /// source_node_id: int
    ///     Node ID of the first node.
    /// destination_node_id: int
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
    /// first_node_name: str
    ///     Node name of the first node.
    /// second_node_name: str
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
    /// source_node_id: int
    ///     Node ID of the first node.
    /// destination_node_id: int
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
    /// first_node_name: str
    ///     Node name of the first node.
    /// second_node_name: str
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
    /// source_node_id: int
    ///     Node ID of the first node.
    /// destination_node_id: int
    ///     Node ID of the second node.
    /// normalize: bool
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
    #[text_signature = "($self, minimum_node_degree)"]
    /// Returns vector with isomorphic node groups IDs.
    ///
    /// Parameters
    /// ----------
    /// minimum_node_degree: Optional[int]
    ///     Minimum node degree for the topological synonims. By default, 5.
    ///
    pub fn get_isomorphic_node_ids_groups(
        &self,
        minimum_node_degree: Option<NodeT>,
    ) -> Vec<Vec<NodeT>> {
        self.inner
            .get_isomorphic_node_ids_groups(minimum_node_degree.into())
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, minimum_node_degree)"]
    /// Returns vector with isomorphic node groups names.
    ///
    /// Parameters
    /// ----------
    /// minimum_node_degree: Optional[int]
    ///     Minimum node degree for the topological synonims. By default, 5.
    ///
    pub fn get_isomorphic_node_names_groups(
        &self,
        minimum_node_degree: Option<NodeT>,
    ) -> Vec<Vec<String>> {
        self.inner
            .get_isomorphic_node_names_groups(minimum_node_degree.into())
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, minimum_node_degree)"]
    /// Returns number of isomorphic node groups.
    ///
    /// Parameters
    /// ----------
    /// minimum_node_degree: Optional[int]
    ///     Minimum node degree for the topological synonims. By default, 5.
    ///
    pub fn get_isomorphic_node_groups_number(&self, minimum_node_degree: Option<NodeT>) -> NodeT {
        self.inner
            .get_isomorphic_node_groups_number(minimum_node_degree.into())
            .into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns vector with isomorphic node type groups IDs
    pub fn get_isomorphic_node_type_ids_groups(&self) -> PyResult<Vec<Vec<NodeTypeT>>> {
        Ok(pe!(self.inner.get_isomorphic_node_type_ids_groups())?
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns vector with isomorphic node type groups names
    pub fn get_isomorphic_node_type_names_groups(&self) -> PyResult<Vec<Vec<String>>> {
        Ok(pe!(self.inner.get_isomorphic_node_type_names_groups())?
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns number of isomorphic node type groups
    pub fn get_isomorphic_node_type_groups_number(&self) -> PyResult<NodeTypeT> {
        Ok(pe!(self.inner.get_isomorphic_node_type_groups_number())?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns vector with isomorphic edge type groups IDs
    pub fn get_isomorphic_edge_type_ids_groups(&self) -> PyResult<Vec<Vec<EdgeTypeT>>> {
        Ok(pe!(self.inner.get_isomorphic_edge_type_ids_groups())?
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns vector with isomorphic edge type groups names
    pub fn get_isomorphic_edge_type_names_groups(&self) -> PyResult<Vec<Vec<String>>> {
        Ok(pe!(self.inner.get_isomorphic_edge_type_names_groups())?
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Returns number of isomorphic edge type groups
    pub fn get_isomorphic_edge_type_groups_number(&self) -> PyResult<EdgeTypeT> {
        Ok(pe!(self.inner.get_isomorphic_edge_type_groups_number())?.into())
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, minimum_node_degree)"]
    /// Returns whether the current graph has topological synonims.
    ///
    /// Parameters
    /// ----------
    /// minimum_node_degree: Optional[int]
    ///     Minimum node degree for the topological synonims.
    ///
    pub fn has_isomorphic_nodes(&self, minimum_node_degree: Option<NodeT>) -> bool {
        self.inner
            .has_isomorphic_nodes(minimum_node_degree.into())
            .into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_ids)"]
    /// Returns whether the set of provided node IDs have isomorphic node types.
    ///
    /// Parameters
    /// ----------
    /// node_ids: List[int]
    ///     Node IDs to check for.
    ///
    pub unsafe fn has_unchecked_isomorphic_node_types_from_node_ids(
        &self,
        node_ids: Vec<NodeT>,
    ) -> bool {
        self.inner
            .has_unchecked_isomorphic_node_types_from_node_ids(&node_ids)
            .into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, node_ids)"]
    /// Returns whether the set of provided node IDs have isomorphic node types.
    ///
    /// Parameters
    /// ----------
    /// node_ids: List[int]
    ///     Node IDs to check for.
    ///
    pub fn has_isomorphic_node_types_from_node_ids(&self, node_ids: Vec<NodeT>) -> PyResult<bool> {
        Ok(pe!(self
            .inner
            .has_isomorphic_node_types_from_node_ids(&node_ids))?
        .into())
    }

    #[staticmethod]
    #[automatically_generated_binding]
    #[text_signature = "(node_type_path, node_type_list_separator, node_types_column_number, node_types_column, node_types_ids_column_number, node_types_ids_column, node_types_number, numeric_node_type_ids, minimum_node_type_id, node_type_list_header, node_type_list_support_balanced_quotes, node_type_list_rows_to_skip, node_type_list_is_correct, node_type_list_max_rows_number, node_type_list_comment_symbol, load_node_type_list_in_parallel, node_path, node_list_separator, node_list_header, node_list_support_balanced_quotes, node_list_rows_to_skip, node_list_is_correct, node_list_max_rows_number, node_list_comment_symbol, default_node_type, nodes_column_number, nodes_column, node_types_separator, node_list_node_types_column_number, node_list_node_types_column, node_ids_column, node_ids_column_number, nodes_number, minimum_node_id, numeric_node_ids, node_list_numeric_node_type_ids, skip_node_types_if_unavailable, load_node_list_in_parallel, edge_type_path, edge_types_column_number, edge_types_column, edge_types_ids_column_number, edge_types_ids_column, edge_types_number, numeric_edge_type_ids, minimum_edge_type_id, edge_type_list_separator, edge_type_list_header, edge_type_list_support_balanced_quotes, edge_type_list_rows_to_skip, edge_type_list_is_correct, edge_type_list_max_rows_number, edge_type_list_comment_symbol, load_edge_type_list_in_parallel, edge_path, edge_list_separator, edge_list_header, edge_list_support_balanced_quotes, edge_list_rows_to_skip, sources_column_number, sources_column, destinations_column_number, destinations_column, edge_list_edge_types_column_number, edge_list_edge_types_column, default_edge_type, weights_column_number, weights_column, default_weight, edge_ids_column, edge_ids_column_number, edge_list_numeric_edge_type_ids, edge_list_numeric_node_ids, skip_weights_if_unavailable, skip_edge_types_if_unavailable, edge_list_is_complete, edge_list_may_contain_duplicates, edge_list_is_sorted, edge_list_is_correct, edge_list_max_rows_number, edge_list_comment_symbol, edges_number, load_edge_list_in_parallel, verbose, may_have_singletons, may_have_singleton_with_selfloops, directed, name)"]
    /// Return graph renderized from given CSVs or TSVs-like files.
    ///
    /// Parameters
    /// ----------
    /// node_type_path: Optional[str]
    ///     The path to the file with the unique node type names.
    /// node_type_list_separator: Optional[str]
    ///     The separator to use for the node types file. Note that if this is not provided, one will be automatically detected among the following`: comma, semi-column, tab and space.
    /// node_types_column_number: Optional[int]
    ///     The number of the column of the node types file from where to load the node types.
    /// node_types_column: Optional[str]
    ///     The name of the column of the node types file from where to load the node types.
    /// node_types_number: Optional[int]
    ///     The number of the unique node types. This will be used in order to allocate the correct size for the data structure.
    /// numeric_node_type_ids: Optional[bool]
    ///     Whether the node type names should be loaded as numeric values, i.e. casted from string to a numeric representation.
    /// minimum_node_type_id: Optional[int]
    ///     The minimum node type ID to be used when using numeric node type IDs.
    /// node_type_list_header: Optional[bool]
    ///     Whether the node type file has an header.
    /// node_type_list_support_balanced_quotes: Optional[bool]
    ///     Whether to support balanced quotes.
    /// node_type_list_rows_to_skip: Optional[int]
    ///     The number of lines to skip in the node types file`: the header is already skipped if it has been specified that the file has an header.
    /// node_type_list_is_correct: Optional[bool]
    ///     Whether the node types file can be assumed to be correct, i.e. does not have something wrong in it. If this parameter is passed as true on a malformed file, the constructor will crash.
    /// node_type_list_max_rows_number: Optional[int]
    ///     The maximum number of lines to be loaded from the node types file.
    /// node_type_list_comment_symbol: Optional[str]
    ///     The comment symbol to skip lines in the node types file. Lines starting with this symbol will be skipped.
    /// load_node_type_list_in_parallel: Optional[bool]
    ///     Whether to load the node type list in parallel. Note that when loading in parallel, the internal order of the node type IDs may result changed across different iterations. We are working to get this to be stable.
    /// node_path: Optional[str]
    ///     The path to the file with the unique node names.
    /// node_list_separator: Optional[str]
    ///     The separator to use for the nodes file. Note that if this is not provided, one will be automatically detected among the following`: comma, semi-column, tab and space.
    /// node_list_header: Optional[bool]
    ///     Whether the nodes file has an header.
    /// node_list_support_balanced_quotes: Optional[bool]
    ///     Whether to support balanced quotes.
    /// node_list_rows_to_skip: Optional[int]
    ///     Number of rows to skip in the node list file.
    /// node_list_is_correct: Optional[bool]
    ///     Whether the nodes file can be assumed to be correct, i.e. does not have something wrong in it. If this parameter is passed as true on a malformed file, the constructor will crash.
    /// node_list_max_rows_number: Optional[int]
    ///     The maximum number of lines to be loaded from the nodes file.
    /// node_list_comment_symbol: Optional[str]
    ///     The comment symbol to skip lines in the nodes file. Lines starting with this symbol will be skipped.
    /// default_node_type: Optional[str]
    ///     The node type to be used when the node type for a given node in the node file is None.
    /// nodes_column_number: Optional[int]
    ///     The number of the column of the node file from where to load the node names.
    /// nodes_column: Optional[str]
    ///     The name of the column of the node file from where to load the node names.
    /// node_types_separator: Optional[str]
    ///     The node types separator.
    /// node_list_node_types_column_number: Optional[int]
    ///     The number of the column of the node file from where to load the node types.
    /// node_list_node_types_column: Optional[str]
    ///     The name of the column of the node file from where to load the node types.
    /// node_ids_column: Optional[str]
    ///     The name of the column of the node file from where to load the node IDs.
    /// node_ids_column_number: Optional[int]
    ///     The number of the column of the node file from where to load the node IDs
    /// nodes_number: Optional[int]
    ///     The expected number of nodes. Note that this must be the EXACT number of nodes in the graph.
    /// minimum_node_id: Optional[int]
    ///     The minimum node ID to be used, when loading the node IDs as numerical.
    /// numeric_node_ids: Optional[bool]
    ///     Whether to load the numeric node IDs as numeric.
    /// node_list_numeric_node_type_ids: Optional[bool]
    ///     Whether to load the node types IDs in the node file to be numeric.
    /// skip_node_types_if_unavailable: Optional[bool]
    ///     Whether to skip the node types without raising an error if these are unavailable.
    /// load_node_list_in_parallel: Optional[bool]
    ///     Whether to load the node list in parallel. When loading in parallel, without node IDs, the nodes may not be loaded in a deterministic order.
    /// edge_type_path: Optional[str]
    ///     The path to the file with the unique edge type names.
    /// edge_types_column_number: Optional[int]
    ///     The number of the column of the edge types file from where to load the edge types.
    /// edge_types_column: Optional[str]
    ///     The name of the column of the edge types file from where to load the edge types.
    /// edge_types_number: Optional[int]
    ///     The number of the unique edge types. This will be used in order to allocate the correct size for the data structure.
    /// numeric_edge_type_ids: Optional[bool]
    ///     Whether the edge type names should be loaded as numeric values, i.e. casted from string to a numeric representation.
    /// minimum_edge_type_id: Optional[int]
    ///     The minimum edge type ID to be used when using numeric edge type IDs.
    /// edge_type_list_separator: Optional[str]
    ///     The separator to use for the edge type list. Note that, if None is provided, one will be attempted to be detected automatically between ';', ',', tab or space.
    /// edge_type_list_header: Optional[bool]
    ///     Whether the edge type file has an header.
    /// edge_type_list_support_balanced_quotes: Optional[bool]
    ///     Whether to support balanced quotes while reading the edge type list.
    /// edge_type_list_rows_to_skip: Optional[int]
    ///     Number of rows to skip in the edge type list file.
    /// edge_type_list_is_correct: Optional[bool]
    ///     Whether the edge types file can be assumed to be correct, i.e. does not have something wrong in it. If this parameter is passed as true on a malformed file, the constructor will crash.
    /// edge_type_list_max_rows_number: Optional[int]
    ///     The maximum number of lines to be loaded from the edge types file.
    /// edge_type_list_comment_symbol: Optional[str]
    ///     The comment symbol to skip lines in the edge types file. Lines starting with this symbol will be skipped.
    /// load_edge_type_list_in_parallel: Optional[bool]
    ///     Whether to load the edge type list in parallel. When loading in parallel, without edge type IDs, the edge types may not be loaded in a deterministic order.
    /// edge_path: Optional[str]
    ///     The path to the file with the edge list.
    /// edge_list_separator: Optional[str]
    ///     The separator to use for the edge list. Note that, if None is provided, one will be attempted to be detected automatically between ';', ',', tab or space.
    /// edge_list_header: Optional[bool]
    ///     Whether the edges file has an header.
    /// edge_list_support_balanced_quotes: Optional[bool]
    ///     Whether to support balanced quotes while reading the edge list.
    /// edge_list_rows_to_skip: Optional[int]
    ///     Number of rows to skip in the edge list file.
    /// sources_column_number: Optional[int]
    ///     The number of the column of the edges file from where to load the source nodes.
    /// sources_column: Optional[str]
    ///     The name of the column of the edges file from where to load the source nodes.
    /// destinations_column_number: Optional[int]
    ///     The number of the column of the edges file from where to load the destinaton nodes.
    /// destinations_column: Optional[str]
    ///     The name of the column of the edges file from where to load the destinaton nodes.
    /// edge_list_edge_types_column_number: Optional[int]
    ///     The number of the column of the edges file from where to load the edge types.
    /// edge_list_edge_types_column: Optional[str]
    ///     The name of the column of the edges file from where to load the edge types.
    /// default_edge_type: Optional[str]
    ///     The edge type to be used when the edge type for a given edge in the edge file is None.
    /// weights_column_number: Optional[int]
    ///     The number of the column of the edges file from where to load the edge weights.
    /// weights_column: Optional[str]
    ///     The name of the column of the edges file from where to load the edge weights.
    /// default_weight: Optional[float]
    ///     The edge weight to be used when the edge weight for a given edge in the edge file is None.
    /// edge_ids_column: Optional[str]
    ///     The name of the column of the edges file from where to load the edge IDs.
    /// edge_ids_column_number: Optional[int]
    ///     The number of the column of the edges file from where to load the edge IDs.
    /// edge_list_numeric_edge_type_ids: Optional[bool]
    ///     Whether to load the edge type IDs as numeric from the edge list.
    /// edge_list_numeric_node_ids: Optional[bool]
    ///     Whether to load the edge node IDs as numeric from the edge list.
    /// skip_weights_if_unavailable: Optional[bool]
    ///     Whether to skip the weights without raising an error if these are unavailable.
    /// skip_edge_types_if_unavailable: Optional[bool]
    ///     Whether to skip the edge types without raising an error if these are unavailable.
    /// edge_list_is_complete: Optional[bool]
    ///     Whether to consider the edge list as complete, i.e. the edges are presented in both directions when loading an undirected graph.
    /// edge_list_may_contain_duplicates: Optional[bool]
    ///     Whether the edge list may contain duplicates. If the edge list surely DOES NOT contain duplicates, a validation step may be skipped. By default, it is assumed that the edge list may contain duplicates.
    /// edge_list_is_sorted: Optional[bool]
    ///     Whether the edge list is sorted. Note that a sorted edge list has the minimal memory peak, but requires the nodes number and the edges number.
    /// edge_list_is_correct: Optional[bool]
    ///     Whether the edges file can be assumed to be correct, i.e. does not have something wrong in it. If this parameter is passed as true on a malformed file, the constructor will crash.
    /// edge_list_max_rows_number: Optional[int]
    ///     The maximum number of lines to be loaded from the edges file.
    /// edge_list_comment_symbol: Optional[str]
    ///     The comment symbol to skip lines in the edges file. Lines starting with this symbol will be skipped.
    /// edges_number: Optional[int]
    ///     The expected number of edges. Note that this must be the EXACT number of edges in the graph.
    /// load_edge_list_in_parallel: Optional[bool]
    ///     Whether to load the edge list in parallel. Note that, if the edge IDs indices are not given, it is NOT possible to load a sorted edge list. Similarly, when loading in parallel, without edge IDs, the edges may not be loaded in a deterministic order.
    /// verbose: Optional[bool]
    ///     Whether to show a loading bar while reading the files. Note that, if parallel loading is enabled, loading bars will not be showed because they are a synchronization bottleneck.
    /// may_have_singletons: Optional[bool]
    ///     Whether the graph may be expected to have singleton nodes. If it is said that it surely DOES NOT have any, it will allow for some speedups and lower mempry peaks.
    /// may_have_singleton_with_selfloops: Optional[bool]
    ///     Whether the graph may be expected to have singleton nodes with selfloops. If it is said that it surely DOES NOT have any, it will allow for some speedups and lower mempry peaks.
    /// directed: bool
    ///     Whether to load the graph as directed or undirected.
    /// name: Optional[str]
    ///     The name of the graph to be loaded.
    ///
    pub fn from_csv(
        node_type_path: Option<String>,
        node_type_list_separator: Option<char>,
        node_types_column_number: Option<usize>,
        node_types_column: Option<String>,
        node_types_ids_column_number: Option<usize>,
        node_types_ids_column: Option<String>,
        node_types_number: Option<NodeTypeT>,
        numeric_node_type_ids: Option<bool>,
        minimum_node_type_id: Option<NodeTypeT>,
        node_type_list_header: Option<bool>,
        node_type_list_support_balanced_quotes: Option<bool>,
        node_type_list_rows_to_skip: Option<usize>,
        node_type_list_is_correct: Option<bool>,
        node_type_list_max_rows_number: Option<usize>,
        node_type_list_comment_symbol: Option<String>,
        load_node_type_list_in_parallel: Option<bool>,
        node_path: Option<String>,
        node_list_separator: Option<char>,
        node_list_header: Option<bool>,
        node_list_support_balanced_quotes: Option<bool>,
        node_list_rows_to_skip: Option<usize>,
        node_list_is_correct: Option<bool>,
        node_list_max_rows_number: Option<usize>,
        node_list_comment_symbol: Option<String>,
        default_node_type: Option<String>,
        nodes_column_number: Option<usize>,
        nodes_column: Option<String>,
        node_types_separator: Option<char>,
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
        edge_type_list_separator: Option<char>,
        edge_type_list_header: Option<bool>,
        edge_type_list_support_balanced_quotes: Option<bool>,
        edge_type_list_rows_to_skip: Option<usize>,
        edge_type_list_is_correct: Option<bool>,
        edge_type_list_max_rows_number: Option<usize>,
        edge_type_list_comment_symbol: Option<String>,
        load_edge_type_list_in_parallel: Option<bool>,
        edge_path: Option<String>,
        edge_list_separator: Option<char>,
        edge_list_header: Option<bool>,
        edge_list_support_balanced_quotes: Option<bool>,
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
            node_type_list_support_balanced_quotes.into(),
            node_type_list_rows_to_skip.into(),
            node_type_list_is_correct.into(),
            node_type_list_max_rows_number.into(),
            node_type_list_comment_symbol.into(),
            load_node_type_list_in_parallel.into(),
            node_path.into(),
            node_list_separator.into(),
            node_list_header.into(),
            node_list_support_balanced_quotes.into(),
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
            edge_type_list_support_balanced_quotes.into(),
            edge_type_list_rows_to_skip.into(),
            edge_type_list_is_correct.into(),
            edge_type_list_max_rows_number.into(),
            edge_type_list_comment_symbol.into(),
            load_edge_type_list_in_parallel.into(),
            edge_path.into(),
            edge_list_separator.into(),
            edge_list_header.into(),
            edge_list_support_balanced_quotes.into(),
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
    "get_laplacian_coo_matrix_edges_number",
    "get_random_walk_normalized_laplacian_transformed_graph",
    "get_symmetric_normalized_laplacian_transformed_graph",
    "get_symmetric_normalized_transformed_graph",
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
    "must_not_have_trap_nodes",
    "must_be_multigraph",
    "must_not_be_multigraph",
    "must_contain_identity_matrix",
    "must_not_contain_weighted_singleton_nodes",
    "must_have_edges",
    "must_have_nodes",
    "must_be_connected",
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
    "set_inplace_all_edge_types",
    "set_all_edge_types",
    "set_inplace_all_node_types",
    "set_all_node_types",
    "remove_inplace_node_type_ids",
    "remove_inplace_singleton_node_types",
    "remove_inplace_homogeneous_node_types",
    "remove_inplace_edge_type_ids",
    "remove_inplace_singleton_edge_types",
    "remove_inplace_node_type_name",
    "remove_node_type_id",
    "remove_singleton_node_types",
    "remove_homogeneous_node_types",
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
    "divide_edge_weights_inplace",
    "divide_edge_weights",
    "multiply_edge_weights_inplace",
    "multiply_edge_weights",
    "get_circles",
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
    "get_clustering_coefficient_per_node",
    "get_clustering_coefficient",
    "get_average_clustering_coefficient",
    "are_nodes_remappable",
    "remap_unchecked_from_node_ids",
    "remap_from_node_ids",
    "remap_from_node_names",
    "remap_from_node_names_map",
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
    "get_node_tuples",
    "get_chains",
    "get_unchecked_breadth_first_search_predecessors_parallel_from_node_id",
    "get_unchecked_breadth_first_search_distances_parallel_from_node_ids",
    "get_unchecked_breadth_first_search_distances_parallel_from_node_id",
    "get_unchecked_breadth_first_search_distances_sequential_from_node_id",
    "get_unchecked_breadth_first_search_from_node_ids",
    "get_unchecked_breadth_first_search_from_node_id",
    "get_unchecked_shortest_path_node_ids_from_node_ids",
    "get_unchecked_shortest_path_node_names_from_node_ids",
    "get_shortest_path_node_ids_from_node_ids",
    "get_shortest_path_node_ids_from_node_names",
    "get_shortest_path_node_names_from_node_names",
    "get_unchecked_k_shortest_path_node_ids_from_node_ids",
    "get_k_shortest_path_node_ids_from_node_ids",
    "get_k_shortest_path_node_ids_from_node_names",
    "get_k_shortest_path_node_names_from_node_names",
    "get_unchecked_eccentricity_and_most_distant_node_id_from_node_id",
    "get_unchecked_weighted_eccentricity_from_node_id",
    "get_eccentricity_and_most_distant_node_id_from_node_id",
    "get_weighted_eccentricity_from_node_id",
    "get_eccentricity_from_node_name",
    "get_weighted_eccentricity_from_node_name",
    "get_unchecked_dijkstra_from_node_ids",
    "get_unchecked_dijkstra_from_node_id",
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
    "get_homogeneous_node_types_number",
    "get_homogeneous_node_type_ids",
    "get_homogeneous_node_type_names",
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
    "get_node_ids_from_node_type_id",
    "get_node_names_from_node_type_id",
    "get_node_ids_from_node_type_name",
    "get_node_names_from_node_type_name",
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
    "get_reciprocal_sqrt_degrees",
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
    "get_approximated_cliques",
    "get_approximated_cliques_number",
    "report",
    "overlap_textual_report",
    "get_node_report_from_node_id",
    "get_node_report_from_node_name",
    "textual_report",
    "generate_random_connected_graph",
    "generate_random_spanning_tree",
    "generate_star_graph",
    "generate_wheel_graph",
    "generate_circle_graph",
    "generate_chain_graph",
    "generate_complete_graph",
    "generate_barbell_graph",
    "generate_lollipop_graph",
    "generate_squared_lattice_graph",
    "filter_from_ids",
    "filter_from_names",
    "drop_unknown_node_types",
    "drop_unknown_edge_types",
    "drop_singleton_nodes",
    "drop_tendrils",
    "drop_dendritic_trees",
    "drop_isomorphic_nodes",
    "drop_singleton_nodes_with_selfloops",
    "drop_disconnected_nodes",
    "drop_selfloops",
    "drop_parallel_edges",
    "random_spanning_arborescence_kruskal",
    "spanning_arborescence_kruskal",
    "get_connected_components",
    "enable",
    "is_compatible",
    "has_same_adjacency_matrix",
    "approximated_vertex_cover_set",
    "get_random_node",
    "get_random_nodes",
    "get_breadth_first_search_random_nodes",
    "get_uniform_random_walk_random_nodes",
    "get_node_sampling_methods",
    "get_subsampled_nodes",
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
    "get_dendritic_trees",
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
    "get_unchecked_reciprocal_sqrt_degree_from_node_id",
    "get_reciprocal_sqrt_degree_from_node_id",
    "get_unchecked_reciprocal_sqrt_degrees_from_node_ids",
    "get_weighted_node_degree_from_node_id",
    "get_node_degree_from_node_name",
    "get_top_k_central_node_names",
    "get_unchecked_node_type_ids_from_node_id",
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
    "get_unchecked_number_of_nodes_from_node_type_id",
    "get_number_of_nodes_from_node_type_id",
    "get_number_of_nodes_from_node_type_name",
    "get_unchecked_number_of_edges_from_edge_type_id",
    "get_number_of_edges_from_edge_type_id",
    "get_number_of_edges_from_edge_type_name",
    "get_unchecked_node_type_id_counts_hashmap_from_node_ids",
    "get_unchecked_edge_type_id_counts_hashmap_from_node_ids",
    "get_tendrils",
    "get_node_degree_geometric_distribution_threshold",
    "get_sparse_edge_weighting_methods",
    "get_edge_weighting_methods",
    "add_selfloops",
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
    "get_approximated_betweenness_centrality_from_node_id",
    "get_approximated_betweenness_centrality_from_node_name",
    "get_weighted_approximated_betweenness_centrality_from_node_id",
    "get_weighted_approximated_betweenness_centrality_from_node_name",
    "get_eigenvector_centrality",
    "get_weighted_eigenvector_centrality",
    "to_dot",
    "get_stars",
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
    "get_isomorphic_node_ids_groups",
    "get_isomorphic_node_names_groups",
    "get_isomorphic_node_groups_number",
    "get_isomorphic_node_type_ids_groups",
    "get_isomorphic_node_type_names_groups",
    "get_isomorphic_node_type_groups_number",
    "get_isomorphic_edge_type_ids_groups",
    "get_isomorphic_edge_type_names_groups",
    "get_isomorphic_edge_type_groups_number",
    "has_isomorphic_nodes",
    "has_unchecked_isomorphic_node_types_from_node_ids",
    "has_isomorphic_node_types_from_node_ids",
    "from_csv",
];

pub const GRAPH_TERMS: &[&str] = &[
    "get",
    "laplacian",
    "transformed",
    "graph",
    "coo",
    "matrix",
    "edges",
    "number",
    "random",
    "walk",
    "normalized",
    "symmetric",
    "is",
    "unchecked",
    "connected",
    "from",
    "node",
    "id",
    "disconnected",
    "singleton",
    "with",
    "selfloops",
    "name",
    "has",
    "type",
    "edge",
    "ids",
    "selfloop",
    "and",
    "trap",
    "names",
    "strongly",
    "components",
    "sort",
    "by",
    "increasing",
    "outbound",
    "degree",
    "decreasing",
    "lexicographic",
    "order",
    "bfs",
    "topological",
    "sorting",
    "reversed",
    "dense",
    "binary",
    "adjacency",
    "weighted",
    "remove",
    "overlaps",
    "contains",
    "bipartite",
    "star",
    "clique",
    "encode",
    "decode",
    "max",
    "encodable",
    "validate",
    "must",
    "not",
    "contain",
    "unknown",
    "types",
    "be",
    "undirected",
    "have",
    "nodes",
    "multigraph",
    "identity",
    "total",
    "weights",
    "mininum",
    "weight",
    "maximum",
    "minimum",
    "unique",
    "generate",
    "new",
    "features",
    "set",
    "inplace",
    "all",
    "homogeneous",
    "divide",
    "multiply",
    "circles",
    "memory",
    "stats",
    "used",
    "requirement",
    "human",
    "readable",
    "requirements",
    "of",
    "triangles",
    "triads",
    "transitivity",
    "per",
    "clustering",
    "coefficient",
    "average",
    "are",
    "remappable",
    "remap",
    "map",
    "sample",
    "negatives",
    "holdout",
    "label",
    "indices",
    "labels",
    "graphs",
    "subgraph",
    "kfold",
    "prediction",
    "tuples",
    "chains",
    "breadth",
    "first",
    "search",
    "predecessors",
    "parallel",
    "distances",
    "sequential",
    "shortest",
    "path",
    "k",
    "eccentricity",
    "most",
    "distant",
    "dijkstra",
    "diameter",
    "naive",
    "density",
    "rate",
    "degrees",
    "mean",
    "median",
    "central",
    "mode",
    "source",
    "directed",
    "destination",
    "urls",
    "ontologies",
    "indegrees",
    "known",
    "mask",
    "one",
    "hot",
    "encoded",
    "mapping",
    "multilabel",
    "count",
    "component",
    "singletons",
    "cumulative",
    "reciprocal",
    "sqrt",
    "counts",
    "hashmap",
    "to",
    "upper",
    "triangular",
    "lower",
    "main",
    "diagonal",
    "anti",
    "bidiagonal",
    "arrowhead",
    "transposed",
    "complementary",
    "approximated",
    "cliques",
    "report",
    "overlap",
    "textual",
    "spanning",
    "tree",
    "wheel",
    "circle",
    "chain",
    "complete",
    "barbell",
    "lollipop",
    "squared",
    "lattice",
    "filter",
    "drop",
    "tendrils",
    "dendritic",
    "trees",
    "isomorphic",
    "arborescence",
    "kruskal",
    "enable",
    "compatible",
    "same",
    "vertex",
    "cover",
    "uniform",
    "sampling",
    "methods",
    "subsampled",
    "okapi",
    "bm25",
    "feature",
    "propagation",
    "default",
    "representing",
    "probabilities",
    "constant",
    "negative",
    "oddities",
    "sorted",
    "transitive",
    "closure",
    "paths",
    "minmax",
    "top",
    "comulative",
    "neighbour",
    "multiple",
    "geometric",
    "distribution",
    "threshold",
    "sparse",
    "weighting",
    "add",
    "centrality",
    "closeness",
    "harmonic",
    "stress",
    "betweenness",
    "eigenvector",
    "dot",
    "stars",
    "louvain",
    "community",
    "detection",
    "modularity",
    "memberships",
    "preferential",
    "attachment",
    "jaccard",
    "adamic",
    "adar",
    "index",
    "resource",
    "allocation",
    "metrics",
    "groups",
    "csv",
];

pub const GRAPH_TFIDF_FREQUENCIES: &[&[(&str, f64)]] = &[
    &[
        ("laplacian", 2.0381856),
        ("transformed", 2.0381856),
        ("get", 0.15829183),
        ("graph", 1.5126685),
    ],
    &[
        ("coo", 1.3567443),
        ("edges", 0.75493956),
        ("get", 0.08573313),
        ("matrix", 1.0192826),
        ("laplacian", 1.1039106),
        ("number", 0.5664492),
    ],
    &[
        ("random", 0.6747377),
        ("normalized", 0.9022622),
        ("transformed", 0.8573434),
        ("get", 0.066583954),
        ("graph", 0.63628966),
        ("laplacian", 0.8573434),
        ("walk", 0.96240187),
    ],
    &[
        ("graph", 0.81928295),
        ("normalized", 1.1617477),
        ("transformed", 1.1039106),
        ("get", 0.08573313),
        ("symmetric", 1.2391833),
        ("laplacian", 1.1039106),
    ],
    &[
        ("symmetric", 1.6492976),
        ("get", 0.11410697),
        ("graph", 1.0904291),
        ("transformed", 1.4692557),
        ("normalized", 1.5462344),
    ],
    &[
        ("connected", 0.908914),
        ("from", 0.25178045),
        ("is", 0.83463126),
        ("node", 0.1440086),
        ("id", 0.3757346),
        ("unchecked", 0.45797387),
    ],
    &[
        ("id", 0.29181132),
        ("node", 0.20876107),
        ("is", 0.64820975),
        ("unchecked", 0.35568178),
        ("from", 0.19554329),
        ("disconnected", 0.8573434),
    ],
    &[
        ("unchecked", 0.45797387),
        ("singleton", 0.65608037),
        ("is", 0.83463126),
        ("from", 0.25178045),
        ("id", 0.3757346),
        ("node", 0.1440086),
    ],
    &[
        ("node", 0.19166903),
        ("is", 1.110857),
        ("from", 0.33510855),
        ("singleton", 0.8732137),
        ("id", 0.500086),
    ],
    &[
        ("node", 0.08921785),
        ("singleton", 0.40646234),
        ("unchecked", 0.28372917),
        ("selfloops", 0.5382417),
        ("is", 0.5170802),
        ("from", 0.15598588),
        ("id", 0.23277935),
        ("with", 0.46091795),
    ],
    &[
        ("node", 0.11184315),
        ("from", 0.19554329),
        ("is", 0.64820975),
        ("singleton", 0.5095396),
        ("id", 0.29181132),
        ("selfloops", 0.6747377),
        ("with", 0.577805),
    ],
    &[
        ("is", 0.83463126),
        ("name", 0.56155246),
        ("node", 0.1440086),
        ("singleton", 0.65608037),
        ("unchecked", 0.45797387),
        ("from", 0.25178045),
    ],
    &[
        ("singleton", 0.8732137),
        ("name", 0.7474013),
        ("from", 0.33510855),
        ("node", 0.19166903),
        ("is", 1.110857),
    ],
    &[("name", 1.5143406), ("node", 0.38834855), ("has", 1.541038)],
    &[
        ("id", 0.6937309),
        ("type", 0.767626),
        ("has", 1.055091),
        ("node", 0.26588774),
    ],
    &[
        ("type", 0.767626),
        ("has", 1.055091),
        ("name", 1.0368124),
        ("node", 0.26588774),
    ],
    &[
        ("type", 0.767626),
        ("has", 1.055091),
        ("edge", 0.5729897),
        ("id", 0.6937309),
    ],
    &[
        ("name", 1.0368124),
        ("type", 0.767626),
        ("edge", 0.5729897),
        ("has", 1.055091),
    ],
    &[
        ("edge", 0.41304797),
        ("has", 0.76057774),
        ("ids", 0.49157697),
        ("from", 0.33510855),
        ("node", 0.19166903),
    ],
    &[
        ("node", 0.19166903),
        ("has", 0.76057774),
        ("selfloop", 1.6492976),
        ("id", 0.500086),
        ("from", 0.33510855),
    ],
    &[
        ("ids", 0.18657504),
        ("and", 0.421619),
        ("from", 0.12718841),
        ("type", 0.21002229),
        ("has", 0.28867266),
        ("id", 0.1898046),
        ("node", 0.07274681),
        ("edge", 0.29960722),
    ],
    &[
        ("id", 0.29181132),
        ("from", 0.19554329),
        ("trap", 0.79161775),
        ("node", 0.20876107),
        ("unchecked", 0.35568178),
        ("is", 0.64820975),
    ],
    &[
        ("trap", 1.0192826),
        ("is", 0.83463126),
        ("from", 0.25178045),
        ("node", 0.26373857),
        ("id", 0.3757346),
    ],
    &[
        ("type", 0.32289457),
        ("node", 0.20876107),
        ("name", 0.8140506),
        ("has", 0.44381404),
        ("and", 0.64820975),
    ],
    &[
        ("has", 0.76057774),
        ("from", 0.33510855),
        ("node", 0.19166903),
        ("edge", 0.41304797),
        ("names", 0.6397664),
    ],
    &[
        ("from", 0.12718841),
        ("has", 0.28867266),
        ("name", 0.28367162),
        ("and", 0.421619),
        ("edge", 0.29960722),
        ("names", 0.24281944),
        ("type", 0.21002229),
        ("node", 0.07274681),
    ],
    &[
        ("components", 2.9769197),
        ("strongly", 3.6587372),
        ("connected", 2.4510717),
    ],
    &[
        ("node", 0.1440086),
        ("sort", 1.1039106),
        ("by", 0.9863495),
        ("increasing", 1.2391833),
        ("outbound", 1.1039106),
        ("degree", 0.73351574),
    ],
    &[
        ("sort", 1.1039106),
        ("by", 0.9863495),
        ("node", 0.1440086),
        ("decreasing", 1.2391833),
        ("outbound", 1.1039106),
        ("degree", 0.73351574),
    ],
    &[
        ("lexicographic", 1.6492976),
        ("order", 1.6492976),
        ("sort", 1.4692557),
        ("node", 0.19166903),
        ("by", 1.3127872),
    ],
    &[
        ("node", 0.11184315),
        ("bfs", 0.9022622),
        ("topological", 0.9022622),
        ("sorting", 0.9022622),
        ("id", 0.29181132),
        ("from", 0.19554329),
        ("get", 0.066583954),
    ],
    &[
        ("bfs", 0.71973914),
        ("reversed", 0.84054554),
        ("id", 0.23277935),
        ("from", 0.15598588),
        ("node", 0.08921785),
        ("topological", 0.71973914),
        ("get", 0.05311436),
        ("sorting", 0.71973914),
    ],
    &[
        ("node", 0.08921785),
        ("id", 0.23277935),
        ("topological", 0.71973914),
        ("bfs", 0.71973914),
        ("sort", 0.68390715),
        ("from", 0.15598588),
        ("sorting", 0.71973914),
        ("by", 0.61107445),
    ],
    &[
        ("get", 0.11410697),
        ("binary", 1.805766),
        ("matrix", 1.3566197),
        ("dense", 1.5462344),
        ("adjacency", 1.5462344),
    ],
    &[
        ("dense", 1.5462344),
        ("adjacency", 1.5462344),
        ("matrix", 1.3566197),
        ("weighted", 0.7962348),
        ("get", 0.11410697),
    ],
    &[("remove", 3.1133206), ("components", 4.6195254)],
    &[("overlaps", 9.289986)],
    &[("contains", 8.485014)],
    &[
        ("bipartite", 3.34171),
        ("edges", 2.0358481),
        ("get", 0.23119685),
    ],
    &[
        ("names", 0.8874988),
        ("edge", 0.5729897),
        ("get", 0.15829183),
        ("bipartite", 2.2879438),
    ],
    &[
        ("get", 0.23119685),
        ("star", 3.1328895),
        ("edges", 2.0358481),
    ],
    &[
        ("star", 2.1449723),
        ("names", 0.8874988),
        ("edge", 0.5729897),
        ("get", 0.15829183),
    ],
    &[
        ("clique", 3.34171),
        ("edges", 2.0358481),
        ("get", 0.23119685),
    ],
    &[
        ("get", 0.15829183),
        ("edge", 0.5729897),
        ("names", 0.8874988),
        ("clique", 2.2879438),
    ],
    &[("encode", 5.6775565), ("edge", 1.2986751)],
    &[("edge", 1.2986751), ("decode", 5.6775565)],
    &[
        ("max", 1.805766),
        ("encodable", 1.805766),
        ("edge", 0.41304797),
        ("number", 0.75391865),
        ("get", 0.11410697),
    ],
    &[
        ("validate", 2.5822139),
        ("id", 1.0132449),
        ("node", 0.38834855),
    ],
    &[
        ("node", 0.38834855),
        ("validate", 2.5822139),
        ("ids", 0.9960044),
    ],
    &[
        ("edge", 0.83689356),
        ("validate", 2.5822139),
        ("id", 1.0132449),
    ],
    &[
        ("ids", 0.9960044),
        ("edge", 0.83689356),
        ("validate", 2.5822139),
    ],
    &[
        ("contain", 1.1039106),
        ("must", 0.8879779),
        ("unknown", 0.7785643),
        ("node", 0.1440086),
        ("types", 0.46108392),
        ("not", 1.0192826),
    ],
    &[
        ("unknown", 0.7785643),
        ("contain", 1.1039106),
        ("not", 1.0192826),
        ("edge", 0.31033945),
        ("types", 0.46108392),
        ("must", 0.8879779),
    ],
    &[
        ("id", 0.6937309),
        ("validate", 1.7679452),
        ("type", 0.767626),
        ("node", 0.26588774),
    ],
    &[
        ("validate", 1.7679452),
        ("type", 0.767626),
        ("ids", 0.68192697),
        ("node", 0.26588774),
    ],
    &[
        ("validate", 1.7679452),
        ("id", 0.6937309),
        ("type", 0.767626),
        ("edge", 0.5729897),
    ],
    &[
        ("ids", 0.68192697),
        ("edge", 0.5729897),
        ("type", 0.767626),
        ("validate", 1.7679452),
    ],
    &[
        ("be", 2.9769197),
        ("must", 2.3946133),
        ("undirected", 2.85238),
    ],
    &[
        ("nodes", 0.8038928),
        ("not", 1.3566197),
        ("trap", 1.3566197),
        ("must", 1.1818589),
        ("have", 1.5462344),
    ],
    &[
        ("be", 2.9769197),
        ("must", 2.3946133),
        ("multigraph", 3.1328895),
    ],
    &[
        ("multigraph", 2.1449723),
        ("must", 1.6395022),
        ("not", 1.8819343),
        ("be", 2.0381856),
    ],
    &[
        ("must", 1.6395022),
        ("contain", 2.0381856),
        ("matrix", 1.8819343),
        ("identity", 2.2879438),
    ],
    &[
        ("singleton", 0.65608037),
        ("nodes", 0.6039968),
        ("contain", 1.1039106),
        ("weighted", 0.598243),
        ("must", 0.8879779),
        ("not", 1.0192826),
    ],
    &[
        ("edges", 2.0358481),
        ("must", 2.3946133),
        ("have", 3.1328895),
    ],
    &[
        ("must", 2.3946133),
        ("have", 3.1328895),
        ("nodes", 1.6288004),
    ],
    &[
        ("must", 2.3946133),
        ("be", 2.9769197),
        ("connected", 2.4510717),
    ],
    &[
        ("total", 1.6040721),
        ("weights", 1.5410066),
        ("edge", 0.5729897),
        ("get", 0.15829183),
    ],
    &[
        ("weight", 1.6395022),
        ("edge", 0.5729897),
        ("get", 0.15829183),
        ("mininum", 2.5050006),
    ],
    &[
        ("maximum", 1.7679452),
        ("weight", 1.6395022),
        ("edge", 0.5729897),
        ("get", 0.15829183),
    ],
    &[
        ("get", 0.11410697),
        ("maximum", 1.274449),
        ("degree", 0.97627676),
        ("node", 0.19166903),
        ("unchecked", 0.6095428),
    ],
    &[
        ("minimum", 1.3127872),
        ("unchecked", 0.6095428),
        ("get", 0.11410697),
        ("node", 0.19166903),
        ("degree", 0.97627676),
    ],
    &[
        ("maximum", 1.274449),
        ("degree", 0.97627676),
        ("node", 0.19166903),
        ("weighted", 0.7962348),
        ("get", 0.11410697),
    ],
    &[
        ("weighted", 0.7962348),
        ("degree", 0.97627676),
        ("minimum", 1.3127872),
        ("get", 0.11410697),
        ("node", 0.19166903),
    ],
    &[
        ("nodes", 0.8038928),
        ("singleton", 0.8732137),
        ("weighted", 0.7962348),
        ("get", 0.11410697),
        ("number", 0.75391865),
    ],
    &[
        ("number", 1.5275458),
        ("selfloops", 2.3428652),
        ("get", 0.23119685),
    ],
    &[
        ("selfloops", 1.6040721),
        ("number", 1.0458535),
        ("get", 0.15829183),
        ("unique", 1.6781572),
    ],
    &[
        ("from", 0.25178045),
        ("generate", 0.8879779),
        ("new", 1.3567443),
        ("edges", 0.75493956),
        ("features", 1.3567443),
        ("node", 0.1440086),
    ],
    &[
        ("types", 0.61368215),
        ("all", 1.3127872),
        ("inplace", 1.0904291),
        ("edge", 0.41304797),
        ("set", 1.4077892),
    ],
    &[
        ("set", 1.9529178),
        ("edge", 0.5729897),
        ("all", 1.8211288),
        ("types", 0.85131407),
    ],
    &[
        ("inplace", 1.0904291),
        ("types", 0.61368215),
        ("node", 0.19166903),
        ("set", 1.4077892),
        ("all", 1.3127872),
    ],
    &[
        ("set", 1.9529178),
        ("node", 0.26588774),
        ("all", 1.8211288),
        ("types", 0.85131407),
    ],
    &[
        ("inplace", 1.0904291),
        ("remove", 0.99020207),
        ("type", 0.5533544),
        ("ids", 0.49157697),
        ("node", 0.19166903),
    ],
    &[
        ("node", 0.19166903),
        ("types", 0.61368215),
        ("remove", 0.99020207),
        ("singleton", 0.8732137),
        ("inplace", 1.0904291),
    ],
    &[
        ("inplace", 1.0904291),
        ("remove", 0.99020207),
        ("node", 0.19166903),
        ("homogeneous", 1.3127872),
        ("types", 0.61368215),
    ],
    &[
        ("edge", 0.41304797),
        ("remove", 0.99020207),
        ("type", 0.5533544),
        ("ids", 0.49157697),
        ("inplace", 1.0904291),
    ],
    &[
        ("types", 0.61368215),
        ("edge", 0.41304797),
        ("remove", 0.99020207),
        ("inplace", 1.0904291),
        ("singleton", 0.8732137),
    ],
    &[
        ("name", 0.7474013),
        ("inplace", 1.0904291),
        ("node", 0.19166903),
        ("remove", 0.99020207),
        ("type", 0.5533544),
    ],
    &[
        ("remove", 1.3736314),
        ("id", 0.6937309),
        ("node", 0.26588774),
        ("type", 0.767626),
    ],
    &[
        ("singleton", 1.2113423),
        ("node", 0.26588774),
        ("remove", 1.3736314),
        ("types", 0.85131407),
    ],
    &[
        ("node", 0.26588774),
        ("homogeneous", 1.8211288),
        ("remove", 1.3736314),
        ("types", 0.85131407),
    ],
    &[
        ("name", 1.0368124),
        ("type", 0.767626),
        ("node", 0.26588774),
        ("remove", 1.3736314),
    ],
    &[
        ("type", 0.5533544),
        ("edge", 0.41304797),
        ("name", 0.7474013),
        ("remove", 0.99020207),
        ("inplace", 1.0904291),
    ],
    &[
        ("type", 0.767626),
        ("remove", 1.3736314),
        ("id", 0.6937309),
        ("edge", 0.5729897),
    ],
    &[
        ("edge", 0.5729897),
        ("types", 0.85131407),
        ("singleton", 1.2113423),
        ("remove", 1.3736314),
    ],
    &[
        ("type", 0.767626),
        ("edge", 0.5729897),
        ("name", 1.0368124),
        ("remove", 1.3736314),
    ],
    &[
        ("remove", 1.3736314),
        ("inplace", 1.5126685),
        ("types", 0.85131407),
        ("node", 0.26588774),
    ],
    &[
        ("types", 1.2434068),
        ("remove", 2.0062895),
        ("node", 0.38834855),
    ],
    &[
        ("edge", 0.5729897),
        ("types", 0.85131407),
        ("inplace", 1.5126685),
        ("remove", 1.3736314),
    ],
    &[
        ("edge", 0.83689356),
        ("types", 1.2434068),
        ("remove", 2.0062895),
    ],
    &[
        ("remove", 1.3736314),
        ("inplace", 1.5126685),
        ("edge", 0.5729897),
        ("weights", 1.5410066),
    ],
    &[
        ("remove", 2.0062895),
        ("edge", 0.83689356),
        ("weights", 2.2507532),
    ],
    &[
        ("divide", 2.2879438),
        ("inplace", 1.5126685),
        ("weights", 1.5410066),
        ("edge", 0.5729897),
    ],
    &[
        ("weights", 2.2507532),
        ("edge", 0.83689356),
        ("divide", 3.34171),
    ],
    &[
        ("edge", 0.5729897),
        ("weights", 1.5410066),
        ("inplace", 1.5126685),
        ("multiply", 2.2879438),
    ],
    &[
        ("edge", 0.83689356),
        ("weights", 2.2507532),
        ("multiply", 3.34171),
    ],
    &[("get", 0.35876673), ("circles", 5.6775565)],
    &[
        ("memory", 2.3428652),
        ("stats", 3.6587372),
        ("get", 0.23119685),
    ],
    &[
        ("used", 2.5050006),
        ("total", 1.6040721),
        ("get", 0.15829183),
        ("memory", 1.6040721),
    ],
    &[
        ("requirement", 1.4692557),
        ("total", 1.1563187),
        ("nodes", 0.8038928),
        ("get", 0.11410697),
        ("memory", 1.1563187),
    ],
    &[
        ("requirement", 0.8573434),
        ("nodes", 0.46908933),
        ("readable", 0.82147634),
        ("memory", 0.6747377),
        ("get", 0.066583954),
        ("human", 0.82147634),
        ("total", 0.6747377),
    ],
    &[
        ("total", 1.1563187),
        ("requirement", 1.4692557),
        ("memory", 1.1563187),
        ("edges", 1.0047908),
        ("get", 0.11410697),
    ],
    &[
        ("requirement", 0.8573434),
        ("edges", 0.58631784),
        ("readable", 0.82147634),
        ("human", 0.82147634),
        ("get", 0.066583954),
        ("total", 0.6747377),
        ("memory", 0.6747377),
    ],
    &[
        ("memory", 0.8687885),
        ("edge", 0.31033945),
        ("weights", 0.83463126),
        ("requirements", 1.0192826),
        ("get", 0.08573313),
        ("total", 0.8687885),
    ],
    &[
        ("get", 0.05311436),
        ("total", 0.5382417),
        ("requirements", 0.63147753),
        ("memory", 0.5382417),
        ("edge", 0.19226502),
        ("readable", 0.65529585),
        ("weights", 0.5170802),
        ("human", 0.65529585),
    ],
    &[
        ("node", 0.1440086),
        ("get", 0.08573313),
        ("total", 0.8687885),
        ("requirements", 1.0192826),
        ("types", 0.46108392),
        ("memory", 0.8687885),
    ],
    &[
        ("memory", 0.5382417),
        ("node", 0.08921785),
        ("readable", 0.65529585),
        ("human", 0.65529585),
        ("types", 0.28565595),
        ("get", 0.05311436),
        ("requirements", 0.63147753),
        ("total", 0.5382417),
    ],
    &[
        ("edge", 0.31033945),
        ("requirements", 1.0192826),
        ("types", 0.46108392),
        ("total", 0.8687885),
        ("memory", 0.8687885),
        ("get", 0.08573313),
    ],
    &[
        ("get", 0.05311436),
        ("types", 0.28565595),
        ("requirements", 0.63147753),
        ("edge", 0.19226502),
        ("readable", 0.65529585),
        ("human", 0.65529585),
        ("total", 0.5382417),
        ("memory", 0.5382417),
    ],
    &[
        ("get", 0.15829183),
        ("of", 1.7679452),
        ("number", 1.0458535),
        ("triangles", 2.2879438),
    ],
    &[
        ("triads", 3.34171),
        ("number", 1.5275458),
        ("get", 0.23119685),
    ],
    &[
        ("get", 0.15829183),
        ("weighted", 1.1045554),
        ("triads", 2.2879438),
        ("number", 1.0458535),
    ],
    &[("transitivity", 5.6775565), ("get", 0.35876673)],
    &[
        ("get", 0.08573313),
        ("of", 0.9575445),
        ("number", 0.5664492),
        ("triangles", 1.2391833),
        ("per", 1.2391833),
        ("node", 0.1440086),
    ],
    &[
        ("per", 1.6492976),
        ("clustering", 1.5462344),
        ("get", 0.11410697),
        ("node", 0.19166903),
        ("coefficient", 1.3566197),
    ],
    &[
        ("get", 0.23119685),
        ("coefficient", 2.7487032),
        ("clustering", 3.1328895),
    ],
    &[
        ("average", 2.5050006),
        ("get", 0.15829183),
        ("coefficient", 1.8819343),
        ("clustering", 2.1449723),
    ],
    &[
        ("remappable", 3.6587372),
        ("nodes", 1.6288004),
        ("are", 3.6587372),
    ],
    &[
        ("from", 0.33510855),
        ("unchecked", 0.6095428),
        ("node", 0.19166903),
        ("remap", 1.4077892),
        ("ids", 0.49157697),
    ],
    &[
        ("ids", 0.68192697),
        ("from", 0.46487033),
        ("remap", 1.9529178),
        ("node", 0.26588774),
    ],
    &[
        ("remap", 1.9529178),
        ("names", 0.8874988),
        ("from", 0.46487033),
        ("node", 0.26588774),
    ],
    &[
        ("node", 0.19166903),
        ("remap", 1.4077892),
        ("from", 0.33510855),
        ("names", 0.6397664),
        ("map", 1.805766),
    ],
    &[
        ("from", 0.67897725),
        ("graph", 2.2093635),
        ("remap", 2.85238),
    ],
    &[("sample", 5.6775565), ("negatives", 5.6775565)],
    &[("connected", 3.8035252), ("holdout", 4.0070286)],
    &[("random", 3.6356122), ("holdout", 4.0070286)],
    &[
        ("node", 0.19166903),
        ("indices", 1.805766),
        ("get", 0.11410697),
        ("holdout", 1.274449),
        ("label", 1.24038),
    ],
    &[
        ("holdout", 1.274449),
        ("get", 0.11410697),
        ("labels", 1.805766),
        ("label", 1.24038),
        ("node", 0.19166903),
    ],
    &[
        ("get", 0.11410697),
        ("node", 0.19166903),
        ("holdout", 1.274449),
        ("label", 1.24038),
        ("graphs", 1.6492976),
    ],
    &[
        ("label", 1.24038),
        ("graphs", 1.6492976),
        ("get", 0.11410697),
        ("holdout", 1.274449),
        ("edge", 0.41304797),
    ],
    &[
        ("random", 2.3428652),
        ("subgraph", 3.6587372),
        ("get", 0.23119685),
    ],
    &[
        ("random", 1.1563187),
        ("get", 0.11410697),
        ("label", 1.24038),
        ("holdout", 1.274449),
        ("node", 0.19166903),
    ],
    &[
        ("label", 1.7206839),
        ("get", 0.15829183),
        ("kfold", 2.1449723),
        ("node", 0.26588774),
    ],
    &[
        ("label", 1.24038),
        ("random", 1.1563187),
        ("get", 0.11410697),
        ("holdout", 1.274449),
        ("edge", 0.41304797),
    ],
    &[
        ("get", 0.15829183),
        ("kfold", 2.1449723),
        ("edge", 0.5729897),
        ("label", 1.7206839),
    ],
    &[
        ("kfold", 2.1449723),
        ("edge", 0.5729897),
        ("get", 0.15829183),
        ("prediction", 2.5050006),
    ],
    &[
        ("node", 0.38834855),
        ("tuples", 3.6587372),
        ("get", 0.23119685),
    ],
    &[("chains", 5.6775565), ("get", 0.35876673)],
    &[
        ("unchecked", 0.19210443),
        ("parallel", 0.44368097),
        ("id", 0.15760785),
        ("from", 0.10561331),
        ("first", 0.39092004),
        ("breadth", 0.39092004),
        ("predecessors", 0.56910795),
        ("node", 0.060406704),
        ("get", 0.035962123),
        ("search", 0.39092004),
    ],
    &[
        ("unchecked", 0.19210443),
        ("parallel", 0.44368097),
        ("first", 0.39092004),
        ("ids", 0.15492614),
        ("breadth", 0.39092004),
        ("from", 0.10561331),
        ("node", 0.060406704),
        ("get", 0.035962123),
        ("distances", 0.48731357),
        ("search", 0.39092004),
    ],
    &[
        ("parallel", 0.44368097),
        ("id", 0.15760785),
        ("first", 0.39092004),
        ("distances", 0.48731357),
        ("unchecked", 0.19210443),
        ("breadth", 0.39092004),
        ("node", 0.060406704),
        ("get", 0.035962123),
        ("search", 0.39092004),
        ("from", 0.10561331),
    ],
    &[
        ("breadth", 0.39092004),
        ("node", 0.060406704),
        ("search", 0.39092004),
        ("get", 0.035962123),
        ("unchecked", 0.19210443),
        ("sequential", 0.56910795),
        ("first", 0.39092004),
        ("from", 0.10561331),
        ("distances", 0.48731357),
        ("id", 0.15760785),
    ],
    &[
        ("breadth", 0.57737046),
        ("from", 0.15598588),
        ("search", 0.57737046),
        ("unchecked", 0.28372917),
        ("ids", 0.22881858),
        ("node", 0.08921785),
        ("first", 0.57737046),
        ("get", 0.05311436),
    ],
    &[
        ("from", 0.15598588),
        ("breadth", 0.57737046),
        ("node", 0.08921785),
        ("first", 0.57737046),
        ("id", 0.23277935),
        ("get", 0.05311436),
        ("search", 0.57737046),
        ("unchecked", 0.28372917),
    ],
    &[
        ("get", 0.043308605),
        ("from", 0.12718841),
        ("node", 0.13902846),
        ("path", 0.421619),
        ("shortest", 0.40659735),
        ("unchecked", 0.23134826),
        ("ids", 0.35656875),
    ],
    &[
        ("unchecked", 0.23134826),
        ("names", 0.24281944),
        ("from", 0.12718841),
        ("path", 0.421619),
        ("get", 0.043308605),
        ("node", 0.13902846),
        ("shortest", 0.40659735),
        ("ids", 0.18657504),
    ],
    &[
        ("get", 0.05311436),
        ("from", 0.15598588),
        ("ids", 0.43294567),
        ("node", 0.16880831),
        ("path", 0.5170802),
        ("shortest", 0.49865744),
    ],
    &[
        ("ids", 0.22881858),
        ("from", 0.15598588),
        ("shortest", 0.49865744),
        ("node", 0.16880831),
        ("names", 0.29779762),
        ("get", 0.05311436),
        ("path", 0.5170802),
    ],
    &[
        ("node", 0.16880831),
        ("names", 0.5634603),
        ("path", 0.5170802),
        ("get", 0.05311436),
        ("from", 0.15598588),
        ("shortest", 0.49865744),
    ],
    &[
        ("shortest", 0.33762583),
        ("k", 0.41374),
        ("node", 0.11632172),
        ("ids", 0.29833236),
        ("unchecked", 0.19210443),
        ("path", 0.35009935),
        ("from", 0.10561331),
        ("get", 0.035962123),
    ],
    &[
        ("path", 0.421619),
        ("get", 0.043308605),
        ("k", 0.4982604),
        ("shortest", 0.40659735),
        ("ids", 0.35656875),
        ("node", 0.13902846),
        ("from", 0.12718841),
    ],
    &[
        ("k", 0.4982604),
        ("get", 0.043308605),
        ("from", 0.12718841),
        ("shortest", 0.40659735),
        ("ids", 0.18657504),
        ("path", 0.421619),
        ("names", 0.24281944),
        ("node", 0.13902846),
    ],
    &[
        ("k", 0.4982604),
        ("node", 0.13902846),
        ("path", 0.421619),
        ("names", 0.464059),
        ("from", 0.12718841),
        ("shortest", 0.40659735),
        ("get", 0.043308605),
    ],
    &[
        ("eccentricity", 0.3605124),
        ("node", 0.09865726),
        ("from", 0.089052804),
        ("get", 0.030323148),
        ("most", 0.39044464),
        ("and", 0.29520264),
        ("distant", 0.43828955),
        ("unchecked", 0.16198184),
        ("id", 0.25740784),
    ],
    &[
        ("get", 0.066583954),
        ("unchecked", 0.35568178),
        ("node", 0.11184315),
        ("id", 0.29181132),
        ("weighted", 0.4646207),
        ("eccentricity", 0.79161775),
        ("from", 0.19554329),
    ],
    &[
        ("node", 0.11632172),
        ("from", 0.10561331),
        ("eccentricity", 0.4275543),
        ("distant", 0.5197951),
        ("id", 0.3034964),
        ("and", 0.35009935),
        ("get", 0.035962123),
        ("most", 0.4630528),
    ],
    &[
        ("get", 0.08573313),
        ("from", 0.25178045),
        ("weighted", 0.598243),
        ("node", 0.1440086),
        ("id", 0.3757346),
        ("eccentricity", 1.0192826),
    ],
    &[
        ("node", 0.19166903),
        ("from", 0.33510855),
        ("get", 0.11410697),
        ("eccentricity", 1.3566197),
        ("name", 0.7474013),
    ],
    &[
        ("weighted", 0.598243),
        ("eccentricity", 1.0192826),
        ("node", 0.1440086),
        ("get", 0.08573313),
        ("name", 0.56155246),
        ("from", 0.25178045),
    ],
    &[
        ("ids", 0.36934143),
        ("unchecked", 0.45797387),
        ("from", 0.25178045),
        ("dijkstra", 1.1039106),
        ("node", 0.1440086),
        ("get", 0.08573313),
    ],
    &[
        ("node", 0.1440086),
        ("get", 0.08573313),
        ("from", 0.25178045),
        ("unchecked", 0.45797387),
        ("id", 0.3757346),
        ("dijkstra", 1.1039106),
    ],
    &[
        ("get", 0.035962123),
        ("ids", 0.29833236),
        ("path", 0.35009935),
        ("weighted", 0.25094256),
        ("from", 0.10561331),
        ("node", 0.11632172),
        ("shortest", 0.33762583),
        ("unchecked", 0.19210443),
    ],
    &[
        ("node", 0.11632172),
        ("from", 0.10561331),
        ("shortest", 0.33762583),
        ("names", 0.20162974),
        ("get", 0.035962123),
        ("weighted", 0.25094256),
        ("path", 0.35009935),
        ("unchecked", 0.19210443),
        ("ids", 0.15492614),
    ],
    &[
        ("ids", 0.35656875),
        ("shortest", 0.40659735),
        ("from", 0.12718841),
        ("weighted", 0.30220607),
        ("node", 0.13902846),
        ("get", 0.043308605),
        ("path", 0.421619),
    ],
    &[
        ("shortest", 0.40659735),
        ("node", 0.13902846),
        ("path", 0.421619),
        ("get", 0.043308605),
        ("names", 0.24281944),
        ("weighted", 0.30220607),
        ("ids", 0.18657504),
        ("from", 0.12718841),
    ],
    &[
        ("from", 0.12718841),
        ("weighted", 0.30220607),
        ("get", 0.043308605),
        ("path", 0.421619),
        ("node", 0.13902846),
        ("names", 0.464059),
        ("shortest", 0.40659735),
    ],
    &[
        ("first", 0.72378933),
        ("get", 0.066583954),
        ("ids", 0.2868461),
        ("breadth", 0.72378933),
        ("node", 0.11184315),
        ("from", 0.19554329),
        ("search", 0.72378933),
    ],
    &[
        ("dijkstra", 1.4692557),
        ("ids", 0.49157697),
        ("get", 0.11410697),
        ("node", 0.19166903),
        ("from", 0.33510855),
    ],
    &[
        ("get", 0.23119685),
        ("diameter", 3.1328895),
        ("naive", 3.34171),
    ],
    &[("get", 0.35876673), ("diameter", 4.8615565)],
    &[
        ("diameter", 2.1449723),
        ("get", 0.15829183),
        ("weighted", 1.1045554),
        ("naive", 2.2879438),
    ],
    &[
        ("first", 0.72378933),
        ("node", 0.11184315),
        ("breadth", 0.72378933),
        ("names", 0.37331793),
        ("from", 0.19554329),
        ("search", 0.72378933),
        ("get", 0.066583954),
    ],
    &[
        ("from", 0.33510855),
        ("node", 0.19166903),
        ("dijkstra", 1.4692557),
        ("get", 0.11410697),
        ("names", 0.6397664),
    ],
    &[
        ("number", 1.0458535),
        ("connected", 1.6781572),
        ("components", 2.0381856),
        ("get", 0.15829183),
    ],
    &[
        ("nodes", 1.1151787),
        ("get", 0.15829183),
        ("connected", 1.6781572),
        ("number", 1.0458535),
    ],
    &[
        ("with", 0.74397844),
        ("nodes", 0.6039968),
        ("number", 0.5664492),
        ("get", 0.08573313),
        ("singleton", 0.65608037),
        ("selfloops", 0.8687885),
    ],
    &[
        ("number", 1.0458535),
        ("nodes", 1.1151787),
        ("singleton", 1.2113423),
        ("get", 0.15829183),
    ],
    &[
        ("disconnected", 2.0381856),
        ("nodes", 1.1151787),
        ("number", 1.0458535),
        ("get", 0.15829183),
    ],
    &[
        ("node", 0.26588774),
        ("ids", 0.68192697),
        ("get", 0.15829183),
        ("singleton", 1.2113423),
    ],
    &[
        ("names", 0.8874988),
        ("singleton", 1.2113423),
        ("get", 0.15829183),
        ("node", 0.26588774),
    ],
    &[
        ("ids", 0.36934143),
        ("selfloops", 0.8687885),
        ("with", 0.74397844),
        ("get", 0.08573313),
        ("node", 0.1440086),
        ("singleton", 0.65608037),
    ],
    &[
        ("node", 0.1440086),
        ("selfloops", 0.8687885),
        ("names", 0.48068208),
        ("get", 0.08573313),
        ("with", 0.74397844),
        ("singleton", 0.65608037),
    ],
    &[("get", 0.35876673), ("density", 5.6775565)],
    &[
        ("rate", 1.8819343),
        ("trap", 1.8819343),
        ("nodes", 1.1151787),
        ("get", 0.15829183),
    ],
    &[
        ("node", 0.26588774),
        ("get", 0.15829183),
        ("mean", 2.2879438),
        ("degrees", 1.6781572),
    ],
    &[
        ("node", 0.19166903),
        ("get", 0.11410697),
        ("degrees", 1.2097241),
        ("mean", 1.6492976),
        ("weighted", 0.7962348),
    ],
    &[
        ("get", 0.15829183),
        ("edges", 1.3938692),
        ("number", 1.0458535),
        ("undirected", 1.9529178),
    ],
    &[
        ("unique", 1.2097241),
        ("undirected", 1.4077892),
        ("number", 0.75391865),
        ("get", 0.11410697),
        ("edges", 1.0047908),
    ],
    &[
        ("get", 0.23119685),
        ("number", 1.5275458),
        ("edges", 2.0358481),
    ],
    &[
        ("number", 1.0458535),
        ("get", 0.15829183),
        ("unique", 1.6781572),
        ("edges", 1.3938692),
    ],
    &[
        ("node", 0.26588774),
        ("median", 2.2879438),
        ("degrees", 1.6781572),
        ("get", 0.15829183),
    ],
    &[
        ("weighted", 0.7962348),
        ("median", 1.6492976),
        ("node", 0.19166903),
        ("degrees", 1.2097241),
        ("get", 0.11410697),
    ],
    &[
        ("maximum", 1.7679452),
        ("get", 0.15829183),
        ("degree", 1.3543137),
        ("node", 0.26588774),
    ],
    &[
        ("get", 0.08573313),
        ("node", 0.1440086),
        ("central", 1.0577283),
        ("id", 0.3757346),
        ("most", 1.1039106),
        ("unchecked", 0.45797387),
    ],
    &[
        ("most", 1.4692557),
        ("id", 0.500086),
        ("central", 1.4077892),
        ("get", 0.11410697),
        ("node", 0.19166903),
    ],
    &[
        ("minimum", 1.8211288),
        ("degree", 1.3543137),
        ("get", 0.15829183),
        ("node", 0.26588774),
    ],
    &[
        ("mode", 2.5050006),
        ("degrees", 1.6781572),
        ("get", 0.15829183),
        ("node", 0.26588774),
    ],
    &[
        ("nodes", 1.1151787),
        ("get", 0.15829183),
        ("selfloop", 2.2879438),
        ("rate", 1.8819343),
    ],
    &[("get", 0.35876673), ("name", 2.349924)],
    &[
        ("get", 0.15829183),
        ("nodes", 1.1151787),
        ("trap", 1.8819343),
        ("number", 1.0458535),
    ],
    &[
        ("ids", 0.68192697),
        ("node", 0.26588774),
        ("get", 0.15829183),
        ("source", 1.6395022),
    ],
    &[
        ("source", 1.1818589),
        ("get", 0.11410697),
        ("ids", 0.49157697),
        ("node", 0.19166903),
        ("directed", 1.2097241),
    ],
    &[
        ("source", 2.3946133),
        ("names", 1.2962571),
        ("get", 0.23119685),
    ],
    &[
        ("destination", 1.8211288),
        ("node", 0.26588774),
        ("ids", 0.68192697),
        ("get", 0.15829183),
    ],
    &[
        ("node", 0.19166903),
        ("directed", 1.2097241),
        ("destination", 1.3127872),
        ("ids", 0.49157697),
        ("get", 0.11410697),
    ],
    &[
        ("destination", 2.6598926),
        ("names", 1.2962571),
        ("get", 0.23119685),
    ],
    &[
        ("names", 1.2962571),
        ("get", 0.23119685),
        ("node", 0.38834855),
    ],
    &[
        ("get", 0.23119685),
        ("node", 0.38834855),
        ("urls", 3.6587372),
    ],
    &[
        ("node", 0.38834855),
        ("get", 0.23119685),
        ("ontologies", 3.6587372),
    ],
    &[
        ("node", 0.38834855),
        ("ids", 0.9960044),
        ("get", 0.23119685),
    ],
    &[
        ("edge", 0.5729897),
        ("type", 0.767626),
        ("ids", 0.68192697),
        ("get", 0.15829183),
    ],
    &[
        ("type", 0.5533544),
        ("ids", 0.49157697),
        ("edge", 0.41304797),
        ("get", 0.11410697),
        ("unique", 1.2097241),
    ],
    &[
        ("type", 0.767626),
        ("edge", 0.5729897),
        ("get", 0.15829183),
        ("names", 0.8874988),
    ],
    &[
        ("names", 0.6397664),
        ("edge", 0.41304797),
        ("unique", 1.2097241),
        ("get", 0.11410697),
        ("type", 0.5533544),
    ],
    &[
        ("weights", 2.2507532),
        ("edge", 0.83689356),
        ("get", 0.23119685),
    ],
    &[
        ("node", 0.26588774),
        ("get", 0.15829183),
        ("weighted", 1.1045554),
        ("indegrees", 2.2879438),
    ],
    &[
        ("node", 0.26588774),
        ("type", 0.767626),
        ("ids", 0.68192697),
        ("get", 0.15829183),
    ],
    &[
        ("known", 1.0712788),
        ("node", 0.19166903),
        ("mask", 1.3566197),
        ("types", 0.61368215),
        ("get", 0.11410697),
    ],
    &[
        ("unknown", 1.0362344),
        ("get", 0.11410697),
        ("types", 0.61368215),
        ("node", 0.19166903),
        ("mask", 1.3566197),
    ],
    &[
        ("node", 0.1440086),
        ("encoded", 1.1039106),
        ("get", 0.08573313),
        ("hot", 1.1039106),
        ("types", 0.46108392),
        ("one", 1.1039106),
    ],
    &[
        ("node", 0.11184315),
        ("get", 0.066583954),
        ("types", 0.3580972),
        ("encoded", 0.8573434),
        ("known", 0.62511504),
        ("hot", 0.8573434),
        ("one", 0.8573434),
    ],
    &[
        ("encoded", 1.1039106),
        ("edge", 0.31033945),
        ("get", 0.08573313),
        ("hot", 1.1039106),
        ("types", 0.46108392),
        ("one", 1.1039106),
    ],
    &[
        ("get", 0.066583954),
        ("types", 0.3580972),
        ("known", 0.62511504),
        ("edge", 0.24102269),
        ("hot", 0.8573434),
        ("encoded", 0.8573434),
        ("one", 0.8573434),
    ],
    &[
        ("names", 0.8874988),
        ("node", 0.26588774),
        ("type", 0.767626),
        ("get", 0.15829183),
    ],
    &[
        ("type", 0.5533544),
        ("node", 0.19166903),
        ("ids", 0.49157697),
        ("unique", 1.2097241),
        ("get", 0.11410697),
    ],
    &[
        ("type", 0.5533544),
        ("node", 0.19166903),
        ("names", 0.6397664),
        ("get", 0.11410697),
        ("unique", 1.2097241),
    ],
    &[
        ("unique", 1.2097241),
        ("get", 0.11410697),
        ("number", 0.75391865),
        ("directed", 1.2097241),
        ("edges", 1.0047908),
    ],
    &[
        ("get", 0.23119685),
        ("mapping", 3.34171),
        ("nodes", 1.6288004),
    ],
    &[
        ("ids", 0.68192697),
        ("get", 0.15829183),
        ("edge", 0.5729897),
        ("node", 0.26588774),
    ],
    &[
        ("node", 0.19166903),
        ("ids", 0.49157697),
        ("edge", 0.41304797),
        ("directed", 1.2097241),
        ("get", 0.11410697),
    ],
    &[
        ("get", 0.15829183),
        ("node", 0.26588774),
        ("edge", 0.5729897),
        ("names", 0.8874988),
    ],
    &[
        ("names", 0.6397664),
        ("node", 0.19166903),
        ("edge", 0.41304797),
        ("get", 0.11410697),
        ("directed", 1.2097241),
    ],
    &[
        ("unknown", 1.0362344),
        ("get", 0.11410697),
        ("types", 0.61368215),
        ("number", 0.75391865),
        ("node", 0.19166903),
    ],
    &[
        ("number", 0.75391865),
        ("types", 0.61368215),
        ("known", 1.0712788),
        ("get", 0.11410697),
        ("node", 0.19166903),
    ],
    &[
        ("unknown", 1.0362344),
        ("types", 0.61368215),
        ("get", 0.11410697),
        ("rate", 1.3566197),
        ("node", 0.19166903),
    ],
    &[
        ("rate", 1.3566197),
        ("get", 0.11410697),
        ("known", 1.0712788),
        ("types", 0.61368215),
        ("node", 0.19166903),
    ],
    &[
        ("types", 0.61368215),
        ("minimum", 1.3127872),
        ("node", 0.19166903),
        ("number", 0.75391865),
        ("get", 0.11410697),
    ],
    &[
        ("get", 0.11410697),
        ("node", 0.19166903),
        ("types", 0.61368215),
        ("maximum", 1.274449),
        ("number", 0.75391865),
    ],
    &[
        ("count", 1.8819343),
        ("get", 0.15829183),
        ("multilabel", 2.2879438),
        ("maximum", 1.7679452),
    ],
    &[
        ("types", 0.61368215),
        ("get", 0.11410697),
        ("number", 0.75391865),
        ("singleton", 0.8732137),
        ("node", 0.19166903),
    ],
    &[
        ("number", 0.75391865),
        ("node", 0.19166903),
        ("types", 0.61368215),
        ("homogeneous", 1.3127872),
        ("get", 0.11410697),
    ],
    &[
        ("ids", 0.49157697),
        ("get", 0.11410697),
        ("type", 0.5533544),
        ("node", 0.19166903),
        ("homogeneous", 1.3127872),
    ],
    &[
        ("get", 0.11410697),
        ("names", 0.6397664),
        ("homogeneous", 1.3127872),
        ("node", 0.19166903),
        ("type", 0.5533544),
    ],
    &[
        ("type", 0.5533544),
        ("singleton", 0.8732137),
        ("get", 0.11410697),
        ("node", 0.19166903),
        ("ids", 0.49157697),
    ],
    &[
        ("names", 0.6397664),
        ("type", 0.5533544),
        ("node", 0.19166903),
        ("get", 0.11410697),
        ("singleton", 0.8732137),
    ],
    &[
        ("number", 0.75391865),
        ("unknown", 1.0362344),
        ("types", 0.61368215),
        ("get", 0.11410697),
        ("edge", 0.41304797),
    ],
    &[
        ("types", 0.3580972),
        ("unknown", 0.6046658),
        ("get", 0.066583954),
        ("ids", 0.2868461),
        ("edge", 0.4498814),
        ("with", 0.577805),
    ],
    &[
        ("get", 0.066583954),
        ("types", 0.3580972),
        ("with", 0.577805),
        ("known", 0.62511504),
        ("ids", 0.2868461),
        ("edge", 0.4498814),
    ],
    &[
        ("types", 0.28565595),
        ("node", 0.08921785),
        ("edge", 0.36378297),
        ("unknown", 0.48234496),
        ("ids", 0.22881858),
        ("with", 0.46091795),
        ("get", 0.05311436),
    ],
    &[
        ("types", 0.28565595),
        ("edge", 0.36378297),
        ("with", 0.46091795),
        ("known", 0.49865744),
        ("get", 0.05311436),
        ("ids", 0.22881858),
        ("node", 0.08921785),
    ],
    &[
        ("node", 0.08921785),
        ("types", 0.28565595),
        ("get", 0.05311436),
        ("unknown", 0.48234496),
        ("with", 0.46091795),
        ("names", 0.29779762),
        ("edge", 0.36378297),
    ],
    &[
        ("edge", 0.36378297),
        ("known", 0.49865744),
        ("with", 0.46091795),
        ("types", 0.28565595),
        ("names", 0.29779762),
        ("node", 0.08921785),
        ("get", 0.05311436),
    ],
    &[
        ("unknown", 0.48234496),
        ("edge", 0.36378297),
        ("types", 0.28565595),
        ("mask", 0.63147753),
        ("with", 0.46091795),
        ("ids", 0.22881858),
        ("get", 0.05311436),
    ],
    &[
        ("mask", 0.63147753),
        ("with", 0.46091795),
        ("edge", 0.36378297),
        ("known", 0.49865744),
        ("types", 0.28565595),
        ("get", 0.05311436),
        ("ids", 0.22881858),
    ],
    &[
        ("with", 0.577805),
        ("ids", 0.2868461),
        ("node", 0.20876107),
        ("unknown", 0.6046658),
        ("types", 0.3580972),
        ("get", 0.066583954),
    ],
    &[
        ("with", 0.577805),
        ("ids", 0.2868461),
        ("get", 0.066583954),
        ("types", 0.3580972),
        ("known", 0.62511504),
        ("node", 0.20876107),
    ],
    &[
        ("unknown", 0.6046658),
        ("get", 0.066583954),
        ("names", 0.37331793),
        ("node", 0.20876107),
        ("types", 0.3580972),
        ("with", 0.577805),
    ],
    &[
        ("type", 0.32289457),
        ("get", 0.066583954),
        ("from", 0.19554329),
        ("id", 0.29181132),
        ("node", 0.20876107),
        ("ids", 0.2868461),
    ],
    &[
        ("from", 0.19554329),
        ("get", 0.066583954),
        ("node", 0.20876107),
        ("type", 0.32289457),
        ("names", 0.37331793),
        ("id", 0.29181132),
    ],
    &[
        ("from", 0.19554329),
        ("type", 0.32289457),
        ("node", 0.20876107),
        ("get", 0.066583954),
        ("ids", 0.2868461),
        ("name", 0.43612528),
    ],
    &[
        ("names", 0.37331793),
        ("name", 0.43612528),
        ("from", 0.19554329),
        ("get", 0.066583954),
        ("node", 0.20876107),
        ("type", 0.32289457),
    ],
    &[
        ("types", 0.3580972),
        ("get", 0.066583954),
        ("names", 0.37331793),
        ("with", 0.577805),
        ("node", 0.20876107),
        ("known", 0.62511504),
    ],
    &[
        ("types", 0.28565595),
        ("with", 0.46091795),
        ("unknown", 0.48234496),
        ("mask", 0.63147753),
        ("get", 0.05311436),
        ("node", 0.16880831),
        ("ids", 0.22881858),
    ],
    &[
        ("known", 0.49865744),
        ("ids", 0.22881858),
        ("node", 0.16880831),
        ("get", 0.05311436),
        ("with", 0.46091795),
        ("types", 0.28565595),
        ("mask", 0.63147753),
    ],
    &[
        ("edge", 0.41304797),
        ("types", 0.61368215),
        ("known", 1.0712788),
        ("number", 0.75391865),
        ("get", 0.11410697),
    ],
    &[
        ("rate", 1.3566197),
        ("edge", 0.41304797),
        ("get", 0.11410697),
        ("unknown", 1.0362344),
        ("types", 0.61368215),
    ],
    &[
        ("rate", 1.3566197),
        ("known", 1.0712788),
        ("types", 0.61368215),
        ("get", 0.11410697),
        ("edge", 0.41304797),
    ],
    &[
        ("edge", 0.41304797),
        ("get", 0.11410697),
        ("number", 0.75391865),
        ("minimum", 1.3127872),
        ("types", 0.61368215),
    ],
    &[
        ("number", 0.75391865),
        ("singleton", 0.8732137),
        ("types", 0.61368215),
        ("edge", 0.41304797),
        ("get", 0.11410697),
    ],
    &[
        ("edge", 0.41304797),
        ("ids", 0.49157697),
        ("type", 0.5533544),
        ("get", 0.11410697),
        ("singleton", 0.8732137),
    ],
    &[
        ("get", 0.11410697),
        ("type", 0.5533544),
        ("names", 0.6397664),
        ("edge", 0.41304797),
        ("singleton", 0.8732137),
    ],
    &[
        ("nodes", 1.6288004),
        ("number", 1.5275458),
        ("get", 0.23119685),
    ],
    &[
        ("component", 1.805766),
        ("ids", 0.49157697),
        ("get", 0.11410697),
        ("connected", 1.2097241),
        ("node", 0.19166903),
    ],
    &[
        ("directed", 1.6781572),
        ("number", 1.0458535),
        ("edges", 1.3938692),
        ("get", 0.15829183),
    ],
    &[
        ("edge", 0.5729897),
        ("number", 1.0458535),
        ("types", 0.85131407),
        ("get", 0.15829183),
    ],
    &[
        ("node", 0.26588774),
        ("types", 0.85131407),
        ("get", 0.15829183),
        ("number", 1.0458535),
    ],
    &[
        ("get", 0.23119685),
        ("degrees", 2.4510717),
        ("node", 0.38834855),
    ],
    &[
        ("node", 0.38834855),
        ("get", 0.23119685),
        ("indegrees", 3.34171),
    ],
    &[
        ("degrees", 1.6781572),
        ("weighted", 1.1045554),
        ("get", 0.15829183),
        ("node", 0.26588774),
    ],
    &[
        ("node", 0.19166903),
        ("ids", 0.49157697),
        ("get", 0.11410697),
        ("not", 1.3566197),
        ("singletons", 1.805766),
    ],
    &[
        ("mapping", 2.2879438),
        ("dense", 2.1449723),
        ("get", 0.15829183),
        ("nodes", 1.1151787),
    ],
    &[
        ("number", 1.0458535),
        ("get", 0.15829183),
        ("parallel", 1.9529178),
        ("edges", 1.3938692),
    ],
    &[
        ("get", 0.15829183),
        ("degrees", 1.6781572),
        ("node", 0.26588774),
        ("cumulative", 2.5050006),
    ],
    &[
        ("get", 0.15829183),
        ("sqrt", 2.0381856),
        ("degrees", 1.6781572),
        ("reciprocal", 2.0381856),
    ],
    &[
        ("get", 0.11410697),
        ("nodes", 0.8038928),
        ("number", 0.75391865),
        ("source", 1.1818589),
        ("unique", 1.2097241),
    ],
    &[
        ("type", 0.41575727),
        ("id", 0.3757346),
        ("edge", 0.31033945),
        ("counts", 1.0192826),
        ("hashmap", 1.0192826),
        ("get", 0.08573313),
    ],
    &[
        ("type", 0.41575727),
        ("names", 0.48068208),
        ("counts", 1.0192826),
        ("edge", 0.31033945),
        ("get", 0.08573313),
        ("hashmap", 1.0192826),
    ],
    &[
        ("get", 0.08573313),
        ("hashmap", 1.0192826),
        ("counts", 1.0192826),
        ("type", 0.41575727),
        ("id", 0.3757346),
        ("node", 0.1440086),
    ],
    &[
        ("hashmap", 1.0192826),
        ("type", 0.41575727),
        ("names", 0.48068208),
        ("node", 0.1440086),
        ("get", 0.08573313),
        ("counts", 1.0192826),
    ],
    &[
        ("directed", 2.4510717),
        ("inplace", 2.2093635),
        ("to", 2.3946133),
    ],
    &[("directed", 3.8035252), ("to", 3.7159138)],
    &[
        ("upper", 3.6587372),
        ("to", 2.3946133),
        ("triangular", 3.34171),
    ],
    &[
        ("to", 2.3946133),
        ("triangular", 3.34171),
        ("lower", 3.6587372),
    ],
    &[
        ("diagonal", 3.34171),
        ("main", 3.6587372),
        ("to", 2.3946133),
    ],
    &[
        ("diagonal", 3.34171),
        ("anti", 3.6587372),
        ("to", 2.3946133),
    ],
    &[("to", 3.7159138), ("bidiagonal", 5.6775565)],
    &[("arrowhead", 5.6775565), ("to", 3.7159138)],
    &[("transposed", 5.6775565), ("to", 3.7159138)],
    &[("to", 3.7159138), ("complementary", 5.6775565)],
    &[
        ("approximated", 2.6598926),
        ("get", 0.23119685),
        ("cliques", 3.34171),
    ],
    &[
        ("approximated", 1.8211288),
        ("number", 1.0458535),
        ("cliques", 2.2879438),
        ("get", 0.15829183),
    ],
    &[("report", 7.2425447)],
    &[
        ("textual", 3.34171),
        ("report", 2.85238),
        ("overlap", 3.6587372),
    ],
    &[
        ("get", 0.08573313),
        ("report", 1.0577283),
        ("from", 0.25178045),
        ("id", 0.3757346),
        ("node", 0.26373857),
    ],
    &[
        ("report", 1.0577283),
        ("node", 0.26373857),
        ("from", 0.25178045),
        ("name", 0.56155246),
        ("get", 0.08573313),
    ],
    &[("report", 4.426267), ("textual", 5.1856003)],
    &[
        ("graph", 1.5126685),
        ("random", 1.6040721),
        ("connected", 1.6781572),
        ("generate", 1.6395022),
    ],
    &[
        ("tree", 2.5050006),
        ("generate", 1.6395022),
        ("random", 1.6040721),
        ("spanning", 2.1449723),
    ],
    &[
        ("graph", 2.2093635),
        ("generate", 2.3946133),
        ("star", 3.1328895),
    ],
    &[
        ("wheel", 3.6587372),
        ("generate", 2.3946133),
        ("graph", 2.2093635),
    ],
    &[
        ("generate", 2.3946133),
        ("graph", 2.2093635),
        ("circle", 3.6587372),
    ],
    &[
        ("chain", 3.6587372),
        ("graph", 2.2093635),
        ("generate", 2.3946133),
    ],
    &[
        ("complete", 3.6587372),
        ("graph", 2.2093635),
        ("generate", 2.3946133),
    ],
    &[
        ("generate", 2.3946133),
        ("barbell", 3.6587372),
        ("graph", 2.2093635),
    ],
    &[
        ("generate", 2.3946133),
        ("graph", 2.2093635),
        ("lollipop", 3.6587372),
    ],
    &[
        ("squared", 2.5050006),
        ("graph", 1.5126685),
        ("lattice", 2.5050006),
        ("generate", 1.6395022),
    ],
    &[
        ("from", 0.67897725),
        ("ids", 0.9960044),
        ("filter", 3.34171),
    ],
    &[
        ("names", 1.2962571),
        ("filter", 3.34171),
        ("from", 0.67897725),
    ],
    &[
        ("types", 0.85131407),
        ("drop", 1.6781572),
        ("unknown", 1.4374883),
        ("node", 0.26588774),
    ],
    &[
        ("types", 0.85131407),
        ("drop", 1.6781572),
        ("unknown", 1.4374883),
        ("edge", 0.5729897),
    ],
    &[
        ("nodes", 1.6288004),
        ("singleton", 1.7692543),
        ("drop", 2.4510717),
    ],
    &[("tendrils", 5.1856003), ("drop", 3.8035252)],
    &[
        ("dendritic", 3.34171),
        ("drop", 2.4510717),
        ("trees", 3.34171),
    ],
    &[
        ("nodes", 1.6288004),
        ("isomorphic", 2.295102),
        ("drop", 2.4510717),
    ],
    &[
        ("with", 0.99020207),
        ("drop", 1.2097241),
        ("nodes", 0.8038928),
        ("singleton", 0.8732137),
        ("selfloops", 1.1563187),
    ],
    &[
        ("disconnected", 2.9769197),
        ("drop", 2.4510717),
        ("nodes", 1.6288004),
    ],
    &[("selfloops", 3.6356122), ("drop", 3.8035252)],
    &[
        ("parallel", 2.85238),
        ("edges", 2.0358481),
        ("drop", 2.4510717),
    ],
    &[
        ("arborescence", 2.2879438),
        ("spanning", 2.1449723),
        ("random", 1.6040721),
        ("kruskal", 2.2879438),
    ],
    &[
        ("kruskal", 3.34171),
        ("spanning", 3.1328895),
        ("arborescence", 3.34171),
    ],
    &[
        ("connected", 2.4510717),
        ("get", 0.23119685),
        ("components", 2.9769197),
    ],
    &[("enable", 9.289986)],
    &[("compatible", 5.6775565), ("is", 3.4926748)],
    &[
        ("matrix", 1.8819343),
        ("adjacency", 2.1449723),
        ("same", 2.5050006),
        ("has", 1.055091),
    ],
    &[
        ("set", 1.9529178),
        ("approximated", 1.8211288),
        ("cover", 2.5050006),
        ("vertex", 2.5050006),
    ],
    &[
        ("node", 0.38834855),
        ("get", 0.23119685),
        ("random", 2.3428652),
    ],
    &[
        ("nodes", 1.6288004),
        ("random", 2.3428652),
        ("get", 0.23119685),
    ],
    &[
        ("random", 0.8687885),
        ("first", 0.9319471),
        ("nodes", 0.6039968),
        ("breadth", 0.9319471),
        ("search", 0.9319471),
        ("get", 0.08573313),
    ],
    &[
        ("uniform", 1.3567443),
        ("nodes", 0.6039968),
        ("walk", 1.2391833),
        ("random", 1.5911065),
        ("get", 0.08573313),
    ],
    &[
        ("methods", 2.1449723),
        ("node", 0.26588774),
        ("sampling", 2.5050006),
        ("get", 0.15829183),
    ],
    &[
        ("nodes", 1.6288004),
        ("get", 0.23119685),
        ("subsampled", 3.6587372),
    ],
    &[
        ("propagation", 1.2391833),
        ("okapi", 1.2391833),
        ("bm25", 1.2391833),
        ("node", 0.1440086),
        ("feature", 1.3567443),
        ("get", 0.08573313),
    ],
    &[
        ("propagation", 1.2391833),
        ("get", 0.08573313),
        ("okapi", 1.2391833),
        ("node", 0.1440086),
        ("bm25", 1.2391833),
        ("label", 0.9319471),
    ],
    &[
        ("name", 1.0368124),
        ("default", 2.5050006),
        ("has", 1.055091),
        ("graph", 1.5126685),
    ],
    &[("nodes", 2.5275404), ("has", 2.3913524)],
    &[("edges", 3.1591895), ("has", 2.3913524)],
    &[("trap", 2.7487032), ("has", 1.541038), ("nodes", 1.6288004)],
    &[("directed", 3.8035252), ("is", 3.4926748)],
    &[
        ("weights", 2.2507532),
        ("has", 1.541038),
        ("edge", 0.83689356),
    ],
    &[
        ("representing", 1.805766),
        ("edge", 0.41304797),
        ("probabilities", 1.805766),
        ("has", 0.76057774),
        ("weights", 1.110857),
    ],
    &[
        ("weighted", 1.1045554),
        ("nodes", 1.1151787),
        ("singleton", 1.2113423),
        ("has", 1.055091),
    ],
    &[
        ("constant", 2.5050006),
        ("edge", 0.5729897),
        ("has", 1.055091),
        ("weights", 1.5410066),
    ],
    &[
        ("negative", 2.5050006),
        ("has", 1.055091),
        ("weights", 1.5410066),
        ("edge", 0.5729897),
    ],
    &[
        ("edge", 0.83689356),
        ("has", 1.541038),
        ("types", 1.2434068),
    ],
    &[("has", 2.3913524), ("selfloops", 3.6356122)],
    &[
        ("nodes", 1.6288004),
        ("has", 1.541038),
        ("disconnected", 2.9769197),
    ],
    &[
        ("has", 1.541038),
        ("nodes", 1.6288004),
        ("singleton", 1.7692543),
    ],
    &[
        ("singleton", 0.8732137),
        ("with", 0.99020207),
        ("nodes", 0.8038928),
        ("has", 0.76057774),
        ("selfloops", 1.1563187),
    ],
    &[("connected", 3.8035252), ("is", 3.4926748)],
    &[
        ("node", 0.38834855),
        ("has", 1.541038),
        ("types", 1.2434068),
    ],
    &[
        ("types", 0.85131407),
        ("multilabel", 2.2879438),
        ("node", 0.26588774),
        ("has", 1.055091),
    ],
    &[
        ("types", 0.85131407),
        ("unknown", 1.4374883),
        ("has", 1.055091),
        ("node", 0.26588774),
    ],
    &[
        ("node", 0.26588774),
        ("known", 1.4861028),
        ("has", 1.055091),
        ("types", 0.85131407),
    ],
    &[
        ("has", 1.055091),
        ("unknown", 1.4374883),
        ("types", 0.85131407),
        ("edge", 0.5729897),
    ],
    &[
        ("edge", 0.5729897),
        ("has", 1.055091),
        ("types", 0.85131407),
        ("known", 1.4861028),
    ],
    &[
        ("has", 1.055091),
        ("homogeneous", 1.8211288),
        ("node", 0.26588774),
        ("types", 0.85131407),
    ],
    &[
        ("has", 1.055091),
        ("edge", 0.5729897),
        ("homogeneous", 1.8211288),
        ("types", 0.85131407),
    ],
    &[
        ("has", 1.055091),
        ("types", 0.85131407),
        ("node", 0.26588774),
        ("singleton", 1.2113423),
    ],
    &[
        ("has", 1.541038),
        ("node", 0.38834855),
        ("oddities", 3.1328895),
    ],
    &[
        ("node", 0.26588774),
        ("types", 0.85131407),
        ("has", 1.055091),
        ("oddities", 2.1449723),
    ],
    &[
        ("singleton", 1.2113423),
        ("edge", 0.5729897),
        ("has", 1.055091),
        ("types", 0.85131407),
    ],
    &[
        ("edge", 0.5729897),
        ("oddities", 2.1449723),
        ("types", 0.85131407),
        ("has", 1.055091),
    ],
    &[("multigraph", 4.8615565), ("is", 3.4926748)],
    &[
        ("degree", 0.45443597),
        ("has", 0.3540327),
        ("node", 0.08921785),
        ("outbound", 0.68390715),
        ("decreasing", 0.76771283),
        ("by", 0.61107445),
        ("sorted", 0.71973914),
        ("nodes", 0.37419492),
    ],
    &[
        ("has", 0.57145244),
        ("sorted", 1.1617477),
        ("nodes", 0.6039968),
        ("by", 0.9863495),
        ("lexicographic", 1.2391833),
        ("order", 1.2391833),
    ],
    &[
        ("matrix", 2.7487032),
        ("contains", 3.34171),
        ("identity", 3.34171),
    ],
    &[
        ("degree", 0.45443597),
        ("node", 0.08921785),
        ("has", 0.3540327),
        ("outbound", 0.68390715),
        ("nodes", 0.37419492),
        ("by", 0.61107445),
        ("increasing", 0.76771283),
        ("sorted", 0.71973914),
    ],
    &[
        ("dendritic", 3.34171),
        ("trees", 3.34171),
        ("get", 0.23119685),
    ],
    &[
        ("get", 0.23119685),
        ("transitive", 3.6587372),
        ("closure", 3.6587372),
    ],
    &[
        ("paths", 2.2879438),
        ("shortest", 1.4861028),
        ("all", 1.8211288),
        ("get", 0.15829183),
    ],
    &[
        ("shortest", 1.0712788),
        ("paths", 1.6492976),
        ("get", 0.11410697),
        ("weighted", 0.7962348),
        ("all", 1.3127872),
    ],
    &[
        ("get", 0.066583954),
        ("id", 0.29181132),
        ("from", 0.19554329),
        ("weight", 0.689641),
        ("edge", 0.4498814),
        ("unchecked", 0.35568178),
    ],
    &[
        ("from", 0.19554329),
        ("ids", 0.2868461),
        ("get", 0.066583954),
        ("node", 0.11184315),
        ("weight", 0.689641),
        ("unchecked", 0.35568178),
        ("edge", 0.24102269),
    ],
    &[
        ("from", 0.19554329),
        ("node", 0.20876107),
        ("name", 0.43612528),
        ("get", 0.066583954),
        ("id", 0.29181132),
        ("unchecked", 0.35568178),
    ],
    &[
        ("name", 0.28367162),
        ("from", 0.12718841),
        ("type", 0.4013794),
        ("get", 0.043308605),
        ("edge", 0.29960722),
        ("id", 0.1898046),
        ("unchecked", 0.23134826),
    ],
    &[
        ("edge", 0.29960722),
        ("from", 0.12718841),
        ("type", 0.4013794),
        ("get", 0.043308605),
        ("unchecked", 0.23134826),
        ("id", 0.1898046),
        ("name", 0.28367162),
    ],
    &[
        ("unchecked", 0.28372917),
        ("get", 0.05311436),
        ("type", 0.25757465),
        ("edge", 0.36378297),
        ("count", 0.63147753),
        ("from", 0.15598588),
        ("id", 0.23277935),
    ],
    &[
        ("type", 0.14705014),
        ("id", 0.25740784),
        ("and", 0.29520264),
        ("get", 0.030323148),
        ("node", 0.050934736),
        ("ids", 0.13063322),
        ("from", 0.089052804),
        ("edge", 0.21260701),
        ("unchecked", 0.16198184),
    ],
    &[
        ("ids", 0.43294567),
        ("edge", 0.19226502),
        ("unchecked", 0.28372917),
        ("node", 0.08921785),
        ("get", 0.05311436),
        ("minmax", 0.68390715),
        ("from", 0.15598588),
    ],
    &[
        ("id", 0.29181132),
        ("from", 0.19554329),
        ("get", 0.066583954),
        ("edge", 0.24102269),
        ("node", 0.11184315),
        ("ids", 0.2868461),
        ("unchecked", 0.35568178),
    ],
    &[
        ("node", 0.11184315),
        ("edge", 0.24102269),
        ("names", 0.37331793),
        ("id", 0.29181132),
        ("unchecked", 0.35568178),
        ("get", 0.066583954),
        ("from", 0.19554329),
    ],
    &[
        ("get", 0.05311436),
        ("from", 0.15598588),
        ("id", 0.4404398),
        ("unchecked", 0.28372917),
        ("edge", 0.19226502),
        ("node", 0.08921785),
        ("source", 0.5501301),
    ],
    &[
        ("id", 0.4404398),
        ("node", 0.08921785),
        ("from", 0.15598588),
        ("get", 0.05311436),
        ("unchecked", 0.28372917),
        ("destination", 0.61107445),
        ("edge", 0.19226502),
    ],
    &[
        ("edge", 0.24102269),
        ("from", 0.19554329),
        ("get", 0.066583954),
        ("id", 0.544681),
        ("source", 0.689641),
        ("node", 0.11184315),
    ],
    &[
        ("from", 0.19554329),
        ("get", 0.066583954),
        ("node", 0.11184315),
        ("id", 0.544681),
        ("destination", 0.76604056),
        ("edge", 0.24102269),
    ],
    &[
        ("node", 0.08921785),
        ("edge", 0.19226502),
        ("source", 0.5501301),
        ("get", 0.05311436),
        ("name", 0.34789935),
        ("id", 0.23277935),
        ("from", 0.15598588),
        ("unchecked", 0.28372917),
    ],
    &[
        ("get", 0.05311436),
        ("from", 0.15598588),
        ("destination", 0.61107445),
        ("name", 0.34789935),
        ("node", 0.08921785),
        ("id", 0.23277935),
        ("edge", 0.19226502),
        ("unchecked", 0.28372917),
    ],
    &[
        ("from", 0.19554329),
        ("node", 0.11184315),
        ("edge", 0.24102269),
        ("source", 0.689641),
        ("get", 0.066583954),
        ("name", 0.43612528),
        ("id", 0.29181132),
    ],
    &[
        ("from", 0.19554329),
        ("node", 0.11184315),
        ("id", 0.29181132),
        ("edge", 0.24102269),
        ("get", 0.066583954),
        ("destination", 0.76604056),
        ("name", 0.43612528),
    ],
    &[
        ("names", 0.48068208),
        ("from", 0.25178045),
        ("id", 0.3757346),
        ("edge", 0.31033945),
        ("node", 0.1440086),
        ("get", 0.08573313),
    ],
    &[
        ("ids", 0.36934143),
        ("edge", 0.31033945),
        ("id", 0.3757346),
        ("get", 0.08573313),
        ("from", 0.25178045),
        ("node", 0.1440086),
    ],
    &[
        ("edge", 0.24102269),
        ("from", 0.19554329),
        ("get", 0.066583954),
        ("node", 0.11184315),
        ("ids", 0.2868461),
        ("id", 0.29181132),
        ("unchecked", 0.35568178),
    ],
    &[
        ("id", 0.3757346),
        ("node", 0.1440086),
        ("get", 0.08573313),
        ("edge", 0.31033945),
        ("from", 0.25178045),
        ("ids", 0.36934143),
    ],
    &[
        ("get", 0.08573313),
        ("unique", 0.908914),
        ("node", 0.1440086),
        ("id", 0.3757346),
        ("source", 0.8879779),
        ("unchecked", 0.45797387),
    ],
    &[
        ("from", 0.089052804),
        ("get", 0.030323148),
        ("ids", 0.13063322),
        ("unchecked", 0.16198184),
        ("id", 0.25740784),
        ("edge", 0.21260701),
        ("node", 0.050934736),
        ("and", 0.29520264),
        ("type", 0.14705014),
    ],
    &[
        ("id", 0.3034964),
        ("get", 0.035962123),
        ("type", 0.174396),
        ("from", 0.10561331),
        ("ids", 0.15492614),
        ("edge", 0.25067404),
        ("and", 0.35009935),
        ("node", 0.060406704),
    ],
    &[
        ("node", 0.032796927),
        ("weight", 0.20223057),
        ("from", 0.057341184),
        ("edge", 0.20349996),
        ("ids", 0.08411486),
        ("get", 0.019525103),
        ("type", 0.094685726),
        ("id", 0.1676274),
        ("unchecked", 0.10430025),
        ("and", 0.3723561),
    ],
    &[
        ("get", 0.022380052),
        ("node", 0.03759247),
        ("weight", 0.2318006),
        ("type", 0.10853061),
        ("from", 0.06572557),
        ("edge", 0.23189111),
        ("id", 0.1915626),
        ("ids", 0.096414074),
        ("and", 0.4255241),
    ],
    &[
        ("central", 1.0577283),
        ("ids", 0.36934143),
        ("k", 0.9863495),
        ("get", 0.08573313),
        ("node", 0.1440086),
        ("top", 1.1617477),
    ],
    &[
        ("top", 0.9022622),
        ("get", 0.066583954),
        ("ids", 0.2868461),
        ("central", 0.82147634),
        ("node", 0.11184315),
        ("weighted", 0.4646207),
        ("k", 0.76604056),
    ],
    &[
        ("from", 0.19554329),
        ("node", 0.20876107),
        ("get", 0.066583954),
        ("id", 0.29181132),
        ("unchecked", 0.35568178),
        ("degree", 0.5696792),
    ],
    &[
        ("unchecked", 0.28372917),
        ("weighted", 0.3706303),
        ("get", 0.05311436),
        ("from", 0.15598588),
        ("degree", 0.45443597),
        ("node", 0.16880831),
        ("id", 0.23277935),
    ],
    &[
        ("node", 0.26373857),
        ("id", 0.3757346),
        ("degree", 0.73351574),
        ("get", 0.08573313),
        ("from", 0.25178045),
    ],
    &[
        ("unchecked", 0.28372917),
        ("node", 0.16880831),
        ("comulative", 0.76771283),
        ("degree", 0.45443597),
        ("from", 0.15598588),
        ("id", 0.23277935),
        ("get", 0.05311436),
    ],
    &[
        ("degree", 0.5696792),
        ("from", 0.19554329),
        ("get", 0.066583954),
        ("node", 0.20876107),
        ("comulative", 0.96240187),
        ("id", 0.29181132),
    ],
    &[
        ("from", 0.15598588),
        ("degree", 0.45443597),
        ("reciprocal", 0.68390715),
        ("sqrt", 0.68390715),
        ("get", 0.05311436),
        ("id", 0.23277935),
        ("node", 0.08921785),
        ("unchecked", 0.28372917),
    ],
    &[
        ("degree", 0.5696792),
        ("from", 0.19554329),
        ("node", 0.11184315),
        ("get", 0.066583954),
        ("sqrt", 0.8573434),
        ("id", 0.29181132),
        ("reciprocal", 0.8573434),
    ],
    &[
        ("node", 0.08921785),
        ("from", 0.15598588),
        ("sqrt", 0.68390715),
        ("degrees", 0.5631007),
        ("ids", 0.22881858),
        ("unchecked", 0.28372917),
        ("get", 0.05311436),
        ("reciprocal", 0.68390715),
    ],
    &[
        ("weighted", 0.4646207),
        ("from", 0.19554329),
        ("degree", 0.5696792),
        ("id", 0.29181132),
        ("node", 0.20876107),
        ("get", 0.066583954),
    ],
    &[
        ("name", 0.56155246),
        ("node", 0.26373857),
        ("get", 0.08573313),
        ("degree", 0.73351574),
        ("from", 0.25178045),
    ],
    &[
        ("top", 1.1617477),
        ("k", 0.9863495),
        ("central", 1.0577283),
        ("node", 0.1440086),
        ("names", 0.48068208),
        ("get", 0.08573313),
    ],
    &[
        ("get", 0.05311436),
        ("node", 0.16880831),
        ("from", 0.15598588),
        ("id", 0.23277935),
        ("unchecked", 0.28372917),
        ("type", 0.25757465),
        ("ids", 0.22881858),
    ],
    &[
        ("get", 0.066583954),
        ("type", 0.32289457),
        ("id", 0.29181132),
        ("node", 0.20876107),
        ("ids", 0.2868461),
        ("from", 0.19554329),
    ],
    &[
        ("unchecked", 0.28372917),
        ("get", 0.05311436),
        ("type", 0.25757465),
        ("id", 0.4404398),
        ("edge", 0.36378297),
        ("from", 0.15598588),
    ],
    &[
        ("from", 0.19554329),
        ("get", 0.066583954),
        ("edge", 0.4498814),
        ("id", 0.544681),
        ("type", 0.32289457),
    ],
    &[
        ("id", 0.23277935),
        ("unchecked", 0.28372917),
        ("node", 0.16880831),
        ("type", 0.25757465),
        ("get", 0.05311436),
        ("from", 0.15598588),
        ("names", 0.29779762),
    ],
    &[
        ("from", 0.19554329),
        ("id", 0.29181132),
        ("type", 0.32289457),
        ("node", 0.20876107),
        ("get", 0.066583954),
        ("names", 0.37331793),
    ],
    &[
        ("get", 0.066583954),
        ("name", 0.43612528),
        ("names", 0.37331793),
        ("node", 0.20876107),
        ("from", 0.19554329),
        ("type", 0.32289457),
    ],
    &[
        ("id", 0.29181132),
        ("from", 0.19554329),
        ("edge", 0.4498814),
        ("get", 0.066583954),
        ("name", 0.43612528),
        ("type", 0.32289457),
    ],
    &[
        ("from", 0.15598588),
        ("edge", 0.36378297),
        ("type", 0.48735476),
        ("get", 0.05311436),
        ("name", 0.34789935),
        ("id", 0.23277935),
    ],
    &[
        ("edge", 0.5683583),
        ("from", 0.25178045),
        ("get", 0.08573313),
        ("id", 0.3757346),
        ("weight", 0.8879779),
    ],
    &[
        ("node", 0.1440086),
        ("weight", 0.8879779),
        ("get", 0.08573313),
        ("edge", 0.31033945),
        ("ids", 0.36934143),
        ("from", 0.25178045),
    ],
    &[
        ("weight", 0.37247643),
        ("from", 0.10561331),
        ("node", 0.060406704),
        ("ids", 0.15492614),
        ("type", 0.174396),
        ("edge", 0.25067404),
        ("and", 0.35009935),
        ("id", 0.15760785),
        ("get", 0.035962123),
    ],
    &[
        ("from", 0.10561331),
        ("node", 0.060406704),
        ("weight", 0.37247643),
        ("type", 0.174396),
        ("get", 0.035962123),
        ("name", 0.23555212),
        ("and", 0.35009935),
        ("edge", 0.25067404),
        ("names", 0.20162974),
    ],
    &[
        ("get", 0.08573313),
        ("weight", 0.8879779),
        ("names", 0.48068208),
        ("node", 0.1440086),
        ("from", 0.25178045),
        ("edge", 0.31033945),
    ],
    &[
        ("from", 0.19554329),
        ("node", 0.20876107),
        ("get", 0.066583954),
        ("unchecked", 0.35568178),
        ("name", 0.43612528),
        ("id", 0.29181132),
    ],
    &[
        ("node", 0.26373857),
        ("id", 0.3757346),
        ("name", 0.56155246),
        ("get", 0.08573313),
        ("from", 0.25178045),
    ],
    &[
        ("node", 0.26373857),
        ("name", 0.56155246),
        ("get", 0.08573313),
        ("id", 0.3757346),
        ("from", 0.25178045),
    ],
    &[
        ("get", 0.08573313),
        ("node", 0.26373857),
        ("from", 0.25178045),
        ("names", 0.48068208),
        ("ids", 0.36934143),
    ],
    &[
        ("ids", 0.22881858),
        ("edge", 0.36378297),
        ("node", 0.16880831),
        ("from", 0.15598588),
        ("get", 0.05311436),
        ("names", 0.29779762),
    ],
    &[
        ("get", 0.05311436),
        ("node", 0.16880831),
        ("ids", 0.22881858),
        ("names", 0.29779762),
        ("from", 0.15598588),
        ("edge", 0.36378297),
    ],
    &[
        ("node", 0.20876107),
        ("from", 0.19554329),
        ("name", 0.43612528),
        ("ids", 0.2868461),
        ("type", 0.32289457),
        ("get", 0.066583954),
    ],
    &[
        ("node", 0.20876107),
        ("type", 0.32289457),
        ("get", 0.066583954),
        ("name", 0.8140506),
        ("from", 0.19554329),
    ],
    &[
        ("from", 0.19554329),
        ("get", 0.066583954),
        ("id", 0.29181132),
        ("edge", 0.4498814),
        ("type", 0.32289457),
        ("count", 0.79161775),
    ],
    &[
        ("type", 0.48735476),
        ("name", 0.34789935),
        ("edge", 0.36378297),
        ("id", 0.23277935),
        ("from", 0.15598588),
        ("get", 0.05311436),
    ],
    &[
        ("type", 0.32289457),
        ("get", 0.066583954),
        ("edge", 0.4498814),
        ("from", 0.19554329),
        ("name", 0.43612528),
        ("count", 0.79161775),
    ],
    &[
        ("get", 0.05311436),
        ("from", 0.15598588),
        ("node", 0.16880831),
        ("name", 0.34789935),
        ("type", 0.48735476),
        ("id", 0.23277935),
    ],
    &[
        ("count", 0.79161775),
        ("from", 0.19554329),
        ("type", 0.32289457),
        ("id", 0.29181132),
        ("get", 0.066583954),
        ("node", 0.20876107),
    ],
    &[
        ("get", 0.066583954),
        ("from", 0.19554329),
        ("count", 0.79161775),
        ("type", 0.32289457),
        ("name", 0.43612528),
        ("node", 0.20876107),
    ],
    &[
        ("ids", 0.2868461),
        ("from", 0.19554329),
        ("neighbour", 0.9022622),
        ("node", 0.20876107),
        ("id", 0.29181132),
        ("get", 0.066583954),
    ],
    &[
        ("from", 0.19554329),
        ("get", 0.066583954),
        ("name", 0.43612528),
        ("neighbour", 0.9022622),
        ("node", 0.20876107),
        ("ids", 0.2868461),
    ],
    &[
        ("from", 0.19554329),
        ("node", 0.20876107),
        ("name", 0.43612528),
        ("get", 0.066583954),
        ("names", 0.37331793),
        ("neighbour", 0.9022622),
    ],
    &[
        ("ids", 0.5354132),
        ("edge", 0.24102269),
        ("node", 0.11184315),
        ("minmax", 0.8573434),
        ("get", 0.066583954),
        ("from", 0.19554329),
    ],
    &[
        ("edge", 0.25067404),
        ("get", 0.035962123),
        ("ids", 0.15492614),
        ("node", 0.060406704),
        ("and", 0.35009935),
        ("id", 0.3034964),
        ("type", 0.174396),
        ("from", 0.10561331),
    ],
    &[
        ("edge", 0.31033945),
        ("id", 0.3757346),
        ("get", 0.08573313),
        ("node", 0.1440086),
        ("from", 0.25178045),
        ("names", 0.48068208),
    ],
    &[
        ("node", 0.060406704),
        ("name", 0.23555212),
        ("and", 0.35009935),
        ("id", 0.15760785),
        ("type", 0.174396),
        ("names", 0.20162974),
        ("edge", 0.25067404),
        ("from", 0.10561331),
        ("get", 0.035962123),
    ],
    &[
        ("get", 0.05311436),
        ("ids", 0.22881858),
        ("type", 0.48735476),
        ("from", 0.15598588),
        ("edge", 0.36378297),
        ("names", 0.29779762),
    ],
    &[
        ("names", 0.29779762),
        ("type", 0.48735476),
        ("from", 0.15598588),
        ("ids", 0.22881858),
        ("node", 0.16880831),
        ("get", 0.05311436),
    ],
    &[
        ("ids", 0.18657504),
        ("type", 0.4013794),
        ("from", 0.12718841),
        ("names", 0.24281944),
        ("get", 0.043308605),
        ("node", 0.13902846),
        ("multiple", 0.6853675),
    ],
    &[
        ("unchecked", 0.23134826),
        ("node", 0.07274681),
        ("minmax", 0.55764705),
        ("edge", 0.15676984),
        ("source", 0.44856736),
        ("get", 0.043308605),
        ("ids", 0.18657504),
        ("from", 0.12718841),
        ("id", 0.1898046),
    ],
    &[
        ("get", 0.05311436),
        ("minmax", 0.68390715),
        ("from", 0.15598588),
        ("source", 0.5501301),
        ("ids", 0.22881858),
        ("id", 0.23277935),
        ("edge", 0.19226502),
        ("node", 0.08921785),
    ],
    &[
        ("id", 0.23277935),
        ("get", 0.05311436),
        ("type", 0.48735476),
        ("from", 0.15598588),
        ("name", 0.34789935),
        ("node", 0.16880831),
    ],
    &[
        ("names", 0.24281944),
        ("type", 0.4013794),
        ("ids", 0.18657504),
        ("node", 0.13902846),
        ("unchecked", 0.23134826),
        ("from", 0.12718841),
        ("get", 0.043308605),
    ],
    &[
        ("from", 0.12718841),
        ("unchecked", 0.23134826),
        ("number", 0.28614524),
        ("get", 0.043308605),
        ("type", 0.21002229),
        ("node", 0.07274681),
        ("of", 0.48370937),
        ("id", 0.1898046),
        ("nodes", 0.3051126),
    ],
    &[
        ("id", 0.23277935),
        ("get", 0.05311436),
        ("number", 0.35093305),
        ("node", 0.08921785),
        ("type", 0.25757465),
        ("nodes", 0.37419492),
        ("from", 0.15598588),
        ("of", 0.5932288),
    ],
    &[
        ("from", 0.15598588),
        ("nodes", 0.37419492),
        ("number", 0.35093305),
        ("type", 0.25757465),
        ("get", 0.05311436),
        ("node", 0.08921785),
        ("of", 0.5932288),
        ("name", 0.34789935),
    ],
    &[
        ("number", 0.28614524),
        ("get", 0.043308605),
        ("of", 0.48370937),
        ("type", 0.21002229),
        ("edge", 0.15676984),
        ("edges", 0.38136226),
        ("from", 0.12718841),
        ("id", 0.1898046),
        ("unchecked", 0.23134826),
    ],
    &[
        ("id", 0.23277935),
        ("from", 0.15598588),
        ("get", 0.05311436),
        ("of", 0.5932288),
        ("edges", 0.46770868),
        ("type", 0.25757465),
        ("number", 0.35093305),
        ("edge", 0.19226502),
    ],
    &[
        ("number", 0.35093305),
        ("edges", 0.46770868),
        ("from", 0.15598588),
        ("edge", 0.19226502),
        ("get", 0.05311436),
        ("name", 0.34789935),
        ("type", 0.25757465),
        ("of", 0.5932288),
    ],
    &[
        ("type", 0.174396),
        ("counts", 0.4275543),
        ("id", 0.15760785),
        ("hashmap", 0.4275543),
        ("node", 0.11632172),
        ("unchecked", 0.19210443),
        ("from", 0.10561331),
        ("get", 0.035962123),
        ("ids", 0.15492614),
    ],
    &[
        ("get", 0.035962123),
        ("type", 0.174396),
        ("from", 0.10561331),
        ("hashmap", 0.4275543),
        ("unchecked", 0.19210443),
        ("edge", 0.13017683),
        ("node", 0.060406704),
        ("counts", 0.4275543),
        ("ids", 0.15492614),
        ("id", 0.15760785),
    ],
    &[("tendrils", 5.1856003), ("get", 0.35876673)],
    &[
        ("threshold", 1.3567443),
        ("get", 0.08573313),
        ("degree", 0.73351574),
        ("node", 0.1440086),
        ("distribution", 1.3567443),
        ("geometric", 1.3567443),
    ],
    &[
        ("get", 0.11410697),
        ("weighting", 1.6492976),
        ("edge", 0.41304797),
        ("methods", 1.5462344),
        ("sparse", 1.805766),
    ],
    &[
        ("edge", 0.5729897),
        ("weighting", 2.2879438),
        ("get", 0.15829183),
        ("methods", 2.1449723),
    ],
    &[("selfloops", 3.6356122), ("add", 5.6775565)],
    &[
        ("get", 0.23119685),
        ("centrality", 2.0995572),
        ("degree", 1.9780747),
    ],
    &[
        ("centrality", 1.4374883),
        ("degree", 1.3543137),
        ("get", 0.15829183),
        ("weighted", 1.1045554),
    ],
    &[
        ("node", 0.11184315),
        ("id", 0.29181132),
        ("from", 0.19554329),
        ("centrality", 0.6046658),
        ("get", 0.066583954),
        ("unchecked", 0.35568178),
        ("closeness", 0.8573434),
    ],
    &[
        ("centrality", 0.48234496),
        ("from", 0.15598588),
        ("id", 0.23277935),
        ("closeness", 0.68390715),
        ("node", 0.08921785),
        ("unchecked", 0.28372917),
        ("get", 0.05311436),
        ("weighted", 0.3706303),
    ],
    &[
        ("closeness", 2.9769197),
        ("centrality", 2.0995572),
        ("get", 0.23119685),
    ],
    &[
        ("centrality", 1.4374883),
        ("closeness", 2.0381856),
        ("get", 0.15829183),
        ("weighted", 1.1045554),
    ],
    &[
        ("harmonic", 0.8573434),
        ("centrality", 0.6046658),
        ("unchecked", 0.35568178),
        ("node", 0.11184315),
        ("id", 0.29181132),
        ("from", 0.19554329),
        ("get", 0.066583954),
    ],
    &[
        ("unchecked", 0.28372917),
        ("node", 0.08921785),
        ("weighted", 0.3706303),
        ("get", 0.05311436),
        ("centrality", 0.48234496),
        ("from", 0.15598588),
        ("id", 0.23277935),
        ("harmonic", 0.68390715),
    ],
    &[
        ("get", 0.23119685),
        ("harmonic", 2.9769197),
        ("centrality", 2.0995572),
    ],
    &[
        ("weighted", 1.1045554),
        ("get", 0.15829183),
        ("centrality", 1.4374883),
        ("harmonic", 2.0381856),
    ],
    &[
        ("centrality", 2.0995572),
        ("stress", 3.6587372),
        ("get", 0.23119685),
    ],
    &[
        ("get", 0.23119685),
        ("centrality", 2.0995572),
        ("betweenness", 2.85238),
    ],
    &[
        ("id", 0.29181132),
        ("from", 0.19554329),
        ("betweenness", 0.82147634),
        ("node", 0.11184315),
        ("get", 0.066583954),
        ("centrality", 0.6046658),
        ("approximated", 0.76604056),
    ],
    &[
        ("centrality", 0.6046658),
        ("node", 0.11184315),
        ("from", 0.19554329),
        ("name", 0.43612528),
        ("betweenness", 0.82147634),
        ("get", 0.066583954),
        ("approximated", 0.76604056),
    ],
    &[
        ("centrality", 0.48234496),
        ("from", 0.15598588),
        ("betweenness", 0.65529585),
        ("weighted", 0.3706303),
        ("get", 0.05311436),
        ("id", 0.23277935),
        ("node", 0.08921785),
        ("approximated", 0.61107445),
    ],
    &[
        ("name", 0.34789935),
        ("centrality", 0.48234496),
        ("betweenness", 0.65529585),
        ("approximated", 0.61107445),
        ("node", 0.08921785),
        ("weighted", 0.3706303),
        ("get", 0.05311436),
        ("from", 0.15598588),
    ],
    &[
        ("centrality", 2.0995572),
        ("eigenvector", 3.34171),
        ("get", 0.23119685),
    ],
    &[
        ("eigenvector", 2.2879438),
        ("get", 0.15829183),
        ("weighted", 1.1045554),
        ("centrality", 1.4374883),
    ],
    &[("to", 3.7159138), ("dot", 5.6775565)],
    &[("stars", 5.6775565), ("get", 0.35876673)],
    &[
        ("community", 1.5462344),
        ("undirected", 1.4077892),
        ("louvain", 1.805766),
        ("get", 0.11410697),
        ("detection", 1.805766),
    ],
    &[
        ("get", 0.066583954),
        ("node", 0.11184315),
        ("from", 0.19554329),
        ("directed", 0.7059009),
        ("memberships", 0.96240187),
        ("modularity", 0.96240187),
        ("community", 0.9022622),
    ],
    &[
        ("modularity", 0.96240187),
        ("undirected", 0.82147634),
        ("get", 0.066583954),
        ("memberships", 0.96240187),
        ("from", 0.19554329),
        ("community", 0.9022622),
        ("node", 0.11184315),
    ],
    &[
        ("preferential", 1.2097241),
        ("attachment", 1.2097241),
        ("minimum", 1.3127872),
        ("get", 0.11410697),
        ("unchecked", 0.6095428),
    ],
    &[
        ("maximum", 1.274449),
        ("get", 0.11410697),
        ("attachment", 1.2097241),
        ("preferential", 1.2097241),
        ("unchecked", 0.6095428),
    ],
    &[
        ("attachment", 0.908914),
        ("weighted", 0.598243),
        ("get", 0.08573313),
        ("minimum", 0.9863495),
        ("preferential", 0.908914),
        ("unchecked", 0.45797387),
    ],
    &[
        ("preferential", 0.908914),
        ("weighted", 0.598243),
        ("maximum", 0.9575445),
        ("get", 0.08573313),
        ("attachment", 0.908914),
        ("unchecked", 0.45797387),
    ],
    &[
        ("preferential", 0.7059009),
        ("get", 0.066583954),
        ("node", 0.11184315),
        ("attachment", 0.7059009),
        ("ids", 0.2868461),
        ("unchecked", 0.35568178),
        ("from", 0.19554329),
    ],
    &[
        ("preferential", 0.908914),
        ("attachment", 0.908914),
        ("get", 0.08573313),
        ("node", 0.1440086),
        ("ids", 0.36934143),
        ("from", 0.25178045),
    ],
    &[
        ("get", 0.08573313),
        ("preferential", 0.908914),
        ("node", 0.1440086),
        ("names", 0.48068208),
        ("attachment", 0.908914),
        ("from", 0.25178045),
    ],
    &[
        ("preferential", 0.5631007),
        ("attachment", 0.5631007),
        ("node", 0.08921785),
        ("unchecked", 0.28372917),
        ("from", 0.15598588),
        ("ids", 0.22881858),
        ("get", 0.05311436),
        ("weighted", 0.3706303),
    ],
    &[
        ("from", 0.19554329),
        ("preferential", 0.7059009),
        ("get", 0.066583954),
        ("ids", 0.2868461),
        ("node", 0.11184315),
        ("weighted", 0.4646207),
        ("attachment", 0.7059009),
    ],
    &[
        ("attachment", 0.7059009),
        ("get", 0.066583954),
        ("names", 0.37331793),
        ("node", 0.11184315),
        ("from", 0.19554329),
        ("weighted", 0.4646207),
        ("preferential", 0.7059009),
    ],
    &[
        ("jaccard", 0.9022622),
        ("coefficient", 0.79161775),
        ("ids", 0.2868461),
        ("unchecked", 0.35568178),
        ("from", 0.19554329),
        ("node", 0.11184315),
        ("get", 0.066583954),
    ],
    &[
        ("ids", 0.36934143),
        ("node", 0.1440086),
        ("from", 0.25178045),
        ("coefficient", 1.0192826),
        ("get", 0.08573313),
        ("jaccard", 1.1617477),
    ],
    &[
        ("jaccard", 1.1617477),
        ("coefficient", 1.0192826),
        ("from", 0.25178045),
        ("node", 0.1440086),
        ("names", 0.48068208),
        ("get", 0.08573313),
    ],
    &[
        ("get", 0.05311436),
        ("index", 0.57737046),
        ("from", 0.15598588),
        ("ids", 0.22881858),
        ("adar", 0.71973914),
        ("unchecked", 0.28372917),
        ("adamic", 0.71973914),
        ("node", 0.08921785),
    ],
    &[
        ("adar", 0.9022622),
        ("from", 0.19554329),
        ("adamic", 0.9022622),
        ("index", 0.72378933),
        ("ids", 0.2868461),
        ("get", 0.066583954),
        ("node", 0.11184315),
    ],
    &[
        ("adar", 0.9022622),
        ("node", 0.11184315),
        ("index", 0.72378933),
        ("from", 0.19554329),
        ("names", 0.37331793),
        ("get", 0.066583954),
        ("adamic", 0.9022622),
    ],
    &[
        ("index", 0.57737046),
        ("allocation", 0.63147753),
        ("ids", 0.22881858),
        ("node", 0.08921785),
        ("from", 0.15598588),
        ("get", 0.05311436),
        ("unchecked", 0.28372917),
        ("resource", 0.63147753),
    ],
    &[
        ("resource", 0.51489675),
        ("weighted", 0.30220607),
        ("from", 0.12718841),
        ("unchecked", 0.23134826),
        ("node", 0.07274681),
        ("index", 0.4707787),
        ("get", 0.043308605),
        ("ids", 0.18657504),
        ("allocation", 0.51489675),
    ],
    &[
        ("resource", 0.79161775),
        ("from", 0.19554329),
        ("allocation", 0.79161775),
        ("index", 0.72378933),
        ("ids", 0.2868461),
        ("node", 0.11184315),
        ("get", 0.066583954),
    ],
    &[
        ("allocation", 0.79161775),
        ("index", 0.72378933),
        ("node", 0.11184315),
        ("get", 0.066583954),
        ("resource", 0.79161775),
        ("names", 0.37331793),
        ("from", 0.19554329),
    ],
    &[
        ("allocation", 0.63147753),
        ("ids", 0.22881858),
        ("node", 0.08921785),
        ("from", 0.15598588),
        ("weighted", 0.3706303),
        ("index", 0.57737046),
        ("resource", 0.63147753),
        ("get", 0.05311436),
    ],
    &[
        ("from", 0.15598588),
        ("get", 0.05311436),
        ("names", 0.29779762),
        ("weighted", 0.3706303),
        ("allocation", 0.63147753),
        ("resource", 0.63147753),
        ("index", 0.57737046),
        ("node", 0.08921785),
    ],
    &[
        ("all", 0.61107445),
        ("ids", 0.22881858),
        ("get", 0.05311436),
        ("from", 0.15598588),
        ("edge", 0.19226502),
        ("unchecked", 0.28372917),
        ("metrics", 0.84054554),
        ("node", 0.08921785),
    ],
    &[
        ("node", 0.19166903),
        ("ids", 0.49157697),
        ("isomorphic", 1.1327451),
        ("groups", 1.24038),
        ("get", 0.11410697),
    ],
    &[
        ("groups", 1.24038),
        ("node", 0.19166903),
        ("isomorphic", 1.1327451),
        ("get", 0.11410697),
        ("names", 0.6397664),
    ],
    &[
        ("get", 0.11410697),
        ("isomorphic", 1.1327451),
        ("node", 0.19166903),
        ("number", 0.75391865),
        ("groups", 1.24038),
    ],
    &[
        ("node", 0.1440086),
        ("type", 0.41575727),
        ("isomorphic", 0.8510767),
        ("ids", 0.36934143),
        ("groups", 0.9319471),
        ("get", 0.08573313),
    ],
    &[
        ("node", 0.1440086),
        ("groups", 0.9319471),
        ("names", 0.48068208),
        ("type", 0.41575727),
        ("get", 0.08573313),
        ("isomorphic", 0.8510767),
    ],
    &[
        ("groups", 0.9319471),
        ("type", 0.41575727),
        ("get", 0.08573313),
        ("isomorphic", 0.8510767),
        ("node", 0.1440086),
        ("number", 0.5664492),
    ],
    &[
        ("ids", 0.36934143),
        ("edge", 0.31033945),
        ("groups", 0.9319471),
        ("get", 0.08573313),
        ("type", 0.41575727),
        ("isomorphic", 0.8510767),
    ],
    &[
        ("isomorphic", 0.8510767),
        ("groups", 0.9319471),
        ("edge", 0.31033945),
        ("get", 0.08573313),
        ("type", 0.41575727),
        ("names", 0.48068208),
    ],
    &[
        ("isomorphic", 0.8510767),
        ("groups", 0.9319471),
        ("type", 0.41575727),
        ("edge", 0.31033945),
        ("number", 0.5664492),
        ("get", 0.08573313),
    ],
    &[
        ("nodes", 1.6288004),
        ("has", 1.541038),
        ("isomorphic", 2.295102),
    ],
    &[
        ("node", 0.16880831),
        ("has", 0.3540327),
        ("isomorphic", 0.5272687),
        ("types", 0.28565595),
        ("from", 0.15598588),
        ("ids", 0.22881858),
        ("unchecked", 0.28372917),
    ],
    &[
        ("from", 0.19554329),
        ("has", 0.44381404),
        ("ids", 0.2868461),
        ("types", 0.3580972),
        ("node", 0.20876107),
        ("isomorphic", 0.660982),
    ],
    &[("csv", 5.6775565), ("from", 1.0536236)],
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
                    jaro_winkler(&name, GRAPH_METHODS_NAMES[id])
                        * frequencies_doc
                            .iter()
                            .map(|(term, weight)| {
                                match tokens_expanded.iter().find(|(token, _)| token == term) {
                                    Some((_, similarity)) => similarity * weight,
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

///
#[pyclass]
#[derive(Debug, Clone)]
pub struct DendriticTree {
    pub inner: graph::DendriticTree,
}

impl From<graph::DendriticTree> for DendriticTree {
    fn from(val: graph::DendriticTree) -> DendriticTree {
        DendriticTree { inner: val }
    }
}

impl From<DendriticTree> for graph::DendriticTree {
    fn from(val: DendriticTree) -> graph::DendriticTree {
        val.inner
    }
}

#[pymethods]
impl DendriticTree {
    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the type of the dendritic tree
    pub fn get_dendritic_tree_type(&self) -> &str {
        self.inner.get_dendritic_tree_type().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the root node ID of the dendritic tree
    pub fn get_root_node_id(&self) -> NodeT {
        self.inner.get_root_node_id().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return whether the current dendritic tree is actually a tree
    pub fn is_tree(&self) -> bool {
        self.inner.is_tree().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return whether the current dendritic tree is actually a tendril
    pub fn is_tendril(&self) -> bool {
        self.inner.is_tendril().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return whether the current dendritic tree is a proper dentritic tree
    pub fn is_dendritic_tree(&self) -> bool {
        self.inner.is_dendritic_tree().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return whether the current dendritic tree is actually a free-floating chain
    pub fn is_free_floating_chain(&self) -> bool {
        self.inner.is_free_floating_chain().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return whether the current dendritic tree is actually a star
    pub fn is_star(&self) -> bool {
        self.inner.is_star().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return whether the current dendritic tree is actually a star of tendrils
    pub fn is_tendril_star(&self) -> bool {
        self.inner.is_tendril_star().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return whether the current dendritic tree is actually a dendritic star
    pub fn is_dendritic_star(&self) -> bool {
        self.inner.is_dendritic_star().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return whether the current dendritic tree is actually a dendritic tendril star
    pub fn is_dendritic_tendril_star(&self) -> bool {
        self.inner.is_dendritic_tendril_star().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the depth of the dentritic tree
    pub fn get_depth(&self) -> NodeT {
        self.inner.get_depth().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the root node name of the DendriticTree
    pub fn get_root_node_name(&self) -> String {
        self.inner.get_root_node_name().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return number of nodes involved in the dendritic tree
    pub fn get_number_of_involved_nodes(&self) -> NodeT {
        self.inner.get_number_of_involved_nodes().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return number of edges involved in the dendritic tree
    pub fn get_number_of_involved_edges(&self) -> EdgeT {
        self.inner.get_number_of_involved_edges().into()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the node IDs of the nodes composing the DendriticTree
    pub fn get_dentritic_trees_node_ids(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_dentritic_trees_node_ids(), NodeT)
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, k)"]
    /// Return the first `k` node IDs of the nodes composing the DendriticTree.
    ///
    /// Parameters
    /// ----------
    ///
    pub fn get_first_k_dentritic_trees_node_ids(&self, k: usize) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(
            gil,
            self.inner.get_first_k_dentritic_trees_node_ids(k.into()),
            NodeT
        )
    }

    #[automatically_generated_binding]
    #[text_signature = "($self, k)"]
    /// Return the first `k` node names of the nodes composing the DendriticTree.
    ///
    /// Parameters
    /// ----------
    ///
    pub fn get_first_k_dentritic_trees_node_names(&self, k: usize) -> Vec<String> {
        self.inner
            .get_first_k_dentritic_trees_node_names(k.into())
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[text_signature = "($self)"]
    /// Return the node names of the nodes composing the DendriticTree
    pub fn get_dentritic_trees_node_names(&self) -> Vec<String> {
        self.inner
            .get_dentritic_trees_node_names()
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }
}

pub const DENDRITICTREE_METHODS_NAMES: &[&str] = &[
    "get_dendritic_tree_type",
    "get_root_node_id",
    "is_tree",
    "is_tendril",
    "is_dendritic_tree",
    "is_free_floating_chain",
    "is_star",
    "is_tendril_star",
    "is_dendritic_star",
    "is_dendritic_tendril_star",
    "get_depth",
    "get_root_node_name",
    "get_number_of_involved_nodes",
    "get_number_of_involved_edges",
    "get_dentritic_trees_node_ids",
    "get_first_k_dentritic_trees_node_ids",
    "get_first_k_dentritic_trees_node_names",
    "get_dentritic_trees_node_names",
];

pub const DENDRITICTREE_TERMS: &[&str] = &[
    "get",
    "dendritic",
    "tree",
    "type",
    "root",
    "node",
    "id",
    "is",
    "tendril",
    "free",
    "floating",
    "chain",
    "star",
    "depth",
    "name",
    "number",
    "of",
    "involved",
    "nodes",
    "edges",
    "dentritic",
    "trees",
    "ids",
    "first",
    "k",
    "names",
];

pub const DENDRITICTREE_TFIDF_FREQUENCIES: &[&[(&str, f64)]] = &[
    &[
        ("dendritic", 0.509799),
        ("tree", 0.59874874),
        ("get", 0.2099079),
        ("type", 0.8986398),
    ],
    &[
        ("id", 0.8986398),
        ("get", 0.2099079),
        ("node", 0.37964714),
        ("root", 0.7178391),
    ],
    &[("is", 0.6956208), ("tree", 1.4629598)],
    &[("tendril", 1.4629598), ("is", 0.6956208)],
    &[
        ("tree", 0.9013809),
        ("is", 0.4285964),
        ("dendritic", 0.76747227),
    ],
    &[
        ("free", 0.8986398),
        ("floating", 0.8986398),
        ("chain", 0.8986398),
        ("is", 0.28469825),
    ],
    &[("star", 1.2456234), ("is", 0.6956208)],
    &[
        ("tendril", 0.9013809),
        ("is", 0.4285964),
        ("star", 0.76747227),
    ],
    &[
        ("is", 0.4285964),
        ("star", 0.76747227),
        ("dendritic", 0.76747227),
    ],
    &[
        ("tendril", 0.59874874),
        ("star", 0.509799),
        ("dendritic", 0.509799),
        ("is", 0.28469825),
    ],
    &[("depth", 2.195702), ("get", 0.5128809)],
    &[
        ("node", 0.37964714),
        ("name", 0.8986398),
        ("get", 0.2099079),
        ("root", 0.7178391),
    ],
    &[
        ("involved", 0.50676936),
        ("of", 0.50676936),
        ("nodes", 0.63440835),
        ("get", 0.14818765),
        ("number", 0.50676936),
    ],
    &[
        ("get", 0.14818765),
        ("involved", 0.50676936),
        ("number", 0.50676936),
        ("of", 0.50676936),
        ("edges", 0.63440835),
    ],
    &[
        ("node", 0.26801765),
        ("get", 0.14818765),
        ("ids", 0.50676936),
        ("trees", 0.3599003),
        ("dentritic", 0.3599003),
    ],
    &[
        ("ids", 0.2880835),
        ("node", 0.15236016),
        ("first", 0.2880835),
        ("trees", 0.20459273),
        ("k", 0.2880835),
        ("get", 0.084240325),
        ("dentritic", 0.20459273),
    ],
    &[
        ("first", 0.2880835),
        ("names", 0.2880835),
        ("trees", 0.20459273),
        ("node", 0.15236016),
        ("get", 0.084240325),
        ("k", 0.2880835),
        ("dentritic", 0.20459273),
    ],
    &[
        ("get", 0.14818765),
        ("node", 0.26801765),
        ("names", 0.50676936),
        ("dentritic", 0.3599003),
        ("trees", 0.3599003),
    ],
];

#[pymethods]
impl DendriticTree {
    fn _repr_html_(&self) -> String {
        self.__repr__()
    }
}

#[pyproto]
impl PyObjectProtocol for DendriticTree {
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
                let mut similarities = DENDRITICTREE_TERMS
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
        let mut doc_scores = DENDRITICTREE_TFIDF_FREQUENCIES
            .par_iter()
            .enumerate()
            // for each document
            .map(|(id, frequencies_doc)| {
                (
                    id,
                    jaro_winkler(&name, DENDRITICTREE_METHODS_NAMES[id])
                        * frequencies_doc
                            .iter()
                            .map(|(term, weight)| {
                                match tokens_expanded.iter().find(|(token, _)| token == term) {
                                    Some((_, similarity)) => similarity * weight,
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
                        DENDRITICTREE_METHODS_NAMES[*method_id].to_string()
                    )
                })
                .take(10)
                .collect::<Vec<String>>()
                .join("\n"),
        )))
    }
}

#[pymodule]
fn edge_list_utils(_py: Python, _m: &PyModule) -> PyResult<()> {
    _m.add_wrapped(wrap_pyfunction!(convert_edge_list_to_numeric))?;
    _m.add_wrapped(wrap_pyfunction!(densify_sparse_numeric_edge_list))?;
    _m.add_wrapped(wrap_pyfunction!(are_there_selfloops_in_edge_list))?;
    _m.add_wrapped(wrap_pyfunction!(get_rows_number))?;
    _m.add_wrapped(wrap_pyfunction!(convert_directed_edge_list_to_undirected))?;
    _m.add_wrapped(wrap_pyfunction!(add_numeric_id_to_csv))?;
    _m.add_wrapped(wrap_pyfunction!(build_optimal_lists_files))?;
    _m.add_wrapped(wrap_pyfunction!(filter_duplicates_from_edge_list))?;
    _m.add_wrapped(wrap_pyfunction!(convert_undirected_edge_list_to_directed))?;
    _m.add_wrapped(wrap_pyfunction!(get_minmax_node_from_numeric_edge_list))?;
    _m.add_wrapped(wrap_pyfunction!(parse_wikipedia_graph))?;
    _m.add_wrapped(wrap_pyfunction!(get_selfloops_number_from_edge_list))?;
    _m.add_wrapped(wrap_pyfunction!(is_numeric_edge_list))?;
    _m.add_wrapped(wrap_pyfunction!(convert_node_list_node_types_to_numeric))?;
    _m.add_wrapped(wrap_pyfunction!(has_duplicated_edges_in_edge_list))?;
    _m.add_wrapped(wrap_pyfunction!(sort_numeric_edge_list))?;
    _m.add_wrapped(wrap_pyfunction!(sort_numeric_edge_list_inplace))?;
    Ok(())
}

#[module(edge_list_utils)]
#[pyfunction]
#[automatically_generated_binding]
#[text_signature = "(original_node_path, original_node_list_separator, original_node_list_header, original_node_list_support_balanced_quotes, node_list_rows_to_skip, node_list_is_correct, node_list_max_rows_number, node_list_comment_symbol, original_nodes_column_number, original_nodes_column, nodes_number, original_minimum_node_id, original_numeric_node_ids, original_load_node_list_in_parallel, original_edge_type_path, original_edge_types_column_number, original_edge_types_column, edge_types_number, original_numeric_edge_type_ids, original_minimum_edge_type_id, original_edge_type_list_separator, original_edge_type_list_header, original_edge_type_list_support_balanced_quotes, edge_type_list_rows_to_skip, edge_type_list_is_correct, edge_type_list_max_rows_number, edge_type_list_comment_symbol, load_edge_type_list_in_parallel, original_edge_path, original_edge_list_separator, original_edge_list_header, original_edge_list_support_balanced_quotes, original_sources_column_number, original_sources_column, original_destinations_column_number, original_destinations_column, original_edge_list_edge_types_column, original_edge_list_edge_types_column_number, original_weights_column, original_weights_column_number, target_edge_path, target_edge_list_separator, target_edge_list_header, target_sources_column, target_sources_column_number, target_destinations_column, target_destinations_column_number, target_edge_list_edge_types_column, target_edge_list_edge_types_column_number, target_weights_column, target_weights_column_number, target_node_path, target_node_list_separator, target_node_list_header, target_nodes_column, target_nodes_column_number, target_edge_type_list_path, target_edge_type_list_separator, target_edge_type_list_header, target_edge_type_list_edge_types_column, target_edge_type_list_edge_types_column_number, comment_symbol, default_edge_type, default_weight, max_rows_number, rows_to_skip, edges_number, skip_edge_types_if_unavailable, skip_weights_if_unavailable, numeric_rows_are_surely_smaller_than_original, directed, verbose, name)"]
/// Create a new edge list starting from given one with node IDs densified.
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
    original_node_list_separator: Option<char>,
    original_node_list_header: Option<bool>,
    original_node_list_support_balanced_quotes: Option<bool>,
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
    original_edge_type_list_separator: Option<char>,
    original_edge_type_list_header: Option<bool>,
    original_edge_type_list_support_balanced_quotes: Option<bool>,
    edge_type_list_rows_to_skip: Option<usize>,
    edge_type_list_is_correct: Option<bool>,
    edge_type_list_max_rows_number: Option<usize>,
    edge_type_list_comment_symbol: Option<String>,
    load_edge_type_list_in_parallel: Option<bool>,
    original_edge_path: &str,
    original_edge_list_separator: Option<char>,
    original_edge_list_header: Option<bool>,
    original_edge_list_support_balanced_quotes: Option<bool>,
    original_sources_column_number: Option<usize>,
    original_sources_column: Option<String>,
    original_destinations_column_number: Option<usize>,
    original_destinations_column: Option<String>,
    original_edge_list_edge_types_column: Option<String>,
    original_edge_list_edge_types_column_number: Option<usize>,
    original_weights_column: Option<String>,
    original_weights_column_number: Option<usize>,
    target_edge_path: &str,
    target_edge_list_separator: Option<char>,
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
    target_node_list_separator: Option<char>,
    target_node_list_header: Option<bool>,
    target_nodes_column: Option<String>,
    target_nodes_column_number: Option<usize>,
    target_edge_type_list_path: Option<String>,
    target_edge_type_list_separator: Option<char>,
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
    numeric_rows_are_surely_smaller_than_original: Option<bool>,
    directed: bool,
    verbose: Option<bool>,
    name: Option<String>,
) -> PyResult<(NodeT, Option<EdgeTypeT>)> {
    Ok({
        let (subresult_0, subresult_1) = pe!(graph::convert_edge_list_to_numeric(
            original_node_path.into(),
            original_node_list_separator.into(),
            original_node_list_header.into(),
            original_node_list_support_balanced_quotes.into(),
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
            original_edge_type_list_support_balanced_quotes.into(),
            edge_type_list_rows_to_skip.into(),
            edge_type_list_is_correct.into(),
            edge_type_list_max_rows_number.into(),
            edge_type_list_comment_symbol.into(),
            load_edge_type_list_in_parallel.into(),
            original_edge_path.into(),
            original_edge_list_separator.into(),
            original_edge_list_header.into(),
            original_edge_list_support_balanced_quotes.into(),
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
            numeric_rows_are_surely_smaller_than_original.into(),
            directed.into(),
            verbose.into(),
            name.into()
        ))?
        .into();
        (subresult_0.into(), subresult_1.into())
    })
}

#[module(edge_list_utils)]
#[pyfunction]
#[automatically_generated_binding]
#[text_signature = "(maximum_node_id, original_edge_path, original_edge_list_separator, original_edge_list_header, original_sources_column, original_sources_column_number, original_destinations_column, original_destinations_column_number, original_edge_list_edge_types_column, original_edge_list_edge_types_column_number, original_weights_column, original_weights_column_number, original_edge_type_path, original_edge_types_column_number, original_edge_types_column, edge_types_number, original_numeric_edge_type_ids, original_minimum_edge_type_id, original_edge_type_list_separator, original_edge_type_list_header, edge_type_list_rows_to_skip, edge_type_list_is_correct, edge_type_list_max_rows_number, edge_type_list_comment_symbol, load_edge_type_list_in_parallel, target_edge_path, target_edge_list_separator, target_edge_list_header, target_sources_column, target_sources_column_number, target_destinations_column, target_destinations_column_number, target_edge_list_edge_types_column, target_edge_list_edge_types_column_number, target_weights_column, target_weights_column_number, target_node_path, target_node_list_separator, target_node_list_header, target_nodes_column, target_nodes_column_number, target_edge_type_list_path, target_edge_type_list_separator, target_edge_type_list_header, target_edge_type_list_edge_types_column, target_edge_type_list_edge_types_column_number, comment_symbol, default_edge_type, default_weight, max_rows_number, rows_to_skip, edges_number, skip_edge_types_if_unavailable, skip_weights_if_unavailable, numeric_rows_are_surely_smaller_than_original, directed, verbose, name)"]
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
/// maximum_node_id: Optional[int]
///     The maximum node ID present in this graph. If available, optimal memory allocation will be used.
/// original_edge_path: str
///     The path from where to load the original edge list.
/// original_edge_list_separator: Optional[str]
///     Separator to use for the original edge list.
/// original_edge_list_header: Optional[bool]
///     Whether the original edge list has an header.
/// original_sources_column: Optional[str]
///     The column name to use to load the sources in the original edges list.
/// original_sources_column_number: Optional[int]
///     The column number to use to load the sources in the original edges list.
/// original_destinations_column: Optional[str]
///     The column name to use to load the destinations in the original edges list.
/// original_destinations_column_number: Optional[int]
///     The column number to use to load the destinations in the original edges list.
/// original_edge_list_edge_types_column: Optional[str]
///     The column name to use for the edge types in the original edges list.
/// original_edge_list_edge_types_column_number: Optional[int]
///     The column number to use for the edge types in the original edges list.
/// original_weights_column: Optional[str]
///     The column name to use for the weights in the original edges list.
/// original_weights_column_number: Optional[int]
///     The column number to use for the weights in the original edges list.
/// target_edge_path: str
///     The path from where to load the target edge list.
/// target_edge_list_separator: Optional[str]
///     Separator to use for the target edge list.
/// target_edge_list_header: Optional[bool]
///     Whether the target edge list has an header.
/// target_sources_column: Optional[str]
///     The column name to use to load the sources in the target edges list.
/// target_sources_column_number: Optional[int]
///     The column number to use to load the sources in the target edges list.
/// target_destinations_column: Optional[str]
///     The column name to use to load the destinations in the target edges list.
/// target_destinations_column_number: Optional[int]
///     The column number to use to load the destinations in the target edges list.
/// target_edge_list_edge_types_column: Optional[str]
///     The column name to use for the edge types in the target edges list.
/// target_edge_list_edge_types_column_number: Optional[int]
///     The column number to use for the edge types in the target edges list.
/// target_weights_column: Optional[str]
///     The column name to use for the weights in the target edges list.
/// target_weights_column_number: Optional[int]
///     The column number to use for the weights in the target edges list.
/// comment_symbol: Optional[str]
///     The comment symbol to use within the original edge list.
/// default_edge_type: Optional[str]
///     The default edge type to use within the original edge list.
/// default_weight: Optional[float]
///     The default weight to use within the original edge list.
/// max_rows_number: Optional[int]
///     The amount of rows to load from the original edge list.
/// rows_to_skip: Optional[int]
///     The amount of rows to skip from the original edge list.
/// edges_number: Optional[int]
///     The expected number of edges. It will be used for the loading bar.
/// skip_edge_types_if_unavailable: Optional[bool]
///     Whether to automatically skip the edge types if they are not available.
/// skip_weights_if_unavailable: Optional[bool]
///     Whether to automatically skip the weights if they are not available.
/// verbose: Optional[bool]
///     Whether to show the loading bar while processing the file.
/// name: Optional[str]
///     The name of the graph to display in the loading bar.
///
pub fn densify_sparse_numeric_edge_list(
    maximum_node_id: Option<EdgeT>,
    original_edge_path: &str,
    original_edge_list_separator: Option<char>,
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
    original_edge_type_list_separator: Option<char>,
    original_edge_type_list_header: Option<bool>,
    edge_type_list_rows_to_skip: Option<usize>,
    edge_type_list_is_correct: Option<bool>,
    edge_type_list_max_rows_number: Option<usize>,
    edge_type_list_comment_symbol: Option<String>,
    load_edge_type_list_in_parallel: Option<bool>,
    target_edge_path: &str,
    target_edge_list_separator: Option<char>,
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
    target_node_list_separator: Option<char>,
    target_node_list_header: Option<bool>,
    target_nodes_column: Option<String>,
    target_nodes_column_number: Option<usize>,
    target_edge_type_list_path: Option<String>,
    target_edge_type_list_separator: Option<char>,
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
    numeric_rows_are_surely_smaller_than_original: Option<bool>,
    directed: bool,
    verbose: Option<bool>,
    name: Option<String>,
) -> PyResult<(NodeT, Option<EdgeTypeT>)> {
    Ok({
        let (subresult_0, subresult_1) = pe!(graph::densify_sparse_numeric_edge_list(
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
            numeric_rows_are_surely_smaller_than_original.into(),
            directed.into(),
            verbose.into(),
            name.into()
        ))?
        .into();
        (subresult_0.into(), subresult_1.into())
    })
}

#[module(edge_list_utils)]
#[pyfunction]
#[automatically_generated_binding]
#[text_signature = "(path, separator, header, sources_column, sources_column_number, destinations_column, destinations_column_number, comment_symbol, support_balanced_quotes, max_rows_number, rows_to_skip, edges_number, load_edge_list_in_parallel, verbose, name)"]
/// Return whether there are selfloops in the edge list.
///
/// Parameters
/// ----------
/// path: str
///     The path from where to load the edge list.
/// separator: Optional[str]
///     The separator for the rows in the edge list.
/// header: Optional[bool]
///     Whether the edge list has an header.
/// sources_column: Optional[str]
///     The column name to use for the source nodes.
/// sources_column_number: Optional[int]
///     The column number to use for the source nodes.
/// destinations_column: Optional[str]
///     The column name to use for the destination nodes.
/// destinations_column_number: Optional[int]
///     The column number to use for the destination nodes.
/// comment_symbol: Optional[str]
///     The comment symbol to use for the lines to skip.
/// support_balanced_quotes: Optional[bool]
///     Whether to support balanced quotes.
/// max_rows_number: Optional[int]
///     The number of rows to read at most. Note that this parameter is ignored when reading in parallel.
/// rows_to_skip: Optional[int]
///     Number of rows to skip in the edge list.
/// edges_number: Optional[int]
///     Number of edges in the edge list.
/// load_edge_list_in_parallel: Optional[bool]
///     Whether to execute the task in parallel or sequential. Generally, parallel is preferable.
/// verbose: Optional[bool]
///     Whether to show the loading bar while processing the file.
/// name: Optional[str]
///     The name of the graph to display in the loading bar.
///
pub fn are_there_selfloops_in_edge_list(
    path: &str,
    separator: Option<char>,
    header: Option<bool>,
    sources_column: Option<String>,
    sources_column_number: Option<usize>,
    destinations_column: Option<String>,
    destinations_column_number: Option<usize>,
    comment_symbol: Option<String>,
    support_balanced_quotes: Option<bool>,
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
        support_balanced_quotes.into(),
        max_rows_number.into(),
        rows_to_skip.into(),
        edges_number.into(),
        load_edge_list_in_parallel.into(),
        verbose.into(),
        name.into()
    ))?
    .into())
}

#[module(edge_list_utils)]
#[pyfunction]
#[automatically_generated_binding]
#[text_signature = "(file_path)"]
/// Return number of rows in given CSV path.
///
/// Parameters
/// ----------
/// file_path: str
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

#[module(edge_list_utils)]
#[pyfunction]
#[automatically_generated_binding]
#[text_signature = "(original_edge_path, original_edge_list_separator, original_edge_list_header, original_edge_list_support_balanced_quotes, original_sources_column, original_sources_column_number, original_destinations_column, original_destinations_column_number, original_edge_list_edge_type_column, original_edge_list_edge_type_column_number, original_weights_column, original_weights_column_number, target_edge_path, target_edge_list_separator, target_edge_list_header, target_sources_column_number, target_sources_column, target_destinations_column_number, target_destinations_column, target_edge_list_edge_type_column, target_edge_list_edge_type_column_number, target_weights_column, target_weights_column_number, comment_symbol, default_edge_type, default_weight, max_rows_number, rows_to_skip, edges_number, skip_edge_types_if_unavailable, skip_weights_if_unavailable, verbose, name)"]
/// Create a new undirected edge list from a given directed one by duplicating the undirected edges.
///
/// Parameters
/// ----------
/// original_edge_path: str
///     The path from where to load the original edge list.
/// original_edge_list_separator: Optional[str]
///     Separator to use for the original edge list.
/// original_edge_list_header: Optional[bool]
///     Whether the original edge list has an header.
/// original_edge_list_support_balanced_quotes: Optional[bool]
///     Whether to support balanced quotes while reading the edge list.
/// original_sources_column: Optional[str]
///     The column name to use to load the sources in the original edges list.
/// original_sources_column_number: Optional[int]
///     The column number to use to load the sources in the original edges list.
/// original_destinations_column: Optional[str]
///     The column name to use to load the destinations in the original edges list.
/// original_destinations_column_number: Optional[int]
///     The column number to use to load the destinations in the original edges list.
/// original_edge_list_edge_type_column: Optional[str]
///     The column name to use for the edge types in the original edges list.
/// original_edge_list_edge_type_column_number: Optional[int]
///     The column number to use for the edge types in the original edges list.
/// original_weights_column: Optional[str]
///     The column name to use for the weights in the original edges list.
/// original_weights_column_number: Optional[int]
///     The column number to use for the weights in the original edges list.
/// target_edge_path: str
///     The path from where to load the target edge list. This must be different from the original edge list path.
/// target_edge_list_separator: Optional[str]
///     Separator to use for the target edge list. If None, the one provided from the original edge list will be used.
/// target_edge_list_header: Optional[bool]
///     Whether the target edge list has an header. If None, the one provided from the original edge list will be used.
/// target_sources_column: Optional[str]
///     The column name to use to load the sources in the target edges list. If None, the one provided from the original edge list will be used.
/// target_sources_column_number: Optional[int]
///     The column number to use to load the sources in the target edges list. If None, the one provided from the original edge list will be used.
/// target_destinations_column: Optional[str]
///     The column name to use to load the destinations in the target edges list. If None, the one provided from the original edge list will be used.
/// target_destinations_column_number: Optional[int]
///     The column number to use to load the destinations in the target edges list. If None, the one provided from the original edge list will be used.
/// target_edge_list_edge_type_column: Optional[str]
///     The column name to use for the edge types in the target edges list. If None, the one provided from the original edge list will be used.
/// target_edge_list_edge_type_column_number: Optional[int]
///     The column number to use for the edge types in the target edges list. If None, the one provided from the original edge list will be used.
/// target_weights_column: Optional[str]
///     The column name to use for the weights in the target edges list. If None, the one provided from the original edge list will be used.
/// target_weights_column_number: Optional[int]
///     The column number to use for the weights in the target edges list. If None, the one provided from the original edge list will be used.
/// comment_symbol: Optional[str]
///     The comment symbol to use within the original edge list.
/// default_edge_type: Optional[str]
///     The default edge type to use within the original edge list.
/// default_weight: Optional[float]
///     The default weight to use within the original edge list.
/// max_rows_number: Optional[int]
///     The amount of rows to load from the original edge list.
/// rows_to_skip: Optional[int]
///     The amount of rows to skip from the original edge list.
/// edges_number: Optional[int]
///     The expected number of edges. It will be used for the loading bar.
/// skip_edge_types_if_unavailable: Optional[bool]
///     Whether to automatically skip the edge types if they are not available.
/// skip_weights_if_unavailable: Optional[bool]
///     Whether to automatically skip the weights if they are not available.
/// verbose: Optional[bool]
///     Whether to show the loading bar while processing the file.
/// name: Optional[str]
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
    original_edge_list_separator: Option<char>,
    original_edge_list_header: Option<bool>,
    original_edge_list_support_balanced_quotes: Option<bool>,
    original_sources_column: Option<String>,
    original_sources_column_number: Option<usize>,
    original_destinations_column: Option<String>,
    original_destinations_column_number: Option<usize>,
    original_edge_list_edge_type_column: Option<String>,
    original_edge_list_edge_type_column_number: Option<usize>,
    original_weights_column: Option<String>,
    original_weights_column_number: Option<usize>,
    target_edge_path: &str,
    target_edge_list_separator: Option<char>,
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
        original_edge_list_support_balanced_quotes.into(),
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

#[module(edge_list_utils)]
#[pyfunction]
#[automatically_generated_binding]
#[text_signature = "(original_csv_path, original_csv_separator, original_csv_header, target_csv_path, target_csv_separator, target_csv_header, target_csv_ids_column, target_csv_ids_column_number, comment_symbol, support_balanced_quotes, max_rows_number, rows_to_skip, lines_number, verbose)"]
/// Create a new CSV with the lines number added to it.
///
/// Parameters
/// ----------
/// original_csv_path: str
///     The path from where to load the original CSV.
/// original_csv_separator: Optional[str]
///     Separator to use for the original CSV.
/// original_csv_header: Optional[bool]
///     Whether the original CSV has an header.
/// target_csv_path: str
///     The path from where to load the target CSV. This cannot be the same as the original CSV.
/// target_csv_separator: Optional[str]
///     Separator to use for the target CSV. If None, the one provided from the original CSV will be used.
/// target_csv_header: Optional[bool]
///     Whether the target CSV has an header. If None, the one provided from the original CSV will be used.
/// target_csv_ids_column: Optional[str]
///     The column name to use for the ids in the target list.
/// target_csv_ids_column_number: Optional[int]
///     The column number to use for the ids in the target list.
/// comment_symbol: Optional[str]
///     The comment symbol to use within the original CSV.
/// support_balanced_quotes: Optional[bool]
///     Whether to support balanced quotes.
/// max_rows_number: Optional[int]
///     The amount of rows to load from the original CSV.
/// rows_to_skip: Optional[int]
///     The amount of rows to skip from the original CSV.
/// verbose: Optional[bool]
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
    original_csv_separator: Option<char>,
    original_csv_header: Option<bool>,
    target_csv_path: &str,
    target_csv_separator: Option<char>,
    target_csv_header: Option<bool>,
    target_csv_ids_column: Option<String>,
    target_csv_ids_column_number: Option<usize>,
    comment_symbol: Option<String>,
    support_balanced_quotes: Option<bool>,
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
        support_balanced_quotes.into(),
        max_rows_number.into(),
        rows_to_skip.into(),
        lines_number.into(),
        verbose.into()
    ))?
    .into())
}

#[module(edge_list_utils)]
#[pyfunction]
#[automatically_generated_binding]
#[text_signature = "(original_node_type_path, original_node_type_list_separator, original_node_types_column_number, original_node_types_column, original_numeric_node_type_ids, original_minimum_node_type_id, original_node_type_list_header, original_node_type_list_support_balanced_quotes, original_node_type_list_rows_to_skip, original_node_type_list_max_rows_number, original_node_type_list_comment_symbol, original_load_node_type_list_in_parallel, original_node_type_list_is_correct, node_types_number, target_node_type_list_path, target_node_type_list_separator, target_node_type_list_node_types_column_number, target_node_type_list_node_types_column, target_node_type_list_header, original_node_path, original_node_list_separator, original_node_list_header, original_node_list_support_balanced_quotes, node_list_rows_to_skip, node_list_is_correct, node_list_max_rows_number, node_list_comment_symbol, default_node_type, original_nodes_column_number, original_nodes_column, original_node_types_separator, original_node_list_node_types_column_number, original_node_list_node_types_column, nodes_number, original_minimum_node_id, original_numeric_node_ids, original_node_list_numeric_node_type_ids, original_skip_node_types_if_unavailable, original_load_node_list_in_parallel, maximum_node_id, target_node_path, target_node_list_separator, target_node_list_header, target_nodes_column, target_nodes_column_number, target_node_types_separator, target_node_list_node_types_column, target_node_list_node_types_column_number, original_edge_type_path, original_edge_type_list_separator, original_edge_types_column_number, original_edge_types_column, original_numeric_edge_type_ids, original_minimum_edge_type_id, original_edge_type_list_header, original_edge_type_list_support_balanced_quotes, edge_type_list_rows_to_skip, edge_type_list_max_rows_number, edge_type_list_comment_symbol, load_edge_type_list_in_parallel, edge_type_list_is_correct, edge_types_number, target_edge_type_list_path, target_edge_type_list_separator, target_edge_type_list_edge_types_column_number, target_edge_type_list_edge_types_column, target_edge_type_list_header, original_edge_path, original_edge_list_separator, original_edge_list_header, original_edge_list_support_balanced_quotes, original_sources_column_number, original_sources_column, original_destinations_column_number, original_destinations_column, original_edge_list_edge_types_column_number, original_edge_list_edge_types_column, default_edge_type, original_weights_column_number, original_weights_column, default_weight, original_edge_list_numeric_node_ids, skip_weights_if_unavailable, skip_edge_types_if_unavailable, edge_list_comment_symbol, edge_list_max_rows_number, edge_list_rows_to_skip, load_edge_list_in_parallel, edges_number, target_edge_path, target_edge_list_separator, numeric_rows_are_surely_smaller_than_original, sort_temporary_directory, verbose, directed, name)"]
/// TODO: write the docstrin
pub fn build_optimal_lists_files(
    original_node_type_path: Option<String>,
    original_node_type_list_separator: Option<char>,
    original_node_types_column_number: Option<usize>,
    original_node_types_column: Option<String>,
    original_numeric_node_type_ids: Option<bool>,
    original_minimum_node_type_id: Option<NodeTypeT>,
    original_node_type_list_header: Option<bool>,
    original_node_type_list_support_balanced_quotes: Option<bool>,
    original_node_type_list_rows_to_skip: Option<usize>,
    original_node_type_list_max_rows_number: Option<usize>,
    original_node_type_list_comment_symbol: Option<String>,
    original_load_node_type_list_in_parallel: Option<bool>,
    original_node_type_list_is_correct: Option<bool>,
    node_types_number: Option<NodeTypeT>,
    target_node_type_list_path: Option<String>,
    target_node_type_list_separator: Option<char>,
    target_node_type_list_node_types_column_number: Option<usize>,
    target_node_type_list_node_types_column: Option<String>,
    target_node_type_list_header: Option<bool>,
    original_node_path: Option<String>,
    original_node_list_separator: Option<char>,
    original_node_list_header: Option<bool>,
    original_node_list_support_balanced_quotes: Option<bool>,
    node_list_rows_to_skip: Option<usize>,
    node_list_is_correct: Option<bool>,
    node_list_max_rows_number: Option<usize>,
    node_list_comment_symbol: Option<String>,
    default_node_type: Option<String>,
    original_nodes_column_number: Option<usize>,
    original_nodes_column: Option<String>,
    original_node_types_separator: Option<char>,
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
    target_node_list_separator: Option<char>,
    target_node_list_header: Option<bool>,
    target_nodes_column: Option<String>,
    target_nodes_column_number: Option<usize>,
    target_node_types_separator: Option<char>,
    target_node_list_node_types_column: Option<String>,
    target_node_list_node_types_column_number: Option<usize>,
    original_edge_type_path: Option<String>,
    original_edge_type_list_separator: Option<char>,
    original_edge_types_column_number: Option<usize>,
    original_edge_types_column: Option<String>,
    original_numeric_edge_type_ids: Option<bool>,
    original_minimum_edge_type_id: Option<EdgeTypeT>,
    original_edge_type_list_header: Option<bool>,
    original_edge_type_list_support_balanced_quotes: Option<bool>,
    edge_type_list_rows_to_skip: Option<usize>,
    edge_type_list_max_rows_number: Option<usize>,
    edge_type_list_comment_symbol: Option<String>,
    load_edge_type_list_in_parallel: Option<bool>,
    edge_type_list_is_correct: Option<bool>,
    edge_types_number: Option<NodeTypeT>,
    target_edge_type_list_path: Option<String>,
    target_edge_type_list_separator: Option<char>,
    target_edge_type_list_edge_types_column_number: Option<usize>,
    target_edge_type_list_edge_types_column: Option<String>,
    target_edge_type_list_header: Option<bool>,
    original_edge_path: String,
    original_edge_list_separator: Option<char>,
    original_edge_list_header: Option<bool>,
    original_edge_list_support_balanced_quotes: Option<bool>,
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
    target_edge_list_separator: Option<char>,
    numeric_rows_are_surely_smaller_than_original: Option<bool>,
    sort_temporary_directory: Option<String>,
    verbose: Option<bool>,
    directed: bool,
    name: Option<String>,
) -> PyResult<(Option<NodeTypeT>, NodeT, Option<EdgeTypeT>, EdgeT)> {
    Ok({
        let (subresult_0, subresult_1, subresult_2, subresult_3) =
            pe!(graph::build_optimal_lists_files(
                original_node_type_path.into(),
                original_node_type_list_separator.into(),
                original_node_types_column_number.into(),
                original_node_types_column.into(),
                original_numeric_node_type_ids.into(),
                original_minimum_node_type_id.into(),
                original_node_type_list_header.into(),
                original_node_type_list_support_balanced_quotes.into(),
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
                original_node_list_support_balanced_quotes.into(),
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
                original_edge_type_list_support_balanced_quotes.into(),
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
                original_edge_list_support_balanced_quotes.into(),
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
                numeric_rows_are_surely_smaller_than_original.into(),
                sort_temporary_directory.into(),
                verbose.into(),
                directed.into(),
                name.into()
            ))?
            .into();
        (
            subresult_0.into(),
            subresult_1.into(),
            subresult_2.into(),
            subresult_3.into(),
        )
    })
}

#[module(edge_list_utils)]
#[pyfunction]
#[automatically_generated_binding]
#[text_signature = "(original_edge_path, original_edge_list_separator, original_edge_list_header, original_edge_list_support_balanced_quotes, original_edge_list_sources_column, original_edge_list_sources_column_number, original_edge_list_destinations_column, original_edge_list_destinations_column_number, original_edge_list_edge_type_column, original_edge_list_edge_type_column_number, original_edge_list_weights_column, original_edge_list_weights_column_number, target_edge_path, target_edge_list_separator, target_edge_list_header, target_edge_list_sources_column_number, target_edge_list_sources_column, target_edge_list_destinations_column_number, target_edge_list_destinations_column, target_edge_list_edge_type_column, target_edge_list_edge_type_column_number, target_edge_list_weights_column, target_edge_list_weights_column_number, comment_symbol, default_edge_type, default_weight, max_rows_number, rows_to_skip, edges_number, skip_edge_types_if_unavailable, skip_weights_if_unavailable, verbose, name)"]
/// Create a new edge list from a given one filtering duplicates.
///
/// Parameters
/// ----------
/// original_edge_path: str
///     The path from where to load the original edge list.
/// original_edge_list_separator: Optional[str]
///     Separator to use for the original edge list.
/// original_edge_list_header: Optional[bool]
///     Whether the original edge list has an header.
/// original_edge_list_support_balanced_quotes: Optional[bool]
///     Whether to support balanced quotes.
/// original_edge_list_sources_column: Optional[str]
///     The column name to use to load the sources in the original edges list.
/// original_edge_list_sources_column_number: Optional[int]
///     The column number to use to load the sources in the original edges list.
/// original_edge_list_destinations_column: Optional[str]
///     The column name to use to load the destinations in the original edges list.
/// original_edge_list_destinations_column_number: Optional[int]
///     The column number to use to load the destinations in the original edges list.
/// original_edge_list_edge_type_column: Optional[str]
///     The column name to use for the edge types in the original edges list.
/// original_edge_list_edge_type_column_number: Optional[int]
///     The column number to use for the edge types in the original edges list.
/// original_edge_list_weights_column: Optional[str]
///     The column name to use for the weights in the original edges list.
/// original_edge_list_weights_column_number: Optional[int]
///     The column number to use for the weights in the original edges list.
/// target_edge_path: str
///     The path from where to load the target edge list.
/// target_edge_list_separator: Optional[str]
///     Separator to use for the target edge list.
/// target_edge_list_header: Optional[bool]
///     Whether the target edge list has an header.
/// target_edge_list_sources_column: Optional[str]
///     The column name to use to load the sources in the target edges list.
/// target_edge_list_sources_column_number: Optional[int]
///     The column number to use to load the sources in the target edges list.
/// target_edge_list_destinations_column: Optional[str]
///     The column name to use to load the destinations in the target edges list.
/// target_edge_list_destinations_column_number: Optional[int]
///     The column number to use to load the destinations in the target edges list.
/// target_edge_list_edge_type_column: Optional[str]
///     The column name to use for the edge types in the target edges list.
/// target_edge_list_edge_type_column_number: Optional[int]
///     The column number to use for the edge types in the target edges list.
/// target_edge_list_weights_column: Optional[str]
///     The column name to use for the weights in the target edges list.
/// target_edge_list_weights_column_number: Optional[int]
///     The column number to use for the weights in the target edges list.
/// comment_symbol: Optional[str]
///     The comment symbol to use within the original edge list.
/// default_edge_type: Optional[str]
///     The default edge type to use within the original edge list.
/// default_weight: Optional[float]
///     The default weight to use within the original edge list.
/// max_rows_number: Optional[int]
///     The amount of rows to load from the original edge list.
/// rows_to_skip: Optional[int]
///     The amount of rows to skip from the original edge list.
/// edges_number: Optional[int]
///     The expected number of edges. It will be used for the loading bar.
/// skip_edge_types_if_unavailable: Optional[bool]
///     Whether to automatically skip the edge types if they are not available.
/// skip_weights_if_unavailable: Optional[bool]
///     Whether to automatically skip the weights if they are not available.
/// verbose: Optional[bool]
///     Whether to show the loading bar while processing the file.
/// name: Optional[str]
///     The name of the graph to display in the loading bar.
///
pub fn filter_duplicates_from_edge_list(
    original_edge_path: &str,
    original_edge_list_separator: Option<char>,
    original_edge_list_header: Option<bool>,
    original_edge_list_support_balanced_quotes: Option<bool>,
    original_edge_list_sources_column: Option<String>,
    original_edge_list_sources_column_number: Option<usize>,
    original_edge_list_destinations_column: Option<String>,
    original_edge_list_destinations_column_number: Option<usize>,
    original_edge_list_edge_type_column: Option<String>,
    original_edge_list_edge_type_column_number: Option<usize>,
    original_edge_list_weights_column: Option<String>,
    original_edge_list_weights_column_number: Option<usize>,
    target_edge_path: &str,
    target_edge_list_separator: Option<char>,
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
        original_edge_list_support_balanced_quotes.into(),
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
    ))?)
}

#[module(edge_list_utils)]
#[pyfunction]
#[automatically_generated_binding]
#[text_signature = "(original_edge_path, original_edge_list_separator, original_edge_list_header, original_edge_list_support_balanced_quotes, original_sources_column, original_sources_column_number, original_destinations_column, original_destinations_column_number, original_edge_list_edge_type_column, original_edge_list_edge_type_column_number, original_weights_column, original_weights_column_number, target_edge_path, target_edge_list_separator, target_edge_list_header, target_sources_column, target_sources_column_number, target_destinations_column, target_destinations_column_number, target_edge_list_edge_type_column, target_edge_list_edge_type_column_number, target_weights_column, target_weights_column_number, comment_symbol, default_edge_type, default_weight, max_rows_number, rows_to_skip, edges_number, skip_edge_types_if_unavailable, skip_weights_if_unavailable, verbose, name)"]
/// Create a new directed edge list from a given undirected one by duplicating the undirected edges.
///
/// Parameters
/// ----------
/// original_edge_path: str
///     The path from where to load the original edge list.
/// original_edge_list_separator: Optional[str]
///     Separator to use for the original edge list.
/// original_edge_list_header: Optional[bool]
///     Whether the original edge list has an header.
/// original_edge_list_support_balanced_quotes: Optional[bool]
///     Whether to support balanced quotes while reading the edge list.
/// original_sources_column: Optional[str]
///     The column name to use to load the sources in the original edges list.
/// original_sources_column_number: Optional[int]
///     The column number to use to load the sources in the original edges list.
/// original_destinations_column: Optional[str]
///     The column name to use to load the destinations in the original edges list.
/// original_destinations_column_number: Optional[int]
///     The column number to use to load the destinations in the original edges list.
/// original_edge_list_edge_type_column: Optional[str]
///     The column name to use for the edge types in the original edges list.
/// original_edge_list_edge_type_column_number: Optional[int]
///     The column number to use for the edge types in the original edges list.
/// original_weights_column: Optional[str]
///     The column name to use for the weights in the original edges list.
/// original_weights_column_number: Optional[int]
///     The column number to use for the weights in the original edges list.
/// target_edge_path: str
///     The path from where to load the target edge list. This must be different from the original edge list path.
/// target_edge_list_separator: Optional[str]
///     Separator to use for the target edge list. If None, the one provided from the original edge list will be used.
/// target_edge_list_header: Optional[bool]
///     Whether the target edge list has an header. If None, the one provided from the original edge list will be used.
/// target_sources_column: Optional[str]
///     The column name to use to load the sources in the target edges list. If None, the one provided from the original edge list will be used.
/// target_sources_column_number: Optional[int]
///     The column number to use to load the sources in the target edges list. If None, the one provided from the original edge list will be used.
/// target_destinations_column: Optional[str]
///     The column name to use to load the destinations in the target edges list. If None, the one provided from the original edge list will be used.
/// target_destinations_column_number: Optional[int]
///     The column number to use to load the destinations in the target edges list. If None, the one provided from the original edge list will be used.
/// target_edge_list_edge_type_column: Optional[str]
///     The column name to use for the edge types in the target edges list. If None, the one provided from the original edge list will be used.
/// target_edge_list_edge_type_column_number: Optional[int]
///     The column number to use for the edge types in the target edges list. If None, the one provided from the original edge list will be used.
/// target_weights_column: Optional[str]
///     The column name to use for the weights in the target edges list. If None, the one provided from the original edge list will be used.
/// target_weights_column_number: Optional[int]
///     The column number to use for the weights in the target edges list. If None, the one provided from the original edge list will be used.
/// comment_symbol: Optional[str]
///     The comment symbol to use within the original edge list.
/// default_edge_type: Optional[str]
///     The default edge type to use within the original edge list.
/// default_weight: Optional[float]
///     The default weight to use within the original edge list.
/// max_rows_number: Optional[int]
///     The amount of rows to load from the original edge list.
/// rows_to_skip: Optional[int]
///     The amount of rows to skip from the original edge list.
/// edges_number: Optional[int]
///     The expected number of edges. It will be used for the loading bar.
/// skip_edge_types_if_unavailable: Optional[bool]
///     Whether to automatically skip the edge types if they are not available.
/// skip_weights_if_unavailable: Optional[bool]
///     Whether to automatically skip the weights if they are not available.
/// verbose: Optional[bool]
///     Whether to show the loading bar while processing the file.
/// name: Optional[str]
///     The name of the graph to display in the loading bar.
///
pub fn convert_undirected_edge_list_to_directed(
    original_edge_path: &str,
    original_edge_list_separator: Option<char>,
    original_edge_list_header: Option<bool>,
    original_edge_list_support_balanced_quotes: Option<bool>,
    original_sources_column: Option<String>,
    original_sources_column_number: Option<usize>,
    original_destinations_column: Option<String>,
    original_destinations_column_number: Option<usize>,
    original_edge_list_edge_type_column: Option<String>,
    original_edge_list_edge_type_column_number: Option<usize>,
    original_weights_column: Option<String>,
    original_weights_column_number: Option<usize>,
    target_edge_path: &str,
    target_edge_list_separator: Option<char>,
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
        original_edge_list_support_balanced_quotes.into(),
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

#[module(edge_list_utils)]
#[pyfunction]
#[automatically_generated_binding]
#[text_signature = "(path, separator, header, sources_column, sources_column_number, destinations_column, destinations_column_number, comment_symbol, max_rows_number, rows_to_skip, edges_number, load_edge_list_in_parallel, verbose, name)"]
/// Return minimum and maximum node number from given numeric edge list.
///
/// Parameters
/// ----------
/// path: str
///     The path from where to load the edge list.
/// separator: Optional[str]
///     The separator for the rows in the edge list.
/// header: Optional[bool]
///     Whether the edge list has an header.
/// sources_column: Optional[str]
///     The column name to use for the source nodes.
/// sources_column_number: Optional[int]
///     The column number to use for the source nodes.
/// destinations_column: Optional[str]
///     The column name to use for the destination nodes.
/// destinations_column_number: Optional[int]
///     The column number to use for the destination nodes.
/// comment_symbol: Optional[str]
///     The comment symbol to use for the lines to skip.
/// max_rows_number: Optional[int]
///     The number of rows to read at most. Note that this parameter is ignored when reading in parallel.
/// rows_to_skip: Optional[int]
///     Number of rows to skip in the edge list.
/// edges_number: Optional[int]
///     Number of edges in the edge list.
/// load_edge_list_in_parallel: Optional[bool]
///     Whether to execute the task in parallel or sequential. Generally, parallel is preferable.
/// verbose: Optional[bool]
///     Whether to show the loading bar while processing the file.
/// name: Optional[str]
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
    separator: Option<char>,
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
    Ok({
        let (subresult_0, subresult_1, subresult_2) =
            pe!(graph::get_minmax_node_from_numeric_edge_list(
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
            .into();
        (subresult_0.into(), subresult_1.into(), subresult_2.into())
    })
}

#[module(edge_list_utils)]
#[pyfunction]
#[automatically_generated_binding]
#[text_signature = "(source_path, edge_path, node_path, node_type_path, edge_type_path, node_list_separator, node_type_list_separator, edge_type_list_separator, node_types_separator, nodes_column, node_types_column, node_list_node_types_column, edge_types_column, node_descriptions_column, edge_list_separator, sort_temporary_directory, directed, compute_node_description, keep_nodes_without_descriptions, keep_nodes_without_categories, keep_interwikipedia_nodes, keep_external_nodes, verbose)"]
/// TODO: write the docstrin
pub fn parse_wikipedia_graph(
    source_path: &str,
    edge_path: &str,
    node_path: &str,
    node_type_path: &str,
    edge_type_path: &str,
    node_list_separator: char,
    node_type_list_separator: char,
    edge_type_list_separator: char,
    node_types_separator: &str,
    nodes_column: &str,
    node_types_column: &str,
    node_list_node_types_column: &str,
    edge_types_column: &str,
    node_descriptions_column: &str,
    edge_list_separator: char,
    sort_temporary_directory: Option<String>,
    directed: bool,
    compute_node_description: Option<bool>,
    keep_nodes_without_descriptions: Option<bool>,
    keep_nodes_without_categories: Option<bool>,
    keep_interwikipedia_nodes: Option<bool>,
    keep_external_nodes: Option<bool>,
    verbose: Option<bool>,
) -> PyResult<(NodeTypeT, NodeT, EdgeT)> {
    Ok({
        let (subresult_0, subresult_1, subresult_2) = pe!(graph::parse_wikipedia_graph(
            source_path.into(),
            edge_path.into(),
            node_path.into(),
            node_type_path.into(),
            edge_type_path.into(),
            node_list_separator.into(),
            node_type_list_separator.into(),
            edge_type_list_separator.into(),
            node_types_separator.into(),
            nodes_column.into(),
            node_types_column.into(),
            node_list_node_types_column.into(),
            edge_types_column.into(),
            node_descriptions_column.into(),
            edge_list_separator.into(),
            sort_temporary_directory.into(),
            directed.into(),
            compute_node_description.into(),
            keep_nodes_without_descriptions.into(),
            keep_nodes_without_categories.into(),
            keep_interwikipedia_nodes.into(),
            keep_external_nodes.into(),
            verbose.into()
        ))?
        .into();
        (subresult_0.into(), subresult_1.into(), subresult_2.into())
    })
}

#[module(edge_list_utils)]
#[pyfunction]
#[automatically_generated_binding]
#[text_signature = "(path, separator, header, support_balanced_quotes, sources_column, sources_column_number, destinations_column, destinations_column_number, comment_symbol, max_rows_number, rows_to_skip, edges_number, load_edge_list_in_parallel, verbose, name)"]
/// Return number of selfloops in the given edge list.
///
/// Parameters
/// ----------
/// path: str
///     The path from where to load the edge list.
/// separator: Optional[str]
///     The separator for the rows in the edge list.
/// header: Optional[bool]
///     Whether the edge list has an header.
/// support_balanced_quotes: Optional[bool]
///     Whether to support balanced quotes.
/// sources_column: Optional[str]
///     The column name to use for the source nodes.
/// sources_column_number: Optional[int]
///     The column number to use for the source nodes.
/// destinations_column: Optional[str]
///     The column name to use for the destination nodes.
/// destinations_column_number: Optional[int]
///     The column number to use for the destination nodes.
/// comment_symbol: Optional[str]
///     The comment symbol to use for the lines to skip.
/// max_rows_number: Optional[int]
///     The number of rows to read at most. Note that this parameter is ignored when reading in parallel.
/// rows_to_skip: Optional[int]
///     Number of rows to skip in the edge list.
/// edges_number: Optional[int]
///     Number of edges in the edge list.
/// load_edge_list_in_parallel: Optional[bool]
///     Whether to execute the task in parallel or sequential. Generally, parallel is preferable.
/// verbose: Optional[bool]
///     Whether to show the loading bar while processing the file.
/// name: Optional[str]
///     The name of the graph to display in the loading bar.
///
pub fn get_selfloops_number_from_edge_list(
    path: &str,
    separator: Option<char>,
    header: Option<bool>,
    support_balanced_quotes: Option<bool>,
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
        support_balanced_quotes.into(),
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

#[module(edge_list_utils)]
#[pyfunction]
#[automatically_generated_binding]
#[text_signature = "(path, separator, header, support_balanced_quotes, sources_column, sources_column_number, destinations_column, destinations_column_number, comment_symbol, max_rows_number, rows_to_skip, edges_number, load_edge_list_in_parallel, verbose, name)"]
/// Return number of selfloops in the given edge list.
///
/// Parameters
/// ----------
/// path: str
///     The path from where to load the edge list.
/// separator: Optional[str]
///     The separator for the rows in the edge list.
/// header: Optional[bool]
///     Whether the edge list has an header.
/// support_balanced_quotes: Optional[bool]
///     Whether to support balanced quotes.
/// sources_column: Optional[str]
///     The column name to use for the source nodes.
/// sources_column_number: Optional[int]
///     The column number to use for the source nodes.
/// destinations_column: Optional[str]
///     The column name to use for the destination nodes.
/// destinations_column_number: Optional[int]
///     The column number to use for the destination nodes.
/// comment_symbol: Optional[str]
///     The comment symbol to use for the lines to skip.
/// max_rows_number: Optional[int]
///     The number of rows to read at most. Note that this parameter is ignored when reading in parallel.
/// rows_to_skip: Optional[int]
///     Number of rows to skip in the edge list.
/// edges_number: Optional[int]
///     Number of edges in the edge list.
/// load_edge_list_in_parallel: Optional[bool]
///     Whether to execute the task in parallel or sequential. Generally, parallel is preferable.
/// verbose: Optional[bool]
///     Whether to show the loading bar while processing the file.
/// name: Optional[str]
///     The name of the graph to display in the loading bar.
///
pub fn is_numeric_edge_list(
    path: &str,
    separator: Option<char>,
    header: Option<bool>,
    support_balanced_quotes: Option<bool>,
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
        support_balanced_quotes.into(),
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

#[module(edge_list_utils)]
#[pyfunction]
#[automatically_generated_binding]
#[text_signature = "(original_node_type_path, original_node_type_list_separator, original_node_types_column_number, original_node_types_column, node_types_number, original_numeric_node_type_ids, original_minimum_node_type_id, original_node_type_list_header, original_node_type_list_support_balanced_quotes, original_node_type_list_rows_to_skip, original_node_type_list_is_correct, original_node_type_list_max_rows_number, original_node_type_list_comment_symbol, original_load_node_type_list_in_parallel, target_node_type_list_path, target_node_type_list_separator, target_node_type_list_header, target_node_type_list_node_types_column, target_node_type_list_node_types_column_number, original_node_path, original_node_list_separator, original_node_list_header, original_node_list_support_balanced_quotes, node_list_rows_to_skip, node_list_max_rows_number, node_list_comment_symbol, default_node_type, original_nodes_column_number, original_nodes_column, original_node_types_separator, original_node_list_node_types_column_number, original_node_list_node_types_column, original_minimum_node_id, original_numeric_node_ids, original_node_list_numeric_node_type_ids, original_skip_node_types_if_unavailable, target_node_path, target_node_list_separator, target_node_list_header, target_nodes_column_number, target_nodes_column, target_node_types_separator, target_node_list_node_types_column_number, target_node_list_node_types_column, nodes_number)"]
/// Converts the node list at given path to numeric saving in stream to file. Furthermore, returns the number of nodes that were written and their node types if any.
///
/// Parameters
/// ----------
///
pub fn convert_node_list_node_types_to_numeric(
    original_node_type_path: Option<String>,
    original_node_type_list_separator: Option<char>,
    original_node_types_column_number: Option<usize>,
    original_node_types_column: Option<String>,
    node_types_number: Option<NodeTypeT>,
    original_numeric_node_type_ids: Option<bool>,
    original_minimum_node_type_id: Option<NodeTypeT>,
    original_node_type_list_header: Option<bool>,
    original_node_type_list_support_balanced_quotes: Option<bool>,
    original_node_type_list_rows_to_skip: Option<usize>,
    original_node_type_list_is_correct: Option<bool>,
    original_node_type_list_max_rows_number: Option<usize>,
    original_node_type_list_comment_symbol: Option<String>,
    original_load_node_type_list_in_parallel: Option<bool>,
    target_node_type_list_path: Option<String>,
    target_node_type_list_separator: Option<char>,
    target_node_type_list_header: Option<bool>,
    target_node_type_list_node_types_column: Option<String>,
    target_node_type_list_node_types_column_number: Option<usize>,
    original_node_path: String,
    original_node_list_separator: Option<char>,
    original_node_list_header: Option<bool>,
    original_node_list_support_balanced_quotes: Option<bool>,
    node_list_rows_to_skip: Option<usize>,
    node_list_max_rows_number: Option<usize>,
    node_list_comment_symbol: Option<String>,
    default_node_type: Option<String>,
    original_nodes_column_number: Option<usize>,
    original_nodes_column: Option<String>,
    original_node_types_separator: Option<char>,
    original_node_list_node_types_column_number: Option<usize>,
    original_node_list_node_types_column: Option<String>,
    original_minimum_node_id: Option<NodeT>,
    original_numeric_node_ids: Option<bool>,
    original_node_list_numeric_node_type_ids: Option<bool>,
    original_skip_node_types_if_unavailable: Option<bool>,
    target_node_path: String,
    target_node_list_separator: Option<char>,
    target_node_list_header: Option<bool>,
    target_nodes_column_number: Option<usize>,
    target_nodes_column: Option<String>,
    target_node_types_separator: Option<char>,
    target_node_list_node_types_column_number: Option<usize>,
    target_node_list_node_types_column: Option<String>,
    nodes_number: Option<NodeT>,
) -> PyResult<(NodeT, Option<NodeTypeT>)> {
    Ok({
        let (subresult_0, subresult_1) = pe!(graph::convert_node_list_node_types_to_numeric(
            original_node_type_path.into(),
            original_node_type_list_separator.into(),
            original_node_types_column_number.into(),
            original_node_types_column.into(),
            node_types_number.into(),
            original_numeric_node_type_ids.into(),
            original_minimum_node_type_id.into(),
            original_node_type_list_header.into(),
            original_node_type_list_support_balanced_quotes.into(),
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
            original_node_list_support_balanced_quotes.into(),
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
        .into();
        (subresult_0.into(), subresult_1.into())
    })
}

#[module(edge_list_utils)]
#[pyfunction]
#[automatically_generated_binding]
#[text_signature = "(edge_path, edge_list_separator, edge_list_header, edge_list_support_balanced_quotes, edge_list_sources_column, edge_list_sources_column_number, edge_list_destinations_column, edge_list_destinations_column_number, edge_list_edge_type_column, edge_list_edge_type_column_number, edge_list_weights_column, edge_list_weights_column_number, comment_symbol, default_edge_type, default_weight, max_rows_number, rows_to_skip, edges_number, skip_edge_types_if_unavailable, skip_weights_if_unavailable, verbose, name)"]
/// Return whether the provided edge list contains duplicated edges.
///
/// Parameters
/// ----------
/// edge_path: str
///     The path from where to load the edge list.
/// edge_list_separator: Optional[str]
///     Separator to use for the edge list.
/// edge_list_header: Optional[bool]
///     Whether the edge list has an header.
/// edge_list_support_balanced_quotes: Optional[bool]
///     Whether to support balanced quotes.
/// edge_list_sources_column: Optional[str]
///     The column name to use to load the sources in the edges list.
/// edge_list_sources_column_number: Optional[int]
///     The column number to use to load the sources in the edges list.
/// edge_list_destinations_column: Optional[str]
///     The column name to use to load the destinations in the edges list.
/// edge_list_destinations_column_number: Optional[int]
///     The column number to use to load the destinations in the edges list.
/// edge_list_edge_type_column: Optional[str]
///     The column name to use for the edge types in the edges list.
/// edge_list_edge_type_column_number: Optional[int]
///     The column number to use for the edge types in the edges list.
/// edge_list_weights_column: Optional[str]
///     The column name to use for the weights in the edges list.
/// edge_list_weights_column_number: Optional[int]
///     The column number to use for the weights in the edges list.
/// comment_symbol: Optional[str]
///     The comment symbol to use within the edge list.
/// default_edge_type: Optional[str]
///     The default edge type to use within the edge list.
/// default_weight: Optional[float]
///     The default weight to use within the edge list.
/// max_rows_number: Optional[int]
///     The amount of rows to load from the edge list.
/// rows_to_skip: Optional[int]
///     The amount of rows to skip from the edge list.
/// edges_number: Optional[int]
///     The expected number of edges. It will be used for the loading bar.
/// skip_edge_types_if_unavailable: Optional[bool]
///     Whether to automatically skip the edge types if they are not available.
/// skip_weights_if_unavailable: Optional[bool]
///     Whether to automatically skip the weights if they are not available.
/// verbose: Optional[bool]
///     Whether to show the loading bar while processing the file.
/// name: Optional[str]
///     The name of the graph to display in the loading bar.
///
pub fn has_duplicated_edges_in_edge_list(
    edge_path: &str,
    edge_list_separator: Option<char>,
    edge_list_header: Option<bool>,
    edge_list_support_balanced_quotes: Option<bool>,
    edge_list_sources_column: Option<String>,
    edge_list_sources_column_number: Option<usize>,
    edge_list_destinations_column: Option<String>,
    edge_list_destinations_column_number: Option<usize>,
    edge_list_edge_type_column: Option<String>,
    edge_list_edge_type_column_number: Option<usize>,
    edge_list_weights_column: Option<String>,
    edge_list_weights_column_number: Option<usize>,
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
) -> PyResult<bool> {
    Ok(pe!(graph::has_duplicated_edges_in_edge_list(
        edge_path.into(),
        edge_list_separator.into(),
        edge_list_header.into(),
        edge_list_support_balanced_quotes.into(),
        edge_list_sources_column.into(),
        edge_list_sources_column_number.into(),
        edge_list_destinations_column.into(),
        edge_list_destinations_column_number.into(),
        edge_list_edge_type_column.into(),
        edge_list_edge_type_column_number.into(),
        edge_list_weights_column.into(),
        edge_list_weights_column_number.into(),
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

#[module(edge_list_utils)]
#[pyfunction]
#[automatically_generated_binding]
#[text_signature = "(path, target_path, separator, header, sources_column, sources_column_number, destinations_column, destinations_column_number, edge_types_column, edge_types_column_number, rows_to_skip, skip_edge_types_if_unavailable, sort_temporary_directory)"]
/// Sort given numeric edge list in place using the sort command.
///
/// Parameters
/// ----------
/// path: str
///     The path from where to load the edge list.
/// target_path: str
///     The where to store the edge list.
/// separator: Optional[str]
///     The separator for the rows in the edge list.
/// header: Optional[bool]
///     Whether the edge list has an header.
/// sources_column: Optional[str]
///     The column name to use for the source nodes.
/// sources_column_number: Optional[int]
///     The column number to use for the source nodes.
/// destinations_column: Optional[str]
///     The column name to use for the destination nodes.
/// destinations_column_number: Optional[int]
///     The column number to use for the destination nodes.
/// edge_types_column: Optional[str]
///     The column name to use for the edge types.
/// edge_types_column_number: Optional[int]
///     The column number to use for the edge types.
/// rows_to_skip: Optional[int]
///     Number of rows to skip in the edge list.
/// skip_edge_types_if_unavailable: Optional[bool]
///     Whether to automatically skip the edge types if they are not available.
/// sort_temporary_directory: Optional[str]
///     Where to store the temporary files that are created during parallel sorting.
///
pub fn sort_numeric_edge_list(
    path: &str,
    target_path: &str,
    separator: Option<char>,
    header: Option<bool>,
    sources_column: Option<String>,
    sources_column_number: Option<usize>,
    destinations_column: Option<String>,
    destinations_column_number: Option<usize>,
    edge_types_column: Option<String>,
    edge_types_column_number: Option<usize>,
    rows_to_skip: Option<usize>,
    skip_edge_types_if_unavailable: Option<bool>,
    sort_temporary_directory: Option<String>,
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
        skip_edge_types_if_unavailable.into(),
        sort_temporary_directory.into()
    ))?)
}

#[module(edge_list_utils)]
#[pyfunction]
#[automatically_generated_binding]
#[text_signature = "(path, separator, header, sources_column, sources_column_number, destinations_column, destinations_column_number, edge_types_column, edge_types_column_number, rows_to_skip, skip_edge_types_if_unavailable, sort_temporary_directory)"]
/// Sort given numeric edge list in place using the sort command.
///
/// Parameters
/// ----------
/// path: str
///     The path from where to load the edge list.
/// separator: Optional[str]
///     The separator for the rows in the edge list.
/// header: Optional[bool]
///     Whether the edge list has an header.
/// sources_column: Optional[str]
///     The column name to use for the source nodes.
/// sources_column_number: Optional[int]
///     The column number to use for the source nodes.
/// destinations_column: Optional[str]
///     The column name to use for the destination nodes.
/// destinations_column_number: Optional[int]
///     The column number to use for the destination nodes.
/// edge_types_column: Optional[str]
///     The column name to use for the edge types.
/// edge_types_column_number: Optional[int]
///     The column number to use for the edge types.
/// rows_to_skip: Optional[int]
///     Number of rows to skip in the edge list.
/// skip_edge_types_if_unavailable: Optional[bool]
///     Whether to automatically skip the edge types if they are not available.
/// sort_temporary_directory: Optional[str]
///     Where to store the temporary files that are created during parallel sorting.
///
pub fn sort_numeric_edge_list_inplace(
    path: &str,
    separator: Option<char>,
    header: Option<bool>,
    sources_column: Option<String>,
    sources_column_number: Option<usize>,
    destinations_column: Option<String>,
    destinations_column_number: Option<usize>,
    edge_types_column: Option<String>,
    edge_types_column_number: Option<usize>,
    rows_to_skip: Option<usize>,
    skip_edge_types_if_unavailable: Option<bool>,
    sort_temporary_directory: Option<String>,
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
        skip_edge_types_if_unavailable.into(),
        sort_temporary_directory.into()
    ))?)
}

#[pymodule]
fn utils(_py: Python, _m: &PyModule) -> PyResult<()> {
    Ok(())
}
