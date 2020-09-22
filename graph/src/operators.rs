use super::*;
use std::ops;

/// # Operators
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
        if self.is_directed != other.is_directed {
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

        let mut unique_edges_tree = GraphDictionary::new();

        [(self, None), (other, Some(self))]
            .iter()
            .for_each(|(one, two)| {
                (0..one.get_edges_number())
                    .map(|edge| {
                        let src = one.sources[edge];
                        let dst = one.destinations[edge];

                        let edge_type = if let Some(et) = &one.edge_types {
                            Some(et.ids[edge])
                        } else {
                            None
                        };

                        let weight = if let Some(w) = &one.weights {
                            Some(w[edge])
                        } else {
                            None
                        };

                        (src, dst, edge_type, weight)
                    })
                    .filter(|(src, dst, edge_type, _)| {
                        // If the secondary graph is given (this is the second iteration)
                        // we filter out the edges that were previously added to avoid
                        // introducing duplicates.
                        if let Some(t) = two {
                            return !t.check_edge_overlap(*src, *dst, *edge_type);
                        }
                        true
                    })
                    .for_each(|(src, dst, edge_type, weight)| {
                        one.extend_tree(&mut unique_edges_tree, src, dst, edge_type, weight, false)
                    })
            });

        Ok(build_graph(
            &mut unique_edges_tree,
            self.nodes.clone(),
            self.node_types.clone(),
            if let Some(et) = &self.edge_types {
                Some(et.vocabulary.clone())
            } else {
                None
            },
            self.is_directed,
        ))
    }
}

/// # Operators
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
        if self.is_directed != other.is_directed {
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

        let mut unique_edges_tree = GraphDictionary::new();

        (0..self.get_edges_number())
            .map(|edge| {
                let src = self.sources[edge];
                let dst = self.destinations[edge];

                let edge_type = if let Some(et) = &self.edge_types {
                    Some(et.ids[edge])
                } else {
                    None
                };

                let weight = if let Some(w) = &self.weights {
                    Some(w[edge])
                } else {
                    None
                };

                (src, dst, edge_type, weight)
            })
            .filter(|(src, dst, edge_type, _)| !other.check_edge_overlap(*src, *dst, *edge_type))
            .for_each(|(src, dst, edge_type, weight)| {
                self.extend_tree(&mut unique_edges_tree, src, dst, edge_type, weight, false)
            });

        Ok(build_graph(
            &mut unique_edges_tree,
            self.nodes.clone(),
            self.node_types.clone(),
            if let Some(et) = &self.edge_types {
                Some(et.vocabulary.clone())
            } else {
                None
            },
            self.is_directed,
        ))
    }
}

/// # Operators
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
        if self.is_directed != other.is_directed {
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

        let mut unique_edges_tree = GraphDictionary::new();

        let (small, big) = if self.get_edges_number() > other.get_edges_number() {
            (other, self)
        } else {
            (self, other)
        };

        (0..small.get_edges_number())
            .map(|edge| {
                let src = small.sources[edge];
                let dst = small.destinations[edge];

                let edge_type = if let Some(et) = &small.edge_types {
                    Some(et.ids[edge])
                } else {
                    None
                };

                let weight = if let Some(w) = &small.weights {
                    Some(w[edge])
                } else {
                    None
                };

                (src, dst, edge_type, weight)
            })
            .filter(|(src, dst, edge_type, _)| big.check_edge_overlap(*src, *dst, *edge_type))
            .for_each(|(src, dst, edge_type, weight)| {
                small.extend_tree(&mut unique_edges_tree, src, dst, edge_type, weight, false)
            });

        Ok(build_graph(
            &mut unique_edges_tree,
            self.nodes.clone(),
            self.node_types.clone(),
            if let Some(et) = &self.edge_types {
                Some(et.vocabulary.clone())
            } else {
                None
            },
            self.is_directed,
        ))
    }
}
