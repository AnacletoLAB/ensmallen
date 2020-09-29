use super::*;
use std::ops;

/// Return graph composed of the two graphs.
///
/// The two graphs must have the same nodes, node types and edge types.
///
/// # Arguments
///
/// * other: Graph - Graph to be summed.
///
fn generic_operator(
    main: &Graph,
    graphs: Vec<(&Graph, Option<&Graph>, Option<&Graph>)>,
) -> Result<Graph, String> {
    let mut unique_edges_tree = GraphDictionary::new();
    // one: left hand side of the operator
    // deny_graph: right hand edges "deny list"
    // must_have_graph: right hand edges "must have list
    graphs.iter().for_each(|(one, deny_graph, must_have_graph)| {
        one.get_edge_quadruples_enumerate()
            .filter(|(_, src, dst, edge_type, _)| {
                // We avoid to insert duplicates.
                if !one.directed && src > dst {
                    return false;
                }
                // If the secondary graph is given
                // we filter out the edges that were previously added to avoid
                // introducing duplicates.
                if let Some(dg) = deny_graph {
                    return dg.get_edge_id(*src, *dst, *edge_type).is_none();
                }
                if let Some(mhg) = must_have_graph {
                    return mhg.get_edge_id(*src, *dst, *edge_type).is_some();
                }
                true
            })
            .for_each(|(_, src, dst, edge_type, weight)| {
                unique_edges_tree.extend(&one, src, dst, edge_type, weight, false)
            })
    });

    Ok(build_graph(
        &mut unique_edges_tree,
        main.nodes.clone(),
        main.node_types.clone(),
        if let Some(et) = &main.edge_types {
            Some(et.vocabulary.clone())
        } else {
            None
        },
        main.directed,
    ))
}

impl<'a, 'b> Graph {
    fn validate_operator_terms(&self, other: &'b Graph) -> Result<(), String> {
        if self.directed != other.directed {
            return Err(String::from(concat!(
                "The graphs must either be both directed or undirected."
            )));
        }

        if self.has_weights() != other.has_weights() {
            return Err(String::from(concat!(
                "Both graphs need to have weights or neither can."
            )));
        }

        if self.has_edge_types() != other.has_edge_types() {
            return Err(String::from(concat!(
                "Both graphs need to have edge types or neither can."
            )));
        }

        if self.has_node_types() != other.has_node_types() {
            return Err(String::from(concat!(
                "Both graphs need to have node types or neither can."
            )));
        }

        if self.nodes != other.nodes {
            return Err(String::from(concat!(
                "The two given graphs do not have ",
                "the same nodes mapping."
            )));
        }

        if let Some(sntm) = &self.node_types {
            if let Some(ontm) = &other.node_types {
                if sntm.vocabulary != ontm.vocabulary {
                    return Err(String::from(concat!(
                        "The two given graphs do not have ",
                        "the same node types mapping."
                    )));
                }
            }
        }

        if let Some(setm) = &self.edge_types {
            if let Some(oetm) = &other.edge_types {
                if setm.vocabulary != oetm.vocabulary {
                    return Err(String::from(concat!(
                        "The two given graphs do not have ",
                        "the same edge types mapping."
                    )));
                }
            }
        }

        Ok(())
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
        self.validate_operator_terms(other)?;
        generic_operator(self, vec![(self, None, None), (other, Some(self), None)])
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
        self.validate_operator_terms(other)?;
        generic_operator(
            self,
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
        self.validate_operator_terms(other)?;
        generic_operator(self, vec![(self, Some(other), None)])
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
        self.validate_operator_terms(other)?;
        generic_operator(self, vec![(self, None, Some(other))])
    }
}
