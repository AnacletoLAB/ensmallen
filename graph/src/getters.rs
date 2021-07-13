use super::*;
use counter::Counter;
use log::info;
use rayon::prelude::*;
use std::collections::HashMap;

/// # Getters
/// The naming convention we follow is:
/// * `/get_(.+)/`
///
/// The naming convention for unchecked methods follows:
/// * `/get_(.+)_unchecked/`
impl Graph {
    /// Returns number a triple with (number of components, number of nodes of the smallest component, number of nodes of the biggest component )
    ///
    /// # Arguments
    ///
    /// * `verbose`: Option<bool> - Whether to show a loading bar or not.
    pub fn get_connected_components_number(&self, verbose: Option<bool>) -> (NodeT, NodeT, NodeT) {
        info!("Computing connected components number.");
        if self.directed {
            let (_, _, components_number, min_component_size, max_component_size) =
                self.spanning_arborescence_kruskal(verbose);
            (components_number, min_component_size, max_component_size)
        } else {
            info!("Executing undirected parallel version of connected components.");
            let (_, components_number, min_component_size, max_component_size) =
                self.connected_components(verbose).unwrap();
            (components_number, min_component_size, max_component_size)
        }
    }

    /// Returns number of singleton nodes within the graph.
    ///
    /// # Example
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// println!("The graph contains {} singleton nodes", graph.get_singleton_nodes_number());
    /// ```
    pub fn get_singleton_nodes_number(&self) -> NodeT {
        self.get_nodes_number()
            - self.get_connected_nodes_number()
            - self.get_singleton_nodes_with_selfloops_number()
    }

    /// Return number of weighted singleton nodes within the graph.
    ///
    /// This number represents the number of nodes that have weighted node
    /// degree equal to 0, which may happen when the graph contains edges
    /// with negative edge weights.
    ///
    /// # Raises
    /// * If the graph does not contain edge weights.
    pub fn get_weighted_singleton_nodes_number(&self) -> Result<NodeT> {
        self.must_have_edge_weights()?;
        Ok(self.weighted_singleton_nodes_number.unwrap())
    }

    /// Returns number of disconnected nodes within the graph.
    /// A Disconnected node is a node which is nor a singleton nor a singleton
    /// with selfloops.
    ///
    /// # Example
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// println!("The graph contains {} disconnected nodes", graph.get_disconnected_nodes_number());
    /// ```
    pub fn get_disconnected_nodes_number(&self) -> NodeT {
        self.get_nodes_number() - self.get_connected_nodes_number()
    }

    /// Returns vector of singleton node IDs of the graph.
    ///
    /// # Example
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// println!("The graph singleton node IDs are {:?}.", graph.get_singleton_node_ids());
    /// ```
    pub fn get_singleton_node_ids(&self) -> Vec<NodeT> {
        self.iter_singleton_node_ids().collect()
    }

    /// Returns vector of singleton node names of the graph.
    ///
    /// # Example
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// println!("The graph singleton node names are {:?}.", graph.get_singleton_node_names());
    /// ```
    pub fn get_singleton_node_names(&self) -> Vec<String> {
        self.iter_singleton_node_names().collect()
    }

    /// Returns number of singleton nodes with self-loops within the graph.
    ///
    /// # Example
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// println!("The graph contains {} singleton nodes with self-loops", graph.get_singleton_nodes_with_selfloops_number());
    /// ```
    pub fn get_singleton_nodes_with_selfloops_number(&self) -> NodeT {
        self.singleton_nodes_with_selfloops_number
    }

    /// Returns vector of singleton_with_selfloops node IDs of the graph.
    ///
    /// # Example
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// println!("The graph singleton_with_selfloops node IDs are {:?}.", graph.get_singleton_with_selfloops_node_ids());
    /// ```
    pub fn get_singleton_with_selfloops_node_ids(&self) -> Vec<NodeT> {
        self.iter_singleton_nodes_with_selfloops_node_ids()
            .collect()
    }

    /// Returns vector of singleton_with_selfloops node names of the graph.
    ///
    /// # Example
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// println!("The graph singleton_with_selfloops node names are {:?}.", graph.get_singleton_with_selfloops_node_names());
    /// ```
    pub fn get_singleton_with_selfloops_node_names(&self) -> Vec<String> {
        self.iter_singleton_nodes_with_selfloops_node_names()
            .collect()
    }

    /// Returns number of not singleton nodes within the graph.
    ///
    /// # Example
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// println!("The graph contains {} not singleton nodes", graph.get_connected_nodes_number());
    /// ```
    pub fn get_connected_nodes_number(&self) -> NodeT {
        self.connected_nodes_number
    }

    /// Returns density of the graph.
    ///
    /// # Example
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// println!("The graph density is {}", graph.get_density().unwrap());
    /// ```
    pub fn get_density(&self) -> Result<f64> {
        if !self.has_nodes() {
            return Err("The density of an empty graph is undefined.".to_string());
        }
        if !self.has_edges() {
            return Ok(0.0);
        }
        let nodes_number = self.get_nodes_number() as EdgeT;
        let total_nodes_number = nodes_number
            * match self.has_selfloops() {
                true => nodes_number,
                false => nodes_number - 1,
            };
        Ok(self.unique_edges_number as f64 / total_nodes_number as f64)
    }
    /// Returns the traps rate of the graph.
    ///
    /// THIS IS EXPERIMENTAL AND MUST BE PROVEN!
    ///
    /// # Example
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// println!("The Graph rate is {}", graph.get_trap_nodes_rate());
    /// ```
    pub fn get_trap_nodes_rate(&self) -> f64 {
        self.par_iter_node_ids()
            .map(|node_id| unsafe {
                if !self.is_unchecked_trap_node_from_node_id(node_id) {
                    self.iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                        .map(|dst| self.is_unchecked_trap_node_from_node_id(dst) as usize as f64)
                        .sum::<f64>()
                        / self.get_unchecked_node_degree_from_node_id(node_id) as f64
                } else {
                    1.0
                }
            })
            .sum::<f64>()
            / self.get_nodes_number() as f64
    }

