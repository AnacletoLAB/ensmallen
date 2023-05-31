use super::*;
use rayon::prelude::*;
use std::cmp::Ordering;

#[derive(Hash, Clone, Debug, PartialEq)]
pub struct Chain {
    graph: Graph,
    root_node_id: NodeT,
    len: NodeT,
    node_ids: Option<Vec<NodeT>>,
}

use std::string::ToString;
impl ToString for Chain {
    fn to_string(&self) -> String {
        let node_ids = if self.graph.has_node_types() || self.graph.has_edge_types() {
            Some(self.get_chain_node_ids())
        } else {
            None
        };
        let show_node_type = if self.graph.has_node_types() {
            node_ids.as_ref().map_or(false, |node_ids| unsafe {
                !self
                    .graph
                    .has_unchecked_isomorphic_node_types_from_node_ids(node_ids)
            })
        } else {
            false
        };
        format!(
            concat!(
                "<p>",
                "Chain containing {number_of_nodes} nodes and starts from the node {root_node}. ",
                "Specifically, the nodes involved in the chain are: {chain_nodes}.",
                "{node_types_counts}",
                "{edge_types_counts}",
                "</p>",
            ),
            number_of_nodes = to_human_readable_high_integer(self.len() as usize),
            root_node = unsafe {
                self.graph.get_unchecked_succinct_node_description(
                    self.get_root_node_id(),
                    2,
                    show_node_type,
                )
            },
            chain_nodes = unsafe {
                get_unchecked_formatted_list(
                    &self
                        .get_chain_node_ids()
                        .into_iter()
                        .skip(1)
                        .map(|node_id| {
                            self.graph.get_unchecked_succinct_node_description(
                                node_id,
                                2,
                                show_node_type,
                            )
                        })
                        .collect::<Vec<String>>(),
                    Some(5),
                )
            },
            node_types_counts =
                if let Some(node_ids) = &node_ids {
                    if self.len() > 5 {
                        unsafe {
                            self.graph
                                .get_unchecked_node_type_id_counts_hashmap_from_node_ids(
                                    node_ids.as_ref(),
                                )
                                .map_or_else(
                                    |_| "".to_string(),
                                    |count| {
                                        if count.is_empty() {
                                            "".to_string()
                                        } else {
                                            format!(
                                        " Its nodes have {}.",
                                        self.graph
                                            .get_unchecked_node_types_description_from_count(count)
                                    )
                                        }
                                    },
                                )
                        }
                    } else {
                        "".to_string()
                    }
                } else {
                    "".to_string()
                },
            edge_types_counts = if let Some(node_ids) = &node_ids {
                unsafe {
                    self.graph
                        .get_unchecked_edge_type_id_counts_hashmap_from_node_ids(node_ids.as_ref())
                        .map_or_else(
                            |_| "".to_string(),
                            |count| {
                                if count.is_empty() {
                                    "".to_string()
                                } else {
                                    format!(
                                        " Its edges have {}.",
                                        self.graph
                                            .get_unchecked_edge_types_description_from_count(count)
                                    )
                                }
                            },
                        )
                }
            } else {
                "".to_string()
            }
        )
    }
}

impl PartialOrd for Chain {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.len.cmp(&other.len))
    }
}

impl Chain {
    /// Return new chain object created with the provided root and length.
    ///
    /// # Arguments
    /// * `graph`: &Graph - The graph of reference of the chain.
    /// * `root_node_id`: NodeT - First node ID of the chain.
    /// * `len`: NodeT - Precomputed length of the chain.
    ///
    pub(crate) fn new(graph: &Graph, root_node_id: NodeT, len: NodeT) -> Chain {
        Chain {
            graph: graph.clone(),
            root_node_id,
            len,
            node_ids: None,
        }
    }

    pub(crate) fn from_node_ids(graph: &Graph, node_ids: Vec<NodeT>) -> Chain {
        Chain {
            graph: graph.clone(),
            root_node_id: node_ids[0],
            len: node_ids.len() as NodeT,
            node_ids: Some(node_ids),
        }
    }

    /// Return the first node ID of the chain.
    pub fn get_root_node_id(&self) -> NodeT {
        self.root_node_id
    }

    /// Return the first node name of the chain.
    pub fn get_root_node_name(&self) -> String {
        unsafe {
            self.graph
                .get_unchecked_node_name_from_node_id(self.root_node_id)
        }
    }

    /// Return length of the chain.
    pub fn len(&self) -> NodeT {
        self.len
    }

    /// Return the node IDs of the nodes composing the chain.
    pub fn get_chain_node_ids(&self) -> Vec<NodeT> {
        if let Some(node_ids) = &self.node_ids {
            node_ids.clone()
        } else {
            unsafe {
                self.graph
                    .get_chain_node_ids_from_root_node_id(self.root_node_id)
            }
        }
    }

