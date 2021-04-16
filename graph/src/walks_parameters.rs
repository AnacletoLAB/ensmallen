use super::*;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
/// Struct to wrap walk weights.
pub struct WalkWeights {
    pub(crate) return_weight: ParamsT,
    pub(crate) explore_weight: ParamsT,
    pub(crate) change_node_type_weight: ParamsT,
    pub(crate) change_edge_type_weight: ParamsT,
}

#[derive(Clone, Debug, PartialEq)]
/// Struct to wrap parameters relative to a single walk.
pub struct SingleWalkParameters {
    pub(crate) walk_length: u64,
    pub(crate) weights: WalkWeights,
    pub(crate) max_neighbours: Option<NodeT>,
}

#[derive(Clone, Debug, PartialEq)]
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
    /// * `weight_name`: &str - name of the weight, used for building the exception.
    /// * `weight`: Option<WeightT> - Value of the weight.
    ///
    /// TODO: is this a duplicate?
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
    ///
    /// # Example
    /// The default parametrization defines a first order walk:
    ///
    /// ```rust
    /// # use graph::walks_parameters::WalkWeights;
    /// let weights = WalkWeights::default();
    /// assert!(weights.is_first_order_walk());
    /// ```
    pub fn is_first_order_walk(&self) -> bool {
        let weights = vec![
            self.change_node_type_weight,
            self.change_edge_type_weight,
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
    /// * `walk_length`: usize - Maximal walk_length of the walk.
    ///
    /// # Example
    /// You can create a single walk parameters struct as follows:
    ///
    /// ```rust
    /// # use graph::walks_parameters::SingleWalkParameters;
    /// assert!(SingleWalkParameters::new(45).is_ok());
    /// ```
    ///
    /// as long as you don't try to make a zero walk length you'll be fine:
    ///
    /// ```rust
    /// # use graph::walks_parameters::SingleWalkParameters;
    /// assert!(SingleWalkParameters::new(0).is_err());
    /// ```
    pub fn new(walk_length: u64) -> Result<SingleWalkParameters, String> {
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
    ///
    /// # Example
    /// The default parametrization defines a first order walk:
    ///
    /// ```rust
    /// # use graph::walks_parameters::SingleWalkParameters;
    /// let weights = SingleWalkParameters::new(32).unwrap();
    /// assert!(weights.is_first_order_walk());
    /// ```
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
    /// * `walk_length`: NodeT - Maximal walk_length of the walk.
    ///
    pub fn new(walk_length: u64) -> Result<WalksParameters, String> {
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
    /// * `iterations`: Option<NodeT> - whether to show the loading bar or not.
    ///
    /// # Example
    /// You can change the `iterations` parameter as follows:
    ///
    /// ```rust
    /// # use graph::walks_parameters::WalksParameters;
    /// assert!(WalksParameters::new(32).unwrap().set_iterations(Some(0)).is_err());
    /// assert!(WalksParameters::new(32).unwrap().set_iterations(Some(2)).is_ok());
    /// ```
    ///
    /// You can also call the method with an option None, in order to avoid a match
    /// wrapper above. This will end up don't doing anything, just a passthrough.
    ///
    /// ```rust
    /// # use graph::walks_parameters::WalksParameters;
    /// assert!(WalksParameters::new(32).unwrap().set_iterations(None).is_ok());
    /// ```
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
    ///
    /// # Example
    /// To retrieve the number of iterations you can do the following:
    ///
    /// ```rust
    /// # use graph::walks_parameters::WalksParameters;
    /// let mut walk_parameters = WalksParameters::new(32).unwrap();
    /// assert_eq!(walk_parameters.get_iterations(), 1);
    /// let iterations_number = 56;
    /// walk_parameters = walk_parameters.set_iterations(Some(iterations_number)).unwrap();
    /// assert_eq!(walk_parameters.get_iterations(), iterations_number);
    /// ```
    pub fn get_iterations(&self) -> NodeT {
        self.iterations
    }

    /// Set the maximum neighbours number to consider, making the walk probabilistic.
    ///
    /// # Arguments
    ///
    /// * `max_neighbours`: Option<NodeT> - Number of neighbours to consider for each extraction.
    ///
    /// # Example
    /// You can change the `max_neighbours` parameter as follows:
    ///
    /// ```rust
    /// # use graph::walks_parameters::WalksParameters;
    /// assert!(WalksParameters::new(32).unwrap().set_max_neighbours(Some(0)).is_err());
    /// assert!(WalksParameters::new(32).unwrap().set_max_neighbours(Some(2)).is_ok());
    /// ```
    ///
    /// You can also call the method with an option None, in order to avoid a match
    /// wrapper above. This will end up don't doing anything, just a passthrough.
    ///
    /// ```rust
    /// # use graph::walks_parameters::WalksParameters;
    /// assert!(WalksParameters::new(32).unwrap().set_max_neighbours(None).is_ok());
    /// ```
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
    /// * `random_state`: Option<usize> - random_state for reproducible random walks.
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
    /// * `dense_node_mapping`: Option<HashMap<NodeT, NodeT>> - mapping for the mapping the nodes of the walks.
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
    /// * `return_weight`: Option<WeightT> - weight for the exploitation factor.
    ///
    /// # Example
    /// You can change the `return_weight` parameter as follows:
    ///
    /// ```rust
    /// # use graph::walks_parameters::WalksParameters;
    /// assert!(WalksParameters::new(32).unwrap().set_return_weight(Some(-1.0)).is_err());
    /// assert!(WalksParameters::new(32).unwrap().set_return_weight(Some(2.0)).is_ok());
    /// assert!(WalksParameters::new(32).unwrap().set_return_weight(Some(1.0)).is_ok());
    /// assert!(WalksParameters::new(32).unwrap().set_return_weight(Some(1.0)).unwrap().is_first_order_walk());
    /// ```
    ///
    /// You can also call the method with an option None, in order to avoid a match
    /// wrapper above. This will end up don't doing anything, just a passthrough.
    ///
    /// ```rust
    /// # use graph::walks_parameters::WalksParameters;
    /// assert!(WalksParameters::new(32).unwrap().set_return_weight(None).unwrap().is_first_order_walk());
    /// ```
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
    /// # Example
    /// You can change the `explore_weight` parameter as follows:
    ///
    /// ```rust
    /// # use graph::walks_parameters::WalksParameters;
    /// assert!(WalksParameters::new(32).unwrap().set_explore_weight(Some(-1.0)).is_err());
    /// assert!(WalksParameters::new(32).unwrap().set_explore_weight(Some(2.0)).is_ok());
    /// assert!(WalksParameters::new(32).unwrap().set_explore_weight(Some(1.0)).is_ok());
    /// assert!(WalksParameters::new(32).unwrap().set_explore_weight(Some(1.0)).unwrap().is_first_order_walk());
    /// ```
    ///
    /// You can also call the method with an option None, in order to avoid a match
    /// wrapper above. This will end up don't doing anything, just a passthrough.
    ///
    /// ```rust
    /// # use graph::walks_parameters::WalksParameters;
    /// assert!(WalksParameters::new(32).unwrap().set_explore_weight(None).unwrap().is_first_order_walk());
    /// ```
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
    /// # Example
    /// You can change the `change_node_type_weight` parameter as follows:
    ///
    /// ```rust
    /// # use graph::walks_parameters::WalksParameters;
    /// assert!(WalksParameters::new(32).unwrap().set_change_node_type_weight(Some(-1.0)).is_err());
    /// assert!(WalksParameters::new(32).unwrap().set_change_node_type_weight(Some(2.0)).is_ok());
    /// assert!(WalksParameters::new(32).unwrap().set_change_node_type_weight(Some(1.0)).is_ok());
    /// assert!(WalksParameters::new(32).unwrap().set_change_node_type_weight(Some(1.0)).unwrap().is_first_order_walk());
    /// ```
    ///
    /// You can also call the method with an option None, in order to avoid a match
    /// wrapper above. This will end up don't doing anything, just a passthrough.
    ///
    /// ```rust
    /// # use graph::walks_parameters::WalksParameters;
    /// assert!(WalksParameters::new(32).unwrap().set_change_node_type_weight(None).unwrap().is_first_order_walk());
    /// ```
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
    /// # Example
    /// You can change the `change_edge_type_weight` parameter as follows:
    ///
    /// ```rust
    /// # use graph::walks_parameters::WalksParameters;
    /// assert!(WalksParameters::new(32).unwrap().set_change_edge_type_weight(Some(-1.0)).is_err());
    /// assert!(WalksParameters::new(32).unwrap().set_change_edge_type_weight(Some(2.0)).is_ok());
    /// assert!(WalksParameters::new(32).unwrap().set_change_edge_type_weight(Some(1.0)).is_ok());
    /// assert!(WalksParameters::new(32).unwrap().set_change_edge_type_weight(Some(1.0)).unwrap().is_first_order_walk());
    /// ```
    /// You can also call the method with an option None, in order to avoid a match
    /// wrapper above. This will end up don't doing anything, just a passthrough.
    /// ```rust
    /// # use graph::walks_parameters::WalksParameters;
    /// assert!(WalksParameters::new(32).unwrap().set_change_edge_type_weight(None).unwrap().is_first_order_walk());
    /// ```
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
    /// # Example
    /// A graph is always remappable to itself:
    /// ```rust
    /// # use graph::walks_parameters::WalksParameters;
    /// # let ppi = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// # let mut parameters = WalksParameters::new(32).unwrap();
    /// assert!(parameters.set_dense_node_mapping(Some(ppi.get_dense_nodes_mapping())).validate(&ppi).is_ok());
    /// ```
    /// Two different graphs, like Cora and STRING, are not remappable:
    /// ```rust
    /// # use graph::walks_parameters::WalksParameters;
    /// # let cora = graph::test_utilities::load_cora().unwrap();
    /// # let ppi = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// # let mut parameters = WalksParameters::new(32).unwrap();
    /// assert!(parameters.set_dense_node_mapping(Some(ppi.get_dense_nodes_mapping())).validate(&cora).is_err());
    /// ```
    ///
    pub fn validate(&self, graph: &Graph) -> Result<(), String> {
        if let Some(dense_node_mapping) = &self.dense_node_mapping {
            if !graph
                .iter_unique_source_node_ids()
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
    ///
    /// # Example
    /// The default parametrization defines a first order walk:
    ///
    /// ```rust
    /// # use graph::walks_parameters::WalksParameters;
    /// assert!(WalksParameters::new(32).unwrap().is_first_order_walk());
    /// ```
    pub fn is_first_order_walk(&self) -> bool {
        self.single_walk_parameters.is_first_order_walk()
    }
}