    /// Returns unweighted mean node degree of the graph.
    ///
    /// # Example
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// println!("The mean node degree of the graph is  {}", graph.get_node_degrees_mean().unwrap());
    /// ```
    pub fn get_node_degrees_mean(&self) -> Result<f64> {
        if !self.has_nodes() {
            return Err(
                "The mean of the node degrees is not defined on an empty graph".to_string(),
            );
        }
        Ok(self.get_directed_edges_number() as f64 / self.get_nodes_number() as f64)
    }

    /// Returns weighted mean node degree of the graph.
    ///
    /// # Example
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// println!("The mean node degree of the graph is  {}", graph.get_weighted_node_degrees_mean().unwrap());
    /// ```
    pub fn get_weighted_node_degrees_mean(&self) -> Result<f64> {
        Ok(self.get_total_edge_weights()? / self.get_nodes_number() as f64)
    }

    /// Returns number of undirected edges of the graph.
    ///
    /// # Example
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// println!("The number of undirected edges of the graph is  {}", graph.get_undirected_edges_number());
    /// ```
    pub fn get_undirected_edges_number(&self) -> EdgeT {
        (self.get_directed_edges_number() - self.get_selfloops_number()) / 2
            + self.get_selfloops_number()
    }

    /// Returns number of undirected edges of the graph.
    ///
    /// # Example
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// println!("The number of unique undirected edges of the graph is  {}", graph.get_unique_undirected_edges_number());
    /// ```
    pub fn get_unique_undirected_edges_number(&self) -> EdgeT {
        (self.unique_edges_number - self.get_unique_selfloop_number() as EdgeT) / 2
            + self.get_unique_selfloop_number() as EdgeT
    }

    /// Returns number of edges of the graph.
    ///
    /// # Example
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// println!("The number of edges of the graph is  {}", graph.get_edges_number());
    /// ```
    pub fn get_edges_number(&self) -> EdgeT {
        match self.directed {
            true => self.get_directed_edges_number(),
            false => self.get_undirected_edges_number(),
        }
    }

    /// Returns number of unique edges of the graph.
    ///
    /// # Example
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// println!("The number of edges of the graph is  {}", graph.get_unique_edges_number());
    /// ```
    pub fn get_unique_edges_number(&self) -> EdgeT {
        match self.directed {
            true => self.get_unique_directed_edges_number(),
            false => self.get_unique_undirected_edges_number(),
        }
    }

    /// Returns unweighted median node degree of the graph
    ///
    /// # Example
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// println!("The median node degree of the graph is  {}", graph.get_node_degrees_median().unwrap());
    /// ```
    pub fn get_node_degrees_median(&self) -> Result<NodeT> {
        self.must_have_nodes()?;
        let mut degrees = self.get_node_degrees();
        degrees.par_sort_unstable();
        Ok(degrees[(self.get_nodes_number() / 2) as usize])
    }

    /// Returns weighted median node degree of the graph
    ///
    /// # Example
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// println!("The weighted median node degree of the graph is  {}", graph.get_weighted_node_degrees_median().unwrap());
    /// ```
    pub fn get_weighted_node_degrees_median(&self) -> Result<f64> {
        self.must_have_nodes()?;
        self.must_have_edge_weights()?;
        let mut weighted_degrees = self.get_weighted_node_degrees()?;
        weighted_degrees.par_sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        Ok(weighted_degrees[(self.get_nodes_number() / 2) as usize])
    }

    /// Returns maximum weighted node degree of the graph.
    ///
    /// # Safety
    /// This method will cause a panic on graphs without
    /// edge weights.
    pub unsafe fn get_unchecked_weighted_maximum_node_degree(&self) -> f64 {
        self.max_weighted_node_degree.unwrap()
    }

    /// Returns maximum weighted node degree of the graph.
    ///
    /// # Raises
    /// * If the current graph does not contain edge weights.
    pub fn get_weighted_maximum_node_degree(&self) -> Result<f64> {
        self.must_have_edge_weights()?;
        Ok(unsafe { self.get_unchecked_weighted_maximum_node_degree() })
    }

    /// Returns minimum weighted node degree of the graph.
    ///
    /// # Safety
    /// This method will cause a panic on graphs without
    /// edge weights.
    pub unsafe fn get_unchecked_weighted_minimum_node_degree(&self) -> f64 {
        self.max_weighted_node_degree.unwrap()
    }

    /// Returns minimum weighted node degree of the graph.
    ///
    /// # Raises
    /// * If the current graph does not contain edge weights.
    pub fn get_weighted_minimum_node_degree(&self) -> Result<f64> {
        self.must_have_edge_weights()?;
        Ok(unsafe { self.get_unchecked_weighted_minimum_node_degree() })
    }

    /// Returns maximum node degree of the graph.
    ///
    /// # Example
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// println!("The maximum node degree of the graph is  {}", graph.get_maximum_node_degree().unwrap());
    /// ```
    ///
    /// # Raises
    /// * If the graph does not contain any node (is an empty graph).
    pub fn get_maximum_node_degree(&self) -> Result<NodeT> {
        self.must_have_nodes()
            .map(|_| unsafe { self.get_unchecked_maximum_node_degree() })
    }

