#[pymethods]
impl EnsmallenGraph {
    #[text_signature = "($self, seed, train_percentage)"]
    /// Returns training and validation holdouts extracted from current graph.
    ///
    /// The holdouts is generated in such a way that the training set remains
    /// connected if the starting graph is connected by using a spanning tree.
    ///
    /// Parameters
    /// -----------------------------
    /// seed: int,
    ///     The seed to use to generate the holdout.
    /// train_percentage: float,
    ///     The percentage to reserve for the training.
    ///
    /// Returns
    /// -----------------------------
    /// Tuple containing training and validation graphs.
    fn connected_holdout(
        &self,
        seed: NodeT,
        train_percentage: f64,
    ) -> PyResult<(EnsmallenGraph, EnsmallenGraph)> {
        match self.graph.connected_holdout(seed, train_percentage) {
            Ok((g1, g2)) => Ok((EnsmallenGraph { graph: g1 }, EnsmallenGraph { graph: g2 })),
            Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
        }
    }

    #[text_signature = "($self, seed, nodes_number)"]
    /// Returns partial subgraph.
    ///
    /// This method creates a subset of the graph starting from a random node
    /// sampled using given seed and includes all neighbouring nodes until
    /// the required number of nodes is reached. All the edges connecting any
    /// of the selected nodes are then inserted into this graph.
    ///
    /// Parameters
    /// -----------------------------
    /// seed: int,
    ///     The seed to use to generate the partial graph.
    /// nodes_number: int,
    ///     The number of edges to insert in the partial graph.
    ///
    /// Returns
    /// -----------------------------
    /// Partial graph.
    fn random_subgraph(&self, seed: NodeT, nodes_number: NodeT) -> PyResult<EnsmallenGraph> {
        match self.graph.random_subgraph(seed, nodes_number) {
            Ok(g) => Ok(EnsmallenGraph { graph: g }),
            Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
        }
    }

    #[text_signature = "($self, seed, train_percentage)"]
    /// Returns training and validation holdouts extracted from current graph.
    ///
    /// The holdouts edges are randomly sampled and have no garanties that any
    /// particular graph structure is maintained.
    ///
    /// Parameters
    /// -----------------------------
    /// seed: int,
    ///     The seed to use to generate the holdout.
    /// train_percentage: float,
    ///     The percentage to reserve for the training.
    ///
    /// Returns
    /// -----------------------------
    /// Tuple containing training and validation graphs.
    fn random_holdout(
        &self,
        seed: NodeT,
        train_percentage: f64,
    ) -> PyResult<(EnsmallenGraph, EnsmallenGraph)> {
        match self.graph.random_holdout(seed, train_percentage) {
            Ok((g1, g2)) => Ok((EnsmallenGraph { graph: g1 }, EnsmallenGraph { graph: g2 })),
            Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
        }
    }

    #[text_signature = "($self, seed, negatives_number, allow_selfloops)"]
    /// Returns Graph with given amount of negative edges as positive edges.
    ///
    /// The graph generated may be used as a testing negatives partition to be
    /// fed into the argument "graph_to_avoid" of the link_prediction or the
    /// binary_skipgrams algorithm.
    ///
    ///
    /// Parameters
    /// -----------------------------
    /// seed: int,
    ///     The seed to use to generate the holdout.
    /// negatives_number: int,
    ///     The number of negative edges to use.
    /// allow_selfloops: bool,
    ///     Wethever to allow creation of self-loops.
    ///
    /// Returns
    /// -----------------------------
    /// Graph containing given amount of missing edges.
    fn sample_negatives(
        &self,
        seed: EdgeT,
        negatives_number: EdgeT,
        allow_selfloops: bool,
    ) -> PyResult<EnsmallenGraph> {
        match self
            .graph
            .sample_negatives(seed, negatives_number, allow_selfloops)
        {
            Ok(g) => Ok(EnsmallenGraph { graph: g }),
            Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(e)),
        }
    }
}
