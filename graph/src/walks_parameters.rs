use super::graph::*;
use super::types::*;
use rayon::prelude::*;
use std::collections::HashMap;

pub struct WalkWeights {
    pub(crate) return_weight: ParamsT,
    pub(crate) explore_weight: ParamsT,
    pub(crate) change_node_type_weight: ParamsT,
    pub(crate) change_edge_type_weight: ParamsT,
}

pub struct SingleWalkParameters {
    pub(crate) length: usize,
    pub(crate) weights: WalkWeights,
}

pub struct WalksParameters {
    pub(crate) single_walk_parameters: SingleWalkParameters,
    pub(crate) iterations: usize,
    pub(crate) min_length: usize,
    pub(crate) verbose: bool,
    pub(crate) start_node: NodeT,
    pub(crate) end_node: NodeT,
    pub(crate) seed: NodeT,
    pub(crate) dense_nodes_mapping: Option<HashMap<NodeT, NodeT>>,
}

impl Default for WalkWeights {
    fn default() -> WalkWeights {
        WalkWeights {
            return_weight: 1.0,
            explore_weight: 1.0,
            change_node_type_weight: 1.0,
            change_edge_type_weight: 1.0,
        }
    }
}

impl WalkWeights {
    /// Validate given weight and format the exception if necessary, eventually.
    ///
    /// # Arguments
    ///
    /// * weight_name: &str - name of the weight, used for building the exception.
    /// * weight: Option<WeightT> - Value of the weight.
    ///
    fn validate_weight(weight_name: &str, weight: WeightT) -> Result<WeightT, String> {
        if weight <= 0.0 || !weight.is_finite() {
            Err(format!(
                concat!(
                    "Given '{}' ({}) ",
                    "is not a strictly positive real number."
                ),
                weight_name, weight
            ))
        } else {
            Ok(weight)
        }
    }

    /// Set the return weight.
    ///
    /// # Arguments
    ///
    /// * return_weight: Option<WeightT> - weight for the exploitation factor.
    ///
    pub fn set_return_weight(
        mut self,
        return_weight: Option<WeightT>,
    ) -> Result<WalkWeights, String> {
        if let Some(rw) = return_weight {
            self.return_weight = WalkWeights::validate_weight("return_weight", rw)?;
        }
        Ok(self)
    }

    /// Set the explore weight.
    ///
    /// # Arguments
    ///
    /// * explore_weight: Option<WeightT> - weight for the exploration factor.
    ///
    pub fn set_explore_weight(
        mut self,
        explore_weight: Option<WeightT>,
    ) -> Result<WalkWeights, String> {
        if let Some(ew) = explore_weight {
            self.explore_weight = WalkWeights::validate_weight("explore_weight", ew)?;
        }
        Ok(self)
    }

    /// Set the change_node_type weight.
    ///
    /// # Arguments
    ///
    /// * change_node_type_weight: Option<WeightT> - weight for the exploration of different node types.
    ///
    pub fn set_change_node_type_weight(
        mut self,
        change_node_type_weight: Option<WeightT>,
    ) -> Result<WalkWeights, String> {
        if let Some(cntw) = change_node_type_weight {
            self.change_node_type_weight =
                WalkWeights::validate_weight("change_node_type_weight", cntw)?;
        }
        Ok(self)
    }

    /// Set the change_edge_type weight.
    ///
    /// # Arguments
    ///
    /// * change_edge_type_weight: Option<WeightT> - weight for the exploration of different node types.
    ///
    pub fn set_change_edge_type_weight(
        mut self,
        change_edge_type_weight: Option<WeightT>,
    ) -> Result<WalkWeights, String> {
        if let Some(cetw) = change_edge_type_weight {
            self.change_edge_type_weight =
                WalkWeights::validate_weight("change_edge_type_weight", cetw)?;
        }
        Ok(self)
    }

    /// Return boolean value representing if walk is of first order.
    pub fn is_first_order_walk(&self) -> bool {
        let weights = vec![
            self.change_node_type_weight,
            self.change_edge_type_weight,
            self.return_weight,
            self.explore_weight
        ];
        weights.iter().all(
            |weight| (weight - 1.0).abs() < f64::EPSILON
        )
    }
}

impl SingleWalkParameters {
    pub fn new(length: usize, weights: WalkWeights) -> Result<SingleWalkParameters, String> {
        if length == 0 {
            return Err(String::from("The provided lenght for the walk is zero!"));
        }
        Ok(SingleWalkParameters { length, weights })
    }

    /// Return boolean value representing if walk is of first order.
    pub fn is_first_order_walk(&self) -> bool {
        self.weights.is_first_order_walk()
    }
}

