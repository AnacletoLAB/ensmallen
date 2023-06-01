use super::*;
use crate::constructors::build_graph_from_integers;
use itertools::Itertools;
use log::info;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::collections::HashMap;

impl Graph {
    /// Return whether nodes are remappable to those of the given graph.
    ///
    /// # Arguments
    /// * `other`: &Graph - graph towards remap the nodes to.
    ///
    /// # Example
    /// A graph is always remappable to itself:
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// assert!(graph.are_nodes_remappable(&graph));
    /// ```
    /// Two different graphs, like Cora and STRING, are not remappable:
    /// ```rust
    /// # let cora = graph::test_utilities::load_cora();
    /// # let ppi = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// assert!(!cora.are_nodes_remappable(&ppi));
    /// ```
    ///
    pub fn are_nodes_remappable(&self, other: &Graph) -> bool {
        if self.get_number_of_nodes() != other.get_number_of_nodes() {
            return false;
        }
        self.par_iter_node_names_and_node_type_names()
            .all(|(_, node_name, _, node_type)| {
                other.has_node_name_and_node_type_name(&node_name, node_type)
            })
    }

    /// Returns graph remapped using given node IDs ordering and vocabulary.
    ///
    /// # Arguments
    /// * `positions`: &[NodeT] - Vector of the node IDs representing the new positions of the node names.
    /// * `vocabulary`: Vocabulary<NodeT> - New vocabulary to use to remap the nodes.
    ///
    /// # Examples
    ///
    /// ## Remapping one to one
    /// Considering the use case where you need to remap the graph nodes vocabulary
    /// to another mapping where to each node curresponding exactly to a single new
    /// entity.
    ///
    /// TODO: add code example.
    ///
    /// ## Remapping many to one
    /// Considering the use case where you need to remap the graph nodes vocabulary
    /// to another mapping where to multiple node may currespond the same destination.
    /// One such use case, for instance, may be the remapping of protein nodes to the
    /// relative coding gene (as many proteins usually do map to the same gene).
    ///
    /// In such cases, we expect the node IDs vector to contain the same value (the ID
    /// relative to the gene) repeated multiple times for each of the proteins that
    /// are meant to remap to the aforementioned gene.
    ///
    /// If the original nodes have node types, these need to be aggregates into the target
    /// node through union, so if the source nodes have for instance as node types
    /// `{"NamedThing"}` and `{"Drug"}`, the resulting node will have as node types
    /// the union of the previous node types set `{"NamedThing", "Drug"}`.
    ///
    /// If the different nodes already share a common neighbouring node, merging the nodes
    /// will turn the original graph into a multi-graph, that is, a graph where multiple
    /// edges exist between some pair of nodes in the graph.
    ///
    /// TODO: add code example.
    ///
    /// # Safety
    /// The method is undefined when the provided node IDs are not compatible with
    /// the current graph instance and may raise a panic.
    unsafe fn remap_unchecked_from_positions_and_vocabulary(
        &self,
        positions: &[NodeT],
        vocabulary: Vocabulary<NodeT>,
    ) -> Graph {
        // The node IDs are expected to have the same length
        // as the number of nodes in the current graph instance.
        if positions.len() as NodeT != self.get_number_of_nodes() {
            panic!(
                concat!(
                    "The provided positions is not compatible with ",
                    "the current graph instance. You have provided a node IDs ",
                    "vector of length {}, while the number of nodes in the ",
                    "current graph is {}."
                ),
                positions.len(),
                self.get_number_of_nodes()
            );
        }

        // Create the new Node Type Vocabulary object
        // if the current graph instance has node types.
        let new_node_types = if let Some(node_types) = self.node_types.as_ref() {
            // First we create the empty vector of node type IDs, with initially
            // value `None`, representing `unknown` node types.
            let mut remapped_node_type_ids: Vec<Option<Vec<NodeTypeT>>> =
                vec![None; vocabulary.len()];
            // Secondly, we iterate over the provided node IDs vector and sequentially
            // merge the nodes' node types with the remapped node types.
            self.iter_node_ids_and_node_type_ids()
                .for_each(|(node_id, node_type_ids)| {
                    // If the node originally had node types
                    if let Some(node_type_ids) = node_type_ids {
                        // We retrieve the new position for the node types.
                        let new_node_id = positions[node_id as usize] as usize;
                        // Assign / extend inplace the set of node type IDs that are already
                        // present.
                        if let Some(new_node_type_ids) = &mut remapped_node_type_ids[new_node_id] {
                            node_type_ids.iter().for_each(|&node_type_id| {
                                if !new_node_type_ids.contains(&node_type_id) {
                                    new_node_type_ids.push(node_type_id);
                                }
                            });
                            new_node_type_ids.sort_unstable();
                        } else {
                            remapped_node_type_ids[new_node_id] = Some(node_type_ids.to_vec());
                        }
                    }
                });
            Some(NodeTypeVocabulary::from_structs(
                remapped_node_type_ids,
                node_types.vocabulary.clone(),
            ))
        } else {
            None
        };

        build_graph_from_integers(
            Some(
                self.par_iter_directed_edge_node_ids_and_edge_type_id_and_edge_weight()
                    .map(|(_, src_name_id, dst_name_id, edge_type_id, weight)| {
                        (
                            0,
                            (
                                positions[src_name_id as usize],
                                positions[dst_name_id as usize],
                                edge_type_id,
                                weight.unwrap_or(WeightT::NAN),
                            ),
                        )
                    }),
            ),
            Arc::new(vocabulary),
            Arc::new(new_node_types),
            self.edge_types
                .as_ref()
                .as_ref()
                .map(|ets| ets.vocabulary.clone()),
            self.has_edge_weights(),
            self.is_directed(),
            Some(true),
            Some(false),
            Some(false),
            Some(self.get_number_of_directed_edges()),
            true,
            true,
            self.get_name(),
        )
        .unwrap()
    }

