#[allow(unused_variables)]
use super::*;
use pyo3::class::basic::CompareOp;
#[allow(unused_imports)]
use pyo3::{wrap_pyfunction, wrap_pymodule};
use rayon::iter::*;
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

pub fn register_ensmallen(_py: Python, _m: &PyModule) -> PyResult<()> {
    _m.add_class::<Chain>()?;
    _m.add_class::<Circle>()?;
    _m.add_class::<Clique>()?;
    _m.add_class::<DendriticTree>()?;
    _m.add_class::<Graph>()?;
    _m.add_class::<NodeTuple>()?;
    _m.add_class::<ShortestPathsDjkstra>()?;
    _m.add_class::<ShortestPathsResultBFS>()?;
    _m.add_class::<Star>()?;
    _m.add_class::<Tendril>()?;
    env_logger::init();
    let submod = PyModule::new(_py, "edge_list_utils")?;
    register_edge_list_utils(_py, submod)?;
    _m.add_submodule(submod)?;
    let submod = PyModule::new(_py, "utils")?;
    register_utils(_py, submod)?;
    _m.add_submodule(submod)?;
    let submod = PyModule::new(_py, "preprocessing")?;
    register_preprocessing(_py, submod)?;
    _m.add_submodule(submod)?;
    let submod = PyModule::new(_py, "models")?;
    register_models(_py, submod)?;
    _m.add_submodule(submod)?;
    let submod = PyModule::new(_py, "express_measures")?;
    register_express_measures(_py, submod)?;
    _m.add_submodule(submod)?;
    Ok(())
}

#[pyclass(module = "ensmallen")]
///
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

impl<'a> From<&'a Chain> for &'a graph::Chain {
    fn from(val: &'a Chain) -> &'a graph::Chain {
        &val.inner
    }
}

#[pymethods]
impl Chain {
    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the first node ID of the chain
    pub fn get_root_node_id(&self) -> NodeT {
        self.inner.get_root_node_id().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the first node name of the chain
    pub fn get_root_node_name(&self) -> String {
        self.inner.get_root_node_name().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return length of the chain
    pub fn len(&self) -> NodeT {
        self.inner.len().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the node IDs of the nodes composing the chain
    pub fn get_chain_node_ids(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_chain_node_ids(), NodeT)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, k)")]
    /// Return the first `k` node IDs of the nodes composing the chain.
    ///
    /// Parameters
    /// ----------
    ///
    pub fn get_first_k_chain_node_ids(&self, k: usize) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_first_k_chain_node_ids(k.clone()), NodeT)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, k)")]
    /// Return the first `k` node names of the nodes composing the chain.
    ///
    /// Parameters
    /// ----------
    ///
    pub fn get_first_k_chain_node_names(&self, k: usize) -> Vec<String> {
        self.inner
            .get_first_k_chain_node_names(k.clone())
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
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
        ("node", 0.07583805),
        ("root", 0.42482838),
    ],
    &[
        ("get", 0.07583805),
        ("name", 0.611402),
        ("node", 0.07583805),
        ("root", 0.42482838),
    ],
    &[("len", 2.5416396)],
    &[
        ("chain", 0.21014561),
        ("get", 0.07583805),
        ("ids", 0.42482838),
        ("node", 0.07583805),
    ],
    &[
        ("chain", 0.110427275),
        ("first", 0.22323874),
        ("get", 0.039851367),
        ("ids", 0.22323874),
        ("k", 0.22323874),
        ("node", 0.039851367),
    ],
    &[
        ("chain", 0.110427275),
        ("first", 0.22323874),
        ("get", 0.039851367),
        ("k", 0.22323874),
        ("names", 0.22323874),
        ("node", 0.039851367),
    ],
    &[
        ("chain", 0.21014561),
        ("get", 0.07583805),
        ("names", 0.42482838),
        ("node", 0.07583805),
    ],
];

#[pymethods]
impl Chain {
    fn _repr_html_(&self) -> String {
        self.__repr__()
    }
}

#[pymethods]
impl Chain {
    fn __str__(&self) -> String {
        self.inner.to_string()
    }
    fn __repr__(&self) -> String {
        self.__str__()
    }

    fn __hash__(&self) -> PyResult<isize> {
        let mut hasher = DefaultHasher::new();
        self.inner.hash(&mut hasher);
        Ok(hasher.finish() as isize)
    }

    fn __richcmp__(&self, other: Self, op: CompareOp) -> bool {
        match op {
            CompareOp::Lt => self.inner < other.inner,
            CompareOp::Le => self.inner <= other.inner,
            CompareOp::Eq => self.inner == other.inner,
            CompareOp::Ne => self.inner != other.inner,
            CompareOp::Gt => self.inner > other.inner,
            CompareOp::Ge => self.inner >= other.inner,
        }
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
            "The method `{}` does not exists, did you mean one of the following?\n\n{}",
            &name,
            doc_scores
                .iter()
                .map(|(method_id, _)| {
                    format!("* `{}`", CHAIN_METHODS_NAMES[*method_id].to_string())
                })
                .take(10)
                .collect::<Vec<String>>()
                .join("\n"),
        )))
    }
}

#[pyclass(module = "ensmallen")]
///
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

impl<'a> From<&'a Circle> for &'a graph::Circle {
    fn from(val: &'a Circle) -> &'a graph::Circle {
        &val.inner
    }
}

#[pymethods]
impl Circle {
    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the first node ID of the Circle
    pub fn get_root_node_id(&self) -> NodeT {
        self.inner.get_root_node_id().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the first node name of the circle
    pub fn get_root_node_name(&self) -> String {
        self.inner.get_root_node_name().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return length of the Circle
    pub fn len(&self) -> NodeT {
        self.inner.len().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the node IDs of the nodes composing the Circle
    pub fn get_circle_node_ids(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_circle_node_ids(), NodeT)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, k)")]
    /// Return the first `k` node IDs of the nodes composing the Circle.
    ///
    /// Parameters
    /// ----------
    ///
    pub fn get_first_k_circle_node_ids(&self, k: usize) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(
            gil,
            self.inner.get_first_k_circle_node_ids(k.clone()),
            NodeT
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, k)")]
    /// Return the first `k` node names of the nodes composing the Circle.
    ///
    /// Parameters
    /// ----------
    ///
    pub fn get_first_k_circle_node_names(&self, k: usize) -> Vec<String> {
        self.inner
            .get_first_k_circle_node_names(k.clone())
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
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
        ("get", 0.07583805),
        ("id", 0.611402),
        ("node", 0.07583805),
        ("root", 0.42482838),
    ],
    &[
        ("get", 0.07583805),
        ("name", 0.611402),
        ("node", 0.07583805),
        ("root", 0.42482838),
    ],
    &[("len", 2.5416396)],
    &[
        ("circle", 0.21014561),
        ("get", 0.07583805),
        ("ids", 0.42482838),
        ("node", 0.07583805),
    ],
    &[
        ("circle", 0.110427275),
        ("first", 0.22323874),
        ("get", 0.039851367),
        ("ids", 0.22323874),
        ("k", 0.22323874),
        ("node", 0.039851367),
    ],
    &[
        ("circle", 0.110427275),
        ("first", 0.22323874),
        ("get", 0.039851367),
        ("k", 0.22323874),
        ("names", 0.22323874),
        ("node", 0.039851367),
    ],
    &[
        ("circle", 0.21014561),
        ("get", 0.07583805),
        ("names", 0.42482838),
        ("node", 0.07583805),
    ],
];

#[pymethods]
impl Circle {
    fn _repr_html_(&self) -> String {
        self.__repr__()
    }
}

#[pymethods]
impl Circle {
    fn __str__(&self) -> String {
        self.inner.to_string()
    }
    fn __repr__(&self) -> String {
        self.__str__()
    }

    fn __hash__(&self) -> PyResult<isize> {
        let mut hasher = DefaultHasher::new();
        self.inner.hash(&mut hasher);
        Ok(hasher.finish() as isize)
    }

    fn __richcmp__(&self, other: Self, op: CompareOp) -> bool {
        match op {
            CompareOp::Lt => self.inner < other.inner,
            CompareOp::Le => self.inner <= other.inner,
            CompareOp::Eq => self.inner == other.inner,
            CompareOp::Ne => self.inner != other.inner,
            CompareOp::Gt => self.inner > other.inner,
            CompareOp::Ge => self.inner >= other.inner,
        }
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
            "The method `{}` does not exists, did you mean one of the following?\n\n{}",
            &name,
            doc_scores
                .iter()
                .map(|(method_id, _)| {
                    format!("* `{}`", CIRCLE_METHODS_NAMES[*method_id].to_string())
                })
                .take(10)
                .collect::<Vec<String>>()
                .join("\n"),
        )))
    }
}

#[pyclass(module = "ensmallen")]
///
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

impl<'a> From<&'a Clique> for &'a graph::Clique {
    fn from(val: &'a Clique) -> &'a graph::Clique {
        &val.inner
    }
}

#[pymethods]
impl Clique {
    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return length of the Clique
    pub fn len(&self) -> NodeT {
        self.inner.len().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the node IDs of the nodes composing the clique
    pub fn get_node_ids(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_node_ids(), NodeT)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
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
        ("names", 0.37932625),
        ("node", 0.18176937),
    ],
];

#[pymethods]
impl Clique {
    fn _repr_html_(&self) -> String {
        self.__repr__()
    }
}

#[pymethods]
impl Clique {
    fn __str__(&self) -> String {
        self.inner.to_string()
    }
    fn __repr__(&self) -> String {
        self.__str__()
    }

    fn __hash__(&self) -> PyResult<isize> {
        let mut hasher = DefaultHasher::new();
        self.inner.hash(&mut hasher);
        Ok(hasher.finish() as isize)
    }

    fn __richcmp__(&self, other: Self, op: CompareOp) -> bool {
        match op {
            CompareOp::Lt => self.inner < other.inner,
            CompareOp::Le => self.inner <= other.inner,
            CompareOp::Eq => self.inner == other.inner,
            CompareOp::Ne => self.inner != other.inner,
            CompareOp::Gt => self.inner > other.inner,
            CompareOp::Ge => self.inner >= other.inner,
        }
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
            "The method `{}` does not exists, did you mean one of the following?\n\n{}",
            &name,
            doc_scores
                .iter()
                .map(|(method_id, _)| {
                    format!("* `{}`", CLIQUE_METHODS_NAMES[*method_id].to_string())
                })
                .take(10)
                .collect::<Vec<String>>()
                .join("\n"),
        )))
    }
}

#[pyclass(module = "ensmallen")]
///
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

impl<'a> From<&'a DendriticTree> for &'a graph::DendriticTree {
    fn from(val: &'a DendriticTree) -> &'a graph::DendriticTree {
        &val.inner
    }
}

#[pymethods]
impl DendriticTree {
    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the type of the dendritic tree
    pub fn get_dendritic_tree_type(&self) -> String {
        self.inner.get_dendritic_tree_type().to_string()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the root node ID of the dendritic tree
    pub fn get_root_node_id(&self) -> NodeT {
        self.inner.get_root_node_id().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return whether the current dendritic tree is actually a tree
    pub fn is_tree(&self) -> bool {
        self.inner.is_tree().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return whether the current dendritic tree is actually a tendril
    pub fn is_tendril(&self) -> bool {
        self.inner.is_tendril().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return whether the current dendritic tree is a proper dentritic tree
    pub fn is_dendritic_tree(&self) -> bool {
        self.inner.is_dendritic_tree().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return whether the current dendritic tree is actually a free-floating chain
    pub fn is_free_floating_chain(&self) -> bool {
        self.inner.is_free_floating_chain().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return whether the current dendritic tree is actually a star
    pub fn is_star(&self) -> bool {
        self.inner.is_star().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return whether the current dendritic tree is actually a star of tendrils
    pub fn is_tendril_star(&self) -> bool {
        self.inner.is_tendril_star().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return whether the current dendritic tree is actually a dendritic star
    pub fn is_dendritic_star(&self) -> bool {
        self.inner.is_dendritic_star().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return whether the current dendritic tree is actually a dendritic tendril star
    pub fn is_dendritic_tendril_star(&self) -> bool {
        self.inner.is_dendritic_tendril_star().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the depth of the dentritic tree
    pub fn get_depth(&self) -> NodeT {
        self.inner.get_depth().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the root node name of the DendriticTree
    pub fn get_root_node_name(&self) -> String {
        self.inner.get_root_node_name().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return number of nodes involved in the dendritic tree
    pub fn get_number_of_involved_nodes(&self) -> NodeT {
        self.inner.get_number_of_involved_nodes().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return number of edges involved in the dendritic tree
    pub fn get_number_of_involved_edges(&self) -> EdgeT {
        self.inner.get_number_of_involved_edges().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the node IDs of the nodes composing the DendriticTree
    pub fn get_dentritic_trees_node_ids(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_dentritic_trees_node_ids(), NodeT)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, k)")]
    /// Return the first `k` node IDs of the nodes composing the DendriticTree.
    ///
    /// Parameters
    /// ----------
    ///
    pub fn get_first_k_dentritic_trees_node_ids(&self, k: usize) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(
            gil,
            self.inner.get_first_k_dentritic_trees_node_ids(k.clone()),
            NodeT
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, k)")]
    /// Return the first `k` node names of the nodes composing the DendriticTree.
    ///
    /// Parameters
    /// ----------
    ///
    pub fn get_first_k_dentritic_trees_node_names(&self, k: usize) -> Vec<String> {
        self.inner
            .get_first_k_dentritic_trees_node_names(k.clone())
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
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
        ("get", 0.2099079),
        ("tree", 0.59874874),
        ("type", 0.8986398),
    ],
    &[
        ("get", 0.2099079),
        ("id", 0.8986398),
        ("node", 0.37964714),
        ("root", 0.7178391),
    ],
    &[("is", 0.6956208), ("tree", 1.4629598)],
    &[("is", 0.6956208), ("tendril", 1.4629598)],
    &[
        ("dendritic", 0.76747227),
        ("is", 0.4285964),
        ("tree", 0.9013809),
    ],
    &[
        ("chain", 0.8986398),
        ("floating", 0.8986398),
        ("free", 0.8986398),
        ("is", 0.28469825),
    ],
    &[("is", 0.6956208), ("star", 1.2456234)],
    &[
        ("is", 0.4285964),
        ("star", 0.76747227),
        ("tendril", 0.9013809),
    ],
    &[
        ("dendritic", 0.76747227),
        ("is", 0.4285964),
        ("star", 0.76747227),
    ],
    &[
        ("dendritic", 0.509799),
        ("is", 0.28469825),
        ("star", 0.509799),
        ("tendril", 0.59874874),
    ],
    &[("depth", 2.195702), ("get", 0.5128809)],
    &[
        ("get", 0.2099079),
        ("name", 0.8986398),
        ("node", 0.37964714),
        ("root", 0.7178391),
    ],
    &[
        ("get", 0.14818765),
        ("involved", 0.50676936),
        ("nodes", 0.63440835),
        ("number", 0.50676936),
        ("of", 0.50676936),
    ],
    &[
        ("edges", 0.63440835),
        ("get", 0.14818765),
        ("involved", 0.50676936),
        ("number", 0.50676936),
        ("of", 0.50676936),
    ],
    &[
        ("dentritic", 0.3599003),
        ("get", 0.14818765),
        ("ids", 0.50676936),
        ("node", 0.26801765),
        ("trees", 0.3599003),
    ],
    &[
        ("dentritic", 0.20459273),
        ("first", 0.2880835),
        ("get", 0.084240325),
        ("ids", 0.2880835),
        ("k", 0.2880835),
        ("node", 0.15236016),
        ("trees", 0.20459273),
    ],
    &[
        ("dentritic", 0.20459273),
        ("first", 0.2880835),
        ("get", 0.084240325),
        ("k", 0.2880835),
        ("names", 0.2880835),
        ("node", 0.15236016),
        ("trees", 0.20459273),
    ],
    &[
        ("dentritic", 0.3599003),
        ("get", 0.14818765),
        ("names", 0.50676936),
        ("node", 0.26801765),
        ("trees", 0.3599003),
    ],
];

#[pymethods]
impl DendriticTree {
    fn _repr_html_(&self) -> String {
        self.__repr__()
    }
}

#[pymethods]
impl DendriticTree {
    fn __str__(&self) -> String {
        self.inner.to_string()
    }
    fn __repr__(&self) -> String {
        self.__str__()
    }

    fn __hash__(&self) -> PyResult<isize> {
        let mut hasher = DefaultHasher::new();
        self.inner.hash(&mut hasher);
        Ok(hasher.finish() as isize)
    }

    fn __richcmp__(&self, other: Self, op: CompareOp) -> bool {
        match op {
            CompareOp::Lt => self.inner < other.inner,
            CompareOp::Le => self.inner <= other.inner,
            CompareOp::Eq => self.inner == other.inner,
            CompareOp::Ne => self.inner != other.inner,
            CompareOp::Gt => self.inner > other.inner,
            CompareOp::Ge => self.inner >= other.inner,
        }
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
            "The method `{}` does not exists, did you mean one of the following?\n\n{}",
            &name,
            doc_scores
                .iter()
                .map(|(method_id, _)| {
                    format!(
                        "* `{}`",
                        DENDRITICTREE_METHODS_NAMES[*method_id].to_string()
                    )
                })
                .take(10)
                .collect::<Vec<String>>()
                .join("\n"),
        )))
    }
}

#[pyclass(module = "ensmallen")]
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

impl<'a> From<&'a Graph> for &'a graph::Graph {
    fn from(val: &'a Graph) -> &'a graph::Graph {
        &val.inner
    }
}

#[pymethods]
impl Graph {
    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
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
            .is_unchecked_connected_from_node_id(node_id.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    /// Returns boolean representing if given node is not a singleton nor a singleton with selfloop.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     The node to be checked for.
    ///
    ///
    /// Raises
    /// -------
    ///
    pub fn is_connected_from_node_id(&self, node_id: NodeT) -> PyResult<bool> {
        Ok(pe!(self.inner.is_connected_from_node_id(node_id.clone()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
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
            .is_unchecked_disconnected_node_from_node_id(node_id.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
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
            .is_unchecked_singleton_from_node_id(node_id.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    /// Returns boolean representing if given node is a singleton.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     The node to be checked for.
    ///
    pub fn is_singleton_from_node_id(&self, node_id: NodeT) -> PyResult<bool> {
        Ok(pe!(self.inner.is_singleton_from_node_id(node_id.clone()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
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
            .is_unchecked_singleton_with_selfloops_from_node_id(node_id.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
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
            .is_singleton_with_selfloops_from_node_id(node_id.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_name)")]
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
            .is_unchecked_singleton_from_node_name(node_name)
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_name)")]
    /// Returns boolean representing if given node is a singleton.
    ///
    /// Parameters
    /// ----------
    /// node_name: str
    ///     The node name to be checked for.
    ///
    pub fn is_singleton_from_node_name(&self, node_name: &str) -> PyResult<bool> {
        Ok(pe!(self.inner.is_singleton_from_node_name(node_name))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_name)")]
    /// Returns whether the graph has the given node name.
    ///
    /// Parameters
    /// ----------
    /// node_name: str
    ///     Name of the node.
    ///
    pub fn has_node_name(&self, node_name: &str) -> bool {
        self.inner.has_node_name(node_name).into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_id)")]
    /// Returns whether the graph has the given node type id.
    ///
    /// Parameters
    /// ----------
    /// node_type_id: int
    ///     id of the node.
    ///
    pub fn has_node_type_id(&self, node_type_id: NodeTypeT) -> bool {
        self.inner.has_node_type_id(node_type_id.clone()).into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_name)")]
    /// Returns whether the graph has the given node type name.
    ///
    /// Parameters
    /// ----------
    /// node_type_name: str
    ///     Name of the node.
    ///
    pub fn has_node_type_name(&self, node_type_name: &str) -> bool {
        self.inner.has_node_type_name(node_type_name).into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type_id)")]
    /// Returns whether the graph has the given edge type id.
    ///
    /// Parameters
    /// ----------
    /// edge_type_id: int
    ///     id of the edge.
    ///
    pub fn has_edge_type_id(&self, edge_type_id: EdgeTypeT) -> bool {
        self.inner.has_edge_type_id(edge_type_id.clone()).into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type_name)")]
    /// Returns whether the graph has the given edge type name.
    ///
    /// Parameters
    /// ----------
    /// edge_type_name: str
    ///     Name of the edge.
    ///
    pub fn has_edge_type_name(&self, edge_type_name: &str) -> bool {
        self.inner.has_edge_type_name(edge_type_name).into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src, dst)")]
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
            .has_edge_from_node_ids(src.clone(), dst.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    /// Returns whether the given node ID has a selfloop.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     Source node id.
    ///
    pub fn has_selfloop_from_node_id(&self, node_id: NodeT) -> bool {
        self.inner.has_selfloop_from_node_id(node_id.clone()).into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src, dst, edge_type)")]
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
            .has_edge_from_node_ids_and_edge_type_id(src.clone(), dst.clone(), edge_type)
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
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
            .is_unchecked_trap_node_from_node_id(node_id.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    /// Returns boolean representing if given node is a trap.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     Integer ID of the node, if this is bigger that the number of nodes it will panic.
    ///
    pub fn is_trap_node_from_node_id(&self, node_id: NodeT) -> PyResult<bool> {
        Ok(pe!(self.inner.is_trap_node_from_node_id(node_id.clone()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_name, node_type_name)")]
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
            .has_node_name_and_node_type_name(node_name, node_type_name)
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src_name, dst_name)")]
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
            .has_edge_from_node_names(src_name, dst_name)
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src_name, dst_name, edge_type_name)")]
    /// Returns whether if edge with type passing between given nodes exists.
    ///
    /// Parameters
    /// ----------
    /// src_name: str
    ///     The source node name of the edge.
    /// dst_name: str
    ///     The destination node name of the edge.
    /// edge_type_name: Optional[&str]
    ///     The (optional) edge type name.
    ///
    pub fn has_edge_from_node_names_and_edge_type_name(
        &self,
        src_name: &str,
        dst_name: &str,
        edge_type_name: Option<&str>,
    ) -> bool {
        self.inner
            .has_edge_from_node_names_and_edge_type_name(src_name, dst_name, edge_type_name)
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src, edge_type_id)")]
    /// Returns whether a given node ID has at least an edge of the given edge type.
    ///
    /// Parameters
    /// ----------
    /// src: int
    ///     The source node of which to check connected edges' type.
    /// edge_type_id: Optional[int]
    ///     The edge type to look for.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given source node does not exist in the current graph.
    /// ValueError
    ///     If the given edge type does not exist in the current graph.
    ///
    pub fn has_edge_from_node_id_and_edge_type_id(
        &self,
        src: NodeT,
        edge_type_id: Option<EdgeTypeT>,
    ) -> PyResult<bool> {
        Ok(pe!(self
            .inner
            .has_edge_from_node_id_and_edge_type_id(src.clone(), edge_type_id))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src, edge_type_id)")]
    /// Returns whether a given node ID has at least an edge of the given edge type.
    ///
    /// Parameters
    /// ----------
    /// src: int
    ///     The source node of which to check connected edges' type.
    /// edge_type_id: Optional[int]
    ///     The edge type to look for.
    ///
    ///
    /// Safety
    /// ------
    /// When
    pub unsafe fn has_unchecked_edge_from_node_id_and_edge_type_id(
        &self,
        src: NodeT,
        edge_type_id: Option<EdgeTypeT>,
    ) -> bool {
        self.inner
            .has_unchecked_edge_from_node_id_and_edge_type_id(src.clone(), edge_type_id)
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
    /// Returns Jaccard coo matrix
    pub fn get_jaccard_coo_matrix(&self) -> (Py<PyArray2<NodeT>>, Py<PyArray1<WeightT>>) {
        let (subresult_0, subresult_1) = self.inner.get_jaccard_coo_matrix();
        (
            {
                // Warning: this copies the array so it uses double the memory.
                // To avoid this you should directly generate data compatible with a numpy array
                // Which is a flat vector with row-first or column-first unrolling
                let gil = pyo3::Python::acquire_gil();
                let body = subresult_0;
                let result_array = ThreadDataRaceAware {
                    t: unsafe { PyArray2::<NodeT>::new(gil.python(), [body.len(), 2], false) },
                };
                body.into_par_iter()
                    .enumerate()
                    .for_each(|(i, (a, b))| unsafe {
                        *(result_array.t.uget_mut([i, 0])) = a;
                        *(result_array.t.uget_mut([i, 1])) = b;
                    });
                result_array.t.to_owned()
            },
            {
                let gil = pyo3::Python::acquire_gil();
                to_ndarray_1d!(gil, subresult_1, WeightT)
            },
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns jaccard weighted graph
    pub fn get_jaccard_graph(&self) -> Graph {
        self.inner.get_jaccard_graph().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns neighbours intersection size coo matrix
    pub fn get_neighbours_intersection_size_coo_matrix(
        &self,
    ) -> (Py<PyArray2<NodeT>>, Py<PyArray1<WeightT>>) {
        let (subresult_0, subresult_1) = self.inner.get_neighbours_intersection_size_coo_matrix();
        (
            {
                // Warning: this copies the array so it uses double the memory.
                // To avoid this you should directly generate data compatible with a numpy array
                // Which is a flat vector with row-first or column-first unrolling
                let gil = pyo3::Python::acquire_gil();
                let body = subresult_0;
                let result_array = ThreadDataRaceAware {
                    t: unsafe { PyArray2::<NodeT>::new(gil.python(), [body.len(), 2], false) },
                };
                body.into_par_iter()
                    .enumerate()
                    .for_each(|(i, (a, b))| unsafe {
                        *(result_array.t.uget_mut([i, 0])) = a;
                        *(result_array.t.uget_mut([i, 1])) = b;
                    });
                result_array.t.to_owned()
            },
            {
                let gil = pyo3::Python::acquire_gil();
                to_ndarray_1d!(gil, subresult_1, WeightT)
            },
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns neighbours intersection size weighted graph
    pub fn get_neighbours_intersection_size_graph(&self) -> Graph {
        self.inner.get_neighbours_intersection_size_graph().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, bfs)")]
    /// Returns shared ancestors size coo matrix.
    ///
    /// Parameters
    /// ----------
    /// bfs: ShortestPathsResultBFS
    ///     The BFS object to use for the ancestors.
    ///
    pub fn get_shared_ancestors_size_coo_matrix(
        &self,
        bfs: &ShortestPathsResultBFS,
    ) -> (Py<PyArray2<NodeT>>, Py<PyArray1<WeightT>>) {
        let (subresult_0, subresult_1) =
            self.inner.get_shared_ancestors_size_coo_matrix(bfs.into());
        (
            {
                // Warning: this copies the array so it uses double the memory.
                // To avoid this you should directly generate data compatible with a numpy array
                // Which is a flat vector with row-first or column-first unrolling
                let gil = pyo3::Python::acquire_gil();
                let body = subresult_0;
                let result_array = ThreadDataRaceAware {
                    t: unsafe { PyArray2::<NodeT>::new(gil.python(), [body.len(), 2], false) },
                };
                body.into_par_iter()
                    .enumerate()
                    .for_each(|(i, (a, b))| unsafe {
                        *(result_array.t.uget_mut([i, 0])) = a;
                        *(result_array.t.uget_mut([i, 1])) = b;
                    });
                result_array.t.to_owned()
            },
            {
                let gil = pyo3::Python::acquire_gil();
                to_ndarray_1d!(gil, subresult_1, WeightT)
            },
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, bfs)")]
    /// Returns shared ancestors size weighted graph.
    ///
    /// Parameters
    /// ----------
    /// bfs: ShortestPathsResultBFS
    ///     The BFS object to use for the ancestors.
    ///
    pub fn get_shared_ancestors_size_graph(&self, bfs: &ShortestPathsResultBFS) -> Graph {
        self.inner
            .get_shared_ancestors_size_graph(bfs.into())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, bfs)")]
    /// Returns Ancestors Jaccard coo matrix.
    ///
    /// Parameters
    /// ----------
    /// bfs: ShortestPathsResultBFS
    ///     The BFS object to use for the ancestors.
    ///
    pub fn get_ancestors_jaccard_coo_matrix(
        &self,
        bfs: &ShortestPathsResultBFS,
    ) -> (Py<PyArray2<NodeT>>, Py<PyArray1<WeightT>>) {
        let (subresult_0, subresult_1) = self.inner.get_ancestors_jaccard_coo_matrix(bfs.into());
        (
            {
                // Warning: this copies the array so it uses double the memory.
                // To avoid this you should directly generate data compatible with a numpy array
                // Which is a flat vector with row-first or column-first unrolling
                let gil = pyo3::Python::acquire_gil();
                let body = subresult_0;
                let result_array = ThreadDataRaceAware {
                    t: unsafe { PyArray2::<NodeT>::new(gil.python(), [body.len(), 2], false) },
                };
                body.into_par_iter()
                    .enumerate()
                    .for_each(|(i, (a, b))| unsafe {
                        *(result_array.t.uget_mut([i, 0])) = a;
                        *(result_array.t.uget_mut([i, 1])) = b;
                    });
                result_array.t.to_owned()
            },
            {
                let gil = pyo3::Python::acquire_gil();
                to_ndarray_1d!(gil, subresult_1, WeightT)
            },
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, bfs)")]
    /// Returns Ancestors Jaccard weighted graph.
    ///
    /// Parameters
    /// ----------
    /// bfs: ShortestPathsResultBFS
    ///     The BFS object to use for the ancestors.
    ///
    pub fn get_ancestors_jaccard_graph(&self, bfs: &ShortestPathsResultBFS) -> Graph {
        self.inner.get_ancestors_jaccard_graph(bfs.into()).into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns Adamic-adar coo matrix
    pub fn get_adamic_adar_coo_matrix(&self) -> (Py<PyArray2<NodeT>>, Py<PyArray1<WeightT>>) {
        let (subresult_0, subresult_1) = self.inner.get_adamic_adar_coo_matrix();
        (
            {
                // Warning: this copies the array so it uses double the memory.
                // To avoid this you should directly generate data compatible with a numpy array
                // Which is a flat vector with row-first or column-first unrolling
                let gil = pyo3::Python::acquire_gil();
                let body = subresult_0;
                let result_array = ThreadDataRaceAware {
                    t: unsafe { PyArray2::<NodeT>::new(gil.python(), [body.len(), 2], false) },
                };
                body.into_par_iter()
                    .enumerate()
                    .for_each(|(i, (a, b))| unsafe {
                        *(result_array.t.uget_mut([i, 0])) = a;
                        *(result_array.t.uget_mut([i, 1])) = b;
                    });
                result_array.t.to_owned()
            },
            {
                let gil = pyo3::Python::acquire_gil();
                to_ndarray_1d!(gil, subresult_1, WeightT)
            },
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns Adamic-Adar weighted graph
    pub fn get_adamic_adar_graph(&self) -> Graph {
        self.inner.get_adamic_adar_graph().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns Laplacian coo matrix
    pub fn get_laplacian_coo_matrix(&self) -> (Py<PyArray2<NodeT>>, Py<PyArray1<WeightT>>) {
        let (subresult_0, subresult_1) = self.inner.get_laplacian_coo_matrix();
        (
            {
                // Warning: this copies the array so it uses double the memory.
                // To avoid this you should directly generate data compatible with a numpy array
                // Which is a flat vector with row-first or column-first unrolling
                let gil = pyo3::Python::acquire_gil();
                let body = subresult_0;
                let result_array = ThreadDataRaceAware {
                    t: unsafe { PyArray2::<NodeT>::new(gil.python(), [body.len(), 2], false) },
                };
                body.into_par_iter()
                    .enumerate()
                    .for_each(|(i, (a, b))| unsafe {
                        *(result_array.t.uget_mut([i, 0])) = a;
                        *(result_array.t.uget_mut([i, 1])) = b;
                    });
                result_array.t.to_owned()
            },
            {
                let gil = pyo3::Python::acquire_gil();
                to_ndarray_1d!(gil, subresult_1, WeightT)
            },
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns Laplacian weighted graph
    pub fn get_laplacian_graph(&self) -> Graph {
        self.inner.get_laplacian_graph().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns left normalized Laplacian coo matrix
    pub fn get_left_normalized_laplacian_coo_matrix(
        &self,
    ) -> (Py<PyArray2<NodeT>>, Py<PyArray1<WeightT>>) {
        let (subresult_0, subresult_1) = self.inner.get_left_normalized_laplacian_coo_matrix();
        (
            {
                // Warning: this copies the array so it uses double the memory.
                // To avoid this you should directly generate data compatible with a numpy array
                // Which is a flat vector with row-first or column-first unrolling
                let gil = pyo3::Python::acquire_gil();
                let body = subresult_0;
                let result_array = ThreadDataRaceAware {
                    t: unsafe { PyArray2::<NodeT>::new(gil.python(), [body.len(), 2], false) },
                };
                body.into_par_iter()
                    .enumerate()
                    .for_each(|(i, (a, b))| unsafe {
                        *(result_array.t.uget_mut([i, 0])) = a;
                        *(result_array.t.uget_mut([i, 1])) = b;
                    });
                result_array.t.to_owned()
            },
            {
                let gil = pyo3::Python::acquire_gil();
                to_ndarray_1d!(gil, subresult_1, WeightT)
            },
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns left normalized Laplacian weighted graph
    pub fn get_left_normalized_laplacian_graph(&self) -> Graph {
        self.inner.get_left_normalized_laplacian_graph().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns right normalized Laplacian coo matrix
    pub fn get_right_normalized_laplacian_coo_matrix(
        &self,
    ) -> (Py<PyArray2<NodeT>>, Py<PyArray1<WeightT>>) {
        let (subresult_0, subresult_1) = self.inner.get_right_normalized_laplacian_coo_matrix();
        (
            {
                // Warning: this copies the array so it uses double the memory.
                // To avoid this you should directly generate data compatible with a numpy array
                // Which is a flat vector with row-first or column-first unrolling
                let gil = pyo3::Python::acquire_gil();
                let body = subresult_0;
                let result_array = ThreadDataRaceAware {
                    t: unsafe { PyArray2::<NodeT>::new(gil.python(), [body.len(), 2], false) },
                };
                body.into_par_iter()
                    .enumerate()
                    .for_each(|(i, (a, b))| unsafe {
                        *(result_array.t.uget_mut([i, 0])) = a;
                        *(result_array.t.uget_mut([i, 1])) = b;
                    });
                result_array.t.to_owned()
            },
            {
                let gil = pyo3::Python::acquire_gil();
                to_ndarray_1d!(gil, subresult_1, WeightT)
            },
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns right normalized Laplacian weighted graph
    pub fn get_right_normalized_laplacian_graph(&self) -> Graph {
        self.inner.get_right_normalized_laplacian_graph().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns symmetric normalized Laplacian coo matrix
    pub fn get_symmetric_normalized_laplacian_coo_matrix(
        &self,
    ) -> (Py<PyArray2<NodeT>>, Py<PyArray1<WeightT>>) {
        let (subresult_0, subresult_1) = self.inner.get_symmetric_normalized_laplacian_coo_matrix();
        (
            {
                // Warning: this copies the array so it uses double the memory.
                // To avoid this you should directly generate data compatible with a numpy array
                // Which is a flat vector with row-first or column-first unrolling
                let gil = pyo3::Python::acquire_gil();
                let body = subresult_0;
                let result_array = ThreadDataRaceAware {
                    t: unsafe { PyArray2::<NodeT>::new(gil.python(), [body.len(), 2], false) },
                };
                body.into_par_iter()
                    .enumerate()
                    .for_each(|(i, (a, b))| unsafe {
                        *(result_array.t.uget_mut([i, 0])) = a;
                        *(result_array.t.uget_mut([i, 1])) = b;
                    });
                result_array.t.to_owned()
            },
            {
                let gil = pyo3::Python::acquire_gil();
                to_ndarray_1d!(gil, subresult_1, WeightT)
            },
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns symmetric normalized Laplacian weighted graph
    pub fn get_symmetric_normalized_laplacian_graph(&self) -> Graph {
        self.inner.get_symmetric_normalized_laplacian_graph().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns graph with node IDs sorted by increasing outbound node degree
    pub fn sort_by_increasing_outbound_node_degree(&self) -> Graph {
        self.inner.sort_by_increasing_outbound_node_degree().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns graph with node IDs sorted by decreasing outbound node degree
    pub fn sort_by_decreasing_outbound_node_degree(&self) -> Graph {
        self.inner.sort_by_decreasing_outbound_node_degree().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns graph with node IDs sorted by lexicographic order
    pub fn sort_by_node_lexicographic_order(&self) -> Graph {
        self.inner.sort_by_node_lexicographic_order().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, root_node_id)")]
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
                    .get_bfs_topological_sorting_from_node_id(root_node_id.clone()))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, root_node_id)")]
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
                    .get_reversed_bfs_topological_sorting_from_node_id(root_node_id.clone()))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, root_node_id)")]
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
            .sort_by_bfs_topological_sorting_from_node_id(root_node_id.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, node_names, node_types, edge_types, minimum_component_size, top_k_components, verbose)"
    )]
    /// remove all the components that are not connected to interesting
    /// nodes and edges.
    ///
    /// Parameters
    /// ----------
    /// node_names: Optional[List[str]]
    ///     The name of the nodes of which components to keep.
    /// node_types: Optional[&List[Optional[&str]]]
    ///     The types of the nodes of which components to keep.
    /// edge_types: Optional[&List[Optional[&str]]]
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
        node_types: Option<Vec<Option<&str>>>,
        edge_types: Option<Vec<Option<&str>>>,
        minimum_component_size: Option<NodeT>,
        top_k_components: Option<NodeT>,
        verbose: Option<bool>,
    ) -> PyResult<Graph> {
        Ok(pe!(self.inner.remove_components(
            node_names,
            node_types.as_ref().map(|x| x.as_slice()),
            edge_types.as_ref().map(|x| x.as_slice()),
            minimum_component_size,
            top_k_components,
            verbose
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, other)")]
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
    #[pyo3(text_signature = "($self, other)")]
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
    #[pyo3(
        text_signature = "($self, removed_existing_edges, first_nodes_set, second_nodes_set, first_node_types_set, second_node_types_set)"
    )]
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
                    removed_existing_edges,
                    first_nodes_set,
                    second_nodes_set,
                    first_node_types_set,
                    second_node_types_set
                ))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, removed_existing_edges, first_nodes_set, second_nodes_set, first_node_types_set, second_node_types_set)"
    )]
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
            removed_existing_edges,
            first_nodes_set,
            second_nodes_set,
            first_node_types_set,
            second_node_types_set
        ))?
        .into_iter()
        .map(|x| x.into_iter().map(|x| x.into()).collect::<Vec<_>>())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, central_node, removed_existing_edges, star_points_nodes_set, star_points_node_types_set)"
    )]
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
                    removed_existing_edges,
                    star_points_nodes_set,
                    star_points_node_types_set
                ))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, central_node, removed_existing_edges, star_points_nodes_set, star_points_node_types_set)"
    )]
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
            removed_existing_edges,
            star_points_nodes_set,
            star_points_node_types_set
        ))?
        .into_iter()
        .map(|x| x.into_iter().map(|x| x.into()).collect::<Vec<_>>())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, directed, allow_selfloops, removed_existing_edges, allow_node_type_set, allow_node_set)"
    )]
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
    #[pyo3(
        text_signature = "($self, directed, allow_selfloops, removed_existing_edges, allow_node_type_set, allow_node_set)"
    )]
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
                directed,
                allow_selfloops,
                removed_existing_edges,
                allow_node_type_set,
                allow_node_set,
            )
            .into_iter()
            .map(|x| x.into_iter().map(|x| x.into()).collect::<Vec<_>>())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src, dst)")]
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
        self.inner.encode_edge(src.clone(), dst.clone()).into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge)")]
    /// Returns source and destination nodes corresponding to given edge ID.
    ///
    /// Parameters
    /// ----------
    /// edge: int
    ///     The edge value to decode.
    ///
    pub fn decode_edge(&self, edge: u64) -> (NodeT, NodeT) {
        let (subresult_0, subresult_1) = self.inner.decode_edge(edge.clone());
        (subresult_0.into(), subresult_1.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return maximum encodable edge number
    pub fn get_max_encodable_edge_number(&self) -> EdgeT {
        self.inner.get_max_encodable_edge_number().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
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
        Ok(pe!(self.inner.validate_node_id(node_id.clone()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_ids)")]
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
            to_ndarray_1d!(gil, pe!(self.inner.validate_node_ids(node_ids))?, NodeT)
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_id)")]
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
        Ok(pe!(self.inner.validate_edge_id(edge_id.clone()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_ids)")]
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
            to_ndarray_1d!(gil, pe!(self.inner.validate_edge_ids(edge_ids))?, EdgeT)
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self, node_type_id)")]
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
        Ok(pe!(self.inner.validate_node_type_id(node_type_id))?.map(|x| x.into()))
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_ids)")]
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
        Ok(pe!(self.inner.validate_node_type_ids(&node_type_ids))?
            .into_iter()
            .map(|x| x.map(|x| x.into()))
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type_id)")]
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
        Ok(pe!(self.inner.validate_edge_type_id(edge_type_id))?.map(|x| x.into()))
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type_ids)")]
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
        Ok(pe!(self.inner.validate_edge_type_ids(edge_type_ids))?
            .into_iter()
            .map(|x| x.map(|x| x.into()))
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Raises an error if the graph's nodes do not have detectable ontologies.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not contain nodes with detectable ontologies.
    ///
    pub fn must_have_node_ontologies(&self) -> PyResult<()> {
        Ok(pe!(self.inner.must_have_node_ontologies())?)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Raises an error if the graph is not undirected.
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
    #[pyo3(text_signature = "($self)")]
    /// Raises an error if the graph is not a directed acyclic.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph is directed.
    ///
    pub fn must_be_directed_acyclic(&self) -> PyResult<()> {
        Ok(pe!(self.inner.must_be_directed_acyclic())?)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self, other)")]
    /// Raises an error if the provided graph does not a node vocabulary compatible with the current graph instance.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the provided graph does not share a compatible node vocabulary with the current instance.
    ///
    pub fn must_share_node_vocabulary(&self, other: &Graph) -> PyResult<()> {
        Ok(pe!(self.inner.must_share_node_vocabulary(&other.inner))?)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
    /// Return the maximum weighted node degree
    pub fn get_weighted_maximum_node_degree(&self) -> PyResult<f64> {
        Ok(pe!(self.inner.get_weighted_maximum_node_degree())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the minimum weighted node degree
    pub fn get_weighted_minimum_node_degree(&self) -> PyResult<f64> {
        Ok(pe!(self.inner.get_weighted_minimum_node_degree())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the number of weighted singleton nodes, i.e. nodes with weighted node degree equal to zero
    pub fn get_number_of_weighted_singleton_nodes(&self) -> PyResult<NodeT> {
        Ok(pe!(self.inner.get_number_of_weighted_singleton_nodes())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns number of self-loops, including also those in eventual multi-edges.
    pub fn get_number_of_selfloops(&self) -> EdgeT {
        self.inner.get_number_of_selfloops().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns number of unique self-loops, excluding those in eventual multi-edges.
    pub fn get_number_of_unique_selfloops(&self) -> NodeT {
        self.inner.get_number_of_unique_selfloops().into()
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, features, neighbours_number, max_degree, distance_name, verbose)"
    )]
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
    /// distance_name: Optional[&str]
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
            features,
            neighbours_number,
            max_degree,
            distance_name,
            verbose
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, name)")]
    /// Set the name of the graph.
    ///
    /// Parameters
    /// ----------
    /// name: str
    ///     Name of the graph.
    ///
    pub fn set_name(&mut self, name: String) {
        self.inner.set_name(name.into());
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type)")]
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
    #[pyo3(text_signature = "($self, edge_type)")]
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
    #[pyo3(text_signature = "($self, node_type)")]
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
    #[pyo3(text_signature = "($self, node_type)")]
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
    #[pyo3(text_signature = "($self, node_type_ids_to_remove)")]
    /// Remove given node type ID from all nodes.
    ///
    /// If any given node remains with no node type, that node is labeled
    /// with node type None. Note that the modification happens inplace.
    ///
    /// Parameters
    /// ----------
    /// node_type_ids_to_remove: List[int]
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
                .remove_inplace_node_type_ids(node_type_ids_to_remove))?;
            ()
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self, node_type_id, node_name_prefixes)")]
    /// Assigns inplace given node type id to the nodes with given prefixes.
    ///
    /// Parameters
    /// ----------
    /// node_type_id: int
    ///     The node type ID to assign.
    /// node_name_prefixes: List[str]
    ///     The node name prefixes to check for.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given list of node name prefixes is empty.
    ///
    pub fn add_node_type_id_from_node_name_prefixes_inplace(
        &mut self,
        node_type_id: NodeTypeT,
        node_name_prefixes: Vec<String>,
    ) -> PyResult<()> {
        Ok({
            pe!(self.inner.add_node_type_id_from_node_name_prefixes_inplace(
                node_type_id.clone(),
                node_name_prefixes
            ))?;
            ()
        })
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, edge_type_id, source_node_type_ids, destination_node_type_ids)"
    )]
    /// Replaces inplace given edge type id to the nodes with given source and destination node type IDs.
    ///
    /// Parameters
    /// ----------
    /// edge_type_id: int
    ///     The edge type ID to replace with.
    /// source_node_type_ids: List[Optional[int]]
    ///     Node types of the source nodes. When an edge has a source node with any of this node types, we may change its edge type if also the destination nodes have the correct node types.
    /// destination_node_type_ids: List[Optional[int]]
    ///     Node types of the destination nodes. When an edge has a destination node with any of this node types, we may change its edge type if also the source nodes have the correct node types.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given list of node name prefixes is empty.
    ///
    pub fn replace_edge_type_id_from_edge_node_type_ids_inplace(
        &mut self,
        edge_type_id: EdgeTypeT,
        source_node_type_ids: Vec<Option<NodeTypeT>>,
        destination_node_type_ids: Vec<Option<NodeTypeT>>,
    ) -> PyResult<()> {
        Ok({
            pe!(self
                .inner
                .replace_edge_type_id_from_edge_node_type_ids_inplace(
                    edge_type_id.clone(),
                    &source_node_type_ids,
                    &destination_node_type_ids
                ))?;
            ()
        })
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, edge_type_id, source_node_type_ids, destination_node_type_ids)"
    )]
    /// Replaces given edge type id to the nodes with given source and destination node type IDs.
    ///
    /// Parameters
    /// ----------
    /// edge_type_id: int
    ///     The edge type ID to replace with.
    /// source_node_type_ids: List[Optional[int]]
    ///     Node types of the source nodes. When an edge has a source node with any of this node types, we may change its edge type if also the destination nodes have the correct node types.
    /// destination_node_type_ids: List[Optional[int]]
    ///     Node types of the destination nodes. When an edge has a destination node with any of this node types, we may change its edge type if also the source nodes have the correct node types.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given list of node name prefixes is empty.
    ///
    pub fn replace_edge_type_id_from_edge_node_type_ids(
        &mut self,
        edge_type_id: EdgeTypeT,
        source_node_type_ids: Vec<Option<NodeTypeT>>,
        destination_node_type_ids: Vec<Option<NodeTypeT>>,
    ) -> PyResult<()> {
        Ok({
            pe!(self.inner.replace_edge_type_id_from_edge_node_type_ids(
                edge_type_id.clone(),
                &source_node_type_ids,
                &destination_node_type_ids
            ))?;
            ()
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_id, node_name_prefixes)")]
    /// Assigns given node type id to the nodes with given prefixes.
    ///
    /// Parameters
    /// ----------
    /// node_type_id: int
    ///     The node type ID to assign.
    /// node_name_prefixes: List[str]
    ///     The node name prefixes to check for.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given list of node name prefixes is empty.
    ///
    pub fn add_node_type_id_from_node_name_prefixes(
        &self,
        node_type_id: NodeTypeT,
        node_name_prefixes: Vec<String>,
    ) -> PyResult<Graph> {
        Ok(pe!(self
            .inner
            .add_node_type_id_from_node_name_prefixes(node_type_id.clone(), node_name_prefixes))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_name)")]
    /// Add node type name to the graph in place.
    ///
    /// Parameters
    /// ----------
    /// node_type_name: str
    ///     The node type name to add.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given node type name already exists in the graph.
    ///
    pub fn add_node_type_name_inplace(&mut self, node_type_name: String) -> PyResult<NodeTypeT> {
        Ok(pe!(self.inner.add_node_type_name_inplace(node_type_name.into()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_name, node_name_prefixes)")]
    /// Assigns inplace given node type name to the nodes with given prefixes.
    ///
    /// Parameters
    /// ----------
    /// node_type_name: str
    ///     The node type ID to assign.
    /// node_name_prefixes: List[str]
    ///     The node name prefixes to check for.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given list of node name prefixes is empty.
    ///
    pub fn add_node_type_name_from_node_name_prefixes_inplace(
        &mut self,
        node_type_name: String,
        node_name_prefixes: Vec<String>,
    ) -> PyResult<()> {
        Ok({
            pe!(self
                .inner
                .add_node_type_name_from_node_name_prefixes_inplace(
                    node_type_name.into(),
                    node_name_prefixes
                ))?;
            ()
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type_name)")]
    /// Add edge type name to the graph in place.
    ///
    /// Parameters
    /// ----------
    /// edge_type_name: str
    ///     The edge type name to add.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given edge type name already exists in the graph.
    ///
    pub fn add_edge_type_name_inplace(&mut self, edge_type_name: String) -> PyResult<EdgeTypeT> {
        Ok(pe!(self.inner.add_edge_type_name_inplace(edge_type_name.into()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, edge_type_name, source_node_type_names, destination_node_type_names)"
    )]
    /// Replaces inplace given edge type name to the nodes with given source and destination node type names.
    ///
    /// Parameters
    /// ----------
    /// edge_type_name: str
    ///     The edge type name to replace with.
    /// source_node_type_names: List[Optional[&str]]
    ///     Node types of the source nodes. When an edge has a source node with any of this node types, we may change its edge type if also the destination nodes have the correct node types.
    /// destination_node_type_names: List[Optional[&str]]
    ///     Node types of the destination nodes. When an edge has a destination node with any of this node types, we may change its edge type if also the source nodes have the correct node types.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given list of node name prefixes is empty.
    ///
    pub fn replace_edge_type_name_from_edge_node_type_names_inplace(
        &mut self,
        edge_type_name: String,
        source_node_type_names: Vec<Option<&str>>,
        destination_node_type_names: Vec<Option<&str>>,
    ) -> PyResult<()> {
        Ok({
            pe!(self
                .inner
                .replace_edge_type_name_from_edge_node_type_names_inplace(
                    edge_type_name.into(),
                    &source_node_type_names,
                    &destination_node_type_names
                ))?;
            ()
        })
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, edge_type_name, source_node_type_names, destination_node_type_names)"
    )]
    /// Replaces given edge type name to the nodes with given source and destination node type names.
    ///
    /// Parameters
    /// ----------
    /// edge_type_name: str
    ///     The edge type name to replace with.
    /// source_node_type_names: List[Optional[&str]]
    ///     Node types of the source nodes. When an edge has a source node with any of this node types, we may change its edge type if also the destination nodes have the correct node types.
    /// destination_node_type_names: List[Optional[&str]]
    ///     Node types of the destination nodes. When an edge has a destination node with any of this node types, we may change its edge type if also the source nodes have the correct node types.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given list of node name prefixes is empty.
    ///
    pub fn replace_edge_type_name_from_edge_node_type_names(
        &mut self,
        edge_type_name: String,
        source_node_type_names: Vec<Option<&str>>,
        destination_node_type_names: Vec<Option<&str>>,
    ) -> PyResult<()> {
        Ok({
            pe!(self.inner.replace_edge_type_name_from_edge_node_type_names(
                edge_type_name.into(),
                &source_node_type_names,
                &destination_node_type_names
            ))?;
            ()
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_name, node_name_prefixes)")]
    /// Assigns given node type name to the nodes with given prefixes.
    ///
    /// Parameters
    /// ----------
    /// node_type_name: str
    ///     The node type ID to assign.
    /// node_name_prefixes: List[str]
    ///     The node name prefixes to check for.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given list of node name prefixes is empty.
    ///
    pub fn add_node_type_name_from_node_name_prefixes(
        &self,
        node_type_name: String,
        node_name_prefixes: Vec<String>,
    ) -> PyResult<Graph> {
        Ok(pe!(self.inner.add_node_type_name_from_node_name_prefixes(
            node_type_name.into(),
            node_name_prefixes
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self, edge_type_ids_to_remove)")]
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
                .remove_inplace_edge_type_ids(edge_type_ids_to_remove))?;
            ()
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self, node_type_names)")]
    /// Remove given node type names from all nodes.
    ///
    /// If any given node remains with no node type, that node is labeled
    /// with node type None. Note that the modification happens inplace.
    ///
    /// Parameters
    /// ----------
    /// node_type_names: List[&str]
    ///     The node type names to remove.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    /// ValueError
    ///     If the given node type name does not exists in the graph.
    ///
    pub fn remove_inplace_node_type_names(&mut self, node_type_names: Vec<&str>) -> PyResult<()> {
        Ok({
            pe!(self.inner.remove_inplace_node_type_names(node_type_names))?;
            ()
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_name)")]
    /// Remove given node type name from all nodes.
    ///
    /// If any given node remains with no node type, that node is labeled
    /// with node type None. Note that the modification happens inplace.
    ///
    /// Parameters
    /// ----------
    /// node_type_name: str
    ///     The node type names to remove.
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
            pe!(self.inner.remove_inplace_node_type_name(node_type_name))?;
            ()
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_id)")]
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
        Ok(pe!(self.inner.remove_node_type_id(node_type_id.clone()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
    /// Remove inplace isomorphic node types.
    ///
    /// This will leave for each isomorphic node tyoe group only an element.
    ///
    /// If any given node remains with no node type, that node is labeled
    /// with node type None. Note that the modification DOES NOT happen inplace.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn remove_inplace_isomorphic_node_types(&mut self) -> PyResult<()> {
        Ok({
            pe!(self.inner.remove_inplace_isomorphic_node_types())?;
            ()
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Remove isomorphic node types.
    ///
    /// This will leave for each isomorphic node tyoe group only an element.
    ///
    /// If any given node remains with no node type, that node is labeled
    /// with node type None. Note that the modification DOES NOT happen inplace.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn remove_isomorphic_node_types(&self) -> PyResult<Graph> {
        Ok(pe!(self.inner.remove_isomorphic_node_types())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, minimum_number_of_edges)")]
    /// Remove inplace isomorphic edge types.
    ///
    /// This will leave for each isomorphic edge tyoe group only an element.
    ///
    /// If any given edge remains with no edge type, that edge is labeled
    /// with edge type None. Note that the modification DOES NOT happen inplace.
    ///
    /// Parameters
    /// ----------
    /// minimum_number_of_edges: Optional[int]
    ///     Minimum number of edges to detect edge types topological synonims. By default, 5.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    ///
    pub fn remove_inplace_isomorphic_edge_types(
        &mut self,
        minimum_number_of_edges: Option<EdgeT>,
    ) -> PyResult<()> {
        Ok({
            pe!(self
                .inner
                .remove_inplace_isomorphic_edge_types(minimum_number_of_edges))?;
            ()
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, minimum_number_of_edges)")]
    /// Remove isomorphic edge types.
    ///
    /// This will leave for each isomorphic edge tyoe group only an element.
    ///
    /// If any given edge remains with no edge type, that edge is labeled
    /// with edge type None. Note that the modification DOES NOT happen inplace.
    ///
    /// Parameters
    /// ----------
    /// minimum_number_of_edges: Optional[int]
    ///     Minimum number of edges to detect edge types topological synonims. By default, 5.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    ///
    pub fn remove_isomorphic_edge_types(
        &self,
        minimum_number_of_edges: Option<EdgeT>,
    ) -> PyResult<Graph> {
        Ok(pe!(self
            .inner
            .remove_isomorphic_edge_types(minimum_number_of_edges))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_names)")]
    /// Remove given node type names from all nodes.
    ///
    /// If any given node remains with no node type, that node is labeled
    /// with node type None. Note that the modification DOES NOT happen inplace.
    ///
    /// Parameters
    /// ----------
    /// node_type_names: List[&str]
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
    pub fn remove_node_type_names(&self, node_type_names: Vec<&str>) -> PyResult<Graph> {
        Ok(pe!(self.inner.remove_node_type_names(node_type_names))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_name)")]
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
        Ok(pe!(self.inner.remove_node_type_name(node_type_name))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type_name)")]
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
            pe!(self.inner.remove_inplace_edge_type_name(edge_type_name))?;
            ()
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type_id)")]
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
        Ok(pe!(self.inner.remove_edge_type_id(edge_type_id.clone()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self, edge_type_name)")]
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
        Ok(pe!(self.inner.remove_edge_type_name(edge_type_name))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self, denominator)")]
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
            .divide_edge_weights_inplace(denominator.clone()))?)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, denominator)")]
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
        Ok(pe!(self.inner.divide_edge_weights(denominator.clone()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Normalize edge weights in place.
    ///
    /// Note that the modification happens inplace.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge weights.
    ///
    pub fn normalize_edge_weights_inplace(&mut self) -> PyResult<()> {
        Ok(pe!(self.inner.normalize_edge_weights_inplace())?)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Normalize edge weights.
    ///
    /// Note that the modification does not happen inplace.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge weights.
    ///
    pub fn normalize_edge_weights(&self) -> PyResult<Graph> {
        Ok(pe!(self.inner.normalize_edge_weights())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, denominator)")]
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
            .multiply_edge_weights_inplace(denominator.clone()))?)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, denominator)")]
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
        Ok(pe!(self.inner.multiply_edge_weights(denominator.clone()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, source_node_ids, destination_node_ids, directed)")]
    /// Returns bipartite graph between the provided source and destination node IDs.
    ///
    /// Parameters
    /// ----------
    /// source_node_ids: List[int]
    ///     The source node IDs.
    /// destination_node_ids: List[int]
    ///     The destination node IDs.
    /// directed: bool
    ///     Whether to make the graph directed or undirected.
    ///
    pub fn build_bipartite_graph_from_edge_node_ids(
        &self,
        source_node_ids: Vec<NodeT>,
        destination_node_ids: Vec<NodeT>,
        directed: bool,
    ) -> PyResult<Graph> {
        Ok(pe!(self.inner.build_bipartite_graph_from_edge_node_ids(
            source_node_ids,
            destination_node_ids,
            directed.clone()
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_ids, directed)")]
    /// Returns clique graph between the provided node IDs.
    ///
    /// Parameters
    /// ----------
    /// node_ids: List[int]
    ///     The node IDs.
    /// directed: bool
    ///     Whether to make the graph directed or undirected.
    ///
    pub fn build_clique_graph_from_node_ids(
        &self,
        node_ids: Vec<NodeT>,
        directed: bool,
    ) -> PyResult<Graph> {
        Ok(pe!(self
            .inner
            .build_clique_graph_from_node_ids(node_ids, directed.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, source_node_names, destination_node_names, directed)")]
    /// Returns bipartite graph between the provided source and destination node names.
    ///
    /// Parameters
    /// ----------
    /// source_node_names: List[&str]
    ///     The source node names.
    /// destination_node_names: List[&str]
    ///     The destination node names.
    /// directed: bool
    ///     Whether to make the graph directed or undirected.
    ///
    pub fn build_bipartite_graph_from_edge_node_names(
        &self,
        source_node_names: Vec<&str>,
        destination_node_names: Vec<&str>,
        directed: bool,
    ) -> PyResult<Graph> {
        Ok(pe!(self.inner.build_bipartite_graph_from_edge_node_names(
            source_node_names,
            destination_node_names,
            directed.clone()
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_names, directed)")]
    /// Returns clique graph between the provided node names.
    ///
    /// Parameters
    /// ----------
    /// node_names: List[&str]
    ///     The node names.
    /// directed: bool
    ///     Whether to make the graph directed or undirected.
    ///
    pub fn build_clique_graph_from_node_names(
        &self,
        node_names: Vec<&str>,
        directed: bool,
    ) -> PyResult<Graph> {
        Ok(pe!(self
            .inner
            .build_clique_graph_from_node_names(node_names, directed.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, source_node_prefixes, destination_node_prefixes, directed)")]
    /// Returns bipartite graph between the provided source and destination node prefixes.
    ///
    /// Parameters
    /// ----------
    /// source_node_prefixes: List[str]
    ///     The source node prefixes.
    /// destination_node_prefixes: List[str]
    ///     The destination node prefixes.
    /// directed: bool
    ///     Whether to make the graph directed or undirected.
    ///
    pub fn build_bipartite_graph_from_edge_node_prefixes(
        &self,
        source_node_prefixes: Vec<&str>,
        destination_node_prefixes: Vec<&str>,
        directed: bool,
    ) -> PyResult<Graph> {
        Ok(
            pe!(self.inner.build_bipartite_graph_from_edge_node_prefixes(
                &source_node_prefixes,
                &destination_node_prefixes,
                directed.clone()
            ))?
            .into(),
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_prefixes, directed)")]
    /// Returns clique graph between the nodes with the provided prefixes.
    ///
    /// Parameters
    /// ----------
    /// node_prefixes: List[str]
    ///     The node name prefixes.
    /// directed: bool
    ///     Whether to make the graph directed or undirected.
    ///
    pub fn build_clique_graph_from_node_prefixes(
        &self,
        node_prefixes: Vec<&str>,
        directed: bool,
    ) -> PyResult<Graph> {
        Ok(pe!(self
            .inner
            .build_clique_graph_from_node_prefixes(&node_prefixes, directed.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, source_node_types, destination_node_types, directed)")]
    /// Returns bipartite graph between the provided source and destination node types.
    ///
    /// Parameters
    /// ----------
    /// source_node_types: List[Optional[&str]]
    ///     The source node types.
    /// destination_node_types: List[Optional[&str]]
    ///     The destination node types.
    /// directed: bool
    ///     Whether to make the graph directed or undirected.
    ///
    pub fn build_bipartite_graph_from_edge_node_types(
        &self,
        source_node_types: Vec<Option<&str>>,
        destination_node_types: Vec<Option<&str>>,
        directed: bool,
    ) -> PyResult<Graph> {
        Ok(pe!(self.inner.build_bipartite_graph_from_edge_node_types(
            &source_node_types,
            &destination_node_types,
            directed.clone()
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_names, directed)")]
    /// Returns clique graph between the nodes with the provided node types.
    ///
    /// Parameters
    /// ----------
    /// node_type_names: List[Optional[&str]]
    ///     The node name types.
    /// directed: bool
    ///     Whether to make the graph directed or undirected.
    ///
    pub fn build_clique_graph_from_node_type_names(
        &self,
        node_type_names: Vec<Option<&str>>,
        directed: bool,
    ) -> PyResult<Graph> {
        Ok(pe!(self
            .inner
            .build_clique_graph_from_node_type_names(&node_type_names, directed.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, minimum_number_of_nodes_per_circle, compute_circle_nodes)")]
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
        Ok(pe!(self
            .inner
            .get_circles(minimum_number_of_nodes_per_circle, compute_circle_nodes))?
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns a string describing the memory usage of all the fields of all the
    /// structures used to store the current graph
    pub fn get_memory_stats(&self) -> String {
        self.inner.get_memory_stats().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns how many bytes are currently used to store the given graph
    pub fn get_total_memory_used(&self) -> usize {
        self.inner.get_total_memory_used().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns how many bytes are currently used to store the nodes
    pub fn get_nodes_total_memory_requirement(&self) -> usize {
        self.inner.get_nodes_total_memory_requirement().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns human readable amount of how many bytes are currently used to store the nodes
    pub fn get_nodes_total_memory_requirement_human_readable(&self) -> String {
        self.inner
            .get_nodes_total_memory_requirement_human_readable()
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns how many bytes are currently used to store the edges
    pub fn get_edges_total_memory_requirement(&self) -> usize {
        self.inner.get_edges_total_memory_requirement().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns human readable amount of how many bytes are currently used to store the edges
    pub fn get_edges_total_memory_requirement_human_readable(&self) -> String {
        self.inner
            .get_edges_total_memory_requirement_human_readable()
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns how many bytes are currently used to store the edge weights
    pub fn get_edge_weights_total_memory_requirements(&self) -> usize {
        self.inner
            .get_edge_weights_total_memory_requirements()
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns human readable amount of how many bytes are currently used to store the edge weights
    pub fn get_edge_weights_total_memory_requirements_human_readable(&self) -> String {
        self.inner
            .get_edge_weights_total_memory_requirements_human_readable()
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns how many bytes are currently used to store the node types
    pub fn get_node_types_total_memory_requirements(&self) -> PyResult<usize> {
        Ok(pe!(self.inner.get_node_types_total_memory_requirements())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns human readable amount of how many bytes are currently used to store the node types
    pub fn get_node_types_total_memory_requirements_human_readable(&self) -> PyResult<String> {
        Ok(pe!(self
            .inner
            .get_node_types_total_memory_requirements_human_readable())?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns how many bytes are currently used to store the edge types
    pub fn get_edge_types_total_memory_requirements(&self) -> PyResult<usize> {
        Ok(pe!(self.inner.get_edge_types_total_memory_requirements())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns human readable amount of how many bytes are currently used to store the edge types
    pub fn get_edge_types_total_memory_requirements_human_readable(&self) -> PyResult<String> {
        Ok(pe!(self
            .inner
            .get_edge_types_total_memory_requirements_human_readable())?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, normalize, low_centrality, verbose)")]
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
            .get_number_of_triangles(normalize, low_centrality, verbose)
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns total number of triads in the graph without taking into account weights
    pub fn get_number_of_triads(&self) -> EdgeT {
        self.inner.get_number_of_triads().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns total number of triads in the weighted graph
    pub fn get_number_of_weighted_triads(&self) -> PyResult<f64> {
        Ok(pe!(self.inner.get_number_of_weighted_triads())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, low_centrality, verbose)")]
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
        self.inner.get_transitivity(low_centrality, verbose).into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, normalize, low_centrality, verbose)")]
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
            self.inner
                .get_number_of_triangles_per_node(normalize, low_centrality, verbose),
            NodeT
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, low_centrality, verbose)")]
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
                .get_clustering_coefficient_per_node(low_centrality, verbose),
            f64
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, low_centrality, verbose)")]
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
            .get_clustering_coefficient(low_centrality, verbose)
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, low_centrality, verbose)")]
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
            .get_average_clustering_coefficient(low_centrality, verbose)
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, other)")]
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
    #[pyo3(text_signature = "($self, node_ids)")]
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
        self.inner.remap_unchecked_from_node_ids(node_ids).into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_ids)")]
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
        Ok(pe!(self.inner.remap_from_node_ids(node_ids))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_names)")]
    /// Returns graph remapped using given node names ordering.
    ///
    /// Parameters
    /// ----------
    /// node_names: List[&str]
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
        Ok(pe!(self.inner.remap_from_node_names(node_names))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_names_map)")]
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
    #[pyo3(text_signature = "($self, other)")]
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
    #[pyo3(
        text_signature = "($self, number_of_negative_samples, random_state, only_from_same_component, sample_only_edges_with_heterogeneous_node_types, minimum_node_degree, maximum_node_degree, source_node_types_names, destination_node_types_names, source_edge_types_names, destination_edge_types_names, source_nodes_prefixes, destination_nodes_prefixes, graph_to_avoid, support, use_scale_free_distribution, sample_edge_types)"
    )]
    /// Returns Graph with given amount of negative edges as positive edges.
    ///
    /// The graph generated may be used as a testing negatives partition to be
    /// fed into the argument "graph_to_avoid" of the link_prediction or the
    /// skipgrams algorithm.
    ///
    /// Parameters
    /// ----------
    /// number_of_negative_samples: int
    ///     Number of negatives edges to include.
    /// random_state: Optional[int]
    ///     random_state to use to reproduce negative edge set.
    /// only_from_same_component: Optional[bool]
    ///     Whether to sample negative edges only from nodes that are from the same component.
    /// sample_only_edges_with_heterogeneous_node_types: Optional[bool]
    ///     Whether to sample negative edges only with source and destination nodes that have different node types.
    /// minimum_node_degree: Optional[int]
    ///     The minimum node degree of either the source or destination node to be sampled. By default 0.
    /// maximum_node_degree: Optional[int]
    ///     The maximum node degree of either the source or destination node to be sampled. By default, the number of nodes.
    /// destination_node_types_names: Optional[List[str]]
    ///     Node type names of the nodes to be samples as destinations. If a node has any of the provided node types, it can be sampled as a destination node.
    /// source_edge_types_names: Optional[List[str]]
    ///     Edge type names of the nodes to be samples as sources. If a node has any of the provided edge types, it can be sampled as a source node.
    /// destination_edge_types_names: Optional[List[str]]
    ///     Edge type names of the nodes to be samples as destinations. If a node has any of the provided edge types, it can be sampled as a destination node.
    /// source_nodes_prefixes: Optional[List[str]]
    ///     Prefixes of the nodes names to be samples as sources. If a node starts with any of the provided prefixes, it can be sampled as a source node.
    /// destination_nodes_prefixes: Optional[List[str]]
    ///     Prefixes of the nodes names to be samples as destinations. If a node starts with any of the provided prefixes, it can be sampled as a destinations node.
    /// graph_to_avoid: Optional[&Graph]
    ///     Compatible graph whose edges are not to be sampled.
    /// support: Optional[&Graph]
    ///     Parent graph of this subgraph, defining the `true` topology of the graph. Node degrees and connected components are sampled from this support graph when provided. Useful when sampling negative edges for a test graph. In this latter case, the support graph should be the training graph.
    /// use_scale_free_distribution: Optional[bool]
    ///     Whether to sample the nodes using scale_free distribution. By default True. Not using this may cause significant biases.
    /// sample_edge_types: Optional[bool]
    ///     Whether to sample edge types, following the edge type counts distribution. By default it is true only when the current graph instance has edge types.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the `sample_only_edges_with_heterogeneous_node_types` argument is provided as true, but the graph does not have node types.
    ///
    pub fn sample_negative_graph(
        &self,
        number_of_negative_samples: EdgeT,
        random_state: Option<EdgeT>,
        only_from_same_component: Option<bool>,
        sample_only_edges_with_heterogeneous_node_types: Option<bool>,
        minimum_node_degree: Option<NodeT>,
        maximum_node_degree: Option<NodeT>,
        source_node_types_names: Option<Vec<String>>,
        destination_node_types_names: Option<Vec<String>>,
        source_edge_types_names: Option<Vec<String>>,
        destination_edge_types_names: Option<Vec<String>>,
        source_nodes_prefixes: Option<Vec<String>>,
        destination_nodes_prefixes: Option<Vec<String>>,
        graph_to_avoid: Option<&Graph>,
        support: Option<&Graph>,
        use_scale_free_distribution: Option<bool>,
        sample_edge_types: Option<bool>,
    ) -> PyResult<Graph> {
        Ok(pe!(self.inner.sample_negative_graph(
            number_of_negative_samples.clone(),
            random_state,
            only_from_same_component,
            sample_only_edges_with_heterogeneous_node_types,
            minimum_node_degree,
            maximum_node_degree,
            source_node_types_names,
            destination_node_types_names,
            source_edge_types_names,
            destination_edge_types_names,
            source_nodes_prefixes,
            destination_nodes_prefixes,
            graph_to_avoid.map(|sg| &sg.inner),
            support.map(|sg| &sg.inner),
            use_scale_free_distribution,
            sample_edge_types
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, number_of_samples, random_state, sample_only_edges_with_heterogeneous_node_types, minimum_node_degree, maximum_node_degree, source_node_types_names, destination_node_types_names, source_edge_types_names, destination_edge_types_names, source_nodes_prefixes, destination_nodes_prefixes, edge_type_names, support)"
    )]
    /// Returns Graph with given amount of subsampled edges.
    ///
    /// Parameters
    /// ----------
    /// number_of_samples: int
    ///     Number of edges to include.
    /// random_state: Optional[int]
    ///     random_state to use to reproduce negative edge set.
    /// sample_only_edges_with_heterogeneous_node_types: Optional[bool]
    ///     Whether to sample negative edges only with source and destination nodes that have different node types.
    /// minimum_node_degree: Optional[int]
    ///     The minimum node degree of either the source or destination node to be sampled. By default 0.
    /// maximum_node_degree: Optional[int]
    ///     The maximum node degree of either the source or destination node to be sampled. By default, the number of nodes.
    /// destination_node_types_names: Optional[List[str]]
    ///     Node type names of the nodes to be samples as destinations. If a node has any of the provided node types, it can be sampled as a destination node.
    /// source_edge_types_names: Optional[List[str]]
    ///     Edge type names of the nodes to be samples as sources. If a node has any of the provided edge types, it can be sampled as a source node.
    /// destination_edge_types_names: Optional[List[str]]
    ///     Edge type names of the nodes to be samples as destinations. If a node has any of the provided edge types, it can be sampled as a destination node.
    /// source_nodes_prefixes: Optional[List[str]]
    ///     Prefixes of the nodes names to be samples as sources. If a node starts with any of the provided prefixes, it can be sampled as a source node.
    /// destination_nodes_prefixes: Optional[List[str]]
    ///     Prefixes of the nodes names to be samples as destinations. If a node starts with any of the provided prefixes, it can be sampled as a destinations node.
    /// edge_type_names: Optional[&List[Optional[&str]]]
    ///     Edge type names of the edges to sample. Only edges with ANY of these edge types will be kept.
    /// support: Optional[&Graph]
    ///     Parent graph of this subgraph, defining the `true` topology of the graph. Node degrees are sampled from this support graph when provided. Useful when sampling positive edges for a test graph. In this latter case, the support graph should be the training graph.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the `sample_only_edges_with_heterogeneous_node_types` argument is provided as true, but the graph does not have node types.
    ///
    pub fn sample_positive_graph(
        &self,
        number_of_samples: usize,
        random_state: Option<EdgeT>,
        sample_only_edges_with_heterogeneous_node_types: Option<bool>,
        minimum_node_degree: Option<NodeT>,
        maximum_node_degree: Option<NodeT>,
        source_node_types_names: Option<Vec<String>>,
        destination_node_types_names: Option<Vec<String>>,
        source_edge_types_names: Option<Vec<String>>,
        destination_edge_types_names: Option<Vec<String>>,
        source_nodes_prefixes: Option<Vec<String>>,
        destination_nodes_prefixes: Option<Vec<String>>,
        edge_type_names: Option<Vec<Option<&str>>>,
        support: Option<&Graph>,
    ) -> PyResult<Graph> {
        Ok(pe!(self.inner.sample_positive_graph(
            number_of_samples.clone(),
            random_state,
            sample_only_edges_with_heterogeneous_node_types,
            minimum_node_degree,
            maximum_node_degree,
            source_node_types_names,
            destination_node_types_names,
            source_edge_types_names,
            destination_edge_types_names,
            source_nodes_prefixes,
            destination_nodes_prefixes,
            edge_type_names.as_ref().map(|x| x.as_slice()),
            support.map(|sg| &sg.inner)
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, train_size, random_state, edge_types, include_all_edge_types, minimum_node_degree, maximum_node_degree, verbose)"
    )]
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
    /// edge_types: Optional[&List[Optional[&str]]]
    ///     Edge types to be selected for in the validation set.
    /// include_all_edge_types: Optional[bool]
    ///     Whether to include all the edges between two nodes.
    /// minimum_node_degree: Optional[int]
    ///     The minimum node degree of either the source or destination node to be sampled. By default 0.
    /// maximum_node_degree: Optional[int]
    ///     The maximum node degree of either the source or destination node to be sampled. By default, the number of nodes.
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
        edge_types: Option<Vec<Option<&str>>>,
        include_all_edge_types: Option<bool>,
        minimum_node_degree: Option<NodeT>,
        maximum_node_degree: Option<NodeT>,
        verbose: Option<bool>,
    ) -> PyResult<(Graph, Graph)> {
        Ok({
            let (subresult_0, subresult_1) = pe!(self.inner.connected_holdout(
                train_size.clone(),
                random_state,
                edge_types.as_ref().map(|x| x.as_slice()),
                include_all_edge_types,
                minimum_node_degree,
                maximum_node_degree,
                verbose
            ))?
            .into();
            (subresult_0.into(), subresult_1.into())
        })
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, train_size, random_state, include_all_edge_types, edge_types, min_number_overlaps, verbose)"
    )]
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
    /// edge_types: Optional[&List[Optional[&str]]]
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
        edge_types: Option<Vec<Option<&str>>>,
        min_number_overlaps: Option<EdgeT>,
        verbose: Option<bool>,
    ) -> PyResult<(Graph, Graph)> {
        Ok({
            let (subresult_0, subresult_1) = pe!(self.inner.random_holdout(
                train_size.clone(),
                random_state,
                include_all_edge_types,
                edge_types.as_ref().map(|x| x.as_slice()),
                min_number_overlaps,
                verbose
            ))?
            .into();
            (subresult_0.into(), subresult_1.into())
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, train_size, use_stratification, random_state)")]
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
                train_size.clone(),
                use_stratification,
                random_state
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
    #[pyo3(text_signature = "($self, train_size, use_stratification, random_state)")]
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
    ) -> PyResult<(
        Vec<Option<Py<PyArray1<NodeTypeT>>>>,
        Vec<Option<Py<PyArray1<NodeTypeT>>>>,
    )> {
        Ok({
            let (subresult_0, subresult_1) = pe!(self.inner.get_node_label_holdout_labels(
                train_size.clone(),
                use_stratification,
                random_state
            ))?
            .into();
            (
                subresult_0
                    .into_iter()
                    .map(|x| {
                        x.map(|x| {
                            let gil = pyo3::Python::acquire_gil();
                            to_ndarray_1d!(gil, x, NodeTypeT)
                        })
                    })
                    .collect::<Vec<_>>(),
                subresult_1
                    .into_iter()
                    .map(|x| {
                        x.map(|x| {
                            let gil = pyo3::Python::acquire_gil();
                            to_ndarray_1d!(gil, x, NodeTypeT)
                        })
                    })
                    .collect::<Vec<_>>(),
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, train_size, use_stratification, random_state)")]
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
                train_size.clone(),
                use_stratification,
                random_state
            ))?
            .into();
            (subresult_0.into(), subresult_1.into())
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, train_size, use_stratification, random_state)")]
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
                train_size.clone(),
                use_stratification,
                random_state
            ))?
            .into();
            (subresult_0.into(), subresult_1.into())
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, nodes_number, random_state, verbose)")]
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
        Ok(pe!(self
            .inner
            .get_random_subgraph(nodes_number.clone(), random_state, verbose))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, train_size, use_stratification, random_state)")]
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
                train_size.clone(),
                use_stratification,
                random_state
            ))?
            .into();
            (subresult_0.into(), subresult_1.into())
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, k, k_index, use_stratification, random_state)")]
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
                k.clone(),
                k_index.clone(),
                use_stratification,
                random_state
            ))?
            .into();
            (subresult_0.into(), subresult_1.into())
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, train_size, use_stratification, random_state)")]
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
                train_size.clone(),
                use_stratification,
                random_state
            ))?
            .into();
            (subresult_0.into(), subresult_1.into())
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, k, k_index, use_stratification, random_state)")]
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
                k.clone(),
                k_index.clone(),
                use_stratification,
                random_state
            ))?
            .into();
            (subresult_0.into(), subresult_1.into())
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, k, k_index, edge_types, random_state, verbose)")]
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
    /// edge_types: Optional[&List[Optional[&str]]]
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
        edge_types: Option<Vec<Option<&str>>>,
        random_state: Option<EdgeT>,
        verbose: Option<bool>,
    ) -> PyResult<(Graph, Graph)> {
        Ok({
            let (subresult_0, subresult_1) = pe!(self.inner.get_edge_prediction_kfold(
                k.clone(),
                k_index.clone(),
                edge_types.as_ref().map(|x| x.as_slice()),
                random_state,
                verbose
            ))?
            .into();
            (subresult_0.into(), subresult_1.into())
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return vector of node tuples in the current graph instance
    pub fn get_node_tuples(&self) -> PyResult<Vec<NodeTuple>> {
        Ok(pe!(self.inner.get_node_tuples())?
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, minimum_number_of_nodes_per_chain, compute_chain_nodes)")]
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
        Ok(pe!(self
            .inner
            .get_chains(minimum_number_of_nodes_per_chain, compute_chain_nodes))?
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src_node_id)")]
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
                src_node_id.clone(),
            )
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src_node_ids, maximal_depth)")]
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
                src_node_ids,
                maximal_depth,
            )
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src_node_id, maximal_depth)")]
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
                src_node_id.clone(),
                maximal_depth,
            )
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src_node_id)")]
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
                src_node_id.clone(),
            )
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, src_node_ids, dst_node_id, compute_predecessors, maximal_depth)"
    )]
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
                src_node_ids,
                dst_node_id,
                compute_predecessors,
                maximal_depth,
            )
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, src_node_id, dst_node_id, compute_predecessors, maximal_depth)"
    )]
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
                src_node_id.clone(),
                dst_node_id,
                compute_predecessors,
                maximal_depth,
            )
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src_node_id, dst_node_id, maximal_depth)")]
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
                        src_node_id.clone(),
                        dst_node_id.clone(),
                        maximal_depth
                    ))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src_node_id, dst_node_id, maximal_depth)")]
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
                src_node_id.clone(),
                dst_node_id.clone(),
                maximal_depth
            ))?
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src_node_id, dst_node_id, maximal_depth)")]
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
                    src_node_id.clone(),
                    dst_node_id.clone(),
                    maximal_depth
                ))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src_node_name, dst_node_name, maximal_depth)")]
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
                    src_node_name,
                    dst_node_name,
                    maximal_depth
                ))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src_node_name, dst_node_name, maximal_depth)")]
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
            src_node_name,
            dst_node_name,
            maximal_depth
        ))?
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src_node_id, dst_node_id, k)")]
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
                src_node_id.clone(),
                dst_node_id.clone(),
                k.clone(),
            )
            .into_iter()
            .map(|x| x.into_iter().map(|x| x.into()).collect::<Vec<_>>())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src_node_id, dst_node_id, k)")]
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
            src_node_id.clone(),
            dst_node_id.clone(),
            k.clone()
        ))?
        .into_iter()
        .map(|x| x.into_iter().map(|x| x.into()).collect::<Vec<_>>())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src_node_name, dst_node_name, k)")]
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
            src_node_name,
            dst_node_name,
            k.clone()
        ))?
        .into_iter()
        .map(|x| x.into_iter().map(|x| x.into()).collect::<Vec<_>>())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src_node_name, dst_node_name, k)")]
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
                src_node_name,
                dst_node_name,
                k.clone()
            ))?
            .into_iter()
            .map(|x| x.into_iter().map(|x| x.into()).collect::<Vec<_>>())
            .collect::<Vec<_>>(),
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
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
            .get_unchecked_eccentricity_and_most_distant_node_id_from_node_id(node_id.clone());
        (subresult_0.into(), subresult_1.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id, use_edge_weights_as_probabilities)")]
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
    ) -> f32 {
        self.inner
            .get_unchecked_weighted_eccentricity_from_node_id(
                node_id.clone(),
                use_edge_weights_as_probabilities,
            )
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
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
                .get_eccentricity_and_most_distant_node_id_from_node_id(node_id.clone()))?
            .into();
            (subresult_0.into(), subresult_1.into())
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id, use_edge_weights_as_probabilities)")]
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
    ) -> PyResult<f32> {
        Ok(pe!(self.inner.get_weighted_eccentricity_from_node_id(
            node_id.clone(),
            use_edge_weights_as_probabilities
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_name)")]
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
        Ok(pe!(self.inner.get_eccentricity_from_node_name(node_name))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_name, use_edge_weights_as_probabilities)")]
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
    ) -> PyResult<f32> {
        Ok(pe!(self.inner.get_weighted_eccentricity_from_node_name(
            node_name,
            use_edge_weights_as_probabilities
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, src_node_ids, maybe_dst_node_id, maybe_dst_node_ids, compute_predecessors, maximal_depth, use_edge_weights_as_probabilities)"
    )]
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
                src_node_ids,
                maybe_dst_node_id,
                maybe_dst_node_ids,
                compute_predecessors,
                maximal_depth,
                use_edge_weights_as_probabilities,
            )
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, src_node_id, maybe_dst_node_id, maybe_dst_node_ids, compute_predecessors, maximal_depth, use_edge_weights_as_probabilities)"
    )]
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
                src_node_id.clone(),
                maybe_dst_node_id,
                maybe_dst_node_ids,
                compute_predecessors,
                maximal_depth,
                use_edge_weights_as_probabilities,
            )
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, src_node_id, dst_node_id, use_edge_weights_as_probabilities, maximal_depth)"
    )]
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
    ) -> (f32, Py<PyArray1<NodeT>>) {
        let (subresult_0, subresult_1) = self
            .inner
            .get_unchecked_weighted_shortest_path_node_ids_from_node_ids(
                src_node_id.clone(),
                dst_node_id.clone(),
                use_edge_weights_as_probabilities,
                maximal_depth,
            );
        (subresult_0.into(), {
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(gil, subresult_1, NodeT)
        })
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, src_node_id, dst_node_id, use_edge_weights_as_probabilities, maximal_depth)"
    )]
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
    ) -> (f32, Vec<String>) {
        let (subresult_0, subresult_1) = self
            .inner
            .get_unchecked_weighted_shortest_path_node_names_from_node_ids(
                src_node_id.clone(),
                dst_node_id.clone(),
                use_edge_weights_as_probabilities,
                maximal_depth,
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
    #[pyo3(
        text_signature = "($self, src_node_id, dst_node_id, use_edge_weights_as_probabilities, maximal_depth)"
    )]
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
    ) -> PyResult<(f32, Py<PyArray1<NodeT>>)> {
        Ok({
            let (subresult_0, subresult_1) = pe!(self
                .inner
                .get_weighted_shortest_path_node_ids_from_node_ids(
                    src_node_id.clone(),
                    dst_node_id.clone(),
                    use_edge_weights_as_probabilities,
                    maximal_depth
                ))?
            .into();
            (subresult_0.into(), {
                let gil = pyo3::Python::acquire_gil();
                to_ndarray_1d!(gil, subresult_1, NodeT)
            })
        })
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, src_node_name, dst_node_name, use_edge_weights_as_probabilities, maximal_depth)"
    )]
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
    ) -> PyResult<(f32, Py<PyArray1<NodeT>>)> {
        Ok({
            let (subresult_0, subresult_1) = pe!(self
                .inner
                .get_weighted_shortest_path_node_ids_from_node_names(
                    src_node_name,
                    dst_node_name,
                    use_edge_weights_as_probabilities,
                    maximal_depth
                ))?
            .into();
            (subresult_0.into(), {
                let gil = pyo3::Python::acquire_gil();
                to_ndarray_1d!(gil, subresult_1, NodeT)
            })
        })
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, src_node_name, dst_node_name, use_edge_weights_as_probabilities, maximal_depth)"
    )]
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
    ) -> PyResult<(f32, Vec<String>)> {
        Ok({
            let (subresult_0, subresult_1) = pe!(self
                .inner
                .get_weighted_shortest_path_node_names_from_node_names(
                    src_node_name,
                    dst_node_name,
                    use_edge_weights_as_probabilities,
                    maximal_depth
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
    #[pyo3(
        text_signature = "($self, src_node_id, dst_node_id, compute_predecessors, maximal_depth)"
    )]
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
            src_node_id.clone(),
            dst_node_id,
            compute_predecessors,
            maximal_depth
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, src_node_id, maybe_dst_node_id, maybe_dst_node_ids, compute_predecessors, maximal_depth, use_edge_weights_as_probabilities)"
    )]
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
            src_node_id.clone(),
            maybe_dst_node_id,
            maybe_dst_node_ids,
            compute_predecessors,
            maximal_depth,
            use_edge_weights_as_probabilities
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns approximated diameter and tentative low eccentricity node for an UNDIRECTED graph.
    /// This method returns a lowerbound of the diameter by doing the following steps:
    /// * Find the most central node
    /// * Find the most distant node from the most central one (and get a first
    /// approximation of the diameter lowerbound)
    /// * Get the median node in this path
    /// * Find the most distant node from the median node
    /// * Find the most distant node form the last one, and get the second approx
    /// of the diameter lowerbound.
    ///
    /// This basically creates a "cross" that spans the graph
    pub fn get_four_sweep(&self) -> (NodeT, NodeT) {
        let (subresult_0, subresult_1) = self.inner.get_four_sweep();
        (subresult_0.into(), subresult_1.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, ignore_infinity, verbose)")]
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
    ) -> PyResult<f32> {
        Ok(pe!(self.inner.get_diameter_naive(ignore_infinity, verbose))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, ignore_infinity, verbose)")]
    /// Returns diameter of the graph.
    ///
    /// Parameters
    /// ----------
    /// ignore_infinity: Optional[bool]
    ///     Whether to ignore infinite distances, which are present when in the graph exist multiple components. By default True.
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
    ) -> PyResult<f32> {
        Ok(pe!(self.inner.get_diameter(ignore_infinity, verbose))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, src_node_name, dst_node_name, compute_predecessors, maximal_depth)"
    )]
    /// Returns vector of minimum paths distances and vector of nodes predecessors from given source node name and optional destination node name.
    ///
    /// Parameters
    /// ----------
    /// src_node_name: str
    ///     Node name root of the tree of minimum paths.
    /// dst_node_name: Optional[&str]
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
            src_node_name,
            dst_node_name,
            compute_predecessors,
            maximal_depth
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, src_node_name, maybe_dst_node_name, maybe_dst_node_names, compute_predecessors, maximal_depth, use_edge_weights_as_probabilities)"
    )]
    /// Returns vector of minimum paths distances and vector of nodes predecessors from given source node name and optional destination node name.
    ///
    /// Parameters
    /// ----------
    /// src_node_name: str
    ///     Node name root of the tree of minimum paths.
    /// maybe_dst_node_name: Optional[&str]
    ///     Optional target destination node name. If provided, Dijkstra will stop upon reaching this node.
    /// maybe_dst_node_names: Optional[List[&str]]
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
            src_node_name,
            maybe_dst_node_name,
            maybe_dst_node_names,
            compute_predecessors,
            maximal_depth,
            use_edge_weights_as_probabilities
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, verbose)")]
    /// Returns number a triple with (number of components, number of nodes of the smallest component, number of nodes of the biggest component )
    ///
    /// Parameters
    /// ----------
    /// verbose: Optional[bool]
    ///     Whether to show a loading bar or not.
    ///
    pub fn get_number_of_connected_components(
        &self,
        verbose: Option<bool>,
    ) -> (NodeT, NodeT, NodeT) {
        let (subresult_0, subresult_1, subresult_2) =
            self.inner.get_number_of_connected_components(verbose);
        (subresult_0.into(), subresult_1.into(), subresult_2.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns number of connected nodes in the graph.
    pub fn get_number_of_connected_nodes(&self) -> NodeT {
        self.inner.get_number_of_connected_nodes().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns number of singleton nodes with selfloops within the graph.
    pub fn get_number_of_singleton_nodes_with_selfloops(&self) -> NodeT {
        self.inner
            .get_number_of_singleton_nodes_with_selfloops()
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns number of singleton nodes within the graph.
    pub fn get_number_of_singleton_nodes(&self) -> NodeT {
        self.inner.get_number_of_singleton_nodes().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns number of disconnected nodes within the graph.
    /// A Disconnected node is a node which is nor a singleton nor a singleton
    /// with selfloops.
    pub fn get_number_of_disconnected_nodes(&self) -> NodeT {
        self.inner.get_number_of_disconnected_nodes().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns vector of singleton node IDs of the graph.
    pub fn get_singleton_node_ids(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_singleton_node_ids(), NodeT)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns vector of singleton node names of the graph.
    pub fn get_singleton_node_names(&self) -> Vec<String> {
        self.inner
            .get_singleton_node_names()
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
    /// Returns vector of singleton_with_selfloops node names of the graph.
    pub fn get_singleton_with_selfloops_node_names(&self) -> Vec<String> {
        self.inner
            .get_singleton_with_selfloops_node_names()
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns density of the graph.
    pub fn get_density(&self) -> PyResult<f64> {
        Ok(pe!(self.inner.get_density())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns the traps rate of the graph.
    ///
    /// THIS IS EXPERIMENTAL AND MUST BE PROVEN!
    pub fn get_trap_nodes_rate(&self) -> f64 {
        self.inner.get_trap_nodes_rate().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns vector of trap nodes present in the current graph
    pub fn get_trap_node_ids(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_trap_node_ids(), NodeT)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns unweighted mean node degree of the graph.
    pub fn get_node_degrees_mean(&self) -> PyResult<f64> {
        Ok(pe!(self.inner.get_node_degrees_mean())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns weighted mean node degree of the graph.
    pub fn get_weighted_node_degrees_mean(&self) -> PyResult<f64> {
        Ok(pe!(self.inner.get_weighted_node_degrees_mean())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns number of undirected edges of the graph.
    pub fn get_number_of_undirected_edges(&self) -> EdgeT {
        self.inner.get_number_of_undirected_edges().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns number of undirected edges of the graph.
    pub fn get_number_of_unique_undirected_edges(&self) -> EdgeT {
        self.inner.get_number_of_unique_undirected_edges().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns number of edges of the graph.
    pub fn get_number_of_edges(&self) -> EdgeT {
        self.inner.get_number_of_edges().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns number of unique edges of the graph.
    pub fn get_number_of_unique_edges(&self) -> EdgeT {
        self.inner.get_number_of_unique_edges().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns unweighted median node degree of the graph
    pub fn get_node_degrees_median(&self) -> PyResult<NodeT> {
        Ok(pe!(self.inner.get_node_degrees_median())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns weighted median node degree of the graph
    pub fn get_weighted_node_degrees_median(&self) -> PyResult<f64> {
        Ok(pe!(self.inner.get_weighted_node_degrees_median())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
    /// Returns maximum node degree of the graph.
    ///
    /// Safety
    /// ------
    /// This method fails with a panic if the graph does not have any node.
    pub unsafe fn get_unchecked_most_central_node_id(&self) -> NodeT {
        self.inner.get_unchecked_most_central_node_id().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns maximum node degree of the graph.
    pub fn get_most_central_node_id(&self) -> PyResult<NodeT> {
        Ok(pe!(self.inner.get_most_central_node_id())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
    /// Returns mode node degree of the graph.
    pub fn get_node_degrees_mode(&self) -> PyResult<NodeT> {
        Ok(pe!(self.inner.get_node_degrees_mode())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns rate of self-loops.
    pub fn get_selfloop_nodes_rate(&self) -> PyResult<f64> {
        Ok(pe!(self.inner.get_selfloop_nodes_rate())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return name of the graph.
    pub fn get_name(&self) -> String {
        self.inner.get_name().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the number of traps (nodes without any outgoing edges that are not singletons)
    /// This also includes nodes with only a self-loops, therefore singletons with
    /// only a self-loops are not considered traps because you could make a walk on them.
    pub fn get_number_of_trap_nodes(&self) -> NodeT {
        self.inner.get_number_of_trap_nodes().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, directed)")]
    /// Return vector of the non-unique source nodes.
    ///
    /// Parameters
    /// ----------
    /// directed: bool
    ///     Whether to filter out the undirected edges.
    ///
    pub fn get_source_node_ids(&self, directed: bool) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_source_node_ids(directed.clone()), NodeT)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return vector on the (non unique) directed source nodes of the graph
    pub fn get_directed_source_node_ids(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_directed_source_node_ids(), NodeT)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, directed)")]
    /// Return vector of the non-unique source nodes names.
    ///
    /// Parameters
    /// ----------
    /// directed: bool
    ///     Whether to filter out the undirected edges.
    ///
    pub fn get_source_names(&self, directed: bool) -> Vec<String> {
        self.inner
            .get_source_names(directed.clone())
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, directed)")]
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
            self.inner.get_destination_node_ids(directed.clone()),
            NodeT
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return vector on the (non unique) directed destination nodes of the graph
    pub fn get_directed_destination_node_ids(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_directed_destination_node_ids(), NodeT)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, directed)")]
    /// Return vector of the non-unique destination nodes names.
    ///
    /// Parameters
    /// ----------
    /// directed: bool
    ///     Whether to filter out the undirected edges.
    ///
    pub fn get_destination_names(&self, directed: bool) -> Vec<String> {
        self.inner
            .get_destination_names(directed.clone())
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return vector with the sorted nodes names
    pub fn get_node_names(&self) -> Vec<String> {
        self.inner
            .get_node_names()
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return vector with the node URLs.
    pub fn get_node_urls(&self) -> Vec<Option<String>> {
        self.inner
            .get_node_urls()
            .into_iter()
            .map(|x| x.map(|x| x.into()))
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return vector with the node predicted ontology.
    pub fn get_node_ontologies(&self) -> Vec<Option<String>> {
        self.inner
            .get_node_ontologies()
            .into_iter()
            .map(|x| x.map(|x| x.into()))
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_name)")]
    /// Return node ontology for the provided node name, if available.
    ///
    /// Parameters
    /// ----------
    /// node_name: str
    ///     The node name to query for.
    ///
    pub unsafe fn get_unchecked_ontology_from_node_name(&self, node_name: &str) -> Option<String> {
        self.inner
            .get_unchecked_ontology_from_node_name(node_name)
            .map(|x| x.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    /// Return node ontology for the provided node id, if available.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     The node id to query for.
    ///
    pub unsafe fn get_unchecked_ontology_from_node_id(&self, node_id: NodeT) -> Option<String> {
        self.inner
            .get_unchecked_ontology_from_node_id(node_id.clone())
            .map(|x| x.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_name)")]
    /// Return node ontology for the provided node name, if available.
    ///
    /// Parameters
    /// ----------
    /// node_name: str
    ///     The node name to query for.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the provided node name does not exist in the current graph.
    ///
    pub fn get_ontology_from_node_name(&self, node_name: &str) -> PyResult<Option<String>> {
        Ok(pe!(self.inner.get_ontology_from_node_name(node_name))?.map(|x| x.into()))
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    /// Return node ontology for the provided node id, if available.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     The node id to query for.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the provided node ID does not exist in the current graph.
    ///
    pub fn get_ontology_from_node_id(&self, node_id: NodeT) -> PyResult<Option<String>> {
        Ok(pe!(self.inner.get_ontology_from_node_id(node_id.clone()))?.map(|x| x.into()))
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return vector with the sorted nodes Ids
    pub fn get_node_ids(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_node_ids(), NodeT)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the directed edge types of the edges
    pub fn get_directed_edge_type_ids(&self) -> PyResult<Vec<Option<EdgeTypeT>>> {
        Ok(pe!(self.inner.get_directed_edge_type_ids())?
            .into_iter()
            .map(|x| x.map(|x| x.into()))
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the undirected edge types of the edges
    pub fn get_undirected_edge_type_ids(&self) -> PyResult<Vec<Option<EdgeTypeT>>> {
        Ok(pe!(self.inner.get_undirected_edge_type_ids())?
            .into_iter()
            .map(|x| x.map(|x| x.into()))
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the known edge types of the edges, dropping unknown ones
    pub fn get_known_edge_type_ids(&self) -> PyResult<Py<PyArray1<EdgeTypeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(gil, pe!(self.inner.get_known_edge_type_ids())?, EdgeTypeT)
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the directed source node IDs with known edge types
    pub fn get_directed_source_nodes_with_known_edge_types(&self) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_directed_source_nodes_with_known_edge_types())?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the directed destination node IDs with known edge types
    pub fn get_directed_destination_nodes_with_known_edge_types(
        &self,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_directed_destination_nodes_with_known_edge_types())?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the unique edge type IDs of the graph edges.
    pub fn get_unique_edge_type_ids(&self) -> PyResult<Py<PyArray1<EdgeTypeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(gil, pe!(self.inner.get_unique_edge_type_ids())?, EdgeTypeT)
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the edge types names
    pub fn get_edge_type_names(&self) -> PyResult<Vec<Option<String>>> {
        Ok(pe!(self.inner.get_edge_type_names())?
            .into_iter()
            .map(|x| x.map(|x| x.into()))
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the edge types names
    pub fn get_unique_edge_type_names(&self) -> PyResult<Vec<String>> {
        Ok(pe!(self.inner.get_unique_edge_type_names())?
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the directed weights of the graph edges.
    pub fn get_directed_edge_weights(&self) -> PyResult<Py<PyArray1<WeightT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(gil, pe!(self.inner.get_directed_edge_weights())?, WeightT)
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the undirected weights of the graph edges, filtering out edges where src > dst.
    pub fn get_undirected_edge_weights(&self) -> PyResult<Py<PyArray1<WeightT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(gil, pe!(self.inner.get_undirected_edge_weights())?, WeightT)
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the weighted indegree (total weighted inbound edge weights) for each node.
    pub fn get_weighted_node_indegrees(&self) -> PyResult<Py<PyArray1<f64>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(gil, pe!(self.inner.get_weighted_node_indegrees())?, f64)
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the node types of the graph nodes.
    pub fn get_node_type_ids(&self) -> PyResult<Vec<Option<Py<PyArray1<NodeTypeT>>>>> {
        Ok(pe!(self.inner.get_node_type_ids())?
            .into_iter()
            .map(|x| {
                x.map(|x| {
                    let gil = pyo3::Python::acquire_gil();
                    to_ndarray_1d!(gil, x, NodeTypeT)
                })
            })
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
    /// Returns boolean mask of known edge types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    ///
    pub fn get_known_edge_types_mask(&self) -> PyResult<Py<PyArray1<bool>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(gil, pe!(self.inner.get_known_edge_types_mask())?, bool)
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns boolean mask of unknown edge types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    ///
    pub fn get_unknown_edge_types_mask(&self) -> PyResult<Py<PyArray1<bool>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(gil, pe!(self.inner.get_unknown_edge_types_mask())?, bool)
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
    /// Return the node types names.
    pub fn get_node_type_names(&self) -> PyResult<Vec<Option<Vec<String>>>> {
        Ok(pe!(self.inner.get_node_type_names())?
            .into_iter()
            .map(|x| x.map(|x| x.into_iter().map(|x| x.into()).collect::<Vec<_>>()))
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the unique node type IDs of the graph nodes.
    pub fn get_unique_node_type_ids(&self) -> PyResult<Py<PyArray1<NodeTypeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(gil, pe!(self.inner.get_unique_node_type_ids())?, NodeTypeT)
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the unique node types names.
    pub fn get_unique_node_type_names(&self) -> PyResult<Vec<String>> {
        Ok(pe!(self.inner.get_unique_node_type_names())?
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return number of the unique edges in the graph
    pub fn get_number_of_unique_directed_edges(&self) -> EdgeT {
        self.inner.get_number_of_unique_directed_edges().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the nodes mapping
    pub fn get_nodes_mapping(&self) -> HashMap<String, NodeT> {
        self.inner.get_nodes_mapping().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, directed)")]
    /// Return vector with the sorted edge Ids.
    ///
    /// Parameters
    /// ----------
    /// directed: bool
    ///     Whether to filter out the undirected edges.
    ///
    pub fn get_edge_node_ids(&self, directed: bool) -> Py<PyArray2<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_2d!(gil, self.inner.get_edge_node_ids(directed.clone()), NodeT)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return vector with the sorted directed edge node IDs
    pub fn get_directed_edge_node_ids(&self) -> Py<PyArray2<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_2d!(gil, self.inner.get_directed_edge_node_ids(), NodeT)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return vector with the sorted directed triples with (source, edge_type, destination) IDs.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not contain edge types.
    ///
    pub fn get_directed_edge_triples_ids(&self) -> PyResult<Py<PyArray2<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_2d!(gil, pe!(self.inner.get_directed_edge_triples_ids())?, NodeT)
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, directed)")]
    /// Return vector with the sorted edge names.
    ///
    /// Parameters
    /// ----------
    /// directed: bool
    ///     Whether to filter out the undirected edges.
    ///
    pub fn get_edge_node_names(&self, directed: bool) -> Vec<(String, String)> {
        self.inner
            .get_edge_node_names(directed.clone())
            .into_iter()
            .map(|x| {
                let (subresult_0, subresult_1) = x;
                (subresult_0.into(), subresult_1.into())
            })
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return vector with the sorted directed edge names
    pub fn get_directed_edge_node_names(&self) -> Vec<(String, String)> {
        self.inner
            .get_directed_edge_node_names()
            .into_iter()
            .map(|x| {
                let (subresult_0, subresult_1) = x;
                (subresult_0.into(), subresult_1.into())
            })
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return vector with the sorted directed triples with (source, edge_type, destination) names.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not contain edge types.
    ///
    pub fn get_directed_edge_triples_names(&self) -> PyResult<Vec<Vec<String>>> {
        Ok(pe!(self.inner.get_directed_edge_triples_names())?
            .into_iter()
            .map(|x| x.into_iter().map(|x| x.into()).collect::<Vec<_>>())
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns number of nodes with unknown node type.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no node types in the graph.
    ///
    pub fn get_number_of_unknown_node_types(&self) -> PyResult<NodeT> {
        Ok(pe!(self.inner.get_number_of_unknown_node_types())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns the number of node with known node type.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no node types in the graph.
    ///
    pub fn get_number_of_known_node_types(&self) -> PyResult<NodeT> {
        Ok(pe!(self.inner.get_number_of_known_node_types())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
    /// Returns number of maximum multilabel count.
    ///
    /// This value is the maximum number of multilabel counts
    /// that appear in any given node in the graph
    pub fn get_maximum_multilabel_count(&self) -> PyResult<NodeTypeT> {
        Ok(pe!(self.inner.get_maximum_multilabel_count())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns number of singleton node types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn get_number_of_singleton_node_types(&self) -> PyResult<NodeTypeT> {
        Ok(pe!(self.inner.get_number_of_singleton_node_types())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns number of homogeneous node types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn get_number_of_homogeneous_node_types(&self) -> PyResult<NodeTypeT> {
        Ok(pe!(self.inner.get_number_of_homogeneous_node_types())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
    /// Returns number of unknown edge types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    ///
    pub fn get_number_of_unknown_edge_types(&self) -> PyResult<EdgeT> {
        Ok(pe!(self.inner.get_number_of_unknown_edge_types())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self, directed)")]
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
    ) -> PyResult<Py<PyArray2<NodeT>>> {
        Ok({
            // Warning: this copies the array so it uses double the memory.
            // To avoid this you should directly generate data compatible with a numpy array
            // Which is a flat vector with row-first or column-first unrolling
            let gil = pyo3::Python::acquire_gil();
            let body = pe!(self
                .inner
                .get_edge_node_ids_with_unknown_edge_types(directed.clone()))?;
            let result_array = ThreadDataRaceAware {
                t: unsafe { PyArray2::<NodeT>::new(gil.python(), [body.len(), 2], false) },
            };
            body.into_par_iter()
                .enumerate()
                .for_each(|(i, (a, b))| unsafe {
                    *(result_array.t.uget_mut([i, 0])) = a;
                    *(result_array.t.uget_mut([i, 1])) = b;
                });
            result_array.t.to_owned()
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, directed)")]
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
    ) -> PyResult<Py<PyArray2<NodeT>>> {
        Ok({
            // Warning: this copies the array so it uses double the memory.
            // To avoid this you should directly generate data compatible with a numpy array
            // Which is a flat vector with row-first or column-first unrolling
            let gil = pyo3::Python::acquire_gil();
            let body = pe!(self
                .inner
                .get_edge_node_ids_with_known_edge_types(directed.clone()))?;
            let result_array = ThreadDataRaceAware {
                t: unsafe { PyArray2::<NodeT>::new(gil.python(), [body.len(), 2], false) },
            };
            body.into_par_iter()
                .enumerate()
                .for_each(|(i, (a, b))| unsafe {
                    *(result_array.t.uget_mut([i, 0])) = a;
                    *(result_array.t.uget_mut([i, 1])) = b;
                });
            result_array.t.to_owned()
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, directed)")]
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
            .get_edge_node_names_with_unknown_edge_types(directed.clone()))?
        .into_iter()
        .map(|x| {
            let (subresult_0, subresult_1) = x;
            (subresult_0.into(), subresult_1.into())
        })
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, directed)")]
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
            .get_edge_node_names_with_known_edge_types(directed.clone()))?
        .into_iter()
        .map(|x| {
            let (subresult_0, subresult_1) = x;
            (subresult_0.into(), subresult_1.into())
        })
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns a boolean vector that for each node contains whether it has an
    /// unknown node type.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    ///
    pub fn get_edges_with_unknown_edge_types_mask(&self) -> PyResult<Py<PyArray1<bool>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_edges_with_unknown_edge_types_mask())?,
                bool
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns a boolean vector that for each node contains whether it has an
    /// unknown edge type.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    ///
    pub fn get_edges_with_known_edge_types_mask(&self) -> PyResult<Py<PyArray1<bool>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_edges_with_known_edge_types_mask())?,
                bool
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self, node_type_id)")]
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
                    .get_node_ids_from_node_type_id(node_type_id.clone()))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_ids)")]
    /// Returns node IDs of the nodes with given node type IDs.
    ///
    /// Parameters
    /// ----------
    /// node_type_ids: List[Optional[int]]
    ///     The node type ID to filter for.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no node types in the graph.
    ///
    pub fn get_node_ids_from_node_type_ids(
        &self,
        node_type_ids: Vec<Option<NodeTypeT>>,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_node_ids_from_node_type_ids(&node_type_ids))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_names)")]
    /// Returns node IDs of the nodes with given node type names.
    ///
    /// Parameters
    /// ----------
    /// node_type_names: List[Optional[&str]]
    ///     The node type ID to filter for.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no node types in the graph.
    ///
    pub fn get_node_ids_from_node_type_names(
        &self,
        node_type_names: Vec<Option<&str>>,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_node_ids_from_node_type_names(&node_type_names))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_id)")]
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
            .get_node_names_from_node_type_id(node_type_id.clone()))?
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_name)")]
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
                pe!(self.inner.get_node_ids_from_node_type_name(node_type_name))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_name)")]
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
            .get_node_names_from_node_type_name(node_type_name))?
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
    /// Returns a boolean vector that for each node contains whether it has an
    /// unknown node type.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no node types in the graph.
    ///
    pub fn get_nodes_with_unknown_node_types_mask(&self) -> PyResult<Py<PyArray1<bool>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_nodes_with_unknown_node_types_mask())?,
                bool
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns a boolean vector that for each node contains whether it has an
    /// known node type.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no node types in the graph.
    ///
    pub fn get_nodes_with_known_node_types_mask(&self) -> PyResult<Py<PyArray1<bool>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_nodes_with_known_node_types_mask())?,
                bool
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns the number of edge with known edge type.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    ///
    pub fn get_number_of_known_edge_types(&self) -> PyResult<EdgeT> {
        Ok(pe!(self.inner.get_number_of_known_edge_types())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
    /// Returns number of singleton edge types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    ///
    pub fn get_number_of_singleton_edge_types(&self) -> PyResult<EdgeTypeT> {
        Ok(pe!(self.inner.get_number_of_singleton_edge_types())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
    /// Returns number of nodes in the graph
    pub fn get_number_of_nodes(&self) -> NodeT {
        self.inner.get_number_of_nodes().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, verbose)")]
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
            self.inner.get_node_connected_component_ids(verbose),
            NodeT
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns number of directed edges in the graph
    pub fn get_number_of_directed_edges(&self) -> EdgeT {
        self.inner.get_number_of_directed_edges().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns number of edge types in the graph.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the current graph.
    ///
    pub fn get_number_of_edge_types(&self) -> PyResult<EdgeTypeT> {
        Ok(pe!(self.inner.get_number_of_edge_types())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns number of node types in the graph.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no node types in the current graph.
    ///
    pub fn get_number_of_node_types(&self) -> PyResult<NodeTypeT> {
        Ok(pe!(self.inner.get_number_of_node_types())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns the unweighted degree of every node in the graph
    pub fn get_node_degrees(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_node_degrees(), NodeT)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the indegree for each node.
    pub fn get_node_indegrees(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_node_indegrees(), NodeT)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns the weighted degree of every node in the graph
    pub fn get_weighted_node_degrees(&self) -> PyResult<Py<PyArray1<f64>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(gil, pe!(self.inner.get_weighted_node_degrees())?, f64)
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return set of nodes that are not singletons
    pub fn get_not_singletons_node_ids(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_not_singletons_node_ids(), NodeT)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return mapping from instance not trap nodes to dense nodes
    pub fn get_dense_nodes_mapping(&self) -> HashMap<NodeT, NodeT> {
        self.inner.get_dense_nodes_mapping().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return number of edges that have multigraph syblings
    pub fn get_number_of_parallel_edges(&self) -> EdgeT {
        self.inner.get_number_of_parallel_edges().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return vector with node cumulative_node_degrees, that is the comulative node degree
    pub fn get_cumulative_node_degrees(&self) -> Py<PyArray1<EdgeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_cumulative_node_degrees(), EdgeT)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return vector wit
    pub fn get_reciprocal_sqrt_degrees(&self) -> Py<PyArray1<WeightT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_reciprocal_sqrt_degrees(), WeightT)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns number of the source nodes.
    pub fn get_number_of_unique_source_nodes(&self) -> NodeT {
        self.inner.get_number_of_unique_source_nodes().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self, unknown_node_types_value)")]
    /// Returns 1D single labeled node types ids vector.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph has multilabel node types.
    ///
    pub fn get_single_label_node_type_ids(
        &self,
        unknown_node_types_value: Option<NodeTypeT>,
    ) -> PyResult<Py<PyArray1<NodeTypeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_single_label_node_type_ids(unknown_node_types_value))?,
                NodeTypeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns 1D known single labeled node types ids vector.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph has multilabel node types.
    ///
    pub fn get_known_single_label_node_type_ids(&self) -> PyResult<Py<PyArray1<NodeTypeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_known_single_label_node_type_ids())?,
                NodeTypeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, target_value, unknown_node_types_value)")]
    /// Returns 1D binarized node types ids vector
    pub fn get_boolean_node_type_ids(
        &self,
        target_value: Option<NodeTypeT>,
        unknown_node_types_value: Option<NodeTypeT>,
    ) -> PyResult<Py<PyArray1<bool>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_boolean_node_type_ids(target_value, unknown_node_types_value))?,
                bool
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, target_value)")]
    /// Returns 1D binarized known node types ids vector
    pub fn get_known_boolean_node_type_ids(
        &self,
        target_value: NodeTypeT,
    ) -> PyResult<Py<PyArray1<bool>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_known_boolean_node_type_ids(target_value.clone()))?,
                bool
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns vector of root node ids, nodes with zero inbound degree and non-zero outbound degree
    pub fn get_root_node_ids(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_root_node_ids(), NodeT)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns vector of root node names, nodes with zero inbound degree and non-zero outbound degree
    pub fn get_root_node_names(&self) -> Vec<String> {
        self.inner
            .get_root_node_names()
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Convert inplace the graph to directed.
    pub fn to_directed_inplace(&mut self) {
        self.inner.to_directed_inplace();
        ()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return a new instance of the current graph as directed
    pub fn to_directed(&self) -> Graph {
        self.inner.to_directed().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the directed graph from the upper triangular adjacency matrix.
    pub fn to_upper_triangular(&self) -> Graph {
        self.inner.to_upper_triangular().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the directed graph from the lower triangular adjacency matrix.
    pub fn to_lower_triangular(&self) -> Graph {
        self.inner.to_lower_triangular().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the graph from the main diagonal adjacency matrix.
    pub fn to_main_diagonal(&self) -> Graph {
        self.inner.to_main_diagonal().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the graph from the anti-diagonal adjacency matrix.
    pub fn to_anti_diagonal(&self) -> Graph {
        self.inner.to_anti_diagonal().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the graph from the bidiagonal adjacency matrix.
    pub fn to_bidiagonal(&self) -> Graph {
        self.inner.to_bidiagonal().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the graph from the arrowhead adjacency matrix.
    pub fn to_arrowhead(&self) -> Graph {
        self.inner.to_arrowhead().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the graph from the transposed adjacency matrix.
    pub fn to_transposed(&self) -> Graph {
        self.inner.to_transposed().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the graph with all symmetric edges
    pub fn to_undirected(&self) -> Graph {
        self.inner.to_undirected().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the complementary graph.
    pub fn to_complementary(&self) -> Graph {
        self.inner.to_complementary().into()
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, maximal_hop_distance, change_layer_probability, random_walk_length, iterations)"
    )]
    /// Returns structural similarity multi-graph.
    ///
    /// Parameters
    /// ----------
    /// maximal_hop_distance: Optional[int]
    ///     The maximal hop distance. By default, equal to the graph diameter. By default, equal to the diameter.
    /// change_layer_probability: Optional[float]
    ///     Probability to change the layer during the random walk. By default 0.5.
    /// random_walk_length: Optional[int]
    ///     Length of the random walk to be used to compute the approximated stationary distribution. By default, 1024.
    /// iterations: Optional[int]
    ///     Number of iterations per node to compute the approximated stationary distribution. By default 1.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the provided graph does not have any edges.
    /// ValueError
    ///     If the provided change layer probability is not a probability.
    /// ValueError
    ///     If the provided random walk parameters are not valid.
    ///
    pub fn to_structural_similarity_multi_graph(
        &self,
        maximal_hop_distance: Option<usize>,
        change_layer_probability: Option<f32>,
        random_walk_length: Option<u64>,
        iterations: Option<NodeT>,
    ) -> PyResult<Graph> {
        Ok(pe!(self.inner.to_structural_similarity_multi_graph(
            maximal_hop_distance,
            change_layer_probability,
            random_walk_length,
            iterations
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, minimum_degree, minimum_clique_size, clique_per_node, verbose)"
    )]
    /// Returns graph cliques with at least `minimum_degree` nodes.
    ///
    /// Parameters
    /// ----------
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the current graph is directed.
    ///
    pub fn get_approximated_cliques(
        &self,
        minimum_degree: Option<NodeT>,
        minimum_clique_size: Option<NodeT>,
        clique_per_node: Option<usize>,
        verbose: Option<bool>,
    ) -> PyResult<Vec<Clique>> {
        Ok(pe!(self.inner.get_approximated_cliques(
            minimum_degree,
            minimum_clique_size,
            clique_per_node,
            verbose
        ))?
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns the maximum clique in the graph.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the current graph is directed.
    ///
    pub fn get_max_clique(&self) -> PyResult<Clique> {
        Ok(pe!(self.inner.get_max_clique())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, minimum_degree, minimum_clique_size, clique_per_node, verbose)"
    )]
    /// Returns number of graph cliques with at least `minimum_degree` nodes.
    ///
    /// Parameters
    /// ----------
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the current graph is directed.
    ///
    pub fn get_approximated_number_of_cliques(
        &self,
        minimum_degree: Option<NodeT>,
        minimum_clique_size: Option<NodeT>,
        clique_per_node: Option<usize>,
        verbose: Option<bool>,
    ) -> PyResult<usize> {
        Ok(pe!(self.inner.get_approximated_number_of_cliques(
            minimum_degree,
            minimum_clique_size,
            clique_per_node,
            verbose
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self, other, verbose)")]
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
        Ok(pe!(self.inner.overlap_textual_report(&other.inner, verbose))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
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
        Ok(pe!(self.inner.get_node_report_from_node_id(node_id.clone()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_name)")]
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
        Ok(pe!(self.inner.get_node_report_from_node_name(node_name))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(
        text_signature = "(random_state, minimum_node_id, minimum_node_sampling, maximum_node_sampling, nodes_number, include_selfloops, node_type, edge_type, weight, directed, name)"
    )]
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
    /// node_type: Optional[&str]
    ///     The node type to use for the chain. By default 'chain'.
    /// edge_type: Optional[&str]
    ///     The node type to use for the chain. By default 'chain'.
    /// weight: Optional[float]
    ///     The weight to use for the edges in the chain. By default None.
    /// directed: Optional[bool]
    ///     Whether the graph is to built as directed. By default false.
    /// name: Optional[&str]
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
            random_state,
            minimum_node_id,
            minimum_node_sampling,
            maximum_node_sampling,
            nodes_number,
            include_selfloops,
            node_type,
            edge_type,
            weight,
            directed,
            name
        ))?
        .into())
    }

    #[staticmethod]
    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "(random_state, minimum_node_id, nodes_number, include_selfloops, node_type, edge_type, weight, directed, name)"
    )]
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
    /// node_type: Optional[&str]
    ///     The node type to use for the chain. By default 'chain'.
    /// edge_type: Optional[&str]
    ///     The node type to use for the chain. By default 'chain'.
    /// weight: Optional[float]
    ///     The weight to use for the edges in the chain. By default None.
    /// directed: Optional[bool]
    ///     Whether the graph is to built as directed. By default false.
    /// name: Optional[&str]
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
            random_state,
            minimum_node_id,
            nodes_number,
            include_selfloops,
            node_type,
            edge_type,
            weight,
            directed,
            name
        ))?
        .into())
    }

    #[staticmethod]
    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "(minimum_node_id, nodes_number, include_selfloops, node_type, edge_type, weight, directed, name)"
    )]
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
    /// node_type: Optional[&str]
    ///     The node type to use for the star. By default 'star'.
    /// edge_type: Optional[&str]
    ///     The node type to use for the star. By default 'star'.
    /// weight: Optional[float]
    ///     The weight to use for the edges in the star. By default None.
    /// directed: Optional[bool]
    ///     Whether the graph is to built as directed. By default false.
    /// name: Optional[&str]
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
            minimum_node_id,
            nodes_number,
            include_selfloops,
            node_type,
            edge_type,
            weight,
            directed,
            name
        ))?
        .into())
    }

    #[staticmethod]
    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "(minimum_node_id, nodes_number, include_selfloops, node_type, edge_type, weight, directed, name)"
    )]
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
    /// node_type: Optional[&str]
    ///     The node type to use for the wheel. By default 'wheel'.
    /// edge_type: Optional[&str]
    ///     The node type to use for the wheel. By default 'wheel'.
    /// weight: Optional[float]
    ///     The weight to use for the edges in the wheel. By default None.
    /// directed: Optional[bool]
    ///     Whether the graph is to built as directed. By default false.
    /// name: Optional[&str]
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
            minimum_node_id,
            nodes_number,
            include_selfloops,
            node_type,
            edge_type,
            weight,
            directed,
            name
        ))?
        .into())
    }

    #[staticmethod]
    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "(minimum_node_id, nodes_number, include_selfloops, node_type, edge_type, weight, directed, name)"
    )]
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
    /// node_type: Optional[&str]
    ///     The node type to use for the circle. By default 'circle'.
    /// edge_type: Optional[&str]
    ///     The node type to use for the circle. By default 'circle'.
    /// weight: Optional[float]
    ///     The weight to use for the edges in the circle. By default None.
    /// directed: Optional[bool]
    ///     Whether the graph is to built as directed. By default false.
    /// name: Optional[&str]
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
            minimum_node_id,
            nodes_number,
            include_selfloops,
            node_type,
            edge_type,
            weight,
            directed,
            name
        ))?
        .into())
    }

    #[staticmethod]
    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "(minimum_node_id, nodes_number, include_selfloops, node_type, edge_type, weight, directed, name)"
    )]
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
    /// node_type: Optional[&str]
    ///     The node type to use for the chain. By default 'chain'.
    /// edge_type: Optional[&str]
    ///     The node type to use for the chain. By default 'chain'.
    /// weight: Optional[float]
    ///     The weight to use for the edges in the chain. By default None.
    /// directed: Optional[bool]
    ///     Whether the graph is to built as directed. By default false.
    /// name: Optional[&str]
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
            minimum_node_id,
            nodes_number,
            include_selfloops,
            node_type,
            edge_type,
            weight,
            directed,
            name
        ))?
        .into())
    }

    #[staticmethod]
    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "(minimum_node_id, nodes_number, include_selfloops, node_type, edge_type, weight, directed, name)"
    )]
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
    /// node_type: Optional[&str]
    ///     The node type to use. By default 'complete'.
    /// edge_type: Optional[&str]
    ///     The node type to use. By default 'complete'.
    /// weight: Optional[float]
    ///     The weight to use for the edges. By default None.
    /// directed: Optional[bool]
    ///     Whether the graph is to built as directed. By default false.
    /// name: Optional[&str]
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
            minimum_node_id,
            nodes_number,
            include_selfloops,
            node_type,
            edge_type,
            weight,
            directed,
            name
        ))?
        .into())
    }

    #[staticmethod]
    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "(minimum_node_id, left_clique_nodes_number, right_clique_nodes_number, chain_nodes_number, include_selfloops, left_clique_node_type, right_clique_node_type, chain_node_type, left_clique_edge_type, right_clique_edge_type, chain_edge_type, left_clique_weight, right_clique_weight, chain_weight, directed, name)"
    )]
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
    /// left_clique_node_type: Optional[&str]
    ///     The node type to use for the left clique. By default 'left_clique'.
    /// right_clique_node_type: Optional[&str]
    ///     The node type to use for the right clique. By default 'right_clique'.
    /// chain_node_type: Optional[&str]
    ///     The node type to use for the chain. By default 'chain'.
    /// left_clique_edge_type: Optional[&str]
    ///     The node type to use for the left clique. By default 'left_clique'.
    /// right_clique_edge_type: Optional[&str]
    ///     The node type to use for the right clique. By default 'right_clique'.
    /// chain_edge_type: Optional[&str]
    ///     The node type to use for the chain. By default 'chain'.
    /// left_clique_weight: Optional[float]
    ///     The weight to use for the edges in the left clique. By default None.
    /// right_clique_weight: Optional[float]
    ///     The weight to use for the edges in the right clique. By default None.
    /// chain_weight: Optional[float]
    ///     The weight to use for the edges in the chain. By default None.
    /// directed: Optional[bool]
    ///     Whether the graph is to built as directed. By default false.
    /// name: Optional[&str]
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
            minimum_node_id,
            left_clique_nodes_number,
            right_clique_nodes_number,
            chain_nodes_number,
            include_selfloops,
            left_clique_node_type,
            right_clique_node_type,
            chain_node_type,
            left_clique_edge_type,
            right_clique_edge_type,
            chain_edge_type,
            left_clique_weight,
            right_clique_weight,
            chain_weight,
            directed,
            name
        ))?
        .into())
    }

    #[staticmethod]
    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "(minimum_node_id, clique_nodes_number, chain_nodes_number, include_selfloops, clique_node_type, chain_node_type, clique_edge_type, chain_edge_type, clique_weight, chain_weight, directed, name)"
    )]
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
    /// clique_node_type: Optional[&str]
    ///     The node type to use for the left clique. By default 'clique'.
    /// chain_node_type: Optional[&str]
    ///     The node type to use for the chain. By default 'chain'.
    /// clique_edge_type: Optional[&str]
    ///     The node type to use for the left clique. By default 'clique'.
    /// chain_edge_type: Optional[&str]
    ///     The node type to use for the chain. By default 'chain'.
    /// clique_weight: Optional[float]
    ///     The weight to use for the edges in the left clique. By default None.
    /// chain_weight: Optional[float]
    ///     The weight to use for the edges in the chain. By default None.
    /// directed: Optional[bool]
    ///     Whether the graph is to built as directed. By default false.
    /// name: Optional[&str]
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
            minimum_node_id,
            clique_nodes_number,
            chain_nodes_number,
            include_selfloops,
            clique_node_type,
            chain_node_type,
            clique_edge_type,
            chain_edge_type,
            clique_weight,
            chain_weight,
            directed,
            name
        ))?
        .into())
    }

    #[staticmethod]
    #[automatically_generated_binding]
    #[pyo3(text_signature = "(sides, minimum_node_id, node_type, weight, directed, name)")]
    /// Creates new squared lattice graph with given sizes and types.
    ///
    /// Parameters
    /// ----------
    /// sides: List[int]
    ///     Sides of the hyper-dimensional lattice with square cell.
    /// minimum_node_id: Optional[int]
    ///     Minimum node ID to start with. May be needed when chaining graphs. By default 0.
    /// node_type: Optional[&str]
    ///     The node type to use for the squared lattice. By default 'squared_lattice'.
    /// weight: Optional[float]
    ///     The weight to use for the edges in the left clique. By default None.
    /// directed: Optional[bool]
    ///     Whether the graph is to built as directed. By default false.
    /// name: Optional[&str]
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
            minimum_node_id,
            node_type,
            weight,
            directed,
            name
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, node_ids_to_keep, node_ids_to_remove, node_names_to_keep_from_graph, node_names_to_remove_from_graph, node_prefixes_to_keep, node_prefixes_to_remove, node_type_ids_to_keep, node_type_ids_to_remove, node_type_id_to_keep, node_type_id_to_remove, source_node_ids_to_keep, source_node_ids_to_remove, source_node_names_to_keep_from_graph, source_node_names_to_remove_from_graph, source_node_prefixes_to_keep, source_node_prefixes_to_remove, source_node_type_ids_to_keep, source_node_type_ids_to_remove, source_node_type_id_to_keep, source_node_type_id_to_remove, destination_node_ids_to_keep, destination_node_ids_to_remove, destination_node_names_to_keep_from_graph, destination_node_names_to_remove_from_graph, destination_node_prefixes_to_keep, destination_node_prefixes_to_remove, destination_node_type_ids_to_keep, destination_node_type_ids_to_remove, destination_node_type_id_to_keep, destination_node_type_id_to_remove, edge_ids_to_keep, edge_ids_to_remove, edge_node_ids_to_keep, edge_node_ids_to_remove, edge_type_ids_to_keep, edge_type_ids_to_remove, min_edge_weight, max_edge_weight, min_node_degree, max_node_degree, filter_singleton_nodes, filter_singleton_nodes_with_selfloop, filter_selfloops, filter_parallel_edges)"
    )]
    /// Returns a **NEW** Graph that does not have the required attributes.
    ///
    /// Parameters
    /// ----------
    /// node_ids_to_keep: Optional[List[int]]
    ///     List of node IDs to keep during filtering.
    /// node_ids_to_remove: Optional[List[int]]
    ///     List of node IDs to remove during filtering.
    /// node_names_to_keep_from_graph: Optional[&Graph]
    ///     Graph whose nodes are to be kept.
    /// node_names_to_remove_from_graph: Optional[&Graph]
    ///     Graph whose nodes are to be removed.
    /// node_prefixes_to_keep: Optional[List[str]]
    ///     List of node prefixes to keep during filtering.
    /// node_prefixes_to_remove: Optional[List[str]]
    ///     List of node prefixes to remove during filtering.
    /// node_type_ids_to_keep: Optional[List[Optional[List[int]]]]
    ///     List of node type IDs to keep during filtering. The node types must match entirely the given node types vector provided.
    /// node_type_ids_to_remove: Optional[List[Optional[List[int]]]]
    ///     List of node type IDs to remove during filtering. The node types must match entirely the given node types vector provided.
    /// node_type_id_to_keep: Optional[List[Optional[int]]]
    ///     List of node type IDs to keep during filtering. Any of node types must match with one of the node types given.
    /// node_type_id_to_remove: Optional[List[Optional[int]]]
    ///     List of node type IDs to remove during filtering. Any of node types must match with one of the node types given.
    /// source_node_ids_to_keep: Optional[List[int]]
    ///     List of source node IDs to keep during filtering.
    /// source_node_ids_to_remove: Optional[List[int]]
    ///     List of source node IDs to remove during filtering.
    /// source_node_names_to_keep_from_graph: Optional[&Graph]
    ///     Graph whose nodes are to be kept when they are source nodes in this graph instance.
    /// source_node_names_to_remove_from_graph: Optional[&Graph]
    ///     Graph whose nodes are to be removed when they are source nodes in this graph instance.
    /// source_node_prefixes_to_keep: Optional[List[str]]
    ///     List of source node prefixes to keep during filtering.
    /// source_node_prefixes_to_remove: Optional[List[str]]
    ///     List of source node prefixes to remove during filtering.
    /// source_node_type_ids_to_keep: Optional[List[Optional[List[int]]]]
    ///     List of source node type IDs to keep during filtering. The node types must match entirely the given node types vector provided.
    /// source_node_type_ids_to_remove: Optional[List[Optional[List[int]]]]
    ///     List of source node type IDs to remove during filtering. The node types must match entirely the given node types vector provided.
    /// source_node_type_id_to_keep: Optional[List[Optional[int]]]
    ///     List of source node type IDs to keep during filtering. Any of source node types must match with one of the node types given.
    /// source_node_type_id_to_remove: Optional[List[Optional[int]]]
    ///     List of source node type IDs to remove during filtering. Any of source node types must match with one of the node types given.
    /// destination_node_ids_to_keep: Optional[List[int]]
    ///     List of destination node IDs to keep during filtering.
    /// destination_node_ids_to_remove: Optional[List[int]]
    ///     List of destination node IDs to remove during filtering.
    /// destination_node_names_to_keep_from_graph: Optional[&Graph]
    ///     Graph whose nodes are to be kept when they are destination nodes in this graph instance.
    /// destination_node_names_to_remove_from_graph: Optional[&Graph]
    ///     Graph whose nodes are to be removed when they are destination nodes in this graph instance.
    /// destination_node_prefixes_to_keep: Optional[List[str]]
    ///     List of destination node prefixes to keep during filtering.
    /// destination_node_prefixes_to_remove: Optional[List[str]]
    ///     List of destination node prefixes to remove during filtering.
    /// destination_node_type_ids_to_keep: Optional[List[Optional[List[int]]]]
    ///     List of destination node type IDs to keep during filtering. The node types must match entirely the given node types vector provided.
    /// destination_node_type_ids_to_remove: Optional[List[Optional[List[int]]]]
    ///     List of destination node type IDs to remove during filtering. The node types must match entirely the given node types vector provided.
    /// destination_node_type_id_to_keep: Optional[List[Optional[int]]]
    ///     List of destination node type IDs to keep during filtering. Any of destination node types must match with one of the node types given.
    /// destination_node_type_id_to_remove: Optional[List[Optional[int]]]
    ///     List of destination node type IDs to remove during filtering. Any of destination node types must match with one of the node types given.
    /// edge_ids_to_keep: Optional[List[int]]
    ///     List of edge IDs to keep during filtering.
    /// edge_ids_to_remove: Optional[List[int]]
    ///     List of edge IDs to remove during filtering.
    /// edge_node_ids_to_keep: Optional[List[Tuple[int, int]]]
    ///     List of tuple of node IDs to keep during filtering.
    /// edge_node_ids_to_remove: Optional[List[Tuple[int, int]]]
    ///     List of tuple of node IDs to remove during filtering.
    /// edge_type_ids_to_keep: Optional[List[Optional[int]]]
    ///     List of edge type IDs to keep during filtering.
    /// edge_type_ids_to_remove: Optional[List[Optional[int]]]
    ///     List of edge type IDs to remove during filtering.
    /// min_edge_weight: Optional[float]
    ///     Minimum edge weight. Values lower than this are removed.
    /// max_edge_weight: Optional[float]
    ///     Maximum edge weight. Values higher than this are removed.
    /// min_node_degree: Optional[int]
    ///     Minimum node degree. Values lower than this are removed.
    /// max_node_degree: Optional[int]
    ///     Maximum node degree. Values higher than this are removed.
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
        node_ids_to_remove: Option<Vec<NodeT>>,
        node_names_to_keep_from_graph: Option<&Graph>,
        node_names_to_remove_from_graph: Option<&Graph>,
        node_prefixes_to_keep: Option<Vec<String>>,
        node_prefixes_to_remove: Option<Vec<String>>,
        node_type_ids_to_keep: Option<Vec<Option<Vec<NodeTypeT>>>>,
        node_type_ids_to_remove: Option<Vec<Option<Vec<NodeTypeT>>>>,
        node_type_id_to_keep: Option<Vec<Option<NodeTypeT>>>,
        node_type_id_to_remove: Option<Vec<Option<NodeTypeT>>>,
        source_node_ids_to_keep: Option<Vec<NodeT>>,
        source_node_ids_to_remove: Option<Vec<NodeT>>,
        source_node_names_to_keep_from_graph: Option<&Graph>,
        source_node_names_to_remove_from_graph: Option<&Graph>,
        source_node_prefixes_to_keep: Option<Vec<String>>,
        source_node_prefixes_to_remove: Option<Vec<String>>,
        source_node_type_ids_to_keep: Option<Vec<Option<Vec<NodeTypeT>>>>,
        source_node_type_ids_to_remove: Option<Vec<Option<Vec<NodeTypeT>>>>,
        source_node_type_id_to_keep: Option<Vec<Option<NodeTypeT>>>,
        source_node_type_id_to_remove: Option<Vec<Option<NodeTypeT>>>,
        destination_node_ids_to_keep: Option<Vec<NodeT>>,
        destination_node_ids_to_remove: Option<Vec<NodeT>>,
        destination_node_names_to_keep_from_graph: Option<&Graph>,
        destination_node_names_to_remove_from_graph: Option<&Graph>,
        destination_node_prefixes_to_keep: Option<Vec<String>>,
        destination_node_prefixes_to_remove: Option<Vec<String>>,
        destination_node_type_ids_to_keep: Option<Vec<Option<Vec<NodeTypeT>>>>,
        destination_node_type_ids_to_remove: Option<Vec<Option<Vec<NodeTypeT>>>>,
        destination_node_type_id_to_keep: Option<Vec<Option<NodeTypeT>>>,
        destination_node_type_id_to_remove: Option<Vec<Option<NodeTypeT>>>,
        edge_ids_to_keep: Option<Vec<EdgeT>>,
        edge_ids_to_remove: Option<Vec<EdgeT>>,
        edge_node_ids_to_keep: Option<Vec<(NodeT, NodeT)>>,
        edge_node_ids_to_remove: Option<Vec<(NodeT, NodeT)>>,
        edge_type_ids_to_keep: Option<Vec<Option<EdgeTypeT>>>,
        edge_type_ids_to_remove: Option<Vec<Option<EdgeTypeT>>>,
        min_edge_weight: Option<WeightT>,
        max_edge_weight: Option<WeightT>,
        min_node_degree: Option<NodeT>,
        max_node_degree: Option<NodeT>,
        filter_singleton_nodes: Option<bool>,
        filter_singleton_nodes_with_selfloop: Option<bool>,
        filter_selfloops: Option<bool>,
        filter_parallel_edges: Option<bool>,
    ) -> PyResult<Graph> {
        Ok(pe!(self.inner.filter_from_ids(
            node_ids_to_keep,
            node_ids_to_remove,
            node_names_to_keep_from_graph.map(|sg| &sg.inner),
            node_names_to_remove_from_graph.map(|sg| &sg.inner),
            node_prefixes_to_keep,
            node_prefixes_to_remove,
            node_type_ids_to_keep,
            node_type_ids_to_remove,
            node_type_id_to_keep,
            node_type_id_to_remove,
            source_node_ids_to_keep,
            source_node_ids_to_remove,
            source_node_names_to_keep_from_graph.map(|sg| &sg.inner),
            source_node_names_to_remove_from_graph.map(|sg| &sg.inner),
            source_node_prefixes_to_keep,
            source_node_prefixes_to_remove,
            source_node_type_ids_to_keep,
            source_node_type_ids_to_remove,
            source_node_type_id_to_keep,
            source_node_type_id_to_remove,
            destination_node_ids_to_keep,
            destination_node_ids_to_remove,
            destination_node_names_to_keep_from_graph.map(|sg| &sg.inner),
            destination_node_names_to_remove_from_graph.map(|sg| &sg.inner),
            destination_node_prefixes_to_keep,
            destination_node_prefixes_to_remove,
            destination_node_type_ids_to_keep,
            destination_node_type_ids_to_remove,
            destination_node_type_id_to_keep,
            destination_node_type_id_to_remove,
            edge_ids_to_keep,
            edge_ids_to_remove,
            edge_node_ids_to_keep,
            edge_node_ids_to_remove,
            edge_type_ids_to_keep,
            edge_type_ids_to_remove,
            min_edge_weight,
            max_edge_weight,
            min_node_degree,
            max_node_degree,
            filter_singleton_nodes,
            filter_singleton_nodes_with_selfloop,
            filter_selfloops,
            filter_parallel_edges
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, node_names_to_keep, node_names_to_remove, node_names_to_keep_from_graph, node_names_to_remove_from_graph, node_prefixes_to_keep, node_prefixes_to_remove, node_type_names_to_keep, node_type_names_to_remove, node_type_name_to_keep, node_type_name_to_remove, source_node_names_to_keep, source_node_names_to_remove, source_node_names_to_keep_from_graph, source_node_names_to_remove_from_graph, source_node_prefixes_to_keep, source_node_prefixes_to_remove, source_node_type_names_to_keep, source_node_type_names_to_remove, source_node_type_name_to_keep, source_node_type_name_to_remove, destination_node_names_to_keep, destination_node_names_to_remove, destination_node_names_to_keep_from_graph, destination_node_names_to_remove_from_graph, destination_node_prefixes_to_keep, destination_node_prefixes_to_remove, destination_node_type_names_to_keep, destination_node_type_names_to_remove, destination_node_type_name_to_keep, destination_node_type_name_to_remove, edge_node_names_to_keep, edge_node_names_to_remove, edge_type_names_to_keep, edge_type_names_to_remove, min_edge_weight, max_edge_weight, min_node_degree, max_node_degree, filter_singleton_nodes, filter_singleton_nodes_with_selfloop, filter_selfloops, filter_parallel_edges)"
    )]
    /// Returns a **NEW** Graph that does not have the required attributes.
    ///
    /// Parameters
    /// ----------
    /// node_names_to_keep: Optional[List[&str]]
    ///     List of node names to keep during filtering.
    /// node_names_to_remove: Optional[List[&str]]
    ///     List of node names to remove during filtering.
    /// node_names_to_keep_from_graph: Optional[&Graph]
    ///     Graph whose nodes are to be kept.
    /// node_names_to_remove_from_graph: Optional[&Graph]
    ///     Graph whose nodes are to be removed.
    /// node_prefixes_to_keep: Optional[List[str]]
    ///     List of node prefixes to keep during filtering.
    /// node_prefixes_to_remove: Optional[List[str]]
    ///     List of node prefixes to remove during filtering.
    /// node_type_names_to_keep: Optional[List[Optional[List[&str]]]]
    ///     List of node type names to keep during filtering. The node types must match entirely the given node types vector provided.
    /// node_type_names_to_remove: Optional[List[Optional[List[&str]]]]
    ///     List of node type names to remove during filtering. The node types must match entirely the given node types vector provided.
    /// node_type_name_to_keep: Optional[&List[Optional[&str]]]
    ///     List of node type name to keep during filtering. Any of node types must match with one of the node types given.
    /// node_type_name_to_remove: Optional[&List[Optional[&str]]]
    ///     List of node type name to remove during filtering. Any of node types must match with one of the node types given.
    /// source_node_names_to_keep: Optional[List[&str]]
    ///     List of source node names to keep during filtering.
    /// source_node_names_to_remove: Optional[List[&str]]
    ///     List of source node names to remove during filtering.
    /// source_node_names_to_keep_from_graph: Optional[&Graph]
    ///     Graph whose nodes are to be kept when they are source nodes.
    /// source_node_names_to_remove_from_graph: Optional[&Graph]
    ///     Graph whose nodes are to be removed when they are source nodes.
    /// source_node_prefixes_to_keep: Optional[List[str]]
    ///     List of source node prefixes to keep during filtering.
    /// source_node_prefixes_to_remove: Optional[List[str]]
    ///     List of source node prefixes to remove during filtering.
    /// source_node_type_names_to_keep: Optional[List[Optional[List[&str]]]]
    ///     List of node type names of source nodes to keep during filtering. The node types must match entirely the given node types vector provided.
    /// source_node_type_names_to_remove: Optional[List[Optional[List[&str]]]]
    ///     List of node type names of source nodes to remove during filtering. The node types must match entirely the given node types vector provided.
    /// source_node_type_name_to_keep: Optional[&List[Optional[&str]]]
    ///     List of node type name of source nodes to keep during filtering. Any of node types must match with one of the node types given.
    /// source_node_type_name_to_remove: Optional[&List[Optional[&str]]]
    ///     List of node type name of source nodes to remove during filtering. Any of node types must match with one of the node types given.
    /// destination_node_names_to_keep: Optional[List[&str]]
    ///     List of destination node names to keep during filtering.
    /// destination_node_names_to_remove: Optional[List[&str]]
    ///     List of destination node names to remove during filtering.
    /// destination_node_names_to_keep_from_graph: Optional[&Graph]
    ///     Graph whose nodes are to be kept when they are destination nodes.
    /// destination_node_names_to_remove_from_graph: Optional[&Graph]
    ///     Graph whose nodes are to be removed when they are destination nodes.
    /// destination_node_prefixes_to_keep: Optional[List[str]]
    ///     List of destination node prefixes to keep during filtering.
    /// destination_node_prefixes_to_remove: Optional[List[str]]
    ///     List of destination node prefixes to remove during filtering.
    /// destination_node_type_names_to_keep: Optional[List[Optional[List[&str]]]]
    ///     List of node type names of destination nodes to keep during filtering. The node types must match entirely the given node types vector provided.
    /// destination_node_type_names_to_remove: Optional[List[Optional[List[&str]]]]
    ///     List of node type names of destination nodes to remove during filtering. The node types must match entirely the given node types vector provided.
    /// destination_node_type_name_to_keep: Optional[&List[Optional[&str]]]
    ///     List of node type name of destination nodes to keep during filtering. Any of node types must match with one of the node types given.
    /// destination_node_type_name_to_remove: Optional[&List[Optional[&str]]]
    ///     List of node type name of destination nodes to remove during filtering. Any of node types must match with one of the node types given.
    /// edge_node_names_to_keep: Optional[List[Tuple[str, str]]]
    ///     List of tuple of node names to keep during filtering.
    /// edge_node_names_to_remove: Optional[List[Tuple[str, str]]]
    ///     List of tuple of node names to remove during filtering.
    /// edge_type_names_to_keep: Optional[&List[Optional[&str]]]
    ///     List of edge type names to keep during filtering.
    /// edge_type_names_to_remove: Optional[&List[Optional[&str]]]
    ///     List of edge type names to remove during filtering.
    /// min_edge_weight: Optional[float]
    ///     Minimum edge weight. Values lower than this are removed.
    /// max_edge_weight: Optional[float]
    ///     Maximum edge weight. Values higher than this are removed.
    /// min_node_degree: Optional[int]
    ///     Minimum node degree. Values lower than this are removed.
    /// max_node_degree: Optional[int]
    ///     Maximum node degree. Values higher than this are removed.
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
        node_names_to_remove: Option<Vec<&str>>,
        node_names_to_keep_from_graph: Option<&Graph>,
        node_names_to_remove_from_graph: Option<&Graph>,
        node_prefixes_to_keep: Option<Vec<String>>,
        node_prefixes_to_remove: Option<Vec<String>>,
        node_type_names_to_keep: Option<Vec<Option<Vec<&str>>>>,
        node_type_names_to_remove: Option<Vec<Option<Vec<&str>>>>,
        node_type_name_to_keep: Option<Vec<Option<&str>>>,
        node_type_name_to_remove: Option<Vec<Option<&str>>>,
        source_node_names_to_keep: Option<Vec<&str>>,
        source_node_names_to_remove: Option<Vec<&str>>,
        source_node_names_to_keep_from_graph: Option<&Graph>,
        source_node_names_to_remove_from_graph: Option<&Graph>,
        source_node_prefixes_to_keep: Option<Vec<String>>,
        source_node_prefixes_to_remove: Option<Vec<String>>,
        source_node_type_names_to_keep: Option<Vec<Option<Vec<&str>>>>,
        source_node_type_names_to_remove: Option<Vec<Option<Vec<&str>>>>,
        source_node_type_name_to_keep: Option<Vec<Option<&str>>>,
        source_node_type_name_to_remove: Option<Vec<Option<&str>>>,
        destination_node_names_to_keep: Option<Vec<&str>>,
        destination_node_names_to_remove: Option<Vec<&str>>,
        destination_node_names_to_keep_from_graph: Option<&Graph>,
        destination_node_names_to_remove_from_graph: Option<&Graph>,
        destination_node_prefixes_to_keep: Option<Vec<String>>,
        destination_node_prefixes_to_remove: Option<Vec<String>>,
        destination_node_type_names_to_keep: Option<Vec<Option<Vec<&str>>>>,
        destination_node_type_names_to_remove: Option<Vec<Option<Vec<&str>>>>,
        destination_node_type_name_to_keep: Option<Vec<Option<&str>>>,
        destination_node_type_name_to_remove: Option<Vec<Option<&str>>>,
        edge_node_names_to_keep: Option<Vec<(&str, &str)>>,
        edge_node_names_to_remove: Option<Vec<(&str, &str)>>,
        edge_type_names_to_keep: Option<Vec<Option<&str>>>,
        edge_type_names_to_remove: Option<Vec<Option<&str>>>,
        min_edge_weight: Option<WeightT>,
        max_edge_weight: Option<WeightT>,
        min_node_degree: Option<NodeT>,
        max_node_degree: Option<NodeT>,
        filter_singleton_nodes: Option<bool>,
        filter_singleton_nodes_with_selfloop: Option<bool>,
        filter_selfloops: Option<bool>,
        filter_parallel_edges: Option<bool>,
    ) -> PyResult<Graph> {
        Ok(pe!(self.inner.filter_from_names(
            node_names_to_keep,
            node_names_to_remove,
            node_names_to_keep_from_graph.map(|sg| &sg.inner),
            node_names_to_remove_from_graph.map(|sg| &sg.inner),
            node_prefixes_to_keep,
            node_prefixes_to_remove,
            node_type_names_to_keep,
            node_type_names_to_remove,
            node_type_name_to_keep.as_ref().map(|x| x.as_slice()),
            node_type_name_to_remove.as_ref().map(|x| x.as_slice()),
            source_node_names_to_keep,
            source_node_names_to_remove,
            source_node_names_to_keep_from_graph.map(|sg| &sg.inner),
            source_node_names_to_remove_from_graph.map(|sg| &sg.inner),
            source_node_prefixes_to_keep,
            source_node_prefixes_to_remove,
            source_node_type_names_to_keep,
            source_node_type_names_to_remove,
            source_node_type_name_to_keep.as_ref().map(|x| x.as_slice()),
            source_node_type_name_to_remove
                .as_ref()
                .map(|x| x.as_slice()),
            destination_node_names_to_keep,
            destination_node_names_to_remove,
            destination_node_names_to_keep_from_graph.map(|sg| &sg.inner),
            destination_node_names_to_remove_from_graph.map(|sg| &sg.inner),
            destination_node_prefixes_to_keep,
            destination_node_prefixes_to_remove,
            destination_node_type_names_to_keep,
            destination_node_type_names_to_remove,
            destination_node_type_name_to_keep
                .as_ref()
                .map(|x| x.as_slice()),
            destination_node_type_name_to_remove
                .as_ref()
                .map(|x| x.as_slice()),
            edge_node_names_to_keep,
            edge_node_names_to_remove,
            edge_type_names_to_keep.as_ref().map(|x| x.as_slice()),
            edge_type_names_to_remove.as_ref().map(|x| x.as_slice()),
            min_edge_weight,
            max_edge_weight,
            min_node_degree,
            max_node_degree,
            filter_singleton_nodes,
            filter_singleton_nodes_with_selfloop,
            filter_selfloops,
            filter_parallel_edges
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns new graph without unknown node types and relative nodes.
    ///
    /// Note that this method will remove ALL nodes labeled with unknown node
    /// type!
    pub fn remove_unknown_node_types(&self) -> Graph {
        self.inner.remove_unknown_node_types().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns new graph without unknown edge types and relative edges.
    ///
    /// Note that this method will remove ALL edges labeled with unknown edge
    /// type!
    pub fn remove_unknown_edge_types(&self) -> Graph {
        self.inner.remove_unknown_edge_types().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns new graph without singleton nodes.
    ///
    /// A node is singleton when does not have neither incoming or outgoing edges.
    pub fn remove_singleton_nodes(&self) -> Graph {
        self.inner.remove_singleton_nodes().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns new graph without tendrils
    pub fn remove_tendrils(&self) -> PyResult<Graph> {
        Ok(pe!(self.inner.remove_tendrils())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns new graph without tendrils
    pub fn remove_dendritic_trees(&self) -> PyResult<Graph> {
        Ok(pe!(self.inner.remove_dendritic_trees())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, minimum_node_degree)")]
    /// Returns new graph without isomorphic nodes, only keeping the smallest node ID of each group.
    ///
    /// Parameters
    /// ----------
    /// minimum_node_degree: Optional[int]
    ///     Minimum node degree for the topological synonims. By default equal to 5.
    ///
    pub fn remove_isomorphic_nodes(&self, minimum_node_degree: Option<NodeT>) -> Graph {
        self.inner
            .remove_isomorphic_nodes(minimum_node_degree)
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns new graph without singleton nodes with selfloops.
    ///
    /// A node is singleton with selfloop when does not have neither incoming or outgoing edges.
    pub fn remove_singleton_nodes_with_selfloops(&self) -> Graph {
        self.inner.remove_singleton_nodes_with_selfloops().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns new graph without disconnected nodes.
    ///
    /// A disconnected node is a node with no connection to any other node.
    pub fn remove_disconnected_nodes(&self) -> Graph {
        self.inner.remove_disconnected_nodes().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns new graph without selfloops.
    pub fn remove_selfloops(&self) -> Graph {
        self.inner.remove_selfloops().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns new graph without parallel edges
    pub fn remove_parallel_edges(&self) -> Graph {
        self.inner.remove_parallel_edges().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, random_state, undesired_edge_types, verbose)")]
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
        let (subresult_0, subresult_1, subresult_2, subresult_3, subresult_4) = self
            .inner
            .random_spanning_arborescence_kruskal(random_state, undesired_edge_types, verbose);
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
    #[pyo3(text_signature = "($self, verbose)")]
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
            self.inner.spanning_arborescence_kruskal(verbose);
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
    #[pyo3(text_signature = "($self, verbose)")]
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
                pe!(self.inner.get_connected_components(verbose))?.into();
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
    #[pyo3(
        text_signature = "($self, vector_sources, vector_destinations, vector_cumulative_node_degrees, vector_reciprocal_sqrt_degrees)"
    )]
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
            vector_sources,
            vector_destinations,
            vector_cumulative_node_degrees,
            vector_reciprocal_sqrt_degrees
        ))?)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Disable all extra perks, reducing memory impact but incresing time requirements
    pub fn disable_all(&mut self) {
        self.inner.disable_all();
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, other)")]
    /// Returns whether the graphs share the same nodes.
    ///
    /// Parameters
    /// ----------
    /// other: Graph
    ///     The other graph.
    ///
    pub fn has_compatible_node_vocabularies(&self, other: &Graph) -> bool {
        self.inner
            .has_compatible_node_vocabularies(&other.inner)
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, other)")]
    /// Returns whether the graphs share the same node types or absence thereof.
    ///
    /// Parameters
    /// ----------
    /// other: Graph
    ///     The other graph.
    ///
    pub fn has_compatible_node_types_vocabularies(&self, other: &Graph) -> bool {
        self.inner
            .has_compatible_node_types_vocabularies(&other.inner)
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, other)")]
    /// Returns whether the graphs share the same edge types or absence thereof.
    ///
    /// Parameters
    /// ----------
    /// other: Graph
    ///     The other graph.
    ///
    pub fn has_compatible_edge_types_vocabularies(&self, other: &Graph) -> bool {
        self.inner
            .has_compatible_edge_types_vocabularies(&other.inner)
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, other)")]
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
    #[pyo3(text_signature = "($self, other)")]
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
    #[pyo3(text_signature = "($self)")]
    /// Returns 2-approximated verted cover set using greedy algorithm.
    pub fn approximated_vertex_cover_set(&self) -> HashSet<NodeT> {
        self.inner.approximated_vertex_cover_set().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, random_state)")]
    /// Return random node type ID.
    ///
    /// Parameters
    /// ----------
    /// random_state: int
    ///     The random state to use to reproduce the sampling.
    ///
    pub fn get_random_node_type(&self, random_state: u64) -> PyResult<NodeTypeT> {
        Ok(pe!(self.inner.get_random_node_type(random_state.clone()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, random_state)")]
    /// Return random edge type ID.
    ///
    /// Parameters
    /// ----------
    /// random_state: int
    ///     The random state to use to reproduce the sampling.
    ///
    pub fn get_random_edge_type(&self, random_state: u64) -> PyResult<EdgeTypeT> {
        Ok(pe!(self.inner.get_random_edge_type(random_state.clone()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, random_state)")]
    /// Return random scale_free edge type ID.
    ///
    /// Parameters
    /// ----------
    /// random_state: int
    ///     The random state to use to reproduce the sampling.
    ///
    ///
    /// Safety
    /// ------
    /// * If the graph does not have edge types, the method will always return None.
    pub unsafe fn get_unchecked_random_scale_free_edge_type(
        &self,
        random_state: u64,
    ) -> Option<EdgeTypeT> {
        self.inner
            .get_unchecked_random_scale_free_edge_type(random_state.clone())
            .map(|x| x.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, random_state)")]
    /// Return random scale_free edge type ID.
    ///
    /// Parameters
    /// ----------
    /// random_state: int
    ///     The random state to use to reproduce the sampling.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have edge types.
    ///
    pub fn get_random_scale_free_edge_type(
        &self,
        random_state: u64,
    ) -> PyResult<Option<EdgeTypeT>> {
        Ok(pe!(self
            .inner
            .get_random_scale_free_edge_type(random_state.clone()))?
        .map(|x| x.into()))
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, random_state)")]
    /// Return random node ID.
    ///
    /// Parameters
    /// ----------
    /// random_state: int
    ///     The random state to use to reproduce the sampling.
    ///
    pub fn get_random_node(&self, random_state: u64) -> NodeT {
        self.inner.get_random_node(random_state.clone()).into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, random_state)")]
    /// Return random edge ID.
    ///
    /// Parameters
    /// ----------
    /// random_state: int
    ///     The random state to use to reproduce the sampling.
    ///
    pub fn get_random_edge_id(&self, random_state: u64) -> EdgeT {
        self.inner.get_random_edge_id(random_state.clone()).into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, random_state)")]
    /// Return random node ID following outbounds degree distribution of the graph.
    ///
    /// Parameters
    /// ----------
    /// random_state: int
    ///     The random state to use to reproduce the sampling.
    ///
    pub fn get_random_outbounds_scale_free_node(&self, random_state: u64) -> NodeT {
        self.inner
            .get_random_outbounds_scale_free_node(random_state.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, random_state)")]
    /// Return random node ID following inbounds degree distribution of the graph.
    ///
    /// Parameters
    /// ----------
    /// random_state: int
    ///     The random state to use to reproduce the sampling.
    ///
    pub fn get_random_inbounds_scale_free_node(&self, random_state: u64) -> NodeT {
        self.inner
            .get_random_inbounds_scale_free_node(random_state.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, number_of_nodes_to_sample, random_state)")]
    /// Return random unique sorted numbers.
    ///
    /// Parameters
    /// ----------
    /// number_of_nodes_to_sample: int
    ///     The number of nodes to sample.
    /// random_state: int
    ///     The random state to use to reproduce the sampling.
    ///
    pub fn get_sorted_unique_random_nodes(
        &self,
        number_of_nodes_to_sample: NodeT,
        random_state: u64,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_sorted_unique_random_nodes(
                    number_of_nodes_to_sample.clone(),
                    random_state.clone()
                ))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, number_of_nodes_to_sample, root_node)")]
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
                    number_of_nodes_to_sample.clone(),
                    root_node.clone()
                ))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node, random_state, walk_length, unique)")]
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
                    node.clone(),
                    random_state.clone(),
                    walk_length.clone(),
                    unique
                ))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return list of the supported node sampling methods
    pub fn get_node_sampling_methods(&self) -> Vec<String> {
        self.inner
            .get_node_sampling_methods()
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, number_of_nodes_to_sample, random_state, node_sampling_method, root_node, unique)"
    )]
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
        node_sampling_method: &str,
        root_node: Option<NodeT>,
        unique: Option<bool>,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_subsampled_nodes(
                    number_of_nodes_to_sample.clone(),
                    random_state.clone(),
                    node_sampling_method,
                    root_node,
                    unique
                ))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, features, iterations, maximal_distance, k1, b, include_central_node, verbose)"
    )]
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
                    features,
                    iterations,
                    maximal_distance,
                    k1,
                    b,
                    include_central_node,
                    verbose
                ))?,
                f64
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, iterations, maximal_distance, k1, b, verbose)")]
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
                    iterations,
                    maximal_distance,
                    k1,
                    b,
                    verbose
                ))?,
                f64
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return if graph has name that is not the default one.
    ///
    /// TODO: use a default for the default graph name
    pub fn has_default_graph_name(&self) -> bool {
        self.inner.has_default_graph_name().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return if the graph has any nodes.
    pub fn has_nodes(&self) -> bool {
        self.inner.has_nodes().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return if the graph has any edges.
    pub fn has_edges(&self) -> bool {
        self.inner.has_edges().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return whether the graph has trap nodes.
    pub fn has_trap_nodes(&self) -> bool {
        self.inner.has_trap_nodes().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns boolean representing if graph is directed.
    pub fn is_directed(&self) -> bool {
        self.inner.is_directed().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns whether graph is a directed acyclic graph.
    pub fn is_directed_acyclic(&self) -> bool {
        self.inner.is_directed_acyclic().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns boolean representing whether graph has weights.
    pub fn has_edge_weights(&self) -> bool {
        self.inner.has_edge_weights().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns whether graph has weights that can represent probabilities
    pub fn has_edge_weights_representing_probabilities(&self) -> PyResult<bool> {
        Ok(pe!(self.inner.has_edge_weights_representing_probabilities())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
    /// Returns boolean representing whether graph has edge types.
    pub fn has_edge_types(&self) -> bool {
        self.inner.has_edge_types().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns boolean representing if graph has self-loops.
    pub fn has_selfloops(&self) -> bool {
        self.inner.has_selfloops().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns boolean representing if nodes which are nor singletons nor
    /// singletons with selfloops.
    pub fn has_disconnected_nodes(&self) -> bool {
        self.inner.has_disconnected_nodes().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns boolean representing if graph has singletons.
    pub fn has_singleton_nodes(&self) -> bool {
        self.inner.has_singleton_nodes().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns boolean representing if graph has singletons
    pub fn has_singleton_nodes_with_selfloops(&self) -> bool {
        self.inner.has_singleton_nodes_with_selfloops().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, verbose)")]
    /// Returns whether the graph is connected.
    ///
    /// Parameters
    /// ----------
    /// verbose: Optional[bool]
    ///     Whether to show the loading bar while computing the connected components, if necessary.
    ///
    pub fn is_connected(&self, verbose: Option<bool>) -> bool {
        self.inner.is_connected(verbose).into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns boolean representing if graph has node types
    pub fn has_node_types(&self) -> bool {
        self.inner.has_node_types().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
    /// Returns whether the nodes have exclusively homogenous node types.
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph does not have node types.
    ///
    pub fn has_exclusively_homogeneous_node_types(&self) -> PyResult<bool> {
        Ok(pe!(self.inner.has_exclusively_homogeneous_node_types())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns whether the nodes have an homogenous node ontology
    pub fn has_homogeneous_node_ontologies(&self) -> PyResult<bool> {
        Ok(pe!(self.inner.has_homogeneous_node_ontologies())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
    /// Return whether the graph has any known node-related graph oddities
    pub fn has_node_oddities(&self) -> bool {
        self.inner.has_node_oddities().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self)")]
    /// Return if there are multiple edges between two node
    pub fn is_multigraph(&self) -> bool {
        self.inner.is_multigraph().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return whether at least a node has a known ontology
    pub fn has_node_ontologies(&self) -> bool {
        self.inner.has_node_ontologies().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return whether at least a node has an unknown ontology
    pub fn has_unknown_node_ontologies(&self) -> bool {
        self.inner.has_unknown_node_ontologies().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns whether the node IDs are sorted by decreasing outbound node degree.
    pub fn has_nodes_sorted_by_decreasing_outbound_node_degree(&self) -> bool {
        self.inner
            .has_nodes_sorted_by_decreasing_outbound_node_degree()
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns whether the node IDs are sorted by decreasing outbound node degree.
    pub fn has_nodes_sorted_by_lexicographic_order(&self) -> bool {
        self.inner.has_nodes_sorted_by_lexicographic_order().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns whether the graph contains the indentity matrix.
    pub fn contains_identity_matrix(&self) -> bool {
        self.inner.contains_identity_matrix().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns whether the node IDs are sorted by increasing outbound node degree.
    pub fn has_nodes_sorted_by_increasing_outbound_node_degree(&self) -> bool {
        self.inner
            .has_nodes_sorted_by_increasing_outbound_node_degree()
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns whether the destinations time-memory tradeoff is enabled
    pub fn has_destinations_tradeoff_enabled(&self) -> bool {
        self.inner.has_destinations_tradeoff_enabled().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns whether the sources time-memory tradeoff is enabled
    pub fn has_sources_tradeoff_enabled(&self) -> bool {
        self.inner.has_sources_tradeoff_enabled().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns whether the cumulative_node_degrees time-memory tradeoff is enabled
    pub fn has_cumulative_node_degrees_tradeoff_enabled(&self) -> bool {
        self.inner
            .has_cumulative_node_degrees_tradeoff_enabled()
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns whether the reciprocal_sqrt_degrees time-memory tradeoff is enabled
    pub fn has_reciprocal_sqrt_degrees_tradeoff_enabled(&self) -> bool {
        self.inner
            .has_reciprocal_sqrt_degrees_tradeoff_enabled()
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns vector of detected dentritic trees
    pub fn get_dendritic_trees(&self) -> PyResult<Vec<DendriticTree>> {
        Ok(pe!(self.inner.get_dendritic_trees())?
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, iterations, verbose)")]
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
            .get_transitive_closure(iterations, verbose)
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, iterations, verbose)")]
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
            .get_all_shortest_paths(iterations, verbose)
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, iterations, use_edge_weights_as_probabilities, verbose)")]
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
            iterations,
            use_edge_weights_as_probabilities,
            verbose
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_id)")]
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
            .get_unchecked_edge_weight_from_edge_id(edge_id.clone())
            .map(|x| x.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src, dst)")]
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
            .get_unchecked_edge_weight_from_node_ids(src.clone(), dst.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_name)")]
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
            .get_unchecked_node_id_from_node_name(node_name)
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type_name)")]
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
            .get_unchecked_edge_type_id_from_edge_type_name(edge_type_name)
            .map(|x| x.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type_id)")]
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
            .get_unchecked_edge_type_name_from_edge_type_id(edge_type_id)
            .map(|x| x.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type)")]
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
            .get_unchecked_edge_count_from_edge_type_id(edge_type)
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type)")]
    /// Return number of nodes of the given node type without checks.
    ///
    /// Parameters
    /// ----------
    ///
    ///
    /// Safety
    /// ------
    /// If the provided value is not within the graph's vocabulary
    ///  the method will panic.
    pub unsafe fn get_unchecked_node_count_from_node_type_id(
        &self,
        node_type: Option<NodeTypeT>,
    ) -> NodeT {
        self.inner
            .get_unchecked_node_count_from_node_type_id(node_type)
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src, dst, edge_type)")]
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
                src.clone(),
                dst.clone(),
                edge_type,
            )
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src, dst)")]
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
            .get_unchecked_minmax_edge_ids_from_node_ids(src.clone(), dst.clone());
        (subresult_0.into(), subresult_1.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_id)")]
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
            .get_unchecked_node_ids_from_edge_id(edge_id.clone());
        (subresult_0.into(), subresult_1.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_id)")]
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
            .get_unchecked_node_names_from_edge_id(edge_id.clone());
        (subresult_0.into(), subresult_1.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_id)")]
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
            .get_unchecked_source_node_id_from_edge_id(edge_id.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_id)")]
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
            .get_unchecked_destination_node_id_from_edge_id(edge_id.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_id)")]
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
        Ok(pe!(self.inner.get_source_node_id_from_edge_id(edge_id.clone()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_id)")]
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
            .get_destination_node_id_from_edge_id(edge_id.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_id)")]
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
            .get_unchecked_source_node_name_from_edge_id(edge_id.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_id)")]
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
            .get_unchecked_destination_node_name_from_edge_id(edge_id.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_id)")]
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
        Ok(pe!(self
            .inner
            .get_source_node_name_from_edge_id(edge_id.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_id)")]
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
            .get_destination_node_name_from_edge_id(edge_id.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_id)")]
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
                pe!(self.inner.get_node_names_from_edge_id(edge_id.clone()))?.into();
            (subresult_0.into(), subresult_1.into())
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_id)")]
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
                pe!(self.inner.get_node_ids_from_edge_id(edge_id.clone()))?.into();
            (subresult_0.into(), subresult_1.into())
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src, dst)")]
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
            .get_unchecked_edge_id_from_node_ids(src.clone(), dst.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src, dst)")]
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
        Ok(pe!(self
            .inner
            .get_edge_id_from_node_ids(src.clone(), dst.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, source_id)")]
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
            .get_unchecked_unique_source_node_id(source_id.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_id)")]
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
            .get_unchecked_node_ids_and_edge_type_id_from_edge_id(edge_id.clone());
        (
            subresult_0.into(),
            subresult_1.into(),
            subresult_2.map(|x| x.into()),
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_id)")]
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
                .get_node_ids_and_edge_type_id_from_edge_id(edge_id.clone()))?
            .into();
            (
                subresult_0.into(),
                subresult_1.into(),
                subresult_2.map(|x| x.into()),
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_id)")]
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
            .get_unchecked_node_ids_and_edge_type_id_and_edge_weight_from_edge_id(edge_id.clone());
        (
            subresult_0.into(),
            subresult_1.into(),
            subresult_2.map(|x| x.into()),
            subresult_3.map(|x| x.into()),
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_id)")]
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
                .get_node_ids_and_edge_type_id_and_edge_weight_from_edge_id(edge_id.clone()))?
            .into();
            (
                subresult_0.into(),
                subresult_1.into(),
                subresult_2.map(|x| x.into()),
                subresult_3.map(|x| x.into()),
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, k)")]
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
                pe!(self.inner.get_top_k_central_node_ids(k.clone()))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, k)")]
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
                pe!(self.inner.get_weighted_top_k_central_node_ids(k.clone()))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
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
            .get_unchecked_node_degree_from_node_id(node_id.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
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
            .get_unchecked_weighted_node_degree_from_node_id(node_id.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    /// Returns the number of outbound neighbours of given node ID.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     Integer ID of the node.
    ///
    pub fn get_node_degree_from_node_id(&self, node_id: NodeT) -> PyResult<NodeT> {
        Ok(pe!(self.inner.get_node_degree_from_node_id(node_id.clone()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
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
            .get_unchecked_comulative_node_degree_from_node_id(node_id.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
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
            .get_comulative_node_degree_from_node_id(node_id.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
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
            .get_unchecked_reciprocal_sqrt_degree_from_node_id(node_id.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
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
            .get_reciprocal_sqrt_degree_from_node_id(node_id.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_ids)")]
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
    #[pyo3(text_signature = "($self, node_id)")]
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
            .get_weighted_node_degree_from_node_id(node_id.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_name)")]
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
        Ok(pe!(self.inner.get_node_degree_from_node_name(node_name))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, k)")]
    /// Return vector with top k central node names.
    ///
    /// Parameters
    /// ----------
    /// k: int
    ///     Number of central nodes to extract.
    ///
    pub fn get_top_k_central_node_names(&self, k: NodeT) -> PyResult<Vec<String>> {
        Ok(pe!(self.inner.get_top_k_central_node_names(k.clone()))?
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
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
            .get_unchecked_node_type_ids_from_node_id(node_id.clone())
            .map(|x| x.into_iter().cloned().map(|x| x.into()).collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
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
        Ok(
            pe!(self.inner.get_node_type_ids_from_node_id(node_id.clone()))?
                .map(|x| x.into_iter().cloned().map(|x| x.into()).collect::<Vec<_>>()),
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_id)")]
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
            .get_unchecked_edge_type_id_from_edge_id(edge_id.clone())
            .map(|x| x.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_id)")]
    /// Returns edge type of given edge.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int
    ///     edge whose edge type is to be returned.
    ///
    pub fn get_edge_type_id_from_edge_id(&self, edge_id: EdgeT) -> PyResult<Option<EdgeTypeT>> {
        Ok(pe!(self.inner.get_edge_type_id_from_edge_id(edge_id.clone()))?.map(|x| x.into()))
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src, dst)")]
    /// Returns edge type from given edge node IDs.
    ///
    /// Parameters
    /// ----------
    /// src: int
    ///     Source node ID of the node of interest.
    /// dst: int
    ///     Destination node ID of the node of interest.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the provided nodes do not form an edge.
    ///
    pub fn get_edge_type_id_from_edge_node_ids(
        &self,
        src: NodeT,
        dst: NodeT,
    ) -> PyResult<Option<EdgeTypeT>> {
        Ok(pe!(self
            .inner
            .get_edge_type_id_from_edge_node_ids(src.clone(), dst.clone()))?
        .map(|x| x.into()))
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
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
            .get_unchecked_node_type_names_from_node_id(node_id.clone())
            .map(|x| x.into_iter().map(|x| x.into()).collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
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
        Ok(
            pe!(self.inner.get_node_type_names_from_node_id(node_id.clone()))?
                .map(|x| x.into_iter().map(|x| x.into()).collect::<Vec<_>>()),
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_name)")]
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
        Ok(
            pe!(self.inner.get_node_type_names_from_node_name(node_name))?
                .map(|x| x.into_iter().map(|x| x.into()).collect::<Vec<_>>()),
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_id)")]
    /// Returns option with the edge type of the given edge id.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int
    ///     The edge ID whose edge type is to be returned.
    ///
    pub fn get_edge_type_name_from_edge_id(&self, edge_id: EdgeT) -> PyResult<Option<String>> {
        Ok(pe!(self.inner.get_edge_type_name_from_edge_id(edge_id.clone()))?.map(|x| x.into()))
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type_id)")]
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
            .get_edge_type_name_from_edge_type_id(edge_type_id.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_id)")]
    /// Returns weight of the given edge id.
    ///
    /// Parameters
    /// ----------
    /// edge_id: int
    ///     The edge ID whose weight is to be returned.
    ///
    pub fn get_edge_weight_from_edge_id(&self, edge_id: EdgeT) -> PyResult<WeightT> {
        Ok(pe!(self.inner.get_edge_weight_from_edge_id(edge_id.clone()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src, dst)")]
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
            .get_edge_weight_from_node_ids(src.clone(), dst.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src, dst, edge_type)")]
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
                src.clone(),
                dst.clone(),
                edge_type
            ))?
            .into(),
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src, dst, edge_type)")]
    /// Returns weight of the given node names and edge type.
    ///
    /// Parameters
    /// ----------
    /// src: str
    ///     The node name of the source node.
    /// dst: str
    ///     The node name of the destination node.
    /// edge_type: Optional[&str]
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
            .get_edge_weight_from_node_names_and_edge_type_name(src, dst, edge_type))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src_name, dst_name)")]
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
            .get_edge_weight_from_node_names(src_name, dst_name))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
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
            .get_unchecked_node_name_from_node_id(node_id.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    /// Returns result with the node name.
    ///
    /// Parameters
    /// ----------
    /// node_id: int
    ///     The node ID whose name is to be returned.
    ///
    pub fn get_node_name_from_node_id(&self, node_id: NodeT) -> PyResult<String> {
        Ok(pe!(self.inner.get_node_name_from_node_id(node_id.clone()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_name)")]
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
        Ok(pe!(self.inner.get_node_id_from_node_name(node_name))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_names)")]
    /// Returns result with the node IDs.
    ///
    /// Parameters
    /// ----------
    /// node_names: List[&str]
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
                pe!(self.inner.get_node_ids_from_node_names(node_names))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_ids)")]
    /// Returns result with the node names.
    ///
    /// Parameters
    /// ----------
    /// node_ids: List[int]
    ///     The node ids whose node names are to be returned.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     When any of the given node ids does not exists in the current graph.
    ///
    pub fn get_node_names_from_node_ids(&self, node_ids: Vec<NodeT>) -> PyResult<Vec<String>> {
        Ok(pe!(self.inner.get_node_names_from_node_ids(node_ids))?
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_node_names)")]
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
    ) -> PyResult<Py<PyArray2<NodeT>>> {
        Ok({
            // Warning: this copies the array so it uses double the memory.
            // To avoid this you should directly generate data compatible with a numpy array
            // Which is a flat vector with row-first or column-first unrolling
            let gil = pyo3::Python::acquire_gil();
            let body = pe!(self
                .inner
                .get_edge_node_ids_from_edge_node_names(edge_node_names))?;
            let result_array = ThreadDataRaceAware {
                t: unsafe { PyArray2::<NodeT>::new(gil.python(), [body.len(), 2], false) },
            };
            body.into_par_iter()
                .enumerate()
                .for_each(|(i, (a, b))| unsafe {
                    *(result_array.t.uget_mut([i, 0])) = a;
                    *(result_array.t.uget_mut([i, 1])) = b;
                });
            result_array.t.to_owned()
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_node_ids)")]
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
            .get_edge_node_names_from_edge_node_ids(edge_node_ids))?
        .into_iter()
        .map(|x| {
            let (subresult_0, subresult_1) = x;
            (subresult_0.into(), subresult_1.into())
        })
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_name)")]
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
        Ok(pe!(self.inner.get_node_type_ids_from_node_name(node_name))?
            .map(|x| x.into_iter().cloned().map(|x| x.into()).collect::<Vec<_>>()))
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_name)")]
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
        Ok(
            pe!(self.inner.get_node_type_name_from_node_name(node_name))?
                .map(|x| x.into_iter().map(|x| x.into()).collect::<Vec<_>>()),
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type_id)")]
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
        Ok(pe!(self.inner.get_edge_count_from_edge_type_id(edge_type_id))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type_name)")]
    /// Return edge type ID curresponding to given edge type name.
    ///
    /// If None is given as an edge type ID, None is returned.
    ///
    /// Parameters
    /// ----------
    /// edge_type_name: Optional[&str]
    ///     The edge type name whose ID is to be returned.
    ///
    pub fn get_edge_type_id_from_edge_type_name(
        &self,
        edge_type_name: Option<&str>,
    ) -> PyResult<Option<EdgeTypeT>> {
        Ok(pe!(self
            .inner
            .get_edge_type_id_from_edge_type_name(edge_type_name))?
        .map(|x| x.into()))
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type_name)")]
    /// Return number of edges with given edge type name.
    ///
    /// If None is given as an edge type name, the unknown edge types
    /// will be returned.
    ///
    /// Parameters
    /// ----------
    /// edge_type_name: Optional[&str]
    ///     The edge type name to count the edges of.
    ///
    pub fn get_edge_count_from_edge_type_name(
        &self,
        edge_type_name: Option<&str>,
    ) -> PyResult<EdgeT> {
        Ok(pe!(self
            .inner
            .get_edge_count_from_edge_type_name(edge_type_name))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_name)")]
    /// Return node type ID curresponding to given node type name.
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
            .get_node_type_id_from_node_type_name(node_type_name))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_id)")]
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
        Ok(pe!(self.inner.get_node_count_from_node_type_id(node_type_id))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_name)")]
    /// Return number of nodes with given node type name.
    ///
    /// If None is given as an node type name, the unknown node types
    /// will be returned.
    ///
    /// Parameters
    /// ----------
    /// node_type_name: Optional[&str]
    ///     The node type name to count the nodes of.
    ///
    pub fn get_node_count_from_node_type_name(
        &self,
        node_type_name: Option<&str>,
    ) -> PyResult<NodeT> {
        Ok(pe!(self
            .inner
            .get_node_count_from_node_type_name(node_type_name))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
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
                    .get_neighbour_node_ids_from_node_id(node_id.clone()))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_name)")]
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
                pe!(self.inner.get_neighbour_node_ids_from_node_name(node_name))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_name)")]
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
            .get_neighbour_node_names_from_node_name(node_name))?
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src, dst)")]
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
                .get_minmax_edge_ids_from_node_ids(src.clone(), dst.clone()))?
            .into();
            (subresult_0.into(), subresult_1.into())
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src, dst, edge_type)")]
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
            src.clone(),
            dst.clone(),
            edge_type
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src_name, dst_name)")]
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
        Ok(pe!(self.inner.get_edge_id_from_node_names(src_name, dst_name))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src_name, dst_name, edge_type_name)")]
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
    /// edge_type_name: Optional[&str]
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
                src_name,
                dst_name,
                edge_type_name
            ))?
            .into(),
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type_names)")]
    /// Return translated edge types from string to internal edge ID.
    ///
    /// Parameters
    /// ----------
    /// edge_type_names: List[Optional[&str]]
    ///     Vector of edge types to be converted.
    ///
    pub fn get_edge_type_ids_from_edge_type_names(
        &self,
        edge_type_names: Vec<Option<&str>>,
    ) -> PyResult<Vec<Option<EdgeTypeT>>> {
        Ok(pe!(self
            .inner
            .get_edge_type_ids_from_edge_type_names(&edge_type_names))?
        .into_iter()
        .map(|x| x.map(|x| x.into()))
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_names)")]
    /// Return translated node types from string to internal node ID.
    ///
    /// Parameters
    /// ----------
    /// node_type_names: List[Optional[&str]]
    ///     Vector of node types to be converted.
    ///
    pub fn get_node_type_ids_from_node_type_names(
        &self,
        node_type_names: Vec<Option<&str>>,
    ) -> PyResult<Vec<Option<NodeTypeT>>> {
        Ok(pe!(self
            .inner
            .get_node_type_ids_from_node_type_names(&node_type_names))?
        .into_iter()
        .map(|x| x.map(|x| x.into()))
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_names)")]
    /// Return translated node types from string to internal node ID.
    ///
    /// Parameters
    /// ----------
    /// node_type_names: List[Optional[List[&str]]]
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
    ) -> PyResult<Vec<Option<Py<PyArray1<NodeTypeT>>>>> {
        Ok(pe!(self
            .inner
            .get_multiple_node_type_ids_from_node_type_names(node_type_names))?
        .into_iter()
        .map(|x| {
            x.map(|x| {
                let gil = pyo3::Python::acquire_gil();
                to_ndarray_1d!(gil, x, NodeTypeT)
            })
        })
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src)")]
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
            .get_unchecked_minmax_edge_ids_from_source_node_id(src.clone());
        (subresult_0.into(), subresult_1.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src)")]
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
                .get_minmax_edge_ids_from_source_node_id(src.clone()))?
            .into();
            (subresult_0.into(), subresult_1.into())
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_id)")]
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
            .get_node_type_name_from_node_type_id(node_type_id.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_ids)")]
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
            .get_unchecked_node_type_names_from_node_type_ids(&node_type_ids)
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_id)")]
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
            .get_unchecked_number_of_nodes_from_node_type_id(node_type_id.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_id)")]
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
            .get_number_of_nodes_from_node_type_id(node_type_id.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_type_name)")]
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
            .get_number_of_nodes_from_node_type_name(node_type_name))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type_id)")]
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
            .get_unchecked_number_of_edges_from_edge_type_id(edge_type_id.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type_id)")]
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
            .get_number_of_edges_from_edge_type_id(edge_type_id.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type_name)")]
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
            .get_number_of_edges_from_edge_type_name(edge_type_name))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_ids)")]
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
    #[pyo3(text_signature = "($self, node_ids)")]
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
    #[pyo3(text_signature = "($self, directed, edge_type_id)")]
    /// Returns vector containing edge node IDs with given edge type.
    ///
    /// Parameters
    /// ----------
    /// edge_type_id: Optional[int]
    ///     Edge type ID to extract.
    /// directed: bool
    ///     Whether to iterate the edge list as directed or undirected.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    /// ValueError
    ///     If the given edge type ID does not exist in the graph.
    ///
    pub fn get_edge_node_ids_from_edge_type_id(
        &self,
        directed: bool,
        edge_type_id: Option<EdgeTypeT>,
    ) -> PyResult<Py<PyArray2<NodeT>>> {
        Ok({
            // Warning: this copies the array so it uses double the memory.
            // To avoid this you should directly generate data compatible with a numpy array
            // Which is a flat vector with row-first or column-first unrolling
            let gil = pyo3::Python::acquire_gil();
            let body = pe!(self
                .inner
                .get_edge_node_ids_from_edge_type_id(directed.clone(), edge_type_id))?;
            let result_array = ThreadDataRaceAware {
                t: unsafe { PyArray2::<NodeT>::new(gil.python(), [body.len(), 2], false) },
            };
            body.into_par_iter()
                .enumerate()
                .for_each(|(i, (a, b))| unsafe {
                    *(result_array.t.uget_mut([i, 0])) = a;
                    *(result_array.t.uget_mut([i, 1])) = b;
                });
            result_array.t.to_owned()
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type_id)")]
    /// Returns vector containing directed edge node IDs with given edge type.
    ///
    /// Parameters
    /// ----------
    /// edge_type_id: Optional[int]
    ///     Edge type ID to extract.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    /// ValueError
    ///     If the given edge type ID does not exist in the graph.
    ///
    pub fn get_directed_edge_node_ids_from_edge_type_id(
        &self,
        edge_type_id: Option<EdgeTypeT>,
    ) -> PyResult<Py<PyArray2<NodeT>>> {
        Ok({
            // Warning: this copies the array so it uses double the memory.
            // To avoid this you should directly generate data compatible with a numpy array
            // Which is a flat vector with row-first or column-first unrolling
            let gil = pyo3::Python::acquire_gil();
            let body = pe!(self
                .inner
                .get_directed_edge_node_ids_from_edge_type_id(edge_type_id))?;
            let result_array = ThreadDataRaceAware {
                t: unsafe { PyArray2::<NodeT>::new(gil.python(), [body.len(), 2], false) },
            };
            body.into_par_iter()
                .enumerate()
                .for_each(|(i, (a, b))| unsafe {
                    *(result_array.t.uget_mut([i, 0])) = a;
                    *(result_array.t.uget_mut([i, 1])) = b;
                });
            result_array.t.to_owned()
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type_id)")]
    /// Returns vector containing directed edge node names with given edge type.
    ///
    /// Parameters
    /// ----------
    /// edge_type_id: Optional[int]
    ///     Edge type ID to extract.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    /// ValueError
    ///     If the given edge type ID does not exist in the graph.
    ///
    pub fn get_directed_edge_node_names_from_edge_type_id(
        &self,
        edge_type_id: Option<EdgeTypeT>,
    ) -> PyResult<Vec<(String, String)>> {
        Ok(pe!(self
            .inner
            .get_directed_edge_node_names_from_edge_type_id(edge_type_id))?
        .into_iter()
        .map(|x| {
            let (subresult_0, subresult_1) = x;
            (subresult_0.into(), subresult_1.into())
        })
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type_name)")]
    /// Returns vector containing directed edge node names with given edge type name.
    ///
    /// Parameters
    /// ----------
    /// edge_type_name: Optional[int]
    ///     Edge type name to extract.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    /// ValueError
    ///     If the given edge type name does not exist in the graph.
    ///
    pub fn get_directed_edge_node_names_from_edge_type_name(
        &self,
        edge_type_name: Option<&str>,
    ) -> PyResult<Vec<(String, String)>> {
        Ok(pe!(self
            .inner
            .get_directed_edge_node_names_from_edge_type_name(edge_type_name))?
        .into_iter()
        .map(|x| {
            let (subresult_0, subresult_1) = x;
            (subresult_0.into(), subresult_1.into())
        })
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type_id)")]
    /// Returns vector containing directed edge IDs with given edge type name.
    ///
    /// Parameters
    /// ----------
    /// edge_type_id: Optional[int]
    ///     Edge type id to extract.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    /// ValueError
    ///     If the given edge type id does not exist in the graph.
    ///
    pub fn get_directed_edge_ids_from_edge_type_id(
        &self,
        edge_type_id: Option<EdgeTypeT>,
    ) -> PyResult<Py<PyArray1<EdgeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_directed_edge_ids_from_edge_type_id(edge_type_id))?,
                EdgeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, directed, edge_type_name)")]
    /// Returns vector containing edge node IDs with given edge type name.
    ///
    /// Parameters
    /// ----------
    /// edge_type_name: Optional[&str]
    ///     Edge type name to extract.
    /// directed: bool
    ///     Whether to iterate the edge list as directed or undirected.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    /// ValueError
    ///     If the given edge type name does not exist in the graph.
    ///
    pub fn get_edge_node_ids_from_edge_type_name(
        &self,
        directed: bool,
        edge_type_name: Option<&str>,
    ) -> PyResult<Py<PyArray2<NodeT>>> {
        Ok({
            // Warning: this copies the array so it uses double the memory.
            // To avoid this you should directly generate data compatible with a numpy array
            // Which is a flat vector with row-first or column-first unrolling
            let gil = pyo3::Python::acquire_gil();
            let body = pe!(self
                .inner
                .get_edge_node_ids_from_edge_type_name(directed.clone(), edge_type_name))?;
            let result_array = ThreadDataRaceAware {
                t: unsafe { PyArray2::<NodeT>::new(gil.python(), [body.len(), 2], false) },
            };
            body.into_par_iter()
                .enumerate()
                .for_each(|(i, (a, b))| unsafe {
                    *(result_array.t.uget_mut([i, 0])) = a;
                    *(result_array.t.uget_mut([i, 1])) = b;
                });
            result_array.t.to_owned()
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type_name)")]
    /// Returns vector containing directed edge node IDs with given edge type name.
    ///
    /// Parameters
    /// ----------
    /// edge_type_names: Optional[int]
    ///     Edge type names to extract.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    /// ValueError
    ///     If the given edge type names does not exist in the graph.
    ///
    pub fn get_directed_edge_node_ids_from_edge_type_name(
        &self,
        edge_type_name: Option<&str>,
    ) -> PyResult<Py<PyArray2<NodeT>>> {
        Ok({
            // Warning: this copies the array so it uses double the memory.
            // To avoid this you should directly generate data compatible with a numpy array
            // Which is a flat vector with row-first or column-first unrolling
            let gil = pyo3::Python::acquire_gil();
            let body = pe!(self
                .inner
                .get_directed_edge_node_ids_from_edge_type_name(edge_type_name))?;
            let result_array = ThreadDataRaceAware {
                t: unsafe { PyArray2::<NodeT>::new(gil.python(), [body.len(), 2], false) },
            };
            body.into_par_iter()
                .enumerate()
                .for_each(|(i, (a, b))| unsafe {
                    *(result_array.t.uget_mut([i, 0])) = a;
                    *(result_array.t.uget_mut([i, 1])) = b;
                });
            result_array.t.to_owned()
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type_name)")]
    /// Returns vector containing directed edge IDs with given edge type name.
    ///
    /// Parameters
    /// ----------
    /// edge_type_names: Optional[int]
    ///     Edge type names to extract.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If there are no edge types in the graph.
    /// ValueError
    ///     If the given edge type names does not exist in the graph.
    ///
    pub fn get_directed_edge_ids_from_edge_type_name(
        &self,
        edge_type_name: Option<&str>,
    ) -> PyResult<Py<PyArray1<EdgeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_directed_edge_ids_from_edge_type_name(edge_type_name))?,
                EdgeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src_node_name_prefixes, dst_node_name_prefixes)")]
    /// Returns vector of directed edge node names with given node name prefixes
    ///
    /// Parameters
    /// ----------
    /// src_node_name_prefixes: Optional[List[&str]]
    ///     Prefix of the source node names.
    /// dst_node_name_prefixes: Optional[List[&str]]
    ///     Prefix of the source node names.
    ///
    pub fn get_directed_edge_node_names_from_node_curie_prefixes(
        &self,
        src_node_name_prefixes: Option<Vec<&str>>,
        dst_node_name_prefixes: Option<Vec<&str>>,
    ) -> Vec<(String, String)> {
        self.inner
            .get_directed_edge_node_names_from_node_curie_prefixes(
                src_node_name_prefixes,
                dst_node_name_prefixes,
            )
            .into_iter()
            .map(|x| {
                let (subresult_0, subresult_1) = x;
                (subresult_0.into(), subresult_1.into())
            })
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src_node_name_prefixes, dst_node_name_prefixes)")]
    /// Returns vector of directed edge node IDs with given node name prefixes
    ///
    /// Parameters
    /// ----------
    /// src_node_name_prefixes: Optional[List[&str]]
    ///     Prefix of the source node names.
    /// dst_node_name_prefixes: Optional[List[&str]]
    ///     Prefix of the source node names.
    ///
    pub fn get_directed_edge_node_ids_from_node_curie_prefixes(
        &self,
        src_node_name_prefixes: Option<Vec<&str>>,
        dst_node_name_prefixes: Option<Vec<&str>>,
    ) -> Py<PyArray2<NodeT>> {
        // Warning: this copies the array so it uses double the memory.
        // To avoid this you should directly generate data compatible with a numpy array
        // Which is a flat vector with row-first or column-first unrolling
        let gil = pyo3::Python::acquire_gil();
        let body = self
            .inner
            .get_directed_edge_node_ids_from_node_curie_prefixes(
                src_node_name_prefixes,
                dst_node_name_prefixes,
            );
        let result_array = ThreadDataRaceAware {
            t: unsafe { PyArray2::<NodeT>::new(gil.python(), [body.len(), 2], false) },
        };
        body.into_par_iter()
            .enumerate()
            .for_each(|(i, (a, b))| unsafe {
                *(result_array.t.uget_mut([i, 0])) = a;
                *(result_array.t.uget_mut([i, 1])) = b;
            });
        result_array.t.to_owned()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src_node_name_prefixes, dst_node_name_prefixes)")]
    /// Returns vector of directed edge IDs with given node name prefixes.
    ///
    /// Parameters
    /// ----------
    /// src_node_name_prefixes: Optional[List[&str]]
    ///     Prefix of the source node names.
    /// dst_node_name_prefixes: Optional[List[&str]]
    ///     Prefix of the source node names.
    ///
    pub fn get_directed_edge_ids_from_node_curie_prefixes(
        &self,
        src_node_name_prefixes: Option<Vec<&str>>,
        dst_node_name_prefixes: Option<Vec<&str>>,
    ) -> Py<PyArray1<EdgeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(
            gil,
            self.inner.get_directed_edge_ids_from_node_curie_prefixes(
                src_node_name_prefixes,
                dst_node_name_prefixes
            ),
            EdgeT
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src_node_name_prefixes, dst_node_name_prefixes)")]
    /// Returns number of directed edge IDs with given node name prefixes.
    ///
    /// Parameters
    /// ----------
    /// src_node_name_prefixes: Optional[List[&str]]
    ///     Prefix of the source node names.
    /// dst_node_name_prefixes: Optional[List[&str]]
    ///     Prefix of the source node names.
    ///
    pub fn get_number_of_directed_edges_from_node_curie_prefixes(
        &self,
        src_node_name_prefixes: Option<Vec<&str>>,
        dst_node_name_prefixes: Option<Vec<&str>>,
    ) -> EdgeT {
        self.inner
            .get_number_of_directed_edges_from_node_curie_prefixes(
                src_node_name_prefixes,
                dst_node_name_prefixes,
            )
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, curie_prefixes)")]
    /// Returns vector with node IDs with given curie prefix.
    ///
    /// Parameters
    /// ----------
    /// curie_prefixes: List[str]
    ///     Prefix of the source node names.
    ///
    pub fn get_node_ids_from_node_curie_prefixes(
        &self,
        curie_prefixes: Vec<&str>,
    ) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(
            gil,
            self.inner
                .get_node_ids_from_node_curie_prefixes(&curie_prefixes),
            NodeT
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, curie_prefixes)")]
    /// Returns vector with node names with given curie prefix.
    ///
    /// Parameters
    /// ----------
    /// curie_prefixes: List[&str]
    ///     Prefix of the source node names.
    ///
    pub fn get_node_names_from_node_curie_prefixes(
        &self,
        curie_prefixes: Vec<&str>,
    ) -> Vec<String> {
        self.inner
            .get_node_names_from_node_curie_prefixes(curie_prefixes)
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, curie_prefixes)")]
    /// Returns number of nodes with node IDs with given curie prefix.
    ///
    /// Parameters
    /// ----------
    /// curie_prefixes: List[&str]
    ///     Prefix of the source node names.
    ///
    pub fn get_number_of_nodes_from_node_curie_prefixes(&self, curie_prefixes: Vec<&str>) -> NodeT {
        self.inner
            .get_number_of_nodes_from_node_curie_prefixes(&curie_prefixes)
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, separator)")]
    /// Returns vector with node names prefixes when the node names include the provided separator.
    ///
    /// Parameters
    /// ----------
    /// separator: Optional[&str]
    ///     The separator to use to determine a prefix. By default, a column
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the provided separator is empty.
    ///
    pub fn get_node_names_prefixes(&self, separator: Option<&str>) -> PyResult<Vec<String>> {
        Ok(pe!(self.inner.get_node_names_prefixes(separator))?
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, other)")]
    /// Returns mapping from the current graph node names to the other provided graph node names.
    ///
    /// Parameters
    /// ----------
    /// other: Graph
    ///     The other graph to which remap the node names.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the graph is not contained in the provided other graph.
    ///
    pub fn get_node_ids_mapping_from_graph(&self, other: &Graph) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_node_ids_mapping_from_graph(&other.inner))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, subgraph)")]
    /// Returns the degree of every node in the provided subgraph
    pub fn get_non_zero_subgraph_node_degrees(
        &self,
        subgraph: &Graph,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_non_zero_subgraph_node_degrees(&subgraph.inner))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src, dst)")]
    /// Returns edge IDs of multigraph edge ids with same source and destination nodes and different edge type.
    ///
    /// Parameters
    /// ----------
    /// src: int
    ///     Source node id of the edge.
    /// dst: int
    ///      Destination node id of the edge.
    ///
    pub fn get_multigraph_edge_ids_from_node_ids(
        &self,
        src: NodeT,
        dst: NodeT,
    ) -> PyResult<Py<PyArray1<EdgeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_multigraph_edge_ids_from_node_ids(src.clone(), dst.clone()))?,
                EdgeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, src, dst)")]
    /// Returns number of multigraph edges with same source and destination nodes and different edge type.
    ///
    /// Parameters
    /// ----------
    /// src: int
    ///     Source node id of the edge.
    /// dst: int
    ///      Destination node id of the edge.
    ///
    pub fn get_number_of_multigraph_edges_from_node_ids(
        &self,
        src: NodeT,
        dst: NodeT,
    ) -> PyResult<usize> {
        Ok(pe!(self
            .inner
            .get_number_of_multigraph_edges_from_node_ids(src.clone(), dst.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, bfs, first_node_ids, second_node_ids)")]
    /// Returns shared ancestors of the provided node ids.
    ///
    /// Parameters
    /// ----------
    /// bfs: ShortestPathsResultBFS
    ///     The BFS object to use for the ancestors.
    /// first_node_ids: List[int]
    ///     The first node ids to query for.
    /// second_node_ids: List[int]
    ///     The second node ids to query for.
    ///
    pub fn get_ancestors_jaccard_from_node_ids(
        &self,
        bfs: &ShortestPathsResultBFS,
        first_node_ids: Vec<NodeT>,
        second_node_ids: Vec<NodeT>,
    ) -> PyResult<Py<PyArray1<WeightT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_ancestors_jaccard_from_node_ids(
                    bfs.into(),
                    &first_node_ids,
                    &second_node_ids
                ))?,
                WeightT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, bfs, first_node_names, second_node_names)")]
    /// Returns shared ancestors of the provided node names.
    ///
    /// Parameters
    /// ----------
    /// bfs: ShortestPathsResultBFS
    ///     The BFS object to use for the ancestors.
    /// first_node_names: List[str]
    ///     The first node names to query for.
    /// second_node_names: List[str]
    ///     The second node names to query for.
    ///
    pub fn get_ancestors_jaccard_from_node_names(
        &self,
        bfs: &ShortestPathsResultBFS,
        first_node_names: Vec<String>,
        second_node_names: Vec<String>,
    ) -> PyResult<Py<PyArray1<WeightT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_ancestors_jaccard_from_node_names(
                    bfs.into(),
                    &first_node_names,
                    &second_node_names
                ))?,
                WeightT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, minimum_number_of_nodes_per_tendril, compute_tendril_nodes)")]
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
        Ok(pe!(self
            .inner
            .get_tendrils(minimum_number_of_nodes_per_tendril, compute_tendril_nodes))?
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, number_of_nodes_above_threshold)")]
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
                number_of_nodes_above_threshold.clone(),
            )
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return list of the supported sparse edge weighting methods
    pub fn get_sparse_edge_weighting_methods(&self) -> Vec<String> {
        self.inner
            .get_sparse_edge_weighting_methods()
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return list of the supported edge weighting methods
    pub fn get_edge_weighting_methods(&self) -> Vec<String> {
        self.inner
            .get_edge_weighting_methods()
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edge_type_name, weight)")]
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
        Ok(pe!(self.inner.add_selfloops(edge_type_name, weight))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns vector of unweighted degree centrality for all nodes
    pub fn get_degree_centrality(&self) -> PyResult<Py<PyArray1<f32>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(gil, pe!(self.inner.get_degree_centrality())?, f32)
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns vector of weighted degree centrality for all nodes
    pub fn get_weighted_degree_centrality(&self) -> PyResult<Py<PyArray1<f32>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(gil, pe!(self.inner.get_weighted_degree_centrality())?, f32)
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
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
    pub unsafe fn get_unchecked_closeness_centrality_from_node_id(&self, node_id: NodeT) -> f32 {
        self.inner
            .get_unchecked_closeness_centrality_from_node_id(node_id.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id, use_edge_weights_as_probabilities)")]
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
    ) -> f32 {
        self.inner
            .get_unchecked_weighted_closeness_centrality_from_node_id(
                node_id.clone(),
                use_edge_weights_as_probabilities.clone(),
            )
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, verbose)")]
    /// Return closeness centrality for all nodes.
    ///
    /// Parameters
    /// ----------
    /// verbose: Optional[bool]
    ///     Whether to show an indicative progress bar.
    ///
    pub fn get_closeness_centrality(&self, verbose: Option<bool>) -> Py<PyArray1<f32>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_closeness_centrality(verbose), f32)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, use_edge_weights_as_probabilities, verbose)")]
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
    ) -> PyResult<Py<PyArray1<f32>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_weighted_closeness_centrality(
                    use_edge_weights_as_probabilities,
                    verbose
                ))?,
                f32
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
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
    pub unsafe fn get_unchecked_harmonic_centrality_from_node_id(&self, node_id: NodeT) -> f32 {
        self.inner
            .get_unchecked_harmonic_centrality_from_node_id(node_id.clone())
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id, use_edge_weights_as_probabilities)")]
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
    ) -> f32 {
        self.inner
            .get_unchecked_weighted_harmonic_centrality_from_node_id(
                node_id.clone(),
                use_edge_weights_as_probabilities.clone(),
            )
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, verbose)")]
    /// Return harmonic centrality for all nodes.
    ///
    /// Parameters
    /// ----------
    /// verbose: Optional[bool]
    ///     Whether to show an indicative progress bar.
    ///
    pub fn get_harmonic_centrality(&self, verbose: Option<bool>) -> Py<PyArray1<f32>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_harmonic_centrality(verbose), f32)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, use_edge_weights_as_probabilities, verbose)")]
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
    ) -> PyResult<Py<PyArray1<f32>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_weighted_harmonic_centrality(use_edge_weights_as_probabilities, verbose))?,
                f32
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, normalize, verbose)")]
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
    ) -> Py<PyArray1<f32>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(
            gil,
            self.inner.get_stress_centrality(normalize, verbose),
            f32
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, edges_normalization, min_max_normalization, verbose)")]
    /// Returns vector of betweenness centrality for all nodes.
    ///
    /// Parameters
    /// ----------
    /// edges_normalization: Optional[bool]
    ///     Whether to normalize the values by the number of edges of the complete graph. By default, it is false.
    /// min_max_normalization: Optional[bool]
    ///     Whether to normalize the values between 0 and 1. By default, it is false.
    /// verbose: Optional[bool]
    ///     Whether to show a loading bar. By default, it is true.
    ///
    pub fn get_betweenness_centrality(
        &self,
        edges_normalization: Option<bool>,
        min_max_normalization: Option<bool>,
        verbose: Option<bool>,
    ) -> Py<PyArray1<f32>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(
            gil,
            self.inner.get_betweenness_centrality(
                edges_normalization,
                min_max_normalization,
                verbose
            ),
            f32
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id, ant, maximum_samples_number, random_state)")]
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
        ant: Option<f32>,
        maximum_samples_number: Option<f32>,
        random_state: Option<u64>,
    ) -> PyResult<f32> {
        Ok(pe!(self
            .inner
            .get_approximated_betweenness_centrality_from_node_id(
                node_id.clone(),
                ant,
                maximum_samples_number,
                random_state
            ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_name, ant, maximum_samples_number, random_state)")]
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
        ant: Option<f32>,
        maximum_samples_number: Option<f32>,
        random_state: Option<u64>,
    ) -> PyResult<f32> {
        Ok(pe!(self
            .inner
            .get_approximated_betweenness_centrality_from_node_name(
                node_name,
                ant,
                maximum_samples_number,
                random_state
            ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, node_id, ant, use_edge_weights_as_probabilities, maximum_samples_number, random_state)"
    )]
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
        ant: Option<f32>,
        use_edge_weights_as_probabilities: Option<bool>,
        maximum_samples_number: Option<f32>,
        random_state: Option<u64>,
    ) -> PyResult<f32> {
        Ok(pe!(self
            .inner
            .get_weighted_approximated_betweenness_centrality_from_node_id(
                node_id.clone(),
                ant,
                use_edge_weights_as_probabilities,
                maximum_samples_number,
                random_state
            ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, node_name, ant, use_edge_weights_as_probabilities, maximum_samples_number, random_state)"
    )]
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
        ant: Option<f32>,
        use_edge_weights_as_probabilities: Option<bool>,
        maximum_samples_number: Option<f32>,
        random_state: Option<u64>,
    ) -> PyResult<f32> {
        Ok(pe!(self
            .inner
            .get_weighted_approximated_betweenness_centrality_from_node_name(
                node_name,
                ant,
                use_edge_weights_as_probabilities,
                maximum_samples_number,
                random_state
            ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, maximum_iterations_number, tollerance)")]
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
        tollerance: Option<f32>,
    ) -> PyResult<Py<PyArray1<f32>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_eigenvector_centrality(maximum_iterations_number, tollerance))?,
                f32
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, maximum_iterations_number, tollerance)")]
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
        tollerance: Option<f32>,
    ) -> PyResult<Py<PyArray1<f32>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_weighted_eigenvector_centrality(maximum_iterations_number, tollerance))?,
                f32
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Print the current graph in a format compatible with Graphviz dot's format
    pub fn to_dot(&self) -> String {
        self.inner.to_dot().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, minimum_number_of_nodes_per_star)")]
    /// Return vector of Stars in the current graph instance.
    ///
    /// Parameters
    /// ----------
    ///
    pub fn get_stars(
        &self,
        minimum_number_of_nodes_per_star: Option<NodeT>,
    ) -> PyResult<Vec<Star>> {
        Ok(pe!(self.inner.get_stars(minimum_number_of_nodes_per_star))?
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(
        text_signature = "($self, recursion_minimum_improvement, first_phase_minimum_improvement, patience, random_state)"
    )]
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
            recursion_minimum_improvement,
            first_phase_minimum_improvement,
            patience,
            random_state
        ))?
        .into_iter()
        .map(|x| x.into_iter().map(|x| x.into()).collect::<Vec<_>>())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_community_memberships)")]
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
    #[pyo3(text_signature = "($self, node_community_memberships)")]
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
    #[pyo3(text_signature = "($self, source_node_id, destination_node_id, maximal_hop_distance)")]
    /// Returns the structural distance from the given node IDs.
    ///
    /// Parameters
    /// ----------
    /// source_node_id: int
    ///     Node ID of the first node.
    /// destination_node_id: int
    ///     Node ID of the second node.
    /// maximal_hop_distance: int
    ///     Maximal hop distance to consider.
    ///
    ///
    /// Safety
    /// ------
    /// If either of the provided one and two node IDs are higher than the
    ///  number of nodes in the graph.
    pub unsafe fn get_unchecked_structural_distance_from_node_ids(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
        maximal_hop_distance: usize,
    ) -> Py<PyArray1<f32>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(
            gil,
            self.inner.get_unchecked_structural_distance_from_node_ids(
                source_node_id.clone(),
                destination_node_id.clone(),
                maximal_hop_distance.clone()
            ),
            f32
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns the minumum unweighted preferential attachment score.
    ///
    /// Safety
    /// ------
    /// If the graph does not contain nodes, the return value will be undefined.
    pub unsafe fn get_unchecked_minimum_preferential_attachment(&self) -> f32 {
        self.inner
            .get_unchecked_minimum_preferential_attachment()
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns the maximum unweighted preferential attachment score.
    ///
    /// Safety
    /// ------
    /// If the graph does not contain nodes, the return value will be undefined.
    pub unsafe fn get_unchecked_maximum_preferential_attachment(&self) -> f32 {
        self.inner
            .get_unchecked_maximum_preferential_attachment()
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns the minumum weighted preferential attachment score.
    ///
    /// Safety
    /// ------
    /// If the graph does not contain nodes, the return value will be undefined.
    pub unsafe fn get_unchecked_weighted_minimum_preferential_attachment(&self) -> f32 {
        self.inner
            .get_unchecked_weighted_minimum_preferential_attachment()
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns the maximum weighted preferential attachment score.
    ///
    /// Safety
    /// ------
    /// If the graph does not contain nodes, the return value will be undefined.
    pub unsafe fn get_unchecked_weighted_maximum_preferential_attachment(&self) -> f32 {
        self.inner
            .get_unchecked_weighted_maximum_preferential_attachment()
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, source_node_id, destination_node_id, normalize)")]
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
    ) -> f32 {
        self.inner
            .get_unchecked_preferential_attachment_from_node_ids(
                source_node_id.clone(),
                destination_node_id.clone(),
                normalize.clone(),
            )
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, source_node_id, destination_node_id, normalize)")]
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
    ) -> PyResult<f32> {
        Ok(pe!(self.inner.get_preferential_attachment_from_node_ids(
            source_node_id.clone(),
            destination_node_id.clone(),
            normalize.clone()
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, first_node_name, second_node_name, normalize)")]
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
    ) -> PyResult<f32> {
        Ok(pe!(self.inner.get_preferential_attachment_from_node_names(
            first_node_name,
            second_node_name,
            normalize.clone()
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, source_node_id, destination_node_id, normalize)")]
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
    ) -> f32 {
        self.inner
            .get_unchecked_weighted_preferential_attachment_from_node_ids(
                source_node_id.clone(),
                destination_node_id.clone(),
                normalize.clone(),
            )
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, source_node_id, destination_node_id, normalize)")]
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
    ) -> PyResult<f32> {
        Ok(pe!(self
            .inner
            .get_weighted_preferential_attachment_from_node_ids(
                source_node_id.clone(),
                destination_node_id.clone(),
                normalize.clone()
            ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, first_node_name, second_node_name, normalize)")]
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
    ) -> PyResult<f32> {
        Ok(pe!(self
            .inner
            .get_weighted_preferential_attachment_from_node_names(
                first_node_name,
                second_node_name,
                normalize.clone()
            ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, source_node_id, destination_node_id)")]
    /// Returns the Neighbours intersection size for the two given nodes from the given node IDs.
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
    pub unsafe fn get_unchecked_neighbours_intersection_size_from_node_ids(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
    ) -> f32 {
        self.inner
            .get_unchecked_neighbours_intersection_size_from_node_ids(
                source_node_id.clone(),
                destination_node_id.clone(),
            )
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, source_node_id, destination_node_id)")]
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
    ) -> f32 {
        self.inner
            .get_unchecked_jaccard_coefficient_from_node_ids(
                source_node_id.clone(),
                destination_node_id.clone(),
            )
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, source_node_id, destination_node_id)")]
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
    ) -> PyResult<f32> {
        Ok(pe!(self.inner.get_jaccard_coefficient_from_node_ids(
            source_node_id.clone(),
            destination_node_id.clone()
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, first_node_name, second_node_name)")]
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
    ) -> PyResult<f32> {
        Ok(pe!(self
            .inner
            .get_jaccard_coefficient_from_node_names(first_node_name, second_node_name))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, source_node_id, destination_node_id)")]
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
    ) -> f32 {
        self.inner
            .get_unchecked_adamic_adar_index_from_node_ids(
                source_node_id.clone(),
                destination_node_id.clone(),
            )
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, source_node_id, destination_node_id)")]
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
    ) -> PyResult<f32> {
        Ok(pe!(self.inner.get_adamic_adar_index_from_node_ids(
            source_node_id.clone(),
            destination_node_id.clone()
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, first_node_name, second_node_name)")]
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
    ) -> PyResult<f32> {
        Ok(pe!(self
            .inner
            .get_adamic_adar_index_from_node_names(first_node_name, second_node_name))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, source_node_id, destination_node_id)")]
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
    ) -> f32 {
        self.inner
            .get_unchecked_resource_allocation_index_from_node_ids(
                source_node_id.clone(),
                destination_node_id.clone(),
            )
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, source_node_id, destination_node_id)")]
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
    ) -> f32 {
        self.inner
            .get_unchecked_weighted_resource_allocation_index_from_node_ids(
                source_node_id.clone(),
                destination_node_id.clone(),
            )
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, source_node_id, destination_node_id)")]
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
    ) -> PyResult<f32> {
        Ok(pe!(self.inner.get_resource_allocation_index_from_node_ids(
            source_node_id.clone(),
            destination_node_id.clone()
        ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, first_node_name, second_node_name)")]
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
    ) -> PyResult<f32> {
        Ok(pe!(self
            .inner
            .get_resource_allocation_index_from_node_names(first_node_name, second_node_name))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, source_node_id, destination_node_id)")]
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
    ) -> PyResult<f32> {
        Ok(pe!(self
            .inner
            .get_weighted_resource_allocation_index_from_node_ids(
                source_node_id.clone(),
                destination_node_id.clone()
            ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, first_node_name, second_node_name)")]
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
    ) -> PyResult<f32> {
        Ok(pe!(self
            .inner
            .get_weighted_resource_allocation_index_from_node_names(
                first_node_name,
                second_node_name
            ))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns number of currently subgraphed edge metrics
    pub fn get_number_of_available_edge_metrics(&self) -> usize {
        self.inner.get_number_of_available_edge_metrics().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns names of currently subgraphed edge metrics
    pub fn get_available_edge_metrics_names(&self) -> Vec<String> {
        self.inner
            .get_available_edge_metrics_names()
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, source_node_id, destination_node_id, normalize)")]
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
    pub unsafe fn get_unchecked_all_edge_metrics_from_node_ids_tuple(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
        normalize: bool,
    ) -> Py<PyArray1<f32>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(
            gil,
            self.inner
                .get_unchecked_all_edge_metrics_from_node_ids_tuple(
                    source_node_id.clone(),
                    destination_node_id.clone(),
                    normalize.clone()
                ),
            f32
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, source_node_id, destination_node_id, normalize)")]
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
    /// Raises
    /// -------
    /// ValueError
    ///     If the provided node IDs do not exist in the current graph instance.
    ///
    pub fn get_all_edge_metrics_from_node_ids_tuple(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
        normalize: bool,
    ) -> PyResult<Py<PyArray1<f32>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self.inner.get_all_edge_metrics_from_node_ids_tuple(
                    source_node_id.clone(),
                    destination_node_id.clone(),
                    normalize.clone()
                ))?,
                f32
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, source_node_ids, destination_node_ids, normalize)")]
    /// Returns all the implemented edge metrics for the vectors source and destination node IDs.
    ///
    /// Specifically, the returned values are:
    /// * Adamic Adar
    /// * Jaccard coefficient
    /// * Resource allocation index
    /// * Preferential attachment
    ///
    /// Parameters
    /// ----------
    /// source_node_ids: List[int]
    ///     Node ID of the first node.
    /// destination_node_ids: List[int]
    ///     Node ID of the second node.
    /// normalize: bool
    ///     Whether to normalize within 0 to 1.
    ///
    ///
    /// Safety
    /// ------
    /// If the given node IDs do not exist in the graph this method will panic.
    pub fn get_all_edge_metrics_from_node_ids(
        &self,
        source_node_ids: Vec<NodeT>,
        destination_node_ids: Vec<NodeT>,
        normalize: bool,
    ) -> PyResult<Py<PyArray2<f32>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_2d!(
                gil,
                pe!(self.inner.get_all_edge_metrics_from_node_ids(
                    source_node_ids,
                    destination_node_ids,
                    normalize.clone()
                ))?,
                f32
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, normalize, subgraph)")]
    /// Returns Preferential Attachment for all edges.
    ///
    /// Parameters
    /// ----------
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the provided subgraph graph does not share a compatible vocabulary with the current graph instance.
    ///
    pub fn get_preferential_attachment_scores(
        &self,
        normalize: Option<bool>,
        subgraph: Option<&Graph>,
    ) -> PyResult<Py<PyArray1<f32>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_preferential_attachment_scores(normalize, subgraph.map(|sg| &sg.inner)))?,
                f32
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, subgraph)")]
    /// Returns Resource Allocation index for all edges.
    ///
    /// Parameters
    /// ----------
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the provided subgraph graph does not share a compatible vocabulary with the current graph instance.
    ///
    pub fn get_resource_allocation_index_scores(
        &self,
        subgraph: Option<&Graph>,
    ) -> PyResult<Py<PyArray1<f32>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_resource_allocation_index_scores(subgraph.map(|sg| &sg.inner)))?,
                f32
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, subgraph)")]
    /// Returns Jaccard Coefficient for all edges.
    ///
    /// Parameters
    /// ----------
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the provided subgraph graph does not share a compatible vocabulary with the current graph instance.
    ///
    pub fn get_jaccard_coefficient_scores(
        &self,
        subgraph: Option<&Graph>,
    ) -> PyResult<Py<PyArray1<f32>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_jaccard_coefficient_scores(subgraph.map(|sg| &sg.inner)))?,
                f32
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, subgraph)")]
    /// Returns Adamic-Adar for all edges.
    ///
    /// Parameters
    /// ----------
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the provided subgraph graph does not share a compatible vocabulary with the current graph instance.
    ///
    pub fn get_adamic_adar_scores(&self, subgraph: Option<&Graph>) -> PyResult<Py<PyArray1<f32>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_adamic_adar_scores(subgraph.map(|sg| &sg.inner)))?,
                f32
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, normalize, subgraph)")]
    /// Returns all available edge metrics for all edges.
    ///
    /// The metrics returned are, in order:
    /// - Adamic-Adar
    /// - Jaccard Coefficient
    /// - Resource Allocation index
    /// - Preferential attachment score
    ///
    /// Parameters
    /// ----------
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the provided subgraph graph does not share a compatible vocabulary with the current graph instance.
    ///
    pub fn get_all_edge_metrics(
        &self,
        normalize: Option<bool>,
        subgraph: Option<&Graph>,
    ) -> PyResult<Py<PyArray2<f32>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_2d!(
                gil,
                pe!(self
                    .inner
                    .get_all_edge_metrics(normalize, subgraph.map(|sg| &sg.inner)))?,
                f32
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, minimum_node_degree)")]
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
            .get_isomorphic_node_ids_groups(minimum_node_degree)
            .into_iter()
            .map(|x| x.into_iter().map(|x| x.into()).collect::<Vec<_>>())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, minimum_node_degree)")]
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
            .get_isomorphic_node_names_groups(minimum_node_degree)
            .into_iter()
            .map(|x| x.into_iter().map(|x| x.into()).collect::<Vec<_>>())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, minimum_node_degree)")]
    /// Returns number of isomorphic node groups.
    ///
    /// Parameters
    /// ----------
    /// minimum_node_degree: Optional[int]
    ///     Minimum node degree for the topological synonims. By default, 5.
    ///
    pub fn get_number_of_isomorphic_node_groups(
        &self,
        minimum_node_degree: Option<NodeT>,
    ) -> NodeT {
        self.inner
            .get_number_of_isomorphic_node_groups(minimum_node_degree)
            .into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns vector with isomorphic node type groups IDs
    pub fn get_isomorphic_node_type_ids_groups(&self) -> PyResult<Vec<Vec<NodeTypeT>>> {
        Ok(pe!(self.inner.get_isomorphic_node_type_ids_groups())?
            .into_iter()
            .map(|x| x.into_iter().map(|x| x.into()).collect::<Vec<_>>())
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns vector with isomorphic node type groups names
    pub fn get_isomorphic_node_type_names_groups(&self) -> PyResult<Vec<Vec<String>>> {
        Ok(pe!(self.inner.get_isomorphic_node_type_names_groups())?
            .into_iter()
            .map(|x| x.into_iter().map(|x| x.into()).collect::<Vec<_>>())
            .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns number of isomorphic node type groups
    pub fn get_number_of_isomorphic_node_type_groups(&self) -> PyResult<NodeTypeT> {
        Ok(pe!(self.inner.get_number_of_isomorphic_node_type_groups())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns vector with isomorphic node type groups IDs
    pub fn get_approximated_isomorphic_node_type_ids_groups(
        &self,
    ) -> PyResult<Vec<Vec<NodeTypeT>>> {
        Ok(pe!(self
            .inner
            .get_approximated_isomorphic_node_type_ids_groups())?
        .into_iter()
        .map(|x| x.into_iter().map(|x| x.into()).collect::<Vec<_>>())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns vector with isomorphic node type groups names
    pub fn get_approximated_isomorphic_node_type_names_groups(&self) -> PyResult<Vec<Vec<String>>> {
        Ok(pe!(self
            .inner
            .get_approximated_isomorphic_node_type_names_groups())?
        .into_iter()
        .map(|x| x.into_iter().map(|x| x.into()).collect::<Vec<_>>())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns number of isomorphic node type groups
    pub fn get_number_of_approximated_isomorphic_node_type_groups(&self) -> PyResult<NodeTypeT> {
        Ok(pe!(self
            .inner
            .get_number_of_approximated_isomorphic_node_type_groups())?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, minimum_number_of_edges)")]
    /// Returns vector with isomorphic edge type groups IDs.
    ///
    /// Parameters
    /// ----------
    /// minimum_number_of_edges: Optional[int]
    ///     Minimum number of edges to detect edge types topological synonims. By default, 5.
    ///
    pub fn get_isomorphic_edge_type_ids_groups(
        &self,
        minimum_number_of_edges: Option<EdgeT>,
    ) -> PyResult<Vec<Vec<EdgeTypeT>>> {
        Ok(pe!(self
            .inner
            .get_isomorphic_edge_type_ids_groups(minimum_number_of_edges))?
        .into_iter()
        .map(|x| x.into_iter().map(|x| x.into()).collect::<Vec<_>>())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, minimum_number_of_edges)")]
    /// Returns vector with isomorphic edge type groups names.
    ///
    /// Parameters
    /// ----------
    /// minimum_number_of_edges: Optional[int]
    ///     Minimum number of edges to detect edge types topological synonims. By default, 5.
    ///
    pub fn get_isomorphic_edge_type_names_groups(
        &self,
        minimum_number_of_edges: Option<EdgeT>,
    ) -> PyResult<Vec<Vec<String>>> {
        Ok(pe!(self
            .inner
            .get_isomorphic_edge_type_names_groups(minimum_number_of_edges))?
        .into_iter()
        .map(|x| x.into_iter().map(|x| x.into()).collect::<Vec<_>>())
        .collect::<Vec<_>>())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, minimum_number_of_edges)")]
    /// Returns number of isomorphic edge type groups.
    ///
    /// Parameters
    /// ----------
    /// minimum_number_of_edges: Optional[int]
    ///     Minimum number of edges to detect edge types topological synonims. By default, 5.
    ///
    pub fn get_number_of_isomorphic_edge_type_groups(
        &self,
        minimum_number_of_edges: Option<EdgeT>,
    ) -> PyResult<EdgeTypeT> {
        Ok(pe!(self
            .inner
            .get_number_of_isomorphic_edge_type_groups(minimum_number_of_edges))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, minimum_node_degree)")]
    /// Returns whether the current graph has topological synonims.
    ///
    /// Parameters
    /// ----------
    /// minimum_node_degree: Optional[int]
    ///     Minimum node degree for the topological synonims.
    ///
    pub fn has_isomorphic_nodes(&self, minimum_node_degree: Option<NodeT>) -> bool {
        self.inner.has_isomorphic_nodes(minimum_node_degree).into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_ids)")]
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
    #[pyo3(text_signature = "($self, node_ids)")]
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
    #[pyo3(
        text_signature = "(directed, node_type_path, node_type_list_separator, node_types_column_number, node_types_column, node_types_ids_column_number, node_types_ids_column, node_types_number, numeric_node_type_ids, minimum_node_type_id, node_type_list_header, node_type_list_support_balanced_quotes, node_type_list_rows_to_skip, node_type_list_is_correct, node_type_list_max_rows_number, node_type_list_comment_symbol, load_node_type_list_in_parallel, node_path, node_list_separator, node_list_header, node_list_support_balanced_quotes, node_list_rows_to_skip, node_list_is_correct, node_list_max_rows_number, node_list_comment_symbol, default_node_type, nodes_column_number, nodes_column, node_types_separator, node_list_node_types_column_number, node_list_node_types_column, node_ids_column, node_ids_column_number, nodes_number, minimum_node_id, numeric_node_ids, node_list_numeric_node_type_ids, skip_node_types_if_unavailable, load_node_list_in_parallel, edge_type_path, edge_types_column_number, edge_types_column, edge_types_ids_column_number, edge_types_ids_column, edge_types_number, numeric_edge_type_ids, minimum_edge_type_id, edge_type_list_separator, edge_type_list_header, edge_type_list_support_balanced_quotes, edge_type_list_rows_to_skip, edge_type_list_is_correct, edge_type_list_max_rows_number, edge_type_list_comment_symbol, load_edge_type_list_in_parallel, edge_path, edge_list_separator, edge_list_header, edge_list_support_balanced_quotes, edge_list_rows_to_skip, sources_column_number, sources_column, destinations_column_number, destinations_column, edge_list_edge_types_column_number, edge_list_edge_types_column, default_edge_type, weights_column_number, weights_column, default_weight, edge_ids_column, edge_ids_column_number, edge_list_numeric_edge_type_ids, edge_list_numeric_node_ids, skip_weights_if_unavailable, skip_edge_types_if_unavailable, edge_list_is_complete, edge_list_may_contain_duplicates, edge_list_is_sorted, edge_list_is_correct, edge_list_max_rows_number, edge_list_comment_symbol, edges_number, load_edge_list_in_parallel, remove_chevrons, remove_spaces, verbose, may_have_singletons, may_have_singleton_with_selfloops, name)"
    )]
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
    /// remove_chevrons: Optional[bool]
    ///     Whether remove chevrons while reading elements.
    /// remove_spaces: Optional[bool]
    ///     Whether remove spaces while reading elements.
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
        directed: bool,
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
        remove_chevrons: Option<bool>,
        remove_spaces: Option<bool>,
        verbose: Option<bool>,
        may_have_singletons: Option<bool>,
        may_have_singleton_with_selfloops: Option<bool>,
        name: Option<String>,
    ) -> PyResult<Graph> {
        Ok(pe!(graph::Graph::from_csv(
            directed.clone(),
            node_type_path,
            node_type_list_separator,
            node_types_column_number,
            node_types_column,
            node_types_ids_column_number,
            node_types_ids_column,
            node_types_number,
            numeric_node_type_ids,
            minimum_node_type_id,
            node_type_list_header,
            node_type_list_support_balanced_quotes,
            node_type_list_rows_to_skip,
            node_type_list_is_correct,
            node_type_list_max_rows_number,
            node_type_list_comment_symbol,
            load_node_type_list_in_parallel,
            node_path,
            node_list_separator,
            node_list_header,
            node_list_support_balanced_quotes,
            node_list_rows_to_skip,
            node_list_is_correct,
            node_list_max_rows_number,
            node_list_comment_symbol,
            default_node_type,
            nodes_column_number,
            nodes_column,
            node_types_separator,
            node_list_node_types_column_number,
            node_list_node_types_column,
            node_ids_column,
            node_ids_column_number,
            nodes_number,
            minimum_node_id,
            numeric_node_ids,
            node_list_numeric_node_type_ids,
            skip_node_types_if_unavailable,
            load_node_list_in_parallel,
            edge_type_path,
            edge_types_column_number,
            edge_types_column,
            edge_types_ids_column_number,
            edge_types_ids_column,
            edge_types_number,
            numeric_edge_type_ids,
            minimum_edge_type_id,
            edge_type_list_separator,
            edge_type_list_header,
            edge_type_list_support_balanced_quotes,
            edge_type_list_rows_to_skip,
            edge_type_list_is_correct,
            edge_type_list_max_rows_number,
            edge_type_list_comment_symbol,
            load_edge_type_list_in_parallel,
            edge_path,
            edge_list_separator,
            edge_list_header,
            edge_list_support_balanced_quotes,
            edge_list_rows_to_skip,
            sources_column_number,
            sources_column,
            destinations_column_number,
            destinations_column,
            edge_list_edge_types_column_number,
            edge_list_edge_types_column,
            default_edge_type,
            weights_column_number,
            weights_column,
            default_weight,
            edge_ids_column,
            edge_ids_column_number,
            edge_list_numeric_edge_type_ids,
            edge_list_numeric_node_ids,
            skip_weights_if_unavailable,
            skip_edge_types_if_unavailable,
            edge_list_is_complete,
            edge_list_may_contain_duplicates,
            edge_list_is_sorted,
            edge_list_is_correct,
            edge_list_max_rows_number,
            edge_list_comment_symbol,
            edges_number,
            load_edge_list_in_parallel,
            remove_chevrons,
            remove_spaces,
            verbose,
            may_have_singletons,
            may_have_singleton_with_selfloops,
            name
        ))?
        .into())
    }
}

pub const GRAPH_METHODS_NAMES: &[&str] = &[
    "is_unchecked_connected_from_node_id",
    "is_connected_from_node_id",
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
    "has_edge_from_node_id_and_edge_type_id",
    "has_unchecked_edge_from_node_id_and_edge_type_id",
    "strongly_connected_components",
    "get_jaccard_coo_matrix",
    "get_jaccard_graph",
    "get_neighbours_intersection_size_coo_matrix",
    "get_neighbours_intersection_size_graph",
    "get_shared_ancestors_size_coo_matrix",
    "get_shared_ancestors_size_graph",
    "get_ancestors_jaccard_coo_matrix",
    "get_ancestors_jaccard_graph",
    "get_adamic_adar_coo_matrix",
    "get_adamic_adar_graph",
    "get_laplacian_coo_matrix",
    "get_laplacian_graph",
    "get_left_normalized_laplacian_coo_matrix",
    "get_left_normalized_laplacian_graph",
    "get_right_normalized_laplacian_coo_matrix",
    "get_right_normalized_laplacian_graph",
    "get_symmetric_normalized_laplacian_coo_matrix",
    "get_symmetric_normalized_laplacian_graph",
    "sort_by_increasing_outbound_node_degree",
    "sort_by_decreasing_outbound_node_degree",
    "sort_by_node_lexicographic_order",
    "get_bfs_topological_sorting_from_node_id",
    "get_reversed_bfs_topological_sorting_from_node_id",
    "sort_by_bfs_topological_sorting_from_node_id",
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
    "must_have_node_ontologies",
    "must_be_undirected",
    "must_be_directed_acyclic",
    "must_not_have_trap_nodes",
    "must_be_multigraph",
    "must_not_be_multigraph",
    "must_contain_identity_matrix",
    "must_not_contain_weighted_singleton_nodes",
    "must_have_edges",
    "must_have_nodes",
    "must_be_connected",
    "must_share_node_vocabulary",
    "get_total_edge_weights",
    "get_mininum_edge_weight",
    "get_maximum_edge_weight",
    "get_unchecked_maximum_node_degree",
    "get_unchecked_minimum_node_degree",
    "get_weighted_maximum_node_degree",
    "get_weighted_minimum_node_degree",
    "get_number_of_weighted_singleton_nodes",
    "get_number_of_selfloops",
    "get_number_of_unique_selfloops",
    "generate_new_edges_from_node_features",
    "set_name",
    "set_inplace_all_edge_types",
    "set_all_edge_types",
    "set_inplace_all_node_types",
    "set_all_node_types",
    "remove_inplace_node_type_ids",
    "remove_inplace_singleton_node_types",
    "add_node_type_id_from_node_name_prefixes_inplace",
    "replace_edge_type_id_from_edge_node_type_ids_inplace",
    "replace_edge_type_id_from_edge_node_type_ids",
    "add_node_type_id_from_node_name_prefixes",
    "add_node_type_name_inplace",
    "add_node_type_name_from_node_name_prefixes_inplace",
    "add_edge_type_name_inplace",
    "replace_edge_type_name_from_edge_node_type_names_inplace",
    "replace_edge_type_name_from_edge_node_type_names",
    "add_node_type_name_from_node_name_prefixes",
    "remove_inplace_homogeneous_node_types",
    "remove_inplace_edge_type_ids",
    "remove_inplace_singleton_edge_types",
    "remove_inplace_node_type_names",
    "remove_inplace_node_type_name",
    "remove_node_type_id",
    "remove_singleton_node_types",
    "remove_homogeneous_node_types",
    "remove_inplace_isomorphic_node_types",
    "remove_isomorphic_node_types",
    "remove_inplace_isomorphic_edge_types",
    "remove_isomorphic_edge_types",
    "remove_node_type_names",
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
    "normalize_edge_weights_inplace",
    "normalize_edge_weights",
    "multiply_edge_weights_inplace",
    "multiply_edge_weights",
    "build_bipartite_graph_from_edge_node_ids",
    "build_clique_graph_from_node_ids",
    "build_bipartite_graph_from_edge_node_names",
    "build_clique_graph_from_node_names",
    "build_bipartite_graph_from_edge_node_prefixes",
    "build_clique_graph_from_node_prefixes",
    "build_bipartite_graph_from_edge_node_types",
    "build_clique_graph_from_node_type_names",
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
    "get_number_of_triads",
    "get_number_of_weighted_triads",
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
    "sample_negative_graph",
    "sample_positive_graph",
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
    "get_four_sweep",
    "get_diameter_naive",
    "get_diameter",
    "get_breadth_first_search_from_node_names",
    "get_dijkstra_from_node_names",
    "get_number_of_connected_components",
    "get_number_of_connected_nodes",
    "get_number_of_singleton_nodes_with_selfloops",
    "get_number_of_singleton_nodes",
    "get_number_of_disconnected_nodes",
    "get_singleton_node_ids",
    "get_singleton_node_names",
    "get_singleton_with_selfloops_node_ids",
    "get_singleton_with_selfloops_node_names",
    "get_density",
    "get_trap_nodes_rate",
    "get_trap_node_ids",
    "get_node_degrees_mean",
    "get_weighted_node_degrees_mean",
    "get_number_of_undirected_edges",
    "get_number_of_unique_undirected_edges",
    "get_number_of_edges",
    "get_number_of_unique_edges",
    "get_node_degrees_median",
    "get_weighted_node_degrees_median",
    "get_maximum_node_degree",
    "get_unchecked_most_central_node_id",
    "get_most_central_node_id",
    "get_minimum_node_degree",
    "get_node_degrees_mode",
    "get_selfloop_nodes_rate",
    "get_name",
    "get_number_of_trap_nodes",
    "get_source_node_ids",
    "get_directed_source_node_ids",
    "get_source_names",
    "get_destination_node_ids",
    "get_directed_destination_node_ids",
    "get_destination_names",
    "get_node_names",
    "get_node_urls",
    "get_node_ontologies",
    "get_unchecked_ontology_from_node_name",
    "get_unchecked_ontology_from_node_id",
    "get_ontology_from_node_name",
    "get_ontology_from_node_id",
    "get_node_ids",
    "get_directed_edge_type_ids",
    "get_undirected_edge_type_ids",
    "get_known_edge_type_ids",
    "get_directed_source_nodes_with_known_edge_types",
    "get_directed_destination_nodes_with_known_edge_types",
    "get_unique_edge_type_ids",
    "get_edge_type_names",
    "get_unique_edge_type_names",
    "get_directed_edge_weights",
    "get_undirected_edge_weights",
    "get_weighted_node_indegrees",
    "get_node_type_ids",
    "get_known_node_types_mask",
    "get_unknown_node_types_mask",
    "get_known_edge_types_mask",
    "get_unknown_edge_types_mask",
    "get_one_hot_encoded_node_types",
    "get_one_hot_encoded_known_node_types",
    "get_one_hot_encoded_edge_types",
    "get_one_hot_encoded_known_edge_types",
    "get_node_type_names",
    "get_unique_node_type_ids",
    "get_unique_node_type_names",
    "get_number_of_unique_directed_edges",
    "get_nodes_mapping",
    "get_edge_node_ids",
    "get_directed_edge_node_ids",
    "get_directed_edge_triples_ids",
    "get_edge_node_names",
    "get_directed_edge_node_names",
    "get_directed_edge_triples_names",
    "get_number_of_unknown_node_types",
    "get_number_of_known_node_types",
    "get_unknown_node_types_rate",
    "get_known_node_types_rate",
    "get_minimum_node_types_number",
    "get_maximum_node_types_number",
    "get_maximum_multilabel_count",
    "get_number_of_singleton_node_types",
    "get_number_of_homogeneous_node_types",
    "get_homogeneous_node_type_ids",
    "get_homogeneous_node_type_names",
    "get_singleton_node_type_ids",
    "get_singleton_node_type_names",
    "get_number_of_unknown_edge_types",
    "get_edge_ids_with_unknown_edge_types",
    "get_edge_ids_with_known_edge_types",
    "get_edge_node_ids_with_unknown_edge_types",
    "get_edge_node_ids_with_known_edge_types",
    "get_edge_node_names_with_unknown_edge_types",
    "get_edge_node_names_with_known_edge_types",
    "get_edges_with_unknown_edge_types_mask",
    "get_edges_with_known_edge_types_mask",
    "get_node_ids_with_unknown_node_types",
    "get_node_ids_with_known_node_types",
    "get_node_names_with_unknown_node_types",
    "get_node_ids_from_node_type_id",
    "get_node_ids_from_node_type_ids",
    "get_node_ids_from_node_type_names",
    "get_node_names_from_node_type_id",
    "get_node_ids_from_node_type_name",
    "get_node_names_from_node_type_name",
    "get_node_names_with_known_node_types",
    "get_nodes_with_unknown_node_types_mask",
    "get_nodes_with_known_node_types_mask",
    "get_number_of_known_edge_types",
    "get_unknown_edge_types_rate",
    "get_known_edge_types_rate",
    "get_minimum_edge_types_number",
    "get_number_of_singleton_edge_types",
    "get_singleton_edge_type_ids",
    "get_singleton_edge_type_names",
    "get_number_of_nodes",
    "get_node_connected_component_ids",
    "get_number_of_directed_edges",
    "get_number_of_edge_types",
    "get_number_of_node_types",
    "get_node_degrees",
    "get_node_indegrees",
    "get_weighted_node_degrees",
    "get_not_singletons_node_ids",
    "get_dense_nodes_mapping",
    "get_number_of_parallel_edges",
    "get_cumulative_node_degrees",
    "get_reciprocal_sqrt_degrees",
    "get_number_of_unique_source_nodes",
    "get_edge_type_id_counts_hashmap",
    "get_edge_type_names_counts_hashmap",
    "get_node_type_id_counts_hashmap",
    "get_node_type_names_counts_hashmap",
    "get_single_label_node_type_ids",
    "get_known_single_label_node_type_ids",
    "get_boolean_node_type_ids",
    "get_known_boolean_node_type_ids",
    "get_root_node_ids",
    "get_root_node_names",
    "to_directed_inplace",
    "to_directed",
    "to_upper_triangular",
    "to_lower_triangular",
    "to_main_diagonal",
    "to_anti_diagonal",
    "to_bidiagonal",
    "to_arrowhead",
    "to_transposed",
    "to_undirected",
    "to_complementary",
    "to_structural_similarity_multi_graph",
    "get_approximated_cliques",
    "get_max_clique",
    "get_approximated_number_of_cliques",
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
    "remove_unknown_node_types",
    "remove_unknown_edge_types",
    "remove_singleton_nodes",
    "remove_tendrils",
    "remove_dendritic_trees",
    "remove_isomorphic_nodes",
    "remove_singleton_nodes_with_selfloops",
    "remove_disconnected_nodes",
    "remove_selfloops",
    "remove_parallel_edges",
    "random_spanning_arborescence_kruskal",
    "spanning_arborescence_kruskal",
    "get_connected_components",
    "enable",
    "disable_all",
    "has_compatible_node_vocabularies",
    "has_compatible_node_types_vocabularies",
    "has_compatible_edge_types_vocabularies",
    "is_compatible",
    "has_same_adjacency_matrix",
    "approximated_vertex_cover_set",
    "get_random_node_type",
    "get_random_edge_type",
    "get_unchecked_random_scale_free_edge_type",
    "get_random_scale_free_edge_type",
    "get_random_node",
    "get_random_edge_id",
    "get_random_outbounds_scale_free_node",
    "get_random_inbounds_scale_free_node",
    "get_sorted_unique_random_nodes",
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
    "is_directed_acyclic",
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
    "has_exclusively_homogeneous_node_types",
    "has_homogeneous_node_ontologies",
    "has_homogeneous_edge_types",
    "has_singleton_node_types",
    "has_node_oddities",
    "has_node_types_oddities",
    "has_singleton_edge_types",
    "has_edge_types_oddities",
    "is_multigraph",
    "has_node_ontologies",
    "has_unknown_node_ontologies",
    "has_nodes_sorted_by_decreasing_outbound_node_degree",
    "has_nodes_sorted_by_lexicographic_order",
    "contains_identity_matrix",
    "has_nodes_sorted_by_increasing_outbound_node_degree",
    "has_destinations_tradeoff_enabled",
    "has_sources_tradeoff_enabled",
    "has_cumulative_node_degrees_tradeoff_enabled",
    "has_reciprocal_sqrt_degrees_tradeoff_enabled",
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
    "get_unchecked_node_count_from_node_type_id",
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
    "get_edge_type_id_from_edge_node_ids",
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
    "get_node_names_from_node_ids",
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
    "get_edge_node_ids_from_edge_type_id",
    "get_directed_edge_node_ids_from_edge_type_id",
    "get_directed_edge_node_names_from_edge_type_id",
    "get_directed_edge_node_names_from_edge_type_name",
    "get_directed_edge_ids_from_edge_type_id",
    "get_edge_node_ids_from_edge_type_name",
    "get_directed_edge_node_ids_from_edge_type_name",
    "get_directed_edge_ids_from_edge_type_name",
    "get_directed_edge_node_names_from_node_curie_prefixes",
    "get_directed_edge_node_ids_from_node_curie_prefixes",
    "get_directed_edge_ids_from_node_curie_prefixes",
    "get_number_of_directed_edges_from_node_curie_prefixes",
    "get_node_ids_from_node_curie_prefixes",
    "get_node_names_from_node_curie_prefixes",
    "get_number_of_nodes_from_node_curie_prefixes",
    "get_node_names_prefixes",
    "get_node_ids_mapping_from_graph",
    "get_non_zero_subgraph_node_degrees",
    "get_multigraph_edge_ids_from_node_ids",
    "get_number_of_multigraph_edges_from_node_ids",
    "get_ancestors_jaccard_from_node_ids",
    "get_ancestors_jaccard_from_node_names",
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
    "get_unchecked_structural_distance_from_node_ids",
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
    "get_unchecked_neighbours_intersection_size_from_node_ids",
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
    "get_number_of_available_edge_metrics",
    "get_available_edge_metrics_names",
    "get_unchecked_all_edge_metrics_from_node_ids_tuple",
    "get_all_edge_metrics_from_node_ids_tuple",
    "get_all_edge_metrics_from_node_ids",
    "get_preferential_attachment_scores",
    "get_resource_allocation_index_scores",
    "get_jaccard_coefficient_scores",
    "get_adamic_adar_scores",
    "get_all_edge_metrics",
    "get_isomorphic_node_ids_groups",
    "get_isomorphic_node_names_groups",
    "get_number_of_isomorphic_node_groups",
    "get_isomorphic_node_type_ids_groups",
    "get_isomorphic_node_type_names_groups",
    "get_number_of_isomorphic_node_type_groups",
    "get_approximated_isomorphic_node_type_ids_groups",
    "get_approximated_isomorphic_node_type_names_groups",
    "get_number_of_approximated_isomorphic_node_type_groups",
    "get_isomorphic_edge_type_ids_groups",
    "get_isomorphic_edge_type_names_groups",
    "get_number_of_isomorphic_edge_type_groups",
    "has_isomorphic_nodes",
    "has_unchecked_isomorphic_node_types_from_node_ids",
    "has_isomorphic_node_types_from_node_ids",
    "from_csv",
];

pub const GRAPH_TERMS: &[&str] = &[
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
    "get",
    "jaccard",
    "coo",
    "matrix",
    "graph",
    "neighbours",
    "intersection",
    "size",
    "shared",
    "ancestors",
    "adamic",
    "adar",
    "laplacian",
    "left",
    "normalized",
    "right",
    "symmetric",
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
    "remove",
    "overlaps",
    "contains",
    "bipartite",
    "edges",
    "star",
    "clique",
    "encode",
    "decode",
    "max",
    "encodable",
    "number",
    "validate",
    "must",
    "not",
    "contain",
    "unknown",
    "types",
    "have",
    "ontologies",
    "be",
    "undirected",
    "directed",
    "acyclic",
    "nodes",
    "multigraph",
    "identity",
    "weighted",
    "share",
    "vocabulary",
    "total",
    "weights",
    "mininum",
    "weight",
    "maximum",
    "minimum",
    "of",
    "unique",
    "generate",
    "new",
    "features",
    "set",
    "inplace",
    "all",
    "add",
    "prefixes",
    "replace",
    "homogeneous",
    "isomorphic",
    "divide",
    "normalize",
    "multiply",
    "build",
    "circles",
    "memory",
    "stats",
    "used",
    "requirement",
    "human",
    "readable",
    "requirements",
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
    "negative",
    "positive",
    "holdout",
    "random",
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
    "four",
    "sweep",
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
    "destination",
    "urls",
    "ontology",
    "known",
    "indegrees",
    "mask",
    "one",
    "hot",
    "encoded",
    "mapping",
    "triples",
    "multilabel",
    "count",
    "component",
    "singletons",
    "dense",
    "cumulative",
    "reciprocal",
    "sqrt",
    "counts",
    "hashmap",
    "single",
    "boolean",
    "root",
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
    "structural",
    "similarity",
    "multi",
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
    "tendrils",
    "dendritic",
    "trees",
    "arborescence",
    "kruskal",
    "enable",
    "disable",
    "compatible",
    "vocabularies",
    "same",
    "adjacency",
    "vertex",
    "cover",
    "scale",
    "free",
    "outbounds",
    "inbounds",
    "sorted",
    "uniform",
    "walk",
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
    "exclusively",
    "oddities",
    "destinations",
    "tradeoff",
    "enabled",
    "sources",
    "transitive",
    "closure",
    "paths",
    "minmax",
    "top",
    "comulative",
    "neighbour",
    "multiple",
    "curie",
    "non",
    "zero",
    "geometric",
    "distribution",
    "threshold",
    "sparse",
    "weighting",
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
    "distance",
    "preferential",
    "attachment",
    "index",
    "resource",
    "allocation",
    "available",
    "metrics",
    "tuple",
    "scores",
    "groups",
    "csv",
];

pub const GRAPH_TFIDF_FREQUENCIES: &[&[(&str, f64)]] = &[
    &[
        ("connected", 0.955911),
        ("from", 0.24791339),
        ("id", 0.40039027),
        ("is", 0.87113893),
        ("node", 0.14050877),
        ("unchecked", 0.4960799),
    ],
    &[
        ("connected", 1.2703197),
        ("from", 0.3294546),
        ("id", 0.5320827),
        ("is", 1.1576654),
        ("node", 0.18672353),
    ],
    &[
        ("disconnected", 0.91457266),
        ("from", 0.19276364),
        ("id", 0.31132117),
        ("is", 0.67734915),
        ("node", 0.20363192),
        ("unchecked", 0.3857241),
    ],
    &[
        ("from", 0.24791339),
        ("id", 0.40039027),
        ("is", 0.87113893),
        ("node", 0.14050877),
        ("singleton", 0.7193002),
        ("unchecked", 0.4960799),
    ],
    &[
        ("from", 0.3294546),
        ("id", 0.5320827),
        ("is", 1.1576654),
        ("node", 0.18672353),
        ("singleton", 0.9558853),
    ],
    &[
        ("from", 0.15390725),
        ("id", 0.24856652),
        ("is", 0.5408123),
        ("node", 0.087229334),
        ("selfloops", 0.5812845),
        ("singleton", 0.4465492),
        ("unchecked", 0.30797166),
        ("with", 0.48925987),
    ],
    &[
        ("from", 0.19276364),
        ("id", 0.31132117),
        ("is", 0.67734915),
        ("node", 0.1092518),
        ("selfloops", 0.7280392),
        ("singleton", 0.5592878),
        ("with", 0.61278147),
    ],
    &[
        ("from", 0.24791339),
        ("is", 0.87113893),
        ("name", 0.5584088),
        ("node", 0.14050877),
        ("singleton", 0.7193002),
        ("unchecked", 0.4960799),
    ],
    &[
        ("from", 0.3294546),
        ("is", 1.1576654),
        ("name", 0.7420751),
        ("node", 0.18672353),
        ("singleton", 0.9558853),
    ],
    &[("has", 1.5375631), ("name", 1.4959569), ("node", 0.3764179)],
    &[
        ("has", 1.0558331),
        ("id", 0.7365677),
        ("node", 0.25848338),
        ("type", 0.7125629),
    ],
    &[
        ("has", 1.0558331),
        ("name", 1.0272624),
        ("node", 0.25848338),
        ("type", 0.7125629),
    ],
    &[
        ("edge", 0.5503924),
        ("has", 1.0558331),
        ("id", 0.7365677),
        ("type", 0.7125629),
    ],
    &[
        ("edge", 0.5503924),
        ("has", 1.0558331),
        ("name", 1.0272624),
        ("type", 0.7125629),
    ],
    &[
        ("edge", 0.3975931),
        ("from", 0.3294546),
        ("has", 0.76271397),
        ("ids", 0.48930165),
        ("node", 0.18672353),
    ],
    &[
        ("from", 0.3294546),
        ("has", 0.76271397),
        ("id", 0.5320827),
        ("node", 0.18672353),
        ("selfloop", 1.7465258),
    ],
    &[
        ("and", 0.4412862),
        ("edge", 0.28934708),
        ("from", 0.12558359),
        ("has", 0.29073617),
        ("id", 0.20282264),
        ("ids", 0.18651511),
        ("node", 0.07117646),
        ("type", 0.19621265),
    ],
    &[
        ("from", 0.19276364),
        ("id", 0.31132117),
        ("is", 0.67734915),
        ("node", 0.20363192),
        ("trap", 0.821306),
        ("unchecked", 0.3857241),
    ],
    &[
        ("from", 0.24791339),
        ("id", 0.40039027),
        ("is", 0.87113893),
        ("node", 0.25688884),
        ("trap", 1.0562819),
    ],
    &[
        ("and", 0.67734915),
        ("has", 0.4462634),
        ("name", 0.8092723),
        ("node", 0.20363192),
        ("type", 0.3011752),
    ],
    &[
        ("edge", 0.3975931),
        ("from", 0.3294546),
        ("has", 0.76271397),
        ("names", 0.64066565),
        ("node", 0.18672353),
    ],
    &[
        ("and", 0.4412862),
        ("edge", 0.28934708),
        ("from", 0.12558359),
        ("has", 0.29073617),
        ("name", 0.2828689),
        ("names", 0.24421299),
        ("node", 0.07117646),
        ("type", 0.19621265),
    ],
    &[
        ("and", 0.4412862),
        ("edge", 0.28934708),
        ("from", 0.12558359),
        ("has", 0.29073617),
        ("id", 0.3872214),
        ("node", 0.07117646),
        ("type", 0.19621265),
    ],
    &[
        ("and", 0.366644),
        ("edge", 0.24226607),
        ("from", 0.10434151),
        ("has", 0.24155901),
        ("id", 0.32421482),
        ("node", 0.059137177),
        ("type", 0.16302387),
        ("unchecked", 0.20878954),
    ],
    &[
        ("components", 3.1510832),
        ("connected", 2.5608506),
        ("strongly", 3.842182),
    ],
    &[
        ("coo", 1.8410478),
        ("get", 0.1692624),
        ("jaccard", 1.7978144),
        ("matrix", 1.7224982),
    ],
    &[
        ("get", 0.24648933),
        ("graph", 1.9073191),
        ("jaccard", 2.6180775),
    ],
    &[
        ("coo", 1.0007738),
        ("get", 0.09200922),
        ("intersection", 1.235245),
        ("matrix", 0.9363315),
        ("neighbours", 1.235245),
        ("size", 1.1291114),
    ],
    &[
        ("get", 0.12227197),
        ("graph", 0.9461329),
        ("intersection", 1.6415294),
        ("neighbours", 1.6415294),
        ("size", 1.5004874),
    ],
    &[
        ("ancestors", 1.0898843),
        ("coo", 1.0007738),
        ("get", 0.09200922),
        ("matrix", 0.9363315),
        ("shared", 1.3142543),
        ("size", 1.1291114),
    ],
    &[
        ("ancestors", 1.4483582),
        ("get", 0.12227197),
        ("graph", 0.9461329),
        ("shared", 1.7465258),
        ("size", 1.5004874),
    ],
    &[
        ("ancestors", 1.4483582),
        ("coo", 1.3299384),
        ("get", 0.12227197),
        ("jaccard", 1.2987074),
        ("matrix", 1.2443004),
    ],
    &[
        ("ancestors", 2.0049777),
        ("get", 0.1692624),
        ("graph", 1.3097419),
        ("jaccard", 1.7978144),
    ],
    &[
        ("adamic", 1.4483582),
        ("adar", 1.4483582),
        ("coo", 1.3299384),
        ("get", 0.12227197),
        ("matrix", 1.2443004),
    ],
    &[
        ("adamic", 2.0049777),
        ("adar", 2.0049777),
        ("get", 0.1692624),
        ("graph", 1.3097419),
    ],
    &[
        ("coo", 1.8410478),
        ("get", 0.1692624),
        ("laplacian", 1.8890946),
        ("matrix", 1.7224982),
    ],
    &[
        ("get", 0.24648933),
        ("graph", 1.9073191),
        ("laplacian", 2.751005),
    ],
    &[
        ("coo", 1.0007738),
        ("get", 0.09200922),
        ("laplacian", 1.0268915),
        ("left", 1.3142543),
        ("matrix", 0.9363315),
        ("normalized", 1.0898843),
    ],
    &[
        ("get", 0.12227197),
        ("graph", 0.9461329),
        ("laplacian", 1.3646464),
        ("left", 1.7465258),
        ("normalized", 1.4483582),
    ],
    &[
        ("coo", 1.0007738),
        ("get", 0.09200922),
        ("laplacian", 1.0268915),
        ("matrix", 0.9363315),
        ("normalized", 1.0898843),
        ("right", 1.3142543),
    ],
    &[
        ("get", 0.12227197),
        ("graph", 0.9461329),
        ("laplacian", 1.3646464),
        ("normalized", 1.4483582),
        ("right", 1.7465258),
    ],
    &[
        ("coo", 1.0007738),
        ("get", 0.09200922),
        ("laplacian", 1.0268915),
        ("matrix", 0.9363315),
        ("normalized", 1.0898843),
        ("symmetric", 1.3142543),
    ],
    &[
        ("get", 0.12227197),
        ("graph", 0.9461329),
        ("laplacian", 1.3646464),
        ("normalized", 1.4483582),
        ("symmetric", 1.7465258),
    ],
    &[
        ("by", 1.0562819),
        ("degree", 0.79830945),
        ("increasing", 1.3142543),
        ("node", 0.14050877),
        ("outbound", 1.1762322),
        ("sort", 1.1762322),
    ],
    &[
        ("by", 1.0562819),
        ("decreasing", 1.3142543),
        ("degree", 0.79830945),
        ("node", 0.14050877),
        ("outbound", 1.1762322),
        ("sort", 1.1762322),
    ],
    &[
        ("by", 1.4037036),
        ("lexicographic", 1.7465258),
        ("node", 0.18672353),
        ("order", 1.7465258),
        ("sort", 1.5631068),
    ],
    &[
        ("bfs", 0.9604577),
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("id", 0.31132117),
        ("node", 0.1092518),
        ("sorting", 0.9604577),
        ("topological", 0.9604577),
    ],
    &[
        ("bfs", 0.76685315),
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("id", 0.24856652),
        ("node", 0.087229334),
        ("reversed", 0.8903694),
        ("sorting", 0.76685315),
        ("topological", 0.76685315),
    ],
    &[
        ("bfs", 0.76685315),
        ("by", 0.655751),
        ("from", 0.15390725),
        ("id", 0.24856652),
        ("node", 0.087229334),
        ("sort", 0.7302174),
        ("sorting", 0.76685315),
        ("topological", 0.76685315),
    ],
    &[("components", 4.8692207), ("remove", 2.808187)],
    &[("overlaps", 9.661365)],
    &[("contains", 8.8533325)],
    &[
        ("bipartite", 2.9197605),
        ("edges", 2.111287),
        ("get", 0.24648933),
    ],
    &[
        ("bipartite", 2.0049777),
        ("edge", 0.5503924),
        ("get", 0.1692624),
        ("names", 0.8868802),
    ],
    &[
        ("edges", 2.111287),
        ("get", 0.24648933),
        ("star", 3.3091762),
    ],
    &[
        ("edge", 0.5503924),
        ("get", 0.1692624),
        ("names", 0.8868802),
        ("star", 2.2723868),
    ],
    &[
        ("clique", 2.8297408),
        ("edges", 2.111287),
        ("get", 0.24648933),
    ],
    &[
        ("clique", 1.9431618),
        ("edge", 0.5503924),
        ("get", 0.1692624),
        ("names", 0.8868802),
    ],
    &[("edge", 1.2385389), ("encode", 5.937143)],
    &[("decode", 5.937143), ("edge", 1.2385389)],
    &[
        ("edge", 0.3975931),
        ("encodable", 1.905929),
        ("get", 0.12227197),
        ("max", 1.7465258),
        ("number", 0.80860204),
    ],
    &[
        ("id", 1.0726309),
        ("node", 0.3764179),
        ("validate", 2.751005),
    ],
    &[
        ("ids", 0.98638827),
        ("node", 0.3764179),
        ("validate", 2.751005),
    ],
    &[
        ("edge", 0.80151206),
        ("id", 1.0726309),
        ("validate", 2.751005),
    ],
    &[
        ("edge", 0.80151206),
        ("ids", 0.98638827),
        ("validate", 2.751005),
    ],
    &[
        ("contain", 1.1762322),
        ("must", 0.9014801),
        ("node", 0.14050877),
        ("not", 1.0898843),
        ("types", 0.48482904),
        ("unknown", 0.8201686),
    ],
    &[
        ("contain", 1.1762322),
        ("edge", 0.29918736),
        ("must", 0.9014801),
        ("not", 1.0898843),
        ("types", 0.48482904),
        ("unknown", 0.8201686),
    ],
    &[
        ("id", 0.7365677),
        ("node", 0.25848338),
        ("type", 0.7125629),
        ("validate", 1.8890946),
    ],
    &[
        ("ids", 0.6773455),
        ("node", 0.25848338),
        ("type", 0.7125629),
        ("validate", 1.8890946),
    ],
    &[
        ("edge", 0.5503924),
        ("id", 0.7365677),
        ("type", 0.7125629),
        ("validate", 1.8890946),
    ],
    &[
        ("edge", 0.5503924),
        ("ids", 0.6773455),
        ("type", 0.7125629),
        ("validate", 1.8890946),
    ],
    &[
        ("have", 2.1638255),
        ("must", 1.6583847),
        ("node", 0.25848338),
        ("ontologies", 2.0771408),
    ],
    &[
        ("be", 3.0248482),
        ("must", 2.4150321),
        ("undirected", 2.751005),
    ],
    &[
        ("acyclic", 2.4177341),
        ("be", 2.0771408),
        ("directed", 1.3664756),
        ("must", 1.6583847),
    ],
    &[
        ("have", 1.5631068),
        ("must", 1.1979859),
        ("nodes", 0.84807146),
        ("not", 1.4483582),
        ("trap", 1.4037036),
    ],
    &[
        ("be", 3.0248482),
        ("multigraph", 3.0248482),
        ("must", 2.4150321),
    ],
    &[
        ("be", 2.0771408),
        ("multigraph", 2.0771408),
        ("must", 1.6583847),
        ("not", 2.0049777),
    ],
    &[
        ("contain", 2.1638255),
        ("identity", 2.4177341),
        ("matrix", 1.7224982),
        ("must", 1.6583847),
    ],
    &[
        ("contain", 1.1762322),
        ("must", 0.9014801),
        ("nodes", 0.6381707),
        ("not", 1.0898843),
        ("singleton", 0.7193002),
        ("weighted", 0.67217934),
    ],
    &[
        ("edges", 2.111287),
        ("have", 3.1510832),
        ("must", 2.4150321),
    ],
    &[
        ("have", 3.1510832),
        ("must", 2.4150321),
        ("nodes", 1.709636),
    ],
    &[
        ("be", 3.0248482),
        ("connected", 2.5608506),
        ("must", 2.4150321),
    ],
    &[
        ("must", 1.6583847),
        ("node", 0.25848338),
        ("share", 2.6383977),
        ("vocabulary", 2.6383977),
    ],
    &[
        ("edge", 0.5503924),
        ("get", 0.1692624),
        ("total", 1.7224982),
        ("weights", 1.577151),
    ],
    &[
        ("edge", 0.5503924),
        ("get", 0.1692624),
        ("mininum", 2.6383977),
        ("weight", 1.758517),
    ],
    &[
        ("edge", 0.5503924),
        ("get", 0.1692624),
        ("maximum", 1.8890946),
        ("weight", 1.758517),
    ],
    &[
        ("degree", 1.0608814),
        ("get", 0.12227197),
        ("maximum", 1.3646464),
        ("node", 0.18672353),
        ("unchecked", 0.65924555),
    ],
    &[
        ("degree", 1.0608814),
        ("get", 0.12227197),
        ("minimum", 1.4037036),
        ("node", 0.18672353),
        ("unchecked", 0.65924555),
    ],
    &[
        ("degree", 1.0608814),
        ("get", 0.12227197),
        ("maximum", 1.3646464),
        ("node", 0.18672353),
        ("weighted", 0.8932659),
    ],
    &[
        ("degree", 1.0608814),
        ("get", 0.12227197),
        ("minimum", 1.4037036),
        ("node", 0.18672353),
        ("weighted", 0.8932659),
    ],
    &[
        ("get", 0.09200922),
        ("nodes", 0.6381707),
        ("number", 0.60847014),
        ("of", 0.62784743),
        ("singleton", 0.7193002),
        ("weighted", 0.67217934),
    ],
    &[
        ("get", 0.1692624),
        ("number", 1.1193564),
        ("of", 1.1550033),
        ("selfloops", 1.7224982),
    ],
    &[
        ("get", 0.12227197),
        ("number", 0.80860204),
        ("of", 0.83435273),
        ("selfloops", 1.2443004),
        ("unique", 1.2703197),
    ],
    &[
        ("edges", 0.78809845),
        ("features", 1.4342046),
        ("from", 0.24791339),
        ("generate", 0.955911),
        ("new", 1.4342046),
        ("node", 0.14050877),
    ],
    &[("name", 2.3116317), ("set", 4.5117683)],
    &[
        ("all", 1.2703197),
        ("edge", 0.3975931),
        ("inplace", 1.0218242),
        ("set", 1.4483582),
        ("types", 0.6442942),
    ],
    &[
        ("all", 1.758517),
        ("edge", 0.5503924),
        ("set", 2.0049777),
        ("types", 0.8919033),
    ],
    &[
        ("all", 1.2703197),
        ("inplace", 1.0218242),
        ("node", 0.18672353),
        ("set", 1.4483582),
        ("types", 0.6442942),
    ],
    &[
        ("all", 1.758517),
        ("node", 0.25848338),
        ("set", 2.0049777),
        ("types", 0.8919033),
    ],
    &[
        ("ids", 0.48930165),
        ("inplace", 1.0218242),
        ("node", 0.18672353),
        ("remove", 0.90147823),
        ("type", 0.5147421),
    ],
    &[
        ("inplace", 1.0218242),
        ("node", 0.18672353),
        ("remove", 0.90147823),
        ("singleton", 0.9558853),
        ("types", 0.6442942),
    ],
    &[
        ("add", 0.5350726),
        ("from", 0.12558359),
        ("id", 0.20282264),
        ("inplace", 0.38950545),
        ("name", 0.2828689),
        ("node", 0.13588743),
        ("prefixes", 0.45665586),
        ("type", 0.19621265),
    ],
    &[
        ("edge", 0.24226607),
        ("from", 0.10434151),
        ("id", 0.1685158),
        ("ids", 0.15496665),
        ("inplace", 0.32362178),
        ("node", 0.059137177),
        ("replace", 0.4950513),
        ("type", 0.31364864),
    ],
    &[
        ("edge", 0.28934708),
        ("from", 0.12558359),
        ("id", 0.20282264),
        ("ids", 0.18651511),
        ("node", 0.07117646),
        ("replace", 0.595835),
        ("type", 0.37460184),
    ],
    &[
        ("add", 0.655751),
        ("from", 0.15390725),
        ("id", 0.24856652),
        ("name", 0.3466661),
        ("node", 0.16484638),
        ("prefixes", 0.5596484),
        ("type", 0.24046573),
    ],
    &[
        ("add", 1.4037036),
        ("inplace", 1.0218242),
        ("name", 0.7420751),
        ("node", 0.18672353),
        ("type", 0.5147421),
    ],
    &[
        ("add", 0.5350726),
        ("from", 0.12558359),
        ("inplace", 0.38950545),
        ("name", 0.5400427),
        ("node", 0.13588743),
        ("prefixes", 0.45665586),
        ("type", 0.19621265),
    ],
    &[
        ("add", 1.4037036),
        ("edge", 0.3975931),
        ("inplace", 1.0218242),
        ("name", 0.7420751),
        ("type", 0.5147421),
    ],
    &[
        ("edge", 0.24226607),
        ("from", 0.10434151),
        ("inplace", 0.32362178),
        ("name", 0.23502247),
        ("names", 0.2029051),
        ("node", 0.059137177),
        ("replace", 0.4950513),
        ("type", 0.31364864),
    ],
    &[
        ("edge", 0.28934708),
        ("from", 0.12558359),
        ("name", 0.2828689),
        ("names", 0.24421299),
        ("node", 0.07117646),
        ("replace", 0.595835),
        ("type", 0.37460184),
    ],
    &[
        ("add", 0.655751),
        ("from", 0.15390725),
        ("name", 0.6551311),
        ("node", 0.16484638),
        ("prefixes", 0.5596484),
        ("type", 0.24046573),
    ],
    &[
        ("homogeneous", 1.3299384),
        ("inplace", 1.0218242),
        ("node", 0.18672353),
        ("remove", 0.90147823),
        ("types", 0.6442942),
    ],
    &[
        ("edge", 0.3975931),
        ("ids", 0.48930165),
        ("inplace", 1.0218242),
        ("remove", 0.90147823),
        ("type", 0.5147421),
    ],
    &[
        ("edge", 0.3975931),
        ("inplace", 1.0218242),
        ("remove", 0.90147823),
        ("singleton", 0.9558853),
        ("types", 0.6442942),
    ],
    &[
        ("inplace", 1.0218242),
        ("names", 0.64066565),
        ("node", 0.18672353),
        ("remove", 0.90147823),
        ("type", 0.5147421),
    ],
    &[
        ("inplace", 1.0218242),
        ("name", 0.7420751),
        ("node", 0.18672353),
        ("remove", 0.90147823),
        ("type", 0.5147421),
    ],
    &[
        ("id", 0.7365677),
        ("node", 0.25848338),
        ("remove", 1.2479259),
        ("type", 0.7125629),
    ],
    &[
        ("node", 0.25848338),
        ("remove", 1.2479259),
        ("singleton", 1.3232422),
        ("types", 0.8919033),
    ],
    &[
        ("homogeneous", 1.8410478),
        ("node", 0.25848338),
        ("remove", 1.2479259),
        ("types", 0.8919033),
    ],
    &[
        ("inplace", 1.0218242),
        ("isomorphic", 1.0899303),
        ("node", 0.18672353),
        ("remove", 0.90147823),
        ("types", 0.6442942),
    ],
    &[
        ("isomorphic", 1.5088022),
        ("node", 0.25848338),
        ("remove", 1.2479259),
        ("types", 0.8919033),
    ],
    &[
        ("edge", 0.3975931),
        ("inplace", 1.0218242),
        ("isomorphic", 1.0899303),
        ("remove", 0.90147823),
        ("types", 0.6442942),
    ],
    &[
        ("edge", 0.5503924),
        ("isomorphic", 1.5088022),
        ("remove", 1.2479259),
        ("types", 0.8919033),
    ],
    &[
        ("names", 0.8868802),
        ("node", 0.25848338),
        ("remove", 1.2479259),
        ("type", 0.7125629),
    ],
    &[
        ("name", 1.0272624),
        ("node", 0.25848338),
        ("remove", 1.2479259),
        ("type", 0.7125629),
    ],
    &[
        ("edge", 0.3975931),
        ("inplace", 1.0218242),
        ("name", 0.7420751),
        ("remove", 0.90147823),
        ("type", 0.5147421),
    ],
    &[
        ("edge", 0.5503924),
        ("id", 0.7365677),
        ("remove", 1.2479259),
        ("type", 0.7125629),
    ],
    &[
        ("edge", 0.5503924),
        ("remove", 1.2479259),
        ("singleton", 1.3232422),
        ("types", 0.8919033),
    ],
    &[
        ("edge", 0.5503924),
        ("name", 1.0272624),
        ("remove", 1.2479259),
        ("type", 0.7125629),
    ],
    &[
        ("inplace", 1.4145222),
        ("node", 0.25848338),
        ("remove", 1.2479259),
        ("types", 0.8919033),
    ],
    &[
        ("node", 0.3764179),
        ("remove", 1.8172992),
        ("types", 1.2988393),
    ],
    &[
        ("edge", 0.5503924),
        ("inplace", 1.4145222),
        ("remove", 1.2479259),
        ("types", 0.8919033),
    ],
    &[
        ("edge", 0.80151206),
        ("remove", 1.8172992),
        ("types", 1.2988393),
    ],
    &[
        ("edge", 0.5503924),
        ("inplace", 1.4145222),
        ("remove", 1.2479259),
        ("weights", 1.577151),
    ],
    &[
        ("edge", 0.80151206),
        ("remove", 1.8172992),
        ("weights", 2.296735),
    ],
    &[
        ("divide", 2.4177341),
        ("edge", 0.5503924),
        ("inplace", 1.4145222),
        ("weights", 1.577151),
    ],
    &[
        ("divide", 3.5208392),
        ("edge", 0.80151206),
        ("weights", 2.296735),
    ],
    &[
        ("edge", 0.5503924),
        ("inplace", 1.4145222),
        ("normalize", 2.4177341),
        ("weights", 1.577151),
    ],
    &[
        ("edge", 0.80151206),
        ("normalize", 3.5208392),
        ("weights", 2.296735),
    ],
    &[
        ("edge", 0.5503924),
        ("inplace", 1.4145222),
        ("multiply", 2.4177341),
        ("weights", 1.577151),
    ],
    &[
        ("edge", 0.80151206),
        ("multiply", 3.5208392),
        ("weights", 2.296735),
    ],
    &[
        ("bipartite", 0.8474333),
        ("build", 0.7984536),
        ("edge", 0.23263143),
        ("from", 0.19276364),
        ("graph", 0.55358166),
        ("ids", 0.28629005),
        ("node", 0.1092518),
    ],
    &[
        ("build", 1.0268915),
        ("clique", 1.0562819),
        ("from", 0.24791339),
        ("graph", 0.71196157),
        ("ids", 0.36819774),
        ("node", 0.14050877),
    ],
    &[
        ("bipartite", 0.8474333),
        ("build", 0.7984536),
        ("edge", 0.23263143),
        ("from", 0.19276364),
        ("graph", 0.55358166),
        ("names", 0.374853),
        ("node", 0.1092518),
    ],
    &[
        ("build", 1.0268915),
        ("clique", 1.0562819),
        ("from", 0.24791339),
        ("graph", 0.71196157),
        ("names", 0.48209855),
        ("node", 0.14050877),
    ],
    &[
        ("bipartite", 0.8474333),
        ("build", 0.7984536),
        ("edge", 0.23263143),
        ("from", 0.19276364),
        ("graph", 0.55358166),
        ("node", 0.1092518),
        ("prefixes", 0.70094067),
    ],
    &[
        ("build", 1.0268915),
        ("clique", 1.0562819),
        ("from", 0.24791339),
        ("graph", 0.71196157),
        ("node", 0.14050877),
        ("prefixes", 0.9014801),
    ],
    &[
        ("bipartite", 0.8474333),
        ("build", 0.7984536),
        ("edge", 0.23263143),
        ("from", 0.19276364),
        ("graph", 0.55358166),
        ("node", 0.1092518),
        ("types", 0.37697607),
    ],
    &[
        ("build", 0.7984536),
        ("clique", 0.821306),
        ("from", 0.19276364),
        ("graph", 0.55358166),
        ("names", 0.374853),
        ("node", 0.1092518),
        ("type", 0.3011752),
    ],
    &[("circles", 5.937143), ("get", 0.38088834)],
    &[
        ("get", 0.24648933),
        ("memory", 2.508398),
        ("stats", 3.842182),
    ],
    &[
        ("get", 0.1692624),
        ("memory", 1.7224982),
        ("total", 1.7224982),
        ("used", 2.6383977),
    ],
    &[
        ("get", 0.12227197),
        ("memory", 1.2443004),
        ("nodes", 0.84807146),
        ("requirement", 1.5631068),
        ("total", 1.2443004),
    ],
    &[
        ("get", 0.07154124),
        ("human", 0.87793416),
        ("memory", 0.7280392),
        ("nodes", 0.496206),
        ("readable", 0.87793416),
        ("requirement", 0.91457266),
        ("total", 0.7280392),
    ],
    &[
        ("edges", 1.0473119),
        ("get", 0.12227197),
        ("memory", 1.2443004),
        ("requirement", 1.5631068),
        ("total", 1.2443004),
    ],
    &[
        ("edges", 0.61278147),
        ("get", 0.07154124),
        ("human", 0.87793416),
        ("memory", 0.7280392),
        ("readable", 0.87793416),
        ("requirement", 0.91457266),
        ("total", 0.7280392),
    ],
    &[
        ("edge", 0.29918736),
        ("get", 0.09200922),
        ("memory", 0.9363315),
        ("requirements", 1.0898843),
        ("total", 0.9363315),
        ("weights", 0.8573223),
    ],
    &[
        ("edge", 0.18573868),
        ("get", 0.057120293),
        ("human", 0.70096433),
        ("memory", 0.5812845),
        ("readable", 0.70096433),
        ("requirements", 0.6766117),
        ("total", 0.5812845),
        ("weights", 0.5322347),
    ],
    &[
        ("get", 0.09200922),
        ("memory", 0.9363315),
        ("node", 0.14050877),
        ("requirements", 1.0898843),
        ("total", 0.9363315),
        ("types", 0.48482904),
    ],
    &[
        ("get", 0.057120293),
        ("human", 0.70096433),
        ("memory", 0.5812845),
        ("node", 0.087229334),
        ("readable", 0.70096433),
        ("requirements", 0.6766117),
        ("total", 0.5812845),
        ("types", 0.300987),
    ],
    &[
        ("edge", 0.29918736),
        ("get", 0.09200922),
        ("memory", 0.9363315),
        ("requirements", 1.0898843),
        ("total", 0.9363315),
        ("types", 0.48482904),
    ],
    &[
        ("edge", 0.18573868),
        ("get", 0.057120293),
        ("human", 0.70096433),
        ("memory", 0.5812845),
        ("readable", 0.70096433),
        ("requirements", 0.6766117),
        ("total", 0.5812845),
        ("types", 0.300987),
    ],
    &[
        ("get", 0.1692624),
        ("number", 1.1193564),
        ("of", 1.1550033),
        ("triangles", 2.4177341),
    ],
    &[
        ("get", 0.1692624),
        ("number", 1.1193564),
        ("of", 1.1550033),
        ("triads", 2.4177341),
    ],
    &[
        ("get", 0.12227197),
        ("number", 0.80860204),
        ("of", 0.83435273),
        ("triads", 1.7465258),
        ("weighted", 0.8932659),
    ],
    &[("get", 0.38088834), ("transitivity", 5.937143)],
    &[
        ("get", 0.09200922),
        ("node", 0.14050877),
        ("number", 0.60847014),
        ("of", 0.62784743),
        ("per", 1.3142543),
        ("triangles", 1.3142543),
    ],
    &[
        ("clustering", 1.6415294),
        ("coefficient", 1.4037036),
        ("get", 0.12227197),
        ("node", 0.18672353),
        ("per", 1.7465258),
    ],
    &[
        ("clustering", 3.3091762),
        ("coefficient", 2.8297408),
        ("get", 0.24648933),
    ],
    &[
        ("average", 2.6383977),
        ("clustering", 2.2723868),
        ("coefficient", 1.9431618),
        ("get", 0.1692624),
    ],
    &[
        ("are", 3.842182),
        ("nodes", 1.709636),
        ("remappable", 3.842182),
    ],
    &[
        ("from", 0.3294546),
        ("ids", 0.48930165),
        ("node", 0.18672353),
        ("remap", 1.5004874),
        ("unchecked", 0.65924555),
    ],
    &[
        ("from", 0.4560675),
        ("ids", 0.6773455),
        ("node", 0.25848338),
        ("remap", 2.0771408),
    ],
    &[
        ("from", 0.4560675),
        ("names", 0.8868802),
        ("node", 0.25848338),
        ("remap", 2.0771408),
    ],
    &[
        ("from", 0.3294546),
        ("map", 1.905929),
        ("names", 0.64066565),
        ("node", 0.18672353),
        ("remap", 1.5004874),
    ],
    &[
        ("from", 0.66415095),
        ("graph", 1.9073191),
        ("remap", 3.0248482),
    ],
    &[
        ("graph", 1.9073191),
        ("negative", 3.5208392),
        ("sample", 3.5208392),
    ],
    &[
        ("graph", 1.9073191),
        ("positive", 3.842182),
        ("sample", 3.5208392),
    ],
    &[("connected", 3.957162), ("holdout", 4.2509985)],
    &[("holdout", 4.2509985), ("random", 3.4950192)],
    &[
        ("get", 0.12227197),
        ("holdout", 1.3646464),
        ("indices", 1.905929),
        ("label", 1.2703197),
        ("node", 0.18672353),
    ],
    &[
        ("get", 0.12227197),
        ("holdout", 1.3646464),
        ("label", 1.2703197),
        ("labels", 1.905929),
        ("node", 0.18672353),
    ],
    &[
        ("get", 0.12227197),
        ("graphs", 1.7465258),
        ("holdout", 1.3646464),
        ("label", 1.2703197),
        ("node", 0.18672353),
    ],
    &[
        ("edge", 0.3975931),
        ("get", 0.12227197),
        ("graphs", 1.7465258),
        ("holdout", 1.3646464),
        ("label", 1.2703197),
    ],
    &[
        ("get", 0.24648933),
        ("random", 2.261778),
        ("subgraph", 3.5208392),
    ],
    &[
        ("get", 0.12227197),
        ("holdout", 1.3646464),
        ("label", 1.2703197),
        ("node", 0.18672353),
        ("random", 1.1219636),
    ],
    &[
        ("get", 0.1692624),
        ("kfold", 2.2723868),
        ("label", 1.758517),
        ("node", 0.25848338),
    ],
    &[
        ("edge", 0.3975931),
        ("get", 0.12227197),
        ("holdout", 1.3646464),
        ("label", 1.2703197),
        ("random", 1.1219636),
    ],
    &[
        ("edge", 0.5503924),
        ("get", 0.1692624),
        ("kfold", 2.2723868),
        ("label", 1.758517),
    ],
    &[
        ("edge", 0.5503924),
        ("get", 0.1692624),
        ("kfold", 2.2723868),
        ("prediction", 2.6383977),
    ],
    &[
        ("get", 0.24648933),
        ("node", 0.3764179),
        ("tuples", 3.842182),
    ],
    &[("chains", 5.937143), ("get", 0.38088834)],
    &[
        ("breadth", 0.42120454),
        ("first", 0.42120454),
        ("from", 0.10434151),
        ("get", 0.038724735),
        ("id", 0.1685158),
        ("node", 0.059137177),
        ("parallel", 0.47521913),
        ("predecessors", 0.60362643),
        ("search", 0.42120454),
        ("unchecked", 0.20878954),
    ],
    &[
        ("breadth", 0.42120454),
        ("distances", 0.5198885),
        ("first", 0.42120454),
        ("from", 0.10434151),
        ("get", 0.038724735),
        ("ids", 0.15496665),
        ("node", 0.059137177),
        ("parallel", 0.47521913),
        ("search", 0.42120454),
        ("unchecked", 0.20878954),
    ],
    &[
        ("breadth", 0.42120454),
        ("distances", 0.5198885),
        ("first", 0.42120454),
        ("from", 0.10434151),
        ("get", 0.038724735),
        ("id", 0.1685158),
        ("node", 0.059137177),
        ("parallel", 0.47521913),
        ("search", 0.42120454),
        ("unchecked", 0.20878954),
    ],
    &[
        ("breadth", 0.42120454),
        ("distances", 0.5198885),
        ("first", 0.42120454),
        ("from", 0.10434151),
        ("get", 0.038724735),
        ("id", 0.1685158),
        ("node", 0.059137177),
        ("search", 0.42120454),
        ("sequential", 0.60362643),
        ("unchecked", 0.20878954),
    ],
    &[
        ("breadth", 0.621291),
        ("first", 0.621291),
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("ids", 0.22858106),
        ("node", 0.087229334),
        ("search", 0.621291),
        ("unchecked", 0.30797166),
    ],
    &[
        ("breadth", 0.621291),
        ("first", 0.621291),
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("id", 0.24856652),
        ("node", 0.087229334),
        ("search", 0.621291),
        ("unchecked", 0.30797166),
    ],
    &[
        ("from", 0.12558359),
        ("get", 0.046608403),
        ("ids", 0.35608768),
        ("node", 0.13588743),
        ("path", 0.45665586),
        ("shortest", 0.4412862),
        ("unchecked", 0.2512954),
    ],
    &[
        ("from", 0.12558359),
        ("get", 0.046608403),
        ("ids", 0.18651511),
        ("names", 0.24421299),
        ("node", 0.13588743),
        ("path", 0.45665586),
        ("shortest", 0.4412862),
        ("unchecked", 0.2512954),
    ],
    &[
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("ids", 0.43197346),
        ("node", 0.16484638),
        ("path", 0.5596484),
        ("shortest", 0.5408123),
    ],
    &[
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("ids", 0.22858106),
        ("names", 0.29929188),
        ("node", 0.16484638),
        ("path", 0.5596484),
        ("shortest", 0.5408123),
    ],
    &[
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("names", 0.5656031),
        ("node", 0.16484638),
        ("path", 0.5596484),
        ("shortest", 0.5408123),
    ],
    &[
        ("from", 0.10434151),
        ("get", 0.038724735),
        ("ids", 0.29814702),
        ("k", 0.44456673),
        ("node", 0.113776565),
        ("path", 0.3794139),
        ("shortest", 0.366644),
        ("unchecked", 0.20878954),
    ],
    &[
        ("from", 0.12558359),
        ("get", 0.046608403),
        ("ids", 0.35608768),
        ("k", 0.5350726),
        ("node", 0.13588743),
        ("path", 0.45665586),
        ("shortest", 0.4412862),
    ],
    &[
        ("from", 0.12558359),
        ("get", 0.046608403),
        ("ids", 0.18651511),
        ("k", 0.5350726),
        ("names", 0.24421299),
        ("node", 0.13588743),
        ("path", 0.45665586),
        ("shortest", 0.4412862),
    ],
    &[
        ("from", 0.12558359),
        ("get", 0.046608403),
        ("k", 0.5350726),
        ("names", 0.4662423),
        ("node", 0.13588743),
        ("path", 0.45665586),
        ("shortest", 0.4412862),
    ],
    &[
        ("and", 0.309302),
        ("distant", 0.4666322),
        ("eccentricity", 0.38696858),
        ("from", 0.08802282),
        ("get", 0.032668307),
        ("id", 0.27514508),
        ("most", 0.4176268),
        ("node", 0.09655654),
        ("unchecked", 0.17613551),
    ],
    &[
        ("eccentricity", 0.8474333),
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("id", 0.31132117),
        ("node", 0.1092518),
        ("unchecked", 0.3857241),
        ("weighted", 0.5226492),
    ],
    &[
        ("and", 0.366644),
        ("distant", 0.5531419),
        ("eccentricity", 0.45870927),
        ("from", 0.10434151),
        ("get", 0.038724735),
        ("id", 0.32421482),
        ("most", 0.4950513),
        ("node", 0.113776565),
    ],
    &[
        ("eccentricity", 1.0898843),
        ("from", 0.24791339),
        ("get", 0.09200922),
        ("id", 0.40039027),
        ("node", 0.14050877),
        ("weighted", 0.67217934),
    ],
    &[
        ("eccentricity", 1.4483582),
        ("from", 0.3294546),
        ("get", 0.12227197),
        ("name", 0.7420751),
        ("node", 0.18672353),
    ],
    &[
        ("eccentricity", 1.0898843),
        ("from", 0.24791339),
        ("get", 0.09200922),
        ("name", 0.5584088),
        ("node", 0.14050877),
        ("weighted", 0.67217934),
    ],
    &[
        ("dijkstra", 1.1762322),
        ("from", 0.24791339),
        ("get", 0.09200922),
        ("ids", 0.36819774),
        ("node", 0.14050877),
        ("unchecked", 0.4960799),
    ],
    &[
        ("dijkstra", 1.1762322),
        ("from", 0.24791339),
        ("get", 0.09200922),
        ("id", 0.40039027),
        ("node", 0.14050877),
        ("unchecked", 0.4960799),
    ],
    &[
        ("from", 0.10434151),
        ("get", 0.038724735),
        ("ids", 0.29814702),
        ("node", 0.113776565),
        ("path", 0.3794139),
        ("shortest", 0.366644),
        ("unchecked", 0.20878954),
        ("weighted", 0.2829061),
    ],
    &[
        ("from", 0.10434151),
        ("get", 0.038724735),
        ("ids", 0.15496665),
        ("names", 0.2029051),
        ("node", 0.113776565),
        ("path", 0.3794139),
        ("shortest", 0.366644),
        ("unchecked", 0.20878954),
        ("weighted", 0.2829061),
    ],
    &[
        ("from", 0.12558359),
        ("get", 0.046608403),
        ("ids", 0.35608768),
        ("node", 0.13588743),
        ("path", 0.45665586),
        ("shortest", 0.4412862),
        ("weighted", 0.34050074),
    ],
    &[
        ("from", 0.12558359),
        ("get", 0.046608403),
        ("ids", 0.18651511),
        ("names", 0.24421299),
        ("node", 0.13588743),
        ("path", 0.45665586),
        ("shortest", 0.4412862),
        ("weighted", 0.34050074),
    ],
    &[
        ("from", 0.12558359),
        ("get", 0.046608403),
        ("names", 0.4662423),
        ("node", 0.13588743),
        ("path", 0.45665586),
        ("shortest", 0.4412862),
        ("weighted", 0.34050074),
    ],
    &[
        ("breadth", 0.77814597),
        ("first", 0.77814597),
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("ids", 0.28629005),
        ("node", 0.1092518),
        ("search", 0.77814597),
    ],
    &[
        ("dijkstra", 1.5631068),
        ("from", 0.3294546),
        ("get", 0.12227197),
        ("ids", 0.48930165),
        ("node", 0.18672353),
    ],
    &[("four", 3.842182), ("get", 0.24648933), ("sweep", 3.842182)],
    &[
        ("diameter", 3.5208392),
        ("get", 0.24648933),
        ("naive", 3.842182),
    ],
    &[("diameter", 5.440587), ("get", 0.38088834)],
    &[
        ("breadth", 0.77814597),
        ("first", 0.77814597),
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("names", 0.374853),
        ("node", 0.1092518),
        ("search", 0.77814597),
    ],
    &[
        ("dijkstra", 1.5631068),
        ("from", 0.3294546),
        ("get", 0.12227197),
        ("names", 0.64066565),
        ("node", 0.18672353),
    ],
    &[
        ("components", 1.5631068),
        ("connected", 1.2703197),
        ("get", 0.12227197),
        ("number", 0.80860204),
        ("of", 0.83435273),
    ],
    &[
        ("connected", 1.2703197),
        ("get", 0.12227197),
        ("nodes", 0.84807146),
        ("number", 0.80860204),
        ("of", 0.83435273),
    ],
    &[
        ("get", 0.07154124),
        ("nodes", 0.496206),
        ("number", 0.47311246),
        ("of", 0.48817918),
        ("selfloops", 0.7280392),
        ("singleton", 0.5592878),
        ("with", 0.61278147),
    ],
    &[
        ("get", 0.12227197),
        ("nodes", 0.84807146),
        ("number", 0.80860204),
        ("of", 0.83435273),
        ("singleton", 0.9558853),
    ],
    &[
        ("disconnected", 1.5631068),
        ("get", 0.12227197),
        ("nodes", 0.84807146),
        ("number", 0.80860204),
        ("of", 0.83435273),
    ],
    &[
        ("get", 0.1692624),
        ("ids", 0.6773455),
        ("node", 0.25848338),
        ("singleton", 1.3232422),
    ],
    &[
        ("get", 0.1692624),
        ("names", 0.8868802),
        ("node", 0.25848338),
        ("singleton", 1.3232422),
    ],
    &[
        ("get", 0.09200922),
        ("ids", 0.36819774),
        ("node", 0.14050877),
        ("selfloops", 0.9363315),
        ("singleton", 0.7193002),
        ("with", 0.78809845),
    ],
    &[
        ("get", 0.09200922),
        ("names", 0.48209855),
        ("node", 0.14050877),
        ("selfloops", 0.9363315),
        ("singleton", 0.7193002),
        ("with", 0.78809845),
    ],
    &[("density", 5.937143), ("get", 0.38088834)],
    &[
        ("get", 0.1692624),
        ("nodes", 1.1739943),
        ("rate", 2.0049777),
        ("trap", 1.9431618),
    ],
    &[
        ("get", 0.1692624),
        ("ids", 0.6773455),
        ("node", 0.25848338),
        ("trap", 1.9431618),
    ],
    &[
        ("degrees", 1.689253),
        ("get", 0.1692624),
        ("mean", 2.4177341),
        ("node", 0.25848338),
    ],
    &[
        ("degrees", 1.2202846),
        ("get", 0.12227197),
        ("mean", 1.7465258),
        ("node", 0.18672353),
        ("weighted", 0.8932659),
    ],
    &[
        ("edges", 1.0473119),
        ("get", 0.12227197),
        ("number", 0.80860204),
        ("of", 0.83435273),
        ("undirected", 1.3646464),
    ],
    &[
        ("edges", 0.78809845),
        ("get", 0.09200922),
        ("number", 0.60847014),
        ("of", 0.62784743),
        ("undirected", 1.0268915),
        ("unique", 0.955911),
    ],
    &[
        ("edges", 1.4498051),
        ("get", 0.1692624),
        ("number", 1.1193564),
        ("of", 1.1550033),
    ],
    &[
        ("edges", 1.0473119),
        ("get", 0.12227197),
        ("number", 0.80860204),
        ("of", 0.83435273),
        ("unique", 1.2703197),
    ],
    &[
        ("degrees", 1.689253),
        ("get", 0.1692624),
        ("median", 2.4177341),
        ("node", 0.25848338),
    ],
    &[
        ("degrees", 1.2202846),
        ("get", 0.12227197),
        ("median", 1.7465258),
        ("node", 0.18672353),
        ("weighted", 0.8932659),
    ],
    &[
        ("degree", 1.4685894),
        ("get", 0.1692624),
        ("maximum", 1.8890946),
        ("node", 0.25848338),
    ],
    &[
        ("central", 1.1291114),
        ("get", 0.09200922),
        ("id", 0.40039027),
        ("most", 1.1762322),
        ("node", 0.14050877),
        ("unchecked", 0.4960799),
    ],
    &[
        ("central", 1.5004874),
        ("get", 0.12227197),
        ("id", 0.5320827),
        ("most", 1.5631068),
        ("node", 0.18672353),
    ],
    &[
        ("degree", 1.4685894),
        ("get", 0.1692624),
        ("minimum", 1.9431618),
        ("node", 0.25848338),
    ],
    &[
        ("degrees", 1.689253),
        ("get", 0.1692624),
        ("mode", 2.6383977),
        ("node", 0.25848338),
    ],
    &[
        ("get", 0.1692624),
        ("nodes", 1.1739943),
        ("rate", 2.0049777),
        ("selfloop", 2.4177341),
    ],
    &[("get", 0.38088834), ("name", 2.3116317)],
    &[
        ("get", 0.12227197),
        ("nodes", 0.84807146),
        ("number", 0.80860204),
        ("of", 0.83435273),
        ("trap", 1.4037036),
    ],
    &[
        ("get", 0.1692624),
        ("ids", 0.6773455),
        ("node", 0.25848338),
        ("source", 1.7224982),
    ],
    &[
        ("directed", 0.98711634),
        ("get", 0.12227197),
        ("ids", 0.48930165),
        ("node", 0.18672353),
        ("source", 1.2443004),
    ],
    &[
        ("get", 0.24648933),
        ("names", 1.2915244),
        ("source", 2.508398),
    ],
    &[
        ("destination", 1.8890946),
        ("get", 0.1692624),
        ("ids", 0.6773455),
        ("node", 0.25848338),
    ],
    &[
        ("destination", 1.3646464),
        ("directed", 0.98711634),
        ("get", 0.12227197),
        ("ids", 0.48930165),
        ("node", 0.18672353),
    ],
    &[
        ("destination", 2.751005),
        ("get", 0.24648933),
        ("names", 1.2915244),
    ],
    &[
        ("get", 0.24648933),
        ("names", 1.2915244),
        ("node", 0.3764179),
    ],
    &[("get", 0.24648933), ("node", 0.3764179), ("urls", 3.842182)],
    &[
        ("get", 0.24648933),
        ("node", 0.3764179),
        ("ontologies", 3.0248482),
    ],
    &[
        ("from", 0.24791339),
        ("get", 0.09200922),
        ("name", 0.5584088),
        ("node", 0.14050877),
        ("ontology", 1.1762322),
        ("unchecked", 0.4960799),
    ],
    &[
        ("from", 0.24791339),
        ("get", 0.09200922),
        ("id", 0.40039027),
        ("node", 0.14050877),
        ("ontology", 1.1762322),
        ("unchecked", 0.4960799),
    ],
    &[
        ("from", 0.3294546),
        ("get", 0.12227197),
        ("name", 0.7420751),
        ("node", 0.18672353),
        ("ontology", 1.5631068),
    ],
    &[
        ("from", 0.3294546),
        ("get", 0.12227197),
        ("id", 0.5320827),
        ("node", 0.18672353),
        ("ontology", 1.5631068),
    ],
    &[
        ("get", 0.24648933),
        ("ids", 0.98638827),
        ("node", 0.3764179),
    ],
    &[
        ("directed", 0.98711634),
        ("edge", 0.3975931),
        ("get", 0.12227197),
        ("ids", 0.48930165),
        ("type", 0.5147421),
    ],
    &[
        ("edge", 0.3975931),
        ("get", 0.12227197),
        ("ids", 0.48930165),
        ("type", 0.5147421),
        ("undirected", 1.3646464),
    ],
    &[
        ("edge", 0.3975931),
        ("get", 0.12227197),
        ("ids", 0.48930165),
        ("known", 1.0608814),
        ("type", 0.5147421),
    ],
    &[
        ("directed", 0.461139),
        ("edge", 0.18573868),
        ("get", 0.057120293),
        ("known", 0.49559894),
        ("nodes", 0.39618313),
        ("source", 0.5812845),
        ("types", 0.300987),
        ("with", 0.48925987),
    ],
    &[
        ("destination", 0.6375051),
        ("directed", 0.461139),
        ("edge", 0.18573868),
        ("get", 0.057120293),
        ("known", 0.49559894),
        ("nodes", 0.39618313),
        ("types", 0.300987),
        ("with", 0.48925987),
    ],
    &[
        ("edge", 0.3975931),
        ("get", 0.12227197),
        ("ids", 0.48930165),
        ("type", 0.5147421),
        ("unique", 1.2703197),
    ],
    &[
        ("edge", 0.5503924),
        ("get", 0.1692624),
        ("names", 0.8868802),
        ("type", 0.7125629),
    ],
    &[
        ("edge", 0.3975931),
        ("get", 0.12227197),
        ("names", 0.64066565),
        ("type", 0.5147421),
        ("unique", 1.2703197),
    ],
    &[
        ("directed", 1.3664756),
        ("edge", 0.5503924),
        ("get", 0.1692624),
        ("weights", 1.577151),
    ],
    &[
        ("edge", 0.5503924),
        ("get", 0.1692624),
        ("undirected", 1.8890946),
        ("weights", 1.577151),
    ],
    &[
        ("get", 0.1692624),
        ("indegrees", 2.4177341),
        ("node", 0.25848338),
        ("weighted", 1.2365575),
    ],
    &[
        ("get", 0.1692624),
        ("ids", 0.6773455),
        ("node", 0.25848338),
        ("type", 0.7125629),
    ],
    &[
        ("get", 0.12227197),
        ("known", 1.0608814),
        ("mask", 1.3646464),
        ("node", 0.18672353),
        ("types", 0.6442942),
    ],
    &[
        ("get", 0.12227197),
        ("mask", 1.3646464),
        ("node", 0.18672353),
        ("types", 0.6442942),
        ("unknown", 1.0899303),
    ],
    &[
        ("edge", 0.3975931),
        ("get", 0.12227197),
        ("known", 1.0608814),
        ("mask", 1.3646464),
        ("types", 0.6442942),
    ],
    &[
        ("edge", 0.3975931),
        ("get", 0.12227197),
        ("mask", 1.3646464),
        ("types", 0.6442942),
        ("unknown", 1.0899303),
    ],
    &[
        ("encoded", 1.1762322),
        ("get", 0.09200922),
        ("hot", 1.1762322),
        ("node", 0.14050877),
        ("one", 1.1762322),
        ("types", 0.48482904),
    ],
    &[
        ("encoded", 0.91457266),
        ("get", 0.07154124),
        ("hot", 0.91457266),
        ("known", 0.620721),
        ("node", 0.1092518),
        ("one", 0.91457266),
        ("types", 0.37697607),
    ],
    &[
        ("edge", 0.29918736),
        ("encoded", 1.1762322),
        ("get", 0.09200922),
        ("hot", 1.1762322),
        ("one", 1.1762322),
        ("types", 0.48482904),
    ],
    &[
        ("edge", 0.23263143),
        ("encoded", 0.91457266),
        ("get", 0.07154124),
        ("hot", 0.91457266),
        ("known", 0.620721),
        ("one", 0.91457266),
        ("types", 0.37697607),
    ],
    &[
        ("get", 0.1692624),
        ("names", 0.8868802),
        ("node", 0.25848338),
        ("type", 0.7125629),
    ],
    &[
        ("get", 0.12227197),
        ("ids", 0.48930165),
        ("node", 0.18672353),
        ("type", 0.5147421),
        ("unique", 1.2703197),
    ],
    &[
        ("get", 0.12227197),
        ("names", 0.64066565),
        ("node", 0.18672353),
        ("type", 0.5147421),
        ("unique", 1.2703197),
    ],
    &[
        ("directed", 0.7428014),
        ("edges", 0.78809845),
        ("get", 0.09200922),
        ("number", 0.60847014),
        ("of", 0.62784743),
        ("unique", 0.955911),
    ],
    &[
        ("get", 0.24648933),
        ("mapping", 3.3091762),
        ("nodes", 1.709636),
    ],
    &[
        ("edge", 0.5503924),
        ("get", 0.1692624),
        ("ids", 0.6773455),
        ("node", 0.25848338),
    ],
    &[
        ("directed", 0.98711634),
        ("edge", 0.3975931),
        ("get", 0.12227197),
        ("ids", 0.48930165),
        ("node", 0.18672353),
    ],
    &[
        ("directed", 0.98711634),
        ("edge", 0.3975931),
        ("get", 0.12227197),
        ("ids", 0.48930165),
        ("triples", 1.7465258),
    ],
    &[
        ("edge", 0.5503924),
        ("get", 0.1692624),
        ("names", 0.8868802),
        ("node", 0.25848338),
    ],
    &[
        ("directed", 0.98711634),
        ("edge", 0.3975931),
        ("get", 0.12227197),
        ("names", 0.64066565),
        ("node", 0.18672353),
    ],
    &[
        ("directed", 0.98711634),
        ("edge", 0.3975931),
        ("get", 0.12227197),
        ("names", 0.64066565),
        ("triples", 1.7465258),
    ],
    &[
        ("get", 0.09200922),
        ("node", 0.14050877),
        ("number", 0.60847014),
        ("of", 0.62784743),
        ("types", 0.48482904),
        ("unknown", 0.8201686),
    ],
    &[
        ("get", 0.09200922),
        ("known", 0.79830945),
        ("node", 0.14050877),
        ("number", 0.60847014),
        ("of", 0.62784743),
        ("types", 0.48482904),
    ],
    &[
        ("get", 0.12227197),
        ("node", 0.18672353),
        ("rate", 1.4483582),
        ("types", 0.6442942),
        ("unknown", 1.0899303),
    ],
    &[
        ("get", 0.12227197),
        ("known", 1.0608814),
        ("node", 0.18672353),
        ("rate", 1.4483582),
        ("types", 0.6442942),
    ],
    &[
        ("get", 0.12227197),
        ("minimum", 1.4037036),
        ("node", 0.18672353),
        ("number", 0.80860204),
        ("types", 0.6442942),
    ],
    &[
        ("get", 0.12227197),
        ("maximum", 1.3646464),
        ("node", 0.18672353),
        ("number", 0.80860204),
        ("types", 0.6442942),
    ],
    &[
        ("count", 1.9431618),
        ("get", 0.1692624),
        ("maximum", 1.8890946),
        ("multilabel", 2.4177341),
    ],
    &[
        ("get", 0.09200922),
        ("node", 0.14050877),
        ("number", 0.60847014),
        ("of", 0.62784743),
        ("singleton", 0.7193002),
        ("types", 0.48482904),
    ],
    &[
        ("get", 0.09200922),
        ("homogeneous", 1.0007738),
        ("node", 0.14050877),
        ("number", 0.60847014),
        ("of", 0.62784743),
        ("types", 0.48482904),
    ],
    &[
        ("get", 0.12227197),
        ("homogeneous", 1.3299384),
        ("ids", 0.48930165),
        ("node", 0.18672353),
        ("type", 0.5147421),
    ],
    &[
        ("get", 0.12227197),
        ("homogeneous", 1.3299384),
        ("names", 0.64066565),
        ("node", 0.18672353),
        ("type", 0.5147421),
    ],
    &[
        ("get", 0.12227197),
        ("ids", 0.48930165),
        ("node", 0.18672353),
        ("singleton", 0.9558853),
        ("type", 0.5147421),
    ],
    &[
        ("get", 0.12227197),
        ("names", 0.64066565),
        ("node", 0.18672353),
        ("singleton", 0.9558853),
        ("type", 0.5147421),
    ],
    &[
        ("edge", 0.29918736),
        ("get", 0.09200922),
        ("number", 0.60847014),
        ("of", 0.62784743),
        ("types", 0.48482904),
        ("unknown", 0.8201686),
    ],
    &[
        ("edge", 0.43359637),
        ("get", 0.07154124),
        ("ids", 0.28629005),
        ("types", 0.37697607),
        ("unknown", 0.6377174),
        ("with", 0.61278147),
    ],
    &[
        ("edge", 0.43359637),
        ("get", 0.07154124),
        ("ids", 0.28629005),
        ("known", 0.620721),
        ("types", 0.37697607),
        ("with", 0.61278147),
    ],
    &[
        ("edge", 0.35100976),
        ("get", 0.057120293),
        ("ids", 0.22858106),
        ("node", 0.087229334),
        ("types", 0.300987),
        ("unknown", 0.5091694),
        ("with", 0.48925987),
    ],
    &[
        ("edge", 0.35100976),
        ("get", 0.057120293),
        ("ids", 0.22858106),
        ("known", 0.49559894),
        ("node", 0.087229334),
        ("types", 0.300987),
        ("with", 0.48925987),
    ],
    &[
        ("edge", 0.35100976),
        ("get", 0.057120293),
        ("names", 0.29929188),
        ("node", 0.087229334),
        ("types", 0.300987),
        ("unknown", 0.5091694),
        ("with", 0.48925987),
    ],
    &[
        ("edge", 0.35100976),
        ("get", 0.057120293),
        ("known", 0.49559894),
        ("names", 0.29929188),
        ("node", 0.087229334),
        ("types", 0.300987),
        ("with", 0.48925987),
    ],
    &[
        ("edge", 0.23263143),
        ("edges", 0.61278147),
        ("get", 0.07154124),
        ("mask", 0.7984536),
        ("types", 0.37697607),
        ("unknown", 0.6377174),
        ("with", 0.61278147),
    ],
    &[
        ("edge", 0.23263143),
        ("edges", 0.61278147),
        ("get", 0.07154124),
        ("known", 0.620721),
        ("mask", 0.7984536),
        ("types", 0.37697607),
        ("with", 0.61278147),
    ],
    &[
        ("get", 0.07154124),
        ("ids", 0.28629005),
        ("node", 0.20363192),
        ("types", 0.37697607),
        ("unknown", 0.6377174),
        ("with", 0.61278147),
    ],
    &[
        ("get", 0.07154124),
        ("ids", 0.28629005),
        ("known", 0.620721),
        ("node", 0.20363192),
        ("types", 0.37697607),
        ("with", 0.61278147),
    ],
    &[
        ("get", 0.07154124),
        ("names", 0.374853),
        ("node", 0.20363192),
        ("types", 0.37697607),
        ("unknown", 0.6377174),
        ("with", 0.61278147),
    ],
    &[
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("id", 0.31132117),
        ("ids", 0.28629005),
        ("node", 0.20363192),
        ("type", 0.3011752),
    ],
    &[
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("ids", 0.53360945),
        ("node", 0.20363192),
        ("type", 0.3011752),
    ],
    &[
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("ids", 0.28629005),
        ("names", 0.374853),
        ("node", 0.20363192),
        ("type", 0.3011752),
    ],
    &[
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("id", 0.31132117),
        ("names", 0.374853),
        ("node", 0.20363192),
        ("type", 0.3011752),
    ],
    &[
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("ids", 0.28629005),
        ("name", 0.4341876),
        ("node", 0.20363192),
        ("type", 0.3011752),
    ],
    &[
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("name", 0.4341876),
        ("names", 0.374853),
        ("node", 0.20363192),
        ("type", 0.3011752),
    ],
    &[
        ("get", 0.07154124),
        ("known", 0.620721),
        ("names", 0.374853),
        ("node", 0.20363192),
        ("types", 0.37697607),
        ("with", 0.61278147),
    ],
    &[
        ("get", 0.07154124),
        ("mask", 0.7984536),
        ("node", 0.1092518),
        ("nodes", 0.496206),
        ("types", 0.37697607),
        ("unknown", 0.6377174),
        ("with", 0.61278147),
    ],
    &[
        ("get", 0.07154124),
        ("known", 0.620721),
        ("mask", 0.7984536),
        ("node", 0.1092518),
        ("nodes", 0.496206),
        ("types", 0.37697607),
        ("with", 0.61278147),
    ],
    &[
        ("edge", 0.29918736),
        ("get", 0.09200922),
        ("known", 0.79830945),
        ("number", 0.60847014),
        ("of", 0.62784743),
        ("types", 0.48482904),
    ],
    &[
        ("edge", 0.3975931),
        ("get", 0.12227197),
        ("rate", 1.4483582),
        ("types", 0.6442942),
        ("unknown", 1.0899303),
    ],
    &[
        ("edge", 0.3975931),
        ("get", 0.12227197),
        ("known", 1.0608814),
        ("rate", 1.4483582),
        ("types", 0.6442942),
    ],
    &[
        ("edge", 0.3975931),
        ("get", 0.12227197),
        ("minimum", 1.4037036),
        ("number", 0.80860204),
        ("types", 0.6442942),
    ],
    &[
        ("edge", 0.29918736),
        ("get", 0.09200922),
        ("number", 0.60847014),
        ("of", 0.62784743),
        ("singleton", 0.7193002),
        ("types", 0.48482904),
    ],
    &[
        ("edge", 0.3975931),
        ("get", 0.12227197),
        ("ids", 0.48930165),
        ("singleton", 0.9558853),
        ("type", 0.5147421),
    ],
    &[
        ("edge", 0.3975931),
        ("get", 0.12227197),
        ("names", 0.64066565),
        ("singleton", 0.9558853),
        ("type", 0.5147421),
    ],
    &[
        ("get", 0.1692624),
        ("nodes", 1.1739943),
        ("number", 1.1193564),
        ("of", 1.1550033),
    ],
    &[
        ("component", 1.905929),
        ("connected", 1.2703197),
        ("get", 0.12227197),
        ("ids", 0.48930165),
        ("node", 0.18672353),
    ],
    &[
        ("directed", 0.98711634),
        ("edges", 1.0473119),
        ("get", 0.12227197),
        ("number", 0.80860204),
        ("of", 0.83435273),
    ],
    &[
        ("edge", 0.3975931),
        ("get", 0.12227197),
        ("number", 0.80860204),
        ("of", 0.83435273),
        ("types", 0.6442942),
    ],
    &[
        ("get", 0.12227197),
        ("node", 0.18672353),
        ("number", 0.80860204),
        ("of", 0.83435273),
        ("types", 0.6442942),
    ],
    &[
        ("degrees", 2.4599845),
        ("get", 0.24648933),
        ("node", 0.3764179),
    ],
    &[
        ("get", 0.24648933),
        ("indegrees", 3.5208392),
        ("node", 0.3764179),
    ],
    &[
        ("degrees", 1.689253),
        ("get", 0.1692624),
        ("node", 0.25848338),
        ("weighted", 1.2365575),
    ],
    &[
        ("get", 0.12227197),
        ("ids", 0.48930165),
        ("node", 0.18672353),
        ("not", 1.4483582),
        ("singletons", 1.905929),
    ],
    &[
        ("dense", 2.6383977),
        ("get", 0.1692624),
        ("mapping", 2.2723868),
        ("nodes", 1.1739943),
    ],
    &[
        ("edges", 1.0473119),
        ("get", 0.12227197),
        ("number", 0.80860204),
        ("of", 0.83435273),
        ("parallel", 1.5004874),
    ],
    &[
        ("cumulative", 2.4177341),
        ("degrees", 1.689253),
        ("get", 0.1692624),
        ("node", 0.25848338),
    ],
    &[
        ("degrees", 1.689253),
        ("get", 0.1692624),
        ("reciprocal", 2.0771408),
        ("sqrt", 2.0771408),
    ],
    &[
        ("get", 0.09200922),
        ("nodes", 0.6381707),
        ("number", 0.60847014),
        ("of", 0.62784743),
        ("source", 0.9363315),
        ("unique", 0.955911),
    ],
    &[
        ("counts", 1.0898843),
        ("edge", 0.29918736),
        ("get", 0.09200922),
        ("hashmap", 1.0898843),
        ("id", 0.40039027),
        ("type", 0.38734153),
    ],
    &[
        ("counts", 1.0898843),
        ("edge", 0.29918736),
        ("get", 0.09200922),
        ("hashmap", 1.0898843),
        ("names", 0.48209855),
        ("type", 0.38734153),
    ],
    &[
        ("counts", 1.0898843),
        ("get", 0.09200922),
        ("hashmap", 1.0898843),
        ("id", 0.40039027),
        ("node", 0.14050877),
        ("type", 0.38734153),
    ],
    &[
        ("counts", 1.0898843),
        ("get", 0.09200922),
        ("hashmap", 1.0898843),
        ("names", 0.48209855),
        ("node", 0.14050877),
        ("type", 0.38734153),
    ],
    &[
        ("get", 0.09200922),
        ("ids", 0.36819774),
        ("label", 0.955911),
        ("node", 0.14050877),
        ("single", 1.3142543),
        ("type", 0.38734153),
    ],
    &[
        ("get", 0.07154124),
        ("ids", 0.28629005),
        ("known", 0.620721),
        ("label", 0.7432631),
        ("node", 0.1092518),
        ("single", 1.021891),
        ("type", 0.3011752),
    ],
    &[
        ("boolean", 1.7465258),
        ("get", 0.12227197),
        ("ids", 0.48930165),
        ("node", 0.18672353),
        ("type", 0.5147421),
    ],
    &[
        ("boolean", 1.3142543),
        ("get", 0.09200922),
        ("ids", 0.36819774),
        ("known", 0.79830945),
        ("node", 0.14050877),
        ("type", 0.38734153),
    ],
    &[
        ("get", 0.1692624),
        ("ids", 0.6773455),
        ("node", 0.25848338),
        ("root", 2.4177341),
    ],
    &[
        ("get", 0.1692624),
        ("names", 0.8868802),
        ("node", 0.25848338),
        ("root", 2.4177341),
    ],
    &[
        ("directed", 1.989938),
        ("inplace", 2.0599062),
        ("to", 2.4599845),
    ],
    &[("directed", 3.0749574), ("to", 3.8012981)],
    &[
        ("to", 2.4599845),
        ("triangular", 3.5208392),
        ("upper", 3.842182),
    ],
    &[
        ("lower", 3.842182),
        ("to", 2.4599845),
        ("triangular", 3.5208392),
    ],
    &[
        ("diagonal", 3.5208392),
        ("main", 3.842182),
        ("to", 2.4599845),
    ],
    &[
        ("anti", 3.842182),
        ("diagonal", 3.5208392),
        ("to", 2.4599845),
    ],
    &[("bidiagonal", 5.937143), ("to", 3.8012981)],
    &[("arrowhead", 5.937143), ("to", 3.8012981)],
    &[("to", 3.8012981), ("transposed", 5.937143)],
    &[("to", 3.8012981), ("undirected", 4.2509985)],
    &[("complementary", 5.937143), ("to", 3.8012981)],
    &[
        ("graph", 0.9461329),
        ("multi", 1.905929),
        ("similarity", 1.905929),
        ("structural", 1.7465258),
        ("to", 1.2202846),
    ],
    &[
        ("approximated", 2.6180775),
        ("cliques", 3.5208392),
        ("get", 0.24648933),
    ],
    &[
        ("clique", 2.8297408),
        ("get", 0.24648933),
        ("max", 3.5208392),
    ],
    &[
        ("approximated", 1.2987074),
        ("cliques", 1.7465258),
        ("get", 0.12227197),
        ("number", 0.80860204),
        ("of", 0.83435273),
    ],
    &[("report", 7.6061373)],
    &[
        ("overlap", 3.842182),
        ("report", 3.0248482),
        ("textual", 3.5208392),
    ],
    &[
        ("from", 0.24791339),
        ("get", 0.09200922),
        ("id", 0.40039027),
        ("node", 0.25688884),
        ("report", 1.1291114),
    ],
    &[
        ("from", 0.24791339),
        ("get", 0.09200922),
        ("name", 0.5584088),
        ("node", 0.25688884),
        ("report", 1.1291114),
    ],
    &[("report", 4.6741557), ("textual", 5.440587)],
    &[
        ("connected", 1.758517),
        ("generate", 1.758517),
        ("graph", 1.3097419),
        ("random", 1.5531461),
    ],
    &[
        ("generate", 1.758517),
        ("random", 1.5531461),
        ("spanning", 2.2723868),
        ("tree", 2.6383977),
    ],
    &[
        ("generate", 2.5608506),
        ("graph", 1.9073191),
        ("star", 3.3091762),
    ],
    &[
        ("generate", 2.5608506),
        ("graph", 1.9073191),
        ("wheel", 3.842182),
    ],
    &[
        ("circle", 3.842182),
        ("generate", 2.5608506),
        ("graph", 1.9073191),
    ],
    &[
        ("chain", 3.842182),
        ("generate", 2.5608506),
        ("graph", 1.9073191),
    ],
    &[
        ("complete", 3.842182),
        ("generate", 2.5608506),
        ("graph", 1.9073191),
    ],
    &[
        ("barbell", 3.842182),
        ("generate", 2.5608506),
        ("graph", 1.9073191),
    ],
    &[
        ("generate", 2.5608506),
        ("graph", 1.9073191),
        ("lollipop", 3.842182),
    ],
    &[
        ("generate", 1.758517),
        ("graph", 1.3097419),
        ("lattice", 2.6383977),
        ("squared", 2.6383977),
    ],
    &[
        ("filter", 3.5208392),
        ("from", 0.66415095),
        ("ids", 0.98638827),
    ],
    &[
        ("filter", 3.5208392),
        ("from", 0.66415095),
        ("names", 1.2915244),
    ],
    &[
        ("node", 0.25848338),
        ("remove", 1.2479259),
        ("types", 0.8919033),
        ("unknown", 1.5088022),
    ],
    &[
        ("edge", 0.5503924),
        ("remove", 1.2479259),
        ("types", 0.8919033),
        ("unknown", 1.5088022),
    ],
    &[
        ("nodes", 1.709636),
        ("remove", 1.8172992),
        ("singleton", 1.9269791),
    ],
    &[("remove", 2.808187), ("tendrils", 5.440587)],
    &[
        ("dendritic", 3.5208392),
        ("remove", 1.8172992),
        ("trees", 3.5208392),
    ],
    &[
        ("isomorphic", 2.197202),
        ("nodes", 1.709636),
        ("remove", 1.8172992),
    ],
    &[
        ("nodes", 0.84807146),
        ("remove", 0.90147823),
        ("selfloops", 1.2443004),
        ("singleton", 0.9558853),
        ("with", 1.0473119),
    ],
    &[
        ("disconnected", 3.1510832),
        ("nodes", 1.709636),
        ("remove", 1.8172992),
    ],
    &[("remove", 2.808187), ("selfloops", 3.8761091)],
    &[
        ("edges", 2.111287),
        ("parallel", 3.0248482),
        ("remove", 1.8172992),
    ],
    &[
        ("arborescence", 2.4177341),
        ("kruskal", 2.4177341),
        ("random", 1.5531461),
        ("spanning", 2.2723868),
    ],
    &[
        ("arborescence", 3.5208392),
        ("kruskal", 3.5208392),
        ("spanning", 3.3091762),
    ],
    &[
        ("components", 3.1510832),
        ("connected", 2.5608506),
        ("get", 0.24648933),
    ],
    &[("enable", 9.661365)],
    &[("all", 3.957162), ("disable", 5.937143)],
    &[
        ("compatible", 2.1638255),
        ("has", 1.0558331),
        ("node", 0.25848338),
        ("vocabularies", 2.2723868),
    ],
    &[
        ("compatible", 1.5631068),
        ("has", 0.76271397),
        ("node", 0.18672353),
        ("types", 0.6442942),
        ("vocabularies", 1.6415294),
    ],
    &[
        ("compatible", 1.5631068),
        ("edge", 0.3975931),
        ("has", 0.76271397),
        ("types", 0.6442942),
        ("vocabularies", 1.6415294),
    ],
    &[("compatible", 4.8692207), ("is", 3.6062331)],
    &[
        ("adjacency", 2.6383977),
        ("has", 1.0558331),
        ("matrix", 1.7224982),
        ("same", 2.6383977),
    ],
    &[
        ("approximated", 1.7978144),
        ("cover", 2.6383977),
        ("set", 2.0049777),
        ("vertex", 2.6383977),
    ],
    &[
        ("get", 0.1692624),
        ("node", 0.25848338),
        ("random", 1.5531461),
        ("type", 0.7125629),
    ],
    &[
        ("edge", 0.5503924),
        ("get", 0.1692624),
        ("random", 1.5531461),
        ("type", 0.7125629),
    ],
    &[
        ("edge", 0.23263143),
        ("free", 0.91457266),
        ("get", 0.07154124),
        ("random", 0.6564601),
        ("scale", 0.91457266),
        ("type", 0.3011752),
        ("unchecked", 0.3857241),
    ],
    &[
        ("edge", 0.29918736),
        ("free", 1.1762322),
        ("get", 0.09200922),
        ("random", 0.84427357),
        ("scale", 1.1762322),
        ("type", 0.38734153),
    ],
    &[
        ("get", 0.24648933),
        ("node", 0.3764179),
        ("random", 2.261778),
    ],
    &[
        ("edge", 0.5503924),
        ("get", 0.1692624),
        ("id", 0.7365677),
        ("random", 1.5531461),
    ],
    &[
        ("free", 1.1762322),
        ("get", 0.09200922),
        ("node", 0.14050877),
        ("outbounds", 1.4342046),
        ("random", 0.84427357),
        ("scale", 1.1762322),
    ],
    &[
        ("free", 1.1762322),
        ("get", 0.09200922),
        ("inbounds", 1.4342046),
        ("node", 0.14050877),
        ("random", 0.84427357),
        ("scale", 1.1762322),
    ],
    &[
        ("get", 0.12227197),
        ("nodes", 0.84807146),
        ("random", 1.1219636),
        ("sorted", 1.5631068),
        ("unique", 1.2703197),
    ],
    &[
        ("breadth", 1.0007738),
        ("first", 1.0007738),
        ("get", 0.09200922),
        ("nodes", 0.6381707),
        ("random", 0.84427357),
        ("search", 1.0007738),
    ],
    &[
        ("get", 0.09200922),
        ("nodes", 0.6381707),
        ("random", 1.5435652),
        ("uniform", 1.4342046),
        ("walk", 1.4342046),
    ],
    &[
        ("get", 0.1692624),
        ("methods", 2.2723868),
        ("node", 0.25848338),
        ("sampling", 2.6383977),
    ],
    &[
        ("get", 0.24648933),
        ("nodes", 1.709636),
        ("subsampled", 3.842182),
    ],
    &[
        ("bm25", 1.3142543),
        ("feature", 1.4342046),
        ("get", 0.09200922),
        ("node", 0.14050877),
        ("okapi", 1.3142543),
        ("propagation", 1.3142543),
    ],
    &[
        ("bm25", 1.3142543),
        ("get", 0.09200922),
        ("label", 0.955911),
        ("node", 0.14050877),
        ("okapi", 1.3142543),
        ("propagation", 1.3142543),
    ],
    &[
        ("default", 2.6383977),
        ("graph", 1.3097419),
        ("has", 1.0558331),
        ("name", 1.0272624),
    ],
    &[("has", 2.3759239), ("nodes", 2.64182)],
    &[("edges", 3.2624724), ("has", 2.3759239)],
    &[("has", 1.5375631), ("nodes", 1.709636), ("trap", 2.8297408)],
    &[("directed", 3.0749574), ("is", 3.6062331)],
    &[
        ("acyclic", 3.5208392),
        ("directed", 1.989938),
        ("is", 2.3337495),
    ],
    &[
        ("edge", 0.80151206),
        ("has", 1.5375631),
        ("weights", 2.296735),
    ],
    &[
        ("edge", 0.3975931),
        ("has", 0.76271397),
        ("probabilities", 1.905929),
        ("representing", 1.905929),
        ("weights", 1.1393042),
    ],
    &[
        ("has", 1.0558331),
        ("nodes", 1.1739943),
        ("singleton", 1.3232422),
        ("weighted", 1.2365575),
    ],
    &[
        ("constant", 2.6383977),
        ("edge", 0.5503924),
        ("has", 1.0558331),
        ("weights", 1.577151),
    ],
    &[
        ("edge", 0.5503924),
        ("has", 1.0558331),
        ("negative", 2.4177341),
        ("weights", 1.577151),
    ],
    &[
        ("edge", 0.80151206),
        ("has", 1.5375631),
        ("types", 1.2988393),
    ],
    &[("has", 2.3759239), ("selfloops", 3.8761091)],
    &[
        ("disconnected", 3.1510832),
        ("has", 1.5375631),
        ("nodes", 1.709636),
    ],
    &[
        ("has", 1.5375631),
        ("nodes", 1.709636),
        ("singleton", 1.9269791),
    ],
    &[
        ("has", 0.76271397),
        ("nodes", 0.84807146),
        ("selfloops", 1.2443004),
        ("singleton", 0.9558853),
        ("with", 1.0473119),
    ],
    &[("connected", 3.957162), ("is", 3.6062331)],
    &[
        ("has", 1.5375631),
        ("node", 0.3764179),
        ("types", 1.2988393),
    ],
    &[
        ("has", 1.0558331),
        ("multilabel", 2.4177341),
        ("node", 0.25848338),
        ("types", 0.8919033),
    ],
    &[
        ("has", 1.0558331),
        ("node", 0.25848338),
        ("types", 0.8919033),
        ("unknown", 1.5088022),
    ],
    &[
        ("has", 1.0558331),
        ("known", 1.4685894),
        ("node", 0.25848338),
        ("types", 0.8919033),
    ],
    &[
        ("edge", 0.5503924),
        ("has", 1.0558331),
        ("types", 0.8919033),
        ("unknown", 1.5088022),
    ],
    &[
        ("edge", 0.5503924),
        ("has", 1.0558331),
        ("known", 1.4685894),
        ("types", 0.8919033),
    ],
    &[
        ("has", 1.0558331),
        ("homogeneous", 1.8410478),
        ("node", 0.25848338),
        ("types", 0.8919033),
    ],
    &[
        ("exclusively", 1.905929),
        ("has", 0.76271397),
        ("homogeneous", 1.3299384),
        ("node", 0.18672353),
        ("types", 0.6442942),
    ],
    &[
        ("has", 1.0558331),
        ("homogeneous", 1.8410478),
        ("node", 0.25848338),
        ("ontologies", 2.0771408),
    ],
    &[
        ("edge", 0.5503924),
        ("has", 1.0558331),
        ("homogeneous", 1.8410478),
        ("types", 0.8919033),
    ],
    &[
        ("has", 1.0558331),
        ("node", 0.25848338),
        ("singleton", 1.3232422),
        ("types", 0.8919033),
    ],
    &[
        ("has", 1.5375631),
        ("node", 0.3764179),
        ("oddities", 3.3091762),
    ],
    &[
        ("has", 1.0558331),
        ("node", 0.25848338),
        ("oddities", 2.2723868),
        ("types", 0.8919033),
    ],
    &[
        ("edge", 0.5503924),
        ("has", 1.0558331),
        ("singleton", 1.3232422),
        ("types", 0.8919033),
    ],
    &[
        ("edge", 0.5503924),
        ("has", 1.0558331),
        ("oddities", 2.2723868),
        ("types", 0.8919033),
    ],
    &[("is", 3.6062331), ("multigraph", 4.6741557)],
    &[
        ("has", 1.5375631),
        ("node", 0.3764179),
        ("ontologies", 3.0248482),
    ],
    &[
        ("has", 1.0558331),
        ("node", 0.25848338),
        ("ontologies", 2.0771408),
        ("unknown", 1.5088022),
    ],
    &[
        ("by", 0.655751),
        ("decreasing", 0.815903),
        ("degree", 0.49559894),
        ("has", 0.35630772),
        ("node", 0.087229334),
        ("nodes", 0.39618313),
        ("outbound", 0.7302174),
        ("sorted", 0.7302174),
    ],
    &[
        ("by", 1.0562819),
        ("has", 0.5739395),
        ("lexicographic", 1.3142543),
        ("nodes", 0.6381707),
        ("order", 1.3142543),
        ("sorted", 1.1762322),
    ],
    &[
        ("contains", 3.5208392),
        ("identity", 3.5208392),
        ("matrix", 2.508398),
    ],
    &[
        ("by", 0.655751),
        ("degree", 0.49559894),
        ("has", 0.35630772),
        ("increasing", 0.815903),
        ("node", 0.087229334),
        ("nodes", 0.39618313),
        ("outbound", 0.7302174),
        ("sorted", 0.7302174),
    ],
    &[
        ("destinations", 2.6383977),
        ("enabled", 2.1638255),
        ("has", 1.0558331),
        ("tradeoff", 2.1638255),
    ],
    &[
        ("enabled", 2.1638255),
        ("has", 1.0558331),
        ("sources", 2.6383977),
        ("tradeoff", 2.1638255),
    ],
    &[
        ("cumulative", 1.3142543),
        ("degrees", 0.9182598),
        ("enabled", 1.1762322),
        ("has", 0.5739395),
        ("node", 0.14050877),
        ("tradeoff", 1.1762322),
    ],
    &[
        ("degrees", 0.9182598),
        ("enabled", 1.1762322),
        ("has", 0.5739395),
        ("reciprocal", 1.1291114),
        ("sqrt", 1.1291114),
        ("tradeoff", 1.1762322),
    ],
    &[
        ("dendritic", 3.5208392),
        ("get", 0.24648933),
        ("trees", 3.5208392),
    ],
    &[
        ("closure", 3.842182),
        ("get", 0.24648933),
        ("transitive", 3.842182),
    ],
    &[
        ("all", 1.758517),
        ("get", 0.1692624),
        ("paths", 2.4177341),
        ("shortest", 1.6025684),
    ],
    &[
        ("all", 1.2703197),
        ("get", 0.12227197),
        ("paths", 1.7465258),
        ("shortest", 1.1576654),
        ("weighted", 0.8932659),
    ],
    &[
        ("edge", 0.43359637),
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("id", 0.31132117),
        ("unchecked", 0.3857241),
        ("weight", 0.7432631),
    ],
    &[
        ("edge", 0.23263143),
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("ids", 0.28629005),
        ("node", 0.1092518),
        ("unchecked", 0.3857241),
        ("weight", 0.7432631),
    ],
    &[
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("id", 0.31132117),
        ("name", 0.4341876),
        ("node", 0.20363192),
        ("unchecked", 0.3857241),
    ],
    &[
        ("edge", 0.28934708),
        ("from", 0.12558359),
        ("get", 0.046608403),
        ("id", 0.20282264),
        ("name", 0.2828689),
        ("type", 0.37460184),
        ("unchecked", 0.2512954),
    ],
    &[
        ("edge", 0.28934708),
        ("from", 0.12558359),
        ("get", 0.046608403),
        ("id", 0.20282264),
        ("name", 0.2828689),
        ("type", 0.37460184),
        ("unchecked", 0.2512954),
    ],
    &[
        ("count", 0.655751),
        ("edge", 0.35100976),
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("id", 0.24856652),
        ("type", 0.24046573),
        ("unchecked", 0.30797166),
    ],
    &[
        ("count", 0.655751),
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("id", 0.24856652),
        ("node", 0.16484638),
        ("type", 0.24046573),
        ("unchecked", 0.30797166),
    ],
    &[
        ("and", 0.309302),
        ("edge", 0.20559923),
        ("from", 0.08802282),
        ("get", 0.032668307),
        ("id", 0.27514508),
        ("ids", 0.13073035),
        ("node", 0.049888305),
        ("type", 0.13752745),
        ("unchecked", 0.17613551),
    ],
    &[
        ("edge", 0.18573868),
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("ids", 0.43197346),
        ("minmax", 0.7302174),
        ("node", 0.087229334),
        ("unchecked", 0.30797166),
    ],
    &[
        ("edge", 0.23263143),
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("id", 0.31132117),
        ("ids", 0.28629005),
        ("node", 0.1092518),
        ("unchecked", 0.3857241),
    ],
    &[
        ("edge", 0.23263143),
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("id", 0.31132117),
        ("names", 0.374853),
        ("node", 0.1092518),
        ("unchecked", 0.3857241),
    ],
    &[
        ("edge", 0.18573868),
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("id", 0.4697421),
        ("node", 0.087229334),
        ("source", 0.5812845),
        ("unchecked", 0.30797166),
    ],
    &[
        ("destination", 0.6375051),
        ("edge", 0.18573868),
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("id", 0.4697421),
        ("node", 0.087229334),
        ("unchecked", 0.30797166),
    ],
    &[
        ("edge", 0.23263143),
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("id", 0.5802644),
        ("node", 0.1092518),
        ("source", 0.7280392),
    ],
    &[
        ("destination", 0.7984536),
        ("edge", 0.23263143),
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("id", 0.5802644),
        ("node", 0.1092518),
    ],
    &[
        ("edge", 0.18573868),
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("id", 0.24856652),
        ("name", 0.3466661),
        ("node", 0.087229334),
        ("source", 0.5812845),
        ("unchecked", 0.30797166),
    ],
    &[
        ("destination", 0.6375051),
        ("edge", 0.18573868),
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("id", 0.24856652),
        ("name", 0.3466661),
        ("node", 0.087229334),
        ("unchecked", 0.30797166),
    ],
    &[
        ("edge", 0.23263143),
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("id", 0.31132117),
        ("name", 0.4341876),
        ("node", 0.1092518),
        ("source", 0.7280392),
    ],
    &[
        ("destination", 0.7984536),
        ("edge", 0.23263143),
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("id", 0.31132117),
        ("name", 0.4341876),
        ("node", 0.1092518),
    ],
    &[
        ("edge", 0.29918736),
        ("from", 0.24791339),
        ("get", 0.09200922),
        ("id", 0.40039027),
        ("names", 0.48209855),
        ("node", 0.14050877),
    ],
    &[
        ("edge", 0.29918736),
        ("from", 0.24791339),
        ("get", 0.09200922),
        ("id", 0.40039027),
        ("ids", 0.36819774),
        ("node", 0.14050877),
    ],
    &[
        ("edge", 0.23263143),
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("id", 0.31132117),
        ("ids", 0.28629005),
        ("node", 0.1092518),
        ("unchecked", 0.3857241),
    ],
    &[
        ("edge", 0.29918736),
        ("from", 0.24791339),
        ("get", 0.09200922),
        ("id", 0.40039027),
        ("ids", 0.36819774),
        ("node", 0.14050877),
    ],
    &[
        ("get", 0.09200922),
        ("id", 0.40039027),
        ("node", 0.14050877),
        ("source", 0.9363315),
        ("unchecked", 0.4960799),
        ("unique", 0.955911),
    ],
    &[
        ("and", 0.309302),
        ("edge", 0.20559923),
        ("from", 0.08802282),
        ("get", 0.032668307),
        ("id", 0.27514508),
        ("ids", 0.13073035),
        ("node", 0.049888305),
        ("type", 0.13752745),
        ("unchecked", 0.17613551),
    ],
    &[
        ("and", 0.366644),
        ("edge", 0.24226607),
        ("from", 0.10434151),
        ("get", 0.038724735),
        ("id", 0.32421482),
        ("ids", 0.15496665),
        ("node", 0.059137177),
        ("type", 0.16302387),
    ],
    &[
        ("and", 0.3903448),
        ("edge", 0.19694783),
        ("from", 0.05673716),
        ("get", 0.021057118),
        ("id", 0.1794091),
        ("ids", 0.084265284),
        ("node", 0.03215667),
        ("type", 0.088646516),
        ("unchecked", 0.113532245),
        ("weight", 0.21876863),
    ],
    &[
        ("and", 0.44591957),
        ("edge", 0.22432975),
        ("from", 0.065014005),
        ("get", 0.024128942),
        ("id", 0.20495223),
        ("ids", 0.09655795),
        ("node", 0.036847703),
        ("type", 0.10157832),
        ("weight", 0.2506827),
    ],
    &[
        ("central", 1.1291114),
        ("get", 0.09200922),
        ("ids", 0.36819774),
        ("k", 1.0562819),
        ("node", 0.14050877),
        ("top", 1.235245),
    ],
    &[
        ("central", 0.87793416),
        ("get", 0.07154124),
        ("ids", 0.28629005),
        ("k", 0.821306),
        ("node", 0.1092518),
        ("top", 0.9604577),
        ("weighted", 0.5226492),
    ],
    &[
        ("degree", 0.620721),
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("id", 0.31132117),
        ("node", 0.20363192),
        ("unchecked", 0.3857241),
    ],
    &[
        ("degree", 0.49559894),
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("id", 0.24856652),
        ("node", 0.16484638),
        ("unchecked", 0.30797166),
        ("weighted", 0.41729605),
    ],
    &[
        ("degree", 0.79830945),
        ("from", 0.24791339),
        ("get", 0.09200922),
        ("id", 0.40039027),
        ("node", 0.25688884),
    ],
    &[
        ("comulative", 0.815903),
        ("degree", 0.49559894),
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("id", 0.24856652),
        ("node", 0.16484638),
        ("unchecked", 0.30797166),
    ],
    &[
        ("comulative", 1.021891),
        ("degree", 0.620721),
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("id", 0.31132117),
        ("node", 0.20363192),
    ],
    &[
        ("degree", 0.49559894),
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("id", 0.24856652),
        ("node", 0.087229334),
        ("reciprocal", 0.70096433),
        ("sqrt", 0.70096433),
        ("unchecked", 0.30797166),
    ],
    &[
        ("degree", 0.620721),
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("id", 0.31132117),
        ("node", 0.1092518),
        ("reciprocal", 0.87793416),
        ("sqrt", 0.87793416),
    ],
    &[
        ("degrees", 0.5700654),
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("ids", 0.22858106),
        ("node", 0.087229334),
        ("reciprocal", 0.70096433),
        ("sqrt", 0.70096433),
        ("unchecked", 0.30797166),
    ],
    &[
        ("degree", 0.620721),
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("id", 0.31132117),
        ("node", 0.20363192),
        ("weighted", 0.5226492),
    ],
    &[
        ("degree", 0.79830945),
        ("from", 0.24791339),
        ("get", 0.09200922),
        ("name", 0.5584088),
        ("node", 0.25688884),
    ],
    &[
        ("central", 1.1291114),
        ("get", 0.09200922),
        ("k", 1.0562819),
        ("names", 0.48209855),
        ("node", 0.14050877),
        ("top", 1.235245),
    ],
    &[
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("id", 0.24856652),
        ("ids", 0.22858106),
        ("node", 0.16484638),
        ("type", 0.24046573),
        ("unchecked", 0.30797166),
    ],
    &[
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("id", 0.31132117),
        ("ids", 0.28629005),
        ("node", 0.20363192),
        ("type", 0.3011752),
    ],
    &[
        ("edge", 0.35100976),
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("id", 0.4697421),
        ("type", 0.24046573),
        ("unchecked", 0.30797166),
    ],
    &[
        ("edge", 0.43359637),
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("id", 0.5802644),
        ("type", 0.3011752),
    ],
    &[
        ("edge", 0.35100976),
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("id", 0.24856652),
        ("ids", 0.22858106),
        ("node", 0.087229334),
        ("type", 0.24046573),
    ],
    &[
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("id", 0.24856652),
        ("names", 0.29929188),
        ("node", 0.16484638),
        ("type", 0.24046573),
        ("unchecked", 0.30797166),
    ],
    &[
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("id", 0.31132117),
        ("names", 0.374853),
        ("node", 0.20363192),
        ("type", 0.3011752),
    ],
    &[
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("name", 0.4341876),
        ("names", 0.374853),
        ("node", 0.20363192),
        ("type", 0.3011752),
    ],
    &[
        ("edge", 0.43359637),
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("id", 0.31132117),
        ("name", 0.4341876),
        ("type", 0.3011752),
    ],
    &[
        ("edge", 0.35100976),
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("id", 0.24856652),
        ("name", 0.3466661),
        ("type", 0.45443317),
    ],
    &[
        ("edge", 0.54699713),
        ("from", 0.24791339),
        ("get", 0.09200922),
        ("id", 0.40039027),
        ("weight", 0.955911),
    ],
    &[
        ("edge", 0.29918736),
        ("from", 0.24791339),
        ("get", 0.09200922),
        ("ids", 0.36819774),
        ("node", 0.14050877),
        ("weight", 0.955911),
    ],
    &[
        ("and", 0.366644),
        ("edge", 0.24226607),
        ("from", 0.10434151),
        ("get", 0.038724735),
        ("id", 0.1685158),
        ("ids", 0.15496665),
        ("node", 0.059137177),
        ("type", 0.16302387),
        ("weight", 0.4023227),
    ],
    &[
        ("and", 0.366644),
        ("edge", 0.24226607),
        ("from", 0.10434151),
        ("get", 0.038724735),
        ("name", 0.23502247),
        ("names", 0.2029051),
        ("node", 0.059137177),
        ("type", 0.16302387),
        ("weight", 0.4023227),
    ],
    &[
        ("edge", 0.29918736),
        ("from", 0.24791339),
        ("get", 0.09200922),
        ("names", 0.48209855),
        ("node", 0.14050877),
        ("weight", 0.955911),
    ],
    &[
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("id", 0.31132117),
        ("name", 0.4341876),
        ("node", 0.20363192),
        ("unchecked", 0.3857241),
    ],
    &[
        ("from", 0.24791339),
        ("get", 0.09200922),
        ("id", 0.40039027),
        ("name", 0.5584088),
        ("node", 0.25688884),
    ],
    &[
        ("from", 0.24791339),
        ("get", 0.09200922),
        ("id", 0.40039027),
        ("name", 0.5584088),
        ("node", 0.25688884),
    ],
    &[
        ("from", 0.24791339),
        ("get", 0.09200922),
        ("ids", 0.36819774),
        ("names", 0.48209855),
        ("node", 0.25688884),
    ],
    &[
        ("from", 0.24791339),
        ("get", 0.09200922),
        ("ids", 0.36819774),
        ("names", 0.48209855),
        ("node", 0.25688884),
    ],
    &[
        ("edge", 0.35100976),
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("ids", 0.22858106),
        ("names", 0.29929188),
        ("node", 0.16484638),
    ],
    &[
        ("edge", 0.35100976),
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("ids", 0.22858106),
        ("names", 0.29929188),
        ("node", 0.16484638),
    ],
    &[
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("ids", 0.28629005),
        ("name", 0.4341876),
        ("node", 0.20363192),
        ("type", 0.3011752),
    ],
    &[
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("name", 0.8092723),
        ("node", 0.20363192),
        ("type", 0.3011752),
    ],
    &[
        ("count", 0.821306),
        ("edge", 0.43359637),
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("id", 0.31132117),
        ("type", 0.3011752),
    ],
    &[
        ("edge", 0.35100976),
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("id", 0.24856652),
        ("name", 0.3466661),
        ("type", 0.45443317),
    ],
    &[
        ("count", 0.821306),
        ("edge", 0.43359637),
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("name", 0.4341876),
        ("type", 0.3011752),
    ],
    &[
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("id", 0.24856652),
        ("name", 0.3466661),
        ("node", 0.16484638),
        ("type", 0.45443317),
    ],
    &[
        ("count", 0.821306),
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("id", 0.31132117),
        ("node", 0.20363192),
        ("type", 0.3011752),
    ],
    &[
        ("count", 0.821306),
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("name", 0.4341876),
        ("node", 0.20363192),
        ("type", 0.3011752),
    ],
    &[
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("id", 0.31132117),
        ("ids", 0.28629005),
        ("neighbour", 0.9604577),
        ("node", 0.20363192),
    ],
    &[
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("ids", 0.28629005),
        ("name", 0.4341876),
        ("neighbour", 0.9604577),
        ("node", 0.20363192),
    ],
    &[
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("name", 0.4341876),
        ("names", 0.374853),
        ("neighbour", 0.9604577),
        ("node", 0.20363192),
    ],
    &[
        ("edge", 0.23263143),
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("ids", 0.53360945),
        ("minmax", 0.91457266),
        ("node", 0.1092518),
    ],
    &[
        ("and", 0.366644),
        ("edge", 0.24226607),
        ("from", 0.10434151),
        ("get", 0.038724735),
        ("id", 0.32421482),
        ("ids", 0.15496665),
        ("node", 0.059137177),
        ("type", 0.16302387),
    ],
    &[
        ("edge", 0.29918736),
        ("from", 0.24791339),
        ("get", 0.09200922),
        ("id", 0.40039027),
        ("names", 0.48209855),
        ("node", 0.14050877),
    ],
    &[
        ("and", 0.366644),
        ("edge", 0.24226607),
        ("from", 0.10434151),
        ("get", 0.038724735),
        ("id", 0.1685158),
        ("name", 0.23502247),
        ("names", 0.2029051),
        ("node", 0.059137177),
        ("type", 0.16302387),
    ],
    &[
        ("edge", 0.35100976),
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("ids", 0.22858106),
        ("names", 0.29929188),
        ("type", 0.45443317),
    ],
    &[
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("ids", 0.22858106),
        ("names", 0.29929188),
        ("node", 0.16484638),
        ("type", 0.45443317),
    ],
    &[
        ("from", 0.12558359),
        ("get", 0.046608403),
        ("ids", 0.18651511),
        ("multiple", 0.7265141),
        ("names", 0.24421299),
        ("node", 0.13588743),
        ("type", 0.37460184),
    ],
    &[
        ("edge", 0.15155706),
        ("from", 0.12558359),
        ("get", 0.046608403),
        ("id", 0.20282264),
        ("ids", 0.18651511),
        ("minmax", 0.595835),
        ("node", 0.07117646),
        ("source", 0.4743103),
        ("unchecked", 0.2512954),
    ],
    &[
        ("edge", 0.18573868),
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("id", 0.24856652),
        ("ids", 0.22858106),
        ("minmax", 0.7302174),
        ("node", 0.087229334),
        ("source", 0.5812845),
    ],
    &[
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("id", 0.24856652),
        ("name", 0.3466661),
        ("node", 0.16484638),
        ("type", 0.45443317),
    ],
    &[
        ("from", 0.12558359),
        ("get", 0.046608403),
        ("ids", 0.18651511),
        ("names", 0.24421299),
        ("node", 0.13588743),
        ("type", 0.37460184),
        ("unchecked", 0.2512954),
    ],
    &[
        ("from", 0.12558359),
        ("get", 0.046608403),
        ("id", 0.20282264),
        ("node", 0.07117646),
        ("nodes", 0.32327324),
        ("number", 0.30822805),
        ("of", 0.31804386),
        ("type", 0.19621265),
        ("unchecked", 0.2512954),
    ],
    &[
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("id", 0.24856652),
        ("node", 0.087229334),
        ("nodes", 0.39618313),
        ("number", 0.3777447),
        ("of", 0.38977432),
        ("type", 0.24046573),
    ],
    &[
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("name", 0.3466661),
        ("node", 0.087229334),
        ("nodes", 0.39618313),
        ("number", 0.3777447),
        ("of", 0.38977432),
        ("type", 0.24046573),
    ],
    &[
        ("edge", 0.15155706),
        ("edges", 0.399221),
        ("from", 0.12558359),
        ("get", 0.046608403),
        ("id", 0.20282264),
        ("number", 0.30822805),
        ("of", 0.31804386),
        ("type", 0.19621265),
        ("unchecked", 0.2512954),
    ],
    &[
        ("edge", 0.18573868),
        ("edges", 0.48925987),
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("id", 0.24856652),
        ("number", 0.3777447),
        ("of", 0.38977432),
        ("type", 0.24046573),
    ],
    &[
        ("edge", 0.18573868),
        ("edges", 0.48925987),
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("name", 0.3466661),
        ("number", 0.3777447),
        ("of", 0.38977432),
        ("type", 0.24046573),
    ],
    &[
        ("counts", 0.45870927),
        ("from", 0.10434151),
        ("get", 0.038724735),
        ("hashmap", 0.45870927),
        ("id", 0.1685158),
        ("ids", 0.15496665),
        ("node", 0.113776565),
        ("type", 0.16302387),
        ("unchecked", 0.20878954),
    ],
    &[
        ("counts", 0.45870927),
        ("edge", 0.12592164),
        ("from", 0.10434151),
        ("get", 0.038724735),
        ("hashmap", 0.45870927),
        ("id", 0.1685158),
        ("ids", 0.15496665),
        ("node", 0.059137177),
        ("type", 0.16302387),
        ("unchecked", 0.20878954),
    ],
    &[
        ("edge", 0.35100976),
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("id", 0.24856652),
        ("ids", 0.22858106),
        ("node", 0.087229334),
        ("type", 0.24046573),
    ],
    &[
        ("directed", 0.37627524),
        ("edge", 0.28934708),
        ("from", 0.12558359),
        ("get", 0.046608403),
        ("id", 0.20282264),
        ("ids", 0.18651511),
        ("node", 0.07117646),
        ("type", 0.19621265),
    ],
    &[
        ("directed", 0.37627524),
        ("edge", 0.28934708),
        ("from", 0.12558359),
        ("get", 0.046608403),
        ("id", 0.20282264),
        ("names", 0.24421299),
        ("node", 0.07117646),
        ("type", 0.19621265),
    ],
    &[
        ("directed", 0.37627524),
        ("edge", 0.28934708),
        ("from", 0.12558359),
        ("get", 0.046608403),
        ("name", 0.2828689),
        ("names", 0.24421299),
        ("node", 0.07117646),
        ("type", 0.19621265),
    ],
    &[
        ("directed", 0.461139),
        ("edge", 0.35100976),
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("id", 0.24856652),
        ("ids", 0.22858106),
        ("type", 0.24046573),
    ],
    &[
        ("edge", 0.35100976),
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("ids", 0.22858106),
        ("name", 0.3466661),
        ("node", 0.087229334),
        ("type", 0.24046573),
    ],
    &[
        ("directed", 0.37627524),
        ("edge", 0.28934708),
        ("from", 0.12558359),
        ("get", 0.046608403),
        ("ids", 0.18651511),
        ("name", 0.2828689),
        ("node", 0.07117646),
        ("type", 0.19621265),
    ],
    &[
        ("directed", 0.461139),
        ("edge", 0.35100976),
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("ids", 0.22858106),
        ("name", 0.3466661),
        ("type", 0.24046573),
    ],
    &[
        ("curie", 0.5350726),
        ("directed", 0.37627524),
        ("edge", 0.15155706),
        ("from", 0.12558359),
        ("get", 0.046608403),
        ("names", 0.24421299),
        ("node", 0.13588743),
        ("prefixes", 0.45665586),
    ],
    &[
        ("curie", 0.5350726),
        ("directed", 0.37627524),
        ("edge", 0.15155706),
        ("from", 0.12558359),
        ("get", 0.046608403),
        ("ids", 0.18651511),
        ("node", 0.13588743),
        ("prefixes", 0.45665586),
    ],
    &[
        ("curie", 0.655751),
        ("directed", 0.461139),
        ("edge", 0.18573868),
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("ids", 0.22858106),
        ("node", 0.087229334),
        ("prefixes", 0.5596484),
    ],
    &[
        ("curie", 0.5350726),
        ("directed", 0.37627524),
        ("edges", 0.399221),
        ("from", 0.12558359),
        ("get", 0.046608403),
        ("node", 0.07117646),
        ("number", 0.30822805),
        ("of", 0.31804386),
        ("prefixes", 0.45665586),
    ],
    &[
        ("curie", 0.821306),
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("ids", 0.28629005),
        ("node", 0.20363192),
        ("prefixes", 0.70094067),
    ],
    &[
        ("curie", 0.821306),
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("names", 0.374853),
        ("node", 0.20363192),
        ("prefixes", 0.70094067),
    ],
    &[
        ("curie", 0.655751),
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("node", 0.087229334),
        ("nodes", 0.39618313),
        ("number", 0.3777447),
        ("of", 0.38977432),
        ("prefixes", 0.5596484),
    ],
    &[
        ("get", 0.1692624),
        ("names", 0.8868802),
        ("node", 0.25848338),
        ("prefixes", 1.6583847),
    ],
    &[
        ("from", 0.24791339),
        ("get", 0.09200922),
        ("graph", 0.71196157),
        ("ids", 0.36819774),
        ("mapping", 1.235245),
        ("node", 0.14050877),
    ],
    &[
        ("degrees", 0.9182598),
        ("get", 0.09200922),
        ("node", 0.14050877),
        ("non", 1.4342046),
        ("subgraph", 1.3142543),
        ("zero", 1.4342046),
    ],
    &[
        ("edge", 0.23263143),
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("ids", 0.53360945),
        ("multigraph", 0.87793416),
        ("node", 0.1092518),
    ],
    &[
        ("edges", 0.48925987),
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("ids", 0.22858106),
        ("multigraph", 0.70096433),
        ("node", 0.087229334),
        ("number", 0.3777447),
        ("of", 0.38977432),
    ],
    &[
        ("ancestors", 1.0898843),
        ("from", 0.24791339),
        ("get", 0.09200922),
        ("ids", 0.36819774),
        ("jaccard", 0.97727257),
        ("node", 0.14050877),
    ],
    &[
        ("ancestors", 1.0898843),
        ("from", 0.24791339),
        ("get", 0.09200922),
        ("jaccard", 0.97727257),
        ("names", 0.48209855),
        ("node", 0.14050877),
    ],
    &[("get", 0.38088834), ("tendrils", 5.440587)],
    &[
        ("degree", 0.79830945),
        ("distribution", 1.4342046),
        ("geometric", 1.4342046),
        ("get", 0.09200922),
        ("node", 0.14050877),
        ("threshold", 1.4342046),
    ],
    &[
        ("edge", 0.3975931),
        ("get", 0.12227197),
        ("methods", 1.6415294),
        ("sparse", 1.905929),
        ("weighting", 1.7465258),
    ],
    &[
        ("edge", 0.5503924),
        ("get", 0.1692624),
        ("methods", 2.2723868),
        ("weighting", 2.4177341),
    ],
    &[("add", 4.372665), ("selfloops", 3.8761091)],
    &[
        ("centrality", 2.261778),
        ("degree", 2.1386418),
        ("get", 0.24648933),
    ],
    &[
        ("centrality", 1.5531461),
        ("degree", 1.4685894),
        ("get", 0.1692624),
        ("weighted", 1.2365575),
    ],
    &[
        ("centrality", 0.6564601),
        ("closeness", 0.91457266),
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("id", 0.31132117),
        ("node", 0.1092518),
        ("unchecked", 0.3857241),
    ],
    &[
        ("centrality", 0.524134),
        ("closeness", 0.7302174),
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("id", 0.24856652),
        ("node", 0.087229334),
        ("unchecked", 0.30797166),
        ("weighted", 0.41729605),
    ],
    &[
        ("centrality", 2.261778),
        ("closeness", 3.1510832),
        ("get", 0.24648933),
    ],
    &[
        ("centrality", 1.5531461),
        ("closeness", 2.1638255),
        ("get", 0.1692624),
        ("weighted", 1.2365575),
    ],
    &[
        ("centrality", 0.6564601),
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("harmonic", 0.91457266),
        ("id", 0.31132117),
        ("node", 0.1092518),
        ("unchecked", 0.3857241),
    ],
    &[
        ("centrality", 0.524134),
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("harmonic", 0.7302174),
        ("id", 0.24856652),
        ("node", 0.087229334),
        ("unchecked", 0.30797166),
        ("weighted", 0.41729605),
    ],
    &[
        ("centrality", 2.261778),
        ("get", 0.24648933),
        ("harmonic", 3.1510832),
    ],
    &[
        ("centrality", 1.5531461),
        ("get", 0.1692624),
        ("harmonic", 2.1638255),
        ("weighted", 1.2365575),
    ],
    &[
        ("centrality", 2.261778),
        ("get", 0.24648933),
        ("stress", 3.842182),
    ],
    &[
        ("betweenness", 3.0248482),
        ("centrality", 2.261778),
        ("get", 0.24648933),
    ],
    &[
        ("approximated", 0.75987273),
        ("betweenness", 0.87793416),
        ("centrality", 0.6564601),
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("id", 0.31132117),
        ("node", 0.1092518),
    ],
    &[
        ("approximated", 0.75987273),
        ("betweenness", 0.87793416),
        ("centrality", 0.6564601),
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("name", 0.4341876),
        ("node", 0.1092518),
    ],
    &[
        ("approximated", 0.60670114),
        ("betweenness", 0.70096433),
        ("centrality", 0.524134),
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("id", 0.24856652),
        ("node", 0.087229334),
        ("weighted", 0.41729605),
    ],
    &[
        ("approximated", 0.60670114),
        ("betweenness", 0.70096433),
        ("centrality", 0.524134),
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("name", 0.3466661),
        ("node", 0.087229334),
        ("weighted", 0.41729605),
    ],
    &[
        ("centrality", 2.261778),
        ("eigenvector", 3.5208392),
        ("get", 0.24648933),
    ],
    &[
        ("centrality", 1.5531461),
        ("eigenvector", 2.4177341),
        ("get", 0.1692624),
        ("weighted", 1.2365575),
    ],
    &[("dot", 5.937143), ("to", 3.8012981)],
    &[("get", 0.38088834), ("stars", 5.937143)],
    &[
        ("community", 1.6415294),
        ("detection", 1.905929),
        ("get", 0.12227197),
        ("louvain", 1.905929),
        ("undirected", 1.3646464),
    ],
    &[
        ("community", 0.9604577),
        ("directed", 0.577561),
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("memberships", 1.021891),
        ("modularity", 1.021891),
        ("node", 0.1092518),
    ],
    &[
        ("community", 0.9604577),
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("memberships", 1.021891),
        ("modularity", 1.021891),
        ("node", 0.1092518),
        ("undirected", 0.7984536),
    ],
    &[
        ("distance", 1.1151576),
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("ids", 0.28629005),
        ("node", 0.1092518),
        ("structural", 1.021891),
        ("unchecked", 0.3857241),
    ],
    &[
        ("attachment", 1.2703197),
        ("get", 0.12227197),
        ("minimum", 1.4037036),
        ("preferential", 1.2703197),
        ("unchecked", 0.65924555),
    ],
    &[
        ("attachment", 1.2703197),
        ("get", 0.12227197),
        ("maximum", 1.3646464),
        ("preferential", 1.2703197),
        ("unchecked", 0.65924555),
    ],
    &[
        ("attachment", 0.955911),
        ("get", 0.09200922),
        ("minimum", 1.0562819),
        ("preferential", 0.955911),
        ("unchecked", 0.4960799),
        ("weighted", 0.67217934),
    ],
    &[
        ("attachment", 0.955911),
        ("get", 0.09200922),
        ("maximum", 1.0268915),
        ("preferential", 0.955911),
        ("unchecked", 0.4960799),
        ("weighted", 0.67217934),
    ],
    &[
        ("attachment", 0.7432631),
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("ids", 0.28629005),
        ("node", 0.1092518),
        ("preferential", 0.7432631),
        ("unchecked", 0.3857241),
    ],
    &[
        ("attachment", 0.955911),
        ("from", 0.24791339),
        ("get", 0.09200922),
        ("ids", 0.36819774),
        ("node", 0.14050877),
        ("preferential", 0.955911),
    ],
    &[
        ("attachment", 0.955911),
        ("from", 0.24791339),
        ("get", 0.09200922),
        ("names", 0.48209855),
        ("node", 0.14050877),
        ("preferential", 0.955911),
    ],
    &[
        ("attachment", 0.59343964),
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("ids", 0.22858106),
        ("node", 0.087229334),
        ("preferential", 0.59343964),
        ("unchecked", 0.30797166),
        ("weighted", 0.41729605),
    ],
    &[
        ("attachment", 0.7432631),
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("ids", 0.28629005),
        ("node", 0.1092518),
        ("preferential", 0.7432631),
        ("weighted", 0.5226492),
    ],
    &[
        ("attachment", 0.7432631),
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("names", 0.374853),
        ("node", 0.1092518),
        ("preferential", 0.7432631),
        ("weighted", 0.5226492),
    ],
    &[
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("ids", 0.22858106),
        ("intersection", 0.76685315),
        ("neighbours", 0.76685315),
        ("node", 0.087229334),
        ("size", 0.70096433),
        ("unchecked", 0.30797166),
    ],
    &[
        ("coefficient", 0.821306),
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("ids", 0.28629005),
        ("jaccard", 0.75987273),
        ("node", 0.1092518),
        ("unchecked", 0.3857241),
    ],
    &[
        ("coefficient", 1.0562819),
        ("from", 0.24791339),
        ("get", 0.09200922),
        ("ids", 0.36819774),
        ("jaccard", 0.97727257),
        ("node", 0.14050877),
    ],
    &[
        ("coefficient", 1.0562819),
        ("from", 0.24791339),
        ("get", 0.09200922),
        ("jaccard", 0.97727257),
        ("names", 0.48209855),
        ("node", 0.14050877),
    ],
    &[
        ("adamic", 0.6766117),
        ("adar", 0.6766117),
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("ids", 0.22858106),
        ("index", 0.60670114),
        ("node", 0.087229334),
        ("unchecked", 0.30797166),
    ],
    &[
        ("adamic", 0.8474333),
        ("adar", 0.8474333),
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("ids", 0.28629005),
        ("index", 0.75987273),
        ("node", 0.1092518),
    ],
    &[
        ("adamic", 0.8474333),
        ("adar", 0.8474333),
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("index", 0.75987273),
        ("names", 0.374853),
        ("node", 0.1092518),
    ],
    &[
        ("allocation", 0.655751),
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("ids", 0.22858106),
        ("index", 0.60670114),
        ("node", 0.087229334),
        ("resource", 0.655751),
        ("unchecked", 0.30797166),
    ],
    &[
        ("allocation", 0.5350726),
        ("from", 0.12558359),
        ("get", 0.046608403),
        ("ids", 0.18651511),
        ("index", 0.4950495),
        ("node", 0.07117646),
        ("resource", 0.5350726),
        ("unchecked", 0.2512954),
        ("weighted", 0.34050074),
    ],
    &[
        ("allocation", 0.821306),
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("ids", 0.28629005),
        ("index", 0.75987273),
        ("node", 0.1092518),
        ("resource", 0.821306),
    ],
    &[
        ("allocation", 0.821306),
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("index", 0.75987273),
        ("names", 0.374853),
        ("node", 0.1092518),
        ("resource", 0.821306),
    ],
    &[
        ("allocation", 0.655751),
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("ids", 0.22858106),
        ("index", 0.60670114),
        ("node", 0.087229334),
        ("resource", 0.655751),
        ("weighted", 0.41729605),
    ],
    &[
        ("allocation", 0.655751),
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("index", 0.60670114),
        ("names", 0.29929188),
        ("node", 0.087229334),
        ("resource", 0.655751),
        ("weighted", 0.41729605),
    ],
    &[
        ("available", 1.3142543),
        ("edge", 0.29918736),
        ("get", 0.09200922),
        ("metrics", 1.0898843),
        ("number", 0.60847014),
        ("of", 0.62784743),
    ],
    &[
        ("available", 1.7465258),
        ("edge", 0.3975931),
        ("get", 0.12227197),
        ("metrics", 1.4483582),
        ("names", 0.64066565),
    ],
    &[
        ("all", 0.48422852),
        ("edge", 0.15155706),
        ("from", 0.12558359),
        ("get", 0.046608403),
        ("ids", 0.18651511),
        ("metrics", 0.5520944),
        ("node", 0.07117646),
        ("tuple", 0.66575176),
        ("unchecked", 0.2512954),
    ],
    &[
        ("all", 0.59343964),
        ("edge", 0.18573868),
        ("from", 0.15390725),
        ("get", 0.057120293),
        ("ids", 0.22858106),
        ("metrics", 0.6766117),
        ("node", 0.087229334),
        ("tuple", 0.815903),
    ],
    &[
        ("all", 0.7432631),
        ("edge", 0.23263143),
        ("from", 0.19276364),
        ("get", 0.07154124),
        ("ids", 0.28629005),
        ("metrics", 0.8474333),
        ("node", 0.1092518),
    ],
    &[
        ("attachment", 1.758517),
        ("get", 0.1692624),
        ("preferential", 1.758517),
        ("scores", 2.1638255),
    ],
    &[
        ("allocation", 1.4037036),
        ("get", 0.12227197),
        ("index", 1.2987074),
        ("resource", 1.4037036),
        ("scores", 1.5631068),
    ],
    &[
        ("coefficient", 1.9431618),
        ("get", 0.1692624),
        ("jaccard", 1.7978144),
        ("scores", 2.1638255),
    ],
    &[
        ("adamic", 2.0049777),
        ("adar", 2.0049777),
        ("get", 0.1692624),
        ("scores", 2.1638255),
    ],
    &[
        ("all", 1.758517),
        ("edge", 0.5503924),
        ("get", 0.1692624),
        ("metrics", 2.0049777),
    ],
    &[
        ("get", 0.12227197),
        ("groups", 1.2443004),
        ("ids", 0.48930165),
        ("isomorphic", 1.0899303),
        ("node", 0.18672353),
    ],
    &[
        ("get", 0.12227197),
        ("groups", 1.2443004),
        ("isomorphic", 1.0899303),
        ("names", 0.64066565),
        ("node", 0.18672353),
    ],
    &[
        ("get", 0.09200922),
        ("groups", 0.9363315),
        ("isomorphic", 0.8201686),
        ("node", 0.14050877),
        ("number", 0.60847014),
        ("of", 0.62784743),
    ],
    &[
        ("get", 0.09200922),
        ("groups", 0.9363315),
        ("ids", 0.36819774),
        ("isomorphic", 0.8201686),
        ("node", 0.14050877),
        ("type", 0.38734153),
    ],
    &[
        ("get", 0.09200922),
        ("groups", 0.9363315),
        ("isomorphic", 0.8201686),
        ("names", 0.48209855),
        ("node", 0.14050877),
        ("type", 0.38734153),
    ],
    &[
        ("get", 0.07154124),
        ("groups", 0.7280392),
        ("isomorphic", 0.6377174),
        ("node", 0.1092518),
        ("number", 0.47311246),
        ("of", 0.48817918),
        ("type", 0.3011752),
    ],
    &[
        ("approximated", 0.75987273),
        ("get", 0.07154124),
        ("groups", 0.7280392),
        ("ids", 0.28629005),
        ("isomorphic", 0.6377174),
        ("node", 0.1092518),
        ("type", 0.3011752),
    ],
    &[
        ("approximated", 0.75987273),
        ("get", 0.07154124),
        ("groups", 0.7280392),
        ("isomorphic", 0.6377174),
        ("names", 0.374853),
        ("node", 0.1092518),
        ("type", 0.3011752),
    ],
    &[
        ("approximated", 0.60670114),
        ("get", 0.057120293),
        ("groups", 0.5812845),
        ("isomorphic", 0.5091694),
        ("node", 0.087229334),
        ("number", 0.3777447),
        ("of", 0.38977432),
        ("type", 0.24046573),
    ],
    &[
        ("edge", 0.29918736),
        ("get", 0.09200922),
        ("groups", 0.9363315),
        ("ids", 0.36819774),
        ("isomorphic", 0.8201686),
        ("type", 0.38734153),
    ],
    &[
        ("edge", 0.29918736),
        ("get", 0.09200922),
        ("groups", 0.9363315),
        ("isomorphic", 0.8201686),
        ("names", 0.48209855),
        ("type", 0.38734153),
    ],
    &[
        ("edge", 0.23263143),
        ("get", 0.07154124),
        ("groups", 0.7280392),
        ("isomorphic", 0.6377174),
        ("number", 0.47311246),
        ("of", 0.48817918),
        ("type", 0.3011752),
    ],
    &[
        ("has", 1.5375631),
        ("isomorphic", 2.197202),
        ("nodes", 1.709636),
    ],
    &[
        ("from", 0.15390725),
        ("has", 0.35630772),
        ("ids", 0.22858106),
        ("isomorphic", 0.5091694),
        ("node", 0.16484638),
        ("types", 0.300987),
        ("unchecked", 0.30797166),
    ],
    &[
        ("from", 0.19276364),
        ("has", 0.4462634),
        ("ids", 0.28629005),
        ("isomorphic", 0.6377174),
        ("node", 0.20363192),
        ("types", 0.37697607),
    ],
    &[("csv", 5.937143), ("from", 1.0262811)],
];

#[pymethods]
impl Graph {
    fn _repr_html_(&self) -> String {
        self.__repr__()
    }
}

#[pymethods]
impl Graph {
    fn __str__(&self) -> String {
        self.inner.to_string()
    }
    fn __repr__(&self) -> String {
        self.__str__()
    }

    fn __hash__(&self) -> PyResult<isize> {
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
            "The method `{}` does not exists, did you mean one of the following?\n\n{}",
            &name,
            doc_scores
                .iter()
                .map(|(method_id, _)| {
                    format!("* `{}`", GRAPH_METHODS_NAMES[*method_id].to_string())
                })
                .take(10)
                .collect::<Vec<String>>()
                .join("\n"),
        )))
    }
}

#[pyclass(module = "ensmallen")]
///
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

impl<'a> From<&'a NodeTuple> for &'a graph::NodeTuple {
    fn from(val: &'a NodeTuple) -> &'a graph::NodeTuple {
        &val.inner
    }
}

#[pymethods]
impl NodeTuple {
    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the first node ID of the tuple
    pub fn get_root_node_id(&self) -> NodeT {
        self.inner.get_root_node_id().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the first node name of the tuple
    pub fn get_root_node_name(&self) -> String {
        self.inner.get_root_node_name().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return length of the tuple
    pub fn len(&self) -> NodeT {
        self.inner.len().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
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
        ("get", 0.1049044),
        ("id", 0.35410967),
        ("node", 0.1049044),
        ("root", 0.20386682),
    ],
    &[
        ("get", 0.1049044),
        ("name", 0.35410967),
        ("node", 0.1049044),
        ("root", 0.20386682),
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

#[pymethods]
impl NodeTuple {
    fn __str__(&self) -> String {
        self.inner.to_string()
    }
    fn __repr__(&self) -> String {
        self.__str__()
    }

    fn __hash__(&self) -> PyResult<isize> {
        let mut hasher = DefaultHasher::new();
        self.inner.hash(&mut hasher);
        Ok(hasher.finish() as isize)
    }

    fn __richcmp__(&self, other: Self, op: CompareOp) -> bool {
        match op {
            CompareOp::Lt => self.inner < other.inner,
            CompareOp::Le => self.inner <= other.inner,
            CompareOp::Eq => self.inner == other.inner,
            CompareOp::Ne => self.inner != other.inner,
            CompareOp::Gt => self.inner > other.inner,
            CompareOp::Ge => self.inner >= other.inner,
        }
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
            "The method `{}` does not exists, did you mean one of the following?\n\n{}",
            &name,
            doc_scores
                .iter()
                .map(|(method_id, _)| {
                    format!("* `{}`", NODETUPLE_METHODS_NAMES[*method_id].to_string())
                })
                .take(10)
                .collect::<Vec<String>>()
                .join("\n"),
        )))
    }
}

#[pyclass(module = "ensmallen")]
///
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

impl<'a> From<&'a ShortestPathsDjkstra> for &'a graph::ShortestPathsDjkstra {
    fn from(val: &'a ShortestPathsDjkstra) -> &'a graph::ShortestPathsDjkstra {
        &val.inner
    }
}

#[pymethods]
impl ShortestPathsDjkstra {
    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    ///
    pub fn has_path_to_node_id(&self, node_id: NodeT) -> PyResult<bool> {
        Ok(pe!(self.inner.has_path_to_node_id(node_id.clone()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    ///
    pub fn get_distance_from_node_id(&self, node_id: NodeT) -> PyResult<f32> {
        Ok(pe!(self.inner.get_distance_from_node_id(node_id.clone()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    ///
    pub fn get_parent_from_node_id(&self, node_id: NodeT) -> PyResult<Option<NodeT>> {
        Ok(pe!(self.inner.get_parent_from_node_id(node_id.clone()))?.map(|x| x.into()))
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, dst_node_id, distance)")]
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
        distance: f32,
    ) -> PyResult<NodeT> {
        Ok(pe!(self
            .inner
            .get_point_at_given_distance_on_shortest_path(dst_node_id.clone(), distance.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, dst_node_id)")]
    ///
    pub fn get_median_point(&self, dst_node_id: NodeT) -> PyResult<NodeT> {
        Ok(pe!(self.inner.get_median_point(dst_node_id.clone()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    ///
    pub fn get_eccentricity(&self) -> f32 {
        self.inner.get_eccentricity().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    ///
    pub fn get_most_distant_node(&self) -> NodeT {
        self.inner.get_most_distant_node().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Returns the number of shortest paths starting from the root node
    pub fn get_number_of_shortest_paths(&self) -> NodeT {
        self.inner.get_number_of_shortest_paths().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
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
            .get_number_of_shortest_paths_from_node_id(node_id.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, source_node_id)")]
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
                    .get_successors_from_node_id(source_node_id.clone()))?,
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
        ("has", 0.58600885),
        ("id", 0.20386681),
        ("node", 0.15473326),
        ("path", 0.435766),
        ("to", 0.58600885),
    ],
    &[
        ("distance", 0.435766),
        ("from", 0.2628876),
        ("get", 0.043118663),
        ("id", 0.20386681),
        ("node", 0.15473326),
    ],
    &[
        ("from", 0.2628876),
        ("get", 0.043118663),
        ("id", 0.20386681),
        ("node", 0.15473326),
        ("parent", 0.58600885),
    ],
    &[
        ("at", 0.2707106),
        ("distance", 0.20130494),
        ("get", 0.019918947),
        ("given", 0.2707106),
        ("on", 0.2707106),
        ("path", 0.20130494),
        ("point", 0.20130494),
        ("shortest", 0.15558861),
    ],
    &[
        ("get", 0.088315345),
        ("median", 1.2002592),
        ("point", 0.8925329),
    ],
    &[("eccentricity", 1.8796511), ("get", 0.13830516)],
    &[
        ("distant", 0.81656975),
        ("get", 0.060083386),
        ("most", 0.81656975),
        ("node", 0.21561193),
    ],
    &[
        ("get", 0.043118663),
        ("number", 0.435766),
        ("of", 0.435766),
        ("paths", 0.435766),
        ("shortest", 0.3368036),
    ],
    &[
        ("from", 0.12144262),
        ("get", 0.019918947),
        ("id", 0.094177596),
        ("node", 0.071480036),
        ("number", 0.20130494),
        ("of", 0.20130494),
        ("paths", 0.20130494),
        ("shortest", 0.15558861),
    ],
    &[
        ("from", 0.2628876),
        ("get", 0.043118663),
        ("id", 0.20386681),
        ("node", 0.15473326),
        ("successors", 0.58600885),
    ],
];

#[pymethods]
impl ShortestPathsDjkstra {
    fn _repr_html_(&self) -> String {
        self.__repr__()
    }
}

#[pymethods]
impl ShortestPathsDjkstra {
    fn __str__(&self) -> String {
        self.inner.to_string()
    }
    fn __repr__(&self) -> String {
        self.__str__()
    }

    fn __hash__(&self) -> PyResult<isize> {
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
            "The method `{}` does not exists, did you mean one of the following?\n\n{}",
            &name,
            doc_scores
                .iter()
                .map(|(method_id, _)| {
                    format!(
                        "* `{}`",
                        SHORTESTPATHSDJKSTRA_METHODS_NAMES[*method_id].to_string()
                    )
                })
                .take(10)
                .collect::<Vec<String>>()
                .join("\n"),
        )))
    }
}

#[pyclass(module = "ensmallen")]
///
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

impl<'a> From<&'a ShortestPathsResultBFS> for &'a graph::ShortestPathsResultBFS {
    fn from(val: &'a ShortestPathsResultBFS) -> &'a graph::ShortestPathsResultBFS {
        &val.inner
    }
}

#[pymethods]
impl ShortestPathsResultBFS {
    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    ///
    pub fn has_path_to_node_id(&self, node_id: NodeT) -> PyResult<bool> {
        Ok(pe!(self.inner.has_path_to_node_id(node_id.clone()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    ///
    pub fn get_distance_from_node_id(&self, node_id: NodeT) -> PyResult<NodeT> {
        Ok(pe!(self.inner.get_distance_from_node_id(node_id.clone()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, node_id)")]
    ///
    pub fn get_parent_from_node_id(&self, node_id: NodeT) -> PyResult<NodeT> {
        Ok(pe!(self.inner.get_parent_from_node_id(node_id.clone()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, dst_node_id, k)")]
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
            .get_unchecked_kth_point_on_shortest_path(dst_node_id.clone(), k.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, dst_node_id, k)")]
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
            .get_kth_point_on_shortest_path(dst_node_id.clone(), k.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, dst_node_id)")]
    ///
    pub fn get_median_point(&self, dst_node_id: NodeT) -> PyResult<NodeT> {
        Ok(pe!(self.inner.get_median_point(dst_node_id.clone()))?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    ///
    pub fn get_median_point_to_most_distant_node(&self) -> PyResult<NodeT> {
        Ok(pe!(self.inner.get_median_point_to_most_distant_node())?.into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    ///
    pub fn get_eccentricity(&self) -> NodeT {
        self.inner.get_eccentricity().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    ///
    pub fn get_most_distant_node(&self) -> NodeT {
        self.inner.get_most_distant_node().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
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
    #[pyo3(text_signature = "($self, node_id)")]
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
            .get_number_of_shortest_paths_from_node_id(node_id.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, source_node_id)")]
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
                    .get_successors_from_node_id(source_node_id.clone()))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, source_node_id)")]
    /// Return list of predecessors of a given node.
    ///
    /// Parameters
    /// ----------
    /// source_node_id: int
    ///     The node for which to return the predecessors.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given node ID does not exist in the graph.
    ///
    pub fn get_predecessors_from_node_id(
        &self,
        source_node_id: NodeT,
    ) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(
                gil,
                pe!(self
                    .inner
                    .get_predecessors_from_node_id(source_node_id.clone()))?,
                NodeT
            )
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, first_node_id, second_node_id)")]
    /// Return Shared Ancestors number.
    ///
    /// Parameters
    /// ----------
    /// first_node_id: int
    ///     The first node for which to compute the predecessors Jaccard index.
    /// second_node_id: int
    ///     The second node for which to compute the predecessors Jaccard index.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given node IDs do not exist in the graph.
    ///
    pub fn get_shared_ancestors_size(
        &self,
        first_node_id: NodeT,
        second_node_id: NodeT,
    ) -> PyResult<f32> {
        Ok(pe!(self
            .inner
            .get_shared_ancestors_size(first_node_id.clone(), second_node_id.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, first_node_id, second_node_id)")]
    /// Return Ancestors Jaccard Index.
    ///
    /// Parameters
    /// ----------
    /// first_node_id: int
    ///     The first node for which to compute the predecessors Jaccard index.
    /// second_node_id: int
    ///     The second node for which to compute the predecessors Jaccard index.
    ///
    ///
    /// Raises
    /// -------
    /// ValueError
    ///     If the given node IDs do not exist in the graph.
    ///
    pub fn get_ancestors_jaccard_index(
        &self,
        first_node_id: NodeT,
        second_node_id: NodeT,
    ) -> PyResult<f32> {
        Ok(pe!(self
            .inner
            .get_ancestors_jaccard_index(first_node_id.clone(), second_node_id.clone()))?
        .into())
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    ///
    pub fn get_distances(&self) -> PyResult<Py<PyArray1<NodeT>>> {
        Ok({
            let gil = pyo3::Python::acquire_gil();
            to_ndarray_1d!(gil, pe!(self.inner.get_distances())?, NodeT)
        })
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
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
    "get_predecessors_from_node_id",
    "get_shared_ancestors_size",
    "get_ancestors_jaccard_index",
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
    "predecessors",
    "shared",
    "ancestors",
    "size",
    "jaccard",
    "index",
    "distances",
];

pub const SHORTESTPATHSRESULTBFS_TFIDF_FREQUENCIES: &[&[(&str, f64)]] = &[
    &[
        ("has", 0.6958796),
        ("id", 0.2852428),
        ("node", 0.21011747),
        ("path", 0.4586001),
        ("to", 0.55282664),
    ],
    &[
        ("distance", 0.6958796),
        ("from", 0.33202505),
        ("get", 0.024366887),
        ("id", 0.2852428),
        ("node", 0.21011747),
    ],
    &[
        ("from", 0.33202505),
        ("get", 0.024366887),
        ("id", 0.2852428),
        ("node", 0.21011747),
        ("parent", 0.6958796),
    ],
    &[
        ("get", 0.014045565),
        ("kth", 0.31866044),
        ("on", 0.31866044),
        ("path", 0.26434636),
        ("point", 0.22377864),
        ("shortest", 0.22377864),
        ("unchecked", 0.40111902),
    ],
    &[
        ("get", 0.01818011),
        ("kth", 0.4124634),
        ("on", 0.4124634),
        ("path", 0.34216106),
        ("point", 0.28965157),
        ("shortest", 0.28965157),
    ],
    &[
        ("get", 0.05054337),
        ("median", 1.1467087),
        ("point", 0.8052739),
    ],
    &[
        ("distant", 0.31866044),
        ("get", 0.014045565),
        ("median", 0.31866044),
        ("most", 0.31866044),
        ("node", 0.12111595),
        ("point", 0.22377864),
        ("to", 0.31866044),
    ],
    &[("eccentricity", 2.2853043), ("get", 0.08002211)],
    &[
        ("distant", 0.7743416),
        ("get", 0.03413058),
        ("most", 0.7743416),
        ("node", 0.2943105),
    ],
    &[
        ("get", 0.024366887),
        ("number", 0.55282664),
        ("of", 0.55282664),
        ("paths", 0.55282664),
        ("shortest", 0.38822138),
    ],
    &[
        ("from", 0.15205239),
        ("get", 0.011158927),
        ("id", 0.13062824),
        ("node", 0.096224256),
        ("number", 0.25316948),
        ("of", 0.25316948),
        ("paths", 0.25316948),
        ("shortest", 0.17778775),
    ],
    &[
        ("from", 0.33202505),
        ("get", 0.024366887),
        ("id", 0.2852428),
        ("node", 0.21011747),
        ("successors", 0.6958796),
    ],
    &[
        ("from", 0.33202505),
        ("get", 0.024366887),
        ("id", 0.2852428),
        ("node", 0.21011747),
        ("predecessors", 0.55282664),
    ],
    &[
        ("ancestors", 0.7743416),
        ("get", 0.03413058),
        ("shared", 0.9747151),
        ("size", 0.9747151),
    ],
    &[
        ("ancestors", 0.7743416),
        ("get", 0.03413058),
        ("index", 0.9747151),
        ("jaccard", 0.9747151),
    ],
    &[("distances", 2.2853043), ("get", 0.08002211)],
    &[("get", 0.08002211), ("predecessors", 1.8155112)],
];

#[pymethods]
impl ShortestPathsResultBFS {
    fn _repr_html_(&self) -> String {
        self.__repr__()
    }
}

#[pymethods]
impl ShortestPathsResultBFS {
    fn __str__(&self) -> String {
        self.inner.to_string()
    }
    fn __repr__(&self) -> String {
        self.__str__()
    }

    fn __hash__(&self) -> PyResult<isize> {
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
            "The method `{}` does not exists, did you mean one of the following?\n\n{}",
            &name,
            doc_scores
                .iter()
                .map(|(method_id, _)| {
                    format!(
                        "* `{}`",
                        SHORTESTPATHSRESULTBFS_METHODS_NAMES[*method_id].to_string()
                    )
                })
                .take(10)
                .collect::<Vec<String>>()
                .join("\n"),
        )))
    }
}

#[pyclass(module = "ensmallen")]
///
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

impl<'a> From<&'a Star> for &'a graph::Star {
    fn from(val: &'a Star) -> &'a graph::Star {
        &val.inner
    }
}

#[pymethods]
impl Star {
    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the central node ID of the Star
    pub fn get_root_node_id(&self) -> NodeT {
        self.inner.get_root_node_id().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the central node name of the star
    pub fn get_root_node_name(&self) -> String {
        self.inner.get_root_node_name().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return length of the Star
    pub fn len(&self) -> NodeT {
        self.inner.len().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the node IDs of the nodes composing the Star
    pub fn get_star_node_ids(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_star_node_ids(), NodeT)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, k)")]
    /// Return the first `k` node IDs of the nodes composing the star.
    ///
    /// Parameters
    /// ----------
    ///
    pub fn get_first_k_star_node_ids(&self, k: usize) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_first_k_star_node_ids(k.clone()), NodeT)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, k)")]
    /// Return the first `k` node names of the nodes composing the star.
    ///
    /// Parameters
    /// ----------
    ///
    pub fn get_first_k_star_node_names(&self, k: usize) -> Vec<String> {
        self.inner
            .get_first_k_star_node_names(k.clone())
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
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
        ("id", 0.611402),
        ("node", 0.07583805),
        ("root", 0.42482838),
    ],
    &[
        ("get", 0.07583805),
        ("name", 0.611402),
        ("node", 0.07583805),
        ("root", 0.42482838),
    ],
    &[("len", 2.5416396)],
    &[
        ("get", 0.07583805),
        ("ids", 0.42482838),
        ("node", 0.07583805),
        ("star", 0.21014561),
    ],
    &[
        ("first", 0.22323874),
        ("get", 0.039851367),
        ("ids", 0.22323874),
        ("k", 0.22323874),
        ("node", 0.039851367),
        ("star", 0.110427275),
    ],
    &[
        ("first", 0.22323874),
        ("get", 0.039851367),
        ("k", 0.22323874),
        ("names", 0.22323874),
        ("node", 0.039851367),
        ("star", 0.110427275),
    ],
    &[
        ("get", 0.07583805),
        ("names", 0.42482838),
        ("node", 0.07583805),
        ("star", 0.21014561),
    ],
];

#[pymethods]
impl Star {
    fn _repr_html_(&self) -> String {
        self.__repr__()
    }
}

#[pymethods]
impl Star {
    fn __str__(&self) -> String {
        self.inner.to_string()
    }
    fn __repr__(&self) -> String {
        self.__str__()
    }

    fn __hash__(&self) -> PyResult<isize> {
        let mut hasher = DefaultHasher::new();
        self.inner.hash(&mut hasher);
        Ok(hasher.finish() as isize)
    }

    fn __richcmp__(&self, other: Self, op: CompareOp) -> bool {
        match op {
            CompareOp::Lt => self.inner < other.inner,
            CompareOp::Le => self.inner <= other.inner,
            CompareOp::Eq => self.inner == other.inner,
            CompareOp::Ne => self.inner != other.inner,
            CompareOp::Gt => self.inner > other.inner,
            CompareOp::Ge => self.inner >= other.inner,
        }
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
            "The method `{}` does not exists, did you mean one of the following?\n\n{}",
            &name,
            doc_scores
                .iter()
                .map(|(method_id, _)| {
                    format!("* `{}`", STAR_METHODS_NAMES[*method_id].to_string())
                })
                .take(10)
                .collect::<Vec<String>>()
                .join("\n"),
        )))
    }
}

#[pyclass(module = "ensmallen")]
///
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

impl<'a> From<&'a Tendril> for &'a graph::Tendril {
    fn from(val: &'a Tendril) -> &'a graph::Tendril {
        &val.inner
    }
}

#[pymethods]
impl Tendril {
    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the first node ID of the Tendril
    pub fn get_root_node_id(&self) -> NodeT {
        self.inner.get_root_node_id().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the first node name of the Tendril
    pub fn get_root_node_name(&self) -> String {
        self.inner.get_root_node_name().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return length of the Tendril
    pub fn len(&self) -> NodeT {
        self.inner.len().into()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
    /// Return the node IDs of the nodes composing the Tendril
    pub fn get_tendril_node_ids(&self) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(gil, self.inner.get_tendril_node_ids(), NodeT)
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, k)")]
    /// Return the first `k` node IDs of the nodes composing the Tendril.
    ///
    /// Parameters
    /// ----------
    ///
    pub fn get_first_k_tendril_node_ids(&self, k: usize) -> Py<PyArray1<NodeT>> {
        let gil = pyo3::Python::acquire_gil();
        to_ndarray_1d!(
            gil,
            self.inner.get_first_k_tendril_node_ids(k.clone()),
            NodeT
        )
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self, k)")]
    /// Return the first `k` node names of the nodes composing the Tendril.
    ///
    /// Parameters
    /// ----------
    ///
    pub fn get_first_k_tendril_node_names(&self, k: usize) -> Vec<String> {
        self.inner
            .get_first_k_tendril_node_names(k.clone())
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>()
    }

    #[automatically_generated_binding]
    #[pyo3(text_signature = "($self)")]
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
        ("id", 0.611402),
        ("node", 0.07583805),
        ("root", 0.42482838),
    ],
    &[
        ("get", 0.07583805),
        ("name", 0.611402),
        ("node", 0.07583805),
        ("root", 0.42482838),
    ],
    &[("len", 2.5416396)],
    &[
        ("get", 0.07583805),
        ("ids", 0.42482838),
        ("node", 0.07583805),
        ("tendril", 0.21014561),
    ],
    &[
        ("first", 0.22323874),
        ("get", 0.039851367),
        ("ids", 0.22323874),
        ("k", 0.22323874),
        ("node", 0.039851367),
        ("tendril", 0.110427275),
    ],
    &[
        ("first", 0.22323874),
        ("get", 0.039851367),
        ("k", 0.22323874),
        ("names", 0.22323874),
        ("node", 0.039851367),
        ("tendril", 0.110427275),
    ],
    &[
        ("get", 0.07583805),
        ("names", 0.42482838),
        ("node", 0.07583805),
        ("tendril", 0.21014561),
    ],
];

#[pymethods]
impl Tendril {
    fn _repr_html_(&self) -> String {
        self.__repr__()
    }
}

#[pymethods]
impl Tendril {
    fn __str__(&self) -> String {
        self.inner.to_string()
    }
    fn __repr__(&self) -> String {
        self.__str__()
    }

    fn __hash__(&self) -> PyResult<isize> {
        let mut hasher = DefaultHasher::new();
        self.inner.hash(&mut hasher);
        Ok(hasher.finish() as isize)
    }

    fn __richcmp__(&self, other: Self, op: CompareOp) -> bool {
        match op {
            CompareOp::Lt => self.inner < other.inner,
            CompareOp::Le => self.inner <= other.inner,
            CompareOp::Eq => self.inner == other.inner,
            CompareOp::Ne => self.inner != other.inner,
            CompareOp::Gt => self.inner > other.inner,
            CompareOp::Ge => self.inner >= other.inner,
        }
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
            "The method `{}` does not exists, did you mean one of the following?\n\n{}",
            &name,
            doc_scores
                .iter()
                .map(|(method_id, _)| {
                    format!("* `{}`", TENDRIL_METHODS_NAMES[*method_id].to_string())
                })
                .take(10)
                .collect::<Vec<String>>()
                .join("\n"),
        )))
    }
}

pub fn register_edge_list_utils(_py: Python, _m: &PyModule) -> PyResult<()> {
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
    _m.add_wrapped(wrap_pyfunction!(is_numeric_edge_list))?;
    _m.add_wrapped(wrap_pyfunction!(convert_node_list_node_types_to_numeric))?;
    _m.add_wrapped(wrap_pyfunction!(has_duplicated_edges_in_edge_list))?;
    _m.add_wrapped(wrap_pyfunction!(sort_numeric_edge_list))?;
    _m.add_wrapped(wrap_pyfunction!(sort_numeric_edge_list_inplace))?;
    _m.add_wrapped(wrap_pyfunction!(get_number_of_selfloops_from_edge_list))?;
    Ok(())
}

#[pyfunction]
#[automatically_generated_binding]
#[pyo3(
    text_signature = "(original_edge_path, target_edge_path, directed, original_node_path, original_node_list_separator, original_node_list_header, original_node_list_support_balanced_quotes, node_list_rows_to_skip, node_list_is_correct, node_list_max_rows_number, node_list_comment_symbol, original_nodes_column_number, original_nodes_column, nodes_number, original_minimum_node_id, original_numeric_node_ids, original_load_node_list_in_parallel, original_edge_type_path, original_edge_types_column_number, original_edge_types_column, edge_types_number, original_numeric_edge_type_ids, original_minimum_edge_type_id, original_edge_type_list_separator, original_edge_type_list_header, original_edge_type_list_support_balanced_quotes, edge_type_list_rows_to_skip, edge_type_list_is_correct, edge_type_list_max_rows_number, edge_type_list_comment_symbol, load_edge_type_list_in_parallel, original_edge_list_separator, original_edge_list_header, original_edge_list_support_balanced_quotes, original_sources_column_number, original_sources_column, original_destinations_column_number, original_destinations_column, original_edge_list_edge_types_column, original_edge_list_edge_types_column_number, original_weights_column, original_weights_column_number, target_edge_list_separator, target_edge_list_header, target_sources_column, target_sources_column_number, target_destinations_column, target_destinations_column_number, target_edge_list_edge_types_column, target_edge_list_edge_types_column_number, target_weights_column, target_weights_column_number, target_node_path, target_node_list_separator, target_node_list_header, target_nodes_column, target_nodes_column_number, target_edge_type_list_path, target_edge_type_list_separator, target_edge_type_list_header, target_edge_type_list_edge_types_column, target_edge_type_list_edge_types_column_number, remove_chevrons, remove_spaces, comment_symbol, default_edge_type, default_weight, max_rows_number, rows_to_skip, edges_number, skip_edge_types_if_unavailable, skip_weights_if_unavailable, numeric_rows_are_surely_smaller_than_original, verbose, name)"
)]
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
    original_edge_path: &str,
    target_edge_path: &str,
    directed: bool,
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
    remove_chevrons: Option<bool>,
    remove_spaces: Option<bool>,
    comment_symbol: Option<String>,
    default_edge_type: Option<String>,
    default_weight: Option<WeightT>,
    max_rows_number: Option<usize>,
    rows_to_skip: Option<usize>,
    edges_number: Option<usize>,
    skip_edge_types_if_unavailable: Option<bool>,
    skip_weights_if_unavailable: Option<bool>,
    numeric_rows_are_surely_smaller_than_original: Option<bool>,
    verbose: Option<bool>,
    name: Option<String>,
) -> PyResult<(NodeT, Option<EdgeTypeT>)> {
    Ok({
        let (subresult_0, subresult_1) = pe!(graph::convert_edge_list_to_numeric(
            original_edge_path,
            target_edge_path,
            directed.clone(),
            original_node_path,
            original_node_list_separator,
            original_node_list_header,
            original_node_list_support_balanced_quotes,
            node_list_rows_to_skip,
            node_list_is_correct,
            node_list_max_rows_number,
            node_list_comment_symbol,
            original_nodes_column_number,
            original_nodes_column,
            nodes_number,
            original_minimum_node_id,
            original_numeric_node_ids,
            original_load_node_list_in_parallel,
            original_edge_type_path,
            original_edge_types_column_number,
            original_edge_types_column,
            edge_types_number,
            original_numeric_edge_type_ids,
            original_minimum_edge_type_id,
            original_edge_type_list_separator,
            original_edge_type_list_header,
            original_edge_type_list_support_balanced_quotes,
            edge_type_list_rows_to_skip,
            edge_type_list_is_correct,
            edge_type_list_max_rows_number,
            edge_type_list_comment_symbol,
            load_edge_type_list_in_parallel,
            original_edge_list_separator,
            original_edge_list_header,
            original_edge_list_support_balanced_quotes,
            original_sources_column_number,
            original_sources_column,
            original_destinations_column_number,
            original_destinations_column,
            original_edge_list_edge_types_column,
            original_edge_list_edge_types_column_number,
            original_weights_column,
            original_weights_column_number,
            target_edge_list_separator,
            target_edge_list_header,
            target_sources_column,
            target_sources_column_number,
            target_destinations_column,
            target_destinations_column_number,
            target_edge_list_edge_types_column,
            target_edge_list_edge_types_column_number,
            target_weights_column,
            target_weights_column_number,
            target_node_path,
            target_node_list_separator,
            target_node_list_header,
            target_nodes_column,
            target_nodes_column_number,
            target_edge_type_list_path,
            target_edge_type_list_separator,
            target_edge_type_list_header,
            target_edge_type_list_edge_types_column,
            target_edge_type_list_edge_types_column_number,
            remove_chevrons,
            remove_spaces,
            comment_symbol,
            default_edge_type,
            default_weight,
            max_rows_number,
            rows_to_skip,
            edges_number,
            skip_edge_types_if_unavailable,
            skip_weights_if_unavailable,
            numeric_rows_are_surely_smaller_than_original,
            verbose,
            name
        ))?
        .into();
        (subresult_0.into(), subresult_1.map(|x| x.into()))
    })
}

#[pyfunction]
#[automatically_generated_binding]
#[pyo3(
    text_signature = "(original_edge_path, target_edge_path, directed, maximum_node_id, original_edge_list_separator, original_edge_list_header, original_sources_column, original_sources_column_number, original_destinations_column, original_destinations_column_number, original_edge_list_edge_types_column, original_edge_list_edge_types_column_number, original_weights_column, original_weights_column_number, original_edge_type_path, original_edge_types_column_number, original_edge_types_column, edge_types_number, original_numeric_edge_type_ids, original_minimum_edge_type_id, original_edge_type_list_separator, original_edge_type_list_header, edge_type_list_rows_to_skip, edge_type_list_is_correct, edge_type_list_max_rows_number, edge_type_list_comment_symbol, load_edge_type_list_in_parallel, target_edge_list_separator, target_edge_list_header, target_sources_column, target_sources_column_number, target_destinations_column, target_destinations_column_number, target_edge_list_edge_types_column, target_edge_list_edge_types_column_number, target_weights_column, target_weights_column_number, target_node_path, target_node_list_separator, target_node_list_header, target_nodes_column, target_nodes_column_number, target_edge_type_list_path, target_edge_type_list_separator, target_edge_type_list_header, target_edge_type_list_edge_types_column, target_edge_type_list_edge_types_column_number, comment_symbol, default_edge_type, default_weight, max_rows_number, rows_to_skip, edges_number, skip_edge_types_if_unavailable, skip_weights_if_unavailable, numeric_rows_are_surely_smaller_than_original, verbose, name)"
)]
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
    original_edge_path: &str,
    target_edge_path: &str,
    directed: bool,
    maximum_node_id: Option<EdgeT>,
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
    verbose: Option<bool>,
    name: Option<String>,
) -> PyResult<(NodeT, Option<EdgeTypeT>)> {
    Ok({
        let (subresult_0, subresult_1) = pe!(graph::densify_sparse_numeric_edge_list(
            original_edge_path,
            target_edge_path,
            directed.clone(),
            maximum_node_id,
            original_edge_list_separator,
            original_edge_list_header,
            original_sources_column,
            original_sources_column_number,
            original_destinations_column,
            original_destinations_column_number,
            original_edge_list_edge_types_column,
            original_edge_list_edge_types_column_number,
            original_weights_column,
            original_weights_column_number,
            original_edge_type_path,
            original_edge_types_column_number,
            original_edge_types_column,
            edge_types_number,
            original_numeric_edge_type_ids,
            original_minimum_edge_type_id,
            original_edge_type_list_separator,
            original_edge_type_list_header,
            edge_type_list_rows_to_skip,
            edge_type_list_is_correct,
            edge_type_list_max_rows_number,
            edge_type_list_comment_symbol,
            load_edge_type_list_in_parallel,
            target_edge_list_separator,
            target_edge_list_header,
            target_sources_column,
            target_sources_column_number,
            target_destinations_column,
            target_destinations_column_number,
            target_edge_list_edge_types_column,
            target_edge_list_edge_types_column_number,
            target_weights_column,
            target_weights_column_number,
            target_node_path,
            target_node_list_separator,
            target_node_list_header,
            target_nodes_column,
            target_nodes_column_number,
            target_edge_type_list_path,
            target_edge_type_list_separator,
            target_edge_type_list_header,
            target_edge_type_list_edge_types_column,
            target_edge_type_list_edge_types_column_number,
            comment_symbol,
            default_edge_type,
            default_weight,
            max_rows_number,
            rows_to_skip,
            edges_number,
            skip_edge_types_if_unavailable,
            skip_weights_if_unavailable,
            numeric_rows_are_surely_smaller_than_original,
            verbose,
            name
        ))?
        .into();
        (subresult_0.into(), subresult_1.map(|x| x.into()))
    })
}

#[pyfunction]
#[automatically_generated_binding]
#[pyo3(
    text_signature = "(path, separator, header, sources_column, sources_column_number, destinations_column, destinations_column_number, comment_symbol, support_balanced_quotes, max_rows_number, rows_to_skip, edges_number, load_edge_list_in_parallel, verbose, name)"
)]
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
        path,
        separator,
        header,
        sources_column,
        sources_column_number,
        destinations_column,
        destinations_column_number,
        comment_symbol,
        support_balanced_quotes,
        max_rows_number,
        rows_to_skip,
        edges_number,
        load_edge_list_in_parallel,
        verbose,
        name
    ))?
    .into())
}

#[pyfunction]
#[automatically_generated_binding]
#[pyo3(text_signature = "(file_path)")]
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
    Ok(pe!(graph::get_rows_number(file_path))?.into())
}

#[pyfunction]
#[automatically_generated_binding]
#[pyo3(
    text_signature = "(original_edge_path, target_edge_path, original_edge_list_separator, original_edge_list_header, original_edge_list_support_balanced_quotes, original_sources_column, original_sources_column_number, original_destinations_column, original_destinations_column_number, original_edge_list_edge_type_column, original_edge_list_edge_type_column_number, original_weights_column, original_weights_column_number, target_edge_list_separator, target_edge_list_header, target_sources_column_number, target_sources_column, target_destinations_column_number, target_destinations_column, target_edge_list_edge_type_column, target_edge_list_edge_type_column_number, target_weights_column, target_weights_column_number, comment_symbol, default_edge_type, default_weight, max_rows_number, rows_to_skip, edges_number, skip_edge_types_if_unavailable, skip_weights_if_unavailable, verbose, name)"
)]
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
    target_edge_path: &str,
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
        original_edge_path,
        target_edge_path,
        original_edge_list_separator,
        original_edge_list_header,
        original_edge_list_support_balanced_quotes,
        original_sources_column,
        original_sources_column_number,
        original_destinations_column,
        original_destinations_column_number,
        original_edge_list_edge_type_column,
        original_edge_list_edge_type_column_number,
        original_weights_column,
        original_weights_column_number,
        target_edge_list_separator,
        target_edge_list_header,
        target_sources_column_number,
        target_sources_column,
        target_destinations_column_number,
        target_destinations_column,
        target_edge_list_edge_type_column,
        target_edge_list_edge_type_column_number,
        target_weights_column,
        target_weights_column_number,
        comment_symbol,
        default_edge_type,
        default_weight,
        max_rows_number,
        rows_to_skip,
        edges_number,
        skip_edge_types_if_unavailable,
        skip_weights_if_unavailable,
        verbose,
        name
    ))?
    .into())
}

#[pyfunction]
#[automatically_generated_binding]
#[pyo3(
    text_signature = "(original_csv_path, target_csv_path, original_csv_separator, original_csv_header, target_csv_separator, target_csv_header, target_csv_ids_column, target_csv_ids_column_number, comment_symbol, support_balanced_quotes, max_rows_number, rows_to_skip, lines_number, verbose)"
)]
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
    target_csv_path: &str,
    original_csv_separator: Option<char>,
    original_csv_header: Option<bool>,
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
        original_csv_path,
        target_csv_path,
        original_csv_separator,
        original_csv_header,
        target_csv_separator,
        target_csv_header,
        target_csv_ids_column,
        target_csv_ids_column_number,
        comment_symbol,
        support_balanced_quotes,
        max_rows_number,
        rows_to_skip,
        lines_number,
        verbose
    ))?
    .into())
}

#[pyfunction]
#[automatically_generated_binding]
#[pyo3(
    text_signature = "(original_edge_path, target_edge_path, directed, original_node_type_path, original_node_type_list_separator, original_node_types_column_number, original_node_types_column, original_numeric_node_type_ids, original_minimum_node_type_id, original_node_type_list_header, original_node_type_list_support_balanced_quotes, original_node_type_list_rows_to_skip, original_node_type_list_max_rows_number, original_node_type_list_comment_symbol, original_load_node_type_list_in_parallel, original_node_type_list_is_correct, node_types_number, target_node_type_list_path, target_node_type_list_separator, target_node_type_list_node_types_column_number, target_node_type_list_node_types_column, target_node_type_list_header, original_node_path, original_node_list_separator, original_node_list_header, original_node_list_support_balanced_quotes, node_list_rows_to_skip, node_list_is_correct, node_list_max_rows_number, node_list_comment_symbol, default_node_type, original_nodes_column_number, original_nodes_column, original_node_types_separator, original_node_list_node_types_column_number, original_node_list_node_types_column, nodes_number, original_minimum_node_id, original_numeric_node_ids, original_node_list_numeric_node_type_ids, original_skip_node_types_if_unavailable, original_load_node_list_in_parallel, maximum_node_id, target_node_path, target_node_list_separator, target_node_list_header, target_nodes_column, target_nodes_column_number, target_node_types_separator, target_node_list_node_types_column, target_node_list_node_types_column_number, original_edge_type_path, original_edge_type_list_separator, original_edge_types_column_number, original_edge_types_column, original_numeric_edge_type_ids, original_minimum_edge_type_id, original_edge_type_list_header, original_edge_type_list_support_balanced_quotes, edge_type_list_rows_to_skip, edge_type_list_max_rows_number, edge_type_list_comment_symbol, load_edge_type_list_in_parallel, edge_type_list_is_correct, edge_types_number, target_edge_type_list_path, target_edge_type_list_separator, target_edge_type_list_edge_types_column_number, target_edge_type_list_edge_types_column, target_edge_type_list_header, original_edge_list_separator, original_edge_list_header, original_edge_list_support_balanced_quotes, original_sources_column_number, original_sources_column, original_destinations_column_number, original_destinations_column, original_edge_list_edge_types_column_number, original_edge_list_edge_types_column, default_edge_type, original_weights_column_number, original_weights_column, default_weight, original_edge_list_numeric_node_ids, skip_weights_if_unavailable, skip_edge_types_if_unavailable, edge_list_comment_symbol, edge_list_max_rows_number, edge_list_rows_to_skip, load_edge_list_in_parallel, edges_number, target_edge_list_separator, remove_chevrons, remove_spaces, numeric_rows_are_surely_smaller_than_original, sort_temporary_directory, verbose, name)"
)]
/// TODO: write the docstrin
pub fn build_optimal_lists_files(
    original_edge_path: String,
    target_edge_path: String,
    directed: bool,
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
    edge_types_number: Option<EdgeTypeT>,
    target_edge_type_list_path: Option<String>,
    target_edge_type_list_separator: Option<char>,
    target_edge_type_list_edge_types_column_number: Option<usize>,
    target_edge_type_list_edge_types_column: Option<String>,
    target_edge_type_list_header: Option<bool>,
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
    target_edge_list_separator: Option<char>,
    remove_chevrons: Option<bool>,
    remove_spaces: Option<bool>,
    numeric_rows_are_surely_smaller_than_original: Option<bool>,
    sort_temporary_directory: Option<String>,
    verbose: Option<bool>,
    name: Option<String>,
) -> PyResult<(Option<NodeTypeT>, NodeT, Option<EdgeTypeT>, EdgeT)> {
    Ok({
        let (subresult_0, subresult_1, subresult_2, subresult_3) =
            pe!(graph::build_optimal_lists_files(
                original_edge_path.into(),
                target_edge_path.into(),
                directed.clone(),
                original_node_type_path,
                original_node_type_list_separator,
                original_node_types_column_number,
                original_node_types_column,
                original_numeric_node_type_ids,
                original_minimum_node_type_id,
                original_node_type_list_header,
                original_node_type_list_support_balanced_quotes,
                original_node_type_list_rows_to_skip,
                original_node_type_list_max_rows_number,
                original_node_type_list_comment_symbol,
                original_load_node_type_list_in_parallel,
                original_node_type_list_is_correct,
                node_types_number,
                target_node_type_list_path,
                target_node_type_list_separator,
                target_node_type_list_node_types_column_number,
                target_node_type_list_node_types_column,
                target_node_type_list_header,
                original_node_path,
                original_node_list_separator,
                original_node_list_header,
                original_node_list_support_balanced_quotes,
                node_list_rows_to_skip,
                node_list_is_correct,
                node_list_max_rows_number,
                node_list_comment_symbol,
                default_node_type,
                original_nodes_column_number,
                original_nodes_column,
                original_node_types_separator,
                original_node_list_node_types_column_number,
                original_node_list_node_types_column,
                nodes_number,
                original_minimum_node_id,
                original_numeric_node_ids,
                original_node_list_numeric_node_type_ids,
                original_skip_node_types_if_unavailable,
                original_load_node_list_in_parallel,
                maximum_node_id,
                target_node_path,
                target_node_list_separator,
                target_node_list_header,
                target_nodes_column,
                target_nodes_column_number,
                target_node_types_separator,
                target_node_list_node_types_column,
                target_node_list_node_types_column_number,
                original_edge_type_path,
                original_edge_type_list_separator,
                original_edge_types_column_number,
                original_edge_types_column,
                original_numeric_edge_type_ids,
                original_minimum_edge_type_id,
                original_edge_type_list_header,
                original_edge_type_list_support_balanced_quotes,
                edge_type_list_rows_to_skip,
                edge_type_list_max_rows_number,
                edge_type_list_comment_symbol,
                load_edge_type_list_in_parallel,
                edge_type_list_is_correct,
                edge_types_number,
                target_edge_type_list_path,
                target_edge_type_list_separator,
                target_edge_type_list_edge_types_column_number,
                target_edge_type_list_edge_types_column,
                target_edge_type_list_header,
                original_edge_list_separator,
                original_edge_list_header,
                original_edge_list_support_balanced_quotes,
                original_sources_column_number,
                original_sources_column,
                original_destinations_column_number,
                original_destinations_column,
                original_edge_list_edge_types_column_number,
                original_edge_list_edge_types_column,
                default_edge_type,
                original_weights_column_number,
                original_weights_column,
                default_weight,
                original_edge_list_numeric_node_ids,
                skip_weights_if_unavailable,
                skip_edge_types_if_unavailable,
                edge_list_comment_symbol,
                edge_list_max_rows_number,
                edge_list_rows_to_skip,
                load_edge_list_in_parallel,
                edges_number,
                target_edge_list_separator,
                remove_chevrons,
                remove_spaces,
                numeric_rows_are_surely_smaller_than_original,
                sort_temporary_directory,
                verbose,
                name
            ))?
            .into();
        (
            subresult_0.map(|x| x.into()),
            subresult_1.into(),
            subresult_2.map(|x| x.into()),
            subresult_3.into(),
        )
    })
}

#[pyfunction]
#[automatically_generated_binding]
#[pyo3(
    text_signature = "(original_edge_path, target_edge_path, original_edge_list_separator, original_edge_list_header, original_edge_list_support_balanced_quotes, original_edge_list_sources_column, original_edge_list_sources_column_number, original_edge_list_destinations_column, original_edge_list_destinations_column_number, original_edge_list_edge_type_column, original_edge_list_edge_type_column_number, original_edge_list_weights_column, original_edge_list_weights_column_number, target_edge_list_separator, target_edge_list_header, target_edge_list_sources_column_number, target_edge_list_sources_column, target_edge_list_destinations_column_number, target_edge_list_destinations_column, target_edge_list_edge_type_column, target_edge_list_edge_type_column_number, target_edge_list_weights_column, target_edge_list_weights_column_number, comment_symbol, default_edge_type, default_weight, max_rows_number, rows_to_skip, edges_number, skip_edge_types_if_unavailable, skip_weights_if_unavailable, verbose, name)"
)]
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
    target_edge_path: &str,
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
        original_edge_path,
        target_edge_path,
        original_edge_list_separator,
        original_edge_list_header,
        original_edge_list_support_balanced_quotes,
        original_edge_list_sources_column,
        original_edge_list_sources_column_number,
        original_edge_list_destinations_column,
        original_edge_list_destinations_column_number,
        original_edge_list_edge_type_column,
        original_edge_list_edge_type_column_number,
        original_edge_list_weights_column,
        original_edge_list_weights_column_number,
        target_edge_list_separator,
        target_edge_list_header,
        target_edge_list_sources_column_number,
        target_edge_list_sources_column,
        target_edge_list_destinations_column_number,
        target_edge_list_destinations_column,
        target_edge_list_edge_type_column,
        target_edge_list_edge_type_column_number,
        target_edge_list_weights_column,
        target_edge_list_weights_column_number,
        comment_symbol,
        default_edge_type,
        default_weight,
        max_rows_number,
        rows_to_skip,
        edges_number,
        skip_edge_types_if_unavailable,
        skip_weights_if_unavailable,
        verbose,
        name
    ))?)
}

#[pyfunction]
#[automatically_generated_binding]
#[pyo3(
    text_signature = "(original_edge_path, target_edge_path, original_edge_list_separator, original_edge_list_header, original_edge_list_support_balanced_quotes, original_sources_column, original_sources_column_number, original_destinations_column, original_destinations_column_number, original_edge_list_edge_type_column, original_edge_list_edge_type_column_number, original_weights_column, original_weights_column_number, target_edge_list_separator, target_edge_list_header, target_sources_column, target_sources_column_number, target_destinations_column, target_destinations_column_number, target_edge_list_edge_type_column, target_edge_list_edge_type_column_number, target_weights_column, target_weights_column_number, comment_symbol, default_edge_type, default_weight, max_rows_number, rows_to_skip, edges_number, skip_edge_types_if_unavailable, skip_weights_if_unavailable, verbose, name)"
)]
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
    target_edge_path: &str,
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
        original_edge_path,
        target_edge_path,
        original_edge_list_separator,
        original_edge_list_header,
        original_edge_list_support_balanced_quotes,
        original_sources_column,
        original_sources_column_number,
        original_destinations_column,
        original_destinations_column_number,
        original_edge_list_edge_type_column,
        original_edge_list_edge_type_column_number,
        original_weights_column,
        original_weights_column_number,
        target_edge_list_separator,
        target_edge_list_header,
        target_sources_column,
        target_sources_column_number,
        target_destinations_column,
        target_destinations_column_number,
        target_edge_list_edge_type_column,
        target_edge_list_edge_type_column_number,
        target_weights_column,
        target_weights_column_number,
        comment_symbol,
        default_edge_type,
        default_weight,
        max_rows_number,
        rows_to_skip,
        edges_number,
        skip_edge_types_if_unavailable,
        skip_weights_if_unavailable,
        verbose,
        name
    ))?
    .into())
}

#[pyfunction]
#[automatically_generated_binding]
#[pyo3(
    text_signature = "(path, separator, header, sources_column, sources_column_number, destinations_column, destinations_column_number, comment_symbol, max_rows_number, rows_to_skip, edges_number, load_edge_list_in_parallel, remove_chevrons, remove_spaces, verbose, name)"
)]
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
/// remove_chevrons: Optional[bool]
///     Whether remove chevrons while reading elements.
/// remove_spaces: Optional[bool]
///     Whether remove spaces while reading elements.
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
    remove_chevrons: Option<bool>,
    remove_spaces: Option<bool>,
    verbose: Option<bool>,
    name: Option<String>,
) -> PyResult<(EdgeT, EdgeT, EdgeT)> {
    Ok({
        let (subresult_0, subresult_1, subresult_2) =
            pe!(graph::get_minmax_node_from_numeric_edge_list(
                path,
                separator,
                header,
                sources_column,
                sources_column_number,
                destinations_column,
                destinations_column_number,
                comment_symbol,
                max_rows_number,
                rows_to_skip,
                edges_number,
                load_edge_list_in_parallel,
                remove_chevrons,
                remove_spaces,
                verbose,
                name
            ))?
            .into();
        (subresult_0.into(), subresult_1.into(), subresult_2.into())
    })
}

#[pyfunction]
#[automatically_generated_binding]
#[pyo3(
    text_signature = "(source_path, edge_path, node_path, node_type_path, edge_type_path, node_list_separator, node_type_list_separator, edge_type_list_separator, node_types_separator, nodes_column, node_types_column, node_list_node_types_column, edge_types_column, node_descriptions_column, edge_list_separator, directed, sort_temporary_directory, compute_node_description, keep_nodes_without_descriptions, keep_nodes_without_categories, keep_interwikipedia_nodes, keep_external_nodes, verbose)"
)]
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
    directed: bool,
    sort_temporary_directory: Option<String>,
    compute_node_description: Option<bool>,
    keep_nodes_without_descriptions: Option<bool>,
    keep_nodes_without_categories: Option<bool>,
    keep_interwikipedia_nodes: Option<bool>,
    keep_external_nodes: Option<bool>,
    verbose: Option<bool>,
) -> PyResult<(NodeTypeT, NodeT, EdgeT)> {
    Ok({
        let (subresult_0, subresult_1, subresult_2) = pe!(graph::parse_wikipedia_graph(
            source_path,
            edge_path,
            node_path,
            node_type_path,
            edge_type_path,
            node_list_separator.clone(),
            node_type_list_separator.clone(),
            edge_type_list_separator.clone(),
            node_types_separator,
            nodes_column,
            node_types_column,
            node_list_node_types_column,
            edge_types_column,
            node_descriptions_column,
            edge_list_separator.clone(),
            directed.clone(),
            sort_temporary_directory,
            compute_node_description,
            keep_nodes_without_descriptions,
            keep_nodes_without_categories,
            keep_interwikipedia_nodes,
            keep_external_nodes,
            verbose
        ))?
        .into();
        (subresult_0.into(), subresult_1.into(), subresult_2.into())
    })
}

#[pyfunction]
#[automatically_generated_binding]
#[pyo3(
    text_signature = "(path, separator, header, support_balanced_quotes, sources_column, sources_column_number, destinations_column, destinations_column_number, comment_symbol, max_rows_number, rows_to_skip, edges_number, load_edge_list_in_parallel, remove_chevrons, remove_spaces, verbose, name)"
)]
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
/// remove_chevrons: Optional[bool]
///     Whether remove chevrons while reading elements.
/// remove_spaces: Optional[bool]
///     Whether remove spaces while reading elements.
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
    remove_chevrons: Option<bool>,
    remove_spaces: Option<bool>,
    verbose: Option<bool>,
    name: Option<String>,
) -> PyResult<bool> {
    Ok(pe!(graph::is_numeric_edge_list(
        path,
        separator,
        header,
        support_balanced_quotes,
        sources_column,
        sources_column_number,
        destinations_column,
        destinations_column_number,
        comment_symbol,
        max_rows_number,
        rows_to_skip,
        edges_number,
        load_edge_list_in_parallel,
        remove_chevrons,
        remove_spaces,
        verbose,
        name
    ))?
    .into())
}

#[pyfunction]
#[automatically_generated_binding]
#[pyo3(
    text_signature = "(original_node_path, target_node_path, original_node_type_path, original_node_type_list_separator, original_node_types_column_number, original_node_types_column, node_types_number, original_numeric_node_type_ids, original_minimum_node_type_id, original_node_type_list_header, original_node_type_list_support_balanced_quotes, original_node_type_list_rows_to_skip, original_node_type_list_is_correct, original_node_type_list_max_rows_number, original_node_type_list_comment_symbol, original_load_node_type_list_in_parallel, target_node_type_list_path, target_node_type_list_separator, target_node_type_list_header, target_node_type_list_node_types_column, target_node_type_list_node_types_column_number, original_node_list_separator, original_node_list_header, original_node_list_support_balanced_quotes, node_list_rows_to_skip, node_list_max_rows_number, node_list_comment_symbol, default_node_type, original_nodes_column_number, original_nodes_column, original_node_types_separator, original_node_list_node_types_column_number, original_node_list_node_types_column, original_minimum_node_id, original_numeric_node_ids, original_node_list_numeric_node_type_ids, original_skip_node_types_if_unavailable, remove_chevrons, remove_spaces, target_node_list_separator, target_node_list_header, target_nodes_column_number, target_nodes_column, target_node_types_separator, target_node_list_node_types_column_number, target_node_list_node_types_column, nodes_number)"
)]
/// Converts the node list at given path to numeric saving in stream to file. Furthermore, returns the number of nodes that were written and their node types if any.
///
/// Parameters
/// ----------
/// original_node_type_path: Optional[str]
///     Path to the original list of node types.
/// original_node_type_list_separator: Optional[str]
///     Separator to be used for the original node type list.
/// original_node_types_column_number: Optional[int]
///     Number of the node types column to be used for the original node types list.
/// original_node_types_column: Optional[str]
///     Name of the node types column to be used for the original node types list.
/// node_types_number: Optional[int]
///     Number of node types present in the provided original list of node types. If provided, it will allow to make assumptions and to load the node types faster.
/// original_numeric_node_type_ids: Optional[bool]
///     Whether to load the node types as numeric.
/// original_minimum_node_type_id: Optional[int]
///     The minimum numeric node type ID. If provided, it will allow for additional assumptions in the creation of the node types vocabulary.
/// original_node_type_list_header: Optional[bool]
///     Whether the provided node type list has a header.
/// original_node_type_list_support_balanced_quotes: Optional[bool]
///     Whether to support balanced quotes while reading the node type list.
/// original_node_type_list_rows_to_skip: Optional[int]
///     Number of rows to skip before starting to parse the file for the provided node type list file.
/// original_node_type_list_is_correct: Optional[bool]
///     Whether is it safe to assume that the provided node type list is correct.
/// original_node_type_list_max_rows_number: Optional[int]
///     Maximum number of rows to be read in the provided original node type list.
/// original_node_type_list_comment_symbol: Optional[str]
///     Symbol to be used to skip rows starting with it.
/// original_load_node_type_list_in_parallel: Optional[bool]
///     Whether to load the node type list in parallel.
/// target_node_type_list_path: Optional[str]
///     Path where to store the parsed node types (if any).
/// target_node_type_list_separator: Optional[str]
///     Separator to be used for the target node type list.
/// target_node_type_list_header: Optional[bool]
///     Whether to add header when writing the target node type list.
/// target_node_type_list_node_types_column: Optional[str]
///     Name of the column of the node types in the target node type list.
/// target_node_type_list_node_types_column_number: Optional[int]
///     Number of the column of the node types in the target node type list.
/// original_node_path: str
///     Path to the original list of nodes.
/// original_node_list_separator: Optional[str]
///     Separator to be used for rows of the original node list.
/// original_node_list_header: Optional[bool]
///     Whether to expect a header in the original node list file.
/// original_node_list_support_balanced_quotes: Optional[bool]
///     Whether to support balanced quotes while reading the node list.
/// node_list_rows_to_skip: Optional[int]
///     Number of rows to skip before starting to parse the original node list.
/// node_list_max_rows_number: Optional[int]
///     Maximum number of rows to read from the origina node list.
/// node_list_comment_symbol: Optional[str]
///     Symbol to use to skip rows starting with it in the original node list.
/// default_node_type: Optional[str]
///     Default node type to be used when none are provided or are missing for some nodes.
/// original_nodes_column_number: Optional[int]
///     Number of the column for the node name in the original node list.
/// original_nodes_column: Optional[str]
///     Name of the column for the node name in the original node list.
/// original_node_types_separator: Optional[str]
///     Separator to be used for the node types within the original node list.
/// original_node_list_node_types_column_number: Optional[int]
///     Number of the column for the node types in the original node list.
/// original_node_list_node_types_column: Optional[str]
///     Name of the column for the node types in the original node list.
/// original_minimum_node_id: Optional[int]
///     The minimum numeric node ID. If provided, it will allow for additional assumptions in the creation of the nodes vocabulary.
/// original_numeric_node_ids: Optional[bool]
///     Whether to load the node names as numeric.
/// original_node_list_numeric_node_type_ids: Optional[bool]
///     Whether to load the node type names from the original node list as numeric.
/// original_skip_node_types_if_unavailable: Optional[bool]
///     Whether to skip the node types if the provided node types column is not provided.
/// remove_chevrons: Optional[bool]
///     Whether remove chevrons while reading elements.
/// remove_spaces: Optional[bool]
///     Whether remove spaces while reading elements.
/// target_node_path: str
///     Path where to store the target node paths.
/// target_node_list_separator: Optional[str]
///     Separator to be used for the target node list.
/// target_node_list_header: Optional[bool]
///     Whether to add an header to the target node list.
/// target_nodes_column_number: Optional[int]
///     Number of the column where to store the node names.
/// target_nodes_column: Optional[str]
///     Name of the column where to store the node names.
/// target_node_types_separator: Optional[str]
///     Separator to be used for the node types within the target node list.
/// target_node_list_node_types_column_number: Optional[int]
///     Number for the column with the node type names within the target node list.
/// target_node_list_node_types_column: Optional[str]
///     Name for the column with the node type names within the target node list.
/// nodes_number: Optional[int]
///     Number of the nodes in the original node list.
///
pub fn convert_node_list_node_types_to_numeric(
    original_node_path: String,
    target_node_path: String,
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
    remove_chevrons: Option<bool>,
    remove_spaces: Option<bool>,
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
            original_node_path.into(),
            target_node_path.into(),
            original_node_type_path,
            original_node_type_list_separator,
            original_node_types_column_number,
            original_node_types_column,
            node_types_number,
            original_numeric_node_type_ids,
            original_minimum_node_type_id,
            original_node_type_list_header,
            original_node_type_list_support_balanced_quotes,
            original_node_type_list_rows_to_skip,
            original_node_type_list_is_correct,
            original_node_type_list_max_rows_number,
            original_node_type_list_comment_symbol,
            original_load_node_type_list_in_parallel,
            target_node_type_list_path,
            target_node_type_list_separator,
            target_node_type_list_header,
            target_node_type_list_node_types_column,
            target_node_type_list_node_types_column_number,
            original_node_list_separator,
            original_node_list_header,
            original_node_list_support_balanced_quotes,
            node_list_rows_to_skip,
            node_list_max_rows_number,
            node_list_comment_symbol,
            default_node_type,
            original_nodes_column_number,
            original_nodes_column,
            original_node_types_separator,
            original_node_list_node_types_column_number,
            original_node_list_node_types_column,
            original_minimum_node_id,
            original_numeric_node_ids,
            original_node_list_numeric_node_type_ids,
            original_skip_node_types_if_unavailable,
            remove_chevrons,
            remove_spaces,
            target_node_list_separator,
            target_node_list_header,
            target_nodes_column_number,
            target_nodes_column,
            target_node_types_separator,
            target_node_list_node_types_column_number,
            target_node_list_node_types_column,
            nodes_number
        ))?
        .into();
        (subresult_0.into(), subresult_1.map(|x| x.into()))
    })
}

#[pyfunction]
#[automatically_generated_binding]
#[pyo3(
    text_signature = "(edge_path, edge_list_separator, edge_list_header, edge_list_support_balanced_quotes, edge_list_sources_column, edge_list_sources_column_number, edge_list_destinations_column, edge_list_destinations_column_number, edge_list_edge_type_column, edge_list_edge_type_column_number, edge_list_weights_column, edge_list_weights_column_number, comment_symbol, default_edge_type, default_weight, max_rows_number, rows_to_skip, edges_number, skip_edge_types_if_unavailable, skip_weights_if_unavailable, verbose, name)"
)]
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
        edge_path,
        edge_list_separator,
        edge_list_header,
        edge_list_support_balanced_quotes,
        edge_list_sources_column,
        edge_list_sources_column_number,
        edge_list_destinations_column,
        edge_list_destinations_column_number,
        edge_list_edge_type_column,
        edge_list_edge_type_column_number,
        edge_list_weights_column,
        edge_list_weights_column_number,
        comment_symbol,
        default_edge_type,
        default_weight,
        max_rows_number,
        rows_to_skip,
        edges_number,
        skip_edge_types_if_unavailable,
        skip_weights_if_unavailable,
        verbose,
        name
    ))?
    .into())
}

#[pyfunction]
#[automatically_generated_binding]
#[pyo3(
    text_signature = "(path, target_path, separator, header, sources_column, sources_column_number, destinations_column, destinations_column_number, edge_types_column, edge_types_column_number, rows_to_skip, skip_edge_types_if_unavailable, sort_temporary_directory)"
)]
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
        path,
        target_path,
        separator,
        header,
        sources_column,
        sources_column_number,
        destinations_column,
        destinations_column_number,
        edge_types_column,
        edge_types_column_number,
        rows_to_skip,
        skip_edge_types_if_unavailable,
        sort_temporary_directory
    ))?)
}

#[pyfunction]
#[automatically_generated_binding]
#[pyo3(
    text_signature = "(path, separator, header, sources_column, sources_column_number, destinations_column, destinations_column_number, edge_types_column, edge_types_column_number, rows_to_skip, skip_edge_types_if_unavailable, sort_temporary_directory)"
)]
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
        path,
        separator,
        header,
        sources_column,
        sources_column_number,
        destinations_column,
        destinations_column_number,
        edge_types_column,
        edge_types_column_number,
        rows_to_skip,
        skip_edge_types_if_unavailable,
        sort_temporary_directory
    ))?)
}

#[pyfunction]
#[automatically_generated_binding]
#[pyo3(
    text_signature = "(path, separator, header, support_balanced_quotes, sources_column, sources_column_number, destinations_column, destinations_column_number, comment_symbol, max_rows_number, rows_to_skip, edges_number, load_edge_list_in_parallel, verbose, name)"
)]
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
pub fn get_number_of_selfloops_from_edge_list(
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
    Ok(pe!(graph::get_number_of_selfloops_from_edge_list(
        path,
        separator,
        header,
        support_balanced_quotes,
        sources_column,
        sources_column_number,
        destinations_column,
        destinations_column_number,
        comment_symbol,
        max_rows_number,
        rows_to_skip,
        edges_number,
        load_edge_list_in_parallel,
        verbose,
        name
    ))?
    .into())
}

pub fn register_utils(_py: Python, _m: &PyModule) -> PyResult<()> {
    Ok(())
}
