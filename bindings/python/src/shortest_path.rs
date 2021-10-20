use super::*;
use rayon::prelude::*;

macro_rules! impl_get_shortest_paths_node_embedding {
    ($($dtype:ty),*) => {
        #[pymethods]
        impl Graph {
            #[args(py_kwargs = "**")]
            #[text_signature = "($self, node_centralities, node_centralities_distribution, adjust_by_central_node_distance, number_of_nodes_to_sample_per_feature, maximum_number_of_features, validate_node_centralities, maximal_depth, central_node_name, central_node_id, random_state, return_sampled_node_names, dtype, verbose)"]
            /// Return node embedding vector obtained from shortest-paths.
            ///
            /// Parameters
            /// ----------------------------
            /// node_centralities: Optional[List[float]] = None
            ///     Vector with the importance of the nodes, used to properly sample the anchors. By default node degree centralities are used. Nodes with node centrality zero won't ever be sampled as an anchor, except for when all other nodes were already sampled.
            /// node_centralities_distribution: Optional[str] = None
            ///     Distribution expected out of the provided node centralities distribution. If no distribution is provided and no node centralities are provided, exponential is used as the default node centrality distribution as node degree centrality is employed as centrality.
            /// adjust_by_central_node_distance: Optional[bool] = True
            ///     Whether to adjust the node eccentricity by the normalized distance to the most central node. By default true.
            /// number_of_nodes_to_sample_per_feature: Optional[int] = 10
            ///     Number of nodes to sample per feature. By default 10.
            /// maximum_number_of_features`: Optional[int]
            ///     Maximum number of node features to generate. By default 50.
            /// remove_neighbouring_nodes: Optional[bool] = True
            ///     Whether to remove the neighbouring nodes from the set of samplable anchor nodes. By default true.
            /// validate_node_centralities: Optional[bool] = True
            ///     Whether to validate the node centralities. By default true when the node centralities are provided.
            /// maximal_depth: Optional[int] = None
            ///     The maximal depth to use if node features are to be focused in a local area of the graph.
            /// central_node_name: Optional[str] = None
            ///     The node name from where to start sampling the BFS mask for the maximal depth. Either the node name of the node ID can be provided at once.
            /// central_node_id: Optional[int] = None
            ///     The node ID from where to start sampling the BFS mask for the maximal depth. Either the node name of the node ID can be provided at once.
            /// random_state: Optional[int] = 42
            ///     The random state to use to sample the central node. By default 42.
            /// return_sampled_node_names: Optional[bool] = None
            ///     Whether to return the name of the sampled nodes. By default true if the `number_of_nodes_to_sample_per_feature` parameter is less than 100.
            /// verbose: Optional[bool] = True
            ///     Whether to show the loading bar. By default true.
            ///
            /// Details on the supported node centrality distributions
            /// ----------------------------------------------------------------------
            /// The node centrality distributions are used to find an optimal threshold that avoids
            /// sorting nodes that include also non-useful nodes, that is nodes that we will never
            /// be interested in sampling. We currently support the following node centrality distributions:
            ///
            /// Exponential
            /// ~~~~~~~~~~~~~~~~~~~~~
            /// The most common distribution for node centralities is the exponential distribution.
            /// Most likely, your node centralities will follow this distribution.
            ///
            /// Geometric
            /// ~~~~~~~~~~~~~~~~~~~~~
            /// The geometric distribution is to be used for an integer distribution, when the normalization
            /// by the distances from the most central node is disabled (or it will make the distribution a float value).
            ///
            /// Unknown
            /// ~~~~~~~~~~~~~~~~~~~~~
            /// For now we do not have support for other distributions implemented, so if the distribution
            /// is not expected to be one of the aforementioned distributions we will use zero as a threshold.
            ///
            /// Raises
            /// ----------------------------------------------------------------------
            /// * If the provided node centralities are not provided for all features.
            /// * If the provided node centralities contain illegal values, like NaNs or infinities.
            /// * If the provided node centralities are not normalized.
            /// * If the number of maximum features is zero.
            /// * If the edge weights are requested but the graph does not have edge weights.
            /// * If the use edge weights as probabilities is requested, but the graph does not have edge weights as probabilities (between 0 and 1).
            /// * If the use edge weights as probabilities is requested, but not the edge weights.
            pub fn get_shortest_paths_node_embedding(
                &self,
                node_centralities: Option<Vec<f32>>,
                node_centralities_distribution: Option<&str>,
                adjust_by_central_node_distance: Option<bool>,
                number_of_nodes_to_sample_per_feature: Option<NodeT>,
                maximum_number_of_features: Option<NodeT>,
                remove_neighbouring_nodes: Option<bool>,
                validate_node_centralities: Option<bool>,
                maximal_depth: Option<NodeT>,
                central_node_name: Option<&str>,
                central_node_id: Option<NodeT>,
                random_state: Option<u64>,
                return_sampled_node_names: Option<bool>,
                dtype: Option<&str>,
                verbose: Option<bool>,
            ) -> PyResult<(Py<PyAny>, Option<Vec<Vec<String>>>)> {
                let gil = pyo3::Python::acquire_gil();
                let dtype = match dtype {
                    Some(dtype) => dtype,
                    None => {
                        let (max_u8, max_u16, max_u32) = (u8::MAX as usize, u16::MAX as usize, u32::MAX as usize);
                        match pe!(self.inner.get_diameter(Some(true), verbose))? as usize {
                            x if (0..=max_u8).contains(&x) => "u8",
                            x if (max_u8..=max_u16).contains(&x) => "u16",
                            x if (max_u16..=max_u32).contains(&x) => "u32",
                            _ => "u64",
                        }
                    }
                };
                let nodes_number = self.inner.get_nodes_number() as usize;
                match dtype {
                    $(
                        stringify!($dtype) => {
                            let (number_of_node_features, node_embedding_iterator, anchor_node_names) =
                            pe!(self.inner.get_shortest_paths_node_embedding::<$dtype>(
                                node_centralities,
                                node_centralities_distribution,
                                adjust_by_central_node_distance,
                                number_of_nodes_to_sample_per_feature,
                                maximum_number_of_features,
                                remove_neighbouring_nodes,
                                validate_node_centralities,
                                maximal_depth.map(|x| x  as $dtype),
                                central_node_name,
                                central_node_id,
                                random_state,
                                return_sampled_node_names,
                                verbose,
                            ))?;
                        let node_embedding: ThreadDataRaceAware<PyArray2<$dtype>> = ThreadDataRaceAware {
                            t: PyArray2::new(
                                gil.python(),
                                [nodes_number, number_of_node_features as usize],
                                false,
                            ),
                        };
                        node_embedding_iterator
                            .enumerate()
                            .for_each(|(number_of_node_feature, iterator)| {
                                iterator
                                    .enumerate()
                                    .for_each(|(node_id, node_feature)| unsafe {
                                        *node_embedding.t.uget_mut([node_id, number_of_node_feature]) =
                                            node_feature;
                                    });
                            });
                        Ok((
                            node_embedding.t.to_owned().into_py(gil.python()),
                            anchor_node_names,
                        ))
                        }
                    )*
                    dtype => pe!(Err(format!(
                        concat!(
                            "The provided dtype {} is not supported. The supported ",
                            "data types are `u8`, `u16`, `u32` and `u64`."
                        ),
                        dtype
                    ))),
                }
            }
        }
    };
}