    /// Returns graph remapped using given node IDs ordering.
    ///
    /// # Arguments
    /// * `node_ids`: Vec<NodeT> - The node Ids to remap the graph to.
    ///
    /// # Safety
    /// This method will cause a panic if the node IDs are either:
    /// * Not unique
    /// * Not available for each of the node IDs of the graph.
    pub unsafe fn remap_unchecked_from_node_ids(&self, node_ids: Vec<NodeT>) -> Graph {
        let new_nodes_vocabulary: Vocabulary<NodeT> = Vocabulary::from_reverse_map(
            node_ids
                .into_par_iter()
                .map(|node_id| self.get_unchecked_node_name_from_node_id(node_id))
                .collect(),
                "Nodes".to_string()
        )
        .unwrap();
        let positions = self
            .par_iter_node_names()
            .map(|node_name| new_nodes_vocabulary.get(&node_name).unwrap())
            .collect::<Vec<NodeT>>();
        self.remap_unchecked_from_positions_and_vocabulary(&positions, new_nodes_vocabulary)
    }

    /// Returns graph remapped using given node IDs ordering.
    ///
    /// # Arguments
    /// * `node_ids`: Vec<NodeT> - The node Ids to remap the graph to.
    ///
    /// # Raises
    /// * If the given node IDs are not unique.
    /// * If the given node IDs are not available for all the values in the graph.
    pub fn remap_from_node_ids(&self, node_ids: Vec<NodeT>) -> Result<Graph> {
        if node_ids.len() != self.get_number_of_nodes() as usize {
            return Err(format!(
                concat!(
                    "The provided node IDs list has length {}, ",
                    "while the number of nodes in the current graph is {}."
                ),
                node_ids.len(),
                self.get_number_of_nodes()
            ));
        }
        if !self.has_nodes() {
            return Ok(self.clone());
        }
        let (min, max) = node_ids.iter().cloned().minmax().into_option().unwrap();
        if min != 0 {
            return Err(format!(
                concat!(
                    "The minimum node ID provided in the given mapping is {}, ",
                    "while 0 was expected."
                ),
                min
            ));
        }
        if max != self.get_number_of_nodes() - 1 {
            return Err(format!(
                concat!(
                    "The maximum node ID provided in the given mapping is {}, ",
                    "while {} was expected."
                ),
                max,
                self.get_number_of_nodes() - 1
            ));
        }
        let without_duplicates_len = node_ids.iter().unique().count();
        if without_duplicates_len != node_ids.len() {
            return Err(format!(
                "There are {} duplicated values in the provided node IDs.",
                node_ids.len() - without_duplicates_len
            ));
        }
        Ok(unsafe { self.remap_unchecked_from_node_ids(node_ids) })
    }

    /// Returns graph remapped using given node names ordering.
    ///
    /// # Arguments
    /// * `node_names`: Vec<&str> - The node names to remap the graph to.
    ///
    /// # Raises
    /// * If the given node names are not unique.
    /// * If the given node names are not available for all the values in the graph.
    pub fn remap_from_node_names(&self, node_names: Vec<&str>) -> Result<Graph> {
        self.remap_from_node_ids(
            node_names
                .into_iter()
                .map(|node_name| self.get_node_id_from_node_name(node_name))
                .collect::<Result<Vec<NodeT>>>()?,
        )
    }