    #[cache_property(most_central_node_id)]
    /// Returns maximum node degree of the graph.
    ///
    /// # Safety
    /// This method fails with a panic if the graph does not have any node.
    ///
    /// # Example
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// println!("The node with maximum node degree of the graph is {}.", unsafe{graph.get_unchecked_most_central_node_id()});
    /// ```
    pub unsafe fn get_unchecked_most_central_node_id(&self) -> NodeT {
        self.par_iter_node_degrees()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .unwrap()
            .0 as NodeT
    }

    /// Returns maximum node degree of the graph.
    ///
    /// # Example
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// println!("The maximum node degree of the graph is  {}", graph.get_most_central_node_id().unwrap());
    /// ```
    pub fn get_most_central_node_id(&self) -> Result<NodeT> {
        self.must_have_nodes()
            .map(|_| unsafe { self.get_unchecked_most_central_node_id() as NodeT })
    }

    /// Returns minimum weighted node degree of the graph.
    pub fn get_weighted_mininum_node_degree(&self) -> Result<f64> {
        self.must_have_edge_weights()?;
        Ok(self.min_weighted_node_degree.unwrap())
    }

    /// Returns minimum node degree of the graph.
    ///
    /// # Example
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// println!("The minimum node degree of the graph is  {}", graph.get_minimum_node_degree().unwrap());
    /// ```
    ///
    /// # Raises
    /// * If the graph does not contain any node (is an empty graph).
    pub fn get_minimum_node_degree(&self) -> Result<NodeT> {
        self.must_have_nodes()
            .map(|_| unsafe { self.get_unchecked_minimum_node_degree() })
    }

    /// Returns mode node degree of the graph.
    ///
    /// # Example
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// println!("The mode node degree of the graph is  {}", graph.get_node_degrees_mode().unwrap());
    /// ```
    pub fn get_node_degrees_mode(&self) -> Result<NodeT> {
        if !self.has_nodes() {
            return Err(
                "The mode of the node degrees is not defined on an empty graph".to_string(),
            );
        }
        let counter: Counter<NodeT, usize> = Counter::init(self.iter_node_degrees());
        Ok(*counter
            .iter()
            .max_by_key(|&(_, count)| count)
            .map(|(degree, _)| degree)
            .unwrap())
    }

    /// Returns number of self-loops, including also those in eventual multi-edges.
    ///
    /// # Example
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// println!("The number of self-loops in the graph is  {}", graph.get_selfloops_number());
    /// ```
    pub fn get_selfloops_number(&self) -> EdgeT {
        self.selfloop_number
    }

    /// Returns number of unique self-loops, excluding those in eventual multi-edges.
    ///
    /// # Example
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// println!("The number of unique self-loops in the graph is  {}", graph.get_unique_selfloop_number());
    /// ```
    pub fn get_unique_selfloop_number(&self) -> NodeT {
        self.unique_selfloop_number
    }

    /// Returns rate of self-loops.
    ///
    /// # Example
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// println!("The rate of self-loops in the graph is  {}", graph.get_selfloop_nodes_rate().unwrap());
    /// ```
    pub fn get_selfloop_nodes_rate(&self) -> Result<f64> {
        if !self.has_edges() {
            return Err("The self-loops rate is not defined for graphs without edges.".to_string());
        }
        Ok(self.get_selfloops_number() as f64 / self.get_directed_edges_number() as f64)
    }
    /// Return name of the graph.
    ///
    /// # Example
    /// To the retrieve the name of the current graph instance selfloop_number can use:
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// assert_eq!(graph.get_name(), "STRING PPI".to_string());
    /// println!("The name of the current graph is {}.", graph.get_name());
    /// ```
    ///
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Return the number of traps (nodes without any outgoing edges that are not singletons)
    /// This also includes nodes with only a self-loops, therefore singletons with
    /// only a self-loops are not considered traps because you could make a walk on them.
    ///
    /// # Example
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// println!("There are {} trap nodes in the current graph.", graph.get_trap_nodes_number());
    /// ```
    ///
    pub fn get_trap_nodes_number(&self) -> EdgeT {
        (self.get_connected_nodes_number() + self.get_singleton_nodes_with_selfloops_number()
            - self.get_unique_source_nodes_number()) as EdgeT
    }

    /// Return vector of the non-unique source nodes.
    ///
    /// # Arguments
    /// * `directed`: bool - Whether to filter out the undirected edges.
    pub fn get_source_node_ids(&self, directed: bool) -> Vec<NodeT> {
        self.par_iter_source_node_ids(directed).collect()
    }

    /// Return vector of the non-unique source nodes names.
    ///
    /// # Arguments
    /// * `directed`: bool - Whether to filter out the undirected edges.
    pub fn get_source_names(&self, directed: bool) -> Vec<String> {
        self.par_iter_source_node_ids(directed)
            .map(|src| unsafe { self.get_unchecked_node_name_from_node_id(src) })
            .collect()
    }

    /// Return vector on the (non unique) destination nodes of the graph.
    ///
    /// # Arguments
    /// * `directed`: bool - Whether to filter out the undirected edges.
    pub fn get_destination_node_ids(&self, directed: bool) -> Vec<NodeT> {
        self.par_iter_destination_node_ids(directed).collect()
    }

    /// Return vector of the non-unique destination nodes names.
    ///
    /// # Arguments
    /// * `directed`: bool - Whether to filter out the undirected edges.
    pub fn get_destination_names(&self, directed: bool) -> Vec<String> {
        self.par_iter_destination_node_ids(directed)
            .map(|dst| unsafe { self.get_unchecked_node_name_from_node_id(dst) })
            .collect()
    }