impl_get_shortest_paths_node_embedding! {u8, u16, u32}

macro_rules! impl_get_shortest_paths_node_embedding_per_node_type {
    ($($dtype:ty),*) => {
        #[pymethods]
        impl Graph {
            #[args(py_kwargs = "**")]
            #[text_signature = "($self, node_centralities, node_centralities_distribution, adjust_by_central_node_distance, number_of_nodes_to_sample_per_feature, maximum_number_of_features_per_node_type, validate_node_centralities, maximal_depth, central_node_name, central_node_id, random_state, return_sampled_node_names, dtype, verbose)"]
            /// Return node embedding vector obtained from shortest-paths.
            ///
            /// Arguments
            /// ----------------------------------------------------------------------
            /// node_centralities: Optional[List[float]] = None
            ///     Vector with the importance of the nodes, used to properly sample the anchors. By default node degree centralities are used. Nodes with node centrality zero won't ever be sampled as an anchor, except for when all other nodes were already sampled.
            /// node_centralities_distribution: Optional[str] = None
            ///     Distribution expected out of the provided node centralities distribution. If no distribution is provided and no node centralities are provided, exponential is used as the default node centrality distribution as node degree centrality is employed as centrality.
            /// adjust_by_central_node_distance: Optional[bool] = True
            ///     Whether to adjust the node eccentricity by the normalized distance to the most central node. By default true.
            /// number_of_nodes_to_sample_per_feature: Optional[int] = 10
            ///     Number of nodes to sample per feature. By default 10.
            /// maximum_number_of_features_per_node_type: Optional[int] = 50
            ///     Maximum number of node features to generate. By default 50.
            /// remove_neighbouring_nodes: Optional[bool] = True
            ///     Whether to remove the neighbouring nodes from the set of samplable anchor nodes. By default true.
            /// validate_node_centralities: Optional[bool] = True
            ///     Whether to validate the node centralities. By default true when the node centralities are provided.
            /// maximal_depth: Optional[int] = None
            ///     The maximal depth to use if node features are to be focused in a local area of the graph.
            /// central_node_name: Optional[str] = None
            ///     The node name from where to start sampling the BFS mask for the maximal depth. Either the node name of the node ID can be provided at once.
            /// central_node_id: Optional[int] = None
            ///     The node ID from where to start sampling the BFS mask for the maximal depth. Either the node name of the node ID can be provided at once.
            /// random_state: Optional[int] = 42
            ///     The random state to use to sample the central node. By default 42.
            /// return_sampled_node_names: Optional[bool] = None
            ///     Whether to return the name of the sampled nodes. By default true if the `number_of_nodes_to_sample_per_feature` parameter is less than 100.
            /// verbose: Optional[bool] = True
            ///     Whether to show the loading bar. By default true.
            ///
            /// Details on the supported node centrality distributions
            /// ----------------------------------------------------------------------
            /// The node centrality distributions are used to find an optimal threshold that avoids
            /// sorting nodes that include also non-useful nodes, that is nodes that we will never
            /// be interested in sampling. We currently support the following node centrality distributions:
            ///
            /// Exponential
            /// ~~~~~~~~~~~~~~~~~~~~~
            /// The most common distribution for node centralities is the exponential distribution.
            /// Most likely, your node centralities will follow this distribution.
            ///
            /// Geometric
            /// ~~~~~~~~~~~~~~~~~~~~~
            /// The geometric distribution is to be used for an integer distribution, when the normalization
            /// by the distances from the most central node is disabled (or it will make the distribution a float value).
            ///
            /// Unknown
            /// ~~~~~~~~~~~~~~~~~~~~~
            /// For now we do not have support for other distributions implemented, so if the distribution
            /// is not expected to be one of the aforementioned distributions we will use zero as a threshold.
            ///
            /// Raises
            /// ----------------------------------------------------------------------
            /// * If the provided node centralities are not provided for all features.
            /// * If the provided node centralities contain illegal values, like NaNs or infinities.
            /// * If the provided node centralities are not normalized.
            /// * If the number of maximum features is zero.
            /// * If the edge weights are requested but the graph does not have edge weights.
            /// * If the use edge weights as probabilities is requested, but the graph does not have edge weights as probabilities (between 0 and 1).
            /// * If the use edge weights as probabilities is requested, but not the edge weights.
            pub fn get_shortest_paths_node_embedding_per_node_type(
                &self,
                node_centralities: Option<Vec<f32>>,
                node_centralities_distribution: Option<&str>,
                adjust_by_central_node_distance: Option<bool>,
                number_of_nodes_to_sample_per_feature: Option<NodeT>,
                maximum_number_of_features_per_node_type: Option<NodeT>,
                remove_neighbouring_nodes: Option<bool>,
                validate_node_centralities: Option<bool>,
                maximal_depth: Option<NodeT>,
                central_node_name: Option<&str>,
                central_node_id: Option<NodeT>,
                random_state: Option<u64>,
                return_sampled_node_names: Option<bool>,
                dtype: Option<&str>,
                verbose: Option<bool>,
            ) -> PyResult<(Py<PyAny>, Vec<String>, Option<Vec<Vec<String>>>)> {
                let gil = pyo3::Python::acquire_gil();
                let dtype = match dtype {
                    Some(dtype) => dtype,
                    None => {
                        let (max_u8, max_u16, max_u32) = (u8::MAX as usize, u16::MAX as usize, u32::MAX as usize);
                        match pe!(self.inner.get_diameter(Some(true), verbose))? as usize {
                            x if (0..=max_u8).contains(&x) => "u8",
                            x if (max_u8..=max_u16).contains(&x) => "u16",
                            x if (max_u16..=max_u32).contains(&x) => "u32",
                            _ => "u64",
                        }
                    }
                };
                let nodes_number = self.inner.get_nodes_number() as usize;
                match dtype {
                    $(
                        stringify!($dtype) => {
                            let (number_of_node_features, node_embedding_iterator, node_type_names, anchor_node_names) =
                            pe!(self.inner.get_shortest_paths_node_embedding_per_node_type::<$dtype>(
                                node_centralities,
                                node_centralities_distribution,
                                adjust_by_central_node_distance,
                                number_of_nodes_to_sample_per_feature,
                                maximum_number_of_features_per_node_type,
                                remove_neighbouring_nodes,
                                validate_node_centralities,
                                maximal_depth.map(|x| x  as $dtype),
                                central_node_name,
                                central_node_id,
                                random_state,
                                return_sampled_node_names,
                                verbose,
                            ))?;
                        let node_embedding: ThreadDataRaceAware<PyArray2<$dtype>> = ThreadDataRaceAware {
                            t: PyArray2::new(
                                gil.python(),
                                [nodes_number, number_of_node_features as usize],
                                false,
                            ),
                        };
                        node_embedding_iterator
                            .enumerate()
                            .for_each(|(number_of_node_feature, iterator)| {
                                iterator
                                    .enumerate()
                                    .for_each(|(node_id, node_feature)| unsafe {
                                        *node_embedding.t.uget_mut([node_id, number_of_node_feature]) =
                                            node_feature;
                                    });
                            });
                        Ok((
                            node_embedding.t.to_owned().into_py(gil.python()),
                            node_type_names,
                            anchor_node_names,
                        ))
                        }
                    )*
                    dtype => pe!(Err(format!(
                        concat!(
                            "The provided dtype {} is not supported. The supported ",
                            "data types are `u8`, `u16`, `u32` and `u64`."
                        ),
                        dtype
                    ))),
                }
            }
        }
    };
}