impl WalksParameters {
    pub fn new(
        single_walk_parameters: SingleWalkParameters,
        start_node: NodeT,
        end_node: NodeT,
    ) -> Result<WalksParameters, String> {
        if start_node > end_node {
            return Err(format!(
                concat!(
                    "Given start node index ({}) ",
                    "is greater than given end node index ({})."
                ),
                start_node, end_node
            ));
        }

        Ok(WalksParameters {
            start_node,
            end_node,
            single_walk_parameters,
            iterations: 1,
            min_length: 1,
            seed: 42,
            verbose: false,
            dense_nodes_mapping: None,
        })
    }

    /// Set the iterations.
    ///
    /// # Arguments
    ///
    /// * iterations: Option<usize> - Wethever to show the loading bar or not.
    ///
    pub fn set_iterations(mut self, iterations: Option<usize>) -> Result<WalksParameters, String> {
        if let Some(it) = iterations {
            if it == 0 {
                return Err(String::from(
                    "Iterations parameter must be a strictly positive integer.",
                ));
            }
            self.iterations = it;
        }
        Ok(self)
    }

    /// Set the min_length.
    ///
    /// # Arguments
    ///
    /// * min_length: Option<usize> - Wethever to show the loading bar or not.
    ///
    pub fn set_min_length(mut self, min_length: Option<usize>) -> Result<WalksParameters, String> {
        if let Some(ml) = min_length {
            if ml == 0 {
                return Err(String::from(
                    "min_length parameter must be a strictly positive integer.",
                ));
            }
            self.min_length = ml;
        }
        Ok(self)
    }

    /// Set the verbose.
    ///
    /// # Arguments
    ///
    /// * verbose: Option<bool> - Wethever to show the loading bar or not.
    ///
    pub fn set_verbose(mut self, verbose: Option<bool>) -> WalksParameters {
        if let Some(v) = verbose {
            self.verbose = v;
        }
        self
    }

    /// Set the seed.
    ///
    /// # Arguments
    ///
    /// * seed: Option<usize> - Seed for reproducible random walks.
    ///
    pub fn set_seed(mut self, seed: Option<usize>) -> WalksParameters {
        if let Some(s) = seed {
            self.seed = s;
        }
        self
    }

    /// Set the dense_nodes_mapping.
    ///
    /// The nodes mapping primary porpose is to map a sparse set of nodes into
    /// a smaller dense set of nodes.
    ///
    /// # Arguments
    ///
    /// * dense_nodes_mapping: Option<HashMap<NodeT, NodeT>> - mapping for the mapping the nodes of the walks.
    ///
    pub fn set_dense_nodes_mapping(
        mut self,
        dense_nodes_mapping: Option<HashMap<NodeT, NodeT>>,
    ) -> WalksParameters {
        self.dense_nodes_mapping = dense_nodes_mapping;
        self
    }

    /// Validate for graph.
    ///
    /// Check if walks parameters are compatible with given graph.
    ///
    /// # Arguments
    ///
    /// * graph: Graph - Graph object for which parameters are to be validated.
    ///
    pub fn validate(&self, graph: &Graph) -> Result<(), String> {
        if self.start_node >= graph.not_trap_nodes.len() {
            return Err(format!(
                concat!(
                    "Given start node index ({})",
                    "is greater than number of not trap nodes in graph ({})."
                ),
                self.start_node,
                graph.not_trap_nodes.len()
            ));
        }

        if self.min_length >= self.single_walk_parameters.length {
            return Err(format!(
                "The given min-walk-length {} is bigger or equal to the given walk length {}",
                self.min_length, self.single_walk_parameters.length
            ));
        }

        if self.end_node > graph.not_trap_nodes.len() {
            return Err(format!(
                concat!(
                    "Given end node index ({})",
                    "is greater than number of not trap nodes in graph ({})."
                ),
                self.end_node,
                graph.not_trap_nodes.len()
            ));
        }

        if let Some(dense_nodes_mapping) = &self.dense_nodes_mapping {
            if !(&graph.not_trap_nodes)
                .into_par_iter()
                .all(|node| dense_nodes_mapping.contains_key(&node))
            {
                return Err(String::from(concat!(
                    "Given nodes mapping does not contain ",
                    "one or more not trap nodes that may be extracted from walk."
                )));
            }
        }

        Ok(())
    }

    /// Return delta between start and end nodes.
    fn delta(&self) -> NodeT {
        self.end_node - self.start_node
    }

    /// Return number of total iterations to execute.
    pub fn total_iterations(&self) -> usize {
        self.iterations * self.delta()
    }

    /// Return given index with mode applied using given parameters.
    pub fn mode_index(&self, index: usize) -> NodeT {
        self.start_node + (index % self.delta())
    }

    /// Return boolean value representing if walk is of first order.
    pub fn is_first_order_walk(&self) -> bool {
        self.single_walk_parameters.is_first_order_walk()
    }
}
