use crate::constructors::{build_graph_from_integers, build_graph_from_strings};
use rayon::prelude::*;

use super::*;
use itertools::Itertools;
use std::ops;

fn build_operator_graph_name(main: &Graph, other: &Graph, operator: String) -> String {
    format!("({} {} {})", main.name, operator, other.name)
}

/// Return graph composed of the two near-incompatible graphs.
///
/// The two graphs can have different nodes, edge types and node types.
/// These operators are slower than the generic integer operators since they
/// require a reverse mapping step.
///
/// # Arguments
/// * `main`: &Graph - The current graph instance.
/// * `other`: &Graph - The other graph.
/// * `operator`: String - The operator used.
/// * `graphs`: Vec<(&Graph, Option<&Graph>, Option<&Graph>)> - Graph list for the operation.
/// * `may_have_singletons`: bool - Whether the resulting graph may have singletons.
fn generic_string_operator(
    main: &Graph,
    other: &Graph,
    operator: String,
    graphs: Vec<(&Graph, Option<&Graph>, Option<&Graph>)>,
    may_have_singletons: bool,
) -> Result<Graph> {
    // Chaining node types of the two graphs so to define the set of shared node types.
    let mut combined_node_type_names: Vec<String> = main
        .get_unique_node_type_names()
        .unwrap_or_else(|_| Vec::new());

    if let Ok(other_node_types_iter) = other.iter_unique_node_type_names() {
        combined_node_type_names.extend(other_node_types_iter);
    }

    // We need to sort the node type names so to avoid having different
    // node type names ordering when executing the same operation multiple times.
    combined_node_type_names.par_sort_unstable();

    let number_of_node_types = combined_node_type_names.len() as NodeT;
    let node_types_iterator: ItersWrapper<_, std::iter::Empty<_>, _> = ItersWrapper::Parallel(
        combined_node_type_names
            .into_par_iter()
            .enumerate()
            .map(|(node_type_id, node_type_name)| Ok((node_type_id, node_type_name))),
    );

    // Chaining edge types of the two graphs so to define the set of shared edge types.
    let mut combined_edge_type_names: Vec<String> = main
        .get_unique_edge_type_names()
        .unwrap_or_else(|_| Vec::new());

    if let Ok(other_edge_types_iter) = other.iter_unique_edge_type_names() {
        combined_edge_type_names.extend(other_edge_types_iter);
    }

    // We need to sort the edge type names so to avoid having different
    // edge type names ordering when executing the same operation multiple times.
    combined_edge_type_names.par_sort_unstable();

    let number_of_edge_types = combined_edge_type_names.len() as NodeT;
    let edge_types_iterator: ItersWrapper<_, std::iter::Empty<_>, _> = ItersWrapper::Parallel(
        combined_edge_type_names
            .into_par_iter()
            .enumerate()
            .map(|(edge_type_id, edge_type_name)| Ok((edge_type_id, edge_type_name))),
    );

    // one: left hand side of the operator
    // deny_graph: right hand edges "deny list"
    // must_have_graph: right hand edges "must have list
    let edges_iterator: ItersWrapper<_, std::iter::Empty<_>, _> =
        ItersWrapper::Parallel(graphs.into_par_iter().flat_map(
            |(one, deny_graph, must_have_graph)| {
                one.par_iter_directed_edge_node_names_and_edge_type_name_and_edge_weight()
                    .filter(move |(_, _, src_name, _, dst_name, _, edge_type_name, _)| {
                        // If the secondary graph is given
                        // we filter out the edges that were previously added to avoid
                        // introducing duplicates.
                        // TODO: handle None type edge types and avoid duplicating those!
                        if let Some(dg) = deny_graph {
                            if dg.has_edge_from_node_names_and_edge_type_name(
                                src_name,
                                dst_name,
                                edge_type_name.as_deref(),
                            ) {
                                return false;
                            }
                        }
                        if let Some(mhg) = must_have_graph {
                            if !mhg.has_edge_from_node_names_and_edge_type_name(
                                src_name,
                                dst_name,
                                edge_type_name.as_deref(),
                            ) {
                                return false;
                            }
                        }
                        true
                    })
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
                    })
            },
        ));

    // Chaining node types in a way that merges the information between
    // two node type sets where one of the two has some unknown node types
    let mut nodes: Vec<_> = main
        .par_iter_node_names_and_node_type_names()
        .map(|(_, node_name, _, node_type_names)| {
            // We retrieve the node type names of the other graph, if the node
            // even exists in the other graph.
            let other_node_type_names = other
                .get_node_type_names_from_node_name(&node_name)
                .unwrap_or(None);
            // According to whether the current node has one or node type names
            // in the current main graph or one or more of the other graphs
            // we need to merge this properly.
            let node_type_names = match (node_type_names, other_node_type_names) {
                // In the first case, the node types are present in both source graphs.
                // In this use case we need to merge the two node types.
                (Some(main_ntns), Some(other_ntns)) => Some(
                    main_ntns
                        .into_iter()
                        .chain(other_ntns.into_iter())
                        .unique()
                        .collect::<Vec<String>>(),
                ),
                // If it is present only in the first one, we keep only the first one.
                (Some(main_ntns), None) => Some(main_ntns),
                // If it is present only in the second one, we keep only the secondo one.
                (None, Some(other_ntns)) => Some(other_ntns),
                // If it is not present in either, we can only return None.
                (None, None) => None,
            };
            (node_name, node_type_names)
        })
        .chain(other.par_iter_node_names_and_node_type_names().filter_map(
            |(_, node_name, _, node_type_names)| match main.has_node_name(&node_name) {
                true => None,
                false => Some((node_name, node_type_names)),
            },
        ))
        .collect();

    // The following is necessary to ensure the node disctionaries are consistent
    // across multiple runs.
    nodes.par_sort_unstable();
    let number_of_nodes = nodes.len() as NodeT;
    let nodes_iterator: ItersWrapper<_, std::iter::Empty<_>, _> =
        ItersWrapper::Parallel(nodes.into_par_iter().enumerate().map(|entry| Ok(entry)));

    build_graph_from_strings(
        if number_of_node_types > 0 {
            Some(node_types_iterator)
        } else {
            None
        },
        Some(number_of_node_types as NodeTypeT),
        Some(false),
        None,
        main.has_node_types() || other.has_node_types(),
        Some(true),
        Some(nodes_iterator),
        Some(number_of_nodes),
        true,
        false,
        false,
        None,
        if number_of_edge_types > 0 {
            Some(edge_types_iterator)
        } else {
            None
        },
        Some(number_of_edge_types as EdgeTypeT),
        Some(false),
        None,
        main.has_edge_types() || other.has_edge_types(),
        Some(true),
        Some(edges_iterator),
        main.has_edge_weights(),
        main.is_directed(),
        Some(true),
        Some(true),
        Some(false),
        Some(false),
        None,
        None,
        None,
        None,
        None,
        true,
        main.has_selfloops() || other.has_selfloops(),
        build_operator_graph_name(main, other, operator),
    )
}

