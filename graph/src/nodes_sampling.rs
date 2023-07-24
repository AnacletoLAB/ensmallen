use super::*;
use itertools::Itertools;
use std::collections::HashSet;
use vec_rand::sorted_unique_sub_sampling;
use vec_rand::{sample_uniform, splitmix64};

/// # Nodes sampling
impl Graph {
    /// Return random node type ID.
    ///
    /// # Arguments
    /// * `random_state`: u64 - The random state to use to reproduce the sampling.
    pub fn get_random_node_type(&self, random_state: u64) -> Result<NodeTypeT> {
        Ok(sample_uniform(
            self.get_number_of_node_types()? as u64,
            splitmix64(random_state),
        ) as NodeTypeT)
    }

    /// Return random edge type ID.
    ///
    /// # Arguments
    /// * `random_state`: u64 - The random state to use to reproduce the sampling.
    pub fn get_random_edge_type(&self, random_state: u64) -> Result<EdgeTypeT> {
        Ok(sample_uniform(
            self.get_number_of_edge_types()? as u64,
            splitmix64(random_state),
        ) as EdgeTypeT)
    }

    /// Return random scale_free edge type ID.
    ///
    /// # Arguments
    /// * `random_state`: u64 - The random state to use to reproduce the sampling.
    ///
    /// # Safety
    /// * If the graph does not have edge types, the method will always return None.
    pub unsafe fn get_unchecked_random_scale_free_edge_type(
        &self,
        random_state: u64,
    ) -> Option<EdgeTypeT> {
        self.get_unchecked_edge_type_id_from_edge_id(self.get_random_edge_id(random_state))
    }

    /// Return random scale_free edge type ID.
    ///
    /// # Arguments
    /// * `random_state`: u64 - The random state to use to reproduce the sampling.
    ///
    /// # Raises
    /// * If the graph does not have edge types.
    pub fn get_random_scale_free_edge_type(&self, random_state: u64) -> Result<Option<EdgeTypeT>> {
        self.must_have_edge_types()
            .map(|_| unsafe { self.get_unchecked_random_scale_free_edge_type(random_state) })
    }

    /// Return random scale free edge weight.
    ///
    /// # Arguments
    /// * `random_state`: u64 - The random state to use to reproduce the sampling.
    ///
    /// # Safety
    /// * If the graph does not have edge types, the method will always return None.
    pub unsafe fn get_unchecked_random_scale_free_edge_weight(
        &self,
        random_state: u64,
    ) -> Option<WeightT> {
        self.get_unchecked_edge_weight_from_edge_id(self.get_random_edge_id(random_state))
    }

    /// Return random scale free edge weight.
    ///
    /// # Arguments
    /// * `random_state`: u64 - The random state to use to reproduce the sampling.
    ///
    /// # Raises
    /// * If the graph does not have edge types.
    pub fn get_random_scale_free_edge_weight(&self, random_state: u64) -> Result<Option<WeightT>> {
        self.must_have_edge_weights()
            .map(|_| unsafe { self.get_unchecked_random_scale_free_edge_weight(random_state) })
    }

    /// Return random node ID.
    ///
    /// # Arguments
    /// * `random_state`: u64 - The random state to use to reproduce the sampling.
    pub fn get_random_node(&self, random_state: u64) -> NodeT {
        sample_uniform(self.get_number_of_nodes() as u64, splitmix64(random_state)) as NodeT
    }

    /// Return random edge ID.
    ///
    /// # Arguments
    /// * `random_state`: u64 - The random state to use to reproduce the sampling.
    pub fn get_random_edge_id(&self, random_state: u64) -> EdgeT {
        sample_uniform(
            self.get_number_of_directed_edges() as u64,
            splitmix64(random_state),
        ) as EdgeT
    }

    /// Return random node ID following outbounds degree distribution of the graph.
    ///
    /// # Arguments
    /// * `random_state`: u64 - The random state to use to reproduce the sampling.
    pub fn get_random_outbounds_scale_free_node(&self, random_state: u64) -> NodeT {
        unsafe {
            self.get_unchecked_node_ids_from_edge_id(self.get_random_edge_id(random_state))
                .0 as NodeT
        }
    }

    /// Return random node ID following inbounds degree distribution of the graph.
    ///
    /// # Arguments
    /// * `random_state`: u64 - The random state to use to reproduce the sampling.
    pub fn get_random_inbounds_scale_free_node(&self, random_state: u64) -> NodeT {
        unsafe {
            self.get_unchecked_node_ids_from_edge_id(self.get_random_edge_id(random_state))
                .1 as NodeT
        }
    }

