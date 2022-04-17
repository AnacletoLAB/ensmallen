use super::*;
use numpy::PyArray2;

#[pymethods]
impl Graph {
    #[args(py_kwargs = "**")]
    #[text_signature = "($self, *, embedding_size, epochs, walk_length, return_weight, explore_weight, change_edge_type_weight, change_node_type_weight, iterations, max_neighbours, normalize_by_degree, window_size, number_of_negative_samples, learning_rate, random_state, verbose)"]
    /// Compute the SkipGram node embedding for the current graph.
    /// This method will return a numpy array with shape (number_of_nodes, embedding_size)
    /// where the i-th row is the embedding of the node with node_id i.
    ///
    /// Paramters
    /// ---------
    /// embedding_size: int =
    ///     The number of dimensions of the embedding
    /// epochs: int = 1
    ///     How many epochs the model will train for.
    ///     In this context an epoch means that the model will compute a random
    ///     walk starting from every node in the graph.
    /// walk_length: int = 100
    ///     How many **steps** the random walk will do from it source
    /// return_weight: f32 = 1.0
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
    /// iterations: int = 1
    ///     Number of cycles on the graphs to execute.
    /// max_neighbours: Optional[int] = 100
    ///     Maximum number of randomly sampled neighbours to consider.
    ///     If this parameter is used, the walks becomes probabilistic in nature
    ///     and becomes an approximation of an exact walk.
    /// normalize_by_degree: bool = False
    ///     Divide the weight of each edge by the degree of its destination.
    ///     This is equivalent to computing the laplacian graph but does not
    ///     require to store the weights, so it can use much less ram and thus
    ///     it can be more fit for some graphs.
    /// window_size: int = 4
    ///     Window size to consider for the sequences.
    /// number_of_negative_samples: usize = 5
    ///     How many non-contextual words we will add to the contextual one
    ///     when computing the gradients.
    /// learning_rate: float = 0.025
    ///     Multiplier for the gradient application, lower learning rates
    ///     makes the embeddings slower to converge but might get to a better
    ///     minimum.
    /// random_state: int = 42
    ///     The seed used for all PRNGs, this makes the embeddings almost
    ///     reproducible. Due to concurrencies in the computation of them
    ///     even with the same random_state the embeddings could be different.
    /// verbose: bool = False
    ///     If we should display the progress bar and logs or not.
    fn compute_skipgram_embedding(
        &self,
        embedding_size: Option<usize>,
        epochs: Option<usize>,
        walk_length: Option<u64>,
        return_weight: Option<f32>,
        explore_weight: Option<f32>,
        change_edge_type_weight: Option<f32>,
        change_node_type_weight: Option<f32>,
        iterations: Option<NodeT>,
        max_neighbours: Option<NodeT>,
        normalize_by_degree: Option<bool>,
        window_size: Option<usize>,
        number_of_negative_samples: Option<usize>,
        learning_rate: Option<f32>,
        random_state: Option<u64>,
        verbose: Option<bool>,
    ) -> PyResult<Py<PyArray2<f32>>> {
        let gil = pyo3::Python::acquire_gil();
        let embedding_size = embedding_size.unwrap_or(100);

        let rows_number = self.inner.get_nodes_number() as usize;
        let columns_number = embedding_size;
        let embedding = PyArray2::zeros(gil.python(), [rows_number, columns_number], false);

        let embedding_slice = unsafe { embedding.as_slice_mut().unwrap() };

        pe!(self.inner.compute_skipgram_embedding(
            embedding_slice,
            Some(embedding_size),
            epochs,
            walk_length,
            return_weight,
            explore_weight,
            change_edge_type_weight,
            change_node_type_weight,
            iterations,
            max_neighbours,
            normalize_by_degree,
            window_size,
            number_of_negative_samples,
            learning_rate,
            random_state,
            verbose,
        ))?;

        Ok(embedding.into_py(gil.python()))
    }
}