    /// Return vector with the sorted nodes names.
    pub fn get_node_names(&self) -> Vec<String> {
        self.nodes.keys()
    }

    /// Return vector with the sorted nodes Ids.
    pub fn get_node_ids(&self) -> Vec<NodeT> {
        self.iter_node_ids().collect()
    }

    /// Return the edge types of the edges.
    pub fn get_edge_type_ids(&self) -> Result<Vec<Option<EdgeTypeT>>> {
        self.must_have_edge_types()
            .map(|_| self.edge_types.as_ref().map(|ets| ets.ids.clone()).unwrap())
    }

    /// Return the unique edge type IDs of the graph edges.
    ///
    /// # Example
    /// To retrieve the unique edge type IDs of the graph edges you can use:
    /// ```rust
    /// # let graph_with_edge_types = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// # let graph_without_edge_types = graph::test_utilities::load_ppi(false, false, true, true, false, false);
    /// assert!(graph_with_edge_types.get_unique_edge_type_ids().is_ok());
    /// assert!(graph_without_edge_types.get_unique_edge_type_ids().is_err());
    /// println!("The graph edge types are {:?}", graph_with_edge_types.get_unique_edge_type_ids());
    /// ```
    ///
    pub fn get_unique_edge_type_ids(&self) -> Result<Vec<EdgeTypeT>> {
        self.iter_unique_edge_type_ids()
            .map(|edge_type_ids| edge_type_ids.collect())
    }

    /// Return the edge types names.
    pub fn get_edge_type_names(&self) -> Result<Vec<Option<String>>> {
        self.must_have_edge_types().map(|_| {
            self.edge_types
                .as_ref()
                .map(|ets| {
                    ets.ids
                        .iter()
                        .map(|edge_type_id| unsafe {
                            self.get_unchecked_edge_type_name_from_edge_type_id(*edge_type_id)
                        })
                        .collect()
                })
                .unwrap()
        })
    }

    /// Return the edge types names.
    pub fn get_unique_edge_type_names(&self) -> Result<Vec<String>> {
        self.iter_unique_edge_type_names()
            .map(|iter_unique_edge_type_names| iter_unique_edge_type_names.collect())
    }

    /// Return the weights of the graph edges.
    ///
    /// # Example
    /// To get an the graph weights you can use:
    /// ```rust
    /// # let graph_with_weights = graph::test_utilities::load_ppi(false, false, true, true, false, false);
    /// # let graph_without_weights = graph::test_utilities::load_ppi(false, false, false, true, false, false);
    /// assert!(graph_with_weights.get_edge_weights().is_ok());
    /// assert!(graph_without_weights.get_edge_weights().is_err());
    /// println!("The graph weights are {:?}.", graph_with_weights.get_edge_weights());
    /// ```
    pub fn get_edge_weights(&self) -> Result<Vec<WeightT>> {
        self.must_have_edge_weights()?;
        Ok(self.weights.clone().unwrap())
    }

    #[cache_property(total_weights)]
    /// Return total edge weights, if graph has weights.
    ///
    /// # Example
    /// To get the total edge weights you can use:
    /// ```rust
    /// # let graph_with_weights = graph::test_utilities::load_ppi(false, false, true, true, false, false);
    /// # let graph_without_weights = graph::test_utilities::load_ppi(false, false, false, true, false, false);
    /// assert!(graph_with_weights.get_total_edge_weights().is_ok());
    /// assert!(graph_without_weights.get_total_edge_weights().is_err());
    /// println!("The graph total edge weights is {:?}.", graph_with_weights.get_total_edge_weights());
    /// ```
    ///
    /// # Raises
    /// * If the graph does not contain edge weights.
    pub fn get_total_edge_weights(&self) -> Result<f64> {
        self.must_have_edge_weights()
            .map(|weights| weights.iter().map(|&w| w as f64).sum())
    }

    /// Return the node types of the graph nodes.
    ///
    /// # Example
    /// To retrieve the node type IDs of the graph nodes you can use:
    /// ```rust
    /// # let graph_with_node_types = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// # let graph_without_node_types = graph::test_utilities::load_ppi(false, true, true, true, false, false);
    /// assert!(graph_with_node_types.get_node_type_ids().is_ok());
    /// assert!(graph_without_node_types.get_node_type_ids().is_err());
    /// println!("The graph node types are {:?}", graph_with_node_types.get_node_type_ids());
    /// ```
    ///
    pub fn get_node_type_ids(&self) -> Result<Vec<Option<Vec<NodeTypeT>>>> {
        self.must_have_node_types()
            .map(|_| self.node_types.as_ref().map(|nts| nts.ids.clone()).unwrap())
    }

    /// Returns one-hot encoded node types.
    ///
    /// # Raises
    /// * If the graph does not have node types.
    pub fn get_one_hot_encoded_node_types(&self) -> Result<Vec<Vec<bool>>> {
        Ok(self.iter_one_hot_encoded_node_type_ids()?.collect())
    }

    /// Returns one-hot encoded known node types.
    ///
    /// # Raises
    /// * If the graph does not have node types.
    pub fn get_one_hot_encoded_known_node_types(&self) -> Result<Vec<Vec<bool>>> {
        Ok(self.iter_one_hot_encoded_known_node_type_ids()?.collect())
    }

    /// Returns one-hot encoded edge types.
    ///
    /// # Raises
    /// * If the graph does not have edge types.
    pub fn get_one_hot_encoded_edge_types(&self) -> Result<Vec<Vec<bool>>> {
        Ok(self.iter_one_hot_encoded_edge_type_ids()?.collect())
    }

