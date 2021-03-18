use super::*;
use graph::{EdgeT, NodeT};

#[pymethods]
impl EnsmallenGraph {
    #[args(py_kwargs = "**")]
    #[text_signature = "($self, train_size, *, random_state, edge_types, include_all_edge_types, verbose)"]
    /// Returns training and validation holdouts extracted from current graph.
    ///
    /// The holdouts is generated in such a way that the training set remains
    /// connected if the starting graph is connected by using a spanning tree.
    ///
    /// Parameters
    /// -----------------------------
    /// train_size: float,
    ///     The rate of edges to reserve for the training.
    /// random_state: int = 42,
    ///     The random_state to use to generate the holdout.
    /// edge_types: List[str] = None,
    ///     List of names of the edge types to put into the validation.
    /// include_all_edge_types: bool = False,
    ///     whether to include all the edges between two nodes.
    ///     This is only relevant in multi-graphs.
    /// verbose: bool = True,
    ///     whether to show the loading bar.
    ///
    /// Raises
    /// -----------------------------
    /// ValueError,
    ///     If the given train rate is not a real number between 0 and 1.
    ///
    /// Returns
    /// -----------------------------
    /// Tuple containing training and validation graphs.
    fn connected_holdout(
        &self,
        train_size: f64,
        py_kwargs: Option<&PyDict>,
    ) -> PyResult<(EnsmallenGraph, EnsmallenGraph)> {
        let py = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, py.python());

        pyex!(validate_kwargs(
            kwargs,
            build_walk_parameters_list(&[
                "random_state",
                "edge_types",
                "include_all_edge_types",
                "verbose"
            ]),
        ))?;