/// Return graph composed of the two compatible graphs.
///
/// The two graphs CANNOT have different nodes, edge types and node types.
/// These operators are faster than the generic string operators since they
/// do NOT require a reverse mapping step.
///
/// # Arguments
///
/// * `main`: &Graph - The current graph instance.
/// * `other`: &Graph - The other graph.
/// * `operator`: String - The operator used.
/// * `graphs`: Vec<(&Graph, Option<&Graph>, Option<&Graph>)> - Graph list for the operation.
/// * `may_have_singletons`: bool - Whether the resulting graph may have singletons.
fn generic_integer_operator(
    main: &Graph,
    other: &Graph,
    operator: String,
    graphs: Vec<(&Graph, Option<&Graph>, Option<&Graph>)>,
    may_have_singletons: bool,
) -> Graph {
    // one: left hand side of the operator
    // deny_graph: right hand edges "deny list"
    // must_have_graph: right hand edges "must have list
    let edges_iterator =
        graphs
            .into_par_iter()
            .flat_map_iter(|(one, deny_graph, must_have_graph)| {
                one.iter_directed_edge_node_ids_and_edge_type_id_and_edge_weight()
                    .filter(move |(_, src, dst, edge_type, _)| {
                        // If the secondary graph is given
                        // we filter out the edges that were previously added to avoid
                        // introducing duplicates.
                        if let Some(dg) = deny_graph {
                            if dg.has_edge_from_node_ids_and_edge_type_id(*src, *dst, *edge_type) {
                                return false;
                            }
                        }
                        if let Some(mhg) = must_have_graph {
                            if !mhg.has_edge_from_node_ids_and_edge_type_id(*src, *dst, *edge_type)
                            {
                                return false;
                            }
                        }
                        true
                    })
                    .map(|(_, src, dst, edge_type, weight)| {
                        (0, (src, dst, edge_type, weight.unwrap_or(WeightT::NAN)))
                    })
            });

    let node_types = match (&*main.node_types, &*other.node_types) {
        (Some(mnts), Some(onts)) => Some(match mnts == onts {
            true => mnts.clone(),
            false => {
                let mut main_node_types = mnts.ids.clone();
                main_node_types
                    .iter_mut()
                    .zip(onts.ids.iter())
                    .for_each(|(mid, oid)| {
                        if mid.is_none() {
                            *mid = oid.clone();
                        }
                    });
                NodeTypeVocabulary::from_structs(main_node_types, mnts.vocabulary.clone())
            }
        }),
        (Some(mnts), _) => Some(mnts.clone()),
        (_, Some(onts)) => Some(onts.clone()),
        _ => None,
    };

    build_graph_from_integers(
        Some(edges_iterator),
        main.nodes.clone(),
        Arc::new(node_types),
        main.edge_types
            .as_ref()
            .as_ref()
            .map(|ets| ets.vocabulary.clone()),
        main.has_edge_weights(),
        main.is_directed(),
        Some(true),
        Some(false),
        Some(false),
        None,
        may_have_singletons,
        main.has_selfloops() || other.has_selfloops(),
        build_operator_graph_name(main, other, operator),
    )
    .unwrap()
}