    /// Returns one-hot encoded known edge types.
    ///
    /// # Raises
    /// * If the graph does not have edge types.
    pub fn get_one_hot_encoded_known_edge_types(&self) -> Result<Vec<Vec<bool>>> {
        Ok(self.iter_one_hot_encoded_known_edge_type_ids()?.collect())
    }

    /// Return the node types names.
    ///
    /// # Example
    /// To retrieve the node type names of the graph nodes you can use:
    /// ```rust
    /// # let graph_with_node_types = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// # let graph_without_node_types = graph::test_utilities::load_ppi(false, true, true, true, false, false);
    /// assert!(graph_with_node_types.get_node_type_names().is_ok());
    /// assert!(graph_without_node_types.get_node_type_names().is_err());
    /// println!("The graph node types are {:?}", graph_with_node_types.get_node_type_names());
    /// ```
    ///
    pub fn get_node_type_names(&self) -> Result<Vec<Option<Vec<String>>>> {
        self.must_have_node_types().map(|_| {
            self.iter_node_ids()
                .map(|node_id| unsafe { self.get_unchecked_node_type_names_from_node_id(node_id) })
                .collect::<Vec<Option<Vec<String>>>>()
        })
    }

    /// Return the unique node type IDs of the graph nodes.
    ///
    /// # Example
    /// To retrieve the unique node type IDs of the graph nodes you can use:
    /// ```rust
    /// # let graph_with_node_types = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// # let graph_without_node_types = graph::test_utilities::load_ppi(false, true, true, true, false, false);
    /// assert!(graph_with_node_types.get_unique_node_type_ids().is_ok());
    /// assert!(graph_without_node_types.get_unique_node_type_ids().is_err());
    /// println!("The graph node types are {:?}", graph_with_node_types.get_unique_node_type_ids());
    /// ```
    ///
    pub fn get_unique_node_type_ids(&self) -> Result<Vec<NodeTypeT>> {
        self.iter_unique_node_type_ids()
            .map(|iter_unique_node_type_ids| iter_unique_node_type_ids.collect())
    }

    /// Return the unique node types names.
    ///
    /// # Example
    /// To retrieve the unique node type names of the graph nodes you can use:
    /// ```rust
    /// # let graph_with_node_types = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// # let graph_without_node_types = graph::test_utilities::load_ppi(false, true, true, true, false, false);
    /// assert!(graph_with_node_types.get_unique_node_type_names().is_ok());
    /// assert!(graph_without_node_types.get_unique_node_type_names().is_err());
    /// println!("The graph node types are {:?}", graph_with_node_types.get_unique_node_type_names());
    /// ```
    ///
    pub fn get_unique_node_type_names(&self) -> Result<Vec<String>> {
        self.iter_unique_node_type_names()
            .map(|iter_unique_node_type_names| iter_unique_node_type_names.collect())
    }

    /// Return number of the unique edges in the graph.
    pub fn get_unique_directed_edges_number(&self) -> EdgeT {
        self.unique_edges_number
    }

    /// Return the nodes mapping.
    pub fn get_nodes_mapping(&self) -> HashMap<String, NodeT> {
        self.nodes.map()
    }

    /// Return vector with the sorted edge Ids.
    ///
    /// # Arguments
    /// * `directed`: bool - Whether to filter out the undirected edges.
    pub fn get_edge_node_ids(&self, directed: bool) -> Vec<Vec<NodeT>> {
        self.par_iter_edge_node_ids(directed)
            .map(|(_, src, dst)| vec![src, dst])
            .collect()
    }

    /// Return vector with the sorted edge names.
    ///
    /// # Arguments
    /// * `directed`: bool - Whether to filter out the undirected edges.
    pub fn get_edge_node_names(&self, directed: bool) -> Vec<(String, String)> {
        self.par_iter_edges(directed)
            .map(|(_, _, src_name, _, dst_name)| (src_name, dst_name))
            .collect()
    }

    /// Returns number of nodes with unknown node type.
    ///
    /// # Raises
    /// * If there are no node types in the graph.
    pub fn get_unknown_node_types_number(&self) -> Result<NodeT> {
        self.must_have_node_types()
            .map(|node_types| node_types.get_unknown_count())
    }

    /// Returns the number of node with known node type.
    ///
    /// # Raises
    /// * If there are no node types in the graph.
    pub fn get_known_node_types_number(&self) -> Result<NodeT> {
        Ok(self.get_nodes_number() - self.get_unknown_node_types_number()?)
    }

    /// Returns rate of unknown node types over total nodes number.
    ///
    /// # Raises
    /// * If there are no node types in the graph.
    pub fn get_unknown_node_types_rate(&self) -> Result<f64> {
        self.get_unknown_node_types_number()
            .map(|unknown_node_types_number| {
                unknown_node_types_number as f64 / self.get_nodes_number() as f64
            })
    }

    /// Returns rate of known node types over total nodes number.
    ///
    /// # Raises
    /// * If there are no node types in the graph.
    pub fn get_known_node_types_rate(&self) -> Result<f64> {
        self.get_known_node_types_number()
            .map(|known_node_types_number| {
                known_node_types_number as f64 / self.get_nodes_number() as f64
            })
    }

    /// Returns minimum number of node types.
    ///
    /// # Raises
    /// * If there are no node types in the graph.
    pub fn get_minimum_node_types_number(&self) -> Result<NodeT> {
        self.must_have_node_types()
            .map(|node_types| node_types.get_minimum_node_type_count())
    }

