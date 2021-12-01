use super::*;
use rayon::prelude::*;
use std::cmp::Ordering;

#[derive(Hash, Clone, Debug, PartialEq)]
pub struct NodeTuple {
    graph: Graph,
    root_node_id: NodeT,
}

use std::string::ToString;
impl ToString for NodeTuple {
    fn to_string(&self) -> String {
        format!(
            concat!(
                "<p>Node tuple containing the nodes {}.</p>",
            ),
            unsafe {
                get_unchecked_formatted_list(
                    &self
                        .get_node_ids()
                        .into_iter()
                        .map(|node_id| {
                            self.graph
                                .get_unchecked_succinct_node_description(node_id, 1, true)
                        })
                        .collect::<Vec<String>>(),
                    Some(5),
                )
            }
        )
    }
}

impl PartialOrd for NodeTuple {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.root_node_id.cmp(&other.root_node_id))
    }
}

impl NodeTuple {
    /// Return new tuple object created with the provided root and length.
    ///
    /// # Arguments
    /// * `graph`: &Graph - The graph of reference of the tuple.
    /// * `root_node_id`: NodeT - First node ID of the tuple.
    ///
    pub(crate) fn new(graph: &Graph, root_node_id: NodeT) -> NodeTuple {
        NodeTuple {
            graph: graph.clone(),
            root_node_id,
        }
    }

    /// Return the first node ID of the tuple.
    pub fn get_root_node_id(&self) -> NodeT {
        self.root_node_id
    }

    /// Return the first node name of the tuple.
    pub fn get_root_node_name(&self) -> String {
        unsafe {
            self.graph
                .get_unchecked_node_name_from_node_id(self.root_node_id)
        }
    }

    /// Return length of the tuple.
    pub fn len(&self) -> NodeT {
        2
    }

    /// Return the node IDs of the nodes composing the tuple.
    pub fn get_node_ids(&self) -> Vec<NodeT> {
        unsafe {
            self.graph
                .get_chain_node_ids_from_root_node_id(self.root_node_id)
        }
    }

    /// Return the node names of the nodes composing the tuple.
    pub fn par_iter_tuple_node_names(&self) -> impl IndexedParallelIterator<Item = String> + '_ {
        self.get_node_ids()
            .into_par_iter()
            .map(move |node_id| unsafe { self.graph.get_unchecked_node_name_from_node_id(node_id) })
    }
}

impl Graph {
    /// Returns parallel iterator over node tuples of the graph.
    pub fn par_iter_tuples(&self) -> Result<impl ParallelIterator<Item = NodeTuple> + '_> {
        self.must_be_undirected()?;
        Ok(self
            .par_iter_node_ids()
            // keep only tuples roots
            .filter(move |&node_id| unsafe {
                let (node_degree, neighbour_node_degree) =
                    self.get_chain_node_degree_with_max_neighbour_id(node_id);
                node_degree == 1 && neighbour_node_degree == 1
            })
            .filter_map(move |node_id| unsafe {
                let last_node = *self
                    .iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                    .filter(|&other_node_id| other_node_id != node_id)
                    .collect::<Vec<NodeT>>()
                    .first()
                    .unwrap();
                // only keep the root with the smaller node_id
                if last_node < node_id {
                    return None;
                }
                // return the tuple
                Some(NodeTuple::new(self, node_id))
            }))
    }

    /// Return vector of node tuples in the current graph instance.
    pub fn get_node_tuples(&self) -> Result<Vec<NodeTuple>> {
        Ok(self.par_iter_tuples()?.collect::<Vec<NodeTuple>>())
    }
}