impl_get_shortest_paths_node_embedding_per_node_type! {u8, u16, u32}

macro_rules! impl_get_shortest_paths_node_embedding_per_edge_type {
    ($($dtype:ty),*) => {
        #[pymethods]
        impl Graph {
            #[args(py_kwargs = "**")]
            #[text_signature = "($self, node_centralities, node_centralities_distribution, adjust_by_central_node_distance, number_of_nodes_to_sample_per_feature, maximum_number_of_features_per_edge_type, remove_neighbouring_nodes, validate_node_centralities, maximal_depth, central_node_name, central_node_id, random_state, return_sampled_node_names, dtype, verbose)"]
            /// Return node embedding vector obtained from shortest-paths.
            ///
            /// Arguments
            /// ----------------------------------------------------------------------
            /// node_centralities: Optional[List[float]] = None
            ///     Vector with the importance of the nodes, used to properly sample the anchors. By default node degree centralities are used. Nodes with node centrality zero won't ever be sampled as an anchor, except for when all other nodes were already sampled.
            /// node_centralities_distribution: Optional[str] = None
            ///     Distribution expected out of the provided node centralities distribution. If no distribution is provided and no node centralities are provided, exponential is used as the default node centrality distribution as node degree centrality is employed as centrality.
            /// adjust_by_central_node_distance: Optional[bool] = True
            ///     Whether to adjust the node eccentricity by the normalized distance to the most central node. By default true.
            /// number_of_nodes_to_sample_per_feature: Optional[int] = 10
            ///     Number of nodes to sample per feature. By default 10.
            /// maximum_number_of_features_per_node_type: Optional[int] = 50
            ///     Maximum number of node features to generate. By default 50.
            /// remove_neighbouring_nodes: Optional[bool] = True
            ///     Whether to remove the neighbouring nodes from the set of samplable anchor nodes. By default true.
            /// validate_node_centralities: Optional[bool] = True
            ///     Whether to validate the node centralities. By default true when the node centralities are provided.
            /// maximal_depth: Optional[int] = None
            ///     The maximal depth to use if node features are to be focused in a local area of the graph.
            /// central_node_name: Optional[str] = None
            ///     The node name from where to start sampling the BFS mask for the maximal depth. Either the node name of the node ID can be provided at once.
            /// central_node_id: Optional[int] = None
            ///     The node ID from where to start sampling the BFS mask for the maximal depth. Either the node name of the node ID can be provided at once.
            /// random_state: Optional[int] = 42
            ///     The random state to use to sample the central node. By default 42.
            /// return_sampled_node_names: Optional[bool] = None
            ///     Whether to return the name of the sampled nodes. By default true if the `number_of_nodes_to_sample_per_feature` parameter is less than 100.
            /// verbose: Optional[bool] = True
            ///     Whether to show the loading bar. By default true.
            ///
            /// Details on the supported node centrality distributions
            /// ----------------------------------------------------------------------
            /// The node centrality distributions are used to find an optimal threshold that avoids
            /// sorting nodes that include also non-useful nodes, that is nodes that we will never
            /// be interested in sampling. We currently support the following node centrality distributions:
            ///
            /// Exponential
            /// ~~~~~~~~~~~~~~~~~~~~~
            /// The most common distribution for node centralities is the exponential distribution.
            /// Most likely, your node centralities will follow this distribution.
            ///
            /// Geometric
            /// ~~~~~~~~~~~~~~~~~~~~~
            /// The geometric distribution is to be used for an integer distribution, when the normalization
            /// by the distances from the most central node is disabled (or it will make the distribution a float value).
            ///
            /// Unknown
            /// ~~~~~~~~~~~~~~~~~~~~~
            /// For now we do not have support for other distributions implemented, so if the distribution
            /// is not expected to be one of the aforementioned distributions we will use zero as a threshold.
            ///
            /// Raises
            /// ----------------------------------------------------------------------
            /// * If the provided node centralities are not provided for all features.
            /// * If the provided node centralities contain illegal values, like NaNs or infinities.
            /// * If the provided node centralities are not normalized.
            /// * If the number of maximum features is zero.
            /// * If the edge weights are requested but the graph does not have edge weights.
            /// * If the use edge weights as probabilities is requested, but the graph does not have edge weights as probabilities (between 0 and 1).
            /// * If the use edge weights as probabilities is requested, but not the edge weights.
            pub fn get_shortest_paths_node_embedding_per_edge_type(
                &self,
                node_centralities: Option<Vec<f32>>,
                node_centralities_distribution: Option<&str>,
                adjust_by_central_node_distance: Option<bool>,
                number_of_nodes_to_sample_per_feature: Option<NodeT>,
                maximum_number_of_features_per_edge_type: Option<NodeT>,
                remove_neighbouring_nodes: Option<bool>,
                validate_node_centralities: Option<bool>,
                maximal_depth: Option<NodeT>,
                central_node_name: Option<&str>,
                central_node_id: Option<NodeT>,
                random_state: Option<u64>,
                return_sampled_node_names: Option<bool>,
                dtype: Option<&str>,
                verbose: Option<bool>,
            ) -> PyResult<(Py<PyAny>, Vec<String>, Option<Vec<Vec<String>>>)> {
                let gil = pyo3::Python::acquire_gil();
                let dtype = match dtype {
                    Some(dtype) => dtype,
                    None => {
                        let (max_u8, max_u16, max_u32) = (u8::MAX as usize, u16::MAX as usize, u32::MAX as usize);
                        match pe!(self.inner.get_diameter(Some(true), verbose))? as usize {
                            x if (0..=max_u8).contains(&x) => "u8",
                            x if (max_u8..=max_u16).contains(&x) => "u16",
                            x if (max_u16..=max_u32).contains(&x) => "u32",
                            _ => "u64",
                        }
                    }
                };
                let nodes_number = self.inner.get_nodes_number() as usize;
                match dtype {
                    $(
                        stringify!($dtype) => {
                            let (number_of_node_features, node_embedding_iterator, edge_type_names, anchor_node_names) =
                            pe!(self.inner.get_shortest_paths_node_embedding_per_edge_type::<$dtype>(
                                node_centralities,
                                node_centralities_distribution,
                                adjust_by_central_node_distance,
                                number_of_nodes_to_sample_per_feature,
                                maximum_number_of_features_per_edge_type,
                                remove_neighbouring_nodes,
                                validate_node_centralities,
                                maximal_depth.map(|x| x  as $dtype),
                                central_node_name,
                                central_node_id,
                                random_state,
                                return_sampled_node_names,
                                verbose,
                            ))?;
                        let node_embedding: ThreadDataRaceAware<PyArray2<$dtype>> = ThreadDataRaceAware {
                            t: PyArray2::new(
                                gil.python(),
                                [nodes_number, number_of_node_features as usize],
                                false,
                            ),
                        };
                        node_embedding_iterator
                            .enumerate()
                            .for_each(|(number_of_node_feature, iterator)| {
                                iterator
                                    .enumerate()
                                    .for_each(|(node_id, node_feature)| unsafe {
                                        *node_embedding.t.uget_mut([node_id, number_of_node_feature]) =
                                            node_feature;
                                    });
                            });
                        Ok((
                            node_embedding.t.to_owned().into_py(gil.python()),
                            edge_type_names,
                            anchor_node_names,
                        ))
                        }
                    )*
                    dtype => pe!(Err(format!(
                        concat!(
                            "The provided dtype {} is not supported. The supported ",
                            "data types are `u8`, `u16`, `u32` and `u64`."
                        ),
                        dtype
                    ))),
                }
            }
        }
    };
}

