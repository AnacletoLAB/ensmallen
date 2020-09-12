use super::*;

/// # Operators
impl Graph {
    /// Return sum for summing graphs objects.
    ///
    /// The add is only defined for disjointed graph components.
    /// The two graphs must have the same nodes, node types and edge types.
    ///
    /// # Arguments
    ///
    /// * other: Graph - Graph to be summed.
    ///
    pub fn sum(&self, other: &Graph) -> Result<Graph, String> {
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
                let mut metadata =
                    ConstructorEdgeMetadata::new(self.has_weights(), self.has_edge_types());
                if let Some(md) = &mut metadata {
                    md.set(
                        self.get_link_weights(*src, *dst),
                        self.get_link_edge_types(*src, *dst),
                    );
                }
                unique_edges_tree.insert((*src, *dst), metadata);
            });

        Ok(build_graph(
            unique_edges_tree,
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
