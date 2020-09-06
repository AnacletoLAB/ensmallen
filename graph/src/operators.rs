use super::Graph;

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
        if self.overlaps(&other)? {
            return Err(String::from(concat!(
                "The two given graphs have overlapping edges, ",
                "this is not supported since it's an undefined ",
                "behaviour."
            )));
        }

        if !other.nodes_mapping.iter().all(|(node_name, node_id)| {
            if let Some(nid) = self.nodes_mapping.get(node_name) {
                *nid == *node_id
            } else {
                false
            }
        }) {
            return Err(String::from(concat!(
                "The two given graphs do not have ",
                "the same nodes mapping."
            )));
        }

        if let Some(sntm) = &self.node_types_mapping {
            if let Some(ontm) = &other.node_types_mapping {
                if !ontm.iter().all(|(node_type_name, node_type_id)| {
                    if let Some(ntid) = sntm.get(node_type_name) {
                        *ntid == *node_type_id
                    } else {
                        false
                    }
                }) {
                    return Err(String::from(concat!(
                        "The two given graphs do not have ",
                        "the same node types mapping."
                    )));
                }
            }
        }

        if let Some(setm) = &self.edge_types_mapping {
            if let Some(oetm) = &other.edge_types_mapping {
                if !oetm.iter().all(|(edge_type_name, edge_type_id)| {
                    if let Some(etid) = setm.get(edge_type_name) {
                        *etid == *edge_type_id
                    } else {
                        false
                    }
                }) {
                    return Err(String::from(concat!(
                        "The two given graphs do not have ",
                        "the same edge types mapping."
                    )));
                }
            }
        }

        let mut sources = self.sources.clone();
        sources.extend(other.sources.clone());

        let mut destinations = self.destinations.clone();
        destinations.extend(other.destinations.clone());

        let weights = if let Some(sw) = &self.weights {
            if let Some(ow) = &other.weights {
                let mut w = sw.clone();
                w.extend(ow.clone());
                Some(w)
            } else {
                None
            }
        } else {
            None
        };

        let edge_types = if let Some(set) = &self.edge_types {
            if let Some(oet) = &other.edge_types {
                let mut et = set.clone();
                et.extend(oet.clone());
                Some(et)
            } else {
                None
            }
        } else {
            None
        };

        self.setup_graph(sources, destinations, edge_types, weights, Some(true))
    }
}
