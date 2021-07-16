use crate::constructors::{
    build_graph_from_integers, build_graph_from_strings_without_type_iterators,
};

use super::*;
use indicatif::ParallelProgressIterator;
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

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
        if self.get_nodes_number() != other.get_nodes_number() {
            return false;
        }
        self.par_iter_node_names_and_node_type_names()
            .all(|(_, node_name, _, node_type)| {
                other.has_node_name_and_node_type_name(&node_name, node_type)
            })
    }

    /// Returns graph remapped using given node IDs ordering.
    ///
    /// # Arguments
    /// * `node_ids`: Vec<NodeT> - The node Ids to remap the graph to.
    /// * `verbose`: Option<bool> - Whether to show a loading bar while building the graph.
    ///
    /// # Safety
    /// This method will cause a panic if the node IDs are either:
    /// * Not unique
    /// * Not available for each of the node IDs of the graph.
    pub unsafe fn remap_unchecked_from_node_ids(
        &self,
        node_ids: Vec<NodeT>,
        verbose: Option<bool>,
    ) -> Graph {
        // let verbose = verbose.unwrap_or(true);

        // let pb_edges = get_loading_bar(
        //     verbose,
        //     "Building sorted edges with node IDs in increasing outbound node degree",
        //     self.get_directed_edges_number() as usize,
        // );

        // let pb_nodes = get_loading_bar(
        //     verbose,
        //     "Building node IDs {} with increasing outbound node degree",
        //     self.get_nodes_number() as usize,
        // );

        build_graph_from_strings_without_type_iterators(
            self.has_node_types(),
            Some(node_ids.into_par_iter().map(|node_id| unsafe {
                Ok((
                    node_id as usize,
                    (
                        self.get_unchecked_node_name_from_node_id(node_id),
                        self.get_unchecked_node_type_names_from_node_id(node_id),
                    ),
                ))
            })),
            Some(self.get_nodes_number()),
            true,
            false,
            false,
            None,
            self.has_edge_types(),
            Some(
                self.par_iter_directed_edge_node_names_and_edge_type_name_and_edge_weight()
                    .map(|(_, _, src_name, _, dst_name, _, edge_type_name, weight)| {
                        Ok((
                            0,
                            (
                                src_name,
                                dst_name,
                                edge_type_name,
                                weight.unwrap_or(WeightT::NAN),
                            ),
                        ))
                    }),
            ),
            self.has_edge_weights(),
            self.is_directed(),
            Some(true),
            Some(true),
            Some(false),
            Some(false),
            Some(self.get_directed_edges_number()),
            None,
            None,
            self.get_name(),
        )
        .unwrap()
    }

    /// Returns graph remapped using given node IDs ordering.
    ///
    /// # Arguments
    /// * `node_ids`: Vec<NodeT> - The node Ids to remap the graph to.
    /// * `verbose`: Option<bool> - Whether to show a loading bar while building the graph.
    ///
    /// # Raises
    /// * If the given node IDs are not unique.
    /// * If the given node IDs are not available for all the values in the graph.
    pub fn remap_from_node_ids(
        &self,
        node_ids: Vec<NodeT>,
        verbose: Option<bool>,
    ) -> Result<Graph> {
        if node_ids.len() != self.get_nodes_number() as usize {
            return Err(format!(
                concat!(
                    "The provided node IDs list has length {}, ",
                    "while the number of nodes in the current graph is {}."
                ),
                node_ids.len(),
                self.get_nodes_number()
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
        if max != self.get_nodes_number() - 1 {
            return Err(format!(
                concat!(
                    "The maximum node ID provided in the given mapping is {}, ",
                    "while {} was expected."
                ),
                max,
                self.get_nodes_number() - 1
            ));
        }
        let without_duplicates_len = node_ids.iter().unique().count();
        if without_duplicates_len != node_ids.len() {
            return Err(format!(
                "There are {} duplicated values in the provided node IDs.",
                node_ids.len() - without_duplicates_len
            ));
        }
        Ok(unsafe { self.remap_unchecked_from_node_ids(node_ids, verbose) })
    }

    /// Returns graph remapped using given node names ordering.
    ///
    /// # Arguments
    /// * `node_names`: Vec<&str> - The node names to remap the graph to.
    /// * `verbose`: Option<bool> - Whether to show a loading bar while building the graph.
    ///
    /// # Raises
    /// * If the given node names are not unique.
    /// * If the given node names are not available for all the values in the graph.
    pub fn remap_from_node_names(
        &self,
        node_names: Vec<&str>,
        verbose: Option<bool>,
    ) -> Result<Graph> {
        self.remap_from_node_ids(
            node_names
                .into_iter()
                .map(|node_name| self.get_node_id_from_node_name(node_name))
                .collect::<Result<Vec<NodeT>>>()?,
            verbose,
        )
    }

    /// Return graph remapped towards nodes of the given graph.
    ///
    /// # Arguments
    ///
    /// * `other`: &Graph - The graph to remap towards.
    /// * `verbose`: Option<bool> - Whether to show a loding bar.
    ///
    /// # Example
    /// A graph is always remappable to itself:
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// assert_eq!(graph, graph.remap_from_graph(&graph, None).unwrap());
    /// ```
    ///
    pub fn remap_from_graph(&self, other: &Graph, verbose: Option<bool>) -> Result<Graph> {
        let verbose = verbose.unwrap_or(false);
        let pb = get_loading_bar(
            verbose,
            format!("Building remapped {}", self.name).as_ref(),
            self.get_directed_edges_number() as usize,
        );

        if !self.are_nodes_remappable(other) {
            return Err("The two graphs nodes sets are not remappable one-another.".to_owned());
        }

        build_graph_from_integers(
            Some(
                self.par_iter_edge_node_names_and_edge_type_name_and_edge_weight(true)
                    .progress_with(pb)
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
            self.edge_types.as_ref().map(|ets| ets.vocabulary.clone()),
            self.has_edge_weights(),
            self.is_directed(),
            Some(true),
            Some(false),
            // Because of the remapping the resulting edge list
            // may not be sorted.
            Some(false),
            Some(self.get_directed_edges_number()),
            self.get_name(),
        )
    }
}