    /// Returns maximum number of node types.
    ///
    /// # Raises
    /// * If there are no node types in the graph.
    pub fn get_maximum_node_types_number(&self) -> Result<NodeT> {
        self.must_have_node_types()
            .map(|node_types| node_types.get_maximum_node_type_count())
    }

    /// Returns number of maximum multilabel count.
    ///
    /// This value is the maximum number of multilabel counts
    /// that appear in any given node in the graph.
    pub fn get_maximum_multilabel_count(&self) -> Result<NodeTypeT> {
        self.must_have_node_types()
            .map(|node_types| node_types.get_maximum_multilabel_count())
    }

    /// Returns number of singleton node types.
    ///
    /// # Raises
    /// * If the graph does not have node types.
    pub fn get_singleton_node_types_number(&self) -> Result<NodeTypeT> {
        self.iter_node_type_counts().map(|iter_node_type_counts| {
            iter_node_type_counts
                .map(|node_type_count| (node_type_count == 1) as NodeTypeT)
                .sum()
        })
    }

    /// Returns vector of singleton node types IDs.
    ///
    /// # Raises
    /// * If the graph does not have node types.
    pub fn get_singleton_node_type_ids(&self) -> Result<Vec<NodeTypeT>> {
        self.iter_singleton_node_type_ids()
            .map(|iter_singleton_node_type_ids| iter_singleton_node_type_ids.collect())
    }

    /// Returns vector of singleton node types names.
    ///
    /// # Raises
    /// * If the graph does not have node types.
    pub fn get_singleton_node_type_names(&self) -> Result<Vec<String>> {
        self.iter_singleton_node_type_names()
            .map(|iter_singleton_node_type_names| iter_singleton_node_type_names.collect())
    }

    /// Returns number of unknown edge types.
    ///
    /// # Raises
    /// * If there are no edge types in the graph.
    pub fn get_unknown_edge_types_number(&self) -> Result<EdgeT> {
        self.must_have_edge_types()
            .map(|edge_types| edge_types.get_unknown_count())
    }

    /// Returns edge IDs of the edges with unknown edge types
    ///
    /// # Raises
    /// * If there are no edge types in the graph.
    pub fn get_edge_ids_with_unknown_edge_types(&self) -> Result<Vec<EdgeT>> {
        self.iter_edge_ids_with_unknown_edge_types()
            .map(|x| x.collect())
    }

    /// Returns edge IDs of the edges with known edge types
    ///
    /// # Raises
    /// * If there are no edge types in the graph.
    pub fn get_edge_ids_with_known_edge_types(&self) -> Result<Vec<EdgeT>> {
        self.iter_edge_ids_with_known_edge_types()
            .map(|x| x.collect())
    }

    /// Returns edge node IDs of the edges with unknown edge types
    ///
    /// # Arguments
    /// * `directed`: bool - Whether to iterated the edges as a directed or undirected edge list.
    ///
    /// # Raises
    /// * If there are no edge types in the graph.
    pub fn get_edge_node_ids_with_unknown_edge_types(
        &self,
        directed: bool,
    ) -> Result<Vec<(NodeT, NodeT)>> {
        self.iter_edge_node_ids_with_unknown_edge_types(directed)
            .map(|x| x.collect())
    }

    /// Returns edge node IDs of the edges with known edge types
    ///
    /// # Arguments
    /// * `directed`: bool - Whether to iterated the edges as a directed or undirected edge list.
    ///
    /// # Raises
    /// * If there are no edge types in the graph.
    pub fn get_edge_node_ids_with_known_edge_types(
        &self,
        directed: bool,
    ) -> Result<Vec<(NodeT, NodeT)>> {
        self.iter_edge_node_ids_with_known_edge_types(directed)
            .map(|x| x.collect())
    }

    /// Returns edge node names of the edges with unknown edge types
    ///
    /// # Arguments
    /// * `directed`: bool - Whether to iterated the edges as a directed or undirected edge list.
    ///
    /// # Raises
    /// * If there are no edge types in the graph.
    pub fn get_edge_node_names_with_unknown_edge_types(
        &self,
        directed: bool,
    ) -> Result<Vec<(String, String)>> {
        self.iter_edge_node_names_with_unknown_edge_types(directed)
            .map(|x| x.collect())
    }

    /// Returns edge node names of the edges with known edge types
    ///
    /// # Arguments
    /// * `directed`: bool - Whether to iterated the edges as a directed or undirected edge list.
    ///
    /// # Raises
    /// * If there are no edge types in the graph.
    pub fn get_edge_node_names_with_known_edge_types(
        &self,
        directed: bool,
    ) -> Result<Vec<(String, String)>> {
        self.iter_edge_node_names_with_known_edge_types(directed)
            .map(|x| x.collect())
    }

    /// Returns a boolean vector that for each node contains whether it has an
    /// unknown node type.
    ///
    /// # Raises
    /// * If there are no edge types in the graph.
    pub fn get_edge_ids_with_unknown_edge_types_mask(&self) -> Result<Vec<bool>> {
        self.iter_edge_ids_with_unknown_edge_types().map(|x| {
            let mut mask = vec![false; self.get_directed_edges_number() as usize];
            x.for_each(|id| {
                mask[id as usize] = true;
            });
            mask
        })
    }

    /// Returns a boolean vector that for each node contains whether it has an
    /// unknown edge type.
    ///
    /// # Raises
    /// * If there are no edge types in the graph.
    pub fn get_edge_ids_with_known_edge_types_mask(&self) -> Result<Vec<bool>> {
        self.iter_edge_ids_with_known_edge_types().map(|x| {
            let mut mask = vec![false; self.get_directed_edges_number() as usize];
            x.for_each(|id| {
                mask[id as usize] = true;
            });
            mask
        })
    }

