use super::*;
use rayon::prelude::*;
use std::cmp::Ordering;

#[derive(Hash, Clone, Debug, PartialEq)]
pub struct Tendril {
    graph: Graph,
    root_node_id: NodeT,
    len: NodeT,
    node_ids: Option<Vec<NodeT>>,
}

use std::string::ToString;
impl ToString for Tendril {
    fn to_string(&self) -> String {
        match self.len() {
            0 => unreachable!("It does not make sense to have an empty tendril."),
            1 => format!("<p>Tendril containing a single node {}.</p>", unsafe {
                self.graph.get_unchecked_succinct_node_description(
                    self.get_root_node_id(),
                    2,
                    false,
                )
            }),
            number_of_nodes => format!(
                concat!(
                    "<p>Tendril containing {} nodes and starts from the node {}. ",
                    "Specifically, the nodes involved in the Tendril are: {}.</p>",
                ),
                to_human_readable_high_integer(number_of_nodes as usize),
                unsafe {
                    self.graph.get_unchecked_succinct_node_description(
                        self.get_root_node_id(),
                        2,
                        false,
                    )
                },
                unsafe {
                    get_unchecked_formatted_list(
                        &self
                            .get_tendril_node_ids()
                            .into_iter()
                            .map(|node_id| {
                                self.graph
                                    .get_unchecked_succinct_node_description(node_id, 2, false)
                            })
                            .collect::<Vec<String>>(),
                        Some(5),
                    )
                }
            ),
        }
    }
}

impl PartialOrd for Tendril {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.len.cmp(&other.len))
    }
}

impl Tendril {
    /// Return new Tendril object created with the provided root and length.
    ///
    /// # Arguments
    /// * `graph`: &Graph - The graph of reference of the Tendril.
    /// * `root_node_id`: NodeT - First node ID of the Tendril.
    /// * `len`: NodeT - Precomputed length of the Tendril.
    ///
    pub(crate) fn new(graph: &Graph, root_node_id: NodeT, len: NodeT) -> Tendril {
        Tendril {
            graph: graph.clone(),
            root_node_id,
            len,
            node_ids: None,
        }
    }

    pub(crate) fn from_node_ids(graph: &Graph, node_ids: Vec<NodeT>) -> Tendril {
        Tendril {
            graph: graph.clone(),
            root_node_id: node_ids[0],
            len: node_ids.len() as NodeT,
            node_ids: Some(node_ids),
        }
    }

    /// Return the first node ID of the Tendril.
    pub fn get_root_node_id(&self) -> NodeT {
        self.root_node_id
    }

    /// Return the first node name of the Tendril.
    pub fn get_root_node_name(&self) -> String {
        unsafe {
            self.graph
                .get_unchecked_node_name_from_node_id(self.root_node_id)
        }
    }

    /// Return length of the Tendril.
    pub fn len(&self) -> NodeT {
        self.len
    }

    /// Return the node IDs of the nodes composing the Tendril.
    pub fn get_tendril_node_ids(&self) -> Vec<NodeT> {
        if let Some(node_ids) = &self.node_ids {
            node_ids.clone()
        } else {
            unsafe {
                self.graph
                    .get_chain_node_ids_from_root_node_id(self.root_node_id)
            }
        }
    }

    /// Return the node names of the nodes composing the Tendril.
    pub fn par_iter_tendril_node_names(&self) -> impl IndexedParallelIterator<Item = String> + '_ {
        self.get_tendril_node_ids()
            .into_par_iter()
            .map(move |node_id| unsafe { self.graph.get_unchecked_node_name_from_node_id(node_id) })
    }

    /// Return the first `k` node IDs of the nodes composing the Tendril.
    ///
    /// # Arguments
    /// `k`: usize - The number of terms to return.
    pub fn get_first_k_tendril_node_ids(&self, k: usize) -> Vec<NodeT> {
        self.get_tendril_node_ids().into_iter().take(k).collect()
    }

    /// Return the first `k` node names of the nodes composing the Tendril.
    ///
    /// # Arguments
    /// `k`: usize - The number of terms to return.
    pub fn get_first_k_tendril_node_names(&self, k: usize) -> Vec<String> {
        self.par_iter_tendril_node_names().take(k).collect()
    }

    /// Return the node names of the nodes composing the Tendril.
    pub fn get_tendril_node_names(&self) -> Vec<String> {
        self.par_iter_tendril_node_names().collect()
    }
}

impl Graph {
    /// Returns parallel iterator over tendrils of the graph.
    ///
    /// # Arguments
    /// `minimum_number_of_nodes_per_tendril`: Option<NodeT> - Minimum size of the tendrils.
    /// `compute_tendril_nodes`: Option<bool> - Whether to pre-compute the tendril nodes.
    pub fn par_iter_tendrils(
        &self,
        minimum_number_of_nodes_per_tendril: Option<NodeT>,
        compute_tendril_nodes: Option<bool>,
    ) -> Result<impl ParallelIterator<Item = Tendril> + '_> {
        self.must_be_undirected()?;
        let minimum_number_of_nodes_per_tendril = minimum_number_of_nodes_per_tendril.unwrap_or(10);
        let compute_tendril_nodes = compute_tendril_nodes.unwrap_or(false);
        Ok(self
            .par_iter_node_ids()
            // keep only tendrils roots
            .filter(move |&node_id| unsafe {
                let (node_degree, neighbour_node_degree) =
                    self.get_chain_node_degree_with_max_neighbour_id(node_id);
                node_degree == 1 && neighbour_node_degree > 1
            })
            .filter_map(move |node_id| unsafe {
                // compute the nodes in the tendril
                let (tendril_length, last_node, node_ids) = if compute_tendril_nodes {
                    // compute explicitely the tendril
                    let node_ids = self.get_chain_node_ids_from_root_node_id(node_id);
                    // return the info about the tendril
                    (
                        node_ids.len() as NodeT,
                        *node_ids.last().unwrap(),
                        Some(node_ids),
                    )
                } else {
                    // just compute the tendril lenght and last node
                    let (tendril_length, last_node) =
                        self.get_chain_last_id_from_root_node_id(node_id);
                    (tendril_length, last_node, None)
                };

                // if the tendril is shorted than what we want, ignore it
                if tendril_length < minimum_number_of_nodes_per_tendril {
                    return None;
                }
                // only keep the root with the smaller node_id
                if last_node < node_id
                    && self.get_unchecked_node_degree_from_node_id(last_node) == 1
                {
                    return None;
                }
                // return the tendril
                Some(if let Some(node_ids) = node_ids {
                    Tendril::from_node_ids(self, node_ids)
                } else {
                    Tendril::new(self, node_id, tendril_length)
                })
            }))
    }

    /// Return vector of Tendrils in the current graph instance.
    ///
    /// # Arguments
    /// `minimum_number_of_nodes_per_tendril`: Option<NodeT> - Minimum size of the Tendrils.
    /// `compute_tendril_nodes`: Option<bool> - Whether to pre-compute the Tendril nodes.
    ///
    /// # Definitions
    /// In an undirected graph, a tendril is a path that only visit nodes that have
    /// degree equals to 2 or 1, starting from a root node with degree one.
    ///
    pub fn get_tendrils(
        &self,
        minimum_number_of_nodes_per_tendril: Option<NodeT>,
        compute_tendril_nodes: Option<bool>,
    ) -> Result<Vec<Tendril>> {
        Ok(self
            .par_iter_tendrils(minimum_number_of_nodes_per_tendril, compute_tendril_nodes)?
            .collect::<Vec<Tendril>>())
    }
}
