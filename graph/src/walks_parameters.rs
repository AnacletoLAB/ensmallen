use super::*;
use rayon::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Debug)]
/// Struct to wrap walk weights.
pub struct WalkWeights {
    pub(crate) return_weight: ParamsT,
    pub(crate) explore_weight: ParamsT,
    pub(crate) change_node_type_weight: ParamsT,
    pub(crate) change_edge_type_weight: ParamsT,
}

#[derive(Clone, Debug)]
/// Struct to wrap parameters relative to a single walk.
pub struct SingleWalkParameters {
    pub(crate) walk_length: NodeT,
    pub(crate) weights: WalkWeights,
    pub(crate) max_neighbours: Option<NodeT>,
}

#[derive(Clone, Debug)]
/// Struct to wrap parameters relative to a set of walks.
pub struct WalksParameters {
    pub(crate) single_walk_parameters: SingleWalkParameters,
    pub(crate) iterations: NodeT,
    pub(crate) random_state: NodeT,
    pub(crate) dense_node_mapping: Option<HashMap<NodeT, NodeT>>,
}

impl Default for WalkWeights {
    /// Create new WalkWeights object.
    ///
    /// The default WalkWeights object is parametrized to execute a first-order walk.
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

    /// Return boolean value representing if walk is of first order.
    pub fn is_first_order_walk(&self) -> bool {
        let weights = vec![
            self.change_node_type_weight,
            self.change_edge_type_weight,
            self.return_weight,
            self.explore_weight,
        ];
        weights.iter().all(|weight| !not_one(*weight))
    }

    /// Return boolean value representing if walk is of first order.
    pub fn is_walk_without_destinations(&self) -> bool {
        let weights = vec![
            self.change_node_type_weight,
            self.return_weight,
            self.explore_weight,
        ];
        weights.iter().all(|weight| !not_one(*weight))
    }
}

impl SingleWalkParameters {
    /// Create new WalksParameters object.
    ///
    /// By default the object is parametrized for a simple first-order walk.
    ///
    /// # Arguments
    ///
    /// * walk_length: usize - Maximal walk_length of the walk.
    pub fn new(walk_length: NodeT) -> Result<SingleWalkParameters, String> {
        if walk_length == 0 {
            return Err(String::from("The provided lenght for the walk is zero!"));
        }
        Ok(SingleWalkParameters {
            walk_length,
            weights: WalkWeights::default(),
            max_neighbours: None,
        })
    }

    /// Return boolean value representing if walk is of first order.
    pub fn is_first_order_walk(&self) -> bool {
        self.weights.is_first_order_walk()
    }
}

/// Setters for the Walk's parameters
impl WalksParameters {
    /// Create new WalksParameters object.
    ///
    /// By default the object is parametrized for a simple first-order walk.
    ///
    /// # Arguments
    ///
    /// * walk_length: NodeT - Maximal walk_length of the walk.
    pub fn new(walk_length: NodeT) -> Result<WalksParameters, String> {
        Ok(WalksParameters {
            single_walk_parameters: SingleWalkParameters::new(walk_length)?,
            iterations: 1,
            random_state: (42 ^ SEED_XOR) as NodeT,
            dense_node_mapping: None,
        })
    }

    /// Set the iterations.
    ///
    /// # Arguments
    ///
    /// * iterations: Option<NodeT> - Wethever to show the loading bar or not.
    ///
    pub fn set_iterations(mut self, iterations: Option<NodeT>) -> Result<WalksParameters, String> {
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

    /// Return the iterations.
    pub fn get_iterations(&self) -> NodeT {
        self.iterations
    }

    /// Set the maximum neighbours number to consider, making the walk probabilistic.
    ///
    /// # Arguments
    ///
    /// * max_neighbours: Option<NodeT> - Number of neighbours to consider for each extraction.
    ///
    pub fn set_max_neighbours(
        mut self,
        max_neighbours: Option<NodeT>,
    ) -> Result<WalksParameters, String> {
        if let Some(mn) = max_neighbours {
            if mn == 0 {
                return Err(String::from(
                    "max_neighbours parameter must be a strictly positive integer.",
                ));
            }
            self.single_walk_parameters.max_neighbours = Some(mn);
        }
        Ok(self)
    }

    /// Set the random_state.
    ///
    /// # Arguments
    ///
    /// * random_state: Option<usize> - random_state for reproducible random walks.
    ///
    pub fn set_random_state(mut self, random_state: Option<usize>) -> WalksParameters {
        if let Some(s) = random_state {
            self.random_state = (s ^ SEED_XOR) as NodeT;
        }
        self
    }

    /// Set the dense_node_mapping.
    ///
    /// The nodes mapping primary porpose is to map a sparse set of nodes into
    /// a smaller dense set of nodes.
    ///
    /// # Arguments
    ///
    /// * dense_node_mapping: Option<HashMap<NodeT, NodeT>> - mapping for the mapping the nodes of the walks.
    ///
    pub fn set_dense_node_mapping(
        mut self,
        dense_node_mapping: Option<HashMap<NodeT, NodeT>>,
    ) -> WalksParameters {
        self.dense_node_mapping = dense_node_mapping;
        self
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
    ) -> Result<WalksParameters, String> {
        if let Some(rw) = return_weight {
            self.single_walk_parameters.weights.return_weight =
                WalkWeights::validate_weight("return_weight", rw)?;
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
    ) -> Result<WalksParameters, String> {
        if let Some(ew) = explore_weight {
            self.single_walk_parameters.weights.explore_weight =
                WalkWeights::validate_weight("explore_weight", ew)?;
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
    ) -> Result<WalksParameters, String> {
        if let Some(cntw) = change_node_type_weight {
            self.single_walk_parameters.weights.change_node_type_weight =
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
    ) -> Result<WalksParameters, String> {
        if let Some(cetw) = change_edge_type_weight {
            self.single_walk_parameters.weights.change_edge_type_weight =
                WalkWeights::validate_weight("change_edge_type_weight", cetw)?;
        }
        Ok(self)
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
        if let Some(dense_node_mapping) = &self.dense_node_mapping {
            if !graph
                .get_unique_sources_par_iter()
                .all(|node| dense_node_mapping.contains_key(&(node as NodeT)))
            {
                return Err(String::from(concat!(
                    "Given nodes mapping does not contain ",
                    "one or more NOT trap nodes that may be extracted from walk."
                )));
            }
        }

        Ok(())
    }

    /// Return boolean value representing if walk is of first order.
    pub fn is_first_order_walk(&self) -> bool {
        self.single_walk_parameters.is_first_order_walk()
    }
}
