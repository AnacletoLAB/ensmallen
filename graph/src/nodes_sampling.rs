use super::*;
use itertools::Itertools;
use std::collections::HashSet;
use vec_rand::sorted_unique_sub_sampling;

/// # Nodes sampling
impl Graph {
    /// Return random unique sorted numbers.
    ///
    /// # Arguments
    /// * `nodes_to_sample_number`: NodeT - The number of nodes to sample.
    /// * `random_state`: u64 - The random state to use to reproduce the sampling.
    pub fn get_random_nodes(
        &self,
        nodes_to_sample_number: NodeT,
        random_state: u64,
    ) -> Result<Vec<NodeT>> {
        sorted_unique_sub_sampling(
            0,
            self.get_nodes_number() as u64,
            nodes_to_sample_number as u64,
            random_state,
        )
        .map(|result| result.into_iter().map(|node_id| node_id as NodeT).collect())
    }

    /// Return nodes sampled from the neighbourhood of given root nodes.
    ///
    /// # Arguments
    /// * `nodes_to_sample_number`: NodeT - The number of nodes to sample.
    /// * `root_node`: NodeT - The root node from .
    ///
    /// # Raises
    /// * If the number of requested nodes is higher than the number of nodes in the graph.
    /// * If the given root node does not exist in the curret graph instance.
    pub fn get_breath_first_search_random_nodes(
        &self,
        nodes_to_sample_number: NodeT,
        root_node: NodeT,
    ) -> Result<Vec<NodeT>> {
        if nodes_to_sample_number > self.get_nodes_number() {
            return Err(format!(
                concat!(
                    "The requested number of nodes to sample `{}` is ",
                    "higher than the number of nodes `{}` that exist in the ",
                    "current graph instance."
                ),
                nodes_to_sample_number,
                self.get_nodes_number()
            ));
        }
        self.validate_node_id(root_node)?;
        let nodes_to_sample_number = nodes_to_sample_number as usize;
        let mut stack = vec![root_node];
        let mut sampled_nodes = HashSet::new();
        sampled_nodes.insert(root_node);
        while let Some(src) = stack.pop() {
            unsafe { self.iter_unchecked_neighbour_node_ids_from_source_node_id(src) }.for_each(
                |dst| {
                    if sampled_nodes.contains(&dst) {
                        return;
                    }
                    sampled_nodes.insert(dst);
                    if sampled_nodes.len() == nodes_to_sample_number {
                        return;
                    }
                    stack.push(dst);
                },
            );
            if sampled_nodes.len() == nodes_to_sample_number {
                break;
            }
        }
        Ok(sampled_nodes.into_iter().collect())
    }

    /// Returns unique nodes sampled from uniform random walk.
    ///
    /// # Arguments
    /// * `node`: NodeT - Node from where to start the random walks.
    /// * `random_state`: usize - the random_state to use for extracting the nodes and edges.
    /// * `walk_length`: u64 - Length of the random walk.
    ///
    /// # Raises
    /// * If the given node does not exist in the current slack.
    pub fn get_uniform_random_walk_random_nodes(
        &self,
        node: NodeT,
        random_state: u64,
        walk_length: u64,
    ) -> Result<Vec<NodeT>> {
        self.validate_node_id(node)?;
        Ok(
            unsafe { self.iter_uniform_walk(node, random_state, walk_length) }
                .unique()
                .collect(),
        )
    }

    /// Return subsampled nodes according to the given method and parameters.
    ///
    /// # Arguments
    /// * `nodes_to_sample_number`: NodeT - The number of nodes to sample.
    /// * `random_state`: u64 - The random state to reproduce the sampling.
    /// * `root_node`: Option<NodeT> - The (optional) root node to use to sample. In not provided, a random one is sampled.
    /// * `node_sampling_method`: &str - The method to use to sample the nodes. Can either be random nodes, breath first search-based or uniform random walk-based.
    ///
    /// # Raises
    /// * If the given node sampling method is not supported.
    pub fn get_subsampled_nodes(
        &self,
        nodes_to_sample_number: NodeT,
        random_state: u64,
        root_node: Option<NodeT>,
        node_sampling_method: &str,
    ) -> Result<Vec<NodeT>> {
        let random_state = splitmix64(random_state);
        let root_node =
            root_node.unwrap_or(splitmix64(random_state) as NodeT % self.get_nodes_number());
        match node_sampling_method {
            "random_nodes" => self.get_random_nodes(nodes_to_sample_number, random_state),
            "breath_first_search" => self.get_breath_first_search_random_nodes(nodes_to_sample_number, root_node),
            "uniform_random_walk" => self.get_uniform_random_walk_random_nodes(root_node, random_state, nodes_to_sample_number as u64),
            node_sampling_method => Err(format!(
                concat!(
                    "The provided node sampling method {} is not supported. The supported methods are:\n",
                    "* random_nodes\n",
                    "* breath_first_search\n",
                    "* uniform_random_walk\n"
                ),
                node_sampling_method
            ))
        }
    }
}
