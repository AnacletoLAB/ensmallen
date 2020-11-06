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
///
fn generic_string_operator(
    main: &Graph,
    other: &Graph,
    operator: String,
    graphs: Vec<(&Graph, Option<&Graph>, Option<&Graph>)>,
) -> Result<Graph, String> {
    // one: left hand side of the operator
    // deny_graph: right hand edges "deny list"
    // must_have_graph: right hand edges "must have list
    let edges_iterator = graphs
        .iter()
        .flat_map(|(one, deny_graph, must_have_graph)| {
            one.get_edges_string_quadruples(main.directed)
                .filter(move |(_, src, dst, edge_type, _)| {
                    // If the secondary graph is given
                    // we filter out the edges that were previously added to avoid
                    // introducing duplicates.
                    if let Some(dg) = deny_graph {
                        return !dg.has_edge_string(src, dst, edge_type.as_ref());
                    }
                    if let Some(mhg) = must_have_graph {
                        return mhg.has_edge_string(src, dst, edge_type.as_ref());
                    }
                    true
                })
                .map(|(_, src, dst, edge_type, weight)| Ok((src, dst, edge_type, weight)))
        });

    let graphs = [main, other];
    let nodes_iterator = graphs
        .iter()
        .flat_map(|graph| graph.get_nodes_names_iter().map(|(_, node_name, node_id)| Ok((node_name, node_id))));

    Graph::from_string_unsorted(
        edges_iterator,
        Some(nodes_iterator),
        main.directed,
        false,
        build_operator_graph_name(main, other, operator),
        true,
        false,
        false,
        false,
        false,
        false,
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
///
fn generic_integer_operator(
    main: &Graph,
    other: &Graph,
    operator: String,
    graphs: Vec<(&Graph, Option<&Graph>, Option<&Graph>)>,
) -> Result<Graph, String> {
    // one: left hand side of the operator
    // deny_graph: right hand edges "deny list"
    // must_have_graph: right hand edges "must have list
    let edges_iterator = graphs
        .iter()
        .flat_map(|(one, deny_graph, must_have_graph)| {
            one.get_edges_quadruples(main.directed)
                .filter(move |(_, src, dst, edge_type, _)| {
                    // If the secondary graph is given
                    // we filter out the edges that were previously added to avoid
                    // introducing duplicates.
                    if let Some(dg) = deny_graph {
                        return !dg.has_edge(*src, *dst, *edge_type);
                    }
                    if let Some(mhg) = must_have_graph {
                        return mhg.has_edge(*src, *dst, *edge_type);
                    }
                    true
                })
                .map(|(_, src, dst, edge_type, weight)| Ok((src, dst, edge_type, weight)))
        });

    Graph::from_integer_unsorted(
        edges_iterator,
        main.nodes.clone(),
        main.node_types.clone(),
        match &main.edge_types {
            Some(ets) => Some(ets.vocabulary.clone()),
            None => None,
        },
        main.directed,
        false,
        build_operator_graph_name(main, other, operator),
        false,
        false,
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
        if self.node_types != other.node_types {
            return Ok(false);
        }
        if let Some(sets) = &self.edge_types {
            if let Some(oets) = &other.edge_types {
                if sets.vocabulary != oets.vocabulary {
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }

    pub(crate) fn generic_operator(
        &self,
        other: &Graph,
        operator: String,
        graphs: Vec<(&Graph, Option<&Graph>, Option<&Graph>)>,
    ) -> Result<Graph, String> {
        match self.is_compatible(other)? {
            true => generic_integer_operator(self, other, operator, graphs),
            false => generic_string_operator(self, other, operator, graphs),
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
        self.generic_operator(other, "-".to_owned(), vec![(self, Some(other), None)])
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
        self.generic_operator(other, "&".to_owned(), vec![(self, None, Some(other))])
    }
}
