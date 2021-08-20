use super::*;
use rayon::prelude::*;
use types::ThreadDataRaceAware;

#[pymethods]
impl EnsmallenGraph {
    #[args(py_kwargs = "**")]
    #[text_signature = "($self, nodes_to_sample_number, random_state, root_node, node_sampling_method, metric)"]
    /// Return subsampled nodes according to the given method and parameters.
    ///
    /// Parameters
    /// --------------------
    /// nodes_to_sample_number: int - The number of nodes to sample.
    /// random_state: int - The random state to reproduce the sampling.
    /// root_node: Optional[int] - The (optional) root node to use to sample. In not provided, a random one is sampled.
    /// node_sampling_method: str - The method to use to sample the nodes. Can either be random nodes, breath first search-based or uniform random walk-based.
    /// metric: List[str] - The metric to use to compute the adjacency matrix.
    ///
    /// Raises
    /// --------------------
    /// ValueError,
    ///     If the given node sampling method is not supported.
    /// ValueError,
    ///     If any of the given subgraph metric is not supported.
    /// ValueError,
    ///     If the list of requested metrics is empty.
    ///
    /// Returns
    /// --------------------
    /// Tuple with the sampled nodes and the computed kernels.
    pub fn get_subgraphs(
        &self,
        nodes_to_sample_number: NodeT,
        random_state: u64,
        root_node: Option<NodeT>,
        node_sampling_method: &str,
        metrics: Vec<&str>,
    ) -> PyResult<(Py<PyArray1<NodeT>>, Vec<Py<PyArray2<WeightT>>>)> {
        // Check if the list of requested metrics is empty.
        if metrics.is_empty() {
            return pe!(Err(concat!(
                "The provided metric list to be used to ",
                "compute the subgraph kernels is empty."
            )));
        }
        // We sample the nodes.
        // We cannot directly allocate this into a
        // numpy array because
        let nodes = pe!(self.graph.get_subsampled_nodes(
            nodes_to_sample_number,
            random_state,
            root_node,
            node_sampling_method,
        ))?;

        // Some of the sampling mechanism are not guaranteed
        // to actually return the requested number of nodes.
        // For instance, sampling of BFS nodes from a component
        // that does not contain the requested number of nodes.
        let nodes_number = nodes.len();

        // Acquire the python gil.
        let gil = pyo3::Python::acquire_gil();

        // Compute the required kernels.
        let kernels = pe!(metrics
            .into_iter()
            .map(|metric| unsafe {
                let kernel = ThreadDataRaceAware {
                    t: PyArray2::zeros(gil.python(), [nodes_number, nodes_number], false),
                };
                if metric == "laplacian" {
                    self.graph
                        .par_iter_subsampled_weighted_adjacency_matrix(&nodes)?
                        .for_each(|(_, i, _, j, value)| {
                            *kernel.t.uget_mut([i, j]) = value;
                        });
                } else {
                    self.graph
                        .par_iter_subsampled_edge_metric_matrix(&nodes, metric)?
                        .for_each(|(_, i, _, j, value)| {
                            *kernel.t.uget_mut([i, j]) = value;
                        });
                }
                Ok(kernel.t.to_owned())
            })
            .collect::<Result<Vec<Py<PyArray2<WeightT>>>>>())?;

        // We now convert the provided nodes into a numpy vector.
        let numpy_nodes = ThreadDataRaceAware {
            t: PyArray1::new(gil.python(), [nodes_number], false),
        };

        // We consume the original vector to populate the numpy one.
        nodes
            .into_par_iter()
            .enumerate()
            .for_each(|(i, node_id)| unsafe { *numpy_nodes.t.uget_mut([i]) = node_id });

        // And return the sampled nodes and kernel
        Ok((numpy_nodes.t.to_owned(), kernels))
    }
}