        let (g1, g2) = pyex!(self.graph.connected_holdout(
            pyex!(extract_value!(kwargs, "random_state", EdgeT))?.unwrap_or(42),
            train_size,
            pyex!(extract_value!(kwargs, "edge_types", Vec<String>))?,
            pyex!(extract_value!(kwargs, "include_all_edge_types", bool))?.unwrap_or(false),
            pyex!(extract_value!(kwargs, "verbose", bool))?.unwrap_or(true),
        ))?;
        Ok((EnsmallenGraph { graph: g1 }, EnsmallenGraph { graph: g2 }))
    }

    #[args(py_kwargs = "**")]
    #[text_signature = "($self, nodes_number, *, random_state, verbose)"]
    /// Returns partial subgraph.
    ///
    /// This method creates a subset of the graph starting from a random node
    /// sampled using given random_state and includes all neighbouring nodes until
    /// the required number of nodes is reached. All the edges connecting any
    /// of the selected nodes are then inserted into this graph.
    ///
    /// Parameters
    /// -----------------------------
    /// nodes_number: int,
    ///     The number of edges to insert in the partial graph.
    /// random_state: int = 42,
    ///     The random_state to use to generate the partial graph.
    /// verbose: bool = True,
    ///     whether to show the loading bar.
    ///
    /// Raises
    /// -----------------------------
    /// TODO: Add the docstring for the raised exceptions.
    ///
    /// Returns
    /// -----------------------------
    /// Partial graph.
    fn random_subgraph(
        &self,
        nodes_number: NodeT,
        py_kwargs: Option<&PyDict>,
    ) -> PyResult<EnsmallenGraph> {
        let py = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, py.python());

        pyex!(validate_kwargs(
            kwargs,
            build_walk_parameters_list(&["random_state", "verbose"])
        ))?;

        Ok(EnsmallenGraph {
            graph: pyex!(self.graph.random_subgraph(
                pyex!(extract_value!(kwargs, "random_state", usize))?.unwrap_or(42),
                nodes_number,
                pyex!(extract_value!(kwargs, "verbose", bool))?.unwrap_or(true),
            ))?,
        })
    }

    #[args(py_kwargs = "**")]
    #[text_signature = "($self, train_size, *, random_state, include_all_edge_types, edge_types, min_number_overlaps, verbose)"]
    /// Returns training and validation holdouts extracted from current graph.
    ///
    /// The holdouts edges are randomly sampled and have no garanties that any
    /// particular graph structure is maintained.
    ///
    /// Parameters
    /// -----------------------------
    /// train_size: float,
    ///     The rate to reserve for the training.
    /// random_state: int = 42,
    ///     The random_state to make the holdout reproducible.
    /// include_all_edge_types: bool = False,
    ///     whether to include all the edges between two nodes.
    ///     This is only relevant in multi-graphs.
    /// edge_types: List[String] = None,
    ///     The edge types to be included in the validation.
    ///     If None (default value) is passed, any edge type can be in the validation set.
    ///     If a non None value is passed, the graph MUST be an heterogeneous graph
    ///     with multiple edge types, otherwise an exception will be raised.
    /// min_number_overlaps: int = None,
    ///     The minimum number of overlapping edges for an edge to be put into the validation set.
    ///     If the value passed is None (default value) any edge can be put into the validation set.
    ///     If a non None value is passed, the graph MUST be a multi-graph, otherwise an exception will be raised.
    /// verbose: bool = True,
    ///     whether to show the loading bar.
    ///
    /// Raises
    /// -----------------------------
    /// ValueError,
    ///     If the given train rate is invalid, for example less or equal to 0
    ///     or greater than one.
    /// ValueError,
    ///     If edge types are required but graph is not heterogeneous.
    /// ValueError,
    ///     If given edge types do not exist.
    /// ValueError,
    ///     If min number overlaps is given but graph is not a multigraph.
    ///
    /// Returns
    /// -----------------------------
    /// Tuple containing training and validation graphs.
    fn random_holdout(
        &self,
        train_size: f64,
        py_kwargs: Option<&PyDict>,
    ) -> PyResult<(EnsmallenGraph, EnsmallenGraph)> {
        let py = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, py.python());

        pyex!(validate_kwargs(
            kwargs,
            build_walk_parameters_list(&[
                "random_state",
                "include_all_edge_types",
                "edge_types",
                "min_number_overlaps",
                "verbose",
            ]),
        ))?;

        let (g1, g2) = pyex!(self.graph.random_holdout(
            pyex!(extract_value!(kwargs, "random_state", EdgeT))?.unwrap_or(42),
            train_size,
            pyex!(extract_value!(kwargs, "include_all_edge_types", bool))?.unwrap_or(false),
            pyex!(extract_value!(kwargs, "edge_types", Vec<String>))?,
            pyex!(extract_value!(kwargs, "min_number_overlaps", EdgeT))?,
            pyex!(extract_value!(kwargs, "verbose", bool))?.unwrap_or(true),
        ))?;
        Ok((EnsmallenGraph { graph: g1 }, EnsmallenGraph { graph: g2 }))
    }

    #[args(py_kwargs = "**")]
    #[text_signature = "($self, negatives_number, *, random_state, seed_graph, only_from_same_component, verbose)"]
    /// Returns Graph with given amount of negative edges as positive edges.
    ///
    /// The graph generated may be used as a testing negatives partition to be
    /// fed into the argument "graph_to_avoid" of the link_prediction or the
    /// binary_skipgrams algorithm.
    ///
    ///
    /// Parameters
    /// -----------------------------
    /// negatives_number: int,
    ///     The number of negative edges to use.
    /// random_state: int = 42,
    ///     The random_state to use to generate the holdout.
    /// seed_graph: EnsmallenGraph = None,
    ///     The (optional) graph whose nodes are used as sources or destinations
    ///     of the generated negative edges.
    /// only_from_same_component: bool = True,
    ///     Wether to sample negative edges only from the same node component.
    ///     This avoids generating topologically impossible negative edges.
    /// verbose: bool = True,
    ///     whether to show the loading bar.
    ///     The loading bar will only be visible in console.
    ///
    /// Raises
    /// -----------------------------
    /// TODO: Add the docstring for the raised exceptions.
    ///
    /// Returns
    /// -----------------------------
    /// Graph containing given amount of edges missing in the original graph.
    fn sample_negatives(
        &self,
        negatives_number: EdgeT,
        py_kwargs: Option<&PyDict>,
    ) -> PyResult<EnsmallenGraph> {
        let py = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, py.python());

        pyex!(validate_kwargs(
            kwargs,
            build_walk_parameters_list(&[
                "random_state",
                "verbose",
                "seed_graph",
                "only_from_same_component"
            ]),
        ))?;

        let seed_graph = pyex!(extract_value!(kwargs, "seed_graph", EnsmallenGraph))?;

        Ok(EnsmallenGraph {
            graph: pyex!(self.graph.sample_negatives(
                pyex!(extract_value!(kwargs, "random_state", EdgeT))?.unwrap_or(42),
                negatives_number,
                match &seed_graph {
                    Some(sg) => Some(&sg.graph),
                    None => None,
                },
                pyex!(extract_value!(kwargs, "only_from_same_component", bool))?.unwrap_or(true),
                pyex!(extract_value!(kwargs, "verbose", bool))?.unwrap_or(true),
            ))?,
        })
    }

    #[args(py_kwargs = "**")]
    #[text_signature = "($self, k, k_index, *, edge_types, random_state, verbose)"]
    /// Returns train and test graph following kfold validation scheme.
    ///
    /// The edges are splitted into k chunks. The k_index-th chunk is used to build
    /// the validation graph, all the other edges create the training graph.
    ///
    /// Parameters
    /// -----------------------------
    /// k: int,
    ///     The number of folds.
    /// k_index: int,
    ///     Which fold to use for the validation.
    /// edge_types: List[str] = None,
    ///     Edge types to be selected when computing the folds
    ///        (All the edge types not listed here will be always be used in the training set).
    /// random_state: int = 42,
    ///     The random_state (seed) to use for the holdout,
    /// verbose: bool = True,
    ///     whether to show the loading bar.
    ///
    /// Raises
    /// -----------------------------
    /// TODO: Add the docstring for the raised exceptions.
    ///
    /// Returns
    /// -----------------------------
    /// train, test graph.
    fn kfold(
        &self,
        k: EdgeT,
        k_index: u64,
        py_kwargs: Option<&PyDict>,
    ) -> PyResult<(EnsmallenGraph, EnsmallenGraph)> {
        let py = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, py.python());

        pyex!(validate_kwargs(
            kwargs,
            build_walk_parameters_list(&["edge_types", "random_state", "verbose"])
        ))?;

        let (train, test) = pyex!(self.graph.kfold(
            k,
            k_index,
            pyex!(extract_value!(kwargs, "edge_types", Vec<String>))?,
            pyex!(extract_value!(kwargs, "random_state", u64))?.unwrap_or(42),
            pyex!(extract_value!(kwargs, "verbose", bool))?.unwrap_or(true),
        ))?;

        Ok((
            EnsmallenGraph { graph: train },
            EnsmallenGraph { graph: test },
        ))
    }

    #[args(py_kwargs = "**")]
    #[text_signature = "($self, train_size, random_state, use_stratification)"]
    /// Returns train and test graphs for node-label prediction tasks.
    ///
    /// The split is done using Monte Carlo split or, if stratification is
    /// enabled, Stratified Monte Carlo.
    ///
    /// Parameters
    /// -----------------------------
    /// train_size: float,
    ///     Rate target to reserve for training,
    /// random_state: int = 42,
    ///     The random_state to use for the holdout,
    /// use_stratification: bool = True,
    ///     Whether to use edge-label stratification,
    ///
    /// Raises
    /// -----------------------------
    /// ValueError,
    ///     If the graph does not have node types.
    /// ValueError,
    ///     If the stratification is required but the graph has multi-label node types.
    /// ValueError,
    ///     If the stratification is required but the graph has some node types with insufficient cardinality.
    ///
    /// Returns
    /// -----------------------------
    /// Train and test graph.
    fn node_label_holdout(
        &self,
        train_size: f64,
        py_kwargs: Option<&PyDict>,
    ) -> PyResult<(EnsmallenGraph, EnsmallenGraph)> {
        let py = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, py.python());

        pyex!(validate_kwargs(
            kwargs,
            build_walk_parameters_list(&["random_state", "use_stratification"])
        ))?;

        let (train, test) = pyex!(self.graph.node_label_holdout(
            train_size,
            pyex!(extract_value!(kwargs, "use_stratification", bool))?.unwrap_or(true),
            pyex!(extract_value!(kwargs, "random_state", u64))?.unwrap_or(42),
        ))?;

        Ok((
            EnsmallenGraph { graph: train },
            EnsmallenGraph { graph: test },
        ))
    }

    #[args(py_kwargs = "**")]
    #[text_signature = "($self, train_size, random_state, use_stratification)"]
    /// Returns train and test graphs for edge-label prediction tasks.
    ///
    /// The split is done using Monte Carlo split or, if stratification is
    /// enabled, Stratified Monte Carlo.
    ///
    /// Parameters
    /// -----------------------------
    /// train_size: float,
    ///     Rate target to reserve for training,
    /// random_state: int = 42,
    ///     The random_state to use for the holdout,
    /// use_stratification: bool = True,
    ///     Whether to use edge-label stratification,
    ///
    /// Raises
    /// -----------------------------
    /// ValueError,
    ///     If the graph does not have edge types.
    /// ValueError,
    ///     If the stratification is required but the graph has some edge types with insufficient cardinality.
    ///
    /// Returns
    /// -----------------------------
    /// Train and test graph.
    fn edge_label_holdout(
        &self,
        train_size: f64,
        py_kwargs: Option<&PyDict>,
    ) -> PyResult<(EnsmallenGraph, EnsmallenGraph)> {
        let py = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, py.python());

        pyex!(validate_kwargs(
            kwargs,
            build_walk_parameters_list(&["random_state", "use_stratification"])
        ))?;

        let (train, test) = pyex!(self.graph.edge_label_holdout(
            train_size,
            pyex!(extract_value!(kwargs, "use_stratification", bool))?.unwrap_or(true),
            pyex!(extract_value!(kwargs, "random_state", u64))?.unwrap_or(42),
        ))?;

        Ok((
            EnsmallenGraph { graph: train },
            EnsmallenGraph { graph: test },
        ))
    }
}
