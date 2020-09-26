use super::*;
use graph::{EdgeT, NodeT};

#[pymethods]
impl EnsmallenGraph {
    #[args(py_kwargs = "**")]
    #[text_signature = "($self, train_percentage, *, seed, include_all_edge_types, verbose)"]
    /// Returns training and validation holdouts extracted from current graph.
    ///
    /// The holdouts is generated in such a way that the training set remains
    /// connected if the starting graph is connected by using a spanning tree.
    ///
    /// Parameters
    /// -----------------------------
    /// train_percentage: float,
    ///     The percentage to reserve for the training.
    /// seed: int = 42,
    ///     The seed to use to generate the holdout.
    /// include_all_edge_types: bool = True,
    ///     Wethever to include all the edges between two nodes.
    ///     This is only relevant in multi-graphs.
    /// verbose: bool = True,
    ///     Wethever to show the loading bar.
    ///
    /// Raises
    /// -----------------------------
    /// TODO: Add the docstring for the raised exceptions.
    ///
    /// Returns
    /// -----------------------------
    /// Tuple containing training and validation graphs.
    fn connected_holdout(
        &self,
        train_percentage: f64,
        py_kwargs: Option<&PyDict>,
    ) -> PyResult<(EnsmallenGraph, EnsmallenGraph)> {
        let py = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, py.python());

        validate_kwargs(
            kwargs,
            build_walk_parameters_list(&["seed", "include_all_edge_types", "verbose"]),
        )?;

        let (g1, g2) = pyex!(self.graph.connected_holdout(
            extract_value!(kwargs, "seed", usize)
                .or_else(|| Some(42))
                .unwrap(),
            train_percentage,
            extract_value!(kwargs, "include_all_edge_types", bool)
                .or_else(|| Some(true))
                .unwrap(),
            extract_value!(kwargs, "verbose", bool)
                .or_else(|| Some(true))
                .unwrap()
        ))?;
        Ok((EnsmallenGraph { graph: g1 }, EnsmallenGraph { graph: g2 }))
    }

    #[args(py_kwargs = "**")]
    #[text_signature = "($self, nodes_number, *, seed, verbose)"]
    /// Returns partial subgraph.
    ///
    /// This method creates a subset of the graph starting from a random node
    /// sampled using given seed and includes all neighbouring nodes until
    /// the required number of nodes is reached. All the edges connecting any
    /// of the selected nodes are then inserted into this graph.
    ///
    /// Parameters
    /// -----------------------------
    /// nodes_number: int,
    ///     The number of edges to insert in the partial graph.
    /// seed: int = 42,
    ///     The seed to use to generate the partial graph.
    /// verbose: bool = True,
    ///     Wethever to show the loading bar.
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

        validate_kwargs(kwargs, build_walk_parameters_list(&["seed", "verbose"]))?;

        Ok(EnsmallenGraph {
            graph: pyex!(self.graph.random_subgraph(
                extract_value!(kwargs, "seed", usize)
                    .or_else(|| Some(42))
                    .unwrap(),
                nodes_number,
                extract_value!(kwargs, "verbose", bool)
                    .or_else(|| Some(true))
                    .unwrap()
            ))?,
        })
    }

    #[args(py_kwargs = "**")]
    #[text_signature = "($self, train_percentage, *, seed, include_all_edge_types, edge_types, min_number_overlaps, verbose)"]
    /// Returns training and validation holdouts extracted from current graph.
    ///
    /// The holdouts edges are randomly sampled and have no garanties that any
    /// particular graph structure is maintained.
    ///
    /// Parameters
    /// -----------------------------
    /// train_percentage: float,
    ///     The percentage to reserve for the training.
    /// seed: int = 42,
    ///     The seed to make the holdout reproducible.
    /// include_all_edge_types: bool = True,
    ///     Wethever to include all the edges between two nodes.
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
    ///     Wethever to show the loading bar.
    ///
    /// Raises
    /// -----------------------------
    /// ValueError,
    ///     If the given train percentage is invalid, for example less or equal to 0
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
        train_percentage: f64,
        py_kwargs: Option<&PyDict>,
    ) -> PyResult<(EnsmallenGraph, EnsmallenGraph)> {
        let py = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, py.python());

        validate_kwargs(
            kwargs,
            build_walk_parameters_list(&[
                "seed",
                "include_all_edge_types",
                "edge_types",
                "min_number_overlaps",
                "verbose",
            ]),
        )?;

        let (g1, g2) = pyex!(self.graph.random_holdout(
            extract_value!(kwargs, "seed", usize)
                .or_else(|| Some(42))
                .unwrap(),
            train_percentage,
            extract_value!(kwargs, "include_all_edge_types", bool)
                .or_else(|| Some(true))
                .unwrap(),
            extract_value!(kwargs, "edge_types", Vec<String>),
            extract_value!(kwargs, "min_number_overlaps", usize),
            extract_value!(kwargs, "verbose", bool)
                .or_else(|| Some(true))
                .unwrap()
        ))?;
        Ok((EnsmallenGraph { graph: g1 }, EnsmallenGraph { graph: g2 }))
    }

    #[args(py_kwargs = "**")]
    #[text_signature = "($self, negatives_number, *, seed, allow_selfloops, verbose)"]
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
    /// seed: int = 42,
    ///     The seed to use to generate the holdout.
    /// allow_selfloops: bool = None,
    ///     Wethever to allow creation of self-loops.
    ///     If None (default value) is provided, self-loops are created only
    ///     if they are present in the original graph.
    /// verbose: bool = True,
    ///     Wethever to show the loading bar.
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

        validate_kwargs(
            kwargs,
            build_walk_parameters_list(&["seed", "allow_selfloops", "verbose"]),
        )?;

        Ok(EnsmallenGraph {
            graph: pyex!(self.graph.sample_negatives(
                extract_value!(kwargs, "seed", usize)
                    .or_else(|| Some(42))
                    .unwrap(),
                negatives_number,
                extract_value!(kwargs, "allow_selfloops", bool)
                    .or_else(|| Some(self.graph.has_selfloops()))
                    .unwrap(),
                extract_value!(kwargs, "verbose", bool)
                    .or_else(|| Some(true))
                    .unwrap()
            ))?,
        })
    }

    #[args(py_kwargs = "**")]
    #[text_signature = "($self, edge_types, *, verbose)"]
    /// Returns Graph with only the required edge types.
    ///
    /// Parameters
    /// -----------------------------
    /// edge_types: List[str],
    ///     Edge types to include in the graph.
    /// verbose: bool = True,
    ///     Wethever to show the loading bar.
    ///
    /// Raises
    /// -----------------------------
    /// TODO: Add the docstring for the raised exceptions.
    ///
    /// Returns
    /// -----------------------------
    /// Graph containing given amount of edges missing in the original graph.
    fn edge_types_subgraph(
        &self,
        edge_types: Vec<String>,
        py_kwargs: Option<&PyDict>,
    ) -> PyResult<EnsmallenGraph> {
        let py = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, py.python());

        validate_kwargs(kwargs, build_walk_parameters_list(&["verbose"]))?;

        Ok(EnsmallenGraph {
            graph: pyex!(self.graph.edge_types_subgraph(
                edge_types,
                extract_value!(kwargs, "verbose", bool)
                    .or_else(|| Some(true))
                    .unwrap()
            ))?,
        })
    }
}
