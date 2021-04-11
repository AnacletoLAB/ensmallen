use super::*;
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
///
/// * main: &Graph - The current graph instance.
/// * other: &Graph - The other graph.
/// * operator: String - The operator used.
/// * graphs: Vec<(&Graph, Option<&Graph>, Option<&Graph>)> - Graph list for the operation.
/// TODO: update docstring
fn generic_string_operator(
    main: &Graph,
    other: &Graph,
    operator: String,
    graphs: Vec<(&Graph, Option<&Graph>, Option<&Graph>)>,
    might_have_singletons: bool,
    might_have_singletons_with_selfloops: bool,
    might_have_trap_nodes: bool,
) -> Result<Graph, String> {
    // one: left hand side of the operator
    // deny_graph: right hand edges "deny list"
    // must_have_graph: right hand edges "must have list
    let edges_iterator = graphs
        .iter()
        .flat_map(|(one, deny_graph, must_have_graph)| {
            one.iter_edge_with_type_and_weight(main.directed)
                .filter(move |(_, _, src_name, _, dst_name, _, edge_type_name, _)| {
                    // If the secondary graph is given
                    // we filter out the edges that were previously added to avoid
                    // introducing duplicates.
                    // TODO: handle None type edge types and avoid duplicating those!
                    if let Some(dg) = deny_graph {
                        return !dg.has_edge_with_type_by_node_names(
                            src_name,
                            dst_name,
                            edge_type_name.as_ref(),
                        );
                    }
                    if let Some(mhg) = must_have_graph {
                        return mhg.has_edge_with_type_by_node_names(
                            src_name,
                            dst_name,
                            edge_type_name.as_ref(),
                        );
                    }
                    true
                })
                .map(|(_, _, src_name, _, dst_name, _, edge_type_name, weight)| {
                    Ok((src_name, dst_name, edge_type_name, weight))
                })
        });

    // Chaining node types in a way that merges the information between
    // two node type sets where one of the two has some unknown node types
    let nodes_iterator = main
        .iter_nodes()
        .map(|(_, node_name, _, node_type_names)| {
            let node_type_names = match node_type_names {
                Some(ntns) => Some(ntns),
                None => other
                    .get_node_id_by_node_name(&node_name)
                    .ok()
                    .and_then(|node_id| other.get_node_type_name_by_node_id(node_id).unwrap()),
            };
            Ok((node_name, node_type_names))
        })
        .chain(
            other
                .iter_nodes()
                .filter_map(|(_, node_name, _, node_type_names)| {
                    match main.has_node_by_node_name(&node_name) {
                        true => None,
                        false => Some(Ok((node_name, node_type_names))),
                    }
                }),
        );

    Graph::from_string_unsorted(
        edges_iterator,
        Some(nodes_iterator),
        main.directed,
        false,
        build_operator_graph_name(main, other, operator),
        true,
        true,
        false,
        true,
        false,
        false,
        false,
        false,
        false,
        main.has_node_types(),
        main.has_edge_types(),
        main.has_weights(),
        might_have_singletons,
        might_have_singletons_with_selfloops,
        might_have_trap_nodes,
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
/// * main: &Graph - The current graph instance.
/// * other: &Graph - The other graph.
/// * operator: String - The operator used.
/// * graphs: Vec<(&Graph, Option<&Graph>, Option<&Graph>)> - Graph list for the operation.
/// TODO: update docstring
fn generic_integer_operator(
    main: &Graph,
    other: &Graph,
    operator: String,
    graphs: Vec<(&Graph, Option<&Graph>, Option<&Graph>)>,
    might_have_singletons: bool,
    might_have_singletons_with_selfloops: bool,
    might_have_trap_nodes: bool,
) -> Result<Graph, String> {
    // one: left hand side of the operator
    // deny_graph: right hand edges "deny list"
    // must_have_graph: right hand edges "must have list
    let edges_iterator = graphs
        .iter()
        .flat_map(|(one, deny_graph, must_have_graph)| {
            one.iter_edge_with_type_and_weight_ids(main.directed)
                .filter(move |(_, src, dst, edge_type, _)| {
                    // If the secondary graph is given
                    // we filter out the edges that were previously added to avoid
                    // introducing duplicates.
                    if let Some(dg) = deny_graph {
                        return !dg.has_edge_with_type_by_node_ids(*src, *dst, *edge_type);
                    }
                    if let Some(mhg) = must_have_graph {
                        return mhg.has_edge_with_type_by_node_ids(*src, *dst, *edge_type);
                    }
                    true
                })
                .map(|(_, src, dst, edge_type, weight)| Ok((src, dst, edge_type, weight)))
        });

    let node_types = match (&main.node_types, &other.node_types) {
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
                NodeTypeVocabulary::from_structs(main_node_types, Some(mnts.vocabulary.clone()))
                    .unwrap()
            }
        }),
        (Some(mnts), _) => Some(mnts.clone()),
        (_, Some(onts)) => Some(onts.clone()),
        _ => None,
    };

    Graph::from_integer_unsorted(
        edges_iterator,
        main.nodes.clone(),
        node_types,
        main.edge_types.as_ref().map(|ets| ets.vocabulary.clone()),
        main.directed,
        build_operator_graph_name(main, other, operator),
        false,
        main.has_edge_types(),
        main.has_weights(),
        false,
        might_have_singletons,
        might_have_singletons_with_selfloops,
        might_have_trap_nodes,
    )
}