impl_get_shortest_paths_node_embedding_per_edge_type! {u8, u16, u32}

#[pymethods]
impl Graph {
    #[args(py_kwargs = "**")]
    #[text_signature = "($self, node_centralities, node_centralities_distribution, adjust_by_central_node_distance, number_of_nodes_to_sample_per_feature, maximum_number_of_features, validate_node_centralities, maximal_depth, central_node_name, central_node_id, use_edge_weights_as_probabilities, random_state, return_sampled_node_names, dtype, verbose)"]
    /// Return node embedding vector obtained from weighted shortest-paths.
    ///
    /// Arguments
    /// ----------------------------------------------------------------------
    /// node_centralities: Optional[List[float]] = None
    ///     Vector with the importance of the nodes, used to properly sample the anchors. By default node degree centralities are used. Nodes with node centrality zero won't ever be sampled as an anchor, except for when all other nodes were already sampled.
    /// node_centralities_distribution: Optional[str] = None
    ///     Distribution expected out of the provided node centralities distribution. If no distribution is provided and no node centralities are provided, exponential is used as the default node centrality distribution as node degree centrality is employed as centrality.
    /// adjust_by_central_node_distance: Optional[bool] = True
    ///     Whether to adjust the node eccentricity by the normalized distance to the most central node. By default true.
    /// number_of_nodes_to_sample_per_feature: Optional[int] = 10
    ///     Number of nodes to sample per feature. By default 10.
    /// maximum_number_of_features`: Optional[int]
    ///     Maximum number of node features to generate. By default 50.
    /// remove_neighbouring_nodes: Optional[bool] = True
    ///     Whether to remove the neighbouring nodes from the set of samplable anchor nodes. By default true.
    /// validate_node_centralities: Optional[bool] = True
    ///     Whether to validate the node centralities. By default true when the node centralities are provided.
    /// maximal_depth: Optional[int] = None
    ///     The maximal depth to use if node features are to be focused in a local area of the graph.
    /// central_node_name: Optional[str] = None
    ///     The node name from where to start sampling the BFS mask for the maximal depth. Either the node name of the node ID can be provided at once.
    /// central_node_id: Optional[int] = None
    ///     The node ID from where to start sampling the BFS mask for the maximal depth. Either the node name of the node ID can be provided at once.
    /// use_edge_weights_as_probabilities`: Optional[bool]
    ///     Whether to use the probabilities. By default false.
    /// random_state: Optional[int] = 42
    ///     The random state to use to sample the central node. By default 42.
    /// return_sampled_node_names: Optional[bool] = None
    ///     Whether to return the name of the sampled nodes. By default true if the `number_of_nodes_to_sample_per_feature` parameter is less than 100.
    /// verbose: Optional[bool] = True
    ///     Whether to show the loading bar. By default true.
    ///
    /// Details on the supported node centrality distributions
    /// ----------------------------------------------------------------------
    /// The node centrality distributions are used to find an optimal threshold that avoids
    /// sorting nodes that include also non-useful nodes, that is nodes that we will never
    /// be interested in sampling. We currently support the following node centrality distributions:
    ///
    /// Exponential
    /// ~~~~~~~~~~~~~~~~~~~~~
    /// The most common distribution for node centralities is the exponential distribution.
    /// Most likely, your node centralities will follow this distribution.
    ///
    /// Geometric
    /// ~~~~~~~~~~~~~~~~~~~~~
    /// The geometric distribution is to be used for an integer distribution, when the normalization
    /// by the distances from the most central node is disabled (or it will make the distribution a float value).
    ///
    /// Unknown
    /// ~~~~~~~~~~~~~~~~~~~~~
    /// For now we do not have support for other distributions implemented, so if the distribution
    /// is not expected to be one of the aforementioned distributions we will use zero as a threshold.
    ///
    /// Raises
    /// ----------------------------------------------------------------------
    /// * If the provided node centralities are not provided for all features.
    /// * If the provided node centralities contain illegal values, like NaNs or infinities.
    /// * If the provided node centralities are not normalized.
    /// * If the number of maximum features is zero.
    /// * If the edge weights are requested but the graph does not have edge weights.
    /// * If the use edge weights as probabilities is requested, but the graph does not have edge weights as probabilities (between 0 and 1).
    /// * If the use edge weights as probabilities is requested, but not the edge weights.
    pub fn get_weighted_shortest_paths_node_embedding(
        &self,
        node_centralities: Option<Vec<f32>>,
        node_centralities_distribution: Option<&str>,
        adjust_by_central_node_distance: Option<bool>,
        number_of_nodes_to_sample_per_feature: Option<NodeT>,
        maximum_number_of_features: Option<NodeT>,
        remove_neighbouring_nodes: Option<bool>,
        validate_node_centralities: Option<bool>,
        maximal_depth: Option<NodeT>,
        central_node_name: Option<&str>,
        central_node_id: Option<NodeT>,
        use_edge_weights_as_probabilities: Option<bool>,
        random_state: Option<u64>,
        return_sampled_node_names: Option<bool>,
        verbose: Option<bool>,
    ) -> PyResult<(Py<PyArray2<f32>>, Option<Vec<Vec<String>>>)> {
        let gil = pyo3::Python::acquire_gil();
        let nodes_number = self.inner.get_nodes_number() as usize;
        let (number_of_node_features, node_embedding_iterator, anchor_node_names) =
            pe!(self.inner.get_weighted_shortest_paths_node_embedding(
                node_centralities,
                node_centralities_distribution,
                adjust_by_central_node_distance,
                number_of_nodes_to_sample_per_feature,
                maximum_number_of_features,
                remove_neighbouring_nodes,
                validate_node_centralities,
                maximal_depth,
                central_node_name,
                central_node_id,
                use_edge_weights_as_probabilities,
                random_state,
                return_sampled_node_names,
                verbose,
            ))?;
        let node_embedding = ThreadDataRaceAware {
            t: PyArray2::new(
                gil.python(),
                [nodes_number, number_of_node_features as usize],
                false,
            ),
        };
        node_embedding_iterator
            .enumerate()
            .for_each(|(number_of_node_feature, iterator)| {
                iterator
                    .enumerate()
                    .for_each(|(node_id, node_feature)| unsafe {
                        *node_embedding.t.uget_mut([node_id, number_of_node_feature]) =
                            node_feature;
                    });
            });
        Ok((node_embedding.t.to_owned(), anchor_node_names))
    }