    /// Returns graph remapped using given node names mapping hashmap.
    ///
    /// # Arguments
    /// * `node_names_map`: HashMap<String, String> - The node names to remap the graph to.
    ///
    pub fn remap_from_node_names_map(
        &self,
        node_names_map: HashMap<String, String>,
    ) -> Result<Graph> {
        // First of all, we need to check that the node names map is indeed mono-directional
        // and does not contain loops or circles.
        info!("Checking that all original node names exist in the current graph.");
        node_names_map
            .keys()
            .map(|node_name| {
                if self.has_node_name(node_name) {
                    Ok(())
                } else {
                    Err(format!(
                        concat!(
                            "One of the provided source nodes, {}, does not exist in ",
                            "the current graph instance."
                        ),
                        node_name
                    ))
                }
            })
            .collect::<Result<()>>()?;
        info!("Checking for the existance of circles in provided mapping.");
        node_names_map
            .iter()
            .map(|(original_node_name, destination_node_name)| {
                if original_node_name != destination_node_name
                    && node_names_map
                        .get(destination_node_name)
                        .map_or(false, |further_step| further_step == destination_node_name)
                {
                    Err(format!(
                        concat!(
                            "A loop or chain was identified in the provided mapping, that is a node existing in the original ",
                            "graph `{}` is remapped to another node `{}` that also exists in the original graph. This ",
                            "latter node is then remapped to yet another node, causing therefore either a chain or a loop of ",
                            "redirections. Since resolving redirection loops is not generally defined, and the expected ",
                            "behaviour changes from case to case, please provide a mapping where the mapping is free of ",
                            "these rediction chains and loops."
                        ),
                        original_node_name,
                        destination_node_name
                    ))
                } else {
                    Ok(())
                }
            }).collect::<Result<()>>()?;

        let new_nodes_vocabulary: Vocabulary<NodeT> =
            Vocabulary::from_reverse_map(node_names_map.values().cloned().unique().collect(), "Nodes".to_string())
                .unwrap();
        let positions = self
            .par_iter_node_names()
            .map(|node_name| {
                new_nodes_vocabulary
                    .get(node_names_map.get(&node_name).unwrap())
                    .unwrap()
            })
            .collect::<Vec<NodeT>>();
        Ok(unsafe {
            self.remap_unchecked_from_positions_and_vocabulary(&positions, new_nodes_vocabulary)
        })
    }

    /// Return graph remapped towards nodes of the given graph.
    ///
    /// # Arguments
    ///
    /// * `other`: &Graph - The graph to remap towards.
    ///
    /// # Example
    /// A graph is always remappable to itself:
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// assert_eq!(graph, graph.remap_from_graph(&graph).unwrap());
    /// ```
    ///
    pub fn remap_from_graph(&self, other: &Graph) -> Result<Graph> {
        if !self.are_nodes_remappable(other) {
            return Err("The two graphs nodes sets are not remappable one-another.".to_owned());
        }

        build_graph_from_integers(
            Some(
                self.par_iter_directed_edge_node_names_and_edge_type_name_and_edge_weight()
                    .map(
                        |(edge_id, _, src_name, _, dst_name, _, edge_type, weight)| unsafe {
                            (
                                edge_id as usize,
                                (
                                    other.get_unchecked_node_id_from_node_name(&src_name),
                                    other.get_unchecked_node_id_from_node_name(&dst_name),
                                    edge_type.and_then(|et| {
                                        self.get_unchecked_edge_type_id_from_edge_type_name(
                                            et.as_str(),
                                        )
                                    }),
                                    weight.unwrap_or(WeightT::NAN),
                                ),
                            )
                        },
                    ),
            ),
            other.nodes.clone(),
            other.node_types.clone(),
            self.edge_types
                .as_ref()
                .as_ref()
                .map(|ets| ets.vocabulary.clone()),
            self.has_edge_weights(),
            self.is_directed(),
            Some(true),
            Some(false),
            // Because of the remapping the resulting edge list
            // may not be sorted.
            Some(false),
            Some(self.get_number_of_directed_edges()),
            self.has_singleton_nodes(),
            self.has_singleton_nodes_with_selfloops(),
            self.get_name(),
        )
    }
}