impl<'a, 'b> Graph {
    pub fn validate_operator_terms(&self, other: &'b Graph) -> Result<(), String> {
        if self.directed != other.directed {
            return Err(String::from(
                "The graphs must either be both directed or undirected.",
            ));
        }

        if self.has_weights() != other.has_weights() {
            return Err(String::from(
                "Both graphs need to have weights or neither can.",
            ));
        }

        if self.has_node_types() != other.has_node_types() {
            return Err(String::from(
                "Both graphs need to have node types or neither can.",
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
    /// Return true if the graphs are compatible.
    pub(crate) fn is_compatible(&self, other: &Graph) -> Result<bool, String> {
        self.validate_operator_terms(other)?;
        if self.nodes != other.nodes {
            return Ok(false);
        }
        if let (Some(snts), Some(onts)) = (&self.node_types, &other.node_types) {
            if snts.vocabulary != onts.vocabulary {
                return Ok(false);
            }
        }
        if let (Some(sets), Some(oets)) = (&self.edge_types, &other.edge_types) {
            if sets.vocabulary != oets.vocabulary {
                return Ok(false);
            }
        }
        Ok(true)
    }

    pub(crate) fn generic_operator(
        &self,
        other: &Graph,
        operator: String,
        graphs: Vec<(&Graph, Option<&Graph>, Option<&Graph>)>,
        might_have_singletons: bool,
        might_have_singletons_with_selfloops: bool,
        might_have_trap_nodes: bool,
    ) -> Result<Graph, String> {
        match self.is_compatible(other)? {
            true => generic_integer_operator(
                self,
                other,
                operator,
                graphs,
                might_have_singletons,
                might_have_singletons_with_selfloops,
                might_have_trap_nodes,
            ),
            false => generic_string_operator(
                self,
                other,
                operator,
                graphs,
                might_have_singletons,
                might_have_singletons_with_selfloops,
                might_have_trap_nodes,
            ),
        }
    }
}

impl<'a, 'b> ops::BitOr<&'b Graph> for &'a Graph {
    type Output = Result<Graph, String>;
    /// Return graph composed of the two graphs.
    ///
    /// The two graphs must have the same nodes, node types and edge types.
    ///
    /// # Arguments
    ///
    /// * other: Graph - Graph to be summed.
    ///
    fn bitor(self, other: &'b Graph) -> Result<Graph, String> {
        self.generic_operator(
            other,
            "|".to_owned(),
            vec![(self, None, None), (other, Some(self), None)],
            // TODO: it is possible to make the following more precise!
            self.has_singletons() || other.has_singletons(),
            // TODO: it is possible to make the following more precise!
            self.has_singleton_nodes_with_self_loops()
                || other.has_singleton_nodes_with_self_loops(),
            // TODO: it is possible to make the following more precise!
            self.has_trap_nodes() || other.has_trap_nodes(),
        )
    }
}

impl<'a, 'b> ops::BitXor<&'b Graph> for &'a Graph {
    type Output = Result<Graph, String>;
    /// Return graph composed of the two graphs.
    ///
    /// The two graphs must have the same nodes, node types and edge types.
    ///
    /// # Arguments
    ///
    /// * other: Graph - Graph to be summed.
    ///
    fn bitxor(self, other: &'b Graph) -> Result<Graph, String> {
        self.generic_operator(
            self,
            "^".to_owned(),
            vec![(self, Some(other), None), (other, Some(self), None)],
            true,
            // TODO: it is possible to make the following more precise!
            self.has_selfloops() || other.has_selfloops(),
            true,
        )
    }
}

impl<'a, 'b> ops::Sub<&'b Graph> for &'a Graph {
    type Output = Result<Graph, String>;
    /// Return subtraction for graphs objects.
    ///
    /// The two graphs must have the same nodes, node types and edge types.
    ///
    /// # Arguments
    ///
    /// * other: Graph - Graph to be subtracted.
    ///
    fn sub(self, other: &'b Graph) -> Result<Graph, String> {
        self.generic_operator(
            other,
            "-".to_owned(),
            vec![(self, Some(other), None)],
            true,
            self.has_selfloops(),
            true,
        )
    }
}

impl<'a, 'b> ops::BitAnd<&'b Graph> for &'a Graph {
    type Output = Result<Graph, String>;
    /// Return graph obtained from the intersection of the two graph.
    ///
    /// The two graphs must have the same nodes, node types and edge types.
    ///
    /// # Arguments
    ///
    /// * other: Graph - Graph to be subtracted.
    ///
    fn bitand(self, other: &'b Graph) -> Result<Graph, String> {
        self.generic_operator(
            other,
            "&".to_owned(),
            vec![(self, None, Some(other))],
            true,
            self.has_selfloops() && other.has_selfloops(),
            true,
        )
    }
}