    /// Returns node IDs of the nodes with unknown node types
    ///
    /// # Raises
    /// * If there are no node types in the graph.
    pub fn get_node_ids_with_unknown_node_types(&self) -> Result<Vec<NodeT>> {
        self.iter_node_ids_with_unknown_node_types()
            .map(|x| x.collect())
    }

    /// Returns node IDs of the nodes with known node types
    ///
    /// # Raises
    /// * If there are no node types in the graph.
    pub fn get_node_ids_with_known_node_types(&self) -> Result<Vec<NodeT>> {
        self.iter_node_ids_with_known_node_types()
            .map(|x| x.collect())
    }

    /// Returns node names of the nodes with unknown node types
    ///
    /// # Raises
    /// * If there are no node types in the graph.
    pub fn get_node_names_with_unknown_node_types(&self) -> Result<Vec<String>> {
        self.iter_node_names_with_unknown_node_types()
            .map(|x| x.collect())
    }

    /// Returns node names of the nodes with known node types
    ///
    /// # Raises
    /// * If there are no node types in the graph.
    pub fn get_node_names_with_known_node_types(&self) -> Result<Vec<String>> {
        self.iter_node_names_with_known_node_types()
            .map(|x| x.collect())
    }

    /// Returns a boolean vector that for each node contains whether it has an
    /// unknown node type.
    ///
    /// # Raises
    /// * If there are no node types in the graph.
    pub fn get_node_ids_with_unknown_node_types_mask(&self) -> Result<Vec<bool>> {
        self.iter_node_ids_with_unknown_node_types().map(|x| {
            let mut mask = vec![false; self.get_nodes_number() as usize];
            x.for_each(|id| {
                mask[id as usize] = true;
            });
            mask
        })
    }

    /// Returns a boolean vector that for each node contains whether it has an
    /// known node type.
    ///
    /// # Raises
    /// * If there are no node types in the graph.
    pub fn get_node_ids_with_known_node_types_mask(&self) -> Result<Vec<bool>> {
        self.iter_node_ids_with_known_node_types().map(|x| {
            let mut mask = vec![false; self.get_nodes_number() as usize];
            x.for_each(|id| {
                mask[id as usize] = true;
            });
            mask
        })
    }

    /// Returns the number of edge with known edge type.
    ///
    /// # Raises
    /// * If there are no edge types in the graph.
    pub fn get_known_edge_types_number(&self) -> Result<EdgeT> {
        Ok(self.get_directed_edges_number() - self.get_unknown_edge_types_number()?)
    }

    /// Returns rate of unknown edge types over total edges number.
    ///
    /// # Raises
    /// * If there are no edge types in the graph.
    pub fn get_unknown_edge_types_rate(&self) -> Result<f64> {
        self.get_unknown_edge_types_number()
            .map(|unknown_edge_types_number| {
                unknown_edge_types_number as f64 / self.get_directed_edges_number() as f64
            })
    }

    /// Returns rate of known edge types over total edges number.
    ///
    /// # Raises
    /// * If there are no edge types in the graph.
    pub fn get_known_edge_types_rate(&self) -> Result<f64> {
        self.get_known_edge_types_number()
            .map(|known_edge_types_number| {
                known_edge_types_number as f64 / self.get_directed_edges_number() as f64
            })
    }

    /// Returns minimum number of edge types.
    ///
    /// # Raises
    /// * If there are no edge types in the graph.
    pub fn get_minimum_edge_types_number(&self) -> Result<EdgeT> {
        self.must_have_edge_types()
            .map(|edge_types| edge_types.min_edge_type_count())
    }

    /// Returns number of singleton edge types.
    ///
    /// # Raises
    /// * If the graph does not have edge types.
    pub fn get_singleton_edge_types_number(&self) -> Result<EdgeTypeT> {
        self.iter_edge_type_counts().map(|iter_edge_type_counts| {
            iter_edge_type_counts
                .map(|edge_type_count| (edge_type_count == 1) as EdgeTypeT)
                .sum()
        })
    }

    /// Returns vector of singleton edge types IDs.
    ///
    /// # Raises
    /// * If the graph does not have edge types.
    pub fn get_singleton_edge_type_ids(&self) -> Result<Vec<EdgeTypeT>> {
        self.iter_singleton_edge_type_ids()
            .map(|iter_singleton_edge_type_ids| iter_singleton_edge_type_ids.collect())
    }

    /// Returns vector of singleton edge types names.
    ///
    /// # Raises
    /// * If the graph does not have edge types.
    pub fn get_singleton_edge_type_names(&self) -> Result<Vec<String>> {
        self.iter_singleton_edge_type_names()
            .map(|iter_singleton_edge_type_names| iter_singleton_edge_type_names.collect())
    }

    /// Returns number of nodes in the graph.
    pub fn get_nodes_number(&self) -> NodeT {
        self.nodes.len() as NodeT
    }

    /// Return a vector with the components each node belongs to.
    ///
    /// E.g. If we have two components `[0, 2, 3]` and `[1, 4, 5]` the result will look like
    /// `[0, 1, 0, 0, 1, 1]`
    ///
    /// # Arguments
    /// * `verbose`: Option<bool> - Whether to show the loading bar.
    pub fn get_node_connected_component_ids(&self, verbose: Option<bool>) -> Vec<NodeT> {
        match self.directed {
            true => self.spanning_arborescence_kruskal(verbose).1,
            false => self.connected_components(verbose).unwrap().0,
        }
    }

