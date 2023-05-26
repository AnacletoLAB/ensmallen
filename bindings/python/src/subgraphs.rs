use super::*;
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rayon::prelude::*;
use types::ThreadDataRaceAware;
use vec_rand::splitmix64;

#[pymethods]
impl Graph {
    #[args(py_kwargs = "**")]
    #[pyo3(
        text_signature = "($self, number_of_nodes_to_sample, random_state, root_node, node_sampling_method, edge_weighting_methods, add_selfloops_where_missing, unique)"
    )]
    /// Return subsampled nodes according to the given method and parameters.
    ///
    /// Parameters
    /// ----------
    /// number_of_nodes_to_sample: int
    ///     the number of nodes to sample.
    /// random_state: int
    ///     The random state to reproduce the sampling.
    /// root_node: Optional[int]
    ///     The (optional) root node to use to sample. In not provided, a random one is sampled.
    /// node_sampling_method: str
    ///     The method to use to sample the nodes. Can either be random nodes, breath first search-based or uniform random walk-based.
    /// edge_weighting_methods: List[str]
    ///     The edge weighting methods to use to compute the adjacency matrix.
    /// add_selfloops_where_missing: Optional[bool]
    ///     Whether to add selfloops where they are missing. This parameter only applies to laplacian edge weighting method. By default, true.
    /// unique: Optional[bool] = True
    ///     Whether to reduce the sampled nodes to a unique set.
    ///
    /// Raises
    /// ------
    /// ValueError
    ///     If the given node sampling method is not supported.
    /// ValueError
    ///     If any of the given subgraph edge weighting method is not supported.
    /// ValueError
    ///     If the list of requested edge weighting methods is empty.
    /// ValueError
    ///     If the `add_selfloops_where_missing` parameter is provided, but the edge weighting method is not laplacian.
    ///
    /// Returns
    /// -------
    /// Tuple with the sampled nodes and the computed kernels.
    pub fn get_subgraphs(
        &self,
        number_of_nodes_to_sample: NodeT,
        random_state: u64,
        root_node: Option<NodeT>,
        node_sampling_method: &str,
        edge_weighting_methods: Vec<&str>,
        add_selfloops_where_missing: Option<bool>,
        unique: Option<bool>,
    ) -> PyResult<(Py<PyArray1<NodeT>>, Vec<Py<PyArray2<WeightT>>>)> {
        // Check if the list of requested edge weighting methods is empty.
        if edge_weighting_methods.is_empty() {
            return pe!(Err(concat!(
                "The provided edge weighting methods list to be used to ",
                "compute the subgraph kernels is empty."
            )));
        }
        // Check if an illegal parametrization was provided.
        if add_selfloops_where_missing.is_some()
            && edge_weighting_methods
                .iter()
                .all(|&edge_weighting_method| edge_weighting_method != "laplacian")
        {
            return pe!(Err(concat!(
                "The parameter add_selfloops_where_missing was provided ",
                "with a non-None value ",
                "but this only makes sense when used with the ",
                "`laplacian` edge weighting method, and none of the requested edge weighting methods were `laplacian`."
            )
            .to_string()));
        }

        let unique = unique.unwrap_or(true);

        // We sample the nodes.
        // We cannot directly allocate this into a
        // numpy array because
        let nodes = pe!(self.inner.get_subsampled_nodes(
            number_of_nodes_to_sample,
            random_state,
            node_sampling_method,
            root_node,
            Some(unique)
        ))?;

        // Some of the sampling mechanism are not guaranteed
        // to actually return the requested number of nodes.
        // For instance, sampling of BFS nodes from a component
        // that does not contain the requested number of nodes.
        let number_of_nodes = nodes.len();

        // Acquire the python gil.
        let gil = pyo3::Python::acquire_gil();

        // Store whether the graph is undirected
        let is_undirected = !self.inner.is_directed();

        // Compute the required kernels.
        let kernels = pe!(edge_weighting_methods
            .into_iter()
            .map(|edge_weighting_method| unsafe {
                // We create the kernel that we will populate
                // with this particular iteration.
                let kernel = ThreadDataRaceAware {
                    // If the edge weighting method for the kernel is either weights
                    // or laplacian, that is edge weighting methods that are not fully defined for all
                    // of the edges of the graph, we need to set the remaining values
                    // to zeros, so we initialize the vector as a matrix of zeros.
                    t: if edge_weighting_method == "weights" || edge_weighting_method == "laplacian"
                    {
                        PyArray2::zeros(gil.python(), [number_of_nodes, number_of_nodes], false)
                    } else {
                        // The same consideration does not apply to edge weighting methods that are fully
                        // defined for all the possible tuple of nodes that may be taken
                        // into consideration: in this case all the values will be provided
                        // by the iterator, and therefore there is no need to set beforehand
                        // a default value: doing so would only be a waste of time.
                        PyArray2::new(gil.python(), [number_of_nodes, number_of_nodes], false)
                    },
                };
                // In order to avoid repeating the same logic,
                // even though arguably small, we define a function
                // that captures the kernel.
                let build_kernel = |(i, j, value)| {
                    *kernel.t.uget_mut([i, j]) = value;
                    if is_undirected && i != j {
                        *kernel.t.uget_mut([j, i]) = value;
                    }
                };
                // If the required edge weighting method are the weights, we extract the weights
                // iterator and populate the kernel using the weights.
                if edge_weighting_method == "weights" {
                    self.inner
                        .par_iter_subsampled_weighted_adjacency_matrix(&nodes, Some(false))?
                        .map(|(_, i, _, j, weight)| (i, j, weight))
                        .for_each(build_kernel);
                // Similarly, if the required edge weighting method is the laplacian
                // we populate the kernel with the laplacian.
                } else if edge_weighting_method == "laplacian" {
                    self.inner
                        .par_iter_subsampled_symmetric_laplacian_adjacency_matrix(
                            &nodes,
                            add_selfloops_where_missing,
                            Some(false),
                        )
                        .for_each(build_kernel);
                } else {
                    // Finally, all other edge weighting methods that are defined for all
                    // the node tuples are handled by this auto-dispatching method
                    self.inner
                        .par_iter_subsampled_edge_metric_matrix(&nodes, edge_weighting_method)?
                        .map(|(_, i, _, j, weight)| (i, j, weight))
                        .for_each(build_kernel);
                }
                // Once the kernel is ready, we extract it from the unsafe cell
                // and return it.
                Ok(kernel.t.to_owned())
            })
            .collect::<Result<Vec<Py<PyArray2<WeightT>>>>>())?;

        // We now convert the provided nodes into a numpy vector.
        let numpy_nodes = ThreadDataRaceAware {
            t: unsafe { PyArray1::new(gil.python(), [number_of_nodes], false) },
        };

        // We consume the original vector to populate the numpy one.
        nodes
            .into_par_iter()
            .enumerate()
            .for_each(|(i, node_id)| unsafe { *numpy_nodes.t.uget_mut([i]) = node_id });

        // And return the sampled nodes and kernel
        Ok((numpy_nodes.t.to_owned(), kernels))
    }

    #[args(py_kwargs = "**")]
    #[pyo3(text_signature = "($self, node_ids, add_selfloops_where_missing, complete)")]
    /// Return subsampled edges connected to the given node Ids.
    ///
    /// Parameters
    /// --------------------
    /// node_ids: List[int]
    ///     List of nodes whose edges are to return.
    /// add_selfloops_where_missing: Optional[bool]
    ///     Whether to add selfloops where they are missing. This parameter only applies to laplacian edge weighting method. By default, true.
    /// complete: Optional[bool] = True
    ///     Whether to return the edges in both directions (when dealing with an undirected graph).
    ///
    /// Returns
    /// --------------------
    /// Tuple with the sampled nodes and the computed kernels.
    pub fn get_edge_ids_from_node_ids(
        &self,
        node_ids: Vec<NodeT>,
        add_selfloops_where_missing: Option<bool>,
        complete: Option<bool>,
    ) -> PyResult<Py<PyArray2<NodeT>>> {
        // Acquire the python gil.
        let gil = pyo3::Python::acquire_gil();

        // If the required edge weighting method are the weights, we extract the weights
        // iterator and populate the kernel using the weights.
        let mut edge_ids: Vec<(NodeT, NodeT)> = unsafe {
            self.inner.par_iter_subsampled_binary_adjacency_matrix(
                &node_ids,
                add_selfloops_where_missing,
                complete,
            )
        }
        .map(|(src, _, dst, _)| (src, dst))
        .collect();

        edge_ids.par_sort_unstable();
        let number_of_edges = edge_ids.len();

        let edge_ids_vector = ThreadDataRaceAware {
            t: unsafe { PyArray2::new(gil.python(), [number_of_edges, 2], false) },
        };

        edge_ids
            .into_par_iter()
            .enumerate()
            .for_each(|(i, (src, dst))| unsafe {
                *edge_ids_vector.t.uget_mut([i, 0]) = src;
                *edge_ids_vector.t.uget_mut([i, 1]) = dst;
            });

        Ok(edge_ids_vector.t.to_owned())
    }

    #[args(py_kwargs = "**")]
    #[pyo3(
        text_signature = "($self, number_of_nodes_to_sample, random_state, root_node, node_sampling_method, edge_weighting_methods, add_selfloops_where_missing, unique)"
    )]
    /// Return subsampled nodes according to the given method and parameters.
    ///
    /// Parameters
    /// --------------------
    /// number_of_nodes_to_sample: int
    ///     The number of nodes to sample.
    /// random_state: int
    ///     The random state to reproduce the sampling.
    /// root_node: Optional[int]
    ///     The (optional) root node to use to sample. In not provided, a random one is sampled.
    /// node_sampling_method: str
    ///     The method to use to sample the nodes. Can either be random nodes, breath first search-based or uniform random walk-based.
    /// edge_weighting_methods: List[str]
    ///     The edge weighting methods to use to compute the adjacency matrix.
    /// add_selfloops_where_missing: Optional[bool]
    ///     Whether to add selfloops where they are missing. This parameter only applies to laplacian edge weighting method. By default, true.
    /// unique: Optional[bool] = True
    ///     Whether to reduce the sampled nodes to a unique set.
    ///
    /// Raises
    /// --------------------
    /// ValueError
    ///     If the given node sampling method is not supported.
    /// ValueError
    ///     If any of the given subgraph edge weighting method is not supported.
    /// ValueError
    ///     If the list of requested edge weighting methods is empty.
    /// ValueError
    ///     If the `add_selfloops_where_missing` parameter is provided, but the edge weighting method is not laplacian.
    ///
    /// Returns
    /// --------------------
    /// Tuple with the sampled nodes and the computed kernels.
    pub fn get_sparse_subgraphs(
        &self,
        number_of_nodes_to_sample: NodeT,
        random_state: u64,
        root_node: Option<NodeT>,
        node_sampling_method: &str,
        edge_weighting_methods: Vec<&str>,
        add_selfloops_where_missing: Option<bool>,
        unique: Option<bool>,
    ) -> PyResult<(
        Py<PyArray1<NodeT>>,
        Vec<(Py<PyArray2<usize>>, Py<PyArray1<WeightT>>)>,
    )> {
        // Check if the list of requested edge weighting methods is empty.
        if edge_weighting_methods.is_empty() {
            return pe!(Err(concat!(
                "The provided edge weighting methods list to be used to ",
                "compute the subgraph kernels is empty."
            )));
        }
        // Check if an illegal parametrization was provided.
        if add_selfloops_where_missing.is_some()
            && edge_weighting_methods
                .iter()
                .all(|&edge_weighting_method| edge_weighting_method != "laplacian")
        {
            return pe!(Err(concat!(
                "The parameter add_selfloops_where_missing was provided ",
                "with a non-None value ",
                "but this only makes sense when used with the ",
                "`laplacian` edge weighting method, and none of the requested edge weighting methods were `laplacian`."
            )
            .to_string()));
        }

        let unique = unique.unwrap_or(true);

        // We sample the nodes.
        // We cannot directly allocate this into a
        // numpy array because
        let nodes = pe!(self.inner.get_subsampled_nodes(
            number_of_nodes_to_sample,
            random_state,
            node_sampling_method,
            root_node,
            Some(unique)
        ))?;

        // Some of the sampling mechanism are not guaranteed
        // to actually return the requested number of nodes.
        // For instance, sampling of BFS nodes from a component
        // that does not contain the requested number of nodes.
        let number_of_nodes = nodes.len();

        // Acquire the python gil.
        let gil = pyo3::Python::acquire_gil();

        // Compute the required kernels.
        let sparse_kernels = pe!(edge_weighting_methods
            .into_iter()
            .map(|edge_weighting_method| unsafe {
                // If the required edge weighting method are the weights, we extract the weights
                // iterator and populate the kernel using the weights.
                let mut edge_node_ids_and_weights: Vec<(usize, usize, WeightT)> =
                    if edge_weighting_method == "weights" {
                        self.inner
                            .par_iter_subsampled_weighted_adjacency_matrix(&nodes, Some(true))?
                            .map(|(_, src, _, dst, weight)| (src, dst, weight))
                            .collect()
                    // Similarly, if the required edge weighting method is the laplacian
                    // we populate the kernel with the laplacian.
                    } else if edge_weighting_method == "laplacian" {
                        self.inner
                            .par_iter_subsampled_symmetric_laplacian_adjacency_matrix(
                                &nodes,
                                add_selfloops_where_missing,
                                Some(true),
                            )
                            .collect()
                    } else {
                        return Err(format!(
                            concat!(
                                "The provided edge weighting method {} is ",
                                "not supported."
                            ),
                            edge_weighting_method
                        ));
                    };
                edge_node_ids_and_weights.par_sort_unstable_by(
                    |(src_a, dst_a, _), (src_b, dst_b, _)| (src_a, dst_a).cmp(&(src_b, dst_b)),
                );

                let number_of_edges = edge_node_ids_and_weights.len();

                let edge_ids_vector = ThreadDataRaceAware {
                    t: PyArray2::new(gil.python(), [number_of_edges, 2], false),
                };
                let weights = ThreadDataRaceAware {
                    t: PyArray1::new(gil.python(), [number_of_edges], false),
                };

                edge_node_ids_and_weights
                    .into_par_iter()
                    .enumerate()
                    .for_each(|(i, (src, dst, weight))| {
                        *edge_ids_vector.t.uget_mut([i, 0]) = src;
                        *edge_ids_vector.t.uget_mut([i, 1]) = dst;
                        *weights.t.uget_mut([i]) = weight;
                    });

                // Once the kernel is ready, we extract it from the unsafe cell
                // and return it.
                Ok((edge_ids_vector.t.to_owned(), weights.t.to_owned()))
            })
            .collect::<Result<Vec<(Py<PyArray2<usize>>, Py<PyArray1<WeightT>>)>>>())?;

        // We now convert the provided nodes into a numpy vector.
        let numpy_nodes = ThreadDataRaceAware {
            t: unsafe { PyArray1::new(gil.python(), [number_of_nodes], false) },
        };

        // We consume the original vector to populate the numpy one.
        nodes
            .into_par_iter()
            .enumerate()
            .for_each(|(i, node_id)| unsafe { *numpy_nodes.t.uget_mut([i]) = node_id });

        // And return the sampled nodes and kernel
        Ok((numpy_nodes.t.to_owned(), sparse_kernels))
    }

    #[args(py_kwargs = "**")]
    #[pyo3(
        text_signature = "($self, number_of_nodes_to_sample, random_state, root_node, node_sampling_method)"
    )]
    /// Return subsampled nodes and edges using laplacian assuming undirected graph with selfloops.
    ///
    /// Parameters
    /// --------------------
    /// number_of_nodes_to_sample: int
    ///     The number of nodes to sample.
    /// random_state: int
    ///     The random state to reproduce the sampling.
    /// root_node: Optional[int]
    ///     The (optional) root node to use to sample. In not provided, a random one is sampled.
    /// node_sampling_method: str
    ///     The method to use to sample the nodes. Can either be random nodes, breath first search-based or uniform random walk-based.
    ///
    /// Raises
    /// --------------------
    /// TODO: Update
    ///
    /// Returns
    /// --------------------
    /// Tuple with the sampled nodes and the computed kernels.
    pub fn get_sparse_undirected_laplacian_subgraphs(
        &self,
        number_of_nodes_to_sample: NodeT,
        random_state: u64,
        root_node: Option<NodeT>,
        node_sampling_method: &str,
    ) -> PyResult<(
        Py<PyArray1<NodeT>>,
        Vec<(Py<PyArray2<usize>>, Py<PyArray1<WeightT>>)>,
    )> {
        // We sample the nodes.
        // We cannot directly allocate this into a
        // numpy array because
        let mut nodes = pe!(self.inner.get_subsampled_nodes(
            number_of_nodes_to_sample,
            random_state,
            node_sampling_method,
            root_node,
            Some(true)
        ))?;

        nodes.par_sort_unstable();

        // Some of the sampling mechanism are not guaranteed
        // to actually return the requested number of nodes.
        // For instance, sampling of BFS nodes from a component
        // that does not contain the requested number of nodes.
        let number_of_nodes = nodes.len();

        // Acquire the python gil.
        let gil = pyo3::Python::acquire_gil();

        // Compute the required kernel.
        let sorted_edge_node_ids_and_weights = pe!(unsafe {
            self.inner
                .par_iter_undirected_with_selfloops_subsampled_symmetric_laplacian_adjacency_matrix(
                    &nodes,
                )
        })?
        .collect::<Vec<_>>();

        let number_of_edges = sorted_edge_node_ids_and_weights.len();

        let edge_ids_vector = ThreadDataRaceAware {
            t: unsafe { PyArray2::new(gil.python(), [number_of_edges, 2], false) },
        };
        let weights = ThreadDataRaceAware {
            t: unsafe { PyArray1::new(gil.python(), [number_of_edges], false) },
        };

        sorted_edge_node_ids_and_weights
            .into_par_iter()
            .enumerate()
            .for_each(|(i, (src, dst, weight))| unsafe {
                *edge_ids_vector.t.uget_mut([i, 0]) = src;
                *edge_ids_vector.t.uget_mut([i, 1]) = dst;
                *weights.t.uget_mut([i]) = weight;
            });

        // We now convert the provided nodes into a numpy vector.
        let numpy_nodes = ThreadDataRaceAware {
            t: unsafe { PyArray1::new(gil.python(), [number_of_nodes], false) },
        };

        // We consume the original vector to populate the numpy one.
        nodes
            .into_par_iter()
            .enumerate()
            .for_each(|(i, node_id)| unsafe { *numpy_nodes.t.uget_mut([i]) = node_id });

        // And return the sampled nodes and kernel
        Ok((
            numpy_nodes.t.to_owned(),
            vec![(edge_ids_vector.t.to_owned(), weights.t.to_owned())],
        ))
    }

    #[args(py_kwargs = "**")]
    #[pyo3(
        text_signature = "($self, number_of_nodes_to_sample, random_state, node_sampling_method, edge_weighting_methods, add_selfloops_where_missing)"
    )]
    /// Return subsampled nodes according to the given method and parameters.
    ///
    /// Parameters
    /// --------------------
    /// number_of_nodes_to_sample: int
    ///     The number of nodes to sample.
    /// random_state: int
    ///     The random state to reproduce the sampling.
    /// node_sampling_method: str
    ///     The method to use to sample the nodes. Can either be random nodes, breath first search-based or uniform random walk-based.
    /// edge_weighting_methods: List[str]
    ///     The edge weighting methods to use to compute the adjacency matrix.
    /// add_selfloops_where_missing: Optional[bool]
    ///     Whether to add selfloops where they are missing. This parameter only applies to laplacian edge weighting method. By default, true.
    ///
    /// Raises
    /// --------------------
    /// ValueError
    ///     If the given node sampling method is not supported.
    /// ValueError
    ///     If any of the given subgraph edge weighting method is not supported.
    /// ValueError
    ///     If the list of requested edge weighting methods is empty.
    /// ValueError
    ///     If the `add_selfloops_where_missing` parameter is provided, but the edge weighting method is not laplacian.
    ///
    /// Returns
    /// --------------------
    /// Tuple with the sampled nodes and the computed kernels.
    pub fn get_edge_prediction_subgraphs(
        &self,
        number_of_nodes_to_sample: NodeT,
        random_state: u64,
        node_sampling_method: &str,
        edge_weighting_methods: Vec<&str>,
        add_selfloops_where_missing: Option<bool>,
    ) -> PyResult<(
        Py<PyArray1<NodeT>>,
        Vec<Py<PyArray2<WeightT>>>,
        Py<PyArray1<NodeT>>,
        Vec<Py<PyArray2<WeightT>>>,
        Py<PyArray1<bool>>,
    )> {
        // Check if the list of requested edge weighting methods is empty.
        if edge_weighting_methods.is_empty() {
            return pe!(Err(concat!(
                "The provided edge weighting method list to be used to ",
                "compute the subgraph kernels is empty."
            )));
        }
        // Check if an illegal parametrization was provided.
        if add_selfloops_where_missing.is_some()
            && edge_weighting_methods
                .iter()
                .all(|&edge_weighting_method| edge_weighting_method != "laplacian")
        {
            return pe!(Err(concat!(
                "The parameter add_selfloops_where_missing was provided ",
                "with a non-None value ",
                "but this only makes sense when used with the ",
                "`laplacian` edge weighting method, and none of the requested edge weighting methods were `laplacian`."
            )
            .to_string()));
        }
        // We sample the nodes.
        // We cannot directly allocate this into a
        // numpy array because
        let source_nodes = pe!(self.inner.get_subsampled_nodes(
            number_of_nodes_to_sample,
            random_state,
            node_sampling_method,
            None,
            Some(false)
        ))?;

        // Some of the sampling mechanism are not guaranteed
        // to actually return the requested number of nodes.
        // For instance, sampling of BFS nodes from a component
        // that does not contain the requested number of nodes.
        let number_of_nodes = source_nodes.len();

        let mut remapping = (0..number_of_nodes).collect::<Vec<usize>>();
        let mut rng = SmallRng::seed_from_u64(splitmix64(random_state) as EdgeT);
        remapping.shuffle(&mut rng);

        let destination_nodes = remapping
            .iter()
            .map(|&i| source_nodes[i])
            .collect::<Vec<NodeT>>();

        // Acquire the python gil.
        let gil = pyo3::Python::acquire_gil();

        // Store whether the graph is undirected
        let is_undirected = !self.inner.is_directed();

        // Compute the required kernels.
        let (source_kernels, destination_kernels) = pe!(edge_weighting_methods
            .into_iter()
            .map(|edge_weighting_method| unsafe {
                // We create the kernel that we will populate
                // with this particular iteration.
                let (source_kernel, destination_kernel) = if edge_weighting_method == "weights"
                    || edge_weighting_method == "laplacian"
                {
                    // If the edge weighting method for the kernel is either weights
                    // or laplacian, that is edge weighting methods that are not fully defined for all
                    // of the edges of the graph, we need to set the remaining values
                    // to zeros, so we initialize the vector as a matrix of zeros.
                    (
                        ThreadDataRaceAware {
                            t: PyArray2::zeros(gil.python(), [number_of_nodes, number_of_nodes], false),
                        },
                        ThreadDataRaceAware {
                            t: PyArray2::zeros(gil.python(), [number_of_nodes, number_of_nodes], false),
                        },
                    )
                } else {
                    // The same consideration does not apply to edge weighting methods that are fully
                    // defined for all the possible tuple of nodes that may be taken
                    // into consideration: in this case all the values will be provided
                    // by the iterator, and therefore there is no need to set beforehand
                    // a default value: doing so would only be a waste of time.
                    (
                        ThreadDataRaceAware {
                            t: PyArray2::new(gil.python(), [number_of_nodes, number_of_nodes], false),
                        },
                        ThreadDataRaceAware {
                            t: PyArray2::new(gil.python(), [number_of_nodes, number_of_nodes], false),
                        },
                    )
                };
                // In order to avoid repeating the same logic,
                // even though arguably small, we define a function
                // that captures the kernel.
                let build_kernel = |(i, j, value)| {
                    let remapped_i = remapping[i];
                    let remapped_j = remapping[j];
                    *source_kernel.t.uget_mut([i, j]) = value;
                    *destination_kernel.t.uget_mut([remapped_i, remapped_j]) = value;
                    if is_undirected && i != j {
                        *source_kernel.t.uget_mut([j, i]) = value;
                        *destination_kernel.t.uget_mut([remapped_j, remapped_i]) = value;
                    }
                };
                // If the required edge weighting method are the weights, we extract the weights
                // iterator and populate the kernel using the weights.
                if edge_weighting_method == "weights" {
                    self.inner
                        .par_iter_subsampled_weighted_adjacency_matrix(&source_nodes, Some(false))?
                        .map(|(_, i, _, j, weight)| (i, j, weight))
                        .for_each(build_kernel);
                // Similarly, if the required edge weighting method is the laplacian
                // we populate the kernel with the laplacian.
                } else if edge_weighting_method == "laplacian" {
                    self.inner
                        .par_iter_subsampled_symmetric_laplacian_adjacency_matrix(
                            &source_nodes,
                            add_selfloops_where_missing,
                            Some(false),
                        )
                        .for_each(build_kernel);
                } else {
                    // Finally, all other edge weighting methods that are defined for all
                    // the node tuples are handled by this auto-dispatching method
                    self.inner
                        .par_iter_subsampled_edge_metric_matrix(
                            &source_nodes,
                            edge_weighting_method,
                        )?
                        .map(|(_, i, _, j, weight)| (i, j, weight))
                        .for_each(build_kernel);
                }
                // Once the kernel is ready, we extract it from the unsafe cell
                // and return it.
                Ok((source_kernel.t.to_owned(), destination_kernel.t.to_owned()))
            })
            .collect::<Result<Vec<(Py<PyArray2<WeightT>>, Py<PyArray2<WeightT>>)>>>())?
        .into_iter()
        .unzip();

        // We now convert the provided nodes into a numpy vector.
        let numpy_labels = ThreadDataRaceAware {
            t: unsafe { PyArray1::new(gil.python(), [number_of_nodes], false) },
        };

        // We consume the original vector to populate the numpy one.
        source_nodes
            .par_iter()
            .zip(destination_nodes.par_iter())
            .enumerate()
            .for_each(|(i, (&src, &dst))| unsafe {
                *numpy_labels.t.uget_mut([i]) = self.inner.has_edge_from_node_ids(src, dst);
            });

        // We now convert the provided nodes into a numpy vector.
        let numpy_source_nodes = ThreadDataRaceAware {
            t: unsafe { PyArray1::new(gil.python(), [number_of_nodes], false) },
        };

        // We consume the original vector to populate the numpy one.
        source_nodes
            .into_par_iter()
            .enumerate()
            .for_each(|(i, node_id)| unsafe { *numpy_source_nodes.t.uget_mut([i]) = node_id });

        // We now convert the provided nodes into a numpy vector.
        let numpy_destination_nodes = ThreadDataRaceAware {
            t: unsafe { PyArray1::new(gil.python(), [number_of_nodes], false) },
        };

        // We consume the original vector to populate the numpy one.
        destination_nodes
            .into_par_iter()
            .enumerate()
            .for_each(|(i, node_id)| unsafe { *numpy_destination_nodes.t.uget_mut([i]) = node_id });

        // And return the sampled nodes and kernel
        Ok((
            numpy_source_nodes.t.to_owned(),
            source_kernels,
            numpy_destination_nodes.t.to_owned(),
            destination_kernels,
            numpy_labels.t.to_owned(),
        ))
    }

    #[args(py_kwargs = "**")]
    #[pyo3(
        text_signature = "($self, number_of_nodes_to_sample, random_state, source_root_node, destination_root_node, node_sampling_method, edge_weighting_methods, add_selfloops_where_missing, unique)"
    )]
    /// Return subsampled nodes according to the given method and parameters.
    ///
    /// Parameters
    /// ----------
    /// number_of_nodes_to_sample: int
    ///     The number of nodes to sample.
    /// random_state: int
    ///     The random state to reproduce the sampling.
    /// source_root_node: int
    ///     The source root node to use to sample. In not provided, a random one is sampled.
    /// destination_root_node: int
    ///     The destination root node to use to sample. In not provided, a random one is sampled.
    /// node_sampling_method: str
    ///     The method to use to sample the nodes. Can either be random nodes, breath first search-based or uniform random walk-based.
    /// edge_weighting_methods: List[str]
    ///     The edge weighting methods to use to compute the adjacency matrix.
    /// add_selfloops_where_missing: Optional[bool]
    ///     Whether to add selfloops where they are missing. This parameter only applies to laplacian edge weighting method. By default, true.
    /// unique: Optional[bool] = True
    ///     Whether to reduce the sampled nodes to a unique set.
    ///
    /// Raises
    /// ------
    /// ValueError
    ///     If the given node sampling method is not supported.
    /// ValueError
    ///     If any of the given subgraph edge weighting method is not supported.
    /// ValueError
    ///     If the list of requested edge weighting methods is empty.
    /// ValueError
    ///     If the `add_selfloops_where_missing` parameter is provided, but the edge weighting method is not laplacian.
    ///
    /// Returns
    /// -------
    /// Tuple with the sampled nodes and the computed kernels.
    pub fn get_edge_prediction_subgraphs_from_node_ids(
        &self,
        number_of_nodes_to_sample: NodeT,
        random_state: u64,
        source_root_node: NodeT,
        destination_root_node: NodeT,
        node_sampling_method: &str,
        edge_weighting_methods: Vec<&str>,
        add_selfloops_where_missing: Option<bool>,
        unique: Option<bool>,
    ) -> PyResult<(
        Py<PyArray1<NodeT>>,
        Vec<Py<PyArray2<WeightT>>>,
        Py<PyArray1<NodeT>>,
        Vec<Py<PyArray2<WeightT>>>,
        bool,
    )> {
        let (src_nodes, src_kernels) = self.get_subgraphs(
            number_of_nodes_to_sample,
            random_state,
            Some(source_root_node),
            node_sampling_method,
            edge_weighting_methods.clone(),
            add_selfloops_where_missing,
            unique,
        )?;
        let (dst_nodes, dst_kernels) = self.get_subgraphs(
            number_of_nodes_to_sample,
            random_state,
            Some(destination_root_node),
            node_sampling_method,
            edge_weighting_methods,
            add_selfloops_where_missing,
            unique,
        )?;

        Ok((
            src_nodes,
            src_kernels,
            dst_nodes,
            dst_kernels,
            self.inner
                .has_edge_from_node_ids(source_root_node, destination_root_node),
        ))
    }
}
