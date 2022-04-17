use super::*;
use graph::NodeT;
use numpy::PyArray2;
use rayon::iter::IndexedParallelIterator;
use rayon::prelude::*;
use types::ThreadDataRaceAware;

#[pymethods]
impl Graph {
    #[args(py_kwargs = "**")]
    #[text_signature = "($self, walk_length, quantity, *, return_weight, explore_weight, change_edge_type_weight, change_node_type_weight, random_state, iterations, dense_node_mapping, max_neighbours)"]
    /// Return random walks done on the graph using Rust.
    ///
    /// Parameters
    /// ----------
    /// walk_length: int
    ///     Maximal length of the random walk.
    ///     On graphs without traps, all walks have this length.
    /// quantity: int
    ///     Number of nodes to sample.
    /// return_weight: float = 1.0
    ///     Weight on the probability of returning to node coming from
    ///     Having this higher tends the walks to be
    ///     more like a Breadth-First Search.
    ///     Having this very high  (> 2) makes search very local.
    ///     Equal to the inverse of p in the Node2Vec paper.
    /// explore_weight: float = 1.0
    ///     Weight on the probability of visiting a neighbor node
    ///     to the one we're coming from in the random walk
    ///     Having this higher tends the walks to be
    ///     more like a Depth-First Search.
    ///     Having this very high makes search more outward.
    ///     Having this very low makes search very local.
    ///     Equal to the inverse of q in the Node2Vec paper.
    /// change_edge_type_weight: float = 1.0
    ///     Weight on the probability of visiting a neighbor node of a
    ///     different type than the previous node. This only applies to
    ///     colored graphs, otherwise it has no impact.
    /// change_node_type_weight: float = 1.0
    ///     Weight on the probability of visiting a neighbor edge of a
    ///     different type than the previous edge. This only applies to
    ///     multigraphs, otherwise it has no impact.
    /// random_state: int = 42
    ///     random_state to use to reproduce the walks.
    /// iterations: int = 1
    ///     Number of cycles on the graphs to execute.
    /// dense_node_mapping: Dict[int, int] = None
    ///     Mapping to use for converting sparse walk space into a dense space.
    ///     This object can be created using the method available from graph
    ///     called `get_dense_node_mapping` that returns a mapping from
    ///     the non trap nodes (those from where a walk could start) and
    ///     maps these nodes into a dense range of values.
    /// max_neighbours: Optional[int] = 100
    ///     Maximum number of randomly sampled neighbours to consider.
    ///     If this parameter is used, the walks becomes probabilistic in nature
    ///     and becomes an approximation of an exact walk.
    ///
    /// Raises
    /// ------
    /// TODO: Update raises
    ///
    /// Returns
    /// -------
    /// List of list of walks containing the numeric IDs of nodes.
    ///
    fn random_walks(
        &self,
        walk_length: u64,
        quantity: NodeT,
        py_kwargs: Option<&PyDict>,
    ) -> PyResult<Py<PyArray2<NodeT>>> {
        let py = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, py.python());

        pe!(validate_kwargs(
            kwargs,
            build_walk_parameters_list(&[]).as_slice()
        ))?;

        let parameters = pe!(self.build_walk_parameters(walk_length, kwargs))?;
        let iter = pe!(self.inner.iter_random_walks(quantity, &parameters))?;
        let array = ThreadDataRaceAware {
            t: PyArray2::new(
                py.python(),
                [
                    quantity as usize * parameters.get_iterations() as usize,
                    walk_length as usize,
                ],
                false,
            ),
        };
        unsafe {
            iter.enumerate().for_each(|(y, vy)| {
                vy.iter()
                    .enumerate()
                    .for_each(|(x, vyx)| *(array.t.uget_mut([y, x])) = *vyx)
            });
        }
        Ok(array.t.to_owned())
    }

    #[args(py_kwargs = "**")]
    #[text_signature = "($self, walk_length, *, return_weight, explore_weight, change_edge_type_weight, change_node_type_weight, random_state, iterations, dense_node_mapping, max_neighbours)"]
    /// Return complete random walks done on the graph using Rust.
    ///
    /// Parameters
    /// ----------
    /// walk_length: int
    ///     Maximal length of the random walk.
    ///     On graphs without traps, all walks have this length.
    /// return_weight: float = 1.0
    ///     Weight on the probability of returning to node coming from
    ///     Having this higher tends the walks to be
    ///     more like a Breadth-First Search.
    ///     Having this very high  (> 2) makes search very local.
    ///     Equal to the inverse of p in the Node2Vec paper.
    /// explore_weight: float = 1.0
    ///     Weight on the probability of visiting a neighbor node
    ///     to the one we're coming from in the random walk
    ///     Having this higher tends the walks to be
    ///     more like a Depth-First Search.
    ///     Having this very high makes search more outward.
    ///     Having this very low makes search very local.
    ///     Equal to the inverse of q in the Node2Vec paper.
    /// change_edge_type_weight: float = 1.0
    ///     Weight on the probability of visiting a neighbor node of a
    ///     different type than the previous node. This only applies to
    ///     colored graphs, otherwise it has no impact.
    /// change_node_type_weight: float = 1.0
    ///     Weight on the probability of visiting a neighbor edge of a
    ///     different type than the previous edge. This only applies to
    ///     multigraphs, otherwise it has no impact.
    /// random_state: int = 42
    ///     random_state to use to reproduce the walks.
    /// iterations: int = 1
    ///     Number of cycles on the graphs to execute.
    /// dense_node_mapping: Dict[int, int] = None
    ///     Mapping to use for converting sparse walk space into a dense space.
    ///     This object can be created using the method available from graph
    ///     called `get_dense_node_mapping` that returns a mapping from
    ///     the non trap nodes (those from where a walk could start) and
    ///     maps these nodes into a dense range of values.
    /// max_neighbours: Optional[int] = 100
    ///     Maximum number of randomly sampled neighbours to consider.
    ///     If this parameter is used, the walks becomes probabilistic in nature
    ///     and becomes an approximation of an exact walk.
    ///
    /// Raises
    /// ------
    /// TODO: Update raises
    ///
    /// Returns
    /// -------
    /// List of list of walks containing the numeric IDs of nodes.
    ///
    fn complete_walks(
        &self,
        walk_length: u64,
        py_kwargs: Option<&PyDict>,
    ) -> PyResult<Py<PyArray2<NodeT>>> {
        let py = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, py.python());

        pe!(validate_kwargs(
            kwargs,
            build_walk_parameters_list(&[]).as_slice()
        ))?;

        let parameters = pe!(self.build_walk_parameters(walk_length, kwargs))?;
        let iter = pe!(self.inner.iter_complete_walks(&parameters))?;
        let array = ThreadDataRaceAware {
            t: PyArray2::new(
                py.python(),
                [
                    self.inner.get_unique_source_nodes_number() as usize
                        * parameters.get_iterations() as usize,
                    walk_length as usize,
                ],
                false,
            ),
        };
        unsafe {
            iter.enumerate().for_each(|(y, vy)| {
                vy.iter()
                    .enumerate()
                    .for_each(|(x, vyx)| *(array.t.uget_mut([y, x])) = *vyx)
            });
        }
        Ok(array.t.to_owned())
    }
}