    /// Returns number of directed edges in the graph.
    pub fn get_directed_edges_number(&self) -> EdgeT {
        self.edges.len() as EdgeT
    }

    /// Returns number of edge types in the graph.
    ///
    /// # Raises
    /// * If there are no edge types in the current graph.
    pub fn get_edge_types_number(&self) -> Result<EdgeTypeT> {
        self.must_have_edge_types()
            .map(|ets| ets.len() as EdgeTypeT)
    }

    /// Returns number of node types in the graph.
    ///
    /// # Raises
    /// * If there are no node types in the current graph.
    pub fn get_node_types_number(&self) -> Result<NodeTypeT> {
        self.must_have_node_types()
            .map(|nts| nts.len() as NodeTypeT)
    }

    /// Returns the unweighted degree of every node in the graph.
    pub fn get_node_degrees(&self) -> Vec<NodeT> {
        self.par_iter_node_degrees().collect()
    }

    /// Returns the weighted degree of every node in the graph.
    pub fn get_weighted_node_degrees(&self) -> Result<Vec<f64>> {
        Ok(self.par_iter_weighted_node_degrees()?.collect())
    }

    /// Return set of nodes that are not singletons.
    pub fn get_not_singletons_node_ids(&self) -> Vec<NodeT> {
        self.iter_connected_node_ids().collect()
    }

    /// Return mapping from instance not trap nodes to dense nodes.
    pub fn get_dense_nodes_mapping(&self) -> HashMap<NodeT, NodeT> {
        self.iter_connected_node_ids()
            .enumerate()
            .map(|(i, node)| (node as NodeT, i as NodeT))
            .collect()
    }

    /// Return number of edges that have multigraph syblings.
    pub fn get_parallel_edges_number(&self) -> EdgeT {
        self.get_directed_edges_number() - self.get_unique_edges_number()
    }

    /// Return vector with node cumulative_node_degrees, that is the comulative node degree.
    pub fn get_cumulative_node_degrees(&self) -> Vec<EdgeT> {
        self.cumulative_node_degrees.as_ref().map_or_else(
            || {
                self.par_iter_node_ids()
                    .map(|src| unsafe { self.get_unchecked_edge_id_from_node_ids(src + 1, 0) })
                    .collect()
            },
            |cumulative_node_degrees| cumulative_node_degrees.clone(),
        )
    }

    /// Returns number of the source nodes.
    ///
    /// # Example
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// println!("The number of sources of the graph (not trap nodes) is {}", graph.get_unique_source_nodes_number());
    /// ```
    pub fn get_unique_source_nodes_number(&self) -> NodeT {
        self.unique_sources
            .as_ref()
            .map_or(self.get_nodes_number(), |x| x.len() as NodeT)
    }

    /// Returns edge type IDs counts hashmap.
    ///
    /// # Example
    /// In order to compute an hashmap of the edge type IDs you can use:
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// for (edge_type_id, count) in graph.get_edge_type_id_counts_hashmap().unwrap().iter() {
    ///     println!("edge type id {}: count: {}", edge_type_id, count);
    /// }
    /// ```
    ///
    /// # Raises
    /// * If there are no edge types in the current graph instance.
    pub fn get_edge_type_id_counts_hashmap(&self) -> Result<HashMap<EdgeTypeT, EdgeT>> {
        self.iter_unique_edge_type_ids_and_counts()
            .map(|iter_unique_edge_type_ids_and_counts| {
                iter_unique_edge_type_ids_and_counts.collect()
            })
    }

    /// Returns edge type names counts hashmap.
    ///
    /// # Example
    /// In order to compute an hashmap of the edge type names you can use:
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// for (edge_type_id, count) in graph.get_edge_type_id_counts_hashmap().unwrap().iter() {
    ///     println!("edge type name {}: count: {}", edge_type_id, count);
    /// }
    /// ```
    ///
    /// # Raises
    /// * If there are no edge types in the current graph instance.
    pub fn get_edge_type_names_counts_hashmap(&self) -> Result<HashMap<String, EdgeT>> {
        self.iter_unique_edge_type_names_and_counts().map(
            |iter_unique_edge_type_names_and_counts| {
                iter_unique_edge_type_names_and_counts.collect()
            },
        )
    }

    /// Returns node type IDs counts hashmap.
    ///
    /// # Example
    /// In order to compute an hashmap of the node type IDs you can use:
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// for (node_type_id, count) in graph.get_node_type_id_counts_hashmap().unwrap().iter() {
    ///     println!("node type id {}: count: {}", node_type_id, count);
    /// }
    /// ```
    ///
    /// # Raises
    /// * If there are no node types in the current graph instance.
    pub fn get_node_type_id_counts_hashmap(&self) -> Result<HashMap<NodeTypeT, NodeT>> {
        self.iter_unique_node_type_ids_and_counts()
            .map(|iter_unique_node_type_ids_and_counts| {
                iter_unique_node_type_ids_and_counts.collect()
            })
    }

    #[no_inverse_method]
    /// Returns node type names counts hashmap.
    ///
    /// # Example
    /// In order to compute an hashmap of the node type names you can use:
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// for (node_type_id, count) in graph.get_node_type_id_counts_hashmap().unwrap().iter() {
    ///     println!("node type name {}: count: {}", node_type_id, count);
    /// }
    /// ```
    ///
    /// # Raises
    /// * If there are no node types in the current graph instance.
    pub fn get_node_type_names_counts_hashmap(&self) -> Result<HashMap<String, NodeT>> {
        self.iter_unique_node_type_names_and_counts().map(
            |iter_unique_node_type_names_and_counts| {
                iter_unique_node_type_names_and_counts.collect()
            },
        )
    }
}
