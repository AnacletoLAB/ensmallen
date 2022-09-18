use super::*;
use crate::constructors::build_graph_from_integers;
use rayon::prelude::*;

/// # Generators of graphs from user-provided edge list definitions.
impl Graph {
    /// Returns bipartite graph between the provided source and destination node IDs.
    ///
    /// # Arguments
    /// * `source_node_ids`: Vec<NodeT> - The source node IDs.
    /// * `destination_node_ids`: Vec<NodeT> - The destination node IDs.
    /// * `directed`: bool - Whether to make the graph directed or undirected.
    ///
    pub fn build_bipartite_graph_from_edge_node_ids(
        &self,
        mut source_node_ids: Vec<NodeT>,
        mut destination_node_ids: Vec<NodeT>,
        directed: bool,
    ) -> Result<Graph> {
        // Validate the provided tuples.
        source_node_ids
            .par_iter()
            .chain(destination_node_ids.par_iter())
            .copied()
            .map(|node_id| self.validate_node_id(node_id).map(|_| ()))
            .collect::<Result<()>>()?;

        // We sort the provided node IDs.
        source_node_ids.par_sort_unstable();
        destination_node_ids.par_sort_unstable();

        // Search duplicates in the provided vectors
        if source_node_ids
            .par_windows(2)
            .any(|window| window[0] == window[1])
        {
            return Err("There are duplicated nodes in the provided sources.".to_string());
        }
        if destination_node_ids
            .par_windows(2)
            .any(|window| window[0] == window[1])
        {
            return Err("There are duplicated nodes in the provided destinations.".to_string());
        }

        let number_of_source_nodes = source_node_ids.len();
        let number_of_destination_nodes = destination_node_ids.len();

        // Check no node ID in the sources
        // is present in the destination nodes
        let mut src_index = 0;
        let mut dst_index = 0;
        while src_index < source_node_ids.len() && dst_index < destination_node_ids.len() {
            let current_src = source_node_ids[src_index];
            let current_dst = destination_node_ids[dst_index];
            match current_src.cmp(&current_dst) {
                std::cmp::Ordering::Equal => {
                    return Err(format!(
                        concat!(
                            "The are duplicate nodes in the provided source ",
                            "and destination nodes defining the bipartite graph. ",
                            "In a bipartite graph the set of source nodes and destination nodes ",
                            "are strictly disjointed. ",
                            "The node that appear in both source and destinations is {node_id} ",
                            "and its node name is {node_name}."
                        ),
                        node_id = current_src,
                        node_name =
                            unsafe { self.get_unchecked_node_name_from_node_id(current_src) }
                    ))
                }
                std::cmp::Ordering::Greater => {
                    dst_index += 1;
                }
                std::cmp::Ordering::Less => {
                    src_index += 1;
                }
            }
        }

        // Depending on whether we need to create a directed
        // or undirected graph using the provided edges
        // we can make very different optimizations.
        if directed {
            let number_of_edges = (number_of_source_nodes * number_of_destination_nodes) as EdgeT;
            build_graph_from_integers(
                Some(
                    source_node_ids
                        .into_par_iter()
                        .enumerate()
                        .flat_map(|(i, src)| {
                            destination_node_ids.par_iter().copied().enumerate().map(
                                move |(j, dst)| {
                                    (
                                        i * number_of_destination_nodes + j,
                                        (
                                            src,
                                            dst,
                                            self.get_edge_type_id_from_edge_node_ids(src, dst)
                                                .unwrap_or(None),
                                            self.get_edge_weight_from_node_ids(src, dst)
                                                .ok()
                                                .unwrap_or(WeightT::NAN),
                                        ),
                                    )
                                },
                            )
                        }),
                ),
                self.nodes.clone(),
                self.node_types.clone(),
                self.edge_types
                    .as_ref()
                    .as_ref()
                    .map(|ets| ets.vocabulary.clone()),
                self.has_edge_weights(),
                true,
                Some(true),
                Some(false),
                Some(true),
                Some(number_of_edges),
                true,
                false,
                self.get_name(),
            )
        } else {
            let number_of_edges =
                2 * (number_of_source_nodes * number_of_destination_nodes) as EdgeT;
            let mut reverse_index =
                (0..(number_of_source_nodes + number_of_destination_nodes)).collect::<Vec<usize>>();
            let get_node_id_from_reverse_index = |reverse_id| {
                if reverse_id < number_of_source_nodes {
                    source_node_ids[reverse_id]
                } else {
                    destination_node_ids[reverse_id - number_of_source_nodes]
                }
            };
            // We need to sort these conjoined sources to create the edge list
            // immediately as sorted and therefore avoid the need for resorting.
            // This is expecially important when the graph to generate is large.
            reverse_index.par_sort_unstable_by(|&a, &b| {
                get_node_id_from_reverse_index(a).cmp(&get_node_id_from_reverse_index(b))
            });

            // We now need to compute the comulative node degrees of the source nodes,
            // which are needed to create the first time around the correct edge IDs.
            let comulative_node_degrees = reverse_index
                .iter()
                .scan(0, |current_degree, &reverse_id| {
                    let old_degree = *current_degree;
                    if reverse_id < number_of_source_nodes {
                        *current_degree += number_of_source_nodes as NodeT;
                    } else {
                        *current_degree += number_of_destination_nodes as NodeT;
                    }
                    Some(old_degree)
                })
                .collect::<Vec<NodeT>>();

            build_graph_from_integers(
                Some(
                    reverse_index
                        .into_par_iter()
                        .zip(comulative_node_degrees.into_par_iter())
                        .flat_map(|(reverse_id, comulative_degree)| {
                            let src = get_node_id_from_reverse_index(reverse_id);
                            let destination_vector: &[NodeT] =
                                if reverse_id < number_of_source_nodes {
                                    source_node_ids.as_ref()
                                } else {
                                    destination_node_ids.as_ref()
                                };

                            destination_vector.par_iter().copied().enumerate().map(
                                move |(j, dst)| {
                                    (
                                        comulative_degree as usize + j,
                                        (
                                            src,
                                            dst,
                                            self.get_edge_type_id_from_edge_node_ids(src, dst)
                                                .unwrap_or(None),
                                            self.get_edge_weight_from_node_ids(src, dst)
                                                .ok()
                                                .unwrap_or(WeightT::NAN),
                                        ),
                                    )
                                },
                            )
                        }),
                ),
                self.nodes.clone(),
                self.node_types.clone(),
                self.edge_types
                    .as_ref()
                    .as_ref()
                    .map(|ets| ets.vocabulary.clone()),
                self.has_edge_weights(),
                false,
                Some(true),
                Some(false),
                Some(true),
                Some(number_of_edges),
                true,
                false,
                self.get_name(),
            )
        }
    }