    /// Return random unique sorted numbers.
    ///
    /// # Arguments
    /// * `number_of_nodes_to_sample`: NodeT - The number of nodes to sample.
    /// * `random_state`: u64 - The random state to use to reproduce the sampling.
    pub fn get_sorted_unique_random_nodes(
        &self,
        number_of_nodes_to_sample: NodeT,
        random_state: u64,
    ) -> Result<Vec<NodeT>> {
        sorted_unique_sub_sampling(
            0,
            self.get_number_of_nodes() as u64,
            number_of_nodes_to_sample as u64,
            random_state,
        )
        .map(|result| result.into_iter().map(|node_id| node_id as NodeT).collect())
    }

    /// Return nodes sampled from the neighbourhood of given root nodes.
    ///
    /// # Arguments
    /// * `number_of_nodes_to_sample`: NodeT - The number of nodes to sample.
    /// * `root_node`: NodeT - The root node from .
    ///
    /// # Raises
    /// * If the number of requested nodes is higher than the number of nodes in the graph.
    /// * If the given root node does not exist in the curret graph instance.
    pub fn get_breadth_first_search_random_nodes(
        &self,
        number_of_nodes_to_sample: NodeT,
        root_node: NodeT,
    ) -> Result<Vec<NodeT>> {
        if number_of_nodes_to_sample > self.get_number_of_nodes() {
            return Err(format!(
                concat!(
                    "The requested number of nodes to sample `{}` is ",
                    "higher than the number of nodes `{}` that exist in the ",
                    "current graph instance."
                ),
                number_of_nodes_to_sample,
                self.get_number_of_nodes()
            ));
        }
        self.validate_node_id(root_node)?;
        let number_of_nodes_to_sample = number_of_nodes_to_sample as usize;
        let mut stack = vec![root_node];
        let mut sampled_nodes = HashSet::with_capacity(number_of_nodes_to_sample);
        sampled_nodes.insert(root_node);
        while let Some(src) = stack.pop() {
            unsafe { self.iter_unchecked_neighbour_node_ids_from_source_node_id(src) }.for_each(
                |dst| {
                    if sampled_nodes.len() == number_of_nodes_to_sample
                        || sampled_nodes.contains(&dst)
                    {
                        return;
                    }
                    sampled_nodes.insert(dst);
                    stack.push(dst);
                },
            );
            if sampled_nodes.len() == number_of_nodes_to_sample {
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
    /// * `unique`: Option<bool> - Whether to make the sampled nodes unique.
    ///
    /// # Raises
    /// * If the given node does not exist in the current slack.
    pub fn get_uniform_random_walk_random_nodes(
        &self,
        node: NodeT,
        random_state: u64,
        walk_length: u64,
        unique: Option<bool>,
    ) -> Result<Vec<NodeT>> {
        self.validate_node_id(node)?;
        let unique = unique.unwrap_or(false);
        Ok(if unique {
            unsafe { self.iter_uniform_walk(node, random_state, walk_length) }
                .unique()
                .collect()
        } else {
            unsafe { self.iter_uniform_walk(node, random_state, walk_length) }.collect()
        })
    }

    /// Return list of the supported node sampling methods.
    pub fn get_node_sampling_methods(&self) -> Vec<&str> {
        vec![
            "random_nodes",
            "breadth_first_search",
            "uniform_random_walk",
        ]
    }

    /// Return subsampled nodes according to the given method and parameters.
    ///
    /// # Arguments
    /// * `number_of_nodes_to_sample`: NodeT - The number of nodes to sample.
    /// * `random_state`: u64 - The random state to reproduce the sampling.
    /// * `root_node`: Option<NodeT> - The (optional) root node to use to sample. In not provided, a random one is sampled.
    /// * `node_sampling_method`: &str - The method to use to sample the nodes. Can either be random nodes, breath first search-based or uniform random walk-based.
    /// * `unique`: Option<bool> - Whether to make the sampled nodes unique.
    ///
    /// # Raises
    /// * If the given node sampling method is not supported.
    pub fn get_subsampled_nodes(
        &self,
        number_of_nodes_to_sample: NodeT,
        random_state: u64,
        node_sampling_method: &str,
        root_node: Option<NodeT>,
        unique: Option<bool>,
    ) -> Result<Vec<NodeT>> {
        let random_state = splitmix64(random_state);
        let root_node =
            root_node.unwrap_or(splitmix64(random_state) as NodeT % self.get_number_of_nodes());
        match node_sampling_method {
            "random_nodes" => self.get_sorted_unique_random_nodes(number_of_nodes_to_sample, random_state),
            "breadth_first_search" => self.get_breadth_first_search_random_nodes(number_of_nodes_to_sample, root_node),
            "uniform_random_walk" => self.get_uniform_random_walk_random_nodes(root_node, random_state, number_of_nodes_to_sample as u64, unique),
            node_sampling_method => Err(format!(
                concat!(
                    "The provided node sampling method {} is not supported. The supported methods are:\n",
                    "{}"
                ),
                node_sampling_method,
                self.get_node_sampling_methods().into_iter().map(|node_sampling_schema| format!("* {}", node_sampling_schema)).join("\n")
            ))
        }
    }
}
