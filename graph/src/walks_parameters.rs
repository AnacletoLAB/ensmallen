use super::*;
use rayon::prelude::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct WalkWeights {
    pub(crate) return_weight: ParamsT,
    pub(crate) explore_weight: ParamsT,
    pub(crate) change_node_type_weight: ParamsT,
    pub(crate) change_edge_type_weight: ParamsT,
}

#[derive(Clone)]
pub struct SingleWalkParameters {
    pub(crate) length: usize,
    pub(crate) weights: WalkWeights,
}

#[derive(Clone)]
pub struct WalksParameters {
    pub(crate) single_walk_parameters: SingleWalkParameters,
    pub(crate) iterations: usize,
    pub(crate) min_length: usize,
    pub(crate) verbose: bool,
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

    /// Return boolean value representing if walk is of first order.
    pub fn is_first_order_walk(&self) -> bool {
        let weights = vec![
            self.change_node_type_weight,
            self.change_edge_type_weight,
            self.return_weight,
            self.explore_weight,
        ];
        weights
            .iter()
            .all(|weight| (weight - 1.0).abs() <= f64::EPSILON)
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
    pub fn new(length: usize) -> Result<WalksParameters, String> {
        Ok(WalksParameters {
            single_walk_parameters: SingleWalkParameters::new(length, WalkWeights::default())?,
            iterations: 1,
            min_length: 1,
            seed: 42 ^ SEED_XOR,
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
            self.seed = s ^ SEED_XOR;
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
        if self.min_length >= self.single_walk_parameters.length {
            return Err(format!(
                "The given min-walk-length {} is bigger or equal to the given walk length {}",
                self.min_length, self.single_walk_parameters.length
            ));
        }

        if let Some(dense_nodes_mapping) = &self.dense_nodes_mapping {
            if !(&graph.not_trap_nodes)
                .into_par_iter()
                .all(|node| dense_nodes_mapping.contains_key(&node))
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