    /// Returns clique graph between the provided node IDs.
    ///
    /// # Arguments
    /// * `node_ids`: Vec<NodeT> - The node IDs.
    /// * `directed`: bool - Whether to make the graph directed or undirected.
    ///
    pub fn build_clique_graph_from_node_ids(
        &self,
        mut node_ids: Vec<NodeT>,
        directed: bool,
    ) -> Result<Graph> {
        // Validate the provided tuples.
        node_ids
            .par_iter()
            .copied()
            .map(|node_id| self.validate_node_id(node_id).map(|_| ()))
            .collect::<Result<()>>()?;

        // We sort the provided node IDs.
        node_ids.par_sort_unstable();

        // Search duplicates in the provided vectors
        if node_ids.par_windows(2).any(|window| window[0] == window[1]) {
            return Err("There are duplicated nodes in the provided nodes.".to_string());
        }

        let number_of_provided_nodes = node_ids.len();

        let number_of_edges = (number_of_provided_nodes * number_of_provided_nodes) as EdgeT;
        build_graph_from_integers(
            Some(
                node_ids
                    .par_iter()
                    .copied()
                    .enumerate()
                    .flat_map(|(i, src)| {
                        node_ids
                            .par_iter()
                            .copied()
                            .enumerate()
                            .map(move |(j, dst)| {
                                (
                                    i * number_of_provided_nodes + j,
                                    (
                                        src,
                                        dst,
                                        self.get_edge_type_id_from_edge_node_ids(src, dst)
                                            .unwrap_or(None),
                                        self.get_edge_weight_from_node_ids(src, dst)
                                            .ok()
                                            .unwrap_or(WeightT::NAN),
                                    ),
                                )
                            })
                    }),
            ),
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types
                .as_ref()
                .as_ref()
                .map(|ets| ets.vocabulary.clone()),
            self.has_edge_weights(),
            directed,
            Some(true),
            Some(false),
            Some(true),
            Some(number_of_edges),
            true,
            false,
            self.get_name(),
        )
    }