    #[args(py_kwargs = "**")]
    #[text_signature = "($self, node_centralities, node_centralities_distribution, adjust_by_central_node_distance, number_of_nodes_to_sample_per_feature, maximum_number_of_features, validate_node_centralities, maximal_depth, central_node_name, central_node_id, random_state, return_sampled_node_names, dtype, verbose)"]
    /// Return node embedding vector obtained from symmetric laplacian shortest-paths.
    ///
    /// Arguments
    /// ----------------------------------------------------------------------
    /// node_centralities: Optional[List[float]] = None
    ///     Vector with the importance of the nodes, used to properly sample the anchors. By default node degree centralities are used. Nodes with node centrality zero won't ever be sampled as an anchor, except for when all other nodes were already sampled.
    /// node_centralities_distribution: Optional[str] = None
    ///     Distribution expected out of the provided node centralities distribution. If no distribution is provided and no node centralities are provided, exponential is used as the default node centrality distribution as node degree centrality is employed as centrality.
    /// adjust_by_central_node_distance: Optional[bool] = True
    ///     Whether to adjust the node eccentricity by the normalized distance to the most central node. By default true.
    /// number_of_nodes_to_sample_per_feature: Optional[int] = 10
    ///     Number of nodes to sample per feature. By default 10.
    /// maximum_number_of_features`: Optional[int]
    ///     Maximum number of node features to generate. By default 50.
    /// remove_neighbouring_nodes: Optional[bool] = True
    ///     Whether to remove the neighbouring nodes from the set of samplable anchor nodes. By default true.
    /// validate_node_centralities: Optional[bool] = True
    ///     Whether to validate the node centralities. By default true when the node centralities are provided.
    /// maximal_depth: Optional[int] = None
    ///     The maximal depth to use if node features are to be focused in a local area of the graph.
    /// central_node_name: Optional[str] = None
    ///     The node name from where to start sampling the BFS mask for the maximal depth. Either the node name of the node ID can be provided at once.
    /// central_node_id: Optional[int] = None
    ///     The node ID from where to start sampling the BFS mask for the maximal depth. Either the node name of the node ID can be provided at once.
    /// random_state: Optional[int] = 42
    ///     The random state to use to sample the central node. By default 42.
    /// return_sampled_node_names: Optional[bool] = None
    ///     Whether to return the name of the sampled nodes. By default true if the `number_of_nodes_to_sample_per_feature` parameter is less than 100.
    /// verbose: Optional[bool] = True
    ///     Whether to show the loading bar. By default true.
    ///
    /// Details on the supported node centrality distributions
    /// ----------------------------------------------------------------------
    /// The node centrality distributions are used to find an optimal threshold that avoids
    /// sorting nodes that include also non-useful nodes, that is nodes that we will never
    /// be interested in sampling. We currently support the following node centrality distributions:
    ///
    /// Exponential
    /// ~~~~~~~~~~~~~~~~~~~~~
    /// The most common distribution for node centralities is the exponential distribution.
    /// Most likely, your node centralities will follow this distribution.
    ///
    /// Geometric
    /// ~~~~~~~~~~~~~~~~~~~~~
    /// The geometric distribution is to be used for an integer distribution, when the normalization
    /// by the distances from the most central node is disabled (or it will make the distribution a float value).
    ///
    /// Unknown
    /// ~~~~~~~~~~~~~~~~~~~~~
    /// For now we do not have support for other distributions implemented, so if the distribution
    /// is not expected to be one of the aforementioned distributions we will use zero as a threshold.
    ///
    /// Raises
    /// ----------------------------------------------------------------------
    /// * If the provided node centralities are not provided for all features.
    /// * If the provided node centralities contain illegal values, like NaNs or infinities.
    /// * If the provided node centralities are not normalized.
    /// * If the number of maximum features is zero.
    pub fn get_symmetric_laplacian_shortest_paths_node_embedding(
        &self,
        node_centralities: Option<Vec<f32>>,
        node_centralities_distribution: Option<&str>,
        adjust_by_central_node_distance: Option<bool>,
        number_of_nodes_to_sample_per_feature: Option<NodeT>,
        maximum_number_of_features: Option<NodeT>,
        remove_neighbouring_nodes: Option<bool>,
        validate_node_centralities: Option<bool>,
        maximal_depth: Option<NodeT>,
        central_node_name: Option<&str>,
        central_node_id: Option<NodeT>,
        random_state: Option<u64>,
        return_sampled_node_names: Option<bool>,
        verbose: Option<bool>,
    ) -> PyResult<(Py<PyArray2<f32>>, Option<Vec<Vec<String>>>)> {
        let gil = pyo3::Python::acquire_gil();
        let nodes_number = self.inner.get_nodes_number() as usize;
        let (number_of_node_features, node_embedding_iterator, anchor_node_names) =
            pe!(self.inner.get_symmetric_laplacian_shortest_paths_node_embedding(
                node_centralities,
                node_centralities_distribution,
                adjust_by_central_node_distance,
                number_of_nodes_to_sample_per_feature,
                maximum_number_of_features,
                remove_neighbouring_nodes,
                validate_node_centralities,
                maximal_depth,
                central_node_name,
                central_node_id,
                random_state,
                return_sampled_node_names,
                verbose,
            ))?;
        let node_embedding = ThreadDataRaceAware {
            t: PyArray2::new(
                gil.python(),
                [nodes_number, number_of_node_features as usize],
                false,
            ),
        };
        node_embedding_iterator
            .enumerate()
            .for_each(|(number_of_node_feature, iterator)| {
                iterator
                    .enumerate()
                    .for_each(|(node_id, node_feature)| unsafe {
                        *node_embedding.t.uget_mut([node_id, number_of_node_feature]) =
                            node_feature;
                    });
            });
        Ok((node_embedding.t.to_owned(), anchor_node_names))
    }

