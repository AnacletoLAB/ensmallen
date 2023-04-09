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

    /// Returns iterator over edges of a laplacian-like matrix using the weights of the provided functions.
    ///
    /// # Arguments
    /// * `get_edge_weight`: fn(&Graph, NodeT, NodeT) -> WeightT - The closure providing the value for the normal edge weight.
    /// * `get_selfloop_edge_weight`: fn(&Graph, NodeT) -> WeightT - The closure providing the value for the normal selfloop weight.
    fn par_iter_laplacian_like_coo_matrix(
        &self,
        get_edge_weight: fn(&Graph, NodeT, NodeT) -> WeightT,
        get_selfloop_edge_weight: fn(&Graph, NodeT) -> WeightT,
    ) -> impl ParallelIterator<Item = (NodeT, NodeT, WeightT)> + '_ {
        self.par_iter_directed_edge_node_ids()
            .map(move |(_, src, dst)| {
                (
                    src,
                    dst,
                    if src == dst {
                        get_selfloop_edge_weight(&self, src)
                    } else {
                        get_edge_weight(&self, src, dst)
                    },
                )
            })
            .chain(
                self.par_iter_node_ids()
                    .filter(move |&node_id| !self.has_selfloop_from_node_id(node_id))
                    .filter_map(move |node_id| {
                        let weight = get_selfloop_edge_weight(&self, node_id);
                        if weight.is_zero() {
                            return None;
                        }
                        Some((node_id, node_id, weight))
                    }),
            )
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
            Some(false),
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

    /// Returns parallel iterator on neighbours intersection size COO matrix.
    pub fn par_iter_neighbours_intersection_size_coo_matrix(
        &self,
    ) -> impl ParallelIterator<Item = (NodeT, NodeT, WeightT)> + '_ {
        self.par_iter_transformed_coo_matrix(self, |support, src, dst| unsafe {
            support.get_unchecked_neighbours_intersection_size_from_node_ids(src, dst)
        })
    }

    /// Returns neighbours intersection size coo matrix.
    pub fn get_neighbours_intersection_size_coo_matrix(
        &self,
    ) -> (Vec<(NodeT, NodeT)>, Vec<WeightT>) {
        self.par_iter_neighbours_intersection_size_coo_matrix()
            .map(|(src, dst, weight)| ((src, dst), weight))
            .unzip()
    }

    /// Returns neighbours intersection size weighted graph.
    pub fn get_neighbours_intersection_size_graph(&self) -> Graph {
        self.get_graph_from_coo_iterator(self.par_iter_neighbours_intersection_size_coo_matrix())
    }

    /// Returns parallel iterator on shared ancestors size COO matrix.
    ///
    /// # Arguments
    /// * `bfs`: &ShortestPathsResultBFS - The BFS object to use for the ancestors.
    pub fn par_iter_shared_ancestors_size_coo_matrix<'a>(
        &'a self,
        bfs: &'a ShortestPathsResultBFS,
    ) -> impl ParallelIterator<Item = (NodeT, NodeT, WeightT)> + 'a {
        self.par_iter_transformed_coo_matrix(bfs, |support, src, dst| {
            support.get_shared_ancestors_size(src, dst).unwrap()
        })
    }

    /// Returns shared ancestors size coo matrix.
    ///
    /// # Arguments
    /// * `bfs`: &ShortestPathsResultBFS - The BFS object to use for the ancestors.
    pub fn get_shared_ancestors_size_coo_matrix(
        &self,
        bfs: &ShortestPathsResultBFS,
    ) -> (Vec<(NodeT, NodeT)>, Vec<WeightT>) {
        self.par_iter_shared_ancestors_size_coo_matrix(bfs)
            .map(|(src, dst, weight)| ((src, dst), weight))
            .unzip()
    }

    /// Returns shared ancestors size weighted graph.
    ///
    /// # Arguments
    /// * `bfs`: &ShortestPathsResultBFS - The BFS object to use for the ancestors.
    pub fn get_shared_ancestors_size_graph(&self, bfs: &ShortestPathsResultBFS) -> Graph {
        self.get_graph_from_coo_iterator(self.par_iter_shared_ancestors_size_coo_matrix(bfs))
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
    /// * `node_ids_of_interest`: Option<&[NodeT]> - While the random walks is graph-wide, we only return edges whose source and destination nodes are within this node ID list.
    pub fn par_iter_cooccurence_matrix<'a>(
        &'a self,
        walks_parameters: &'a WalksParameters,
        window_size: usize,
        node_ids_of_interest: Option<&'a [NodeT]>,
    ) -> Result<impl ParallelIterator<Item = (NodeT, NodeT, NodeT)> + 'a> {
        Ok(self
            .par_iter_complete_walks(walks_parameters)?
            .flat_map(move |sequence| {
                let mut cooccurence_matrix: HashMap<NodeT, HashMap<NodeT, NodeT>> = HashMap::new();
                (0..sequence.len())
                    .map(|position| {
                        (
                            sequence[position],
                            &sequence[position.saturating_sub(window_size)
                                ..(position + window_size).min(sequence.len())],
                        )
                    })
                    .for_each(|(central_id, context)| {
                        let local_cooccurence_matrix =
                            cooccurence_matrix.entry(central_id).or_default();
                        context
                            .iter()
                            .copied()
                            .filter(|&context_id| context_id != central_id)
                            .for_each(|context_id| {
                                // Get the current value for this pair of nodes
                                local_cooccurence_matrix
                                    .entry(context_id)
                                    .and_modify(|e| *e += 1)
                                    .or_insert(1);
                            });
                    });
                cooccurence_matrix
                    .into_par_iter()
                    .flat_map(move |(src, local_cooccurence)| {
                        local_cooccurence
                            .into_par_iter()
                            .filter_map(move |(dst, count)| {
                                if node_ids_of_interest.as_ref().map_or(
                                    true,
                                    |node_ids_of_interest| {
                                        node_ids_of_interest.contains(&src)
                                            && node_ids_of_interest.contains(&dst)
                                    },
                                ) {
                                    Some((src, dst, count))
                                } else {
                                    None
                                }
                            })
                    })
            }))
    }

    /// Returns unweighted laplacian COO matrix representation of the graph.
    pub fn par_iter_laplacian_coo_matrix(
        &self,
    ) -> impl ParallelIterator<Item = (NodeT, NodeT, WeightT)> + '_ {
        self.par_iter_laplacian_like_coo_matrix(
            |_, _, _| -1.0,
            |graph, node_id| unsafe {
                graph.get_unchecked_node_degree_from_node_id(node_id) as WeightT
            },
        )
    }

    /// Returns Laplacian coo matrix.
    pub fn get_laplacian_coo_matrix(&self) -> (Vec<(NodeT, NodeT)>, Vec<WeightT>) {
        self.par_iter_laplacian_coo_matrix()
            .map(|(src, dst, weight)| ((src, dst), weight))
            .unzip()
    }

    /// Returns Laplacian weighted graph.
    pub fn get_laplacian_graph(&self) -> Graph {
        self.get_graph_from_coo_iterator(self.par_iter_laplacian_coo_matrix())
    }

    /// Returns unweighted left normalized laplacian COO matrix representation of the graph.
    pub fn par_iter_left_normalized_laplacian_coo_matrix(
        &self,
    ) -> impl ParallelIterator<Item = (NodeT, NodeT, WeightT)> + '_ {
        self.par_iter_laplacian_like_coo_matrix(
            |graph, src, _| {
                -1.0 / unsafe { graph.get_unchecked_node_degree_from_node_id(src) as WeightT }
            },
            |_, _| 1.0,
        )
    }

    /// Returns left normalized Laplacian coo matrix.
    pub fn get_left_normalized_laplacian_coo_matrix(&self) -> (Vec<(NodeT, NodeT)>, Vec<WeightT>) {
        self.par_iter_left_normalized_laplacian_coo_matrix()
            .map(|(src, dst, weight)| ((src, dst), weight))
            .unzip()
    }

    /// Returns left normalized Laplacian weighted graph.
    pub fn get_left_normalized_laplacian_graph(&self) -> Graph {
        self.get_graph_from_coo_iterator(self.par_iter_left_normalized_laplacian_coo_matrix())
    }

    /// Returns unweighted right normalized laplacian COO matrix representation of the graph.
    pub fn par_iter_right_normalized_laplacian_coo_matrix(
        &self,
    ) -> impl ParallelIterator<Item = (NodeT, NodeT, WeightT)> + '_ {
        self.par_iter_laplacian_like_coo_matrix(
            |graph, _, dst| {
                -1.0 / (unsafe {
                    graph.get_unchecked_node_degree_from_node_id(dst) as WeightT + 1.0
                })
            },
            |_, _| 1.0,
        )
    }

    /// Returns right normalized Laplacian coo matrix.
    pub fn get_right_normalized_laplacian_coo_matrix(&self) -> (Vec<(NodeT, NodeT)>, Vec<WeightT>) {
        self.par_iter_right_normalized_laplacian_coo_matrix()
            .map(|(src, dst, weight)| ((src, dst), weight))
            .unzip()
    }

    /// Returns right normalized Laplacian weighted graph.
    pub fn get_right_normalized_laplacian_graph(&self) -> Graph {
        self.get_graph_from_coo_iterator(self.par_iter_right_normalized_laplacian_coo_matrix())
    }

    /// Returns unweighted symmetric normalized laplacian COO matrix representation of the graph.
    pub fn par_iter_symmetric_normalized_laplacian_coo_matrix(
        &self,
    ) -> impl ParallelIterator<Item = (NodeT, NodeT, WeightT)> + '_ {
        self.par_iter_laplacian_like_coo_matrix(
            |graph, src, dst| unsafe {
                -1.0 / {
                    (graph.get_unchecked_node_degree_from_node_id(src) as f64
                        * graph.get_unchecked_node_degree_from_node_id(dst) as f64)
                        .sqrt() as WeightT
                }
            },
            |_, _| 1.0,
        )
    }

    /// Returns symmetric normalized Laplacian coo matrix.
    pub fn get_symmetric_normalized_laplacian_coo_matrix(
        &self,
    ) -> (Vec<(NodeT, NodeT)>, Vec<WeightT>) {
        self.par_iter_symmetric_normalized_laplacian_coo_matrix()
            .map(|(src, dst, weight)| ((src, dst), weight))
            .unzip()
    }

    /// Returns symmetric normalized Laplacian weighted graph.
    pub fn get_symmetric_normalized_laplacian_graph(&self) -> Graph {
        self.get_graph_from_coo_iterator(self.par_iter_symmetric_normalized_laplacian_coo_matrix())
    }
}