    /// Returns bipartite graph between the provided source and destination node names.
    ///
    /// # Arguments
    /// * `source_node_names`: Vec<&str> - The source node names.
    /// * `destination_node_names`: Vec<&str> - The destination node names.
    /// * `directed`: bool - Whether to make the graph directed or undirected.
    ///
    pub fn build_bipartite_graph_from_edge_node_names(
        &self,
        source_node_names: Vec<&str>,
        destination_node_names: Vec<&str>,
        directed: bool,
    ) -> Result<Graph> {
        self.build_bipartite_graph_from_edge_node_ids(
            source_node_names
                .into_par_iter()
                .map(|node_name| self.get_node_id_from_node_name(node_name))
                .collect::<Result<Vec<NodeT>>>()?,
            destination_node_names
                .into_par_iter()
                .map(|node_name| self.get_node_id_from_node_name(node_name))
                .collect::<Result<Vec<NodeT>>>()?,
            directed,
        )
    }

    /// Returns clique graph between the provided node names.
    ///
    /// # Arguments
    /// * `node_names`: Vec<&str> - The node names.
    /// * `directed`: bool - Whether to make the graph directed or undirected.
    ///
    pub fn build_clique_graph_from_node_names(
        &self,
        node_names: Vec<&str>,
        directed: bool,
    ) -> Result<Graph> {
        self.build_clique_graph_from_node_ids(
            node_names
                .into_par_iter()
                .map(|node_name| self.get_node_id_from_node_name(node_name))
                .collect::<Result<Vec<NodeT>>>()?,
            directed,
        )
    }

    /// Returns bipartite graph between the provided source and destination node prefixes.
    ///
    /// # Arguments
    /// * `source_node_prefixes`: &[&str] - The source node prefixes.
    /// * `destination_node_prefixes`: &[&str] - The destination node prefixes.
    /// * `directed`: bool - Whether to make the graph directed or undirected.
    ///
    pub fn build_bipartite_graph_from_edge_node_prefixes(
        &self,
        source_node_prefixes: &[&str],
        destination_node_prefixes: &[&str],
        directed: bool,
    ) -> Result<Graph> {
        self.build_bipartite_graph_from_edge_node_ids(
            self.get_node_ids_from_node_curie_prefixes(source_node_prefixes),
            self.get_node_ids_from_node_curie_prefixes(destination_node_prefixes),
            directed,
        )
    }

    /// Returns clique graph between the nodes with the provided prefixes.
    ///
    /// # Arguments
    /// * `node_prefixes`: &[&str] - The node name prefixes.
    /// * `directed`: bool - Whether to make the graph directed or undirected.
    ///
    pub fn build_clique_graph_from_node_prefixes(
        &self,
        node_prefixes: &[&str],
        directed: bool,
    ) -> Result<Graph> {
        self.build_clique_graph_from_node_ids(
            self.get_node_ids_from_node_curie_prefixes(node_prefixes),
            directed,
        )
    }

    /// Returns bipartite graph between the provided source and destination node types.
    ///
    /// # Arguments
    /// * `source_node_types`: &[Option<&str>] - The source node types.
    /// * `destination_node_types`: &[Option<&str>] - The destination node types.
    /// * `directed`: bool - Whether to make the graph directed or undirected.
    ///
    pub fn build_bipartite_graph_from_edge_node_types(
        &self,
        source_node_types: &[Option<&str>],
        destination_node_types: &[Option<&str>],
        directed: bool,
    ) -> Result<Graph> {
        self.build_bipartite_graph_from_edge_node_ids(
            self.get_node_ids_from_node_type_names(source_node_types)?,
            self.get_node_ids_from_node_type_names(destination_node_types)?,
            directed,
        )
    }

    /// Returns clique graph between the nodes with the provided node types.
    ///
    /// # Arguments
    /// * `node_type_names`: &[Option<&str>] - The node name types.
    /// * `directed`: bool - Whether to make the graph directed or undirected.
    ///
    pub fn build_clique_graph_from_node_type_names(
        &self,
        node_type_names: &[Option<&str>],
        directed: bool,
    ) -> Result<Graph> {
        self.build_clique_graph_from_node_ids(
            self.get_node_ids_from_node_type_names(node_type_names)?,
            directed,
        )
    }
}