    #[args(py_kwargs = "**")]
    #[text_signature = "($self, node_centralities, node_centralities_distribution, adjust_by_central_node_distance, number_of_nodes_to_sample_per_feature, maximum_number_of_features, validate_node_centralities, maximal_depth, central_node_name, central_node_id, random_state, return_sampled_node_names, dtype, verbose)"]
    /// Return node embedding vector obtained from random-walk laplacian shortest-paths.
    ///
    /// Arguments
    /// ----------------------------------------------------------------------
    /// node_centralities: Optional[List[float]] = None
    ///     Vector with the importance of the nodes, used to properly sample the anchors. By default node degree centralities are used. Nodes with node centrality zero won't ever be sampled as an anchor, except for when all other nodes were already sampled.
    /// node_centralities_distribution: Optional[str] = None
    ///     Distribution expected out of the provided node centralities distribution. If no distribution is provided and no node centralities are provided, exponential is used as the default node centrality distribution as node degree centrality is employed as centrality.
    /// adjust_by_central_node_distance: Optional[bool] = True
    ///     Whether to adjust the node eccentricity by the normalized distance to the most central node. By default true.
    /// number_of_nodes_to_sample_per_feature: Optional[int] = 10
    ///     Number of nodes to sample per feature. By default 10.
    /// maximum_number_of_features`: Optional[int]
    ///     Maximum number of node features to generate. By default 50.
    /// remove_neighbouring_nodes: Optional[bool] = True
    ///     Whether to remove the neighbouring nodes from the set of samplable anchor nodes. By default true.
    /// validate_node_centralities: Optional[bool] = True
    ///     Whether to validate the node centralities. By default true when the node centralities are provided.
    /// maximal_depth: Optional[int] = None
    ///     The maximal depth to use if node features are to be focused in a local area of the graph.
    /// central_node_name: Optional[str] = None
    ///     The node name from where to start sampling the BFS mask for the maximal depth. Either the node name of the node ID can be provided at once.
    /// central_node_id: Optional[int] = None
    ///     The node ID from where to start sampling the BFS mask for the maximal depth. Either the node name of the node ID can be provided at once.
    /// random_state: Optional[int] = 42
    ///     The random state to use to sample the central node. By default 42.
    /// return_sampled_node_names: Optional[bool] = None
    ///     Whether to return the name of the sampled nodes. By default true if the `number_of_nodes_to_sample_per_feature` parameter is less than 100.
    /// verbose: Optional[bool] = True
    ///     Whether to show the loading bar. By default true.
    ///
    /// Details on the supported node centrality distributions
    /// ----------------------------------------------------------------------
    /// The node centrality distributions are used to find an optimal threshold that avoids
    /// sorting nodes that include also non-useful nodes, that is nodes that we will never
    /// be interested in sampling. We currently support the following node centrality distributions:
    ///
    /// Exponential
    /// ~~~~~~~~~~~~~~~~~~~~~
    /// The most common distribution for node centralities is the exponential distribution.
    /// Most likely, your node centralities will follow this distribution.
    ///
    /// Geometric
    /// ~~~~~~~~~~~~~~~~~~~~~
    /// The geometric distribution is to be used for an integer distribution, when the normalization
    /// by the distances from the most central node is disabled (or it will make the distribution a float value).
    ///
    /// Unknown
    /// ~~~~~~~~~~~~~~~~~~~~~
    /// For now we do not have support for other distributions implemented, so if the distribution
    /// is not expected to be one of the aforementioned distributions we will use zero as a threshold.
    ///
    /// Raises
    /// ----------------------------------------------------------------------
    /// * If the provided node centralities are not provided for all features.
    /// * If the provided node centralities contain illegal values, like NaNs or infinities.
    /// * If the provided node centralities are not normalized.
    /// * If the number of maximum features is zero.
    pub fn get_random_walk_laplacian_shortest_paths_node_embedding(
        &self,
        node_centralities: Option<Vec<f32>>,
        node_centralities_distribution: Option<&str>,
        adjust_by_central_node_distance: Option<bool>,
        number_of_nodes_to_sample_per_feature: Option<NodeT>,
        maximum_number_of_features: Option<NodeT>,
        remove_neighbouring_nodes: Option<bool>,
        validate_node_centralities: Option<bool>,
        maximal_depth: Option<NodeT>,
        central_node_name: Option<&str>,
        central_node_id: Option<NodeT>,
        random_state: Option<u64>,
        return_sampled_node_names: Option<bool>,
        verbose: Option<bool>,
    ) -> PyResult<(Py<PyArray2<f32>>, Option<Vec<Vec<String>>>)> {
        let gil = pyo3::Python::acquire_gil();
        let nodes_number = self.inner.get_nodes_number() as usize;
        let (number_of_node_features, node_embedding_iterator, anchor_node_names) =
            pe!(self.inner.get_random_walk_laplacian_shortest_paths_node_embedding(
                node_centralities,
                node_centralities_distribution,
                adjust_by_central_node_distance,
                number_of_nodes_to_sample_per_feature,
                maximum_number_of_features,
                remove_neighbouring_nodes,
                validate_node_centralities,
                maximal_depth,
                central_node_name,
                central_node_id,
                random_state,
                return_sampled_node_names,
                verbose,
            ))?;
        let node_embedding = ThreadDataRaceAware {
            t: PyArray2::new(
                gil.python(),
                [nodes_number, number_of_node_features as usize],
                false,
            ),
        };
        node_embedding_iterator
            .enumerate()
            .for_each(|(number_of_node_feature, iterator)| {
                iterator
                    .enumerate()
                    .for_each(|(node_id, node_feature)| unsafe {
                        *node_embedding.t.uget_mut([node_id, number_of_node_feature]) =
                            node_feature;
                    });
            });
        Ok((node_embedding.t.to_owned(), anchor_node_names))
    }

