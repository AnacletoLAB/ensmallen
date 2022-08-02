use super::*;

#[derive(Clone, Debug, PartialEq)]
/// Struct to wrap walk weights.
#[no_binding]
pub struct WalkWeights {
    pub return_weight: ParamsT,
    pub(crate) explore_weight: ParamsT,
    pub(crate) change_node_type_weight: ParamsT,
    pub(crate) change_edge_type_weight: ParamsT,
}

#[derive(Clone, Debug, PartialEq)]
/// Struct to wrap parameters relative to a single walk.
#[no_binding]
pub struct SingleWalkParameters {
    pub(crate) walk_length: u64,
    pub(crate) weights: WalkWeights,
    pub(crate) max_neighbours: Option<NodeT>,
    pub(crate) normalize_by_degree: bool,
}

#[derive(Clone, Debug, PartialEq)]
/// Struct to wrap parameters relative to a set of walks.
#[no_binding]
pub struct WalksParameters {
    pub(crate) single_walk_parameters: SingleWalkParameters,
    pub(crate) iterations: NodeT,
    pub(crate) random_state: NodeT,
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
    fn validate_weight(weight_name: &str, weight: WeightT) -> Result<WeightT> {
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
        [
            self.change_node_type_weight,
            self.change_edge_type_weight,
            self.return_weight,
            self.explore_weight,
        ]
        .iter()
        .all(|weight| !not_one(*weight))
    }

    /// Return boolean value representing if walk is a Node2Vec walk.
    ///
    /// # Example
    /// The default parametrization defines a Node2Vec walk:
    ///
    /// ```rust
    /// # use graph::walks_parameters::WalkWeights;
    /// let weights = WalkWeights::default();
    /// assert!(!weights.is_node2vec_walk());
    /// ```
    pub fn is_node2vec_walk(&self) -> bool {
        [self.return_weight, self.explore_weight]
            .iter()
            .any(|weight| not_one(*weight))
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
    pub fn new(walk_length: u64) -> Result<SingleWalkParameters> {
        if walk_length == 0 {
            return Err(String::from("The provided lenght for the walk is zero!"));
        }
        Ok(SingleWalkParameters {
            walk_length,
            weights: WalkWeights::default(),
            max_neighbours: Some(100),
            normalize_by_degree: false,
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
        self.weights.is_first_order_walk() && !self.normalize_by_degree
    }

    /// Return boolean value representing if walk is a Node2Vec walk.
    ///
    /// # Example
    /// The default parametrization defines a Node2Vec walk:
    ///
    /// ```rust
    /// # use graph::walks_parameters::SingleWalkParameters;
    /// let weights = SingleWalkParameters::new(32).unwrap();
    /// assert!(!weights.is_node2vec_walk());
    /// ```
    pub fn is_node2vec_walk(&self) -> bool {
        self.weights.is_node2vec_walk()
    }
}

impl Default for WalksParameters {
    /// Create a default WalksParameters object.
    ///
    /// By default the object is parametrized for a simple first-order walk.
    fn default() -> Self {
        WalksParameters::new(32).unwrap()
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
    pub fn new(walk_length: u64) -> Result<WalksParameters> {
        Ok(WalksParameters {
            single_walk_parameters: SingleWalkParameters::new(walk_length)?,
            iterations: 1,
            random_state: splitmix64(42) as NodeT,
        })
    }

    /// Set the iterations.
    ///
    /// # Arguments
    ///
    /// * `iterations`: Option<NodeT> - Whether to show the loading bar or not.
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
    pub fn set_iterations(mut self, iterations: Option<NodeT>) -> Result<WalksParameters> {
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

    /// Return the length of the random walk.
    pub fn get_random_walk_length(&self) -> u64 {
        self.single_walk_parameters.walk_length
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
    pub fn set_max_neighbours(mut self, max_neighbours: Option<NodeT>) -> Result<WalksParameters> {
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

    /// Set whether the walk destination nodes weights should be weighted by the destination node degree.
    ///
    /// # Arguments
    /// * `normalize_by_degree`: Option<NodeT> - Number of neighbours to consider for each extraction.
    pub fn set_normalize_by_degree(mut self, normalize_by_degree: Option<bool>) -> WalksParameters {
        if let Some(normalize_by_degree) = normalize_by_degree {
            self.single_walk_parameters.normalize_by_degree = normalize_by_degree;
        }
        self
    }

    /// Set the random_state.
    ///
    /// # Arguments
    /// * `random_state`: Option<usize> - random_state for reproducible random walks.
    ///
    pub fn set_random_state(mut self, random_state: Option<usize>) -> WalksParameters {
        if let Some(s) = random_state {
            self.random_state = splitmix64(s as u64) as NodeT;
        }
        self
    }

    /// Return the random_state used in the walks.
    pub fn get_random_state(&self) -> NodeT {
        self.random_state
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
    pub fn set_return_weight(mut self, return_weight: Option<WeightT>) -> Result<WalksParameters> {
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
    /// * `explore_weight`: Option<WeightT> - weight for the exploration factor.
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
    ) -> Result<WalksParameters> {
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
    /// * `change_node_type_weight`: Option<WeightT> - weight for the exploration of different node types.
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
    ) -> Result<WalksParameters> {
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
    /// * `change_edge_type_weight`: Option<WeightT> - weight for the exploration of different node types.
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
    ) -> Result<WalksParameters> {
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
    /// * `graph`: Graph - Graph object for which parameters are to be validated.
    ///
    /// # Example
    /// A graph is always remappable to itself:
    /// ```rust
    /// # use graph::walks_parameters::WalksParameters;
    /// # let ppi = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// # let mut parameters = WalksParameters::new(32).unwrap();
    /// assert!(parameters.set_dense_node_mapping(Some(ppi.get_dense_nodes_mapping())).validate(&ppi).is_ok());
    /// ```
    /// Two different graphs, like Cora and STRING, are not remappable:
    /// ```rust
    /// # use graph::walks_parameters::WalksParameters;
    /// # let cora = graph::test_utilities::load_cora();
    /// # let ppi = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// # let mut parameters = WalksParameters::new(32).unwrap();
    /// assert!(parameters.set_dense_node_mapping(Some(ppi.get_dense_nodes_mapping())).validate(&cora).is_err());
    /// ```
    ///
    pub fn validate(&self, graph: &Graph) -> Result<()> {
        if self.is_node2vec_walk() && graph.is_directed() {
            return Err(concat!(
                "The walk is a Node2Vec walk and ",
                "the graph is directed, which is not yet supported."
            )
            .to_string());
        }
        if graph.has_trap_nodes() {
            return Err(concat!(
                "The graph is directed with trap nodes which is not yet supported."
            )
            .to_string());
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

    /// Return boolean value representing if walk is a Node2Vec walk.
    ///
    /// # Example
    /// The default parametrization defines a Node2Vec walk:
    ///
    /// ```rust
    /// # use graph::walks_parameters::WalksParameters;
    /// assert!(!WalksParameters::new(32).unwrap().is_node2vec_walk());
    /// ```
    pub fn is_node2vec_walk(&self) -> bool {
        self.single_walk_parameters.is_node2vec_walk()
    }
}
