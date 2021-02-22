use super::*;
use graph::NodeT;
use numpy::PyArray2;
use rayon::iter::IndexedParallelIterator;
use rayon::prelude::*;
use thread_safe::ThreadSafe;

#[pymethods]
impl EnsmallenGraph {
    #[args(py_kwargs = "**")]
    #[text_signature = "($self, length, quantity, *, return_weight, explore_weight, change_edge_type_weight, change_node_type_weight, random_state, iterations, dense_node_mapping, max_neighbours)"]
    /// Return random walks done on the graph using Rust.
    ///
    /// Parameters
    /// ---------------------
    /// length: int,
    ///     Maximal length of the random walk.
    ///     On graphs without traps, all walks have this length.
    /// quantity: int,
    ///     Number of nodes to sample.
    /// return_weight: float = 1.0,
    ///     Weight on the probability of returning to node coming from
    ///     Having this higher tends the walks to be
    ///     more like a Breadth-First Search.
    ///     Having this very high  (> 2) makes search very local.
    ///     Equal to the inverse of p in the Node2Vec paper.
    /// explore_weight: float = 1.0,
    ///     Weight on the probability of visiting a neighbor node
    ///     to the one we're coming from in the random walk
    ///     Having this higher tends the walks to be
    ///     more like a Depth-First Search.
    ///     Having this very high makes search more outward.
    ///     Having this very low makes search very local.
    ///     Equal to the inverse of q in the Node2Vec paper.
    /// change_edge_type_weight: float = 1.0,
    ///     Weight on the probability of visiting a neighbor node of a
    ///     different type than the previous node. This only applies to
    ///     colored graphs, otherwise it has no impact.
    /// change_node_type_weight: float = 1.0,
    ///     Weight on the probability of visiting a neighbor edge of a
    ///     different type than the previous edge. This only applies to
    ///     multigraphs, otherwise it has no impact.
    /// random_state: int = 42,
    ///     random_state to use to reproduce the walks.
    /// iterations: int = 1,
    ///     Number of cycles on the graphs to execute.
    /// dense_node_mapping: Dict[int, int] = None,
    ///     Mapping to use for converting sparse walk space into a dense space.
    ///     This object can be created using the method available from graph
    ///     called `get_dense_node_mapping` that returns a mapping from
    ///     the non trap nodes (those from where a walk could start) and
    ///     maps these nodes into a dense range of values.
    /// max_neighbours: int = None,
    ///     Maximum number of randomly sampled neighbours to consider.
    ///     If this parameter is used, the walks becomes probabilistic in nature
    ///     and becomes an approximation of an exact walk.
    ///
    /// Raises
    /// ----------------------------
    /// TODO: Update raises
    ///
    /// Returns
    /// ----------------------------
    /// List of list of walks containing the numeric IDs of nodes.
    ///
    fn random_walks(
        &self,
        length: NodeT,
        quantity: NodeT,
        py_kwargs: Option<&PyDict>,
    ) -> PyResult<Py<PyArray2<NodeT>>> {
        let py = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, py.python());

        pyex!(validate_kwargs(kwargs, build_walk_parameters_list(&[])))?;

        let parameters = pyex!(self.build_walk_parameters(length, kwargs))?;
        let iter = pyex!(self.graph.random_walks_iter(quantity, &parameters))?;
        let array = ThreadSafe {
            t: PyArray2::new(
                py.python(),
                [
                    quantity as usize * parameters.get_iterations() as usize,
                    length as usize,
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
    #[text_signature = "($self, length, *, return_weight, explore_weight, change_edge_type_weight, change_node_type_weight, random_state, iterations, dense_node_mapping, max_neighbours)"]
    /// Return complete random walks done on the graph using Rust.
    ///
    /// Parameters
    /// ---------------------
    /// length: int,
    ///     Maximal length of the random walk.
    ///     On graphs without traps, all walks have this length.
    /// return_weight: float = 1.0,
    ///     Weight on the probability of returning to node coming from
    ///     Having this higher tends the walks to be
    ///     more like a Breadth-First Search.
    ///     Having this very high  (> 2) makes search very local.
    ///     Equal to the inverse of p in the Node2Vec paper.
    /// explore_weight: float = 1.0,
    ///     Weight on the probability of visiting a neighbor node
    ///     to the one we're coming from in the random walk
    ///     Having this higher tends the walks to be
    ///     more like a Depth-First Search.
    ///     Having this very high makes search more outward.
    ///     Having this very low makes search very local.
    ///     Equal to the inverse of q in the Node2Vec paper.
    /// change_edge_type_weight: float = 1.0,
    ///     Weight on the probability of visiting a neighbor node of a
    ///     different type than the previous node. This only applies to
    ///     colored graphs, otherwise it has no impact.
    /// change_node_type_weight: float = 1.0,
    ///     Weight on the probability of visiting a neighbor edge of a
    ///     different type than the previous edge. This only applies to
    ///     multigraphs, otherwise it has no impact.
    /// random_state: int = 42,
    ///     random_state to use to reproduce the walks.
    /// iterations: int = 1,
    ///     Number of cycles on the graphs to execute.
    /// dense_node_mapping: Dict[int, int] = None,
    ///     Mapping to use for converting sparse walk space into a dense space.
    ///     This object can be created using the method available from graph
    ///     called `get_dense_node_mapping` that returns a mapping from
    ///     the non trap nodes (those from where a walk could start) and
    ///     maps these nodes into a dense range of values.
    /// max_neighbours: int = None,
    ///     Maximum number of randomly sampled neighbours to consider.
    ///     If this parameter is used, the walks becomes probabilistic in nature
    ///     and becomes an approximation of an exact walk.
    ///
    /// Raises
    /// ----------------------------
    /// TODO: Update raises
    ///
    /// Returns
    /// ----------------------------
    /// List of list of walks containing the numeric IDs of nodes.
    ///
    fn complete_walks(
        &self,
        length: NodeT,
        py_kwargs: Option<&PyDict>,
    ) -> PyResult<Py<PyArray2<NodeT>>> {
        let py = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, py.python());

        pyex!(validate_kwargs(kwargs, build_walk_parameters_list(&[])))?;

        let parameters = pyex!(self.build_walk_parameters(length, kwargs))?;
        let iter = pyex!(self.graph.complete_walks_iter(&parameters))?;
        let array = ThreadSafe {
            t: PyArray2::new(
                py.python(),
                [
                    self.graph.get_unique_sources_number() as usize
                        * parameters.get_iterations() as usize,
                    length as usize,
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