    #[args(py_kwargs = "**")]
    #[text_signature = "($self, node_centralities, node_centralities_distribution, adjust_by_central_node_distance, number_of_nodes_to_sample_per_feature, maximum_number_of_features_per_node_type, validate_node_centralities, maximal_depth, central_node_name, central_node_id, use_edge_weights_as_probabilities, random_state, return_sampled_node_names, dtype, verbose)"]
    /// Return node embedding vector obtained from weighted shortest-paths.
    ///
    /// Arguments
    /// ----------------------------------------------------------------------
    /// node_centralities: Optional[List[float]] = None
    ///     Vector with the importance of the nodes, used to properly sample the anchors. By default node degree centralities are used. Nodes with node centrality zero won't ever be sampled as an anchor, except for when all other nodes were already sampled.
    /// node_centralities_distribution: Optional[str] = None
    ///     Distribution expected out of the provided node centralities distribution. If no distribution is provided and no node centralities are provided, exponential is used as the default node centrality distribution as node degree centrality is employed as centrality.
    /// adjust_by_central_node_distance: Optional[bool] = True
    ///     Whether to adjust the node eccentricity by the normalized distance to the most central node. By default true.
    /// number_of_nodes_to_sample_per_feature: Optional[int] = 10
    ///     Number of nodes to sample per feature. By default 10.
    /// maximum_number_of_features_per_node_type: Optional[int] = 50
    ///     Maximum number of node features to generate. By default 50.
    /// remove_neighbouring_nodes: Optional[bool] = True
    ///     Whether to remove the neighbouring nodes from the set of samplable anchor nodes. By default true.
    /// validate_node_centralities: Optional[bool] = True
    ///     Whether to validate the node centralities. By default true when the node centralities are provided.
    /// maximal_depth: Optional[int] = None
    ///     The maximal depth to use if node features are to be focused in a local area of the graph.
    /// central_node_name: Optional[str] = None
    ///     The node name from where to start sampling the BFS mask for the maximal depth. Either the node name of the node ID can be provided at once.
    /// central_node_id: Optional[int] = None
    ///     The node ID from where to start sampling the BFS mask for the maximal depth. Either the node name of the node ID can be provided at once.
    /// use_edge_weights_as_probabilities`: Optional[bool]
    ///     Whether to use the probabilities. By default false.
    /// random_state: Optional[int] = 42
    ///     The random state to use to sample the central node. By default 42.
    /// return_sampled_node_names: Optional[bool] = None
    ///     Whether to return the name of the sampled nodes. By default true if the `number_of_nodes_to_sample_per_feature` parameter is less than 100.
    /// verbose: Optional[bool] = True
    ///     Whether to show the loading bar. By default true.
    ///
    /// Details on the supported node centrality distributions
    /// ----------------------------------------------------------------------
    /// The node centrality distributions are used to find an optimal threshold that avoids
    /// sorting nodes that include also non-useful nodes, that is nodes that we will never
    /// be interested in sampling. We currently support the following node centrality distributions:
    ///
    /// Exponential
    /// ~~~~~~~~~~~~~~~~~~~~~
    /// The most common distribution for node centralities is the exponential distribution.
    /// Most likely, your node centralities will follow this distribution.
    ///
    /// Geometric
    /// ~~~~~~~~~~~~~~~~~~~~~
    /// The geometric distribution is to be used for an integer distribution, when the normalization
    /// by the distances from the most central node is disabled (or it will make the distribution a float value).
    ///
    /// Unknown
    /// ~~~~~~~~~~~~~~~~~~~~~
    /// For now we do not have support for other distributions implemented, so if the distribution
    /// is not expected to be one of the aforementioned distributions we will use zero as a threshold.
    ///
    /// Raises
    /// ----------------------------------------------------------------------
    /// * If the provided node centralities are not provided for all features.
    /// * If the provided node centralities contain illegal values, like NaNs or infinities.
    /// * If the provided node centralities are not normalized.
    /// * If the number of maximum features is zero.
    /// * If the edge weights are requested but the graph does not have edge weights.
    /// * If the use edge weights as probabilities is requested, but the graph does not have edge weights as probabilities (between 0 and 1).
    /// * If the use edge weights as probabilities is requested, but not the edge weights.
    pub fn get_weighted_shortest_paths_node_embedding_per_node_type(
        &self,
        node_centralities: Option<Vec<f32>>,
        node_centralities_distribution: Option<&str>,
        adjust_by_central_node_distance: Option<bool>,
        number_of_nodes_to_sample_per_feature: Option<NodeT>,
        maximum_number_of_features_per_node_type: Option<NodeT>,
        remove_neighbouring_nodes: Option<bool>,
        validate_node_centralities: Option<bool>,
        maximal_depth: Option<NodeT>,
        central_node_name: Option<&str>,
        central_node_id: Option<NodeT>,
        use_edge_weights_as_probabilities: Option<bool>,
        random_state: Option<u64>,
        return_sampled_node_names: Option<bool>,
        verbose: Option<bool>,
    ) -> PyResult<(Py<PyArray2<f32>>, Vec<String>, Option<Vec<Vec<String>>>)> {
        let gil = pyo3::Python::acquire_gil();
        let nodes_number = self.inner.get_nodes_number() as usize;
        let (number_of_node_features, node_embedding_iterators, node_type_names, anchor_node_names) =
            pe!(self
                .inner
                .get_weighted_shortest_paths_node_embedding_per_node_type(
                    node_centralities,
                    node_centralities_distribution,
                    adjust_by_central_node_distance,
                    number_of_nodes_to_sample_per_feature,
                    maximum_number_of_features_per_node_type,
                    remove_neighbouring_nodes,
                    validate_node_centralities,
                    maximal_depth,
                    central_node_name,
                    central_node_id,
                    use_edge_weights_as_probabilities,
                    random_state,
                    return_sampled_node_names,
                    verbose,
                ))?;
        let node_embedding = ThreadDataRaceAware {
            t: PyArray2::new(
                gil.python(),
                [nodes_number, number_of_node_features as usize],
                false,
            ),
        };
        let mut offset = 0;
        node_embedding_iterators
            .into_iter()
            .zip(anchor_node_names.iter().map(|anchors| anchors.len()))
            .for_each(|(node_embedding_iterator, current_features_number)| {
                node_embedding_iterator.enumerate().for_each(
                    |(number_of_node_feature, iterator)| {
                        iterator
                            .enumerate()
                            .for_each(|(node_id, node_feature)| unsafe {
                                *node_embedding
                                    .t
                                    .uget_mut([node_id, offset + number_of_node_feature]) =
                                    node_feature;
                            });
                    },
                );
                offset += current_features_number;
            });

        Ok((
            node_embedding.t.to_owned(),
            node_type_names,
            anchor_node_names,
        ))
    }