    /// Return the node names of the nodes composing the chain.
    pub fn par_iter_chain_node_names(&self) -> impl IndexedParallelIterator<Item = String> + '_ {
        self.get_chain_node_ids()
            .into_par_iter()
            .map(move |node_id| unsafe { self.graph.get_unchecked_node_name_from_node_id(node_id) })
    }

    /// Return the first `k` node IDs of the nodes composing the chain.
    ///
    /// # Arguments
    /// `k`: usize - The number of terms to return.
    pub fn get_first_k_chain_node_ids(&self, k: usize) -> Vec<NodeT> {
        self.get_chain_node_ids().into_iter().take(k).collect()
    }

    /// Return the first `k` node names of the nodes composing the chain.
    ///
    /// # Arguments
    /// `k`: usize - The number of terms to return.
    pub fn get_first_k_chain_node_names(&self, k: usize) -> Vec<String> {
        self.par_iter_chain_node_names().take(k).collect()
    }

    /// Return the node names of the nodes composing the chain.
    pub fn get_chain_node_names(&self) -> Vec<String> {
        self.par_iter_chain_node_names().collect()
    }
}

impl Graph {
    /// Return the length of the chain and the last node in the chain
    ///
    /// # Arguments
    /// `node_id`: NodeT - The root of the provided chain.
    ///
    /// # Safety
    /// The node ID must be among the node IDs present in the graph, or the method will panic.
    /// Additionally, it must be the root node of a chain.
    pub(crate) unsafe fn get_chain_last_id_from_root_node_id(
        &self,
        mut node_id: NodeT,
    ) -> (NodeT, NodeT) {
        let mut chain_length = 1;
        let mut previous_node_id = node_id;
        'outer: loop {
            for neighbour_node_id in
                self.iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
            {
                if neighbour_node_id != node_id
                    && neighbour_node_id != previous_node_id
                    && self.get_chain_node_degree(neighbour_node_id) <= 2
                {
                    previous_node_id = node_id;
                    node_id = neighbour_node_id;
                    chain_length += 1;
                    continue 'outer;
                }
            }
            break 'outer;
        }
        (chain_length, node_id)
    }

    /// Return the ids of the nodes in the chain with root `node_id`.
    ///
    /// # Arguments
    /// `node_id`: NodeT - The root of the provided chain.
    ///
    /// # Safety
    /// The node ID must be among the node IDs present in the graph, or the method will panic.
    /// Additionally, it must be the root node of a chain.
    pub(crate) unsafe fn get_chain_node_ids_from_root_node_id(
        &self,
        mut node_id: NodeT,
    ) -> Vec<NodeT> {
        let mut chain_node_ids: Vec<NodeT> = vec![node_id];
        let mut previous_node_id = node_id;
        'outer: loop {
            for neighbour_node_id in
                self.iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
            {
                if neighbour_node_id != node_id
                    && neighbour_node_id != previous_node_id
                    && self.get_chain_node_degree(neighbour_node_id) <= 2
                {
                    previous_node_id = node_id;
                    node_id = neighbour_node_id;
                    chain_node_ids.push(node_id);
                    continue 'outer;
                }
            }
            break;
        }
        chain_node_ids
    }

    /// Get the "degree" of a node, as defined for the chains.
    /// In particular we ignore selfloops and consider the unique destinations
    /// to be able to support multi-graph chains.
    ///
    /// Since we only care for the cases where it's equal to 1 or 2,
    /// if it's bigger than 2 we will always return 3.
    pub(crate) unsafe fn get_chain_node_degree(&self, node_id: NodeT) -> NodeT {
        let mut node_degree = 0;
        let mut previous_node_id = node_id;

        for neighbour_node_id in self.iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
        {
            // ignore selfloops and destinations already visited
            if neighbour_node_id == node_id || neighbour_node_id == previous_node_id {
                continue;
            }
            node_degree += 1;
            // early stop
            if node_degree > 2 {
                return 3;
            }
            previous_node_id = node_id;
        }

        node_degree
    }

    /// The same as `get_chain_node_degree` but we also return the max "chain degree" of the neighbours
    /// of the current node
    pub(crate) unsafe fn get_chain_node_degree_with_max_neighbour_id(
        &self,
        node_id: NodeT,
    ) -> (NodeT, NodeT) {
        let mut node_degree = 0;
        let mut max_neighbour_degree = 0;
        let mut previous_node_id = node_id;
        for neighbour_node_id in self.iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
        {
            // ignore selfloops and destinations already visited
            if neighbour_node_id == node_id || neighbour_node_id == previous_node_id {
                continue;
            }
            node_degree += 1;
            if node_degree > 2 {
                return (3, 3);
            }
            max_neighbour_degree =
                max_neighbour_degree.max(self.get_chain_node_degree(neighbour_node_id));
            previous_node_id = node_id;
        }

        (node_degree, max_neighbour_degree)
    }

    /// Returns parallel iterator over chains of the graph.
    ///
    /// # Arguments
    /// `minimum_number_of_nodes_per_chain`: Option<NodeT> - Minimum size of the chains.
    /// `compute_chain_nodes`: Option<bool> - Whether to pre-compute the chain nodes.
    pub fn par_iter_chains(
        &self,
        minimum_number_of_nodes_per_chain: Option<NodeT>,
        compute_chain_nodes: Option<bool>,
    ) -> Result<impl ParallelIterator<Item = Chain> + '_> {
        self.must_be_undirected()?;
        let minimum_number_of_nodes_per_chain = minimum_number_of_nodes_per_chain.unwrap_or(10);
        let compute_chain_nodes = compute_chain_nodes.unwrap_or(false);
        Ok(self
            .par_iter_node_ids()
            // keep only chains roots
            .filter(move |&node_id| unsafe {
                let (node_degree, max_neighbour_degree) =
                    self.get_chain_node_degree_with_max_neighbour_id(node_id);

                // brenchless filter, here we just apply the definition
                // of chain root
                node_degree == 2 && max_neighbour_degree > 2
            })
            .filter_map(move |node_id| unsafe {
                // compute the nodes in the chain
                let (chain_length, last_node, node_ids) = if compute_chain_nodes {
                    // compute explicitely the chain
                    let node_ids = self.get_chain_node_ids_from_root_node_id(node_id);
                    // return the info about the chain
                    (
                        node_ids.len() as NodeT,
                        *node_ids.last().unwrap(),
                        Some(node_ids),
                    )
                } else {
                    // just compute the chain lenght and last node
                    let (chain_length, last_node) =
                        self.get_chain_last_id_from_root_node_id(node_id);
                    (chain_length, last_node, None)
                };

                // if the chain is shorted than what we want, ignore it
                if chain_length < minimum_number_of_nodes_per_chain {
                    return None;
                }
                // only keep the root with the smaller node_id
                if last_node < node_id {
                    return None;
                }
                // We check that this chain is not a tendril.
                if self.get_unchecked_node_degree_from_node_id(last_node) == 1 {
                    return None;
                }
                // return the chain
                Some(if let Some(node_ids) = node_ids {
                    Chain::from_node_ids(self, node_ids)
                } else {
                    Chain::new(self, node_id, chain_length)
                })
            }))
    }

    /// Return vector of chains in the current graph instance.
    ///
    /// # Arguments
    /// `minimum_number_of_nodes_per_chain`: Option<NodeT> - Minimum size of the chains.
    /// `compute_chain_nodes`: Option<bool> - Whether to pre-compute the chain nodes.
    ///
    /// # Definitions
    /// In an undirected graph, a chain is a path that only visit nodes that have
    /// degree equals to 2 or 1.
    ///
    /// In a chain the root nodes are defined as the nodes
    /// with either degree 1 and a neighbour with degree 2, or a node with degree 2
    /// and a neighbour with degree strictly higher than 2.
    ///
    /// Of the two roots, we always return the one with lower node id.
    ///
    /// In this section we will always consider degree as the number of unique destinations
    /// at distance 1 from the given node. This allows for multi-graph chains and ignores
    /// self-loops.
    ///
    ///
    /// Example: O are ignored nodes, C and R are nodes in the chain, and R are the root nodes
    /// ```ignore
    /// O - O - R - C = C - R
    /// | \ |
    /// O - O
    /// ```
    ///
    /// By definition we ignore the following case:
    /// ```ignore
    /// O - O - X
    /// | \ |
    /// O - O
    /// ```
    /// Here `X` is NOT part of a chain.
    ///
    /// By definition `X` is not a chain root because we do not count selfloops in the degree:
    /// ```ignore
    /// O - O - X )
    /// | \ |
    /// O - O
    /// ```
    ///
    /// Also this is not a chain:
    /// ```ignore
    /// X - X
    /// ```
    ///
    pub fn get_chains(
        &self,
        minimum_number_of_nodes_per_chain: Option<NodeT>,
        compute_chain_nodes: Option<bool>,
    ) -> Result<Vec<Chain>> {
        Ok(self
            .par_iter_chains(minimum_number_of_nodes_per_chain, compute_chain_nodes)?
            .collect())
    }
}