impl<'a, 'b> Graph {
    /// Return result containing either empty tuple or error representing what makes impossible to combine the two graphs.
    ///
    /// # Arguments
    ///
    /// * `other`: &Graph - The other graph to validate operation with.
    ///
    /// # Raises
    /// * If a graph is directed and the other is undirected.
    /// * If one of the two graphs has edge weights and the other does not.
    /// * If one of the two graphs has node types and the other does not.
    /// * If one of the two graphs has edge types and the other does not.
    pub(crate) fn validate_operator_terms(&self, other: &'b Graph) -> Result<()> {
        if self.directed != other.directed {
            return Err(String::from(
                "The graphs must either be both directed or undirected.",
            ));
        }

        if self.has_edge_weights() != other.has_edge_weights() {
            return Err(String::from(
                "Both graphs need to have weights or neither can.",
            ));
        }

        if self.has_edge_types() != other.has_edge_types() {
            return Err(String::from(
                "Both graphs need to have edge types or neither can.",
            ));
        }

        Ok(())
    }
}

impl Graph {
    /// Returns whether the graphs share the same nodes.
    ///
    /// # Arguments
    /// * `other`: &Graph - The other graph.
    pub fn has_compatible_node_vocabularies(&self, other: &Graph) -> bool {
        self.nodes == other.nodes
    }

    /// Returns whether the graphs share the same node types or absence thereof.
    ///
    /// # Arguments
    /// * `other`: &Graph - The other graph.
    pub fn has_compatible_node_types_vocabularies(&self, other: &Graph) -> bool {
        match (&*self.node_types, &*other.node_types) {
            (Some(snts), Some(onts)) => snts.vocabulary == onts.vocabulary,
            (None, None) => true,
            _ => false,
        }
    }

    /// Returns whether the graphs share the same edge types or absence thereof.
    ///
    /// # Arguments
    /// * `other`: &Graph - The other graph.
    pub fn has_compatible_edge_types_vocabularies(&self, other: &Graph) -> bool {
        match (&*self.edge_types, &*other.edge_types) {
            (Some(sets), Some(oets)) => sets.vocabulary == oets.vocabulary,
            (None, None) => true,
            _ => false,
        }
    }

    /// Return true if the graphs are compatible.
    ///
    /// # Arguments
    ///
    /// * `other`: &Graph - The other graph.
    ///
    /// # Raises
    /// * If a graph is directed and the other is undirected.
    /// * If one of the two graphs has edge weights and the other does not.
    /// * If one of the two graphs has node types and the other does not.
    /// * If one of the two graphs has edge types and the other does not.
    pub fn is_compatible(&self, other: &Graph) -> Result<bool> {
        self.validate_operator_terms(other)?;
        Ok(self.has_compatible_node_vocabularies(other)
            & self.has_compatible_node_types_vocabularies(other)
            & self.has_compatible_edge_types_vocabularies(other))
    }