    #[args(py_kwargs = "**")]
    #[text_signature = "($self, node_centralities, node_centralities_distribution, adjust_by_central_node_distance, number_of_nodes_to_sample_per_feature, maximum_number_of_features_per_edge_type, remove_neighbouring_nodes, validate_node_centralities, maximal_depth, central_node_name, central_node_id, use_edge_weights_as_probabilities, random_state, return_sampled_node_names, dtype, verbose)"]
    /// Return node embedding vector obtained from weighted shortest-paths.
    ///
    /// Arguments
    /// ----------------------------------------------------------------------
    /// node_centralities: Optional[List[float]] = None
    ///     Vector with the importance of the nodes, used to properly sample the anchors. By default node degree centralities are used. Nodes with node centrality zero won't ever be sampled as an anchor, except for when all other nodes were already sampled.
    /// node_centralities_distribution: Optional[str] = None
    ///     Distribution expected out of the provided node centralities distribution. If no distribution is provided and no node centralities are provided, exponential is used as the default node centrality distribution as node degree centrality is employed as centrality.
    /// adjust_by_central_node_distance: Optional[bool] = True
    ///     Whether to adjust the node eccentricity by the normalized distance to the most central node. By default true.
    /// number_of_nodes_to_sample_per_feature: Optional[int] = 10
    ///     Number of nodes to sample per feature. By default 10.
    /// maximum_number_of_features_per_edge_type`: Optional[int]
    ///     Maximum number of node features to generate. By default 50.
    /// remove_neighbouring_nodes: Optional[bool] = True
    ///     Whether to remove the neighbouring nodes from the set of samplable anchor nodes. By default true.
    /// validate_node_centralities: Optional[bool] = True
    ///     Whether to validate the node centralities. By default true when the node centralities are provided.
    /// maximal_depth: Optional[int] = None
    ///     The maximal depth to use if node features are to be focused in a local area of the graph.
    /// central_node_name: Optional[str] = None
    ///     The node name from where to start sampling the BFS mask for the maximal depth. Either the node name of the node ID can be provided at once.
    /// central_node_id: Optional[int] = None
    ///     The node ID from where to start sampling the BFS mask for the maximal depth. Either the node name of the node ID can be provided at once.
    /// use_edge_weights_as_probabilities`: Optional[bool]
    ///     Whether to use the probabilities. By default false.
    /// random_state: Optional[int] = 42
    ///     The random state to use to sample the central node. By default 42.
    /// return_sampled_node_names: Optional[bool] = None
    ///     Whether to return the name of the sampled nodes. By default true if the `number_of_nodes_to_sample_per_feature` parameter is less than 100.
    /// verbose: Optional[bool] = True
    ///     Whether to show the loading bar. By default true.
    ///
    /// Details on the supported node centrality distributions
    /// ----------------------------------------------------------------------
    /// The node centrality distributions are used to find an optimal threshold that avoids
    /// sorting nodes that include also non-useful nodes, that is nodes that we will never
    /// be interested in sampling. We currently support the following node centrality distributions:
    ///
    /// Exponential
    /// ~~~~~~~~~~~~~~~~~~~~~
    /// The most common distribution for node centralities is the exponential distribution.
    /// Most likely, your node centralities will follow this distribution.
    ///
    /// Geometric
    /// ~~~~~~~~~~~~~~~~~~~~~
    /// The geometric distribution is to be used for an integer distribution, when the normalization
    /// by the distances from the most central node is disabled (or it will make the distribution a float value).
    ///
    /// Unknown
    /// ~~~~~~~~~~~~~~~~~~~~~
    /// For now we do not have support for other distributions implemented, so if the distribution
    /// is not expected to be one of the aforementioned distributions we will use zero as a threshold.
    ///
    /// Raises
    /// ----------------------------------------------------------------------
    /// * If the provided node centralities are not provided for all features.
    /// * If the provided node centralities contain illegal values, like NaNs or infinities.
    /// * If the provided node centralities are not normalized.
    /// * If the number of maximum features is zero.
    /// * If the edge weights are requested but the graph does not have edge weights.
    /// * If the use edge weights as probabilities is requested, but the graph does not have edge weights as probabilities (between 0 and 1).
    /// * If the use edge weights as probabilities is requested, but not the edge weights.
    pub fn get_weighted_shortest_paths_node_embedding_per_edge_type(
        &self,
        node_centralities: Option<Vec<f32>>,
        node_centralities_distribution: Option<&str>,
        adjust_by_central_node_distance: Option<bool>,
        number_of_nodes_to_sample_per_feature: Option<NodeT>,
        maximum_number_of_features_per_edge_type: Option<NodeT>,
        remove_neighbouring_nodes: Option<bool>,
        validate_node_centralities: Option<bool>,
        maximal_depth: Option<NodeT>,
        central_node_name: Option<&str>,
        central_node_id: Option<NodeT>,
        use_edge_weights_as_probabilities: Option<bool>,
        random_state: Option<u64>,
        return_sampled_node_names: Option<bool>,
        verbose: Option<bool>,
    ) -> PyResult<(Py<PyArray2<f32>>, Vec<String>, Option<Vec<Vec<String>>>)> {
        let gil = pyo3::Python::acquire_gil();
        let nodes_number = self.inner.get_nodes_number() as usize;
        let (number_of_node_features, node_embedding_iterators, edge_type_names, anchor_node_names) =
            pe!(self
                .inner
                .get_weighted_shortest_paths_node_embedding_per_edge_type(
                    node_centralities,
                    node_centralities_distribution,
                    adjust_by_central_node_distance,
                    number_of_nodes_to_sample_per_feature,
                    maximum_number_of_features_per_edge_type,
                    remove_neighbouring_nodes,
                    validate_node_centralities,
                    maximal_depth,
                    central_node_name,
                    central_node_id,
                    use_edge_weights_as_probabilities,
                    random_state,
                    return_sampled_node_names,
                    verbose,
                ))?;
        let node_embedding = ThreadDataRaceAware {
            t: PyArray2::new(
                gil.python(),
                [nodes_number, number_of_node_features as usize],
                false,
            ),
        };
        let mut offset = 0;
        node_embedding_iterators
            .into_iter()
            .zip(anchor_node_names.iter().map(|anchors| anchors.len()))
            .for_each(|(node_embedding_iterator, current_features_number)| {
                node_embedding_iterator.enumerate().for_each(
                    |(number_of_node_feature, iterator)| {
                        iterator
                            .enumerate()
                            .for_each(|(node_id, node_feature)| unsafe {
                                *node_embedding
                                    .t
                                    .uget_mut([node_id, offset + number_of_node_feature]) =
                                    node_feature;
                            });
                    },
                );
                offset += current_features_number;
            });

        Ok((
            node_embedding.t.to_owned(),
            edge_type_names,
            anchor_node_names,
        ))
    }
}
