use super::*;
use std::ops;

/// # Operators
impl<'a, 'b> ops::Add<&'b Graph> for &'a Graph {
    type Output = Result<Graph, String>;
    /// Return sum for summing graphs objects.
    ///
    /// The add is only defined for disjointed graph components.
    /// The two graphs must have the same nodes, node types and edge types.
    ///
    /// # Arguments
    ///
    /// * other: Graph - Graph to be summed.
    ///
    fn add(self, other: &'b Graph) -> Result<Graph, String> {
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

        if self.overlaps(&other)? {
            return Err(String::from(concat!(
                "The two given graphs have overlapping edges, ",
                "this is not supported since it's an undefined ",
                "behaviour."
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

        self.unique_edges
            .keys()
            .chain(other.unique_edges.keys())
            .for_each(|(src, dst)| {
                if unique_edges_tree.contains_key(&(*src, *dst)) {
                    return;
                }
                let mut metadata =
                    ConstructorEdgeMetadata::new(self.has_weights(), self.has_edge_types());
                if let Some(md) = &mut metadata {
                    md.extend(
                        self.get_link_weights(*src, *dst),
                        self.get_link_edge_types(*src, *dst),
                    );
                    md.extend(
                        other.get_link_weights(*src, *dst),
                        other.get_link_edge_types(*src, *dst),
                    );
                }
                unique_edges_tree.insert((*src, *dst), metadata);
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
    /// The subtraction is only defined for disjointed graph components.
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
            .filter(|(src, dst, edge_type, _)| {
                !other.check_edge_overlap(*src, *dst, *edge_type)
            })
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
    /// Return subtraction for graphs objects.
    ///
    /// The subtraction is only defined for disjointed graph components.
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
            .filter(|(src, dst, edge_type, _)| {
                big.check_edge_overlap(*src, *dst, *edge_type)
            })
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
