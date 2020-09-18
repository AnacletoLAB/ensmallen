use super::*;
use graph::{EdgeT, NodeT};
use pyo3::exceptions;
use pyo3::prelude::*;

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
    /// include_all_edge_types: bool,
    ///     Wethever to include all the edges between two nodes.
    ///     This is only relevant in multi-graphs.
    /// verbose: bool,
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
        seed: NodeT,
        train_percentage: f64,
        include_all_edge_types: bool,
        verbose: bool,
    ) -> PyResult<(EnsmallenGraph, EnsmallenGraph)> {
        let (g1, g2) = pyex!(self.graph.connected_holdout(
            seed,
            train_percentage,
            include_all_edge_types,
            verbose
        ))?;
        Ok((EnsmallenGraph { graph: g1 }, EnsmallenGraph { graph: g2 }))
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
    /// verbose: bool,
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
        seed: NodeT,
        nodes_number: NodeT,
        verbose: bool,
    ) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pyex!(self.graph.random_subgraph(seed, nodes_number, verbose))?,
        })
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
    /// Raises
    /// -----------------------------
    /// TODO: Add the docstring for the raised exceptions.
    ///
    /// Returns
    /// -----------------------------
    /// Tuple containing training and validation graphs.
    fn random_holdout(
        &self,
        seed: NodeT,
        train_percentage: f64,
        include_all_edge_types: bool,
        verbose: bool,
    ) -> PyResult<(EnsmallenGraph, EnsmallenGraph)> {
        let (g1, g2) = pyex!(self.graph.random_holdout(
            seed,
            train_percentage,
            include_all_edge_types,
            verbose
        ))?;
        Ok((EnsmallenGraph { graph: g1 }, EnsmallenGraph { graph: g2 }))
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
    /// verbose: bool,
    ///     Wethever to show the loading bar.
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
        seed: EdgeT,
        negatives_number: EdgeT,
        allow_selfloops: bool,
        verbose: bool,
    ) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pyex!(self.graph.sample_negatives(
                seed,
                negatives_number,
                allow_selfloops,
                verbose
            ))?,
        })
    }
}