    /// Return true if the graphs share the same adjacency matrix.
    ///
    /// # Arguments
    /// * `other`: &Graph - The other graph.
    pub fn has_same_adjacency_matrix(&self, other: &Graph) -> Result<bool> {
        if self.nodes != other.nodes {
            return Ok(false);
        }
        if self.get_number_of_directed_edges() != other.get_number_of_directed_edges() {
            return Ok(false);
        }
        if self.get_number_of_selfloops() != other.get_number_of_selfloops() {
            return Ok(false);
        }
        Ok(self
            .par_iter_directed_edge_node_ids()
            .zip(other.par_iter_directed_edge_node_ids())
            .all(|(e1, e2)| e1 == e2))
    }

    /// Return graph composed of the two near-incompatible graphs.
    ///
    /// The two graphs can have different nodes, edge types and node types.
    /// These operators are slower than the generic integer operators since they
    /// require a reverse mapping step.
    ///
    /// # Arguments
    ///
    /// * `other`: &Graph - The other graph.
    /// * `operator`: String - The operator used.
    /// * `graphs`: Vec<(&Graph, Option<&Graph>, Option<&Graph>)> - Graph list for the operation.
    /// * `may_have_singletons`: bool - Whether the resulting graph may have singletons.
    pub(crate) fn generic_operator(
        &self,
        other: &Graph,
        operator: String,
        graphs: Vec<(&Graph, Option<&Graph>, Option<&Graph>)>,
        may_have_singletons: bool,
    ) -> Result<Graph> {
        match self.is_compatible(other)? {
            true => Ok(generic_integer_operator(
                self,
                other,
                operator,
                graphs,
                may_have_singletons,
            )),
            false => generic_string_operator(self, other, operator, graphs, may_have_singletons),
        }
    }
}

impl<'a, 'b> ops::BitOr<&'b Graph> for &'a Graph {
    type Output = Result<Graph>;
    /// Return graph composed of the two graphs.
    ///
    /// The two graphs must have the same nodes, node types and edge types.
    ///
    /// # Arguments
    ///
    /// * `other`: &Graph - Graph to be summed.
    ///
    fn bitor(self, other: &'b Graph) -> Result<Graph> {
        self.generic_operator(
            other,
            "|".to_owned(),
            vec![(self, None, None), (other, Some(self), None)],
            // An or operation merges two graphs. The resulting graph can have
            // at most a number of singleton nodes equal to the sum of the
            // singleton nodes of the two graphs. If the two original graphs
            // have no singleton nodes, the resulting graph cannot have singleton
            // nodes.
            self.has_singleton_nodes() || other.has_singleton_nodes(),
        )
    }
}

impl<'a, 'b> ops::BitXor<&'b Graph> for &'a Graph {
    type Output = Result<Graph>;
    /// Return graph composed of the two graphs.
    ///
    /// The two graphs must have the same nodes, node types and edge types.
    ///
    /// # Arguments
    ///
    /// * `other`: &Graph - Graph to be summed.
    ///
    fn bitxor(self, other: &'b Graph) -> Result<Graph> {
        self.generic_operator(
            self,
            "^".to_owned(),
            vec![(self, Some(other), None), (other, Some(self), None)],
            // A bitwise xor operation merges two graphs. The resulting graph can have
            // more singleton nodes than either of the two original graph as edges are
            // being removed as part of this operation.
            true,
        )
    }
}

impl<'a, 'b> ops::Sub<&'b Graph> for &'a Graph {
    type Output = Result<Graph>;
    /// Return subtraction for graphs objects.
    ///
    /// The two graphs must have the same nodes, node types and edge types.
    ///
    /// # Arguments
    ///
    /// * `other`: &Graph - Graph to be subtracted.
    ///
    fn sub(self, other: &'b Graph) -> Result<Graph> {
        self.generic_operator(other, "-".to_owned(), vec![(self, Some(other), None)], true)
    }
}

impl<'a, 'b> ops::BitAnd<&'b Graph> for &'a Graph {
    type Output = Result<Graph>;
    /// Return graph obtained from the intersection of the two graph.
    ///
    /// The two graphs must have the same nodes, node types and edge types.
    ///
    /// # Arguments
    ///
    /// * `other`: &Graph - Graph to be subtracted.
    ///
    fn bitand(self, other: &'b Graph) -> Result<Graph> {
        self.generic_operator(other, "&".to_owned(), vec![(self, None, Some(other))], true)
    }
}
