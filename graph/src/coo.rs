use super::*;
use hashbrown::HashMap;
use num_traits::Zero;
use rayon::prelude::*;

impl Graph {
    /// Returns parallel iterator on coo matrix following the two provided metrics.
    ///
    /// # Arguments
    /// * `get_edge_weight`: fn(&Graph, NodeT, NodeT) -> WeightT - The closure providing the value for the edge weight.
    fn par_iter_transformed_coo_matrix<'a, T: Send + Sync>(
        &'a self,
        support: &'a T,
        get_edge_weight: fn(&T, NodeT, NodeT) -> WeightT,
    ) -> impl ParallelIterator<Item = (NodeT, NodeT, WeightT)> + 'a {
        self.par_iter_node_ids().flat_map(move |src| {
            self.par_iter_node_ids().filter_map(move |dst| {
                let edge_weight = get_edge_weight(support, src, dst);
                if edge_weight.is_zero() {
                    None
                } else {
                    Some((src, dst, edge_weight))
                }
            })
        })
    }

    /// Returns weighted graph from provided coo edges iterator.
    ///
    /// # Arguments
    /// * `edges_iterator`: Iterator over the edges.
    fn get_graph_from_coo_iterator<I>(&self, edges_iterator: I) -> Graph
    where
        I: ParallelIterator<Item = (NodeT, NodeT, WeightT)>,
    {
        build_graph_from_integers(
            Some(edges_iterator.map(|(src, dst, weight)| (0, (src, dst, None, weight)))),
            self.nodes.clone(),
            self.node_types.clone(),
            None,
            true,
            self.is_directed(),
            Some(true),
            Some(false),
            Some(true),
            None,
            true,
            true,
            self.get_name(),
        )
        .unwrap()
    }

    /// Returns parallel iterator on Jaccard COO matrix.
    pub fn par_iter_jaccard_coo_matrix(
        &self,
    ) -> impl ParallelIterator<Item = (NodeT, NodeT, WeightT)> + '_ {
        self.par_iter_transformed_coo_matrix(self, |support, src, dst| unsafe {
            support.get_unchecked_jaccard_coefficient_from_node_ids(src, dst)
        })
    }

    /// Returns Jaccard coo matrix.
    pub fn get_jaccard_coo_matrix(&self) -> (Vec<(NodeT, NodeT)>, Vec<WeightT>) {
        self.par_iter_jaccard_coo_matrix()
            .map(|(src, dst, weight)| ((src, dst), weight))
            .unzip()
    }

    /// Returns jaccard weighted graph.
    pub fn get_jaccard_graph(&self) -> Graph {
        self.get_graph_from_coo_iterator(self.par_iter_jaccard_coo_matrix())
    }

    /// Returns parallel iterator on ancestors Jaccard COO matrix.
    ///
    /// # Arguments
    /// * `bfs`: &ShortestPathsResultBFS - The BFS object to use for the ancestors.
    pub fn par_iter_ancestors_jaccard_coo_matrix<'a>(
        &'a self,
        bfs: &'a ShortestPathsResultBFS,
    ) -> impl ParallelIterator<Item = (NodeT, NodeT, WeightT)> + 'a {
        self.par_iter_transformed_coo_matrix(bfs, |support, src, dst| {
            support.get_ancestors_jaccard_index(src, dst).unwrap()
        })
    }

    /// Returns Ancestors Jaccard coo matrix.
    ///
    /// # Arguments
    /// * `bfs`: &ShortestPathsResultBFS - The BFS object to use for the ancestors.
    pub fn get_ancestors_jaccard_coo_matrix(
        &self,
        bfs: &ShortestPathsResultBFS,
    ) -> (Vec<(NodeT, NodeT)>, Vec<WeightT>) {
        self.par_iter_ancestors_jaccard_coo_matrix(bfs)
            .map(|(src, dst, weight)| ((src, dst), weight))
            .unzip()
    }

    /// Returns Ancestors Jaccard weighted graph.
    ///
    /// # Arguments
    /// * `bfs`: &ShortestPathsResultBFS - The BFS object to use for the ancestors.
    pub fn get_ancestors_jaccard_graph(&self, bfs: &ShortestPathsResultBFS) -> Graph {
        self.get_graph_from_coo_iterator(self.par_iter_ancestors_jaccard_coo_matrix(bfs))
    }

    /// Returns parallel iterator on Adamic-Adar coo matrix following the two provided metrics.
    ///
    /// # Arguments
    /// * `get_edge_weight`: fn(&Graph, NodeT, NodeT) -> WeightT - The closure providing the value for the edge weight.
    pub fn par_iter_adamic_adar_coo_matrix(
        &self,
    ) -> impl ParallelIterator<Item = (NodeT, NodeT, WeightT)> + '_ {
        self.par_iter_transformed_coo_matrix(self, |support, src, dst| unsafe {
            support.get_unchecked_adamic_adar_index_from_node_ids(src, dst)
        })
    }

    /// Returns Adamic-adar coo matrix.
    pub fn get_adamic_adar_coo_matrix(&self) -> (Vec<(NodeT, NodeT)>, Vec<WeightT>) {
        self.par_iter_adamic_adar_coo_matrix()
            .map(|(src, dst, weight)| ((src, dst), weight))
            .unzip()
    }

    /// Returns Adamic-Adar weighted graph.
    pub fn get_adamic_adar_graph(&self) -> Graph {
        self.get_graph_from_coo_iterator(self.par_iter_adamic_adar_coo_matrix())
    }

    /// Returns parallel iterator over the co-occurrence matrix
    ///
    /// # Arguments
    /// * `walks_parameters`: &'a WalksParameters - the walks parameters.
    /// * `window_size`: usize - Window size to consider for the sequences.
    /// * `node_ids_of_interest`: Option<&Vec<NodeT>> - While the random walks is graph-wide, we only return edges whose source and destination nodes are within this node ID list.
    pub fn par_iter_cooccurence_matrix<'a>(
        &'a self,
        walks_parameters: &'a WalksParameters,
        window_size: usize,
        node_ids_of_interest: Option<&'a Vec<NodeT>>,
    ) -> Result<impl ParallelIterator<Item = (NodeT, NodeT, f32)> + 'a> {
        Ok(self
            .par_iter_complete_walks(walks_parameters)?
            .flat_map(move |sequence| {
                let mut cooccurence_matrix: HashMap<(NodeT, NodeT), f32> = HashMap::new();
                let mut total = 0.0;
                (0..sequence.len())
                    .map(|position| {
                        (
                            sequence[position],
                            &sequence[(position.saturating_sub(window_size)
                                ..(position + window_size).min(sequence.len()))],
                        )
                    })
                    .for_each(|(central_id, context)| {
                        context.iter().copied().for_each(|context_id| {
                            // Get the current value for this pair of nodes
                            cooccurence_matrix
                                .entry((central_id, context_id))
                                .and_modify(|e| *e += 1.0)
                                .or_insert(1.0);
                            total += 1.0;
                        });
                    });
                cooccurence_matrix.into_par_iter().filter_map(
                    move |((src, dst), freq)| {
                        if node_ids_of_interest
                            .as_ref()
                            .map_or(true, |node_ids_of_interest| {
                                node_ids_of_interest.contains(&src)
                                    && node_ids_of_interest.contains(&dst)
                            })
                        {
                            Some((src, dst, freq / total))
                        } else {
                            None
                        }
                    },
                )
            }))
    }

    /// Returns Cooccurrence coo matrix.
    ///
    /// # Arguments
    /// * `walks_parameters`: &WalksParameters - the walks parameters.
    /// * `window_size`: usize - Window size to consider for the sequences.
    /// * `node_ids_of_interest`: Option<&Vec<NodeT>> - While the random walks is graph-wide, we only return edges whose source and destination nodes are within this node ID list.
    pub fn get_cooccurrence_coo_matrix(
        &self,
        walks_parameters: &WalksParameters,
        window_size: usize,
        node_ids_of_interest: Option<&Vec<NodeT>>,
    ) -> Result<(Vec<(NodeT, NodeT)>, Vec<WeightT>)> {
        Ok(self
            .par_iter_cooccurence_matrix(walks_parameters, window_size, node_ids_of_interest)?
            .map(|(src, dst, weight)| ((src, dst), weight))
            .unzip())
    }

    /// Returns Cooccurrence weighted graph.
    ///
    /// # Arguments
    /// * `walks_parameters`: &WalksParameters - the walks parameters.
    /// * `window_size`: usize - Window size to consider for the sequences.
    /// * `node_ids_of_interest`: Option<&Vec<NodeT>> - While the random walks is graph-wide, we only return edges whose source and destination nodes are within this node ID list.
    pub fn get_cooccurrence_graph(
        &self,
        walks_parameters: &WalksParameters,
        window_size: usize,
        node_ids_of_interest: Option<&Vec<NodeT>>,
    ) -> Result<Graph> {
        Ok(
            self.get_graph_from_coo_iterator(self.par_iter_cooccurence_matrix(
                walks_parameters,
                window_size,
                node_ids_of_interest,
            )?),
        )
    }

    /// Returns parallel iterator over the normalized co-occurrence matrix
    ///
    /// # Arguments
    /// * `walks_parameters`: &'a WalksParameters - the walks parameters.
    /// * `window_size`: usize - Window size to consider for the sequences.
    /// * `node_ids_of_interest`: Option<&Vec<NodeT>> - While the random walks is graph-wide, we only return edges whose source and destination nodes are within this node ID list.
    pub fn par_iter_normalized_cooccurence_matrix<'a>(
        &'a self,
        walks_parameters: &'a WalksParameters,
        window_size: usize,
        node_ids_of_interest: Option<&'a Vec<NodeT>>,
    ) -> Result<impl ParallelIterator<Item = (NodeT, NodeT, f32)> + 'a> {
        let nodes_number_squared = (self.get_nodes_number() * self.get_nodes_number()) as f32;
        Ok(self
            .par_iter_cooccurence_matrix(walks_parameters, window_size, node_ids_of_interest)?
            .map(move |(src, dst, frequency)| {
                (
                    src,
                    dst,
                    frequency * nodes_number_squared
                        / unsafe {
                            (self.get_unchecked_node_degree_from_node_id(src)
                                * self.get_unchecked_node_degree_from_node_id(dst))
                                as f32
                        },
                )
            }))
    }

    /// Returns Normalized Cooccurrence coo matrix.
    ///
    /// # Arguments
    /// * `walks_parameters`: &WalksParameters - the walks parameters.
    /// * `window_size`: usize - Window size to consider for the sequences.
    /// * `node_ids_of_interest`: Option<&Vec<NodeT>> - While the random walks is graph-wide, we only return edges whose source and destination nodes are within this node ID list.
    pub fn get_normalized_cooccurrence_coo_matrix(
        &self,
        walks_parameters: &WalksParameters,
        window_size: usize,
        node_ids_of_interest: Option<&Vec<NodeT>>,
    ) -> Result<(Vec<(NodeT, NodeT)>, Vec<WeightT>)> {
        Ok(self
            .par_iter_normalized_cooccurence_matrix(
                walks_parameters,
                window_size,
                node_ids_of_interest,
            )?
            .map(|(src, dst, weight)| ((src, dst), weight))
            .unzip())
    }

    /// Returns Normalized Cooccurrence weighted graph.
    ///
    /// # Arguments
    /// * `walks_parameters`: &WalksParameters - the walks parameters.
    /// * `window_size`: usize - Window size to consider for the sequences.
    /// * `node_ids_of_interest`: Option<&Vec<NodeT>> - While the random walks is graph-wide, we only return edges whose source and destination nodes are within this node ID list.
    pub fn get_normalized_cooccurrence_graph(
        &self,
        walks_parameters: &WalksParameters,
        window_size: usize,
        node_ids_of_interest: Option<&Vec<NodeT>>,
    ) -> Result<Graph> {
        Ok(
            self.get_graph_from_coo_iterator(self.par_iter_normalized_cooccurence_matrix(
                walks_parameters,
                window_size,
                node_ids_of_interest,
            )?),
        )
    }

    /// Returns parallel iterator over the log normalized co-occurrence matrix
    ///
    /// # Arguments
    /// * `walks_parameters`: &'a WalksParameters - the walks parameters.
    /// * `window_size`: usize - Window size to consider for the sequences.
    /// * `node_ids_of_interest`: Option<&Vec<NodeT>> - While the random walks is graph-wide, we only return edges whose source and destination nodes are within this node ID list.
    pub fn par_iter_log_normalized_cooccurence_matrix<'a>(
        &'a self,
        walks_parameters: &'a WalksParameters,
        window_size: usize,
        node_ids_of_interest: Option<&'a Vec<NodeT>>,
    ) -> Result<impl ParallelIterator<Item = (NodeT, NodeT, f32)> + 'a> {
        Ok(self
            .par_iter_normalized_cooccurence_matrix(
                walks_parameters,
                window_size,
                node_ids_of_interest,
            )?
            .filter_map(move |(src, dst, frequency)| {
                if frequency <= 1.0 {
                    None
                } else {
                    Some((src, dst, frequency.ln()))
                }
            }))
    }

    /// Returns Log Normalized Cooccurrence coo matrix.
    ///
    /// # Arguments
    /// * `walks_parameters`: &WalksParameters - the walks parameters.
    /// * `window_size`: usize - Window size to consider for the sequences.
    /// * `node_ids_of_interest`: Option<&Vec<NodeT>> - While the random walks is graph-wide, we only return edges whose source and destination nodes are within this node ID list.
    pub fn get_log_normalized_cooccurrence_coo_matrix(
        &self,
        walks_parameters: &WalksParameters,
        window_size: usize,
        node_ids_of_interest: Option<&Vec<NodeT>>,
    ) -> Result<(Vec<(NodeT, NodeT)>, Vec<WeightT>)> {
        Ok(self
            .par_iter_log_normalized_cooccurence_matrix(
                walks_parameters,
                window_size,
                node_ids_of_interest,
            )?
            .map(|(src, dst, weight)| ((src, dst), weight))
            .unzip())
    }

    /// Returns Log Normalized Cooccurrence weighted graph.
    ///
    /// # Arguments
    /// * `walks_parameters`: &WalksParameters - the walks parameters.
    /// * `window_size`: usize - Window size to consider for the sequences.
    /// * `node_ids_of_interest`: Option<&Vec<NodeT>> - While the random walks is graph-wide, we only return edges whose source and destination nodes are within this node ID list.
    pub fn get_log_normalized_cooccurrence_graph(
        &self,
        walks_parameters: &WalksParameters,
        window_size: usize,
        node_ids_of_interest: Option<&Vec<NodeT>>,
    ) -> Result<Graph> {
        Ok(
            self.get_graph_from_coo_iterator(self.par_iter_log_normalized_cooccurence_matrix(
                walks_parameters,
                window_size,
                node_ids_of_interest,
            )?),
        )
    }
}
