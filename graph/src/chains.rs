use super::*;
use indicatif::ParallelProgressIterator;
use rayon::prelude::*;

#[derive(Hash, Clone, Debug)]
pub struct Chain {
    graph: Graph,
    root_node_id: NodeT,
    len: NodeT,
    node_ids: Option<Vec<NodeT>>,
}

use std::string::ToString;
impl ToString for Chain {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl Chain {
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
    pub fn get_chain_node_names(&self) -> Vec<String> {
        self.get_chain_node_ids()
            .into_par_iter()
            .map(|node_id| unsafe { self.graph.get_unchecked_node_name_from_node_id(node_id) })
            .collect()
    }
}

impl Graph {
    /// Return node IDs in the chain starting from the provided node ID.
    ///
    /// # Arguments
    /// `node_id`: NodeT - The root of the provided chain.
    ///
    /// # Safety
    /// The node ID must be among the node IDs present in the graph, or the method will panic.
    /// Additionally, it must be the root node of a chain.
    unsafe fn get_chain_node_ids_from_root_node_id(&self, mut node_id: NodeT) -> Vec<NodeT> {
        let mut chain_node_ids: Vec<NodeT> = vec![node_id];
        let mut previous_node_id = node_id;
        'outer: loop {
            for neighbour_node_id in
                self.iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
            {
                if neighbour_node_id != node_id
                    && neighbour_node_id != previous_node_id
                    && self.get_unchecked_node_degree_from_node_id(neighbour_node_id) <= 2
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

    /// Return vector of chains in the current graph instance.
    ///
    /// # Arguments
    /// `minimum_number_of_nodes_per_chain`: Option<NodeT> - Minimum size of the chains.
    /// `compute_chain_nodes`: Option<bool> - Whether to pre-compute the chain nodes.
    /// `verbose`: Option<bool> - Whether to show the loading bars.
    pub fn get_chains(
        &self,
        minimum_number_of_nodes_per_chain: Option<NodeT>,
        compute_chain_nodes: Option<bool>,
        verbose: Option<bool>,
    ) -> Result<Vec<Chain>> {
        self.must_be_undirected()?;
        let minimum_number_of_nodes_per_chain = minimum_number_of_nodes_per_chain.unwrap_or(10);
        let verbose = verbose.unwrap_or(true);
        let compute_chain_nodes = compute_chain_nodes.unwrap_or(false);
        let progress_bar = get_loading_bar(
            verbose,
            "Detecting nodes inside chains",
            self.get_nodes_number() as usize,
        );
        Ok(self
            .par_iter_node_ids()
            .progress_with(progress_bar)
            .filter(|&node_id| unsafe {
                let node_degree = self.get_unchecked_node_degree_from_node_id(node_id);
                node_degree == 2
                    && self
                        .iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                        .any(|node_id| self.get_unchecked_node_degree_from_node_id(node_id) > 2)
                    || node_degree == 1
                        && self
                            .iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                            .all(|node_id| {
                                self.get_unchecked_node_degree_from_node_id(node_id) == 2
                            })
            })
            .filter_map(|node_id| unsafe {
                let node_ids = self.get_chain_node_ids_from_root_node_id(node_id);
                if node_ids.len() as NodeT >= minimum_number_of_nodes_per_chain
                    && *node_ids.last().unwrap() > node_id
                {
                    Some(if compute_chain_nodes {
                        Chain::from_node_ids(self, node_ids)
                    } else {
                        Chain::new(self, node_id, node_ids.len() as NodeT)
                    })
                } else {
                    None
                }
            })
            .collect::<Vec<Chain>>())
    }
}
